//! # Immutable Application State Snapshot
//!
//! Provides the complete, immutable state snapshot of Pyxillum consumed by front-end
//! presentation layers.

use crate::catalog::EffectCategory;
use crate::{DeviceState, ScenePreset};
use pyxillum_protocol::{AudioInput, ChipOrder, RgbColor};
use pyxillum_transport::DiscoveredDevice;
use serde::{Deserialize, Serialize};

/// Comprehensive connection status of the controller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ConnectionStatus {
    /// No device connected; Bluetooth subsystem is idle.
    #[default]
    Disconnected,
    /// Active Bluetooth Low Energy background advertisement scanning is in progress.
    Scanning,
    /// Connection attempt to a specific peripheral address is underway.
    Connecting {
        /// Bluetooth address or identifier.
        address: String,
    },
    /// Fully connected and synchronized with controller GATT services.
    Connected {
        /// Active peripheral address.
        address: String,
        /// Human-readable device name.
        name: String,
    },
    /// Communication failure or connection aborted.
    Error(String),
}

/// Strongly typed user-facing error classification keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorKey {
    /// Bluetooth adapter is turned off or missing on host.
    BluetoothUnavailable,
    /// Scanning for devices encountered an operating system error.
    ScanFailed,
    /// Establishing connection to peripheral failed.
    ConnectionFailed,
    /// Peripheral connection was lost unexpectedly.
    DeviceDisconnected,
    /// Writing unacknowledged packet or reading characteristic failed.
    CommandFailed,
    /// Modifying non-volatile registers requires explicit safety confirmation.
    SafetyGateRequiresConfirmation,
    /// Requested lighting preset was not found in storage.
    PresetNotFound,
    /// Writing presets or configuration to disk failed.
    PersistenceFailed,
    /// A numeric or geometric parameter was out of valid range.
    InvalidParameter,
}

impl ErrorKey {
    /// Returns the localization message key string.
    pub fn message_key(&self) -> &'static str {
        match self {
            Self::BluetoothUnavailable => "error.bluetooth_unavailable",
            Self::ScanFailed => "error.scan_failed",
            Self::ConnectionFailed => "error.connection_failed",
            Self::DeviceDisconnected => "error.device_disconnected",
            Self::CommandFailed => "error.command_failed",
            Self::SafetyGateRequiresConfirmation => "error.safety_gate_required",
            Self::PresetNotFound => "error.preset_not_found",
            Self::PersistenceFailed => "error.persistence_failed",
            Self::InvalidParameter => "error.invalid_parameter",
        }
    }

    /// Default fallback message in British English.
    pub fn default_message(&self) -> &'static str {
        match self {
            Self::BluetoothUnavailable => "Bluetooth adapter is powered off or unavailable.",
            Self::ScanFailed => "Failed to scan for Bluetooth Low Energy controllers.",
            Self::ConnectionFailed => "Unable to establish GATT connection with controller.",
            Self::DeviceDisconnected => "Controller connection lost.",
            Self::CommandFailed => "Failed to deliver command to controller.",
            Self::SafetyGateRequiresConfirmation => {
                "Safety confirmation required to alter non-volatile hardware configuration."
            }
            Self::PresetNotFound => "Requested scene preset not found.",
            Self::PersistenceFailed => "Failed to persist configuration to local storage.",
            Self::InvalidParameter => "Specified parameter value is out of permitted range.",
        }
    }
}

/// Typed user-facing error representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserError {
    /// Classification error key for localization lookups.
    pub key: ErrorKey,
    /// Detailed diagnostic message.
    pub message: String,
}

impl UserError {
    /// Constructs a user error with default localized message.
    pub fn from_key(key: ErrorKey) -> Self {
        Self {
            key,
            message: key.default_message().to_string(),
        }
    }

    /// Constructs a user error with a custom message.
    pub fn new(key: ErrorKey, message: impl Into<String>) -> Self {
        Self {
            key,
            message: message.into(),
        }
    }
}

/// Top-level view or screen identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ScreenId {
    /// Device discovery, pairing, and connection management.
    #[default]
    DeviceHub,
    /// 146+ dynamic addressable SPI lighting patterns.
    Dynamic,
    /// Solid chromatic colour disc, temperature, and single-colour effects.
    Solid,
    /// 18 sound-reactive music algorithms and spectrum visualizer.
    Music,
    /// 14-node DIY custom chase and gradient sequencer.
    Diy,
    /// Non-volatile controller hardware settings (IC, chip sequence, pixel length).
    HardwareSettings,
}

/// Single-colour dynamic effect styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CustomColorEffect {
    /// Steady continuous illumination.
    #[default]
    Solid,
    /// Sinusoidal breathing fade.
    Breathe,
    /// Rhythmic flash bursts.
    Strobe,
    /// Double-pulse radial heartbeat.
    Pulse,
    /// Undulating sine-wave traverse.
    Wave,
    /// High-velocity comet head with decaying tail.
    Comet,
    /// Sparkling twinkling stars on dark canvas.
    Stars,
    /// Spatial hue sweep centered on chosen colour.
    Sweep,
}

