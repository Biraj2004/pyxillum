#![warn(missing_docs)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Pyxillum Desktop Application
//!
//! Native cross-platform GUI for controlling Bluetooth Low Energy pixel LED controllers.
//! Presentation-only layer; all protocol logic is isolated in `pyxillum-protocol`
//! and BLE communication is decoupled on a background Tokio runtime.

use eframe::egui::{self, Color32, Pos2, Stroke, Vec2};
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
    HardwareSetup,
    Diagnostics,
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
    strip_pixels: Vec<RgbColor>,
}

impl PyxillumApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        cmd_tx: mpsc::Sender<GuiCommand>,
        state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
        simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    ) -> Self {
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
        if self.logs.len() > 50 {
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

        // Update virtual strip pixels
        self.update_virtual_pixels();

        // Keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.ui_power = !self.ui_power;
            let cmd = Command::Power(self.ui_power);
            self.log(
                &format!("Keyboard toggle power: {}", self.ui_power),
                &hex_string(&cmd.encode()),
            );
            self.send(GuiCommand::SendCommand(cmd));
        }

        // Top Navigation Bar
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Pyxillum");
                ui.label("• Pixel LED Desktop Controller");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .selectable_label(self.is_demo_mode, "Demo Device (Simulated)")
                        .clicked()
                    {
                        self.is_demo_mode = !self.is_demo_mode;
                        if self.is_demo_mode {
                            self.send(GuiCommand::Connect("SIM:00:11:22:33:44:55".to_string()));
                            self.log("Switched to Virtual Simulator Transport", "");
                        } else {
                            self.send(GuiCommand::Disconnect);
                            self.log("Disconnected from Simulator", "");
                        }
                    }

                    if ui.button("Scan BLE").clicked() {
                        self.send(GuiCommand::Scan);
                        self.log("Scanning for Bluetooth peripherals...", "");
                    }
                });
            });
        });

        // Main Control Area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(6.0);

                // Section 1: Live Virtual Pixel Strip Visualizer
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Virtual LED Strip Monitor");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pwr_lbl = if self.ui_power { "ACTIVE" } else { "OFF" };
                            ui.label(format!("{} pixels | Power: {}", self.ui_length, pwr_lbl));
                        });
                    });
                    ui.separator();
                    self.render_virtual_strip(ui);
                });

                ui.add_space(10.0);

                // Section 2: Tab Bar Navigation
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.active_tab,
                        ActiveTab::Illumination,
                        "Illumination",
                    );
                    ui.selectable_value(
                        &mut self.active_tab,
                        ActiveTab::DynamicEffects,
                        "Dynamic Effects",
                    );
                    ui.selectable_value(
                        &mut self.active_tab,
                        ActiveTab::MusicReactive,
                        "Music & Sound",
                    );
                    ui.selectable_value(
                        &mut self.active_tab,
                        ActiveTab::HardwareSetup,
                        "Hardware Setup",
                    );
                    ui.selectable_value(
                        &mut self.active_tab,
                        ActiveTab::Diagnostics,
                        "Diagnostics",
                    );
                });
                ui.separator();

                ui.add_space(8.0);

                // Section 3: Active Tab Content
                match self.active_tab {
                    ActiveTab::Illumination => self.render_tab_illumination(ui),
                    ActiveTab::DynamicEffects => self.render_tab_effects(ui),
                    ActiveTab::MusicReactive => self.render_tab_music(ui),
                    ActiveTab::HardwareSetup => self.render_tab_hardware(ui),
                    ActiveTab::Diagnostics => self.render_tab_diagnostics(ui),
                }
            });
        });

        // Request constant repaints for 60 FPS strip simulation
        ctx.request_repaint();
    }
}

