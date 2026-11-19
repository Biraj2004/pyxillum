//! # Pyxillum UniFFI Bindings Surface
//!
//! Exposes Pyxillum Core (Engine, Action, AppState, StripFrame, PlatformServices)
//! to native host languages (C# on Windows, Swift on macOS) via Mozilla UniFFI.

use pyxillum_core::{
    Action, AppState, ConnectionStatus, CoreError, CustomColorEffect, DiyEffectType,
    DiyPatternStyle, Engine, PlatformServices, ScreenId, StateListener, StripConfig, UserError,
    WindowStateData,
};
use pyxillum_protocol::{AudioInput, ChipOrder, RgbColor};
use pyxillum_transport::DiscoveredDevice;
use std::sync::Arc;

uniffi::setup_scaffolding!();

/// Returns the semantic version of the Pyxillum native FFI interface.
#[uniffi::export]
pub fn ffi_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ---------------------------------------------------------------------------
// Callbacks (Host Platform -> Rust Core)
// ---------------------------------------------------------------------------

/// Host platform operating system capability callbacks.
#[uniffi::export(callback_interface)]
pub trait PlatformServicesCallback: Send + Sync {
    /// Opens the operating system's native Bluetooth settings pane.
    fn open_bluetooth_settings(&self) -> bool;

    /// Returns the standard local application data storage path.
    fn data_dir(&self) -> String;

    /// Queries or requests host audio capture permissions.
    fn request_audio_permission(&self) -> bool;
}

struct PlatformServicesBridge {
    callback: Box<dyn PlatformServicesCallback>,
}

impl PlatformServices for PlatformServicesBridge {
    fn open_bluetooth_settings(&self) -> Result<(), CoreError> {
        if self.callback.open_bluetooth_settings() {
            Ok(())
        } else {
            Err(CoreError::Io(std::io::Error::other(
                "Failed to open host Bluetooth settings pane",
            )))
        }
    }

    fn data_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(self.callback.data_dir())
    }

    fn request_audio_permission(&self) -> Result<bool, CoreError> {
        Ok(self.callback.request_audio_permission())
    }
}

/// Listener interface receiving coalesced application state snapshots from the engine.
#[uniffi::export(callback_interface)]
pub trait StateListenerCallback: Send + Sync {
    /// Dispatched whenever state is mutated, coalesced to at most once per frame period (~60 Hz).
    fn on_state_changed(&self, state: AppStateFfi);
}

struct StateListenerBridge {
    callback: Box<dyn StateListenerCallback>,
}

impl StateListener for StateListenerBridge {
    fn on_state_changed(&self, state: &AppState) {
        self.callback.on_state_changed(state.clone().into());
    }
}

// ---------------------------------------------------------------------------
// Enums & Records
// ---------------------------------------------------------------------------

/// Primary top-level navigation screens.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenIdFfi {
    DeviceHub,
    Dynamic,
    Solid,
    Music,
    Diy,
    HardwareSettings,
}

impl From<ScreenId> for ScreenIdFfi {
    fn from(s: ScreenId) -> Self {
        match s {
            ScreenId::DeviceHub => ScreenIdFfi::DeviceHub,
            ScreenId::Dynamic => ScreenIdFfi::Dynamic,
            ScreenId::Solid => ScreenIdFfi::Solid,
            ScreenId::Music => ScreenIdFfi::Music,
            ScreenId::Diy => ScreenIdFfi::Diy,
            ScreenId::HardwareSettings => ScreenIdFfi::HardwareSettings,
        }
    }
}

impl From<ScreenIdFfi> for ScreenId {
    fn from(s: ScreenIdFfi) -> Self {
        match s {
            ScreenIdFfi::DeviceHub => ScreenId::DeviceHub,
            ScreenIdFfi::Dynamic => ScreenId::Dynamic,
            ScreenIdFfi::Solid => ScreenId::Solid,
            ScreenIdFfi::Music => ScreenId::Music,
            ScreenIdFfi::Diy => ScreenId::Diy,
            ScreenIdFfi::HardwareSettings => ScreenId::HardwareSettings,
        }
    }
}

/// Device connection state snapshot.
#[derive(uniffi::Enum, Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatusFfi {
    Disconnected,
    Scanning,
    Connecting { address: String },
    Connected { address: String, name: String },
    Error { message: String },
}

