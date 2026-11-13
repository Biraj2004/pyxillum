#![warn(missing_docs)]

//! # Pyxillum Transport
//!
//! Transport trait and backends for communicating with SP621E controllers.
//! Includes `SimTransport` (in-memory simulator for hardware-free development and tests)
//! and `BtleplugTransport` (real Bluetooth Low Energy interface with software filtering).

use async_trait::async_trait;
#[allow(unused_imports)]
use btleplug as _;
use pyxillum_protocol::{
    DATA_CHARACTERISTIC_UUID, MANUFACTURER_ID_ALT, MANUFACTURER_ID_SP, SERVICE_UUID,
};
use pyxillum_sim::{DeviceSimulator, SimError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, Mutex};
use tracing::{info, warn};

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

use btleplug::api::{
    Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use uuid::Uuid;

/// Real Bluetooth Low Energy transport backend powered by `btleplug`.
pub struct BtleplugTransport {
    adapter: Arc<tokio::sync::Mutex<Option<Adapter>>>,
    connected_address: Option<String>,
    connected_peripheral: Option<Peripheral>,
    write_characteristic: Option<Characteristic>,
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
            adapter: Arc::new(tokio::sync::Mutex::new(None)),
            connected_address: None,
            connected_peripheral: None,
            write_characteristic: None,
        }
    }

    /// Evaluates whether an advertised BLE device matches the SP6xxE controller family.
    pub fn is_sp6xx_advertisement(
        name: Option<&str>,
        manufacturer_ids: &[u16],
        service_uuids: &[String],
    ) -> bool {
        let has_manufacturer = manufacturer_ids
            .iter()
            .any(|&id| id == MANUFACTURER_ID_SP || id == MANUFACTURER_ID_ALT);

        let has_service = service_uuids
            .iter()
            .any(|uuid| uuid.to_lowercase() == SERVICE_UUID.to_lowercase());

        let has_name = if let Some(n) = name {
            let lower = n.to_lowercase();
            lower.contains("sp6")
                || lower.contains("sp5")
                || lower.contains("sp8")
                || lower.contains("banlan")
                || lower.contains("pixel")
                || lower.contains("led")
        } else {
            false
        };

        has_manufacturer || has_service || has_name
    }

    async fn get_adapter(&self) -> Result<Adapter, TransportError> {
        let mut adapter_guard = self.adapter.lock().await;
        if let Some(adapter) = adapter_guard.as_ref() {
            return Ok(adapter.clone());
        }

        let manager = Manager::new()
            .await
            .map_err(|e| TransportError::Bluetooth(e.to_string()))?;
        let adapters = manager
            .adapters()
            .await
            .map_err(|e| TransportError::Bluetooth(e.to_string()))?;
        let adapter = adapters.into_iter().next().ok_or_else(|| {
            TransportError::Bluetooth(
                "No Bluetooth Low Energy adapter detected on host system".to_string(),
            )
        })?;

        *adapter_guard = Some(adapter.clone());
        Ok(adapter)
    }
}

#[async_trait]
impl Transport for BtleplugTransport {
    async fn scan(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        info!("Scanning for nearby BLE pixel LED controllers...");
        let adapter = match self.get_adapter().await {
            Ok(a) => a,
            Err(e) => {
                warn!("Bluetooth adapter unavailable: {}", e);
                return Err(e);
            }
        };

        // Start scanning
        adapter
            .start_scan(ScanFilter::default())
            .await
            .map_err(|e| TransportError::Bluetooth(e.to_string()))?;

        // Allow scan to gather advertisements over 2.5 seconds
        tokio::time::sleep(std::time::Duration::from_millis(2500)).await;

        let _ = adapter.stop_scan().await;

        let peripherals = adapter
            .peripherals()
            .await
            .map_err(|e| TransportError::Bluetooth(e.to_string()))?;

        let mut results = Vec::new();
        for peripheral in peripherals {
            let address = peripheral.address().to_string();
            let props = peripheral.properties().await.ok().flatten();

            let name = props.as_ref().and_then(|p| p.local_name.clone());
            let rssi = props.as_ref().and_then(|p| p.rssi);

            // Extract manufacturer IDs and service UUIDs for sorting & classification
            let mfg_ids: Vec<u16> = props
                .as_ref()
                .map(|p| p.manufacturer_data.keys().copied().collect())
                .unwrap_or_default();
            let srv_uuids: Vec<String> = props
                .as_ref()
                .map(|p| p.services.iter().map(|u| u.to_string()).collect())
                .unwrap_or_default();

            let is_compatible = Self::is_sp6xx_advertisement(name.as_deref(), &mfg_ids, &srv_uuids);

            // Include if compatible or if device has a local name
            if is_compatible || name.is_some() {
                results.push(DiscoveredDevice {
                    address,
                    name,
                    rssi,
                    is_simulated: false,
                });
            }
        }

        // Sort compatible and stronger RSSI devices first
        results.sort_by_key(|a| std::cmp::Reverse(a.rssi.unwrap_or(-100)));

        info!("BLE scan completed. Found {} devices.", results.len());
        Ok(results)
    }