impl PyxillumApp {
    fn render_tab_illumination(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            // Left Column: Power, Color, Brightness
            columns[0].group(|ui| {
                ui.heading("Primary Illumination");
                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .button(if self.ui_power { "Turn OFF" } else { "Turn ON" })
                        .clicked()
                    {
                        self.ui_power = !self.ui_power;
                        let cmd = Command::Power(self.ui_power);
                        self.log("Toggled power", &hex_string(&cmd.encode()));
                        self.send(GuiCommand::SendCommand(cmd));
                    }
                    let status = if self.ui_power {
                        "Status: ACTIVE (ON)"
                    } else {
                        "Status: OFF"
                    };
                    ui.label(status);
                });

                ui.add_space(8.0);
                ui.label("Static Colour Selection:");
                if ui.color_edit_button_rgb(&mut self.ui_color).changed() {
                    self.ui_effect = EFFECT_SOLID;
                    let r = (self.ui_color[0] * 255.0) as u8;
                    let g = (self.ui_color[1] * 255.0) as u8;
                    let b = (self.ui_color[2] * 255.0) as u8;
                    let cmd = Command::SetColor {
                        color: RgbColor::new(r, g, b),
                        level: self.ui_brightness,
                    };
                    self.log("Updated static colour", &hex_string(&cmd.encode()));
                    self.send(GuiCommand::SendCommand(cmd));
                }

                ui.add_space(8.0);
                ui.label(format!("Brightness Level: {}/255", self.ui_brightness));
                if ui
                    .add(egui::Slider::new(&mut self.ui_brightness, 0..=255))
                    .changed()
                {
                    let cmd = Command::SetBrightness(self.ui_brightness);
                    self.send(GuiCommand::SendCommand(cmd));
                }

                ui.add_space(10.0);
                ui.label("Palette Quick Swatches:");
                ui.horizontal_wrapped(|ui| {
                    let swatches = [
                        ("Amber", [1.0, 0.47, 0.0]),
                        ("Cyan", [0.0, 0.9, 1.0]),
                        ("Magenta", [1.0, 0.0, 0.5]),
                        ("Green", [0.0, 0.9, 0.3]),
                        ("Indigo", [0.24, 0.35, 1.0]),
                        ("Warm White", [1.0, 0.88, 0.7]),
                        ("Pure White", [1.0, 1.0, 1.0]),
                    ];
                    for (label, col) in swatches {
                        if ui.button(label).clicked() {
                            self.ui_color = col;
                            self.ui_effect = EFFECT_SOLID;
                            let r = (col[0] * 255.0) as u8;
                            let g = (col[1] * 255.0) as u8;
                            let b = (col[2] * 255.0) as u8;
                            let cmd = Command::SetColor {
                                color: RgbColor::new(r, g, b),
                                level: self.ui_brightness,
                            };
                            self.send(GuiCommand::SendCommand(cmd));
                        }
                    }
                });
            });

            // Right Column: Presets
            columns[1].group(|ui| {
                ui.heading("Curated Scene Presets");
                ui.separator();

                let presets = [
                    ("Cozy Amber", [1.0, 0.47, 0.08], EFFECT_SOLID, 5, 200),
                    ("Deep Ocean", [0.0, 0.35, 1.0], EFFECT_SOLID, 5, 180),
                    ("Party Rainbow", [1.0, 1.0, 1.0], 0x01, 8, 255),
                    ("Zen Breath", [0.2, 0.9, 0.4], 0x5E, 3, 140),
                    ("Fire Glow", [1.0, 0.25, 0.0], 0x12, 6, 220),
                    ("Cyber Neon", [0.0, 1.0, 0.9], 0x02, 7, 240),
                ];

                for (name, col, eff, spd, br) in presets {
                    if ui.button(name).clicked() {
                        self.ui_color = col;
                        self.ui_effect = eff;
                        self.ui_speed = spd;
                        self.ui_brightness = br;

                        let r = (col[0] * 255.0) as u8;
                        let g = (col[1] * 255.0) as u8;
                        let b = (col[2] * 255.0) as u8;

                        self.send(GuiCommand::SendCommand(Command::SetEffect(eff)));
                        self.send(GuiCommand::SendCommand(Command::SetSpeed(spd)));
                        self.send(GuiCommand::SendCommand(Command::SetColor {
                            color: RgbColor::new(r, g, b),
                            level: br,
                        }));
                        self.log(&format!("Applied preset '{}'", name), "");
                    }
                }
            });
        });
    }

    fn render_tab_effects(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Dynamic Flow Patterns (SP6xxE Standard)");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label(format!("Animation Speed: {}/10", self.ui_speed));
                if ui
                    .add(egui::Slider::new(&mut self.ui_speed, 1..=10))
                    .changed()
                {
                    self.send(GuiCommand::SendCommand(Command::SetSpeed(self.ui_speed)));
                }
            });

            ui.add_space(8.0);
            ui.label("Pattern Selection:");

            let effects = [
                (EFFECT_SOLID, "Static Solid Illumination"),
                (0x01, "Rainbow Spectrum Sweep"),
                (0x02, "Meteor Trail Stream"),
                (0x03, "Seven Colour Flow"),
                (0x07, "Stacking & Chasing Dots"),
                (0x12, "Fire Heat Waves"),
                (0x19, "Dual Snake Flow"),
                (0x2E, "Wave Ripple Pulse"),
                (0x3A, "Spin & Vortex Flow"),
                (0x45, "Starfield Sparkle"),
                (0x5E, "Atmospheric Breath"),
                (0x81, "Strobe Flash Pulse"),
            ];

            ui.columns(2, |cols| {
                for (idx, (code, label)) in effects.iter().enumerate() {
                    let col_idx = idx % 2;
                    if cols[col_idx]
                        .selectable_label(self.ui_effect == *code, *label)
                        .clicked()
                    {
                        self.ui_effect = *code;
                        let cmd = Command::SetEffect(*code);
                        self.log(
                            &format!("Selected pattern: {}", label),
                            &hex_string(&cmd.encode()),
                        );
                        self.send(GuiCommand::SendCommand(cmd));
                    }
                }
            });

            ui.add_space(12.0);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Operating Cycle Mode:");
                if ui.button("Single Pattern Mode").clicked() {
                    self.send(GuiCommand::SendCommand(Command::SetLightMode(
                        LightMode::Single,
                    )));
                    self.log("Set operating mode: Single", "");
                }
                if ui.button("Auto-Cycle Dynamic Patterns").clicked() {
                    self.send(GuiCommand::SendCommand(Command::SetLightMode(
                        LightMode::AutoDynamic,
                    )));
                    self.log("Set operating mode: Auto Dynamic", "");
                }
            });
        });
    }

    fn render_tab_music(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Sound-Reactive & Music Modes");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Audio Input Source:");
                egui::ComboBox::from_id_source("audio_input_cb")
                    .selected_text(match self.ui_audio_input {
                        AudioInput::InternalMic => "Internal Controller Microphone",
                        AudioInput::Player => "Host Software Stream (Player)",
                        AudioInput::ExternalMic => "External 3.5mm Aux Microphone",
                    })
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut self.ui_audio_input,
                                AudioInput::InternalMic,
                                "Internal Controller Microphone",
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
                                "Host Software Stream (Player)",
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
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Microphone Sensitivity: {}/16",
                    self.ui_sensitivity
                ));
                if ui
                    .add(egui::Slider::new(&mut self.ui_sensitivity, 1..=16))
                    .changed()
                {
                    self.send(GuiCommand::SendCommand(Command::SetSensitivity(
                        self.ui_sensitivity,
                    )));
                }
            });

            ui.add_space(12.0);
            ui.label("Rhythm & Spectrum Patterns:");
            let music_effects = [
                (0xC9, "Music Spectrum Pulse"),
                (0xCA, "Music Energy Blink"),
                (0xCB, "Music VuMeter Flow"),
                (0xCC, "Music Peak Hits"),
            ];

            for (code, label) in music_effects {
                if ui.selectable_label(self.ui_effect == code, label).clicked() {
                    self.ui_effect = code;
                    self.send(GuiCommand::SendCommand(Command::SetEffect(code)));
                    self.log(&format!("Selected sound pattern: {}", label), "");
                }
            }

            ui.add_space(12.0);
            if ui.button("Activate Auto-Sound Cycle").clicked() {
                self.send(GuiCommand::SendCommand(Command::SetLightMode(
                    LightMode::AutoSound,
                )));
                self.log("Set operating mode: Auto Sound Reactive", "");
            }
        });
    }

    fn render_tab_hardware(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Physical Hardware & LED Strip Configuration");
            ui.separator();

            ui.colored_label(
                Color32::from_rgb(255, 170, 0),
                "Hardware Safety Gate: These settings write directly to non-volatile EEPROM.",
            );
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(format!("Strip Pixel Count: {} pixels", self.ui_length));
                ui.add(egui::Slider::new(&mut self.ui_length, 1..=150));
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Colour Sequence / Chip Order:");
                egui::ComboBox::from_id_source("chip_order_cb")
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
            if ui
                .add_enabled(btn_enabled, egui::Button::new("Apply Hardware Configuration"))
                .clicked()
            {
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

    fn render_tab_diagnostics(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Live Diagnostic Command Log");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Clear Log").clicked() {
                        self.logs.clear();
                    }
                });
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(240.0)
                .show(ui, |ui| {
                    for entry in self.logs.iter().rev() {
                        ui.horizontal(|ui| {
                            ui.monospace(&entry.timestamp);
                            ui.label(&entry.message);
                            if !entry.hex.is_empty() {
                                ui.colored_label(
                                    Color32::LIGHT_GREEN,
                                    format!("HEX: [{}]", entry.hex),
                                );
                            }
                        });
                    }
                });
        });
    }

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
                // Rainbow Spectrum Sweep
                for i in 0..num_pixels {
                    let hue = (self.animation_timer * 15.0
                        + (i as f32 * 360.0 / num_pixels as f32))
                        % 360.0;
                    let (r, g, b) = hsv_to_rgb(hue, 1.0, factor);
                    self.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            0x02 => {
                // Meteor Trail
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
                // Fire Flicker
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
            _ => {
                // Animated wave ripple
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

    fn render_virtual_strip(&self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 44.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Dark background track
        painter.rect_filled(rect, 4.0, Color32::from_rgb(18, 20, 24));
        painter.rect_stroke(
            rect,
            4.0,
            Stroke::new(1.0_f32, Color32::from_rgb(40, 44, 52)),
        );

        let count = self.strip_pixels.len();
        if count == 0 {
            return;
        }

        let spacing = rect.width() / (count as f32);
        let radius = (spacing * 0.35).clamp(3.0, 10.0);

        for (i, p) in self.strip_pixels.iter().enumerate() {
            let center = Pos2::new(
                rect.min.x + spacing * (i as f32 + 0.5),
                rect.min.y + rect.height() * 0.5,
            );

            let color = Color32::from_rgb(p.r, p.g, p.b);

            // Glow aura
            if self.ui_power && (p.r > 20 || p.g > 20 || p.b > 20) {
                let glow_color = Color32::from_rgba_premultiplied(p.r / 4, p.g / 4, p.b / 4, 30);
                painter.circle_filled(center, radius * 1.8, glow_color);
            }

            // Core LED diode
            painter.circle_filled(center, radius, color);
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(0.5_f32, Color32::from_black_alpha(100)),
            );
        }
    }
}

fn hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
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
            .with_inner_size([760.0, 620.0])
            .with_min_inner_size([540.0, 460.0])
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