impl From<ConnectionStatus> for ConnectionStatusFfi {
    fn from(c: ConnectionStatus) -> Self {
        match c {
            ConnectionStatus::Disconnected => ConnectionStatusFfi::Disconnected,
            ConnectionStatus::Scanning => ConnectionStatusFfi::Scanning,
            ConnectionStatus::Connecting { address } => ConnectionStatusFfi::Connecting { address },
            ConnectionStatus::Connected { address, name } => {
                ConnectionStatusFfi::Connected { address, name }
            }
            ConnectionStatus::Error(msg) => ConnectionStatusFfi::Error { message: msg },
        }
    }
}

/// Colour channel ordering for pixel LED IC drivers.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipOrderFfi {
    Rgb,
    Rbg,
    Grb,
    Gbr,
    Brg,
    Bgr,
}

impl From<ChipOrder> for ChipOrderFfi {
    fn from(o: ChipOrder) -> Self {
        match o {
            ChipOrder::Rgb => ChipOrderFfi::Rgb,
            ChipOrder::Rbg => ChipOrderFfi::Rbg,
            ChipOrder::Grb => ChipOrderFfi::Grb,
            ChipOrder::Gbr => ChipOrderFfi::Gbr,
            ChipOrder::Brg => ChipOrderFfi::Brg,
            ChipOrder::Bgr => ChipOrderFfi::Bgr,
        }
    }
}

impl From<ChipOrderFfi> for ChipOrder {
    fn from(o: ChipOrderFfi) -> Self {
        match o {
            ChipOrderFfi::Rgb => ChipOrder::Rgb,
            ChipOrderFfi::Rbg => ChipOrder::Rbg,
            ChipOrderFfi::Grb => ChipOrder::Grb,
            ChipOrderFfi::Gbr => ChipOrder::Gbr,
            ChipOrderFfi::Brg => ChipOrder::Brg,
            ChipOrderFfi::Bgr => ChipOrder::Bgr,
        }
    }
}

/// Acoustic capture input source.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioInputFfi {
    Player,
    InternalMic,
    ExternalMic,
}

impl From<AudioInput> for AudioInputFfi {
    fn from(a: AudioInput) -> Self {
        match a {
            AudioInput::Player => AudioInputFfi::Player,
            AudioInput::InternalMic => AudioInputFfi::InternalMic,
            AudioInput::ExternalMic => AudioInputFfi::ExternalMic,
        }
    }
}

impl From<AudioInputFfi> for AudioInput {
    fn from(a: AudioInputFfi) -> Self {
        match a {
            AudioInputFfi::Player => AudioInput::Player,
            AudioInputFfi::InternalMic => AudioInput::InternalMic,
            AudioInputFfi::ExternalMic => AudioInput::ExternalMic,
        }
    }
}

/// Custom solid colour animation mode.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomColorEffectFfi {
    Solid,
    Breathe,
    Strobe,
    Pulse,
    Wave,
    Comet,
    Stars,
    Sweep,
}

impl From<CustomColorEffect> for CustomColorEffectFfi {
    fn from(e: CustomColorEffect) -> Self {
        match e {
            CustomColorEffect::Solid => CustomColorEffectFfi::Solid,
            CustomColorEffect::Breathe => CustomColorEffectFfi::Breathe,
            CustomColorEffect::Strobe => CustomColorEffectFfi::Strobe,
            CustomColorEffect::Pulse => CustomColorEffectFfi::Pulse,
            CustomColorEffect::Wave => CustomColorEffectFfi::Wave,
            CustomColorEffect::Comet => CustomColorEffectFfi::Comet,
            CustomColorEffect::Stars => CustomColorEffectFfi::Stars,
            CustomColorEffect::Sweep => CustomColorEffectFfi::Sweep,
        }
    }
}

impl From<CustomColorEffectFfi> for CustomColorEffect {
    fn from(e: CustomColorEffectFfi) -> Self {
        match e {
            CustomColorEffectFfi::Solid => CustomColorEffect::Solid,
            CustomColorEffectFfi::Breathe => CustomColorEffect::Breathe,
            CustomColorEffectFfi::Strobe => CustomColorEffect::Strobe,
            CustomColorEffectFfi::Pulse => CustomColorEffect::Pulse,
            CustomColorEffectFfi::Wave => CustomColorEffect::Wave,
            CustomColorEffectFfi::Comet => CustomColorEffect::Comet,
            CustomColorEffectFfi::Stars => CustomColorEffect::Stars,
            CustomColorEffectFfi::Sweep => CustomColorEffect::Sweep,
        }
    }
}

