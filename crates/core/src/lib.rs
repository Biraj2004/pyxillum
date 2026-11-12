#![warn(missing_docs)]

//! # Pyxillum Core
//!
//! High-level state management, device coordination, lighting effects generation,
//! and scene preset persistence for Pyxillum.

use pyxillum_protocol::{
    AudioInput, ChipOrder, Command, DeviceStatus, LightMode, ProtocolError, RgbColor,
    StatusAssembler, EFFECT_SOLID,
};
use pyxillum_transport::{DiscoveredDevice, Transport, TransportError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use tracing::warn;

/// Core domain errors.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Transport communication error.
    #[error("transport error: {0}")]
    Transport(#[from] TransportError),

    /// Protocol encoding error.
    #[error("protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    /// Preset file persistence I/O error.
    #[error("persistence error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error.
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Requested scene preset not found.
    #[error("preset '{0}' not found")]
    PresetNotFound(String),
}

/// Snapshot of the controller's active runtime state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceState {
    /// Whether connected over BLE or simulation.
    pub is_connected: bool,
    /// Active peripheral address.
    pub address: Option<String>,
    /// Device power state.
    pub power: bool,
    /// Operating light mode.
    pub mode: LightMode,
    /// Active effect code.
    pub effect: u8,
    /// Physical strip RGB channel sequence.
    pub chip_order: ChipOrder,
    /// Master brightness (0–255).
    pub brightness: u8,
    /// Animation playback speed (1–10).
    pub speed: u8,
    /// Pixel length (1–150).
    pub length: u8,
    /// Current solid colour.
    pub color: RgbColor,
    /// Audio input mode.
    pub audio_input: AudioInput,
    /// Sensitivity level (1–16).
    pub sensitivity: u8,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            is_connected: false,
            address: None,
            power: true,
            mode: LightMode::Single,
            effect: EFFECT_SOLID,
            chip_order: ChipOrder::Grb,
            brightness: 255,
            speed: 5,
            length: 30,
            color: RgbColor::new(255, 120, 0),
            audio_input: AudioInput::InternalMic,
            sensitivity: 8,
        }
    }
}

impl DeviceState {
    /// Updates the state snapshot from an assembled device status notification.
    pub fn update_from_status(&mut self, status: &DeviceStatus) {
        self.power = status.power;
        self.mode = status.mode;
        self.effect = status.effect;
        self.chip_order = status.chip_order;
        self.brightness = status.brightness;
        self.speed = status.speed;
        self.length = status.length;
        self.color = status.color;
        self.audio_input = status.audio_input;
        self.sensitivity = status.sensitivity;
    }
}

/// Saved lighting preset / scene.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScenePreset {
    /// Schema format version for backwards compatibility.
    pub version: u32,
    /// Human-readable title of the scene preset.
    pub name: String,
    /// Preset colour.
    pub color: RgbColor,
    /// Preset effect pattern code.
    pub effect: u8,
    /// Playback speed.
    pub speed: u8,
    /// Target brightness.
    pub brightness: u8,
}

impl ScenePreset {
    /// Creates a new scene preset from current parameters.
    pub fn new(name: &str, color: RgbColor, effect: u8, speed: u8, brightness: u8) -> Self {
        Self {
            version: 1,
            name: name.to_string(),
            color,
            effect,
            speed,
            brightness,
        }
    }
}

/// High-level device controller managing connection, command queuing, and state synchronization.
pub struct DeviceManager {
    transport: Arc<Mutex<Box<dyn Transport>>>,
    state: Arc<RwLock<DeviceState>>,
    running: Arc<AtomicBool>,
    last_write: Arc<Mutex<Option<std::time::Instant>>>,
}

impl DeviceManager {
    /// Constructs a new device manager wrapping a given transport backend.
    pub fn new(transport: Box<dyn Transport>) -> Self {
        Self {
            transport: Arc::new(Mutex::new(transport)),
            state: Arc::new(RwLock::new(DeviceState::default())),
            running: Arc::new(AtomicBool::new(true)),
            last_write: Arc::new(Mutex::new(None)),
        }
    }

    /// Paces outbound writes to prevent Bluetooth buffer drops and rate violations.
    async fn write_paced(&self, t: &mut dyn Transport, bytes: &[u8]) -> Result<(), CoreError> {
        let mut last_guard = self.last_write.lock().await;
        if let Some(prev) = *last_guard {
            let elapsed = prev.elapsed();
            let min_interval = std::time::Duration::from_millis(30);
            if elapsed < min_interval {
                tokio::time::sleep(min_interval - elapsed).await;
            }
        }
        t.write(bytes).await?;
        *last_guard = Some(std::time::Instant::now());
        Ok(())
    }

    /// Returns a thread-safe handle to the current device state.
    pub fn state(&self) -> Arc<RwLock<DeviceState>> {
        self.state.clone()
    }

    /// Discovers nearby compatible controllers.
    pub async fn scan(&self) -> Result<Vec<DiscoveredDevice>, CoreError> {
        let t = self.transport.lock().await;
        Ok(t.scan().await?)
    }

