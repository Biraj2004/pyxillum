#![warn(missing_docs)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Pyxillum Desktop Application
//!
//! Native cross-platform GUI for controlling Bluetooth Low Energy pixel LED controllers.
//! Presentation-only layer; all protocol logic is isolated in `pyxillum-protocol`
//! and BLE communication is decoupled on a background Tokio runtime.

use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_core::{DeviceManager, DeviceState};
use pyxillum_protocol::{AudioInput, ChipOrder, Command, LightMode, RgbColor, EFFECT_SOLID};
use pyxillum_sim::DeviceSimulator;
use pyxillum_transport::{DiscoveredDevice, SimTransport};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex};
use tracing::info;

mod icon_data;

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

    // UI Local State
    is_demo_mode: bool,
    _discovered: Vec<DiscoveredDevice>,
    _selected_device: Option<String>,
    logs: Vec<LogEntry>,
    active_tab: ActiveTab,
    flow_direction: FlowDirection,

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
        // Configure dark theme inspired by smart lighting companion apps
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(Color32::from_rgb(240, 244, 248));
        visuals.panel_fill = Color32::from_rgb(16, 19, 27);
        visuals.window_fill = Color32::from_rgb(22, 27, 38);
        visuals.extreme_bg_color = Color32::from_rgb(11, 14, 20);
        visuals.faint_bg_color = Color32::from_rgb(26, 32, 46);
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(22, 28, 40);
        visuals.widgets.noninteractive.bg_stroke =
            Stroke::new(1.0_f32, Color32::from_rgb(38, 48, 68));
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(26, 34, 50);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(44, 56, 80));
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(38, 50, 75);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.5_f32, Color32::from_rgb(0, 210, 255));
        visuals.widgets.active.bg_fill = Color32::from_rgb(0, 160, 220);
        visuals.selection.bg_fill = Color32::from_rgb(0, 180, 230);
        visuals.widgets.inactive.rounding = Rounding::same(6.0);
        visuals.widgets.hovered.rounding = Rounding::same(6.0);
        visuals.widgets.active.rounding = Rounding::same(6.0);
        visuals.window_rounding = Rounding::same(10.0);
        cc.egui_ctx.set_visuals(visuals);

        Self {
            cmd_tx,
            _state_reader: state_reader,
            _simulator_ref: simulator_ref,
            is_demo_mode: true,
            _discovered: Vec::new(),
            _selected_device: None,
            logs: vec![LogEntry {
                timestamp: "00:00:00".to_string(),
                message: "Pyxillum initialized in offline-first mode.".to_string(),
                hex: String::new(),
            }],
            active_tab: ActiveTab::Illumination,
            flow_direction: FlowDirection::Forward,
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

        // Top Navigation Bar
        egui::TopBottomPanel::top("header_panel")
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(14, 17, 24))
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // App Logo and Name
                    ui.label(
                        egui::RichText::new("PYXILLUM")
                            .strong()
                            .size(17.0)
                            .color(Color32::from_rgb(0, 210, 255)),
                    );
                    ui.label(
                        egui::RichText::new("Pixel LED Controller")
                            .size(12.0)
                            .color(Color32::from_rgb(140, 155, 175)),
                    );

                    ui.add_space(12.0);

                    // Connection status pill
                    let (status_text, status_color) = if self.is_demo_mode {
                        (
                            "Simulated Controller [SIM:00:11:22:33:44:55]",
                            Color32::from_rgb(52, 211, 153),
                        )
                    } else {
                        (
                            "Hardware Controller [Disconnected]",
                            Color32::from_rgb(251, 146, 60),
                        )
                    };

                    ui.group(|ui| {
                        ui.label(
                            egui::RichText::new(status_text)
                                .size(11.0)
                                .color(status_color),
                        );
                    });

                    // Right actions: Scan and Power Toggle
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Large Power Toggle Button
                        let pwr_text = if self.ui_power {
                            "POWER ON"
                        } else {
                            "POWER OFF"
                        };
                        let pwr_bg = if self.ui_power {
                            Color32::from_rgb(16, 185, 129)
                        } else {
                            Color32::from_rgb(45, 55, 75)
                        };
                        let pwr_btn = egui::Button::new(
                            egui::RichText::new(pwr_text).strong().color(Color32::WHITE),
                        )
                        .fill(pwr_bg)
                        .rounding(Rounding::same(8.0))
                        .min_size(Vec2::new(100.0, 28.0));

                        if ui.add(pwr_btn).clicked() {
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
                                    .color(Color32::from_rgb(200, 215, 235)),
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

        // Main Content Area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(4.0);

                // Section 1: Illuminated Virtual LED Neon Strip Monitor
                self.render_virtual_strip_container(ui);

                ui.add_space(8.0);

                // Section 2: Segmented Mode Navigation Bar
                self.render_navigation_bar(ui);

                ui.add_space(10.0);

                // Section 3: Active Mode Content
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

        // Continuous 60 FPS repaints for LED visualizer
        ctx.request_repaint();
    }
}

