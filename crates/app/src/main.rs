#![warn(missing_docs)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Pyxillum Desktop Application
//!
//! Native cross-platform GUI for controlling Bluetooth Low Energy pixel LED controllers.
//! Faithfully implements the multi-device hub, circular dynamic dial, solid colour disc,
//! music-reactive sound engine with Windows Media Session integration, and 14-node DIY
//! lighting sequencer matching the companion application architecture.

use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_core::{DeviceManager, DeviceState};
use pyxillum_protocol::{AudioInput, ChipOrder, Command, LightMode, RgbColor, EFFECT_SOLID};
use pyxillum_sim::DeviceSimulator;
use pyxillum_transport::{DiscoveredDevice, SimTransport};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex};
use tracing::info;
use windows_media::{NowPlayingInfo, WindowsAudioService};

mod icon_data;
mod windows_media;

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum GuiCommand {
    Scan,
    Connect(String),
    Disconnect,
    SendCommand(Command),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LogEntry {
    timestamp: String,
    message: String,
    hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AppScreen {
    DeviceHub,
    Dashboard(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DashboardMode {
    Dynamic,
    Solid,
    Music,
    Diy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiyEffectType {
    Solid,
    Gradient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiyChaseMode {
    Forward,
    Backward,
    Middle,
    Out,
}

#[derive(Debug, Clone)]
struct DiyNode {
    color: RgbColor,
    value: u8,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct DeviceCardInfo {
    id: String,
    name: String,
    model: String,
    protocol_type: String,
    is_simulated: bool,
    is_connected: bool,
}

struct PyxillumApp {
    cmd_tx: mpsc::Sender<GuiCommand>,
    _state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
    _simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    audio_service: WindowsAudioService,

    // Navigation State
    current_screen: AppScreen,
    dashboard_mode: DashboardMode,
    hub_tab_devices: bool, // true = Devices, false = Groups

    // Modals
    show_settings_modal: bool,
    show_help_modal: bool,
    show_effect_drawer: bool,

    // Devices & Diagnostics
    is_demo_mode: bool,
    _discovered: Vec<DiscoveredDevice>,
    saved_devices: Vec<DeviceCardInfo>,
    logs: Vec<LogEntry>,

    // Dynamic Mode State
    ui_power: bool,
    ui_effect: u8,
    ui_speed: u8,
    ui_brightness: u8,
    is_playing_dynamic: bool,
    is_loop_all: bool,
    is_favourite: bool,

    // Solid Mode State
    ui_color: [f32; 3],
    ui_color_temp: f32, // 0.0 = Warm Amber, 1.0 = Cool Daylight White

    // Music Mode State
    music_mode_index: u8, // 1..16
    ui_audio_input: AudioInput,
    ui_sensitivity: u8,
    sync_with_windows_audio: bool,
    now_playing: NowPlayingInfo,
    live_audio_peak: f32,

    // DIY Mode State
    diy_effect_type: DiyEffectType,
    diy_chase_mode: DiyChaseMode,
    diy_nodes: Vec<DiyNode>,
    selected_diy_index: Option<usize>,
    diy_speed: u8,
    diy_brightness: u8,

    // Hardware Configuration (Safety Gated)
    ui_length: u8,
    ui_chip_order: ChipOrder,
    ui_confirm_safety: bool,

    // 60 FPS Animation Clocks
    last_frame_instant: Instant,
    animation_timer: f32,
    audio_visualizer_timer: f32,
    strip_pixels: Vec<RgbColor>,
}

impl PyxillumApp {
    fn new(
        cc: &eframe::CreationContext<'_>,
        cmd_tx: mpsc::Sender<GuiCommand>,
        state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
        simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    ) -> Self {
        // Configure clean modern light styling matching the companion application screens
        let mut visuals = egui::Visuals::light();
        visuals.override_text_color = Some(Color32::from_rgb(30, 41, 59));
        visuals.panel_fill = Color32::from_rgb(238, 245, 255);
        visuals.window_fill = Color32::WHITE;
        visuals.faint_bg_color = Color32::from_rgb(241, 245, 249);
        visuals.extreme_bg_color = Color32::from_rgb(226, 232, 240);

        visuals.widgets.noninteractive.bg_fill = Color32::WHITE;
        visuals.widgets.noninteractive.bg_stroke =
            Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240));

        visuals.widgets.inactive.bg_fill = Color32::WHITE;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(218, 226, 237));
        visuals.widgets.inactive.rounding = Rounding::same(12.0);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(240, 246, 255);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.5_f32, Color32::from_rgb(59, 130, 246));
        visuals.widgets.hovered.rounding = Rounding::same(12.0);

        visuals.widgets.active.bg_fill = Color32::from_rgb(59, 130, 246);
        visuals.widgets.active.rounding = Rounding::same(12.0);
        visuals.window_rounding = Rounding::same(20.0);
        cc.egui_ctx.set_visuals(visuals);

        let audio_service = WindowsAudioService::new();

        // Initialise the 14 DIY sequence nodes matching Image 5
        let diy_nodes = vec![
            DiyNode {
                color: RgbColor::new(240, 50, 50),
                value: 150,
            }, // Red
            DiyNode {
                color: RgbColor::new(40, 100, 240),
                value: 98,
            }, // Blue
            DiyNode {
                color: RgbColor::new(50, 220, 80),
                value: 128,
            }, // Green
            DiyNode {
                color: RgbColor::new(255, 150, 30),
                value: 11,
            }, // Orange
            DiyNode {
                color: RgbColor::new(80, 230, 170),
                value: 11,
            }, // Mint
            DiyNode {
                color: RgbColor::new(40, 200, 60),
                value: 11,
            }, // Lime
            DiyNode {
                color: RgbColor::new(160, 40, 240),
                value: 11,
            }, // Purple
            DiyNode {
                color: RgbColor::new(50, 180, 255),
                value: 11,
            }, // Cyan
            DiyNode {
                color: RgbColor::new(255, 210, 40),
                value: 3,
            }, // Yellow
            DiyNode {
                color: RgbColor::new(80, 220, 80),
                value: 0,
            }, // Green
            DiyNode {
                color: RgbColor::new(80, 220, 80),
                value: 0,
            }, // Green
            DiyNode {
                color: RgbColor::new(100, 240, 100),
                value: 0,
            }, // Lime
            DiyNode {
                color: RgbColor::new(100, 240, 100),
                value: 0,
            }, // Lime
            DiyNode {
                color: RgbColor::new(20, 80, 240),
                value: 0,
            }, // Blue
        ];

        let saved_devices = vec![
            DeviceCardInfo {
                id: "SIM:00:11:22:33:44:55".to_string(),
                name: "SP621E Virtual".to_string(),
                model: "SP621E | SIMULATOR".to_string(),
                protocol_type: "Bluetooth Low Energy".to_string(),
                is_simulated: true,
                is_connected: true,
            },
            DeviceCardInfo {
                id: "HW:2A:27:50:AA:BB:CC".to_string(),
                name: "SP621E Mini".to_string(),
                model: "SP621E | 2A2750".to_string(),
                protocol_type: "Bluetooth Low Energy".to_string(),
                is_simulated: false,
                is_connected: false,
            },
            DeviceCardInfo {
                id: "HW:D8:8A:3A:11:22:33".to_string(),
                name: "SP530E Controller".to_string(),
                model: "SP530E | D88A3A".to_string(),
                protocol_type: "Bluetooth / 2.4G".to_string(),
                is_simulated: false,
                is_connected: false,
            },
            DeviceCardInfo {
                id: "HW:00:0D:7D:44:55:66".to_string(),
                name: "SP630E Controller".to_string(),
                model: "SP630E | 000D7D".to_string(),
                protocol_type: "Bluetooth Mesh".to_string(),
                is_simulated: false,
                is_connected: false,
            },
        ];

        Self {
            cmd_tx,
            _state_reader: state_reader,
            _simulator_ref: simulator_ref,
            audio_service,
            current_screen: AppScreen::Dashboard("SIM:00:11:22:33:44:55".to_string()),
            dashboard_mode: DashboardMode::Dynamic,
            hub_tab_devices: true,
            show_settings_modal: false,
            show_help_modal: false,
            show_effect_drawer: false,
            is_demo_mode: true,
            _discovered: Vec::new(),
            saved_devices,
            logs: vec![LogEntry {
                timestamp: "00:00:00".to_string(),
                message: "Pyxillum ready in offline-first mode.".to_string(),
                hex: String::new(),
            }],
            ui_power: true,
            ui_effect: 146, // Default: Gradient Flow matching screenshot Image 2
            ui_speed: 10,
            ui_brightness: 255,
            is_playing_dynamic: true,
            is_loop_all: false,
            is_favourite: false,
            ui_color: [0.0, 0.55, 1.0], // Azure Blue
            ui_color_temp: 0.5,
            music_mode_index: 5, // Default: Strip Music Mode 5 matching screenshot Image 3
            ui_audio_input: AudioInput::InternalMic,
            ui_sensitivity: 8,
            sync_with_windows_audio: true,
            now_playing: NowPlayingInfo::default(),
            live_audio_peak: 0.0,
            diy_effect_type: DiyEffectType::Solid,
            diy_chase_mode: DiyChaseMode::Forward,
            diy_nodes,
            selected_diy_index: None,
            diy_speed: 10,
            diy_brightness: 255,
            ui_length: 30,
            ui_chip_order: ChipOrder::Grb,
            ui_confirm_safety: false,
            last_frame_instant: Instant::now(),
            animation_timer: 0.0,
            audio_visualizer_timer: 0.0,
            strip_pixels: vec![RgbColor::new(0, 140, 255); 30],
        }
    }

    fn send(&self, cmd: GuiCommand) {
        let _ = self.cmd_tx.try_send(cmd);
    }

    fn log(&mut self, msg: &str, hex: &str) {
        let entry = LogEntry {
            timestamp: "now".to_string(),
            message: msg.to_string(),
            hex: hex.to_string(),
        };
        self.logs.push(entry);
        if self.logs.len() > 60 {
            self.logs.remove(0);
        }
    }
}

impl eframe::App for PyxillumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Delta time computation
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_instant).as_secs_f32();
        self.last_frame_instant = now;
        if self.is_playing_dynamic {
            self.animation_timer += dt * (self.ui_speed as f32);
        }
        self.audio_visualizer_timer += dt;

        // Query Windows media status & master audio peak
        let (now_playing, peak) = self.audio_service.get_state();
        self.now_playing = now_playing;
        self.live_audio_peak = peak;

        // Update virtual strip rendering buffer
        self.update_virtual_pixels();

        // Keyboard shortcut: Space toggles power
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.ui_power = !self.ui_power;
            let cmd = Command::Power(self.ui_power);
            self.log(
                &format!("Keyboard toggled power: {}", self.ui_power),
                &hex_string(&cmd.encode()),
            );
            self.send(GuiCommand::SendCommand(cmd));
        }

        // Render Background Sky-Blue Atmospheric Gradient
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(234, 242, 255)))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let painter = ui.painter();

                // Top sky-blue hero wash
                painter.rect_filled(
                    Rect::from_min_max(rect.min, Pos2::new(rect.max.x, rect.min.y + 190.0)),
                    Rounding::ZERO,
                    Color32::from_rgb(59, 130, 246),
                );
                // Ambient diffuse circular curve accents
                painter.circle_filled(
                    Pos2::new(rect.max.x + 40.0, rect.min.y + 70.0),
                    240.0,
                    Color32::from_rgba_premultiplied(255, 255, 255, 30),
                );
                painter.circle_filled(
                    Pos2::new(rect.min.x - 30.0, rect.min.y + 160.0),
                    200.0,
                    Color32::from_rgba_premultiplied(255, 255, 255, 25),
                );

                egui::ScrollArea::vertical().show(ui, |ui| match &self.current_screen {
                    AppScreen::DeviceHub => self.render_device_hub(ui),
                    AppScreen::Dashboard(_dev_id) => self.render_dashboard(ui),
                });
            });

        // Modals (Overlays)
        if self.show_settings_modal {
            self.render_settings_modal(ctx);
        }
        if self.show_help_modal {
            self.render_help_modal(ctx);
        }
        if self.show_effect_drawer {
            self.render_effect_drawer_modal(ctx);
        }

        // Maintain 60 FPS repaints
        ctx.request_repaint();
    }
}

