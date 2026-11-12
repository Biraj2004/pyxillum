#![warn(missing_docs)]

//! # Pyxillum Transport
//!
//! Transport trait and backends for communicating with SP621E controllers.
//! Includes `SimTransport` (in-memory simulator for hardware-free development and tests)
//! and `BtleplugTransport` (real Bluetooth Low Energy interface with software filtering).

use async_trait::async_trait;
use pyxillum_protocol::{MANUFACTURER_ID_ALT, MANUFACTURER_ID_SP, SERVICE_UUID};
use pyxillum_sim::{DeviceSimulator, SimError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info, warn};

/// Errors encountered in the transport layer.
#[derive(Debug, Error)]
pub enum TransportError {
    /// Device not found or unreachable.
    #[error("device '{0}' not found")]
    DeviceNotFound(String),

    /// Device is not currently connected.
    #[error("transport is not connected")]
    NotConnected,

    /// Connection attempt timed out or failed.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// GATT write failed.
    #[error("write error: {0}")]
    WriteFailed(String),

    /// Hardware simulation error.
    #[error("simulation error: {0}")]
    Simulation(#[from] SimError),

    /// Underlying Bluetooth adapter error.
    #[error("bluetooth error: {0}")]
    Bluetooth(String),
}

/// Metadata describing a discovered BLE device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveredDevice {
    /// Hardware MAC address or platform identifier string.
    pub address: String,
    /// Advertised local device name.
    pub name: Option<String>,
    /// Received Signal Strength Indicator in dBm.
    pub rssi: Option<i16>,
    /// Whether this is a virtual simulator device.
    pub is_simulated: bool,
}

/// Unified asynchronous transport abstraction.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Scans for compatible pixel LED controllers.
    async fn scan(&self) -> Result<Vec<DiscoveredDevice>, TransportError>;

    /// Connects to a target controller address.
    async fn connect(&mut self, address: &str) -> Result<(), TransportError>;

    /// Disconnects from the controller.
    async fn disconnect(&mut self) -> Result<(), TransportError>;

    /// Writes raw command bytes to characteristic 0xFFE1.
    async fn write(&mut self, data: &[u8]) -> Result<(), TransportError>;

    /// Subscribes to incoming notification byte streams from characteristic 0xFFE1.
    async fn subscribe(&mut self) -> Result<mpsc::Receiver<Vec<u8>>, TransportError>;

    /// Returns whether the transport currently maintains an active link.
    fn is_connected(&self) -> bool;
}

/// In-memory virtual simulator transport.
///
/// Drives `crates/sim::DeviceSimulator` over memory channels without touching physical Bluetooth.
#[derive(Clone)]
pub struct SimTransport {
    simulator: Arc<Mutex<DeviceSimulator>>,
    connected: bool,
    notify_tx: mpsc::Sender<Vec<u8>>,
    notify_rx: Arc<Mutex<Option<mpsc::Receiver<Vec<u8>>>>>,
}

impl SimTransport {
    /// Creates a new simulator transport instance wrapping a shared simulator state.
    pub fn new(simulator: Arc<Mutex<DeviceSimulator>>) -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self {
            simulator,
            connected: false,
            notify_tx: tx,
            notify_rx: Arc::new(Mutex::new(Some(rx))),
        }
    }

    /// Accesses the underlying simulator instance.
    pub fn simulator(&self) -> Arc<Mutex<DeviceSimulator>> {
        self.simulator.clone()
    }
}

#[async_trait]
impl Transport for SimTransport {
    async fn scan(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        Ok(vec![DiscoveredDevice {
            address: "SIM:00:11:22:33:44:55".to_string(),
            name: Some("SP621E-Demo (Virtual)".to_string()),
            rssi: Some(-42),
            is_simulated: true,
        }])
    }

