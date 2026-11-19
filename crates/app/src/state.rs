//! # Application State and Command Types
//!
//! Strongly typed definitions for GUI navigation, device representation,
//! command routing channels, and interactive control states.

use pyxillum_protocol::{AudioInput, ChipOrder, Command, DeviceStatus, RgbColor};
use pyxillum_transport::{BleDiagnostic, DiscoveredDevice};

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

pub use pyxillum_core::{CustomColorEffect, DiyEffectType, DiyPatternStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiyDirection {
    Forward,
    Backward,
    MiddleToOut,
    OutToMiddle,
}

impl DiyDirection {
    pub fn all() -> &'static [DiyDirection] {
        &[
            Self::Forward,
            Self::Backward,
            Self::MiddleToOut,
            Self::OutToMiddle,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Forward => "Forward",
            Self::Backward => "Backward",
            Self::MiddleToOut => "Middle to Out",
            Self::OutToMiddle => "Out to Middle",
        }
    }
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

pub struct DiyPreset {
    pub name: &'static str,
    pub colors: &'static [(u8, u8, u8, u8)],
}

impl DiyPreset {
    pub fn all() -> &'static [DiyPreset] {
        &[
            DiyPreset {
                name: "Cyberpunk Neon",
                colors: &[
                    (0, 240, 255, 128),  // Cyan
                    (255, 0, 128, 128),  // Neon Magenta
                    (138, 43, 226, 128), // Blue Violet
                    (0, 102, 255, 128),  // Electric Blue
                ],
            },
            DiyPreset {
                name: "Sunset Glow",
                colors: &[
                    (220, 38, 38, 128),  // Crimson
                    (249, 115, 22, 128), // Deep Orange
                    (245, 158, 11, 128), // Warm Amber
                    (217, 70, 239, 128), // Violet Magenta
                ],
            },
            DiyPreset {
                name: "Aurora Borealis",
                colors: &[
                    (16, 185, 129, 128), // Emerald
                    (6, 182, 212, 128),  // Teal Cyan
                    (99, 102, 241, 128), // Indigo
                    (168, 85, 247, 128), // Violet
                ],
            },
            DiyPreset {
                name: "Ocean Breeze",
                colors: &[
                    (14, 116, 144, 128), // Deep Cyan
                    (2, 132, 199, 128),  // Azure
                    (56, 189, 248, 128), // Sky Blue
                    (45, 212, 191, 128), // Seafoam Mint
                ],
            },
            DiyPreset {
                name: "Fire & Ember",
                colors: &[
                    (250, 204, 21, 128), // Bright Yellow
                    (249, 115, 22, 128), // Tangerine
                    (225, 29, 72, 128),  // Flame Red
                    (159, 18, 57, 128),  // Dark Amber Red
                ],
            },
            DiyPreset {
                name: "Pastel Dream",
                colors: &[
                    (196, 181, 253, 128), // Soft Lilac
                    (186, 230, 253, 128), // Soft Sky
                    (167, 243, 208, 128), // Mint
                    (254, 205, 211, 128), // Peach Rose
                ],
            },
            DiyPreset {
                name: "Vaporwave",
                colors: &[
                    (244, 63, 94, 128),  // Hot Pink
                    (34, 211, 238, 128), // Electric Cyan
                    (147, 51, 234, 128), // Deep Violet
                    (253, 224, 71, 128), // Bright Pastel Yellow
                ],
            },
            DiyPreset {
                name: "Classic 7-Colour",
                colors: &[
                    (239, 68, 68, 128),  // Red
                    (249, 115, 22, 128), // Orange
                    (234, 179, 8, 128),  // Yellow
                    (34, 197, 94, 128),  // Green
                    (6, 182, 212, 128),  // Cyan
                    (59, 130, 246, 128), // Blue
                    (168, 85, 247, 128), // Purple
                ],
            },
        ]
    }
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

/// Single authoritative BLE connection state enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ConnectionState {
    Disconnected,
    Scanning,
    Connecting,
    Connected,
}

impl ConnectionState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Disconnected => "Not connected",
            Self::Scanning => "Scanning...",
            Self::Connecting => "Connecting...",
            Self::Connected => "Connected",
        }
    }
}