impl PyxillumApp {
    /// Renders Screen 1: Device List & Hub Screen (Matching Image 1).
    fn render_device_hub(&mut self, ui: &mut egui::Ui) {
        ui.add_space(14.0);

        // Header: Profile Avatar & Add Button
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            // Profile Pill
            let (avatar_rect, _) =
                ui.allocate_exact_size(Vec2::new(32.0, 32.0), egui::Sense::hover());
            let painter = ui.painter_at(avatar_rect);
            painter.circle_filled(avatar_rect.center(), 16.0, Color32::from_white_alpha(70));
            painter.circle_stroke(
                avatar_rect.center(),
                16.0,
                Stroke::new(1.0_f32, Color32::WHITE),
            );
            painter.circle_filled(
                Pos2::new(avatar_rect.center().x, avatar_rect.center().y - 4.0),
                5.0,
                Color32::WHITE,
            );
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(avatar_rect.center().x - 7.0, avatar_rect.center().y + 3.0),
                    Pos2::new(avatar_rect.center().x + 7.0, avatar_rect.center().y + 11.0),
                ),
                Rounding::same(4.0),
                Color32::WHITE,
            );

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Guest Studio")
                    .strong()
                    .size(16.0)
                    .color(Color32::WHITE),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(18.0);
                // Circular "+" Add Button
                let (add_rect, add_resp) =
                    ui.allocate_exact_size(Vec2::new(32.0, 32.0), egui::Sense::click());
                let p = ui.painter_at(add_rect);
                p.circle_filled(add_rect.center(), 16.0, Color32::from_white_alpha(50));
                p.circle_stroke(
                    add_rect.center(),
                    16.0,
                    Stroke::new(1.0_f32, Color32::WHITE),
                );
                p.text(
                    add_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "+",
                    egui::FontId::proportional(20.0),
                    Color32::WHITE,
                );

                if add_resp.clicked() {
                    self.send(GuiCommand::Scan);
                    self.log("Scanning for Bluetooth peripherals...", "");
                }
            });
        });

        ui.add_space(14.0);

        // Hero Card Banner: "New experience of light control"
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            let width = ui.available_width() - 18.0;
            let (hero_rect, _) =
                ui.allocate_exact_size(Vec2::new(width, 100.0), egui::Sense::hover());
            let painter = ui.painter_at(hero_rect);

            painter.rect_filled(
                hero_rect,
                Rounding::same(16.0),
                Color32::from_rgb(79, 145, 255),
            );
            painter.rect_stroke(
                hero_rect,
                Rounding::same(16.0),
                Stroke::new(1.0_f32, Color32::from_white_alpha(50)),
            );

            // Hero Text
            painter.text(
                Pos2::new(hero_rect.min.x + 20.0, hero_rect.min.y + 24.0),
                egui::Align2::LEFT_TOP,
                "New experience",
                egui::FontId::proportional(19.0),
                Color32::WHITE,
            );
            painter.text(
                Pos2::new(hero_rect.min.x + 20.0, hero_rect.min.y + 48.0),
                egui::Align2::LEFT_TOP,
                "of light control",
                egui::FontId::proportional(19.0),
                Color32::WHITE,
            );

            // Architectural graphic accents
            let house_rect = Rect::from_min_max(
                Pos2::new(hero_rect.max.x - 90.0, hero_rect.min.y + 20.0),
                Pos2::new(hero_rect.max.x - 20.0, hero_rect.max.y - 15.0),
            );
            painter.rect_filled(
                house_rect,
                Rounding::same(6.0),
                Color32::from_rgb(147, 197, 253),
            );
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(house_rect.min.x + 10.0, house_rect.min.y + 12.0),
                    Pos2::new(house_rect.min.x + 30.0, house_rect.min.y + 36.0),
                ),
                Rounding::ZERO,
                Color32::from_rgb(254, 240, 138),
            );
        });

        ui.add_space(18.0);

        // Segmented Tabs: "Devices" | "Groups"
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            if ui
                .selectable_label(
                    self.hub_tab_devices,
                    egui::RichText::new("Devices").strong().size(16.0),
                )
                .clicked()
            {
                self.hub_tab_devices = true;
            }
            ui.add_space(12.0);
            if ui
                .selectable_label(
                    !self.hub_tab_devices,
                    egui::RichText::new("Groups").size(16.0),
                )
                .clicked()
            {
                self.hub_tab_devices = false;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("[Grid View]")
                        .size(12.0)
                        .color(Color32::from_rgb(100, 116, 139)),
                );
            });
        });

        ui.add_space(12.0);

        // Grid of Device Cards
        let devices = self.saved_devices.clone();
        let card_width = ((ui.available_width() - 48.0) / 2.0).max(180.0);

        ui.horizontal(|ui| {
            ui.add_space(18.0);
            ui.vertical(|ui| {
                for chunk in devices.chunks(2) {
                    ui.horizontal(|ui| {
                        for dev in chunk {
                            let (card_rect, card_resp) = ui.allocate_exact_size(
                                Vec2::new(card_width, 110.0),
                                egui::Sense::click(),
                            );
                            let painter = ui.painter_at(card_rect);

                            // White rounded card with soft elevation
                            painter.rect_filled(card_rect, Rounding::same(16.0), Color32::WHITE);
                            painter.rect_stroke(
                                card_rect,
                                Rounding::same(16.0),
                                Stroke::new(
                                    1.0_f32,
                                    if card_resp.hovered() {
                                        Color32::from_rgb(59, 130, 246)
                                    } else {
                                        Color32::from_rgb(226, 232, 240)
                                    },
                                ),
                            );

                            // Device icon thumbnail box
                            let thumb_rect = Rect::from_min_size(
                                Pos2::new(card_rect.min.x + 14.0, card_rect.min.y + 14.0),
                                Vec2::new(36.0, 36.0),
                            );
                            painter.rect_filled(
                                thumb_rect,
                                Rounding::same(8.0),
                                Color32::from_rgb(241, 245, 249),
                            );
                            painter.rect_stroke(
                                thumb_rect,
                                Rounding::same(8.0),
                                Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225)),
                            );
                            painter.text(
                                thumb_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "IC",
                                egui::FontId::proportional(12.0),
                                Color32::from_rgb(100, 116, 139),
                            );

                            // Title & Model
                            painter.text(
                                Pos2::new(card_rect.min.x + 14.0, card_rect.min.y + 60.0),
                                egui::Align2::LEFT_TOP,
                                &dev.name,
                                egui::FontId::proportional(14.0),
                                Color32::from_rgb(15, 23, 42),
                            );
                            painter.text(
                                Pos2::new(card_rect.min.x + 14.0, card_rect.min.y + 82.0),
                                egui::Align2::LEFT_TOP,
                                &dev.model,
                                egui::FontId::proportional(11.0),
                                Color32::from_rgb(148, 163, 184),
                            );

                            // Signal / Status badge on top right of card
                            let status_col = if dev.is_connected {
                                Color32::from_rgb(16, 185, 129)
                            } else {
                                Color32::from_rgb(148, 163, 184)
                            };
                            painter.circle_filled(
                                Pos2::new(card_rect.max.x - 22.0, card_rect.min.y + 22.0),
                                4.0,
                                status_col,
                            );

                            if card_resp.clicked() {
                                if dev.is_simulated {
                                    self.is_demo_mode = true;
                                    self.send(GuiCommand::Connect(dev.id.clone()));
                                }
                                self.current_screen = AppScreen::Dashboard(dev.name.clone());
                            }
                        }
                    });
                    ui.add_space(10.0);
                }
            });
        });
    }

    /// Renders Screen 2: Device Control Dashboard View (Matching Images 2, 3, 4, 5).
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        let dev_title = match &self.current_screen {
            AppScreen::Dashboard(name) => name.clone(),
            _ => "SP621E Mini".to_string(),
        };

        ui.add_space(12.0);

        // Header Bar: "<  SP530E [BT]  (?)  (Settings)"
        ui.horizontal(|ui| {
            ui.add_space(16.0);

            // "<" Back Arrow
            if ui
                .button(
                    egui::RichText::new("<")
                        .strong()
                        .size(18.0)
                        .color(Color32::WHITE),
                )
                .clicked()
            {
                self.current_screen = AppScreen::DeviceHub;
            }

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!("{} (Bluetooth)", dev_title))
                    .strong()
                    .size(17.0)
                    .color(Color32::WHITE),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(16.0);

                // Settings Gear Button
                if ui
                    .button(
                        egui::RichText::new("Settings")
                            .size(12.0)
                            .color(Color32::WHITE),
                    )
                    .clicked()
                {
                    self.show_settings_modal = true;
                }

                ui.add_space(6.0);

                // Help Button
                if ui
                    .button(
                        egui::RichText::new("?")
                            .strong()
                            .size(14.0)
                            .color(Color32::WHITE),
                    )
                    .clicked()
                {
                    self.show_help_modal = true;
                }
            });
        });

        ui.add_space(8.0);

        // Sub-Header Mode Tabs: "Dynamic  Solid  Music  DIY" and Red Power Button
        ui.horizontal(|ui| {
            ui.add_space(20.0);

            let modes = [
                (DashboardMode::Dynamic, "Dynamic"),
                (DashboardMode::Solid, "Solid"),
                (DashboardMode::Music, "Music"),
                (DashboardMode::Diy, "DIY"),
            ];

            for (m, lbl) in modes {
                let is_active = self.dashboard_mode == m;
                ui.vertical(|ui| {
                    let text = egui::RichText::new(lbl)
                        .strong()
                        .size(15.0)
                        .color(if is_active {
                            Color32::WHITE
                        } else {
                            Color32::from_white_alpha(180)
                        });

                    if ui.selectable_label(false, text).clicked() {
                        self.dashboard_mode = m;
                    }

                    // Active underline indicator
                    if is_active {
                        let (line_rect, _) =
                            ui.allocate_exact_size(Vec2::new(36.0, 3.0), egui::Sense::hover());
                        ui.painter()
                            .rect_filled(line_rect, Rounding::same(1.5), Color32::WHITE);
                    } else {
                        ui.add_space(3.0);
                    }
                });
                ui.add_space(14.0);
            }

            // Circular Red Master Power Button on the right
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(20.0);
                let (pwr_rect, pwr_resp) =
                    ui.allocate_exact_size(Vec2::new(38.0, 38.0), egui::Sense::click());
                let p = ui.painter_at(pwr_rect);

                let pwr_color = if self.ui_power {
                    Color32::from_rgb(255, 75, 75) // Active Red
                } else {
                    Color32::from_rgb(148, 163, 184) // Inactive Grey
                };

                p.circle_filled(pwr_rect.center(), 18.0, pwr_color);
                p.circle_stroke(
                    pwr_rect.center(),
                    18.0,
                    Stroke::new(1.0_f32, Color32::from_white_alpha(100)),
                );
                p.text(
                    pwr_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "P",
                    egui::FontId::proportional(16.0),
                    Color32::WHITE,
                );

                if pwr_resp.clicked() {
                    self.ui_power = !self.ui_power;
                    let cmd = Command::Power(self.ui_power);
                    self.log(
                        &format!("Toggled power: {}", self.ui_power),
                        &hex_string(&cmd.encode()),
                    );
                    self.send(GuiCommand::SendCommand(cmd));
                }
            });
        });

        ui.add_space(10.0);

        // White Floating Sheet Card Container (Rounded corners 20.0 px)
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            let card_w = ui.available_width() - 16.0;

            let frame = egui::Frame::none()
                .fill(Color32::WHITE)
                .rounding(Rounding::same(20.0))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)))
                .inner_margin(egui::Margin::symmetric(18.0, 16.0));

            frame.show(ui, |ui| {
                ui.set_width(card_w);

                // Live Addressable Neon Strip Monitor across top of card
                self.render_neon_strip_compact(ui);
                ui.add_space(10.0);

                match self.dashboard_mode {
                    DashboardMode::Dynamic => self.render_mode_dynamic(ui),
                    DashboardMode::Solid => self.render_mode_solid(ui),
                    DashboardMode::Music => self.render_mode_music(ui),
                    DashboardMode::Diy => self.render_mode_diy(ui),
                }
            });
        });
    }

    /// Renders Dynamic Mode: Large Radial Arc Dial (146 Gradient), Arrows, and 4-Icon Action Bar.
    fn render_mode_dynamic(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // Large Radial Arc Dial
            let eff_name = get_effect_title(self.ui_effect);
            self.render_arc_dial(ui, self.ui_effect as u32, &eff_name, true);

            ui.add_space(14.0);

            // 4-Icon Action Bar (Loop, Play/Pause, Favorite, List)
            ui.horizontal(|ui| {
                let bar_w = ui.available_width();
                ui.add_space((bar_w - 240.0) / 2.0);

                // 1. Loop Button
                let loop_lbl = if self.is_loop_all { "Cycle" } else { "Loop" };
                if render_round_action_button(ui, loop_lbl, self.is_loop_all).clicked() {
                    self.is_loop_all = !self.is_loop_all;
                    let m = if self.is_loop_all {
                        LightMode::AutoDynamic
                    } else {
                        LightMode::Single
                    };
                    self.send(GuiCommand::SendCommand(Command::SetLightMode(m)));
                }
                ui.add_space(16.0);

                // 2. Play / Pause Button
                let play_lbl = if self.is_playing_dynamic { "||" } else { ">" };
                if render_round_action_button(ui, play_lbl, self.is_playing_dynamic).clicked() {
                    self.is_playing_dynamic = !self.is_playing_dynamic;
                }
                ui.add_space(16.0);

                // 3. Favorite Button
                if render_round_action_button(ui, "Fav", self.is_favourite).clicked() {
                    self.is_favourite = !self.is_favourite;
                }
                ui.add_space(16.0);

                // 4. List / Drawer Button
                if render_round_action_button(ui, "List", self.show_effect_drawer).clicked() {
                    self.show_effect_drawer = !self.show_effect_drawer;
                }
            });

            ui.add_space(16.0);

            // Dual Adjustment Sliders: Speed and Brightness
            ui.columns(2, |cols| {
                // Brightness
                cols[0].group(|ui| {
                    ui.label(
                        egui::RichText::new("Brightness")
                            .size(12.0)
                            .color(Color32::from_rgb(100, 116, 139)),
                    );
                    let pct = (self.ui_brightness as f32 / 255.0 * 100.0) as u32;
                    ui.label(
                        egui::RichText::new(format!("{}%", pct))
                            .strong()
                            .size(16.0)
                            .color(Color32::from_rgb(15, 23, 42)),
                    );
                    if ui
                        .add(egui::Slider::new(&mut self.ui_brightness, 0..=255).show_value(false))
                        .changed()
                    {
                        self.send(GuiCommand::SendCommand(Command::SetBrightness(
                            self.ui_brightness,
                        )));
                    }
                });

                // Speed
                cols[1].group(|ui| {
                    ui.label(
                        egui::RichText::new("Speed")
                            .size(12.0)
                            .color(Color32::from_rgb(100, 116, 139)),
                    );
                    ui.label(
                        egui::RichText::new(format!("{}", self.ui_speed))
                            .strong()
                            .size(16.0)
                            .color(Color32::from_rgb(16, 185, 129)),
                    );
                    if ui
                        .add(egui::Slider::new(&mut self.ui_speed, 1..=10).show_value(false))
                        .changed()
                    {
                        self.send(GuiCommand::SendCommand(Command::SetSpeed(self.ui_speed)));
                    }
                });
            });
        });
    }

    /// Renders Solid Mode: Radial Hue-Saturation Disc, Swatches, and Warmth.
    fn render_mode_solid(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            // Left: Color Wheel
            columns[0].vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Chromatic Colour Wheel")
                        .strong()
                        .size(13.0)
                        .color(Color32::from_rgb(30, 41, 59)),
                );
                ui.add_space(4.0);

                if render_color_disc(ui, &mut self.ui_color, 85.0) {
                    self.ui_effect = EFFECT_SOLID;
                    let r = (self.ui_color[0] * 255.0) as u8;
                    let g = (self.ui_color[1] * 255.0) as u8;
                    let b = (self.ui_color[2] * 255.0) as u8;
                    self.send(GuiCommand::SendCommand(Command::SetColor {
                        color: RgbColor::new(r, g, b),
                        level: self.ui_brightness,
                    }));
                }

                ui.add_space(8.0);
                let r = (self.ui_color[0] * 255.0) as u8;
                let g = (self.ui_color[1] * 255.0) as u8;
                let b = (self.ui_color[2] * 255.0) as u8;

                let (swatch_rect, _) =
                    ui.allocate_exact_size(Vec2::new(180.0, 30.0), egui::Sense::hover());
                let p = ui.painter_at(swatch_rect);
                p.rect_filled(swatch_rect, Rounding::same(8.0), Color32::from_rgb(r, g, b));
                p.rect_stroke(
                    swatch_rect,
                    Rounding::same(8.0),
                    Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225)),
                );
                p.text(
                    swatch_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("#{:02X}{:02X}{:02X}  RGB({}, {}, {})", r, g, b, r, g, b),
                    egui::FontId::proportional(11.0),
                    if (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000 > 128 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    },
                );
            });

            // Right: Brightness & Palette Swatches
            columns[1].vertical(|ui| {
                ui.label(egui::RichText::new("Brightness").strong().size(13.0));
                let pct = (self.ui_brightness as f32 / 255.0 * 100.0) as u32;
                ui.label(format!("Level: {}%", pct));
                if ui
                    .add(egui::Slider::new(&mut self.ui_brightness, 0..=255).show_value(false))
                    .changed()
                {
                    self.send(GuiCommand::SendCommand(Command::SetBrightness(
                        self.ui_brightness,
                    )));
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Quick Swatches").strong().size(13.0));
                let swatches = [
                    [1.0, 0.47, 0.0],  // Amber
                    [1.0, 0.2, 0.3],   // Coral
                    [1.0, 0.8, 0.0],   // Gold
                    [0.0, 0.88, 0.35], // Emerald
                    [0.0, 0.85, 1.0],  // Cyan
                    [0.0, 0.45, 1.0],  // Azure
                    [0.45, 0.2, 1.0],  // Violet
                    [1.0, 0.0, 0.6],   // Magenta
                    [1.0, 0.88, 0.7],  // Warm White
                    [1.0, 1.0, 1.0],   // Pure White
                ];

                ui.horizontal_wrapped(|ui| {
                    for rgb in swatches {
                        let c = Color32::from_rgb(
                            (rgb[0] * 255.0) as u8,
                            (rgb[1] * 255.0) as u8,
                            (rgb[2] * 255.0) as u8,
                        );
                        let (rect, resp) =
                            ui.allocate_exact_size(Vec2::new(30.0, 30.0), egui::Sense::click());
                        let p = ui.painter_at(rect);
                        p.circle_filled(rect.center(), 13.0, c);
                        p.circle_stroke(
                            rect.center(),
                            13.0,
                            Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225)),
                        );

                        if resp.clicked() {
                            self.ui_color = rgb;
                            self.ui_effect = EFFECT_SOLID;
                            let r = (rgb[0] * 255.0) as u8;
                            let g = (rgb[1] * 255.0) as u8;
                            let b = (rgb[2] * 255.0) as u8;
                            self.send(GuiCommand::SendCommand(Command::SetColor {
                                color: RgbColor::new(r, g, b),
                                level: self.ui_brightness,
                            }));
                        }
                    }
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("White Temperature").strong().size(13.0));
                if ui
                    .add(egui::Slider::new(&mut self.ui_color_temp, 0.0..=1.0).text("Warm -> Cool"))
                    .changed()
                {
                    let warm = [1.0_f32, 0.77_f32, 0.5_f32];
                    let cool = [0.8_f32, 0.9_f32, 1.0_f32];
                    let t = self.ui_color_temp;
                    self.ui_color = [
                        warm[0] * (1.0 - t) + cool[0] * t,
                        warm[1] * (1.0 - t) + cool[1] * t,
                        warm[2] * (1.0 - t) + cool[2] * t,
                    ];
                    self.ui_effect = EFFECT_SOLID;
                    let r = (self.ui_color[0] * 255.0) as u8;
                    let g = (self.ui_color[1] * 255.0) as u8;
                    let b = (self.ui_color[2] * 255.0) as u8;
                    self.send(GuiCommand::SendCommand(Command::SetColor {
                        color: RgbColor::new(r, g, b),
                        level: self.ui_brightness,
                    }));
                }
            });
        });
    }

    /// Renders Music Lights Mode: Radial Music Dial (Strip Music Mode 5), Audio Sources, and Windows Media.
    fn render_mode_music(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // Large Music Dial
            let mode_name = format!("Strip Music Mode {}", self.music_mode_index);
            self.render_arc_dial(ui, self.music_mode_index as u32, &mode_name, false);

            ui.add_space(10.0);

            // Windows Now Playing Media Banner
            let (np_rect, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 52.0), egui::Sense::hover());
            let painter = ui.painter_at(np_rect);
            painter.rect_filled(
                np_rect,
                Rounding::same(12.0),
                Color32::from_rgb(241, 245, 249),
            );
            painter.rect_stroke(
                np_rect,
                Rounding::same(12.0),
                Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225)),
            );

            if !self.now_playing.title.is_empty() {
                painter.text(
                    Pos2::new(np_rect.min.x + 14.0, np_rect.min.y + 8.0),
                    egui::Align2::LEFT_TOP,
                    format!("NOW PLAYING: {}", self.now_playing.title),
                    egui::FontId::proportional(13.0),
                    Color32::from_rgb(15, 23, 42),
                );
                painter.text(
                    Pos2::new(np_rect.min.x + 14.0, np_rect.min.y + 28.0),
                    egui::Align2::LEFT_TOP,
                    format!(
                        "{} | {}",
                        self.now_playing.artist, self.now_playing.app_name
                    ),
                    egui::FontId::proportional(11.0),
                    Color32::from_rgb(100, 116, 139),
                );
                let pct = (self.live_audio_peak * 100.0) as u32;
                painter.text(
                    Pos2::new(np_rect.max.x - 14.0, np_rect.min.y + 16.0),
                    egui::Align2::RIGHT_TOP,
                    format!("Peak: {}%", pct),
                    egui::FontId::proportional(12.0),
                    Color32::from_rgb(59, 130, 246),
                );
            } else {
                painter.text(
                    np_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Listening for Windows Media Audio (Spotify, YouTube, Media Player)...",
                    egui::FontId::proportional(11.0),
                    Color32::from_rgb(148, 163, 184),
                );
            }

            ui.add_space(10.0);

            // 16-Band Animated Graphic Equalizer Visualizer
            self.render_equalizer_bars_clean(ui);

            ui.add_space(14.0);

            // Bottom Audio Source Bar (Auto Cycle, Music Stream, Host Mic, Internal Mic)
            ui.horizontal(|ui| {
                let bar_w = ui.available_width();
                ui.add_space((bar_w - 320.0) / 2.0);

                if render_source_pill(ui, "Auto Cycle", false).clicked() {
                    self.send(GuiCommand::SendCommand(Command::SetLightMode(
                        LightMode::AutoSound,
                    )));
                }
                ui.add_space(8.0);
                if render_source_pill(ui, "Music Stream", self.sync_with_windows_audio).clicked() {
                    self.sync_with_windows_audio = !self.sync_with_windows_audio;
                }
                ui.add_space(8.0);
                if render_source_pill(ui, "Host Mic", self.ui_audio_input == AudioInput::Player)
                    .clicked()
                {
                    self.ui_audio_input = AudioInput::Player;
                    self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                        AudioInput::Player,
                    )));
                }
                ui.add_space(8.0);
                if render_source_pill(
                    ui,
                    "Internal Mic",
                    self.ui_audio_input == AudioInput::InternalMic,
                )
                .clicked()
                {
                    self.ui_audio_input = AudioInput::InternalMic;
                    self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                        AudioInput::InternalMic,
                    )));
                }
            });
        });
    }

    /// Renders DIY Lighting Mode: 14 Color Grid, Mode (Chase Forward), Brightness, and Speed (Matching Image 5).
    fn render_mode_diy(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Header Row: "DIY Effect" with segmented toggle "Solid" | "Gradient"
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("DIY Effect")
                        .strong()
                        .size(15.0)
                        .color(Color32::from_rgb(15, 23, 42)),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (pill_rect, _) =
                        ui.allocate_exact_size(Vec2::new(140.0, 30.0), egui::Sense::hover());
                    let p = ui.painter_at(pill_rect);
                    p.rect_filled(
                        pill_rect,
                        Rounding::same(15.0),
                        Color32::from_rgb(241, 245, 249),
                    );

                    let half_w = pill_rect.width() / 2.0;
                    let is_grad = self.diy_effect_type == DiyEffectType::Gradient;

                    // Solid Tab
                    let solid_rect = Rect::from_min_size(pill_rect.min, Vec2::new(half_w, 30.0));
                    let solid_resp = ui.allocate_rect(solid_rect, egui::Sense::click());
                    if !is_grad {
                        p.rect_filled(solid_rect, Rounding::same(15.0), Color32::WHITE);
                        p.rect_stroke(
                            solid_rect,
                            Rounding::same(15.0),
                            Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225)),
                        );
                    }
                    p.text(
                        solid_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Solid",
                        egui::FontId::proportional(12.0),
                        if !is_grad {
                            Color32::from_rgb(15, 23, 42)
                        } else {
                            Color32::from_rgb(100, 116, 139)
                        },
                    );

                    // Gradient Tab
                    let grad_rect = Rect::from_min_size(
                        Pos2::new(pill_rect.min.x + half_w, pill_rect.min.y),
                        Vec2::new(half_w, 30.0),
                    );
                    let grad_resp = ui.allocate_rect(grad_rect, egui::Sense::click());
                    if is_grad {
                        p.rect_filled(grad_rect, Rounding::same(15.0), Color32::WHITE);
                        p.rect_stroke(
                            grad_rect,
                            Rounding::same(15.0),
                            Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225)),
                        );
                    }
                    p.text(
                        grad_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Gradient",
                        egui::FontId::proportional(12.0),
                        if is_grad {
                            Color32::from_rgb(15, 23, 42)
                        } else {
                            Color32::from_rgb(100, 116, 139)
                        },
                    );

                    if solid_resp.clicked() {
                        self.diy_effect_type = DiyEffectType::Solid;
                    }
                    if grad_resp.clicked() {
                        self.diy_effect_type = DiyEffectType::Gradient;
                    }
                });
            });

            ui.add_space(10.0);

            // "Color" Section (14 circular color swatch pills in a 2x7 grid)
            let (color_card_rect, _) = ui
                .allocate_exact_size(Vec2::new(ui.available_width(), 105.0), egui::Sense::hover());
            let painter = ui.painter_at(color_card_rect);
            painter.rect_filled(
                color_card_rect,
                Rounding::same(14.0),
                Color32::from_rgb(248, 250, 252),
            );
            painter.rect_stroke(
                color_card_rect,
                Rounding::same(14.0),
                Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)),
            );

            painter.text(
                Pos2::new(color_card_rect.min.x + 14.0, color_card_rect.min.y + 10.0),
                egui::Align2::LEFT_TOP,
                "Color",
                egui::FontId::proportional(13.0),
                Color32::from_rgb(71, 85, 105),
            );

            // Render 14 pills in 2 rows of 7
            let pill_spacing_x = (color_card_rect.width() - 28.0) / 7.0;
            let row1_y = color_card_rect.min.y + 45.0;
            let row2_y = color_card_rect.min.y + 80.0;

            for idx in 0..14 {
                let row = idx / 7;
                let col = idx % 7;
                let center_x = color_card_rect.min.x + 14.0 + (col as f32 + 0.5) * pill_spacing_x;
                let center_y = if row == 0 { row1_y } else { row2_y };
                let center = Pos2::new(center_x, center_y);

                let node = &self.diy_nodes[idx];
                let col_c = Color32::from_rgb(node.color.r, node.color.g, node.color.b);

                painter.circle_filled(center, 13.0, col_c);
                painter.circle_stroke(
                    center,
                    13.0,
                    Stroke::new(1.0_f32, Color32::from_black_alpha(40)),
                );

                // Number inside pill
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    format!("{}", node.value),
                    egui::FontId::proportional(10.0),
                    Color32::WHITE,
                );

                // Check click
                let click_rect = Rect::from_center_size(center, Vec2::splat(28.0));
                if ui.allocate_rect(click_rect, egui::Sense::click()).clicked() {
                    self.selected_diy_index = Some(idx);
                }
            }

            ui.add_space(10.0);

            // "Mode" Dropdown Selector (e.g. Chase Forward, Chase Backward, Chase Middle, Chase Out)
            let (mode_rect, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 50.0), egui::Sense::hover());
            let painter = ui.painter_at(mode_rect);
            painter.rect_filled(
                mode_rect,
                Rounding::same(12.0),
                Color32::from_rgb(248, 250, 252),
            );
            painter.rect_stroke(
                mode_rect,
                Rounding::same(12.0),
                Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)),
            );

            painter.text(
                Pos2::new(mode_rect.min.x + 14.0, mode_rect.center().y),
                egui::Align2::LEFT_CENTER,
                "Mode",
                egui::FontId::proportional(14.0),
                Color32::from_rgb(15, 23, 42),
            );

            let mode_lbl = match self.diy_chase_mode {
                DiyChaseMode::Forward => "Chase forward",
                DiyChaseMode::Backward => "Chase backward",
                DiyChaseMode::Middle => "Chase middle",
                DiyChaseMode::Out => "Chase out",
            };

            ui.allocate_ui_at_rect(
                Rect::from_min_max(
                    Pos2::new(mode_rect.max.x - 170.0, mode_rect.min.y + 10.0),
                    Pos2::new(mode_rect.max.x - 10.0, mode_rect.max.y - 10.0),
                ),
                |ui| {
                    egui::ComboBox::from_id_source("diy_chase_mode_cb")
                        .selected_text(mode_lbl)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.diy_chase_mode,
                                DiyChaseMode::Forward,
                                "Chase forward",
                            );
                            ui.selectable_value(
                                &mut self.diy_chase_mode,
                                DiyChaseMode::Backward,
                                "Chase backward",
                            );
                            ui.selectable_value(
                                &mut self.diy_chase_mode,
                                DiyChaseMode::Middle,
                                "Chase middle",
                            );
                            ui.selectable_value(
                                &mut self.diy_chase_mode,
                                DiyChaseMode::Out,
                                "Chase out",
                            );
                        });
                },
            );

            ui.add_space(10.0);

            // Dual Metric Tiles: Brightness (100%) and Speed (10)
            ui.columns(2, |cols| {
                cols[0].group(|ui| {
                    ui.label(
                        egui::RichText::new("Brightness")
                            .size(12.0)
                            .color(Color32::from_rgb(100, 116, 139)),
                    );
                    let pct = (self.diy_brightness as f32 / 255.0 * 100.0) as u32;
                    ui.label(egui::RichText::new(format!("{}%", pct)).strong().size(15.0));
                    ui.add(egui::Slider::new(&mut self.diy_brightness, 0..=255).show_value(false));
                });

                cols[1].group(|ui| {
                    ui.label(
                        egui::RichText::new("Speed")
                            .size(12.0)
                            .color(Color32::from_rgb(100, 116, 139)),
                    );
                    ui.label(
                        egui::RichText::new(format!("{}", self.diy_speed))
                            .strong()
                            .size(15.0)
                            .color(Color32::from_rgb(16, 185, 129)),
                    );
                    ui.add(egui::Slider::new(&mut self.diy_speed, 1..=10).show_value(false));
                });
            });

            ui.add_space(10.0);

            // Bottom Animated Multi-Colour Chase Preview Bar
            let (prev_rect, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 38.0), egui::Sense::hover());
            let painter = ui.painter_at(prev_rect);
            painter.rect_filled(
                prev_rect,
                Rounding::same(19.0),
                Color32::from_rgb(226, 232, 240),
            );

            let step_w = prev_rect.width() / 14.0;
            let offset = (self.animation_timer * 4.0) as usize % 14;

            for i in 0..14 {
                let node_idx = match self.diy_chase_mode {
                    DiyChaseMode::Backward => (14 + offset - (i % 14)) % 14,
                    _ => (i + offset) % 14,
                };
                let n = &self.diy_nodes[node_idx];
                let col = Color32::from_rgb(n.color.r, n.color.g, n.color.b);
                let seg = Rect::from_min_max(
                    Pos2::new(prev_rect.min.x + (i as f32) * step_w, prev_rect.min.y),
                    Pos2::new(prev_rect.min.x + ((i + 1) as f32) * step_w, prev_rect.max.y),
                );
                painter.rect_filled(seg, Rounding::same(4.0), col);
            }
            painter.rect_stroke(
                prev_rect,
                Rounding::same(19.0),
                Stroke::new(1.0_f32, Color32::WHITE),
            );
        });
    }

    /// Renders the signature Large Circular Arc Gauge / Dial with Center Number and Flanking Arrows.
    fn render_arc_dial(
        &mut self,
        ui: &mut egui::Ui,
        current_num: u32,
        title: &str,
        is_dynamic: bool,
    ) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(260.0, 240.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);
        let center = Pos2::new(rect.center().x, rect.min.y + 115.0);
        let radius = 86.0;
        let track_thickness = 13.0_f32;

        // Total arc span: 270 degrees (from 135 deg to 405 deg)
        let start_deg = 135.0f32;
        let sweep_deg = 270.0f32;
        let progress = if is_dynamic {
            (current_num as f32 / 218.0).clamp(0.01, 1.0)
        } else {
            (current_num as f32 / 16.0).clamp(0.01, 1.0)
        };

        // 1. Inactive Arc Track
        let segments = 50;
        for i in 0..segments {
            let a1 = (start_deg + (i as f32 / segments as f32) * sweep_deg).to_radians();
            let a2 = (start_deg + ((i + 1) as f32 / segments as f32) * sweep_deg).to_radians();
            let p1 = Pos2::new(center.x + a1.cos() * radius, center.y + a1.sin() * radius);
            let p2 = Pos2::new(center.x + a2.cos() * radius, center.y + a2.sin() * radius);
            painter.line_segment(
                [p1, p2],
                Stroke::new(track_thickness, Color32::from_rgb(226, 238, 255)),
            );
        }

        // 2. Active Gradient Arc
        let active_segs = ((segments as f32 * progress) as usize).max(1);
        for i in 0..active_segs {
            let a1 = (start_deg + (i as f32 / segments as f32) * sweep_deg).to_radians();
            let a2 = (start_deg + ((i + 1) as f32 / segments as f32) * sweep_deg).to_radians();
            let p1 = Pos2::new(center.x + a1.cos() * radius, center.y + a1.sin() * radius);
            let p2 = Pos2::new(center.x + a2.cos() * radius, center.y + a2.sin() * radius);
            painter.line_segment(
                [p1, p2],
                Stroke::new(track_thickness, Color32::from_rgb(59, 130, 246)),
            );
        }

        // 3. Draggable Thumb Dot on Arc
        let thumb_angle = (start_deg + sweep_deg * progress).to_radians();
        let thumb_pos = Pos2::new(
            center.x + thumb_angle.cos() * radius,
            center.y + thumb_angle.sin() * radius,
        );
        painter.circle_filled(thumb_pos, 10.0, Color32::WHITE);
        painter.circle_stroke(
            thumb_pos,
            10.0,
            Stroke::new(2.0_f32, Color32::from_rgb(59, 130, 246)),
        );

        // 4. Interior Illustration Disc
        painter.circle_filled(center, 70.0, Color32::from_rgb(37, 99, 235));
        painter.circle_stroke(
            center,
            70.0,
            Stroke::new(1.0_f32, Color32::from_white_alpha(50)),
        );

        // Night city skyline / vinyl lines inside disc
        let disc_bot = center.y + 70.0;
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(center.x - 45.0, disc_bot - 38.0),
                Pos2::new(center.x - 25.0, disc_bot),
            ),
            Rounding::ZERO,
            Color32::from_rgb(29, 78, 216),
        );
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(center.x - 20.0, disc_bot - 50.0),
                Pos2::new(center.x + 10.0, disc_bot),
            ),
            Rounding::ZERO,
            Color32::from_rgb(30, 64, 175),
        );
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(center.x + 15.0, disc_bot - 34.0),
                Pos2::new(center.x + 40.0, disc_bot),
            ),
            Rounding::ZERO,
            Color32::from_rgb(29, 78, 216),
        );

        // 5. Large Center Number (e.g. "146" or "5")
        painter.text(
            center,
            egui::Align2::CENTER_CENTER,
            format!("{}", current_num),
            egui::FontId::proportional(38.0),
            Color32::WHITE,
        );

        // 6. Flanking "<" and ">" Step Arrow Buttons
        let left_rect =
            Rect::from_center_size(Pos2::new(center.x - 110.0, center.y), Vec2::splat(32.0));
        let right_rect =
            Rect::from_center_size(Pos2::new(center.x + 110.0, center.y), Vec2::splat(32.0));

        let left_resp = ui.allocate_rect(left_rect, egui::Sense::click());
        let right_resp = ui.allocate_rect(right_rect, egui::Sense::click());

        let lp = ui.painter_at(left_rect);
        lp.circle_filled(left_rect.center(), 15.0, Color32::from_rgb(241, 245, 249));
        lp.text(
            left_rect.center(),
            egui::Align2::CENTER_CENTER,
            "<",
            egui::FontId::proportional(16.0),
            Color32::from_rgb(100, 116, 139),
        );

        let rp = ui.painter_at(right_rect);
        rp.circle_filled(right_rect.center(), 15.0, Color32::from_rgb(241, 245, 249));
        rp.text(
            right_rect.center(),
            egui::Align2::CENTER_CENTER,
            ">",
            egui::FontId::proportional(16.0),
            Color32::from_rgb(100, 116, 139),
        );

        if left_resp.clicked() {
            if is_dynamic {
                self.ui_effect = if self.ui_effect > 1 {
                    self.ui_effect - 1
                } else {
                    218
                };
                self.send(GuiCommand::SendCommand(Command::SetEffect(self.ui_effect)));
            } else {
                self.music_mode_index = if self.music_mode_index > 1 {
                    self.music_mode_index - 1
                } else {
                    16
                };
            }
        }
        if right_resp.clicked() {
            if is_dynamic {
                self.ui_effect = if self.ui_effect < 218 {
                    self.ui_effect + 1
                } else {
                    1
                };
                self.send(GuiCommand::SendCommand(Command::SetEffect(self.ui_effect)));
            } else {
                self.music_mode_index = if self.music_mode_index < 16 {
                    self.music_mode_index + 1
                } else {
                    1
                };
            }
        }

        // 7. Effect / Mode Label below Dial
        painter.text(
            Pos2::new(center.x, rect.max.y - 10.0),
            egui::Align2::CENTER_BOTTOM,
            title,
            egui::FontId::proportional(17.0),
            Color32::from_rgb(15, 23, 42),
        );
    }

    /// Renders the compact 60 FPS live addressable strip visualizer inside the card.
    fn render_neon_strip_compact(&self, ui: &mut egui::Ui) {
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 44.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Dark acrylic strip track
        painter.rect_filled(rect, Rounding::same(10.0), Color32::from_rgb(15, 23, 42));
        painter.rect_stroke(
            rect,
            Rounding::same(10.0),
            Stroke::new(1.0_f32, Color32::from_rgb(51, 65, 85)),
        );

        let count = self.strip_pixels.len();
        if count == 0 {
            return;
        }

        let spacing = (rect.width() - 20.0) / (count as f32);
        let radius = (spacing * 0.36).clamp(3.0, 9.0);
        let center_y = rect.center().y;

        for (i, p) in self.strip_pixels.iter().enumerate() {
            let center = Pos2::new(rect.min.x + 10.0 + spacing * (i as f32 + 0.5), center_y);
            let color = Color32::from_rgb(p.r, p.g, p.b);

            // Glow bloom
            if self.ui_power && (p.r > 20 || p.g > 20 || p.b > 20) {
                let bloom = Color32::from_rgba_premultiplied(p.r / 3, p.g / 3, p.b / 3, 50);
                painter.circle_filled(center, radius * 2.2, bloom);
            }

            // Core diode
            painter.circle_filled(center, radius, color);
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(0.5_f32, Color32::from_black_alpha(100)),
            );

            // Reflection point
            if self.ui_power && (p.r > 50 || p.g > 50 || p.b > 50) {
                painter.circle_filled(center, radius * 0.35, Color32::from_white_alpha(120));
            }
        }
    }

    /// Equalizer bars visualizer.
    fn render_equalizer_bars_clean(&self, ui: &mut egui::Ui) {
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        painter.rect_filled(rect, Rounding::same(8.0), Color32::from_rgb(241, 245, 249));
        painter.rect_stroke(
            rect,
            Rounding::same(8.0),
            Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)),
        );

        let bars = 16;
        let spacing = rect.width() / (bars as f32);
        let bar_w = spacing * 0.65;

        for i in 0..bars {
            let freq = i as f32 * 1.35;
            let height_factor = if self.ui_power {
                if self.sync_with_windows_audio && self.live_audio_peak > 0.01 {
                    let harmonic = (self.audio_visualizer_timer * 12.0 + freq).sin() * 0.35 + 0.65;
                    (self.live_audio_peak * harmonic).clamp(0.08, 0.98)
                } else {
                    let wave = ((self.audio_visualizer_timer * 8.0 + freq).sin() * 0.4 + 0.5)
                        * ((self.audio_visualizer_timer * 3.7 + freq * 2.1).cos() * 0.3 + 0.7);
                    wave.clamp(0.1, 0.95)
                }
            } else {
                0.04
            };

            let bar_h = rect.height() * height_factor * (self.ui_sensitivity as f32 / 16.0);
            let bar_rect = Rect::from_min_size(
                Pos2::new(
                    rect.min.x + spacing * (i as f32 + 0.18),
                    rect.max.y - bar_h - 2.0,
                ),
                Vec2::new(bar_w, bar_h),
            );

            let col = if i < 6 {
                Color32::from_rgb(59, 130, 246) // Blue
            } else if i < 11 {
                Color32::from_rgb(16, 185, 129) // Emerald
            } else {
                Color32::from_rgb(245, 158, 11) // Amber
            };

            painter.rect_filled(bar_rect, Rounding::same(3.0), col);
        }
    }

    /// Renders the Settings Modal Sheet (Hardware Strip Configuration & Safety Gating).
    fn render_settings_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("Controller Settings")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("Hardware & Pixel Configuration")
                        .strong()
                        .size(14.0),
                );
                ui.separator();

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(format!("Strip Length: {} pixels", self.ui_length));
                    ui.add(egui::Slider::new(&mut self.ui_length, 1..=150));
                });

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("RGB Sequence:");
                    egui::ComboBox::from_id_source("chip_order_modal_cb")
                        .selected_text(format!("{:?}", self.ui_chip_order))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.ui_chip_order,
                                ChipOrder::Grb,
                                "GRB (WS2812B Standard)",
                            );
                            ui.selectable_value(
                                &mut self.ui_chip_order,
                                ChipOrder::Rgb,
                                "RGB (Standard)",
                            );
                            ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Rbg, "RBG");
                            ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Gbr, "GBR");
                            ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Brg, "BRG");
                            ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Bgr, "BGR");
                        });
                });

                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(
                        egui::RichText::new("Safety Gate")
                            .strong()
                            .color(Color32::from_rgb(220, 38, 38)),
                    );
                    ui.label("Writing incorrect parameters alters non-volatile flash memory.");
                    ui.checkbox(
                        &mut self.ui_confirm_safety,
                        "I understand this changes the device",
                    );
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    let btn_enabled = self.ui_confirm_safety;
                    if ui
                        .add_enabled(btn_enabled, egui::Button::new("Apply Configuration"))
                        .clicked()
                    {
                        self.send(GuiCommand::SendCommand(Command::SetLength(self.ui_length)));
                        self.send(GuiCommand::SendCommand(Command::SetChipOrder(
                            self.ui_chip_order,
                        )));
                        self.log("Applied hardware parameters to controller", "");
                        self.show_settings_modal = false;
                    }

                    if ui.button("Close").clicked() {
                        self.show_settings_modal = false;
                    }
                });
            });
    }

    /// Renders the Help & Verification Guide Modal.
    fn render_help_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("Hardware & Verification Guide")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("Pyxillum Open-Source Core")
                        .strong()
                        .size(14.0),
                );
                ui.separator();
                ui.label(
                    "Reverse-engineered client for SP621E, SP530E, and SP6xxE LED controllers.",
                );
                ui.label(
                    "Operating under statutory interoperability exemptions with zero telemetry.",
                );
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Features:").strong());
                ui.label("- Dynamic mode with 218 pattern codes");
                ui.label("- 14-node DIY step sequencer");
                ui.label("- Windows native media & audio reactivity");
                ui.label("- 60 FPS in-memory virtual hardware simulator");
                ui.add_space(12.0);
                if ui.button("Close").clicked() {
                    self.show_help_modal = false;
                }
            });
    }

    /// Renders the Effect Drawer / Grid Modal (Listing all 218 patterns).
    fn render_effect_drawer_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("Pattern Library (218 Presets)")
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(420.0, 360.0))
            .show(ctx, |ui| {
                ui.label("Select any built-in pattern code:");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let patterns = [
                        (0x01, "Rainbow Spectrum Wave"),
                        (0x02, "Meteor Trail Stream"),
                        (0x03, "Seven Colour Flow"),
                        (0x07, "Stacking Chasing Dots"),
                        (0x12, "Fire Heat Waves"),
                        (0x19, "Dual Gradual Snake"),
                        (0x2E, "Wave Ripple Pulse"),
                        (0x3A, "Vortex & Comet Spin"),
                        (0x45, "Starfield Sparkle"),
                        (0x5E, "Atmospheric Breath"),
                        (0x72, "Full Color Stacking"),
                        (0x81, "Strobe Flash Pulse"),
                        (0x88, "Comet Spin"),
                        (0x8E, "Dot Spin"),
                        (0x92, "Gradient Flow (Code 146)"),
                        (0x95, "Bouncing Light Ball"),
                        (0x9A, "Color Twinkles"),
                        (0xA1, "Drip Flow"),
                    ];

                    for (code, name) in patterns {
                        if ui
                            .selectable_label(
                                self.ui_effect == code,
                                format!("[0x{:02X} - {}] {}", code, code, name),
                            )
                            .clicked()
                        {
                            self.ui_effect = code;
                            self.send(GuiCommand::SendCommand(Command::SetEffect(code)));
                            self.show_effect_drawer = false;
                        }
                    }
                });

                ui.add_space(8.0);
                if ui.button("Close").clicked() {
                    self.show_effect_drawer = false;
                }
            });
    }

    /// Updates internal pixel buffer for the 60 FPS animation visualizer.
    fn update_virtual_pixels(&mut self) {
        let num_pixels = self.ui_length as usize;
        self.strip_pixels.clear();

        if !self.ui_power {
            self.strip_pixels.resize(num_pixels, RgbColor::BLACK);
            return;
        }

        let mut factor = (self.ui_brightness as f32) / 255.0;

        if self.dashboard_mode == DashboardMode::Music
            && self.sync_with_windows_audio
            && self.live_audio_peak > 0.01
        {
            factor = (factor * (0.35 + 0.65 * self.live_audio_peak)).clamp(0.1, 1.0);
        }

        match self.dashboard_mode {
            DashboardMode::Solid => {
                let r = (self.ui_color[0] * 255.0 * factor) as u8;
                let g = (self.ui_color[1] * 255.0 * factor) as u8;
                let b = (self.ui_color[2] * 255.0 * factor) as u8;
                self.strip_pixels.resize(num_pixels, RgbColor::new(r, g, b));
            }
            DashboardMode::Music => {
                for i in 0..num_pixels {
                    let beat_intensity =
                        if self.sync_with_windows_audio && self.live_audio_peak > 0.01 {
                            self.live_audio_peak
                        } else {
                            (self.animation_timer * 10.0).sin().abs()
                        };
                    let beat = (beat_intensity * factor).clamp(0.05, 1.0);

                    let (r, g, b) = if self.music_mode_index == 3 {
                        // VuMeter: green -> yellow -> red
                        let norm = i as f32 / num_pixels as f32;
                        if norm < beat * 0.6 {
                            (0, 220, 100)
                        } else if norm < beat * 0.85 {
                            (255, 200, 0)
                        } else if norm < beat {
                            (255, 40, 40)
                        } else {
                            (15, 20, 30)
                        }
                    } else {
                        let (r_b, g_b, b_b) = hsv_to_rgb((i as f32 * 12.0) % 360.0, 1.0, beat);
                        (r_b, g_b, b_b)
                    };
                    self.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            DashboardMode::Diy => {
                let offset = (self.animation_timer * (self.diy_speed as f32 * 0.5)) as usize % 14;
                for i in 0..num_pixels {
                    let node_idx = match self.diy_chase_mode {
                        DiyChaseMode::Backward => (14 + offset - (i % 14)) % 14,
                        _ => (i + offset) % 14,
                    };
                    let n = &self.diy_nodes[node_idx];
                    let r = ((n.color.r as f32) * factor) as u8;
                    let g = ((n.color.g as f32) * factor) as u8;
                    let b = ((n.color.b as f32) * factor) as u8;
                    self.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            DashboardMode::Dynamic => {
                match self.ui_effect {
                    0x01 => {
                        // Rainbow Spectrum Wave
                        for i in 0..num_pixels {
                            let hue = (self.animation_timer * 15.0
                                + (i as f32 * 360.0 / num_pixels as f32))
                                % 360.0;
                            let (r, g, b) = hsv_to_rgb(hue, 1.0, factor);
                            self.strip_pixels.push(RgbColor::new(r, g, b));
                        }
                    }
                    0x02 => {
                        // Meteor Trail Stream
                        let head = ((self.animation_timer * 12.0) as usize) % num_pixels.max(1);
                        for i in 0..num_pixels {
                            let dist = (num_pixels + head - i) % num_pixels;
                            let decay = if dist < 8 {
                                (1.0 - (dist as f32 / 8.0)) * factor
                            } else {
                                0.05 * factor
                            };
                            let r = (self.ui_color[0] * 255.0 * decay) as u8;
                            let g = (self.ui_color[1] * 255.0 * decay) as u8;
                            let b = (self.ui_color[2] * 255.0 * decay) as u8;
                            self.strip_pixels.push(RgbColor::new(r, g, b));
                        }
                    }
                    0x12 => {
                        // Fire Heat Waves
                        for i in 0..num_pixels {
                            let flicker = ((self.animation_timer * 20.0 + (i as f32 * 1.7)).sin()
                                * 0.25
                                + 0.75)
                                * factor;
                            let r = (255.0 * flicker) as u8;
                            let g = (80.0 * flicker) as u8;
                            let b = (10.0 * flicker) as u8;
                            self.strip_pixels.push(RgbColor::new(r, g, b));
                        }
                    }
                    _ => {
                        // Gradient Flow / Wave Ripple
                        for i in 0..num_pixels {
                            let wave =
                                (((self.animation_timer * 6.0 + i as f32 * 0.5).sin() + 1.0) * 0.5)
                                    * factor;
                            let hue = (self.animation_timer * 10.0 + i as f32 * 8.0) % 360.0;
                            let (r, g, b) = hsv_to_rgb(hue, 0.85, wave);
                            self.strip_pixels.push(RgbColor::new(r, g, b));
                        }
                    }
                }
            }
        }
    }
}

/// Helper that renders a circular action button (e.g. Loop, Play, Fav, List).
fn render_round_action_button(ui: &mut egui::Ui, text: &str, is_active: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(40.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let bg = if is_active {
        Color32::from_rgb(239, 246, 255)
    } else {
        Color32::from_rgb(248, 250, 252)
    };
    let border = if is_active {
        Color32::from_rgb(59, 130, 246)
    } else {
        Color32::from_rgb(226, 232, 240)
    };
    let txt_col = if is_active {
        Color32::from_rgb(37, 99, 235)
    } else {
        Color32::from_rgb(100, 116, 139)
    };

    p.circle_filled(rect.center(), 19.0, bg);
    p.circle_stroke(rect.center(), 19.0, Stroke::new(1.0_f32, border));
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(12.0),
        txt_col,
    );

    resp
}

/// Helper that renders a sound source switcher pill.
fn render_source_pill(ui: &mut egui::Ui, text: &str, is_selected: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(72.0, 32.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let bg = if is_selected {
        Color32::from_rgb(59, 130, 246)
    } else {
        Color32::from_rgb(241, 245, 249)
    };
    let txt_col = if is_selected {
        Color32::WHITE
    } else {
        Color32::from_rgb(100, 116, 139)
    };

    p.rect_filled(rect, Rounding::same(16.0), bg);
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(10.0),
        txt_col,
    );

    resp
}

/// Helper to get human-readable pattern name for an effect code.
fn get_effect_title(code: u8) -> String {
    match code {
        0x01 => "Rainbow Spectrum".to_string(),
        0x02 => "Meteor Trail".to_string(),
        0x03 => "Seven Colour Flow".to_string(),
        0x07 => "Chasing Dots".to_string(),
        0x12 => "Fire Heat Waves".to_string(),
        0x19 => "Dual Snake".to_string(),
        0x2E => "Wave Ripple".to_string(),
        0x3A => "Vortex Spin".to_string(),
        0x45 => "Starfield Sparkle".to_string(),
        0x5E => "Atmospheric Breath".to_string(),
        0x72 => "Color Stacking".to_string(),
        0x81 => "Strobe Flash".to_string(),
        146 => "Gradient".to_string(),
        0x95 => "Bouncing Light Ball".to_string(),
        other => format!("Effect Pattern {}", other),
    }
}

/// Draws an interactive continuous radial Hue & Saturation color disc.
fn render_color_disc(ui: &mut egui::Ui, current_color: &mut [f32; 3], radius: f32) -> bool {
    let size = Vec2::splat(radius * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    let center = rect.center();

    let segments = 64;
    let mut mesh = egui::Mesh::default();
    let center_idx = mesh.vertices.len() as u32;
    mesh.colored_vertex(center, Color32::WHITE);

    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

        let p1 = center + Vec2::new(angle1.cos(), angle1.sin()) * radius;
        let p2 = center + Vec2::new(angle2.cos(), angle2.sin()) * radius;

        let hue1 = (angle1.to_degrees() + 360.0) % 360.0;
        let hue2 = (angle2.to_degrees() + 360.0) % 360.0;

        let (r1, g1, b1) = hsv_to_rgb(hue1, 1.0, 1.0);
        let (r2, g2, b2) = hsv_to_rgb(hue2, 1.0, 1.0);

        let c1 = Color32::from_rgb(r1, g1, b1);
        let c2 = Color32::from_rgb(r2, g2, b2);

        let idx1 = mesh.vertices.len() as u32;
        mesh.colored_vertex(p1, c1);
        let idx2 = mesh.vertices.len() as u32;
        mesh.colored_vertex(p2, c2);

        mesh.add_triangle(center_idx, idx1, idx2);
    }

    painter.add(egui::Shape::mesh(mesh));

    // Outer border ring
    painter.circle_stroke(
        center,
        radius,
        Stroke::new(1.5_f32, Color32::from_rgb(203, 213, 225)),
    );

    let (curr_h, curr_s, _) = rgb_to_hsv(current_color[0], current_color[1], current_color[2]);
    let curr_angle = curr_h.to_radians();
    let indicator_pos = center + Vec2::new(curr_angle.cos(), curr_angle.sin()) * (curr_s * radius);

    painter.circle_stroke(indicator_pos, 8.5, Stroke::new(2.5_f32, Color32::WHITE));
    painter.circle_stroke(indicator_pos, 10.0, Stroke::new(1.0_f32, Color32::BLACK));

    let mut changed = false;
    if response.dragged() || response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            let delta = pos - center;
            let dist = delta.length();
            let angle = delta.y.atan2(delta.x);
            let hue = (angle.to_degrees() + 360.0) % 360.0;
            let sat = (dist / radius).clamp(0.0, 1.0);

            let (r, g, b) = hsv_to_rgb(hue, sat, 1.0);
            current_color[0] = r as f32 / 255.0;
            current_color[1] = g as f32 / 255.0;
            current_color[2] = b as f32 / 255.0;
            changed = true;
        }
    }

    changed
}

fn hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let v = max;
    let s = if max > 0.0 { delta / max } else { 0.0 };

    let h = if delta == 0.0 {
        0.0
    } else if (max - r).abs() < f32::EPSILON {
        60.0 * (((g - b) / delta) % 6.0)
    } else if (max - g).abs() < f32::EPSILON {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    (h, s, v)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_prime + m) * 255.0) as u8,
        ((g_prime + m) * 255.0) as u8,
        ((b_prime + m) * 255.0) as u8,
    )
}