/// DIY pattern sequencing styles.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiyPatternStyleFfi {
    Chase,
    Flow,
    Breathe,
    Wave,
    Comet,
    Strobe,
    Sparkle,
    Static,
}

impl From<DiyPatternStyle> for DiyPatternStyleFfi {
    fn from(s: DiyPatternStyle) -> Self {
        match s {
            DiyPatternStyle::Chase => DiyPatternStyleFfi::Chase,
            DiyPatternStyle::Flow => DiyPatternStyleFfi::Flow,
            DiyPatternStyle::Breathe => DiyPatternStyleFfi::Breathe,
            DiyPatternStyle::Wave => DiyPatternStyleFfi::Wave,
            DiyPatternStyle::Comet => DiyPatternStyleFfi::Comet,
            DiyPatternStyle::Strobe => DiyPatternStyleFfi::Strobe,
            DiyPatternStyle::Sparkle => DiyPatternStyleFfi::Sparkle,
            DiyPatternStyle::Static => DiyPatternStyleFfi::Static,
        }
    }
}

impl From<DiyPatternStyleFfi> for DiyPatternStyle {
    fn from(s: DiyPatternStyleFfi) -> Self {
        match s {
            DiyPatternStyleFfi::Chase => DiyPatternStyle::Chase,
            DiyPatternStyleFfi::Flow => DiyPatternStyle::Flow,
            DiyPatternStyleFfi::Breathe => DiyPatternStyle::Breathe,
            DiyPatternStyleFfi::Wave => DiyPatternStyle::Wave,
            DiyPatternStyleFfi::Comet => DiyPatternStyle::Comet,
            DiyPatternStyleFfi::Strobe => DiyPatternStyle::Strobe,
            DiyPatternStyleFfi::Sparkle => DiyPatternStyle::Sparkle,
            DiyPatternStyleFfi::Static => DiyPatternStyle::Static,
        }
    }
}

/// DIY sequence step transition type.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiyEffectTypeFfi {
    Solid,
    Gradient,
}

impl From<DiyEffectType> for DiyEffectTypeFfi {
    fn from(e: DiyEffectType) -> Self {
        match e {
            DiyEffectType::Solid => DiyEffectTypeFfi::Solid,
            DiyEffectType::Gradient => DiyEffectTypeFfi::Gradient,
        }
    }
}

impl From<DiyEffectTypeFfi> for DiyEffectType {
    fn from(e: DiyEffectTypeFfi) -> Self {
        match e {
            DiyEffectTypeFfi::Solid => DiyEffectType::Solid,
            DiyEffectTypeFfi::Gradient => DiyEffectType::Gradient,
        }
    }
}

/// Audio capture source target for real-time spectrum analysis.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSourceKindFfi {
    SystemLoopback,
    Microphone,
}

impl From<pyxillum_core::AudioSourceKind> for AudioSourceKindFfi {
    fn from(s: pyxillum_core::AudioSourceKind) -> Self {
        match s {
            pyxillum_core::AudioSourceKind::SystemLoopback => AudioSourceKindFfi::SystemLoopback,
            pyxillum_core::AudioSourceKind::Microphone => AudioSourceKindFfi::Microphone,
        }
    }
}

impl From<AudioSourceKindFfi> for pyxillum_core::AudioSourceKind {
    fn from(s: AudioSourceKindFfi) -> Self {
        match s {
            AudioSourceKindFfi::SystemLoopback => pyxillum_core::AudioSourceKind::SystemLoopback,
            AudioSourceKindFfi::Microphone => pyxillum_core::AudioSourceKind::Microphone,
        }
    }
}

/// User-facing error message with localizable error key.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct UserErrorFfi {
    pub message: String,
    pub key: String,
}

impl From<UserError> for UserErrorFfi {
    fn from(e: UserError) -> Self {
        UserErrorFfi {
            message: e.message,
            key: e.key.message_key().to_string(),
        }
    }
}

/// Hardware pixel configuration parameters.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct StripConfigFfi {
    pub length: u8,
    pub chip_order: ChipOrderFfi,
    pub preview_pixel_count: u32,
}

impl From<StripConfig> for StripConfigFfi {
    fn from(c: StripConfig) -> Self {
        StripConfigFfi {
            length: c.length,
            chip_order: c.chip_order.into(),
            preview_pixel_count: c.preview_pixel_count as u32,
        }
    }
}

