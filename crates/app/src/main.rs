#![warn(missing_docs)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Pyxillum Desktop Application
//!
//! Native cross-platform GUI for controlling Bluetooth Low Energy pixel LED controllers.
//! Features a modern glassmorphic Material/iOS-inspired interface, 60 FPS neon strip
//! simulation, and native Windows audio and media session integration.

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
enum GuiCommand {
    Scan,
    Connect(String),
    Disconnect,
    SendCommand(Command),
}

#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: String,
    message: String,
    hex: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveTab {
    Illumination,
    DynamicEffects,
    MusicReactive,
    CustomScenes,
    HardwareSetup,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlowDirection {
    Forward,
    Reverse,
    Inward,
    Outward,
}

struct PyxillumApp {
    cmd_tx: mpsc::Sender<GuiCommand>,
    _state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
    _simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    audio_service: WindowsAudioService,

    // UI Local State
    is_demo_mode: bool,
    _discovered: Vec<DiscoveredDevice>,
    _selected_device: Option<String>,
    logs: Vec<LogEntry>,
    active_tab: ActiveTab,
    flow_direction: FlowDirection,
    sync_with_windows_audio: bool,

    // Controls
    ui_power: bool,
    ui_color: [f32; 3],
    ui_brightness: u8,
    ui_speed: u8,
    ui_length: u8,
    ui_effect: u8,
    ui_chip_order: ChipOrder,
    ui_audio_input: AudioInput,
    ui_sensitivity: u8,
    ui_confirm_safety: bool,

    // Live Windows Media State
    now_playing: NowPlayingInfo,
    live_audio_peak: f32,

    // Animation preview
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
        // Configure refined glassmorphic visuals with frosted translucency
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(Color32::from_rgb(242, 246, 252));
        visuals.panel_fill = Color32::from_rgba_premultiplied(12, 16, 24, 240);
        visuals.window_fill = Color32::from_rgba_premultiplied(18, 24, 36, 230);
        visuals.extreme_bg_color = Color32::from_rgb(8, 11, 16);
        visuals.faint_bg_color = Color32::from_rgba_premultiplied(24, 32, 48, 180);

        visuals.widgets.noninteractive.bg_fill = Color32::from_rgba_premultiplied(20, 26, 40, 200);
        visuals.widgets.noninteractive.bg_stroke =
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(65, 85, 125, 70));

        visuals.widgets.inactive.bg_fill = Color32::from_rgba_premultiplied(26, 34, 52, 210);
        visuals.widgets.inactive.bg_stroke =
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(75, 95, 140, 90));

        visuals.widgets.hovered.bg_fill = Color32::from_rgba_premultiplied(38, 52, 80, 230);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.5_f32, Color32::from_rgb(0, 210, 255));

        visuals.widgets.active.bg_fill = Color32::from_rgb(0, 160, 225);
        visuals.selection.bg_fill = Color32::from_rgb(0, 180, 235);

        visuals.widgets.inactive.rounding = Rounding::same(8.0);
        visuals.widgets.hovered.rounding = Rounding::same(8.0);
        visuals.widgets.active.rounding = Rounding::same(8.0);
        visuals.window_rounding = Rounding::same(12.0);
        cc.egui_ctx.set_visuals(visuals);

        let audio_service = WindowsAudioService::new();

        Self {
            cmd_tx,
            _state_reader: state_reader,
            _simulator_ref: simulator_ref,
            audio_service,
            is_demo_mode: true,
            _discovered: Vec::new(),
            _selected_device: None,
            logs: vec![LogEntry {
                timestamp: "00:00:00".to_string(),
                message: "Pyxillum initialized in offline-first glassmorphism mode.".to_string(),
                hex: String::new(),
            }],
            active_tab: ActiveTab::Illumination,
            flow_direction: FlowDirection::Forward,
            sync_with_windows_audio: true,
            ui_power: true,
            ui_color: [1.0, 0.47, 0.0], // Amber
            ui_brightness: 255,
            ui_speed: 5,
            ui_length: 30,
            ui_effect: EFFECT_SOLID,
            ui_chip_order: ChipOrder::Grb,
            ui_audio_input: AudioInput::InternalMic,
            ui_sensitivity: 8,
            ui_confirm_safety: false,
            now_playing: NowPlayingInfo::default(),
            live_audio_peak: 0.0,
            last_frame_instant: Instant::now(),
            animation_timer: 0.0,
            audio_visualizer_timer: 0.0,
            strip_pixels: vec![RgbColor::new(255, 120, 0); 30],
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
        // Delta time for 60 FPS animation preview
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_instant).as_secs_f32();
        self.last_frame_instant = now;
        self.animation_timer += dt * (self.ui_speed as f32);
        self.audio_visualizer_timer += dt;

        // Query Windows native audio peak & media status
        let (now_playing, peak) = self.audio_service.get_state();
        self.now_playing = now_playing;
        self.live_audio_peak = peak;

        // Update virtual strip pixels
        self.update_virtual_pixels();

        // Keyboard shortcut: Space toggles power
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.ui_power = !self.ui_power;
            let cmd = Command::Power(self.ui_power);
            self.log(
                &format!("Keyboard shortcut toggled power: {}", self.ui_power),
                &hex_string(&cmd.encode()),
            );
            self.send(GuiCommand::SendCommand(cmd));
        }

        // Top Navigation Bar (Frosted Glass Header)
        egui::TopBottomPanel::top("header_panel")
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgba_premultiplied(12, 16, 24, 235))
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Brand and Logo Title
                    ui.label(
                        egui::RichText::new("PYXILLUM")
                            .strong()
                            .size(18.0)
                            .color(Color32::from_rgb(0, 215, 255)),
                    );
                    ui.label(
                        egui::RichText::new("Pixel LED Studio")
                            .size(11.0)
                            .color(Color32::from_rgb(140, 155, 180)),
                    );

                    ui.add_space(14.0);

                    // Dynamic Now Playing Pill or Device Connection Status
                    if !self.now_playing.title.is_empty() && self.now_playing.is_playing {
                        let media_pill = format!(
                            "NOW PLAYING: {} - {}",
                            self.now_playing.title, self.now_playing.artist
                        );
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::new(320.0, 26.0), egui::Sense::hover());
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(
                            rect,
                            Rounding::same(13.0),
                            Color32::from_rgba_premultiplied(0, 140, 220, 45),
                        );
                        painter.rect_stroke(
                            rect,
                            Rounding::same(13.0),
                            Stroke::new(
                                1.0_f32,
                                Color32::from_rgba_premultiplied(0, 215, 255, 120),
                            ),
                        );
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            media_pill,
                            egui::FontId::proportional(11.0),
                            Color32::from_rgb(0, 230, 255),
                        );
                    } else {
                        let (status_text, status_col) = if self.is_demo_mode {
                            (
                                "Virtual Simulator Active [SIM:00:11:22:33:44:55]",
                                Color32::from_rgb(52, 211, 153),
                            )
                        } else {
                            (
                                "Hardware Peripheral [Disconnected]",
                                Color32::from_rgb(251, 146, 60),
                            )
                        };
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::new(300.0, 26.0), egui::Sense::hover());
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(
                            rect,
                            Rounding::same(13.0),
                            Color32::from_rgba_premultiplied(22, 28, 42, 180),
                        );
                        painter.rect_stroke(
                            rect,
                            Rounding::same(13.0),
                            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(50, 65, 95, 90)),
                        );
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            status_text,
                            egui::FontId::proportional(11.0),
                            status_col,
                        );
                    }

                    // Right Actions: Power Toggle and Peripheral Scanner
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Glassmorphic Power Button
                        let pwr_lbl = if self.ui_power {
                            "POWER ACTIVE"
                        } else {
                            "STANDBY"
                        };
                        let pwr_bg = if self.ui_power {
                            Color32::from_rgba_premultiplied(16, 185, 129, 210)
                        } else {
                            Color32::from_rgba_premultiplied(45, 55, 75, 180)
                        };

                        let (btn_rect, btn_resp) =
                            ui.allocate_exact_size(Vec2::new(125.0, 32.0), egui::Sense::click());
                        let painter = ui.painter_at(btn_rect);

                        // Specular glass bevel
                        painter.rect_filled(btn_rect, Rounding::same(16.0), pwr_bg);
                        painter.rect_stroke(
                            btn_rect,
                            Rounding::same(16.0),
                            Stroke::new(
                                1.0_f32,
                                if self.ui_power {
                                    Color32::from_rgb(110, 231, 183)
                                } else {
                                    Color32::from_rgba_premultiplied(85, 105, 140, 100)
                                },
                            ),
                        );
                        painter.text(
                            btn_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            pwr_lbl,
                            egui::FontId::proportional(12.0),
                            Color32::WHITE,
                        );

                        if btn_resp.clicked() {
                            self.ui_power = !self.ui_power;
                            let cmd = Command::Power(self.ui_power);
                            self.log(
                                &format!("Toggled power state: {}", self.ui_power),
                                &hex_string(&cmd.encode()),
                            );
                            self.send(GuiCommand::SendCommand(cmd));
                        }

                        ui.add_space(8.0);

                        if ui
                            .button(
                                egui::RichText::new("Scan BLE")
                                    .size(12.0)
                                    .color(Color32::from_rgb(205, 220, 240)),
                            )
                            .clicked()
                        {
                            self.send(GuiCommand::Scan);
                            self.log("Scanning for Bluetooth peripherals...", "");
                        }

                        if ui
                            .selectable_label(self.is_demo_mode, "Simulation Mode")
                            .clicked()
                        {
                            self.is_demo_mode = !self.is_demo_mode;
                            if self.is_demo_mode {
                                self.send(GuiCommand::Connect("SIM:00:11:22:33:44:55".to_string()));
                                self.log("Connected to virtual hardware simulator", "");
                            } else {
                                self.send(GuiCommand::Disconnect);
                                self.log("Disconnected from simulator", "");
                            }
                        }
                    });
                });
            });

        // Main Canvas Panel with Ambient Background Lighting Pools
        egui::CentralPanel::default().show(ctx, |ui| {
            // Paint ambient background light pools for authentic glassmorphism refraction
            let canvas_rect = ui.max_rect();
            let bg_painter = ui.painter();

            // Top-right diffuse cyan orb
            bg_painter.circle_filled(
                Pos2::new(canvas_rect.max.x - 120.0, canvas_rect.min.y + 140.0),
                300.0,
                Color32::from_rgba_premultiplied(0, 150, 220, 22),
            );
            // Bottom-left diffuse violet orb
            bg_painter.circle_filled(
                Pos2::new(canvas_rect.min.x + 140.0, canvas_rect.max.y - 120.0),
                260.0,
                Color32::from_rgba_premultiplied(130, 50, 200, 18),
            );
            // Center warm amber orb
            bg_painter.circle_filled(
                Pos2::new(canvas_rect.center().x, canvas_rect.center().y + 60.0),
                220.0,
                Color32::from_rgba_premultiplied(255, 140, 0, 12),
            );

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(6.0);

                // Section 1: Glass Neon Tube LED Visualizer
                self.render_glass_neon_strip(ui);

                ui.add_space(10.0);

                // Section 2: Segmented Glass Navigation Bar
                self.render_navigation_bar(ui);

                ui.add_space(12.0);

                // Section 3: Active Tab Content
                match self.active_tab {
                    ActiveTab::Illumination => self.render_tab_illumination(ui),
                    ActiveTab::DynamicEffects => self.render_tab_effects(ui),
                    ActiveTab::MusicReactive => self.render_tab_music(ui),
                    ActiveTab::CustomScenes => self.render_tab_custom_scenes(ui),
                    ActiveTab::HardwareSetup => self.render_tab_hardware(ui),
                    ActiveTab::Diagnostics => self.render_tab_diagnostics(ui),
                }
            });
        });

        // Maintain continuous 60 FPS repaint cadence for fluid neon strip rendering
        ctx.request_repaint();
    }
}