    async fn connect(&mut self, address: &str) -> Result<(), TransportError> {
        info!("Attempting BLE connection to target: {}", address);
        let adapter = self.get_adapter().await?;

        // 1. Check existing peripherals
        let mut target_peripheral = None;
        if let Ok(peripherals) = adapter.peripherals().await {
            for p in peripherals {
                if p.address().to_string().eq_ignore_ascii_case(address) {
                    target_peripheral = Some(p);
                    break;
                }
            }
        }

        // 2. If not found in current cache, perform targeted 1.5s discovery
        if target_peripheral.is_none() {
            let _ = adapter.start_scan(ScanFilter::default()).await;
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
            let _ = adapter.stop_scan().await;

            if let Ok(peripherals) = adapter.peripherals().await {
                for p in peripherals {
                    if p.address().to_string().eq_ignore_ascii_case(address) {
                        target_peripheral = Some(p);
                        break;
                    }
                }
            }
        }

        let peripheral =
            target_peripheral.ok_or_else(|| TransportError::DeviceNotFound(address.to_string()))?;

        peripheral
            .connect()
            .await
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;

        peripheral
            .discover_services()
            .await
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;

        // Look for 0xFFE1 data characteristic
        let target_uuid = Uuid::parse_str(DATA_CHARACTERISTIC_UUID).unwrap_or_default();
        let mut write_char = None;

        for c in peripheral.characteristics() {
            if c.uuid == target_uuid || c.uuid.to_string().to_lowercase().contains("ffe1") {
                write_char = Some(c);
                break;
            }
        }

        // Fallback: look for any characteristic that supports Write
        if write_char.is_none() {
            for c in peripheral.characteristics() {
                if c.properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE)
                    || c.properties.contains(CharPropFlags::WRITE)
                {
                    write_char = Some(c);
                    break;
                }
            }
        }

        self.write_characteristic = write_char;
        self.connected_peripheral = Some(peripheral);
        self.connected_address = Some(address.to_string());

        info!(
            "Successfully connected to BLE device: {}. Characteristic ready: {}",
            address,
            self.write_characteristic.is_some()
        );
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        info!("Disconnecting active BLE connection");
        if let Some(peripheral) = self.connected_peripheral.take() {
            let _ = peripheral.disconnect().await;
        }
        self.write_characteristic = None;
        self.connected_address = None;
        Ok(())
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), TransportError> {
        let peripheral = self
            .connected_peripheral
            .as_ref()
            .ok_or(TransportError::NotConnected)?;
        let characteristic = self
            .write_characteristic
            .as_ref()
            .ok_or(TransportError::NotConnected)?;

        if characteristic
            .properties
            .contains(CharPropFlags::WRITE_WITHOUT_RESPONSE)
        {
            peripheral
                .write(characteristic, data, WriteType::WithoutResponse)
                .await
                .map_err(|e| TransportError::WriteFailed(e.to_string()))?;
        } else {
            peripheral
                .write(characteristic, data, WriteType::WithResponse)
                .await
                .map_err(|e| TransportError::WriteFailed(e.to_string()))?;
        }

        Ok(())
    }

    async fn subscribe(&mut self) -> Result<mpsc::Receiver<Vec<u8>>, TransportError> {
        let (tx, rx) = mpsc::channel(64);

        if let (Some(peripheral), Some(characteristic)) =
            (&self.connected_peripheral, &self.write_characteristic)
        {
            if characteristic.properties.contains(CharPropFlags::NOTIFY)
                || characteristic.properties.contains(CharPropFlags::INDICATE)
            {
                if let Err(e) = peripheral.subscribe(characteristic).await {
                    warn!("Failed to subscribe to characteristic notifications: {}", e);
                } else if let Ok(mut stream) = peripheral.notifications().await {
                    tokio::spawn(async move {
                        while let Some(notif) = stream.next().await {
                            if tx.send(notif.value).await.is_err() {
                                break;
                            }
                        }
                    });
                }
            }
        }

        Ok(rx)
    }

    fn is_connected(&self) -> bool {
        self.connected_address.is_some() && self.connected_peripheral.is_some()
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