/// Discovered Bluetooth Low Energy peripheral metadata.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredDeviceFfi {
    pub name: String,
    pub address: String,
    pub rssi: Option<i16>,
}

impl From<DiscoveredDevice> for DiscoveredDeviceFfi {
    fn from(d: DiscoveredDevice) -> Self {
        DiscoveredDeviceFfi {
            name: d
                .name
                .unwrap_or_else(|| "Unknown SP621E Device".to_string()),
            address: d.address,
            rssi: d.rssi,
        }
    }
}

/// Saved scene preset reference.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct ScenePresetFfi {
    pub name: String,
}

/// Saved window position, dimensions, and state.
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct WindowStateDataFfi {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_maximized: bool,
}

impl From<WindowStateData> for WindowStateDataFfi {
    fn from(w: WindowStateData) -> Self {
        WindowStateDataFfi {
            x: w.x,
            y: w.y,
            width: w.width,
            height: w.height,
            is_maximized: w.is_maximized,
        }
    }
}

/// Snapshot of the Solid colour page.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct SolidPageStateFfi {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub active_custom_effect: CustomColorEffectFfi,
    pub color_temperature_k: u32,
    pub recent_colors_rgb: Vec<u8>,
}

/// Snapshot of the Dynamic effect page.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct DynamicPageStateFfi {
    pub selected_effect: u8,
    pub search_query: String,
    pub loop_all: bool,
}

/// Snapshot of the Music reactive page.
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct MusicPageStateFfi {
    pub selected_music_mode: u8,
    pub audio_input: AudioInputFfi,
    pub sensitivity: u8,
    pub is_mic_active: bool,
    pub spectrum_bands: Vec<f32>,
}

/// Snapshot of the DIY sequence page.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct DiyPageStateFfi {
    pub active_step_index: u32,
    pub pattern_style: DiyPatternStyleFfi,
    pub effect_type: DiyEffectTypeFfi,
    pub is_playing: bool,
    /// Packed RGB bytes for each sequenced step.
    pub steps_rgb: Vec<u8>,
}

/// Complete immutable application state snapshot rendered by native front ends.
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct AppStateFfi {
    pub connection_status: ConnectionStatusFfi,
    pub bluetooth_available: bool,
    pub discovered_devices: Vec<DiscoveredDeviceFfi>,
    pub power_on: bool,
    pub brightness: u8,
    pub speed: u8,
    pub preview_mode: bool,
    pub active_screen: ScreenIdFfi,
    pub dynamic_page: DynamicPageStateFfi,
    pub solid_page: SolidPageStateFfi,
    pub music_page: MusicPageStateFfi,
    pub diy_page: DiyPageStateFfi,
    pub strip_config: StripConfigFfi,
    pub saved_presets: Vec<ScenePresetFfi>,
    pub window_state: WindowStateDataFfi,
    pub user_error: Option<UserErrorFfi>,
}

impl From<AppState> for AppStateFfi {
    fn from(s: AppState) -> Self {
        let mut diy_colors = Vec::with_capacity(s.diy_page.steps.len() * 3);
        for color in &s.diy_page.steps {
            diy_colors.push(color.r);
            diy_colors.push(color.g);
            diy_colors.push(color.b);
        }

        let mut recent_colors = Vec::with_capacity(s.solid_page.recent_colors.len() * 3);
        for color in &s.solid_page.recent_colors {
            recent_colors.push(color.r);
            recent_colors.push(color.g);
            recent_colors.push(color.b);
        }

        let presets = s
            .saved_presets
            .into_iter()
            .map(|p| ScenePresetFfi { name: p.name })
            .collect();

        let devices = s.discovered_devices.into_iter().map(Into::into).collect();

        AppStateFfi {
            connection_status: s.connection_status.into(),
            bluetooth_available: s.bluetooth_available,
            discovered_devices: devices,
            power_on: s.device_state.power,
            brightness: s.device_state.brightness,
            speed: s.device_state.speed,
            preview_mode: s.preview_mode,
            active_screen: s.active_screen.into(),
            dynamic_page: DynamicPageStateFfi {
                selected_effect: s.dynamic_page.selected_effect,
                search_query: s.dynamic_page.search_query,
                loop_all: s.dynamic_page.loop_all,
            },
            solid_page: SolidPageStateFfi {
                r: s.solid_page.current_color.r,
                g: s.solid_page.current_color.g,
                b: s.solid_page.current_color.b,
                active_custom_effect: s.solid_page.active_custom_effect.into(),
                color_temperature_k: s.solid_page.color_temperature_k,
                recent_colors_rgb: recent_colors,
            },
            music_page: MusicPageStateFfi {
                selected_music_mode: s.music_page.selected_music_mode,
                audio_input: s.music_page.audio_input.into(),
                sensitivity: s.music_page.sensitivity,
                is_mic_active: s.music_page.is_mic_active,
                spectrum_bands: s.music_page.spectrum_bands.to_vec(),
            },
            diy_page: DiyPageStateFfi {
                active_step_index: s.diy_page.active_step_index as u32,
                pattern_style: s.diy_page.pattern_style.into(),
                effect_type: s.diy_page.effect_type.into(),
                is_playing: s.diy_page.is_playing,
                steps_rgb: diy_colors,
            },
            strip_config: s.strip_config.into(),
            saved_presets: presets,
            window_state: s.window_state.into(),
            user_error: s.user_error.map(Into::into),
        }
    }
}