impl CustomColorEffect {
    /// Returns the display label of the effect.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Solid => "Solid",
            Self::Breathe => "Breathe",
            Self::Strobe => "Strobe",
            Self::Pulse => "Pulse",
            Self::Wave => "Wave",
            Self::Comet => "Comet",
            Self::Stars => "Stars",
            Self::Sweep => "Sweep",
        }
    }

    /// Returns a short technical description of the effect.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Solid => "Continuous steady illumination of custom colour",
            Self::Breathe => "Smooth sinusoidal breathing fade (15% to 100%)",
            Self::Strobe => "High-energy transient flash bursts",
            Self::Pulse => "Heartbeat double-pulse radial expansion",
            Self::Wave => "Sinusoidal undulating wave traversing strip",
            Self::Comet => "High-velocity comet head with smooth fading tail",
            Self::Stars => "Sparkling twinkling star embers across dark canvas",
            Self::Sweep => "Spatial hue oscillation centered on custom colour",
        }
    }

    /// Returns all available custom colour effects.
    pub fn all() -> &'static [CustomColorEffect] {
        &[
            Self::Solid,
            Self::Breathe,
            Self::Strobe,
            Self::Pulse,
            Self::Wave,
            Self::Comet,
            Self::Stars,
            Self::Sweep,
        ]
    }
}

/// DIY pattern sequencing styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DiyPatternStyle {
    /// Sequential segment stepping.
    #[default]
    Chase,
    /// Flowing chromatic gradient liquid.
    Flow,
    /// Synchronous multi-node breathing.
    Breathe,
    /// Undulating wave travel.
    Wave,
    /// High-velocity meteor trail.
    Comet,
    /// Rapid flash switching.
    Strobe,
    /// Sparkling star embers.
    Sparkle,
    /// Static mapping across physical LEDs.
    Static,
}

impl DiyPatternStyle {
    /// Returns all DIY pattern styles.
    pub fn all() -> &'static [DiyPatternStyle] {
        &[
            Self::Chase,
            Self::Flow,
            Self::Breathe,
            Self::Wave,
            Self::Comet,
            Self::Strobe,
            Self::Sparkle,
            Self::Static,
        ]
    }

    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Chase => "Chase",
            Self::Flow => "Gradient Flow",
            Self::Breathe => "Breathe",
            Self::Wave => "Colour Wave",
            Self::Comet => "Meteor / Comet",
            Self::Strobe => "Strobe Flash",
            Self::Sparkle => "Twinkle Stars",
            Self::Static => "Static Map",
        }
    }

    /// Technical description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Chase => "Segments step sequentially along the pixel strip",
            Self::Flow => "Continuous smooth flowing liquid gradient of custom nodes",
            Self::Breathe => "Synchronous breathing pulsation across custom palette nodes",
            Self::Wave => "Undulating sine-wave travel with custom chromatic peaks",
            Self::Comet => "High-velocity comets with fading phosphor decay tails",
            Self::Strobe => "Instantaneous rhythmic flash switching between colour nodes",
            Self::Sparkle => "Random ambient glittering stars in your custom palette",
            Self::Static => "Custom palette mapped statically across the physical LEDs",
        }
    }
}

/// Transition mode between DIY nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DiyEffectType {
    /// Hard step transitions between palette nodes.
    #[default]
    Solid,
    /// Smooth interpolation between palette nodes.
    Gradient,
}

/// Physical strip configuration parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StripConfig {
    /// RGB colour channel ordering on physical LEDs.
    pub chip_order: ChipOrder,
    /// Configured length of the physical strip (1–150).
    pub length: u8,
    /// Number of virtual LEDs rendered in preview strip.
    pub preview_pixel_count: usize,
}

impl Default for StripConfig {
    fn default() -> Self {
        Self {
            chip_order: ChipOrder::Grb,
            length: 30,
            preview_pixel_count: 60,
        }
    }
}

/// State of the Dynamic Lighting screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicPageState {
    /// Active dynamic pattern code (1–146).
    pub selected_effect: u8,
    /// Active filter category in the drawer.
    pub selected_category: Option<EffectCategory>,
    /// Search keyword query for filtering presets.
    pub search_query: String,
    /// Whether cycle through all effects continuously.
    pub loop_all: bool,
}

impl Default for DynamicPageState {
    fn default() -> Self {
        Self {
            selected_effect: 1,
            selected_category: None,
            search_query: String::new(),
            loop_all: false,
        }
    }
}

/// State of the Solid Colour screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolidPageState {
    /// Currently selected primary solid RGB colour.
    pub current_color: RgbColor,
    /// Active dynamic effect applied to chosen colour.
    pub active_custom_effect: CustomColorEffect,
    /// Correlated Colour Temperature in Kelvin (2700K - 6500K).
    pub color_temperature_k: u32,
    /// Recent palette history tiles.
    pub recent_colors: Vec<RgbColor>,
}