    /// Establishes a connection to the specified device and begins background notification listening.
    pub async fn connect(&self, address: &str) -> Result<(), CoreError> {
        let mut t = self.transport.lock().await;
        t.connect(address).await?;

        {
            let mut s = self.state.write().await;
            s.is_connected = true;
            s.address = Some(address.to_string());
        }

        // Subscribe to notifications and spawn background listener
        if let Ok(mut rx) = t.subscribe().await {
            let state_clone = self.state.clone();
            let running_clone = self.running.clone();

            tokio::spawn(async move {
                let mut assembler = StatusAssembler::new();
                while running_clone.load(Ordering::Relaxed) {
                    match rx.recv().await {
                        Some(chunk) => {
                            if let Ok(Some(status)) = assembler.feed(&chunk) {
                                let mut s = state_clone.write().await;
                                s.update_from_status(&status);
                            }
                        }
                        None => break,
                    }
                }
            });
        }

        // Query initial state
        let query_cmd = Command::QueryState.encode();
        self.write_paced(t.as_mut(), &query_cmd).await?;

        Ok(())
    }

    /// Disconnects the active controller.
    pub async fn disconnect(&self) -> Result<(), CoreError> {
        let mut t = self.transport.lock().await;
        t.disconnect().await?;
        let mut s = self.state.write().await;
        s.is_connected = false;
        s.address = None;
        Ok(())
    }

    /// Sends a strongly-typed command to the controller.
    pub async fn send_command(&self, cmd: Command) -> Result<(), CoreError> {
        let encoded = cmd.encode();
        let mut t = self.transport.lock().await;
        self.write_paced(t.as_mut(), &encoded).await?;

        // Update local state snapshot
        {
            let mut s = self.state.write().await;
            match &cmd {
                Command::Power(on) => s.power = *on,
                Command::SetColor { color, level } => {
                    s.color = *color;
                    s.brightness = *level;
                    s.effect = EFFECT_SOLID;
                }
                Command::SetBrightness(b) => s.brightness = *b,
                Command::SetSpeed(sp) => s.speed = *sp,
                Command::SetLength(len) => s.length = *len,
                Command::SetEffect(eff) => s.effect = *eff,
                Command::SetChipOrder(ord) => s.chip_order = *ord,
                Command::SetLightMode(m) => s.mode = *m,
                Command::SetSensitivity(sens) => s.sensitivity = *sens,
                Command::SetAudioInput(inp) => s.audio_input = *inp,
                _ => {}
            }
        }

        Ok(())
    }

    /// Applies a scene preset to the active device.
    pub async fn apply_preset(&self, preset: &ScenePreset) -> Result<(), CoreError> {
        self.send_command(Command::SetEffect(preset.effect)).await?;
        self.send_command(Command::SetSpeed(preset.speed)).await?;
        self.send_command(Command::SetColor {
            color: preset.color,
            level: preset.brightness,
        })
        .await?;
        Ok(())
    }
}

impl Drop for DeviceManager {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

/// Preset storage manager saving versioned JSON presets in system configuration folders.
pub struct PresetStore {
    base_dir: PathBuf,
}

impl PresetStore {
    /// Initializes preset storage in standard OS data directories.
    pub fn new() -> Self {
        let base_dir = if let Some(proj_dirs) =
            directories::ProjectDirs::from("org", "pyxillum", "Pyxillum")
        {
            proj_dirs.config_dir().to_path_buf()
        } else {
            PathBuf::from(".pyxillum")
        };

        let _ = std::fs::create_dir_all(&base_dir);
        Self { base_dir }
    }

    /// Saves a preset to disk.
    pub fn save_preset(&self, preset: &ScenePreset) -> Result<(), CoreError> {
        let path = self.base_dir.join(format!("{}.json", preset.name));
        let data = serde_json::to_string_pretty(preset)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Loads a preset from disk.
    pub fn load_preset(&self, name: &str) -> Result<ScenePreset, CoreError> {
        let path = self.base_dir.join(format!("{}.json", name));
        if !path.exists() {
            return Err(CoreError::PresetNotFound(name.to_string()));
        }
        let data = std::fs::read_to_string(path)?;
        let preset = serde_json::from_str(&data)?;
        Ok(preset)
    }

    /// Returns default starter presets.
    pub fn default_presets() -> Vec<ScenePreset> {
        vec![
            ScenePreset::new(
                "Cozy Amber",
                RgbColor::new(255, 120, 20),
                EFFECT_SOLID,
                5,
                200,
            ),
            ScenePreset::new(
                "Nordic Frost",
                RgbColor::new(100, 200, 255),
                EFFECT_SOLID,
                5,
                180,
            ),
            ScenePreset::new("Party Rainbow", RgbColor::WHITE, 0x01, 8, 255),
            ScenePreset::new("Night Dim", RgbColor::new(255, 40, 0), EFFECT_SOLID, 1, 30),
        ]
    }
}

impl Default for PresetStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyxillum_sim::DeviceSimulator;
    use pyxillum_transport::SimTransport;

    #[tokio::test]
    async fn test_device_manager_with_simulator() {
        let sim = Arc::new(Mutex::new(DeviceSimulator::new()));
        sim.lock()
            .await
            .set_min_write_interval(std::time::Duration::ZERO);
        let transport = Box::new(SimTransport::new(sim.clone()));
        let manager = DeviceManager::new(transport);

        let devices = manager.scan().await.unwrap();
        assert!(!devices.is_empty());

        manager.connect(&devices[0].address).await.unwrap();
        assert!(manager.state().read().await.is_connected);

        manager.send_command(Command::Power(false)).await.unwrap();
        assert!(!manager.state().read().await.power);
        assert!(!sim.lock().await.power);

        let preset = ScenePreset::new("Test Blue", RgbColor::new(0, 0, 255), EFFECT_SOLID, 6, 150);
        manager.apply_preset(&preset).await.unwrap();

        let state_lock = manager.state();
        let s = state_lock.read().await;
        assert_eq!(s.color, RgbColor::new(0, 0, 255));
        assert_eq!(s.brightness, 150);
    }
}