/// 60 FPS pixel strip rendering frame.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq)]
pub struct StripFrameFfi {
    pub length: u32,
    pub is_powered_on: bool,
    /// Packed RGB byte buffer (length = length * 3).
    pub pixels_rgb: Vec<u8>,
}

impl StripFrameFfi {
    /// Constructs an FFI strip frame from the core frame snapshot.
    pub fn from_frame(frame: pyxillum_core::StripFrame, is_powered_on: bool) -> Self {
        let length = frame.len() as u32;
        let mut pixels_rgb = Vec::with_capacity(frame.len() * 3);
        for c in frame.colors() {
            pixels_rgb.push(c.r);
            pixels_rgb.push(c.g);
            pixels_rgb.push(c.b);
        }
        Self {
            length,
            is_powered_on,
            pixels_rgb,
        }
    }
}

// ---------------------------------------------------------------------------
// Actions (Host Platform -> Rust Core)
// ---------------------------------------------------------------------------

/// Strongly-typed actions representing every user operation.
#[derive(uniffi::Enum, Debug, Clone)]
pub enum ActionFfi {
    StartScan,
    StopScan,
    Connect {
        address: String,
    },
    Disconnect,
    SetPreviewMode {
        enabled: bool,
    },
    TogglePreviewMode,
    QueryStatus,
    OpenBluetoothSettings,
    SelectScreen {
        screen: ScreenIdFfi,
    },
    SetPower {
        on: bool,
    },
    TogglePower,
    SetBrightness {
        value: u8,
    },
    SetSpeed {
        speed: u8,
    },
    SetSolidColor {
        r: u8,
        g: u8,
        b: u8,
    },
    SetCustomColorEffect {
        effect: CustomColorEffectFfi,
    },
    SetColorTemperature {
        kelvin: u32,
    },
    SetEffect {
        effect_code: u8,
    },
    NextEffect,
    PreviousEffect,
    SearchEffects {
        query: String,
    },
    ToggleLoopAll {
        loop_all: bool,
    },
    SetMusicMode {
        mode_index: u8,
    },
    SetAudioInput {
        input: AudioInputFfi,
    },
    SetSensitivity {
        sensitivity: u8,
    },
    SetMicActive {
        active: bool,
    },
    SetDiyStepColor {
        index: u32,
        r: u8,
        g: u8,
        b: u8,
    },
    SetDiyPatternStyle {
        style: DiyPatternStyleFfi,
    },
    SetDiyEffectType {
        effect_type: DiyEffectTypeFfi,
    },
    SetDiyActiveStep {
        step_index: u32,
    },
    ToggleDiyPlayback {
        is_playing: bool,
    },
    SaveDiyPreset {
        name: String,
    },
    LoadDiyPreset {
        name: String,
    },
    SavePreset {
        name: String,
    },
    ApplyPreset {
        name: String,
    },
    DeletePreset {
        name: String,
    },
    ConfigureHardware {
        chip_order: ChipOrderFfi,
        length: u8,
        confirmed: bool,
    },
    UpdateWindowState {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        is_maximized: bool,
    },
    DismissError,
}