impl PyxillumApp {
    /// Renders the top addressable LED neon strip simulation.
    fn render_virtual_strip_container(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Virtual LED Strip Monitor")
                        .strong()
                        .size(12.0)
                        .color(Color32::from_rgb(180, 195, 215)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let pwr_lbl = if self.ui_power { "ACTIVE" } else { "STANDBY" };
                    let pwr_col = if self.ui_power {
                        Color32::from_rgb(52, 211, 153)
                    } else {
                        Color32::from_rgb(156, 163, 175)
                    };
                    ui.label(
                        egui::RichText::new(format!(
                            "{} Pixels | Power: {} | 60 FPS",
                            self.ui_length, pwr_lbl
                        ))
                        .size(11.0)
                        .color(pwr_col),
                    );
                });
            });
            ui.add_space(4.0);
            self.render_virtual_strip(ui);
        });
    }

    /// Renders the modern navigation segmented buttons.
    fn render_navigation_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let tabs = [
                (ActiveTab::Illumination, "Illumination"),
                (ActiveTab::DynamicEffects, "Dynamic Scenes"),
                (ActiveTab::MusicReactive, "Music & Sound"),
                (ActiveTab::CustomScenes, "DIY Scenes"),
                (ActiveTab::HardwareSetup, "Hardware Config"),
                (ActiveTab::Diagnostics, "Diagnostics"),
            ];

            for (tab, label) in tabs {
                let is_selected = self.active_tab == tab;
                let bg_color = if is_selected {
                    Color32::from_rgb(0, 140, 200)
                } else {
                    Color32::from_rgb(24, 30, 44)
                };
                let text_color = if is_selected {
                    Color32::WHITE
                } else {
                    Color32::from_rgb(160, 175, 195)
                };

                let btn = egui::Button::new(egui::RichText::new(label).strong().color(text_color))
                    .fill(bg_color)
                    .rounding(Rounding::same(6.0))
                    .min_size(Vec2::new(105.0, 30.0));

                if ui.add(btn).clicked() {
                    self.active_tab = tab;
                }
            }
        });
        ui.separator();
    }

    /// Renders the Illumination screen featuring the interactive Hue-Saturation wheel.
    fn render_tab_illumination(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            // Left Column: Interactive Radial Hue-Saturation Disc
            columns[0].group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Radial Hue & Saturation Disc")
                            .strong()
                            .size(13.0)
                            .color(Color32::from_rgb(200, 215, 235)),
                    );
                    ui.label(
                        egui::RichText::new("Drag on the disc to pick chromatic colour")
                            .size(11.0)
                            .color(Color32::from_rgb(130, 145, 165)),
                    );

                    ui.add_space(8.0);

                    // Circular Hue-Saturation Wheel
                    if render_color_disc(ui, &mut self.ui_color, 90.0) {
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

                    // Active Colour Preview Swatch
                    let r = (self.ui_color[0] * 255.0) as u8;
                    let g = (self.ui_color[1] * 255.0) as u8;
                    let b = (self.ui_color[2] * 255.0) as u8;
                    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

                    let (preview_rect, _) =
                        ui.allocate_exact_size(Vec2::new(200.0, 32.0), egui::Sense::hover());
                    let painter = ui.painter_at(preview_rect);
                    painter.rect_filled(
                        preview_rect,
                        Rounding::same(6.0),
                        Color32::from_rgb(r, g, b),
                    );
                    painter.rect_stroke(
                        preview_rect,
                        Rounding::same(6.0),
                        Stroke::new(1.0_f32, Color32::from_rgb(255, 255, 255)),
                    );

                    // Text contrast
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

            // Right Column: Master Brightness and Quick Palette Swatches
            columns[1].group(|ui| {
                ui.label(
                    egui::RichText::new("Master Brightness & Quick Swatches")
                        .strong()
                        .size(13.0)
                        .color(Color32::from_rgb(200, 215, 235)),
                );
                ui.separator();

                ui.add_space(4.0);
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

                // Quick brightness steps
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
                            ui.allocate_exact_size(Vec2::new(32.0, 32.0), egui::Sense::click());
                        let painter = ui.painter_at(rect);
                        painter.circle_filled(rect.center(), 14.0, c32);

                        // Highlight ring if active
                        let is_active = (self.ui_color[0] - rgb_arr[0]).abs() < 0.05
                            && (self.ui_color[1] - rgb_arr[1]).abs() < 0.05
                            && (self.ui_color[2] - rgb_arr[2]).abs() < 0.05;

                        if is_active {
                            painter.circle_stroke(
                                rect.center(),
                                15.5,
                                Stroke::new(2.0_f32, Color32::WHITE),
                            );
                        } else {
                            painter.circle_stroke(
                                rect.center(),
                                14.0,
                                Stroke::new(1.0_f32, Color32::from_rgb(40, 50, 70)),
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

    /// Renders the Dynamic Effects screen with pattern cards and flow controls.
    fn render_tab_effects(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Flow Dynamics & Direction Controls")
                        .strong()
                        .size(13.0)
                        .color(Color32::from_rgb(200, 215, 235)),
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

            // Direction buttons and Speed Slider
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
                egui::RichText::new("Built-In Dynamic Pattern Library")
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

            // Render effect cards in 2 columns
            ui.columns(2, |cols| {
                for (idx, (code, title, desc)) in effects.iter().enumerate() {
                    let col_idx = idx % 2;
                    let is_active = self.ui_effect == *code;

                    let bg_color = if is_active {
                        Color32::from_rgb(26, 48, 75)
                    } else {
                        Color32::from_rgb(20, 25, 36)
                    };
                    let border_color = if is_active {
                        Color32::from_rgb(0, 210, 255)
                    } else {
                        Color32::from_rgb(36, 44, 62)
                    };

                    cols[col_idx].group(|ui| {
                        let (rect, response) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 44.0),
                            egui::Sense::click(),
                        );
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, Rounding::same(6.0), bg_color);
                        painter.rect_stroke(
                            rect,
                            Rounding::same(6.0),
                            Stroke::new(1.0_f32, border_color),
                        );

                        // Card Content
                        let code_str = format!("[0x{:02X}]", code);
                        painter.text(
                            Pos2::new(rect.min.x + 8.0, rect.min.y + 6.0),
                            egui::Align2::LEFT_TOP,
                            code_str,
                            egui::FontId::monospace(11.0),
                            Color32::from_rgb(0, 210, 255),
                        );
                        painter.text(
                            Pos2::new(rect.min.x + 60.0, rect.min.y + 6.0),
                            egui::Align2::LEFT_TOP,
                            *title,
                            egui::FontId::proportional(12.0),
                            Color32::WHITE,
                        );
                        painter.text(
                            Pos2::new(rect.min.x + 8.0, rect.min.y + 24.0),
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
                    });
                }
            });
        });
    }

    /// Renders the Music & Sound Reactive screen.
    fn render_tab_music(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(
                egui::RichText::new("Audio Reactive Engine & Equalizer")
                    .strong()
                    .size(13.0)
                    .color(Color32::from_rgb(200, 215, 235)),
            );
            ui.separator();

            // Audio Input Selection and Sensitivity
            ui.horizontal(|ui| {
                ui.label("Audio Source:");
                egui::ComboBox::from_id_source("audio_input_cb_music")
                    .selected_text(match self.ui_audio_input {
                        AudioInput::InternalMic => "Controller Hardware Microphone",
                        AudioInput::Player => "Host Software Audio Stream",
                        AudioInput::ExternalMic => "External 3.5mm Aux Microphone",
                    })
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut self.ui_audio_input,
                                AudioInput::InternalMic,
                                "Controller Hardware Microphone",
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
                                "Host Software Audio Stream",
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
                                "External 3.5mm Aux Microphone",
                            )
                            .clicked()
                        {
                            self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                                AudioInput::ExternalMic,
                            )));
                        }
                    });

                ui.add_space(16.0);
                ui.label(format!("Gain Sensitivity: {}/16", self.ui_sensitivity));
                if ui
                    .add(egui::Slider::new(&mut self.ui_sensitivity, 1..=16))
                    .changed()
                {
                    self.send(GuiCommand::SendCommand(Command::SetSensitivity(
                        self.ui_sensitivity,
                    )));
                }
            });

            ui.add_space(10.0);

            // Animated Graphic Equalizer Visualizer
            ui.group(|ui| {
                ui.label(
                    egui::RichText::new("Live Graphic Equalizer Spectrum")
                        .size(11.0)
                        .color(Color32::from_rgb(140, 155, 175)),
                );
                ui.add_space(4.0);
                self.render_equalizer_bars(ui);
            });

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
                        Color32::from_rgb(26, 48, 75)
                    } else {
                        Color32::from_rgb(20, 25, 36)
                    };
                    let border_color = if is_active {
                        Color32::from_rgb(0, 210, 255)
                    } else {
                        Color32::from_rgb(36, 44, 62)
                    };

                    cols[col_idx].group(|ui| {
                        let (rect, response) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 44.0),
                            egui::Sense::click(),
                        );
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, Rounding::same(6.0), bg_color);
                        painter.rect_stroke(
                            rect,
                            Rounding::same(6.0),
                            Stroke::new(1.0_f32, border_color),
                        );

                        let code_str = format!("[0x{:02X}]", code);
                        painter.text(
                            Pos2::new(rect.min.x + 8.0, rect.min.y + 6.0),
                            egui::Align2::LEFT_TOP,
                            code_str,
                            egui::FontId::monospace(11.0),
                            Color32::from_rgb(255, 160, 0),
                        );
                        painter.text(
                            Pos2::new(rect.min.x + 60.0, rect.min.y + 6.0),
                            egui::Align2::LEFT_TOP,
                            *title,
                            egui::FontId::proportional(12.0),
                            Color32::WHITE,
                        );
                        painter.text(
                            Pos2::new(rect.min.x + 8.0, rect.min.y + 24.0),
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
                    });
                }
            });

            ui.add_space(10.0);
            if ui.button("Activate Auto-Sound Cycle Mode").clicked() {
                self.send(GuiCommand::SendCommand(Command::SetLightMode(
                    LightMode::AutoSound,
                )));
                self.log("Activated operating mode: Auto Sound Reactive", "");
            }
        });
    }

    /// Renders the DIY custom scenes and mood transitions.
    fn render_tab_custom_scenes(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(
                egui::RichText::new("Curated Ambient Scenes & Custom Palettes")
                    .strong()
                    .size(13.0)
                    .color(Color32::from_rgb(200, 215, 235)),
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
                    cols[col_idx].group(|ui| {
                        let (rect, response) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 52.0),
                            egui::Sense::click(),
                        );
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(
                            rect,
                            Rounding::same(6.0),
                            Color32::from_rgb(22, 28, 40),
                        );
                        painter.rect_stroke(
                            rect,
                            Rounding::same(6.0),
                            Stroke::new(1.0_f32, Color32::from_rgb(40, 50, 72)),
                        );

                        // Swatch dot
                        let r = (rgb_arr[0] * 255.0) as u8;
                        let g = (rgb_arr[1] * 255.0) as u8;
                        let b = (rgb_arr[2] * 255.0) as u8;
                        painter.circle_filled(
                            Pos2::new(rect.min.x + 16.0, rect.min.y + 18.0),
                            8.0,
                            Color32::from_rgb(r, g, b),
                        );

                        painter.text(
                            Pos2::new(rect.min.x + 32.0, rect.min.y + 10.0),
                            egui::Align2::LEFT_TOP,
                            *title,
                            egui::FontId::proportional(13.0),
                            Color32::WHITE,
                        );
                        painter.text(
                            Pos2::new(rect.min.x + 32.0, rect.min.y + 28.0),
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
                    });
                }
            });
        });
    }

    /// Renders the Hardware Configuration screen with safety confirmation.
    fn render_tab_hardware(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(
                egui::RichText::new("Physical Controller Hardware & EEPROM Configuration")
                    .strong()
                    .size(13.0)
                    .color(Color32::from_rgb(200, 215, 235)),
            );
            ui.separator();

            // Safety Warning Banner
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("CAUTION: Hardware Safety Gate")
                            .strong()
                            .color(Color32::from_rgb(255, 160, 0)),
                    );
                });
                ui.label(
                    egui::RichText::new(
                        "Writing out-of-range or incorrect parameters directly modifies non-volatile EEPROM registers. Ensure pixel count and channel order match your physical hardware strip before applying.",
                    )
                    .size(11.0)
                    .color(Color32::from_rgb(220, 225, 235)),
                );
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(format!("Pixel Count / Strip Length: {} pixels", self.ui_length));
                ui.add(egui::Slider::new(&mut self.ui_length, 1..=150));
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("RGB Channel Sequence / Chip Order:");
                egui::ComboBox::from_id_source("chip_order_cb_hw")
                    .selected_text(format!("{:?}", self.ui_chip_order))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Grb, "GRB (WS2812B standard)");
                        ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Rgb, "RGB (Standard)");
                        ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Rbg, "RBG");
                        ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Gbr, "GBR");
                        ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Brg, "BRG");
                        ui.selectable_value(&mut self.ui_chip_order, ChipOrder::Bgr, "BGR");
                    });
            });

            ui.add_space(12.0);
            ui.checkbox(
                &mut self.ui_confirm_safety,
                "I understand this modifies non-volatile controller registers (--i-understand-this-changes-the-device)",
            );

            ui.add_space(8.0);
            let btn_enabled = self.ui_confirm_safety;
            let apply_btn = egui::Button::new(
                egui::RichText::new("Apply Hardware Configuration")
                    .strong()
                    .color(if btn_enabled { Color32::WHITE } else { Color32::GRAY }),
            )
            .fill(if btn_enabled {
                Color32::from_rgb(16, 185, 129)
            } else {
                Color32::from_rgb(35, 42, 58)
            })
            .rounding(Rounding::same(6.0));

            if ui.add_enabled(btn_enabled, apply_btn).clicked() {
                self.send(GuiCommand::SendCommand(Command::SetLength(self.ui_length)));
                self.send(GuiCommand::SendCommand(Command::SetChipOrder(self.ui_chip_order)));
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

    /// Renders the real-time command hex trace.
    fn render_tab_diagnostics(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Live Protocol Hex Command Trace")
                        .strong()
                        .size(13.0)
                        .color(Color32::from_rgb(200, 215, 235)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Clear Trace Log").clicked() {
                        self.logs.clear();
                    }
                });
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(260.0)
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
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 50.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        painter.rect_filled(rect, Rounding::same(4.0), Color32::from_rgb(14, 18, 26));
        painter.rect_stroke(
            rect,
            Rounding::same(4.0),
            Stroke::new(1.0_f32, Color32::from_rgb(30, 38, 54)),
        );

        let bars = 16;
        let spacing = rect.width() / (bars as f32);
        let bar_width = spacing * 0.7;

        for i in 0..bars {
            let freq = i as f32 * 1.35;
            let height_factor = if self.ui_power {
                let wave = ((self.audio_visualizer_timer * 8.0 + freq).sin() * 0.4 + 0.5)
                    * ((self.audio_visualizer_timer * 3.7 + freq * 2.1).cos() * 0.3 + 0.7);
                wave.clamp(0.1, 0.95)
            } else {
                0.05
            };

            let bar_h = rect.height() * height_factor * (self.ui_sensitivity as f32 / 16.0);
            let bar_rect = Rect::from_min_size(
                Pos2::new(
                    rect.min.x + spacing * (i as f32 + 0.15),
                    rect.max.y - bar_h - 2.0,
                ),
                Vec2::new(bar_width, bar_h),
            );

            // Equalizer color gradient
            let col = if i < 6 {
                Color32::from_rgb(0, 210, 255) // Cyan
            } else if i < 11 {
                Color32::from_rgb(255, 175, 0) // Amber
            } else {
                Color32::from_rgb(255, 50, 100) // Hot pink
            };

            painter.rect_filled(bar_rect, Rounding::same(2.0), col);
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

        let factor = (self.ui_brightness as f32) / 255.0;

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
                for _i in 0..num_pixels {
                    let beat = ((self.animation_timer * 10.0).sin().abs()) * factor;
                    let r = (self.ui_color[0] * 255.0 * beat) as u8;
                    let g = (self.ui_color[1] * 255.0 * beat) as u8;
                    let b = (self.ui_color[2] * 255.0 * beat) as u8;
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

    /// Renders the individual physical LED diodes along the strip track.
    fn render_virtual_strip(&self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 46.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Dark acrylic strip casing
        painter.rect_filled(rect, Rounding::same(6.0), Color32::from_rgb(14, 17, 24));
        painter.rect_stroke(
            rect,
            Rounding::same(6.0),
            Stroke::new(1.0_f32, Color32::from_rgb(36, 44, 62)),
        );

        let count = self.strip_pixels.len();
        if count == 0 {
            return;
        }

        let spacing = rect.width() / (count as f32);
        let radius = (spacing * 0.36).clamp(3.0, 10.0);

        for (i, p) in self.strip_pixels.iter().enumerate() {
            let center = Pos2::new(
                rect.min.x + spacing * (i as f32 + 0.5),
                rect.min.y + rect.height() * 0.5,
            );

            let color = Color32::from_rgb(p.r, p.g, p.b);

            // Outer diffusion aura glow
            if self.ui_power && (p.r > 20 || p.g > 20 || p.b > 20) {
                let glow_color = Color32::from_rgba_premultiplied(p.r / 3, p.g / 3, p.b / 3, 40);
                painter.circle_filled(center, radius * 2.2, glow_color);
            }

            // Core LED diode
            painter.circle_filled(center, radius, color);
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(0.5_f32, Color32::from_black_alpha(100)),
            );

            // Center diode specular highlight
            if self.ui_power && (p.r > 50 || p.g > 50 || p.b > 50) {
                painter.circle_filled(center, radius * 0.35, Color32::from_white_alpha(120));
            }
        }
    }
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

    // Outer border ring
    painter.circle_stroke(
        center,
        radius,
        Stroke::new(1.5_f32, Color32::from_rgb(60, 75, 105)),
    );

    // Calculate indicator position from current RGB
    let (curr_h, curr_s, _) = rgb_to_hsv(current_color[0], current_color[1], current_color[2]);
    let curr_angle = curr_h.to_radians();
    let indicator_pos = center + Vec2::new(curr_angle.cos(), curr_angle.sin()) * (curr_s * radius);

    // Draw crosshair thumb ring
    painter.circle_stroke(indicator_pos, 8.0, Stroke::new(2.5_f32, Color32::WHITE));
    painter.circle_stroke(indicator_pos, 9.5, Stroke::new(1.0_f32, Color32::BLACK));

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

    // Spawn async background worker managing the device
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
            // Default connect to simulator on startup for immediate out-of-the-box demo
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
            .with_inner_size([780.0, 640.0])
            .with_min_inner_size([560.0, 480.0])
            .with_title("Pyxillum - Pixel LED Controller")
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