impl PyxillumApp {
    /// Renders the addressable LED visualizer inside a frosted glass neon tube enclosure.
    fn render_glass_neon_strip(&self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 68.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Frosted glass tube container
        painter.rect_filled(
            rect,
            Rounding::same(14.0),
            Color32::from_rgba_premultiplied(16, 22, 34, 210),
        );
        // Specular top highlight line on glass edge
        painter.line_segment(
            [
                Pos2::new(rect.min.x + 14.0, rect.min.y + 1.0),
                Pos2::new(rect.max.x - 14.0, rect.min.y + 1.0),
            ],
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 45)),
        );
        // Translucent frosted border
        painter.rect_stroke(
            rect,
            Rounding::same(14.0),
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(75, 95, 140, 80)),
        );

        // Header caption inside tube
        let pwr_lbl = if self.ui_power { "ACTIVE" } else { "STANDBY" };
        let pwr_col = if self.ui_power {
            Color32::from_rgb(52, 211, 153)
        } else {
            Color32::from_rgb(156, 163, 175)
        };
        painter.text(
            Pos2::new(rect.min.x + 14.0, rect.min.y + 8.0),
            egui::Align2::LEFT_TOP,
            "Frosted Glass Neon Strip Monitor",
            egui::FontId::proportional(11.0),
            Color32::from_rgb(180, 195, 215),
        );
        painter.text(
            Pos2::new(rect.max.x - 14.0, rect.min.y + 8.0),
            egui::Align2::RIGHT_TOP,
            format!("{} Pixels | Power: {} | 60 FPS", self.ui_length, pwr_lbl),
            egui::FontId::proportional(11.0),
            pwr_col,
        );

        let count = self.strip_pixels.len();
        if count == 0 {
            return;
        }

        let spacing = (rect.width() - 28.0) / (count as f32);
        let radius = (spacing * 0.38).clamp(3.5, 11.0);
        let center_y = rect.min.y + 40.0;

        for (i, p) in self.strip_pixels.iter().enumerate() {
            let center = Pos2::new(rect.min.x + 14.0 + spacing * (i as f32 + 0.5), center_y);
            let color = Color32::from_rgb(p.r, p.g, p.b);

            // Wide diffuse neon aura bloom through frosted glass
            if self.ui_power && (p.r > 15 || p.g > 15 || p.b > 15) {
                let bloom_color = Color32::from_rgba_premultiplied(p.r / 3, p.g / 3, p.b / 3, 50);
                painter.circle_filled(center, radius * 2.4, bloom_color);
            }

            // Core LED diode
            painter.circle_filled(center, radius, color);
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(0.5_f32, Color32::from_black_alpha(110)),
            );

            // Specular crystal core point
            if self.ui_power && (p.r > 40 || p.g > 40 || p.b > 40) {
                painter.circle_filled(center, radius * 0.36, Color32::from_white_alpha(140));
            }
        }
    }

    /// Renders the floating segmented glass navigation island.
    fn render_navigation_bar(&mut self, ui: &mut egui::Ui) {
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 42.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Translucent floating glass bar
        painter.rect_filled(
            rect,
            Rounding::same(21.0),
            Color32::from_rgba_premultiplied(18, 24, 36, 190),
        );
        painter.line_segment(
            [
                Pos2::new(rect.min.x + 21.0, rect.min.y + 1.0),
                Pos2::new(rect.max.x - 21.0, rect.min.y + 1.0),
            ],
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 35)),
        );
        painter.rect_stroke(
            rect,
            Rounding::same(21.0),
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(65, 85, 125, 75)),
        );

        let tabs = [
            (ActiveTab::Illumination, "Illumination"),
            (ActiveTab::DynamicEffects, "Dynamic Scenes"),
            (ActiveTab::MusicReactive, "Windows Audio & Music"),
            (ActiveTab::CustomScenes, "DIY Palettes"),
            (ActiveTab::HardwareSetup, "Hardware Config"),
            (ActiveTab::Diagnostics, "Diagnostics"),
        ];

        let tab_count = tabs.len() as f32;
        let tab_width = (rect.width() - 8.0) / tab_count;

        for (i, (tab, label)) in tabs.into_iter().enumerate() {
            let tab_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + 4.0 + (i as f32) * tab_width, rect.min.y + 4.0),
                Vec2::new(tab_width, 34.0),
            );

            let is_selected = self.active_tab == tab;
            let tab_resp = ui.allocate_rect(tab_rect, egui::Sense::click());

            if is_selected {
                painter.rect_filled(
                    tab_rect,
                    Rounding::same(17.0),
                    Color32::from_rgb(0, 150, 220),
                );
                painter.rect_stroke(
                    tab_rect,
                    Rounding::same(17.0),
                    Stroke::new(1.0_f32, Color32::from_rgb(125, 225, 255)),
                );
            } else if tab_resp.hovered() {
                painter.rect_filled(
                    tab_rect,
                    Rounding::same(17.0),
                    Color32::from_rgba_premultiplied(35, 48, 72, 160),
                );
            }

            let text_color = if is_selected {
                Color32::WHITE
            } else {
                Color32::from_rgb(165, 180, 205)
            };

            painter.text(
                tab_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(12.0),
                text_color,
            );

            if tab_resp.clicked() {
                self.active_tab = tab;
            }
        }
    }

    /// Renders the Illumination screen with glassmorphic cards and circular disc.
    fn render_tab_illumination(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            // Left Column: Radial Hue-Saturation Glass Disc
            render_glass_card(&mut columns[0], 360.0, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Radial Hue & Saturation Disc")
                            .strong()
                            .size(13.0)
                            .color(Color32::from_rgb(215, 230, 250)),
                    );
                    ui.label(
                        egui::RichText::new("Drag on the disc to pick chromatic light colour")
                            .size(11.0)
                            .color(Color32::from_rgb(135, 150, 175)),
                    );

                    ui.add_space(8.0);

                    // Circular Hue-Saturation Disc
                    if render_color_disc(ui, &mut self.ui_color, 95.0) {
                        self.ui_effect = EFFECT_SOLID;
                        let r = (self.ui_color[0] * 255.0) as u8;
                        let g = (self.ui_color[1] * 255.0) as u8;
                        let b = (self.ui_color[2] * 255.0) as u8;
                        let cmd = Command::SetColor {
                            color: RgbColor::new(r, g, b),
                            level: self.ui_brightness,
                        };
                        self.send(GuiCommand::SendCommand(cmd));
                    }

                    ui.add_space(10.0);

                    // Active Colour Preview Swatch with Glass Bevel
                    let r = (self.ui_color[0] * 255.0) as u8;
                    let g = (self.ui_color[1] * 255.0) as u8;
                    let b = (self.ui_color[2] * 255.0) as u8;
                    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

                    let (preview_rect, _) =
                        ui.allocate_exact_size(Vec2::new(210.0, 34.0), egui::Sense::hover());
                    let painter = ui.painter_at(preview_rect);
                    painter.rect_filled(
                        preview_rect,
                        Rounding::same(10.0),
                        Color32::from_rgb(r, g, b),
                    );
                    painter.rect_stroke(
                        preview_rect,
                        Rounding::same(10.0),
                        Stroke::new(1.0_f32, Color32::from_white_alpha(180)),
                    );

                    let text_col =
                        if (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000 > 128 {
                            Color32::BLACK
                        } else {
                            Color32::WHITE
                        };
                    painter.text(
                        preview_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}  RGB({}, {}, {})", hex, r, g, b),
                        egui::FontId::proportional(12.0),
                        text_col,
                    );
                });
            });

            // Right Column: Master Brightness & Chromatic Glass Swatches
            render_glass_card(&mut columns[1], 360.0, |ui| {
                ui.label(
                    egui::RichText::new("Master Brightness & Quick Swatches")
                        .strong()
                        .size(13.0)
                        .color(Color32::from_rgb(215, 230, 250)),
                );
                ui.separator();

                ui.add_space(6.0);
                let pct = (self.ui_brightness as f32 / 255.0 * 100.0) as u32;
                ui.label(format!(
                    "Brightness Level: {}/255 ({}%)",
                    self.ui_brightness, pct
                ));

                if ui
                    .add(egui::Slider::new(&mut self.ui_brightness, 0..=255).show_value(false))
                    .changed()
                {
                    let cmd = Command::SetBrightness(self.ui_brightness);
                    self.send(GuiCommand::SendCommand(cmd));
                }

                // Quick brightness step buttons
                ui.horizontal(|ui| {
                    let steps = [("25%", 64), ("50%", 128), ("75%", 192), ("100%", 255)];
                    for (lbl, val) in steps {
                        if ui.button(lbl).clicked() {
                            self.ui_brightness = val;
                            self.send(GuiCommand::SendCommand(Command::SetBrightness(val)));
                        }
                    }
                });

                ui.add_space(14.0);
                ui.label(
                    egui::RichText::new("Chromatic Quick Swatches")
                        .strong()
                        .size(12.0)
                        .color(Color32::from_rgb(180, 195, 215)),
                );
                ui.separator();

                let swatches = [
                    ("Amber", [1.0, 0.47, 0.0], Color32::from_rgb(255, 120, 0)),
                    ("Coral", [1.0, 0.2, 0.3], Color32::from_rgb(255, 50, 75)),
                    ("Gold", [1.0, 0.8, 0.0], Color32::from_rgb(255, 204, 0)),
                    ("Emerald", [0.0, 0.88, 0.35], Color32::from_rgb(0, 224, 90)),
                    ("Cyan", [0.0, 0.85, 1.0], Color32::from_rgb(0, 215, 255)),
                    ("Azure", [0.0, 0.45, 1.0], Color32::from_rgb(0, 115, 255)),
                    ("Violet", [0.45, 0.2, 1.0], Color32::from_rgb(115, 50, 255)),
                    ("Magenta", [1.0, 0.0, 0.6], Color32::from_rgb(255, 0, 150)),
                    (
                        "Warm White",
                        [1.0, 0.88, 0.7],
                        Color32::from_rgb(255, 225, 180),
                    ),
                    (
                        "Pure White",
                        [1.0, 1.0, 1.0],
                        Color32::from_rgb(255, 255, 255),
                    ),
                ];

                ui.horizontal_wrapped(|ui| {
                    for (_label, rgb_arr, c32) in swatches {
                        let (rect, response) =
                            ui.allocate_exact_size(Vec2::new(34.0, 34.0), egui::Sense::click());
                        let painter = ui.painter_at(rect);
                        painter.circle_filled(rect.center(), 14.5, c32);

                        let is_active = (self.ui_color[0] - rgb_arr[0]).abs() < 0.05
                            && (self.ui_color[1] - rgb_arr[1]).abs() < 0.05
                            && (self.ui_color[2] - rgb_arr[2]).abs() < 0.05;

                        if is_active {
                            painter.circle_stroke(
                                rect.center(),
                                16.5,
                                Stroke::new(2.5_f32, Color32::WHITE),
                            );
                        } else {
                            painter.circle_stroke(
                                rect.center(),
                                15.0,
                                Stroke::new(
                                    1.0_f32,
                                    Color32::from_rgba_premultiplied(70, 85, 120, 80),
                                ),
                            );
                        }

                        if response.clicked() {
                            self.ui_color = rgb_arr;
                            self.ui_effect = EFFECT_SOLID;
                            let r = (rgb_arr[0] * 255.0) as u8;
                            let g = (rgb_arr[1] * 255.0) as u8;
                            let b = (rgb_arr[2] * 255.0) as u8;
                            let cmd = Command::SetColor {
                                color: RgbColor::new(r, g, b),
                                level: self.ui_brightness,
                            };
                            self.send(GuiCommand::SendCommand(cmd));
                        }
                    }
                });
            });
        });
    }

    /// Renders the Dynamic Effects screen with frosted pattern cards.
    fn render_tab_effects(&mut self, ui: &mut egui::Ui) {
        render_glass_card(ui, 420.0, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Flow Dynamics & Direction Controls")
                        .strong()
                        .size(13.0)
                        .color(Color32::from_rgb(215, 230, 250)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Auto-Cycle Dynamic Mode").clicked() {
                        self.send(GuiCommand::SendCommand(Command::SetLightMode(
                            LightMode::AutoDynamic,
                        )));
                        self.log("Activated operating mode: Auto Dynamic Cycle", "");
                    }
                    if ui.button("Single Pattern Mode").clicked() {
                        self.send(GuiCommand::SendCommand(Command::SetLightMode(
                            LightMode::Single,
                        )));
                        self.log("Activated operating mode: Single Pattern", "");
                    }
                });
            });
            ui.separator();

            // Direction Buttons and Speed Gauge
            ui.horizontal(|ui| {
                ui.label("Pattern Direction:");
                let dirs = [
                    (FlowDirection::Forward, "Forward [->]"),
                    (FlowDirection::Reverse, "Reverse [<-]"),
                    (FlowDirection::Inward, "Inward [>|<]"),
                    (FlowDirection::Outward, "Outward [<|>]"),
                ];

                for (dir, label) in dirs {
                    if ui
                        .selectable_label(self.flow_direction == dir, label)
                        .clicked()
                    {
                        self.flow_direction = dir;
                    }
                }

                ui.add_space(16.0);
                ui.label(format!("Speed: {}/10", self.ui_speed));
                if ui
                    .add(egui::Slider::new(&mut self.ui_speed, 1..=10))
                    .changed()
                {
                    self.send(GuiCommand::SendCommand(Command::SetSpeed(self.ui_speed)));
                }
            });

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("Built-In Dynamic Flow Patterns")
                    .strong()
                    .size(12.0)
                    .color(Color32::from_rgb(180, 195, 215)),
            );
            ui.separator();

            let effects = [
                (
                    0x01,
                    "Rainbow Spectrum Wave",
                    "Full-spectrum continuous RGB wave",
                ),
                (
                    0x02,
                    "Meteor Trail Stream",
                    "Shooting comet with fading tail",
                ),
                (0x03, "Seven Colour Flow", "Smooth 7-colour transition flow"),
                (
                    0x07,
                    "Stacking Chasing Dots",
                    "Segmented chasing dot accumulation",
                ),
                (0x12, "Fire Heat Waves", "Organic flame temperature flicker"),
                (
                    0x19,
                    "Dual Gradual Snake",
                    "Twin running snakes across strip",
                ),
                (
                    0x2E,
                    "Wave Ripple Pulse",
                    "Harmonic sinusoidal light ripple",
                ),
                (
                    0x3A,
                    "Vortex & Comet Spin",
                    "Accelerating rotary vortex flow",
                ),
                (0x45, "Starfield Sparkle", "Randomised starlight twinkling"),
                (
                    0x5E,
                    "Atmospheric Breath",
                    "Slow rhythmic ambient breathing",
                ),
                (
                    0x81,
                    "Strobe Flash Pulse",
                    "High-intensity rhythmic stroboscope",
                ),
                (
                    0x95,
                    "Bouncing Light Ball",
                    "Kinetic bouncing pulse emulation",
                ),
            ];

            ui.columns(2, |cols| {
                for (idx, (code, title, desc)) in effects.iter().enumerate() {
                    let col_idx = idx % 2;
                    let is_active = self.ui_effect == *code;

                    let bg_color = if is_active {
                        Color32::from_rgba_premultiplied(0, 140, 220, 60)
                    } else {
                        Color32::from_rgba_premultiplied(22, 28, 42, 190)
                    };
                    let border_color = if is_active {
                        Color32::from_rgb(0, 215, 255)
                    } else {
                        Color32::from_rgba_premultiplied(65, 85, 125, 75)
                    };

                    let (rect, response) = cols[col_idx].allocate_exact_size(
                        Vec2::new(cols[col_idx].available_width(), 46.0),
                        egui::Sense::click(),
                    );
                    let painter = cols[col_idx].painter_at(rect);

                    painter.rect_filled(rect, Rounding::same(10.0), bg_color);
                    painter.rect_stroke(
                        rect,
                        Rounding::same(10.0),
                        Stroke::new(1.0_f32, border_color),
                    );

                    let code_str = format!("[0x{:02X}]", code);
                    painter.text(
                        Pos2::new(rect.min.x + 10.0, rect.min.y + 7.0),
                        egui::Align2::LEFT_TOP,
                        code_str,
                        egui::FontId::monospace(11.0),
                        Color32::from_rgb(0, 215, 255),
                    );
                    painter.text(
                        Pos2::new(rect.min.x + 64.0, rect.min.y + 7.0),
                        egui::Align2::LEFT_TOP,
                        *title,
                        egui::FontId::proportional(12.0),
                        Color32::WHITE,
                    );
                    painter.text(
                        Pos2::new(rect.min.x + 10.0, rect.min.y + 25.0),
                        egui::Align2::LEFT_TOP,
                        *desc,
                        egui::FontId::proportional(10.0),
                        Color32::from_rgb(140, 155, 175),
                    );

                    if response.clicked() {
                        self.ui_effect = *code;
                        let cmd = Command::SetEffect(*code);
                        self.log(
                            &format!("Selected dynamic effect: {}", title),
                            &hex_string(&cmd.encode()),
                        );
                        self.send(GuiCommand::SendCommand(cmd));
                    }
                }
            });
        });
    }

    /// Renders the Music & Sound screen with Windows Now Playing Media detection.
    fn render_tab_music(&mut self, ui: &mut egui::Ui) {
        render_glass_card(ui, 460.0, |ui| {
            ui.label(
                egui::RichText::new("Windows Audio Session & Sound-Reactive Engine")
                    .strong()
                    .size(13.0)
                    .color(Color32::from_rgb(215, 230, 250)),
            );
            ui.separator();

            // Native Windows Now Playing Media Glass Card
            let (np_rect, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 76.0), egui::Sense::hover());
            let painter = ui.painter_at(np_rect);

            painter.rect_filled(
                np_rect,
                Rounding::same(12.0),
                Color32::from_rgba_premultiplied(18, 26, 42, 220),
            );
            painter.line_segment(
                [
                    Pos2::new(np_rect.min.x + 12.0, np_rect.min.y + 1.0),
                    Pos2::new(np_rect.max.x - 12.0, np_rect.min.y + 1.0),
                ],
                Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 40)),
            );
            painter.rect_stroke(
                np_rect,
                Rounding::same(12.0),
                Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0, 200, 255, 90)),
            );

            if !self.now_playing.title.is_empty() {
                let status_lbl = if self.now_playing.is_playing {
                    "PLAYING"
                } else {
                    "PAUSED"
                };
                let status_col = if self.now_playing.is_playing {
                    Color32::from_rgb(52, 211, 153)
                } else {
                    Color32::from_rgb(251, 146, 60)
                };

                painter.text(
                    Pos2::new(np_rect.min.x + 14.0, np_rect.min.y + 10.0),
                    egui::Align2::LEFT_TOP,
                    format!("NOW PLAYING: {}", self.now_playing.title),
                    egui::FontId::proportional(14.0),
                    Color32::WHITE,
                );
                painter.text(
                    Pos2::new(np_rect.min.x + 14.0, np_rect.min.y + 32.0),
                    egui::Align2::LEFT_TOP,
                    format!(
                        "Artist: {} | Source: {}",
                        self.now_playing.artist, self.now_playing.app_name
                    ),
                    egui::FontId::proportional(11.0),
                    Color32::from_rgb(175, 190, 215),
                );
                painter.text(
                    Pos2::new(np_rect.max.x - 14.0, np_rect.min.y + 10.0),
                    egui::Align2::RIGHT_TOP,
                    status_lbl,
                    egui::FontId::proportional(11.0),
                    status_col,
                );

                // Audio peak meter level readout
                let meter_pct = (self.live_audio_peak * 100.0) as u32;
                painter.text(
                    Pos2::new(np_rect.max.x - 14.0, np_rect.min.y + 32.0),
                    egui::Align2::RIGHT_TOP,
                    format!("Peak Volume: {}%", meter_pct),
                    egui::FontId::proportional(11.0),
                    Color32::from_rgb(0, 215, 255),
                );

                // Mini peak level progress bar
                let meter_bar_rect = Rect::from_min_size(
                    Pos2::new(np_rect.min.x + 14.0, np_rect.max.y - 14.0),
                    Vec2::new((np_rect.width() - 28.0) * self.live_audio_peak, 4.0),
                );
                painter.rect_filled(
                    meter_bar_rect,
                    Rounding::same(2.0),
                    Color32::from_rgb(0, 215, 255),
                );
            } else {
                painter.text(
                    np_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Listening for Windows Media Playback (Spotify, YouTube, Media Player)...",
                    egui::FontId::proportional(12.0),
                    Color32::from_rgb(140, 155, 180),
                );
            }

            ui.add_space(10.0);

            // Audio Routing and Sensitivity
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.sync_with_windows_audio,
                    "Sync LED Pulse to Windows Media Audio",
                );
                ui.add_space(16.0);
                ui.label("Audio Source:");
                egui::ComboBox::from_id_source("audio_input_cb_music")
                    .selected_text(match self.ui_audio_input {
                        AudioInput::InternalMic => "Controller Hardware Mic",
                        AudioInput::Player => "Host Software Audio",
                        AudioInput::ExternalMic => "External 3.5mm Aux",
                    })
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut self.ui_audio_input,
                                AudioInput::InternalMic,
                                "Controller Hardware Mic",
                            )
                            .clicked()
                        {
                            self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                                AudioInput::InternalMic,
                            )));
                        }
                        if ui
                            .selectable_value(
                                &mut self.ui_audio_input,
                                AudioInput::Player,
                                "Host Software Audio",
                            )
                            .clicked()
                        {
                            self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                                AudioInput::Player,
                            )));
                        }
                        if ui
                            .selectable_value(
                                &mut self.ui_audio_input,
                                AudioInput::ExternalMic,
                                "External 3.5mm Aux",
                            )
                            .clicked()
                        {
                            self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                                AudioInput::ExternalMic,
                            )));
                        }
                    });

                ui.add_space(16.0);
                ui.label(format!("Audio Sensitivity: {}/16", self.ui_sensitivity));
                if ui
                    .add(egui::Slider::new(&mut self.ui_sensitivity, 1..=16))
                    .changed()
                {
                    self.send(GuiCommand::SendCommand(Command::SetSensitivity(
                        self.ui_sensitivity,
                    )));
                }
            });

            ui.add_space(8.0);

            // Animated Equalizer Visualizer Bars
            self.render_equalizer_bars(ui);

            ui.add_space(12.0);
            ui.label(
                egui::RichText::new("Sound-Reactive Rhythm Patterns")
                    .strong()
                    .size(12.0)
                    .color(Color32::from_rgb(180, 195, 215)),
            );
            ui.separator();

            let rhythm_effects = [
                (
                    0xC9,
                    "Music Spectrum Pulse",
                    "Equalizer bar mapping directly to strip length",
                ),
                (
                    0xCA,
                    "Music Energy Blink",
                    "Transient threshold bass beat flasher",
                ),
                (
                    0xCB,
                    "Music VuMeter Flow",
                    "Classic logarithmic volume meter cascade",
                ),
                (
                    0xCC,
                    "Music Peak Rhythm Hits",
                    "High-frequency peak transient impact flashes",
                ),
                (
                    0xCD,
                    "Music Force Ejection",
                    "Sound energy bursting outward from center",
                ),
                (
                    0xCE,
                    "Music Flowing Water Wave",
                    "Harmonic liquid ripples timed to audio beats",
                ),
            ];

            ui.columns(2, |cols| {
                for (idx, (code, title, desc)) in rhythm_effects.iter().enumerate() {
                    let col_idx = idx % 2;
                    let is_active = self.ui_effect == *code;

                    let bg_color = if is_active {
                        Color32::from_rgba_premultiplied(0, 140, 220, 60)
                    } else {
                        Color32::from_rgba_premultiplied(22, 28, 42, 190)
                    };
                    let border_color = if is_active {
                        Color32::from_rgb(0, 215, 255)
                    } else {
                        Color32::from_rgba_premultiplied(65, 85, 125, 75)
                    };

                    let (rect, response) = cols[col_idx].allocate_exact_size(
                        Vec2::new(cols[col_idx].available_width(), 46.0),
                        egui::Sense::click(),
                    );
                    let painter = cols[col_idx].painter_at(rect);

                    painter.rect_filled(rect, Rounding::same(10.0), bg_color);
                    painter.rect_stroke(
                        rect,
                        Rounding::same(10.0),
                        Stroke::new(1.0_f32, border_color),
                    );

                    let code_str = format!("[0x{:02X}]", code);
                    painter.text(
                        Pos2::new(rect.min.x + 10.0, rect.min.y + 7.0),
                        egui::Align2::LEFT_TOP,
                        code_str,
                        egui::FontId::monospace(11.0),
                        Color32::from_rgb(255, 160, 0),
                    );
                    painter.text(
                        Pos2::new(rect.min.x + 64.0, rect.min.y + 7.0),
                        egui::Align2::LEFT_TOP,
                        *title,
                        egui::FontId::proportional(12.0),
                        Color32::WHITE,
                    );
                    painter.text(
                        Pos2::new(rect.min.x + 10.0, rect.min.y + 25.0),
                        egui::Align2::LEFT_TOP,
                        *desc,
                        egui::FontId::proportional(10.0),
                        Color32::from_rgb(140, 155, 175),
                    );

                    if response.clicked() {
                        self.ui_effect = *code;
                        let cmd = Command::SetEffect(*code);
                        self.log(
                            &format!("Selected rhythm effect: {}", title),
                            &hex_string(&cmd.encode()),
                        );
                        self.send(GuiCommand::SendCommand(cmd));
                    }
                }
            });

            ui.add_space(8.0);
            if ui.button("Activate Auto-Sound Cycle Mode").clicked() {
                self.send(GuiCommand::SendCommand(Command::SetLightMode(
                    LightMode::AutoSound,
                )));
                self.log("Activated operating mode: Auto Sound Reactive", "");
            }
        });
    }

    /// Renders curated ambient scenes and mood presets.
    fn render_tab_custom_scenes(&mut self, ui: &mut egui::Ui) {
        render_glass_card(ui, 380.0, |ui| {
            ui.label(
                egui::RichText::new("Curated Ambient Scenes & Custom Palettes")
                    .strong()
                    .size(13.0)
                    .color(Color32::from_rgb(215, 230, 250)),
            );
            ui.separator();

            let scenes = [
                (
                    "Cozy Hearth",
                    [1.0, 0.45, 0.08],
                    0x12,
                    4,
                    210,
                    "Warm fire flicker with deep amber embers",
                ),
                (
                    "Deep Sea Bioluminescence",
                    [0.0, 0.4, 1.0],
                    0x2E,
                    5,
                    190,
                    "Oceanic cyan and azure pulsing wave",
                ),
                (
                    "Cyberpunk Neon",
                    [1.0, 0.0, 0.6],
                    0x01,
                    8,
                    255,
                    "High-contrast magenta and electric blue sweep",
                ),
                (
                    "Zen Forest Breath",
                    [0.1, 0.88, 0.4],
                    0x5E,
                    3,
                    150,
                    "Calming mint and emerald breath rhythm",
                ),
                (
                    "Solar Sunset",
                    [1.0, 0.25, 0.1],
                    0x03,
                    5,
                    230,
                    "Gradient blend of coral, peach, and gold",
                ),
                (
                    "Arctic Aurora",
                    [0.0, 0.85, 0.85],
                    0x02,
                    6,
                    200,
                    "Glacial teal comet flowing across strip",
                ),
            ];

            ui.columns(2, |cols| {
                for (idx, (title, rgb_arr, eff, spd, br, desc)) in scenes.iter().enumerate() {
                    let col_idx = idx % 2;
                    let (rect, response) = cols[col_idx].allocate_exact_size(
                        Vec2::new(cols[col_idx].available_width(), 54.0),
                        egui::Sense::click(),
                    );
                    let painter = cols[col_idx].painter_at(rect);

                    painter.rect_filled(
                        rect,
                        Rounding::same(10.0),
                        Color32::from_rgba_premultiplied(22, 28, 42, 190),
                    );
                    painter.rect_stroke(
                        rect,
                        Rounding::same(10.0),
                        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(65, 85, 125, 75)),
                    );

                    let r = (rgb_arr[0] * 255.0) as u8;
                    let g = (rgb_arr[1] * 255.0) as u8;
                    let b = (rgb_arr[2] * 255.0) as u8;
                    painter.circle_filled(
                        Pos2::new(rect.min.x + 18.0, rect.min.y + 20.0),
                        9.0,
                        Color32::from_rgb(r, g, b),
                    );

                    painter.text(
                        Pos2::new(rect.min.x + 36.0, rect.min.y + 10.0),
                        egui::Align2::LEFT_TOP,
                        *title,
                        egui::FontId::proportional(13.0),
                        Color32::WHITE,
                    );
                    painter.text(
                        Pos2::new(rect.min.x + 36.0, rect.min.y + 29.0),
                        egui::Align2::LEFT_TOP,
                        *desc,
                        egui::FontId::proportional(10.0),
                        Color32::from_rgb(140, 155, 175),
                    );

                    if response.clicked() {
                        self.ui_color = *rgb_arr;
                        self.ui_effect = *eff;
                        self.ui_speed = *spd;
                        self.ui_brightness = *br;

                        self.send(GuiCommand::SendCommand(Command::SetEffect(*eff)));
                        self.send(GuiCommand::SendCommand(Command::SetSpeed(*spd)));
                        self.send(GuiCommand::SendCommand(Command::SetColor {
                            color: RgbColor::new(r, g, b),
                            level: *br,
                        }));
                        self.log(&format!("Applied scene preset '{}'", title), "");
                    }
                }
            });
        });
    }

    /// Renders the Hardware Configuration screen with EEPROM safety gate.
    fn render_tab_hardware(&mut self, ui: &mut egui::Ui) {
        render_glass_card(ui, 320.0, |ui| {
            ui.label(
                egui::RichText::new("Physical Controller Hardware & EEPROM Configuration")
                    .strong()
                    .size(13.0)
                    .color(Color32::from_rgb(215, 230, 250)),
            );
            ui.separator();

            // Safety Warning Banner
            let (warn_rect, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), egui::Sense::hover());
            let painter = ui.painter_at(warn_rect);
            painter.rect_filled(
                warn_rect,
                Rounding::same(8.0),
                Color32::from_rgba_premultiplied(65, 45, 15, 180),
            );
            painter.rect_stroke(
                warn_rect,
                Rounding::same(8.0),
                Stroke::new(1.0_f32, Color32::from_rgb(251, 146, 60)),
            );
            painter.text(
                Pos2::new(warn_rect.min.x + 10.0, warn_rect.min.y + 8.0),
                egui::Align2::LEFT_TOP,
                "CAUTION: Hardware Safety Gate",
                egui::FontId::proportional(12.0),
                Color32::from_rgb(251, 146, 60),
            );
            painter.text(
                Pos2::new(warn_rect.min.x + 10.0, warn_rect.min.y + 26.0),
                egui::Align2::LEFT_TOP,
                "Writing incorrect parameters directly modifies non-volatile EEPROM registers. Verify strip settings.",
                egui::FontId::proportional(10.0),
                Color32::from_rgb(230, 235, 245),
            );

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(format!(
                    "Pixel Count / Strip Length: {} pixels",
                    self.ui_length
                ));
                ui.add(egui::Slider::new(&mut self.ui_length, 1..=150));
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("RGB Channel Sequence / Chip Order:");
                egui::ComboBox::from_id_source("chip_order_cb_hw")
                    .selected_text(format!("{:?}", self.ui_chip_order))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.ui_chip_order,
                            ChipOrder::Grb,
                            "GRB (WS2812B standard)",
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
            ui.checkbox(
                &mut self.ui_confirm_safety,
                "I understand this modifies non-volatile controller registers (--i-understand-this-changes-the-device)",
            );

            ui.add_space(8.0);
            let btn_enabled = self.ui_confirm_safety;
            let apply_btn = egui::Button::new(
                egui::RichText::new("Apply Hardware Configuration")
                    .strong()
                    .color(if btn_enabled {
                        Color32::WHITE
                    } else {
                        Color32::GRAY
                    }),
            )
            .fill(if btn_enabled {
                Color32::from_rgb(16, 185, 129)
            } else {
                Color32::from_rgba_premultiplied(35, 42, 58, 160)
            })
            .rounding(Rounding::same(8.0));

            if ui.add_enabled(btn_enabled, apply_btn).clicked() {
                self.send(GuiCommand::SendCommand(Command::SetLength(self.ui_length)));
                self.send(GuiCommand::SendCommand(Command::SetChipOrder(
                    self.ui_chip_order,
                )));
                self.log(
                    &format!(
                        "Reconfigured hardware: length={}, chip_order={:?}",
                        self.ui_length, self.ui_chip_order
                    ),
                    "",
                );
            }
        });
    }

    /// Renders the real-time protocol trace log.
    fn render_tab_diagnostics(&mut self, ui: &mut egui::Ui) {
        render_glass_card(ui, 320.0, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Live Protocol Hex Command Trace")
                        .strong()
                        .size(13.0)
                        .color(Color32::from_rgb(215, 230, 250)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Clear Trace Log").clicked() {
                        self.logs.clear();
                    }
                });
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(250.0)
                .show(ui, |ui| {
                    for entry in self.logs.iter().rev() {
                        ui.horizontal(|ui| {
                            ui.monospace(&entry.timestamp);
                            ui.label(&entry.message);
                            if !entry.hex.is_empty() {
                                ui.colored_label(
                                    Color32::from_rgb(52, 211, 153),
                                    format!("HEX: [{}]", entry.hex),
                                );
                            }
                        });
                    }
                });
        });
    }

    /// Draws the bouncing 16-band graphic equalizer visualizer.
    fn render_equalizer_bars(&self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 52.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        painter.rect_filled(
            rect,
            Rounding::same(8.0),
            Color32::from_rgba_premultiplied(14, 18, 28, 200),
        );
        painter.rect_stroke(
            rect,
            Rounding::same(8.0),
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(55, 75, 110, 75)),
        );

        let bars = 16;
        let spacing = rect.width() / (bars as f32);
        let bar_width = spacing * 0.72;

        for i in 0..bars {
            let freq = i as f32 * 1.35;
            let height_factor = if self.ui_power {
                if self.sync_with_windows_audio && self.live_audio_peak > 0.01 {
                    // Modulate to live Windows audio peak meter + harmonic frequency
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
                    rect.min.x + spacing * (i as f32 + 0.14),
                    rect.max.y - bar_h - 2.0,
                ),
                Vec2::new(bar_width, bar_h),
            );

            let col = if i < 6 {
                Color32::from_rgb(0, 215, 255) // Electric Cyan
            } else if i < 11 {
                Color32::from_rgb(255, 180, 0) // Amber
            } else {
                Color32::from_rgb(255, 55, 115) // Hot pink
            };

            painter.rect_filled(bar_rect, Rounding::same(3.0), col);
        }
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

        // If synced with Windows audio and audio is playing, modulate brightness dynamically
        if self.sync_with_windows_audio && self.live_audio_peak > 0.01 {
            factor = (factor * (0.4 + 0.6 * self.live_audio_peak)).clamp(0.1, 1.0);
        }

        match self.ui_effect {
            EFFECT_SOLID => {
                let r = (self.ui_color[0] * 255.0 * factor) as u8;
                let g = (self.ui_color[1] * 255.0 * factor) as u8;
                let b = (self.ui_color[2] * 255.0 * factor) as u8;
                self.strip_pixels.resize(num_pixels, RgbColor::new(r, g, b));
            }
            0x01 => {
                // Rainbow Spectrum Wave
                for i in 0..num_pixels {
                    let idx = match self.flow_direction {
                        FlowDirection::Reverse => num_pixels.saturating_sub(1 + i),
                        _ => i,
                    };
                    let hue = (self.animation_timer * 15.0
                        + (idx as f32 * 360.0 / num_pixels as f32))
                        % 360.0;
                    let (r, g, b) = hsv_to_rgb(hue, 1.0, factor);
                    self.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            0x02 => {
                // Meteor Trail Stream
                let head = ((self.animation_timer * 12.0) as usize) % num_pixels.max(1);
                for i in 0..num_pixels {
                    let pos = match self.flow_direction {
                        FlowDirection::Reverse => num_pixels.saturating_sub(1 + i),
                        _ => i,
                    };
                    let dist = (num_pixels + head - pos) % num_pixels;
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
                    let flicker = ((self.animation_timer * 20.0 + (i as f32 * 1.7)).sin() * 0.25
                        + 0.75)
                        * factor;
                    let r = (255.0 * flicker) as u8;
                    let g = (80.0 * flicker) as u8;
                    let b = (10.0 * flicker) as u8;
                    self.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            0xC9..=0xCC => {
                // Sound Reactive Rhythm Modes
                for i in 0..num_pixels {
                    let beat_intensity =
                        if self.sync_with_windows_audio && self.live_audio_peak > 0.01 {
                            self.live_audio_peak
                        } else {
                            (self.animation_timer * 10.0).sin().abs()
                        };
                    let beat = (beat_intensity * factor).clamp(0.05, 1.0);
                    let (r, g, b) = if self.ui_effect == 0xCB {
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
                        (
                            (self.ui_color[0] * 255.0 * beat) as u8,
                            (self.ui_color[1] * 255.0 * beat) as u8,
                            (self.ui_color[2] * 255.0 * beat) as u8,
                        )
                    };
                    self.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            _ => {
                // Animated sinusoidal wave ripple
                for i in 0..num_pixels {
                    let wave = (((self.animation_timer * 6.0 + i as f32 * 0.5).sin() + 1.0) * 0.5)
                        * factor;
                    let r = (self.ui_color[0] * 255.0 * wave) as u8;
                    let g = (self.ui_color[1] * 255.0 * wave) as u8;
                    let b = (self.ui_color[2] * 255.0 * wave) as u8;
                    self.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
        }
    }
}

/// Helper that renders a frosted glass card container with specular bevel highlights.
fn render_glass_card<R>(
    ui: &mut egui::Ui,
    min_height: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let frame = egui::Frame::none()
        .fill(Color32::from_rgba_premultiplied(18, 24, 38, 215))
        .rounding(Rounding::same(14.0))
        .stroke(Stroke::new(
            1.0_f32,
            Color32::from_rgba_premultiplied(70, 90, 130, 75),
        ))
        .inner_margin(egui::Margin::same(14.0));

    let response = frame.show(ui, |ui| {
        ui.set_min_height(min_height);
        add_contents(ui)
    });

    // Draw specular highlight along top edge of glass card
    let rect = response.response.rect;
    let painter = ui.painter();
    painter.line_segment(
        [
            Pos2::new(rect.min.x + 14.0, rect.min.y + 1.0),
            Pos2::new(rect.max.x - 14.0, rect.min.y + 1.0),
        ],
        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 38)),
    );

    response.inner
}

/// Draws an interactive continuous radial Hue & Saturation color disc.
fn render_color_disc(ui: &mut egui::Ui, current_color: &mut [f32; 3], radius: f32) -> bool {
    let size = Vec2::splat(radius * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    let center = rect.center();

    // Draw radial hue-saturation disc using triangle mesh
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

    // Outer border ring with glass bevel
    painter.circle_stroke(
        center,
        radius,
        Stroke::new(1.5_f32, Color32::from_rgba_premultiplied(75, 95, 140, 100)),
    );

    // Calculate indicator position from current RGB
    let (curr_h, curr_s, _) = rgb_to_hsv(current_color[0], current_color[1], current_color[2]);
    let curr_angle = curr_h.to_radians();
    let indicator_pos = center + Vec2::new(curr_angle.cos(), curr_angle.sin()) * (curr_s * radius);

    // Draw crosshair thumb ring
    painter.circle_stroke(indicator_pos, 8.5, Stroke::new(2.5_f32, Color32::WHITE));
    painter.circle_stroke(indicator_pos, 10.0, Stroke::new(1.0_f32, Color32::BLACK));

    // Handle user drag or click on the disc
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
            .with_inner_size([800.0, 660.0])
            .with_min_inner_size([580.0, 500.0])
            .with_title("Pyxillum - Pixel LED Studio")
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