impl From<ActionFfi> for Action {
    fn from(a: ActionFfi) -> Self {
        match a {
            ActionFfi::StartScan => Action::StartScan,
            ActionFfi::StopScan => Action::StopScan,
            ActionFfi::Connect { address } => Action::Connect { address },
            ActionFfi::Disconnect => Action::Disconnect,
            ActionFfi::SetPreviewMode { enabled } => Action::SetPreviewMode(enabled),
            ActionFfi::TogglePreviewMode => Action::TogglePreviewMode,
            ActionFfi::QueryStatus => Action::QueryStatus,
            ActionFfi::OpenBluetoothSettings => Action::OpenBluetoothSettings,
            ActionFfi::SelectScreen { screen } => Action::SelectScreen(screen.into()),
            ActionFfi::SetPower { on } => Action::SetPower(on),
            ActionFfi::TogglePower => Action::TogglePower,
            ActionFfi::SetBrightness { value } => Action::SetBrightness(value),
            ActionFfi::SetSpeed { speed } => Action::SetSpeed(speed),
            ActionFfi::SetSolidColor { r, g, b } => Action::SetSolidColor(RgbColor::new(r, g, b)),
            ActionFfi::SetCustomColorEffect { effect } => {
                Action::SetCustomColorEffect(effect.into())
            }
            ActionFfi::SetColorTemperature { kelvin } => Action::SetColorTemperature(kelvin),
            ActionFfi::SetEffect { effect_code } => Action::SetEffect(effect_code),
            ActionFfi::NextEffect => Action::NextEffect,
            ActionFfi::PreviousEffect => Action::PreviousEffect,
            ActionFfi::SearchEffects { query } => Action::SearchEffects(query),
            ActionFfi::ToggleLoopAll { loop_all } => Action::ToggleLoopAll(loop_all),
            ActionFfi::SetMusicMode { mode_index } => Action::SetMusicMode(mode_index),
            ActionFfi::SetAudioInput { input } => Action::SetAudioInput(input.into()),
            ActionFfi::SetSensitivity { sensitivity } => Action::SetSensitivity(sensitivity),
            ActionFfi::SetMicActive { active } => Action::SetMicActive(active),
            ActionFfi::SetDiyStepColor { index, r, g, b } => Action::SetDiyStepColor {
                index: index as usize,
                color: RgbColor::new(r, g, b),
            },
            ActionFfi::SetDiyPatternStyle { style } => Action::SetDiyPatternStyle(style.into()),
            ActionFfi::SetDiyEffectType { effect_type } => {
                Action::SetDiyEffectType(effect_type.into())
            }
            ActionFfi::SetDiyActiveStep { step_index } => {
                Action::SetDiyActiveStep(step_index as usize)
            }
            ActionFfi::ToggleDiyPlayback { is_playing } => Action::ToggleDiyPlayback(is_playing),
            ActionFfi::SaveDiyPreset { name } => Action::SaveDiyPreset { name },
            ActionFfi::LoadDiyPreset { name } => Action::LoadDiyPreset { name },
            ActionFfi::SavePreset { name } => Action::SavePreset { name },
            ActionFfi::ApplyPreset { name } => Action::ApplyPreset { name },
            ActionFfi::DeletePreset { name } => Action::DeletePreset { name },
            ActionFfi::ConfigureHardware {
                chip_order,
                length,
                confirmed,
            } => Action::ConfigureHardware {
                chip_order: chip_order.into(),
                length,
                confirmed,
            },
            ActionFfi::UpdateWindowState {
                x,
                y,
                width,
                height,
                is_maximized,
            } => Action::UpdateWindowState(WindowStateData {
                x,
                y,
                width,
                height,
                is_maximized,
            }),
            ActionFfi::DismissError => Action::DismissError,
        }
    }
}

// ---------------------------------------------------------------------------
// Engine Object Surface
// ---------------------------------------------------------------------------

/// Strongly-typed error classification for operations traversing the FFI boundary.
#[derive(uniffi::Error, thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum FfiError {
    #[error("Initialization failure: {message}")]
    InitializationFailed { message: String },
    #[error("Operation failed: {message}")]
    OperationFailed { message: String },
}

/// Configuration parameters for engine instantiation.
#[derive(uniffi::Record, Debug, Clone)]
pub struct EngineConfigFfi {
    /// Whether to run in in-memory simulation mode (zero hardware required).
    pub is_simulator: bool,
}