fn generate_window_icon() -> egui::IconData {
    egui::IconData {
        rgba: icon_data::APP_ICON_RGBA_32.to_vec(),
        width: 32,
        height: 32,
    }
}

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<GuiCommand>(64);

    let sim = Arc::new(Mutex::new(DeviceSimulator::new()));
    let sim_transport = Box::new(SimTransport::new(sim.clone()));
    let manager = Arc::new(DeviceManager::new(sim_transport));

    let state_reader = manager.state();

    let manager_clone = manager.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            info!("Async DeviceManager worker started.");
            let _ = manager_clone.connect("SIM:00:11:22:33:44:55").await;

            while let Some(cmd) = cmd_rx.recv().await {
                match cmd {
                    GuiCommand::Scan => {
                        let _ = manager_clone.scan().await;
                    }
                    GuiCommand::Connect(addr) => {
                        let _ = manager_clone.connect(&addr).await;
                    }
                    GuiCommand::Disconnect => {
                        let _ = manager_clone.disconnect().await;
                    }
                    GuiCommand::SendCommand(c) => {
                        let _ = manager_clone.send_command(c).await;
                    }
                }
            }
        });
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([460.0, 780.0])
            .with_min_inner_size([400.0, 680.0])
            .with_title("Pyxillum")
            .with_icon(generate_window_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Pyxillum",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(PyxillumApp::new(
                cc,
                cmd_tx,
                state_reader,
                Some(sim),
            )))
        }),
    )
}