impl Default for SolidPageState {
    fn default() -> Self {
        Self {
            current_color: RgbColor::new(255, 120, 0),
            active_custom_effect: CustomColorEffect::Solid,
            color_temperature_k: 4000,
            recent_colors: vec![
                RgbColor::new(255, 0, 0),
                RgbColor::new(0, 255, 0),
                RgbColor::new(0, 100, 255),
                RgbColor::new(255, 215, 0),
                RgbColor::new(195, 0, 255),
                RgbColor::new(255, 255, 255),
            ],
        }
    }
}

/// State of the Music Reactive screen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MusicPageState {
    /// Selected music mode pattern index (1–18).
    pub selected_music_mode: u8,
    /// Selected audio source on controller (Internal Mic vs Line/App).
    pub audio_input: AudioInput,
    /// Acoustic sensitivity factor (1–16).
    pub sensitivity: u8,
    /// Whether software microphone capture is currently enabled.
    pub is_mic_active: bool,
    /// Real-time 16-band normalized FFT energy magnitudes (0.0 to 1.0).
    pub spectrum_bands: [f32; 16],
}

impl Default for MusicPageState {
    fn default() -> Self {
        Self {
            selected_music_mode: 1,
            audio_input: AudioInput::InternalMic,
            sensitivity: 8,
            is_mic_active: false,
            spectrum_bands: [0.0; 16],
        }
    }
}

/// State of the DIY Custom Sequencer screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiyPageState {
    /// Sequenced colour nodes (up to 14 nodes).
    pub steps: Vec<RgbColor>,
    /// Currently focused node index for editing.
    pub active_step_index: usize,
    /// Chase algorithm style.
    pub pattern_style: DiyPatternStyle,
    /// Node interpolation mode (Solid vs Gradient).
    pub effect_type: DiyEffectType,
    /// Whether sequence animation preview is actively playing.
    pub is_playing: bool,
}

impl Default for DiyPageState {
    fn default() -> Self {
        Self {
            steps: vec![
                RgbColor::new(255, 0, 60),
                RgbColor::new(255, 120, 0),
                RgbColor::new(0, 220, 255),
                RgbColor::new(140, 0, 255),
            ],
            active_step_index: 0,
            pattern_style: DiyPatternStyle::Chase,
            effect_type: DiyEffectType::Solid,
            is_playing: true,
        }
    }
}

/// Saved window position, dimensions, and maximize state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowStateData {
    /// Top-left horizontal coordinate in screen pixels.
    pub x: f32,
    /// Top-left vertical coordinate in screen pixels.
    pub y: f32,
    /// Window width in screen pixels.
    pub width: f32,
    /// Window height in screen pixels.
    pub height: f32,
    /// Whether the window was maximized.
    pub is_maximized: bool,
}

impl Default for WindowStateData {
    fn default() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            width: 1080.0,
            height: 720.0,
            is_maximized: false,
        }
    }
}

/// Immutable, complete snapshot of Pyxillum application state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    /// Active Bluetooth or simulation connection status.
    pub connection_status: ConnectionStatus,
    /// Whether the host machine's Bluetooth adapter is powered and accessible.
    pub bluetooth_available: bool,
    /// Discovered peripheral controllers during scan.
    pub discovered_devices: Vec<DiscoveredDevice>,
    /// Active controller hardware state.
    pub device_state: DeviceState,
    /// Whether running in virtual simulator / demo mode.
    pub preview_mode: bool,
    /// Active view or screen ID.
    pub active_screen: ScreenId,
    /// Dynamic lighting page parameters.
    pub dynamic_page: DynamicPageState,
    /// Solid chromatic colour page parameters.
    pub solid_page: SolidPageState,
    /// Music reactive page parameters.
    pub music_page: MusicPageState,
    /// DIY sequencer page parameters.
    pub diy_page: DiyPageState,
    /// Physical strip configuration.
    pub strip_config: StripConfig,
    /// Saved user lighting scene presets.
    pub saved_presets: Vec<ScenePreset>,
    /// Persisted window rectangle and state.
    pub window_state: WindowStateData,
    /// Active user-facing notification or error.
    pub user_error: Option<UserError>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            connection_status: ConnectionStatus::Disconnected,
            bluetooth_available: true,
            discovered_devices: Vec::new(),
            device_state: DeviceState::default(),
            preview_mode: false,
            active_screen: ScreenId::DeviceHub,
            dynamic_page: DynamicPageState::default(),
            solid_page: SolidPageState::default(),
            music_page: MusicPageState::default(),
            diy_page: DiyPageState::default(),
            strip_config: StripConfig::default(),
            saved_presets: Vec::new(),
            window_state: WindowStateData::default(),
            user_error: None,
        }
    }
}

impl AppState {
    /// Returns true if a controller (hardware or simulator) is currently connected.
    pub fn is_connected(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected { .. })
    }

    /// Returns the currently active solid or dynamic RGB colour.
    pub fn current_color(&self) -> RgbColor {
        self.device_state.color
    }

    /// Returns the active master brightness level (0–255).
    pub fn brightness(&self) -> u8 {
        self.device_state.brightness
    }

    /// Returns the active animation speed (1–10).
    pub fn speed(&self) -> u8 {
        self.device_state.speed
    }
}