/// Thread-safe high-level Engine wrapper exported across the FFI boundary.
#[derive(uniffi::Object)]
pub struct EngineFfi {
    inner: Arc<Engine>,
}

#[uniffi::export]
impl EngineFfi {
    /// Dispatches a user action into the asynchronous command processor.
    pub fn dispatch(&self, action: ActionFfi) {
        self.inner.dispatch(action.into());
    }

    /// Obtains the latest 60 FPS pixel strip frame buffer for UI canvas rendering.
    pub fn frame(&self) -> StripFrameFfi {
        let is_powered_on = self.inner.snapshot().device_state.power;
        StripFrameFfi::from_frame(self.inner.frame(), is_powered_on)
    }

    /// Obtains an immediate snapshot of current application state.
    pub fn snapshot(&self) -> AppStateFfi {
        self.inner.snapshot().into()
    }

    /// Starts real-time acoustic capture from the specified audio source.
    pub fn start_audio(&self, source: AudioSourceKindFfi) -> Result<(), FfiError> {
        let engine = self.inner.clone();
        let src = source.into();
        self.inner
            .handle()
            .block_on(engine.start_audio(src))
            .map_err(|e| FfiError::OperationFailed {
                message: e.to_string(),
            })
    }

    /// Stops active acoustic capture.
    pub fn stop_audio(&self) {
        let engine = self.inner.clone();
        self.inner.handle().block_on(engine.stop_audio());
    }

    /// Returns the current 24-band audio spectrum energy levels (0.0..=1.0).
    pub fn audio_spectrum(&self) -> Vec<f32> {
        let engine = self.inner.clone();
        self.inner
            .handle()
            .block_on(engine.audio_spectrum())
            .to_vec()
    }
}

/// Instantiates and starts a new Engine instance wired to host platform services and listener.
#[uniffi::export]
pub fn create_engine(
    config: EngineConfigFfi,
    services: Box<dyn PlatformServicesCallback>,
    listener: Box<dyn StateListenerCallback>,
) -> Result<Arc<EngineFfi>, FfiError> {
    let platform = Arc::new(PlatformServicesBridge { callback: services });
    let state_listener = Arc::new(StateListenerBridge { callback: listener });

    let engine = if config.is_simulator {
        Engine::new_simulator(platform).map_err(|e| FfiError::InitializationFailed {
            message: e.to_string(),
        })?
    } else {
        Engine::new_ble(platform).map_err(|e| FfiError::InitializationFailed {
            message: e.to_string(),
        })?
    };

    engine.subscribe(state_listener);

    Ok(Arc::new(EngineFfi {
        inner: Arc::new(engine),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockServices;
    impl PlatformServicesCallback for MockServices {
        fn open_bluetooth_settings(&self) -> bool {
            true
        }
        fn data_dir(&self) -> String {
            std::env::temp_dir().to_string_lossy().to_string()
        }
        fn request_audio_permission(&self) -> bool {
            true
        }
    }

    struct MockListener {
        count: Arc<std::sync::atomic::AtomicUsize>,
    }
    impl StateListenerCallback for MockListener {
        fn on_state_changed(&self, _state: AppStateFfi) {
            self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    #[test]
    fn test_ffi_version() {
        assert_eq!(ffi_version(), "1.0.0");
    }

    #[test]
    fn test_ffi_contract_lifecycle() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let listener = Box::new(MockListener {
            count: count.clone(),
        });
        let services = Box::new(MockServices);

        let config = EngineConfigFfi { is_simulator: true };
        let engine = create_engine(config, services, listener).expect("Failed to create engine");

        // Verify initial snapshot
        let snap = engine.snapshot();
        assert!(snap.power_on);
        assert_eq!(snap.brightness, 255);

        // Verify frame generation
        let frame = engine.frame();
        assert!(frame.length > 0);
        assert_eq!(frame.pixels_rgb.len(), (frame.length * 3) as usize);

        // Dispatch action
        engine.dispatch(ActionFfi::SetBrightness { value: 120 });
        std::thread::sleep(std::time::Duration::from_millis(50));

        let updated = engine.snapshot();
        assert_eq!(updated.brightness, 120);

        // Audio start & stop
        assert!(engine
            .start_audio(AudioSourceKindFfi::SystemLoopback)
            .is_ok());
        let spec = engine.audio_spectrum();
        assert_eq!(spec.len(), 24);
        engine.stop_audio();
    }
}