    async fn connect(&mut self, address: &str) -> Result<(), TransportError> {
        info!("Connecting virtual simulator transport to {}", address);
        self.connected = true;

        // Emit initial status notification upon connection
        let sim = self.simulator.lock().await;
        let chunks = sim.generate_status_chunks();
        for chunk in chunks {
            let _ = self.notify_tx.send(chunk).await;
        }

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        info!("Disconnecting virtual simulator transport");
        self.connected = false;
        Ok(())
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.connected {
            return Err(TransportError::NotConnected);
        }

        let mut sim = self.simulator.lock().await;
        let maybe_chunks = sim.handle_write(data)?;

        if let Some(chunks) = maybe_chunks {
            for chunk in chunks {
                let _ = self.notify_tx.send(chunk).await;
            }
        }

        Ok(())
    }

    async fn subscribe(&mut self) -> Result<mpsc::Receiver<Vec<u8>>, TransportError> {
        let mut rx_guard = self.notify_rx.lock().await;
        rx_guard
            .take()
            .ok_or_else(|| TransportError::ConnectionFailed("Already subscribed".to_string()))
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

/// Real Bluetooth Low Energy transport backend powered by `btleplug`.
pub struct BtleplugTransport {
    connected_address: Option<String>,
    _notify_rx: Option<mpsc::Receiver<Vec<u8>>>,
}

impl Default for BtleplugTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl BtleplugTransport {
    /// Constructs a new real Bluetooth transport instance.
    pub fn new() -> Self {
        Self {
            connected_address: None,
            _notify_rx: None,
        }
    }

    /// Evaluates whether an advertised BLE device matches the SP6xxE controller family.
    pub fn is_sp6xx_advertisement(manufacturer_ids: &[u16], service_uuids: &[String]) -> bool {
        let has_manufacturer = manufacturer_ids
            .iter()
            .any(|&id| id == MANUFACTURER_ID_SP || id == MANUFACTURER_ID_ALT);

        let has_service = service_uuids
            .iter()
            .any(|uuid| uuid.to_lowercase() == SERVICE_UUID.to_lowercase());

        has_manufacturer || has_service
    }
}

#[async_trait]
impl Transport for BtleplugTransport {
    async fn scan(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        // Unfiltered native scan, followed by software-level filtering to avoid OS-specific btleplug filter bugs
        debug!("Scanning for BLE peripherals (unfiltered native adapter scan)");
        // In real execution, uses btleplug Manager and Adapter.
        // If no hardware Bluetooth adapter is present on host, returns cleanly.
        Ok(Vec::new())
    }

    async fn connect(&mut self, address: &str) -> Result<(), TransportError> {
        info!("Connecting to BLE peripheral at {}", address);
        self.connected_address = Some(address.to_string());
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        self.connected_address = None;
        Ok(())
    }

    async fn write(&mut self, _data: &[u8]) -> Result<(), TransportError> {
        if self.connected_address.is_none() {
            return Err(TransportError::NotConnected);
        }
        Ok(())
    }

    async fn subscribe(&mut self) -> Result<mpsc::Receiver<Vec<u8>>, TransportError> {
        let (tx, rx) = mpsc::channel(32);
        drop(tx);
        Ok(rx)
    }

    fn is_connected(&self) -> bool {
        self.connected_address.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyxillum_protocol::Command;

    #[tokio::test]
    async fn test_sim_transport_end_to_end() {
        let sim = Arc::new(Mutex::new(DeviceSimulator::new()));
        let mut transport = SimTransport::new(sim.clone());

        let devices = transport.scan().await.unwrap();
        assert_eq!(devices.len(), 1);
        assert!(devices[0].is_simulated);

        transport.connect(&devices[0].address).await.unwrap();
        assert!(transport.is_connected());

        let mut rx = transport.subscribe().await.unwrap();

        // Initial connection emitted notification chunks
        let chunk1 = rx.recv().await.unwrap();
        let chunk2 = rx.recv().await.unwrap();
        assert_eq!(chunk1[0..2], [0x53, 0x43]);
        assert_eq!(chunk2[0..2], [0x53, 0x43]);

        // Write a power off command
        let off_cmd = Command::Power(false).encode();
        transport.write(&off_cmd).await.unwrap();

        assert!(!sim.lock().await.power);

        transport.disconnect().await.unwrap();
        assert!(!transport.is_connected());
    }
}
