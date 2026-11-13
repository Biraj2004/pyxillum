//! # Application State and Command Types
//!
//! Strongly typed definitions for GUI navigation, device representation,
//! command routing channels, and interactive control states.

use pyxillum_protocol::{AudioInput, ChipOrder, Command, DeviceStatus, RgbColor};
use pyxillum_transport::DiscoveredDevice;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum GuiCommand {
    Scan,
    Connect(String),
    Disconnect,
    SendCommand(Command),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum GuiEvent {
    ScanStarted,
    ScanFinished(Vec<DiscoveredDevice>),
    Connected { address: String, name: String },
    Disconnected,
    StatusUpdated(DeviceStatus),
    Error(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LogEntry {
    pub timestamp: String,
    pub message: String,
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    DeviceHub,
    Dashboard(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardMode {
    Dynamic,
    Solid,
    Music,
    Diy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiyEffectType {
    Solid,
    Gradient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiyChaseMode {
    Forward,
    Backward,
    Middle,
    Out,
}

#[derive(Debug, Clone)]
pub struct DiyNode {
    pub color: RgbColor,
    pub value: u8,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeviceCardInfo {
    pub id: String,
    pub name: String,
    pub model: String,
    pub protocol_type: String,
    pub is_simulated: bool,
    pub is_connected: bool,
    pub rssi: i8,
}

/// Consolidated state model for Pyxillum UI
pub struct AppState {
    // Navigation
    pub current_screen: AppScreen,
    pub dashboard_mode: DashboardMode,
    #[allow(dead_code)]
    pub hub_tab_devices: bool,
    pub active_device_id: String,

    // Modal and Drawer Visibility
    pub show_settings_modal: bool,
    pub show_help_modal: bool,
    pub show_effect_drawer: bool,
    pub effect_search_query: String,
    pub effect_category_filter: Option<crate::catalog::EffectCategory>,

    // Dynamic Mode Controls
    pub ui_power: bool,
    pub ui_effect: u8,
    pub ui_speed: u8,
    pub ui_brightness: u8,
    pub is_playing_dynamic: bool,
    pub is_loop_all: bool,
    pub is_favourite: bool,

    // Solid Mode Controls
    pub ui_color: [f32; 3],
    pub ui_color_temp: f32, // 0.0 = Warm Amber (2700K), 1.0 = Cool White (6500K)

    // Music Mode Controls
    pub music_mode_index: u8, // 1..18
    pub ui_audio_input: AudioInput,
    pub ui_sensitivity: u8,
    pub sync_with_windows_audio: bool,

    // DIY Mode Controls
    pub diy_effect_type: DiyEffectType,
    pub diy_chase_mode: DiyChaseMode,
    pub diy_nodes: Vec<DiyNode>,
    pub selected_diy_index: Option<usize>,
    pub diy_speed: u8,
    pub diy_brightness: u8,

    // Hardware Parameters (Safety Gated)
    pub ui_length: u8,
    pub ui_chip_order: ChipOrder,
    pub ui_confirm_safety: bool,

    // Real-Time Strip Monitoring & Audio
    pub strip_pixels: Vec<RgbColor>,
    pub animation_timer: f32,
    pub audio_visualizer_timer: f32,
    pub live_audio_peak: f32,

    // Real BLE Connection & Scanning Status
    pub is_connected: bool,
    pub is_scanning: bool,
    pub status_message: String,
}

impl Default for AppState {
    fn default() -> Self {
        // Initialize default 14 nodes matching the companion application palette grid
        let initial_colors = [
            (239, 68, 68, 150), // Red (150)
            (59, 130, 246, 98), // Blue (98)
            (34, 197, 94, 128), // Green (128)
            (249, 115, 22, 11), // Orange (11)
            (52, 211, 153, 11), // Mint (11)
            (163, 230, 53, 11), // Lime (11)
            (168, 85, 247, 11), // Purple (11)
            (6, 182, 212, 11),  // Cyan (11)
            (234, 179, 8, 3),   // Yellow (3)
            (34, 197, 94, 0),   // Green (0)
            (59, 130, 246, 0),  // Blue (0)
            (168, 85, 247, 0),  // Purple (0)
            (239, 68, 68, 0),   // Red (0)
            (59, 130, 246, 0),  // Blue (0)
        ];

        let diy_nodes = initial_colors
            .iter()
            .map(|(r, g, b, val)| DiyNode {
                color: RgbColor::new(*r, *g, *b),
                value: *val,
            })
            .collect();

        Self {
            current_screen: AppScreen::DeviceHub,
            dashboard_mode: DashboardMode::Dynamic,
            hub_tab_devices: true,
            active_device_id: String::new(),

            show_settings_modal: false,
            show_help_modal: false,
            show_effect_drawer: false,
            effect_search_query: String::new(),
            effect_category_filter: None,

            ui_power: true,
            ui_effect: 146, // Default to 146 (Gradient)
            ui_speed: 8,
            ui_brightness: 255,
            is_playing_dynamic: true,
            is_loop_all: false,
            is_favourite: false,

            ui_color: [0.23, 0.51, 0.96],
            ui_color_temp: 0.5,

            music_mode_index: 5, // Default to 5 (Energy Gradient)
            ui_audio_input: AudioInput::InternalMic,
            ui_sensitivity: 12,
            sync_with_windows_audio: true,

            diy_effect_type: DiyEffectType::Solid,
            diy_chase_mode: DiyChaseMode::Forward,
            diy_nodes,
            selected_diy_index: Some(0),
            diy_speed: 10,
            diy_brightness: 255,

            ui_length: 60,
            ui_chip_order: ChipOrder::Rgb,
            ui_confirm_safety: false,

            strip_pixels: Vec::new(),
            animation_timer: 0.0,
            audio_visualizer_timer: 0.0,
            live_audio_peak: 0.0,

            is_connected: false,
            is_scanning: false,
            status_message: "Ready to scan for nearby Bluetooth Low Energy LED controllers."
                .to_string(),
        }
    }
}