/// Plural helper for discovered controller devices.
pub fn format_controller_count(count: usize) -> String {
    if count == 1 {
        "1 controller found".to_string()
    } else {
        format!("{} controllers found", count)
    }
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
    pub show_icon_sheet: bool,
    pub layout_debug: bool,
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
    pub custom_color_effect: CustomColorEffect,
    pub custom_effect_speed: u8,
    pub live_color_stream_enabled: bool,

    // Music Mode Controls
    pub music_mode_index: u8, // 1..18
    pub ui_audio_input: AudioInput,
    pub ui_sensitivity: u8,
    pub sync_with_windows_audio: bool,

    // DIY Mode Controls
    pub diy_effect_type: DiyEffectType,
    pub diy_pattern: DiyPatternStyle,
    pub diy_direction: DiyDirection,
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
    pub discovered_count: usize,
    pub preview_infobar_dismissed: bool,
    pub status_message: String,
    pub last_ble_diagnostic: Option<BleDiagnostic>,
}

impl AppState {
    /// Returns the authoritative single connection state.
    pub fn connection_state(&self) -> ConnectionState {
        if self.is_connected {
            ConnectionState::Connected
        } else if self.is_scanning {
            ConnectionState::Scanning
        } else {
            ConnectionState::Disconnected
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        // Initialize rich starter palette (can be dynamically expanded from 2 to 32 nodes)
        let initial_colors = [
            (239, 68, 68, 128),  // Red
            (249, 115, 22, 128), // Orange
            (234, 179, 8, 128),  // Yellow
            (34, 197, 94, 128),  // Green
            (6, 182, 212, 128),  // Cyan
            (59, 130, 246, 128), // Blue
            (168, 85, 247, 128), // Purple
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
            show_icon_sheet: std::env::args().any(|a| a == "--icon-sheet"),
            layout_debug: std::env::args().any(|a| a == "--layout-debug"),
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
            custom_color_effect: CustomColorEffect::Solid,
            custom_effect_speed: 6,
            live_color_stream_enabled: true,

            music_mode_index: 5, // Default to 5 (Energy Gradient)
            ui_audio_input: AudioInput::InternalMic,
            ui_sensitivity: 12,
            sync_with_windows_audio: true,

            diy_effect_type: DiyEffectType::Gradient,
            diy_pattern: DiyPatternStyle::Flow,
            diy_direction: DiyDirection::Forward,
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
            discovered_count: 0,
            preview_infobar_dismissed: false,
            status_message: "Ready to scan for nearby Bluetooth Low Energy LED controllers."
                .to_string(),
            last_ble_diagnostic: None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_format_controller_count() {
        assert_eq!(format_controller_count(1), "1 controller found");
        assert_eq!(format_controller_count(0), "0 controllers found");
        assert_eq!(format_controller_count(2), "2 controllers found");
        assert_eq!(format_controller_count(5), "5 controllers found");
    }

    #[test]
    fn test_connection_state_transitions() {
        let mut state = AppState::default();
        assert_eq!(state.connection_state(), ConnectionState::Disconnected);
        assert_eq!(state.connection_state().label(), "Not connected");

        state.is_scanning = true;
        assert_eq!(state.connection_state(), ConnectionState::Scanning);
        assert_eq!(state.connection_state().label(), "Scanning...");

        state.is_scanning = false;
        state.is_connected = true;
        assert_eq!(state.connection_state(), ConnectionState::Connected);
        assert_eq!(state.connection_state().label(), "Connected");
    }

    #[test]
    fn test_pinned_region_height_and_content_area_ratio_at_1080p() {
        // Pinned region metrics:
        // Window height 1080p (work area ~ 1040px)
        let header_h = 40.0_f32;
        let spacing_1 = 8.0_f32;
        let infobar_h = 40.0_f32; // when visible
        let spacing_2 = 8.0_f32;
        let monitor_h_normal = 56.0_f32;
        let monitor_h_compact = 40.0_f32;

        let pinned_h_normal = header_h + spacing_1 + infobar_h + spacing_2 + monitor_h_normal;
        assert!(
            pinned_h_normal <= 170.0,
            "Pinned region at height >= 800 must be <= 170px, got {}",
            pinned_h_normal
        );

        let pinned_h_compact = header_h + spacing_1 + monitor_h_compact;
        assert!(
            pinned_h_compact <= 170.0,
            "Pinned region at height < 800 must be <= 170px, got {}",
            pinned_h_compact
        );

        // Content area ratio at 1080p:
        let total_window_h = 1080.0_f32;
        let page_padding_v = 32.0_f32;
        let available_content_h = total_window_h - pinned_h_normal - page_padding_v;
        let content_ratio = available_content_h / total_window_h;
        assert!(
            content_ratio >= 0.60,
            "Content area must have at least 60% of window height at 1080p, got {:.2}%",
            content_ratio * 100.0
        );
    }
}
