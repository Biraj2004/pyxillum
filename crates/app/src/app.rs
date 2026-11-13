//! # Pyxillum Desktop Application Coordinator
//!
//! Orchestrates the multi-column desktop layout, screen-wise view routing,
//! real-time 60 FPS strip simulation, and background command dispatch.

use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};
use pyxillum_core::DeviceState;
use pyxillum_protocol::{Command, LightMode, RgbColor};
use pyxillum_sim::DeviceSimulator;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex};

use crate::screens::{
    render_device_hub, render_diy_mode, render_dynamic_mode, render_music_mode,
    render_settings_modal, render_solid_mode,
};
use crate::state::{AppScreen, AppState, DashboardMode, DeviceCardInfo, GuiCommand};
use crate::theme::Theme;
use crate::widgets::{render_effect_drawer, render_neon_strip, render_sidebar};
use crate::windows_media::{NowPlayingInfo, WindowsAudioService};

pub struct PyxillumApp {
    cmd_tx: mpsc::Sender<GuiCommand>,
    _state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
    _simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    audio_service: WindowsAudioService,

    pub state: AppState,
    pub saved_devices: Vec<DeviceCardInfo>,
    now_playing: NowPlayingInfo,

    last_frame_instant: Instant,
}

impl PyxillumApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        cmd_tx: mpsc::Sender<GuiCommand>,
        state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
        simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    ) -> Self {
        let audio_service = WindowsAudioService::new();

        let saved_devices = vec![
            DeviceCardInfo {
                id: "00:1A:7D:DA:71:09".to_string(),
                name: "SP621E Virtual".to_string(),
                model: "SP621E".to_string(),
                protocol_type: "BLE v2 (0x53)".to_string(),
                is_simulated: true,
                is_connected: true,
                rssi: -58,
            },
            DeviceCardInfo {
                id: "A4:C1:38:52:11:02".to_string(),
                name: "SP621E Mini".to_string(),
                model: "SP621E".to_string(),
                protocol_type: "BLE v2 (0x53)".to_string(),
                is_simulated: false,
                is_connected: false,
                rssi: -72,
            },
            DeviceCardInfo {
                id: "D8:8A:3A:44:91:BC".to_string(),
                name: "SP530E Controller".to_string(),
                model: "SP530E".to_string(),
                protocol_type: "BLE v3 / SPI".to_string(),
                is_simulated: false,
                is_connected: false,
                rssi: -65,
            },
            DeviceCardInfo {
                id: "00:0D:7D:81:4F:2E".to_string(),
                name: "SP630E Dual SPI".to_string(),
                model: "SP630E".to_string(),
                protocol_type: "BLE Mesh / PWM".to_string(),
                is_simulated: false,
                is_connected: false,
                rssi: -80,
            },
        ];

        Self {
            cmd_tx,
            _state_reader: state_reader,
            _simulator_ref: simulator_ref,
            audio_service,

            state: AppState::default(),
            saved_devices,
            now_playing: NowPlayingInfo::default(),

            last_frame_instant: Instant::now(),
        }
    }

    fn send(&self, cmd: GuiCommand) {
        let _ = self.cmd_tx.try_send(cmd);
    }

    /// Computes the 60 FPS real-time LED strip rendering buffer.
    fn update_strip_simulation(&mut self, dt: f32) {
        self.state.animation_timer += dt;
        self.state.audio_visualizer_timer += dt;

        let num_pixels = self.state.ui_length as usize;
        let brightness_factor = if self.state.ui_power {
            self.state.ui_brightness as f32 / 255.0
        } else {
            0.0
        };

        self.state.strip_pixels.clear();

        match self.state.dashboard_mode {
            DashboardMode::Solid => {
                let r = (self.state.ui_color[0] * 255.0 * brightness_factor) as u8;
                let g = (self.state.ui_color[1] * 255.0 * brightness_factor) as u8;
                let b = (self.state.ui_color[2] * 255.0 * brightness_factor) as u8;
                let col = RgbColor::new(r, g, b);
                for _ in 0..num_pixels {
                    self.state.strip_pixels.push(col);
                }
            }
            DashboardMode::Diy => {
                let offset = (self.state.animation_timer * (self.state.diy_speed as f32 * 0.5))
                    as usize
                    % 14;
                for i in 0..num_pixels {
                    let node_idx = match self.state.diy_chase_mode {
                        crate::state::DiyChaseMode::Backward => (14 + offset - (i % 14)) % 14,
                        _ => (i + offset) % 14,
                    };
                    let n = &self.state.diy_nodes[node_idx];
                    let r = ((n.color.r as f32) * brightness_factor) as u8;
                    let g = ((n.color.g as f32) * brightness_factor) as u8;
                    let b = ((n.color.b as f32) * brightness_factor) as u8;
                    self.state.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            DashboardMode::Dynamic => {
                let speed_mult = self.state.ui_speed as f32 * 0.4;
                let t = self.state.animation_timer * speed_mult;

                for i in 0..num_pixels {
                    let pos_ratio = i as f32 / num_pixels as f32;
                    let hue = match self.state.ui_effect {
                        0x01 => (pos_ratio * 360.0 + t * 60.0) % 360.0, // Rainbow Flow
                        0x09 => 20.0 + (pos_ratio * 30.0 + t * 40.0).sin().abs() * 35.0, // Fire
                        0x0F => 0.0,                                    // Comet Red
                        0x11 => 220.0,                                  // Comet Blue
                        146 => (pos_ratio * 180.0 + 180.0) % 360.0,     // Gradient
                        _ => (pos_ratio * 360.0 + t * 40.0) % 360.0,
                    };

                    let (r, g, b) = crate::widgets::color_disc::hsv_to_rgb(hue, 1.0, 1.0);
                    let r = (r * 255.0 * brightness_factor) as u8;
                    let g = (g * 255.0 * brightness_factor) as u8;
                    let b = (b * 255.0 * brightness_factor) as u8;
                    self.state.strip_pixels.push(RgbColor::new(r, g, b));
                }
            }
            DashboardMode::Music => {
                let peak = if self.state.sync_with_windows_audio {
                    self.state.live_audio_peak
                } else {
                    (self.state.audio_visualizer_timer * 6.0).sin() * 0.4 + 0.6
                };

                let active_count = ((num_pixels as f32 * peak) as usize).min(num_pixels);
                for i in 0..num_pixels {
                    if i < active_count && self.state.ui_power {
                        let ratio = i as f32 / num_pixels as f32;
                        let hue = ratio * 240.0;
                        let (r, g, b) = crate::widgets::color_disc::hsv_to_rgb(hue, 1.0, 1.0);
                        let r = (r * 255.0 * brightness_factor) as u8;
                        let g = (g * 255.0 * brightness_factor) as u8;
                        let b = (b * 255.0 * brightness_factor) as u8;
                        self.state.strip_pixels.push(RgbColor::new(r, g, b));
                    } else {
                        self.state.strip_pixels.push(RgbColor::new(15, 23, 42));
                    }
                }
            }
        }
    }
}

impl eframe::App for PyxillumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt = now
            .duration_since(self.last_frame_instant)
            .as_secs_f32()
            .min(0.1);
        self.last_frame_instant = now;

        // Poll live Windows audio metrics
        let (info, peak) = self.audio_service.get_state();
        self.state.live_audio_peak = peak;
        self.now_playing = info;

        // Update real-time LED strip frame
        self.update_strip_simulation(dt);

        // 1. Persistent Left Navigation Rail (Desktop Sidebar)
        egui::SidePanel::left("desktop_sidebar")
            .resizable(false)
            .default_width(220.0)
            .frame(
                egui::Frame::none()
                    .fill(Theme::SIDEBAR_BG)
                    .stroke(Stroke::new(1.0_f32, Theme::SIDEBAR_BORDER))
                    .inner_margin(egui::Margin::symmetric(16.0, 18.0)),
            )
            .show(ctx, |ui| {
                render_sidebar(ui, &mut self.state);
            });

        // 2. Central Workspace Canvas
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Theme::CANVAS_BG)
                    .inner_margin(egui::Margin::symmetric(24.0, 20.0)),
            )
            .show(ctx, |ui| {
                // Top Header Action Bar
                self.render_top_header(ui);

                ui.add_space(14.0);

                // Embedded 60 FPS Strip Simulation
                render_neon_strip(ui, &self.state.strip_pixels, self.state.ui_power);

                ui.add_space(16.0);

                // Responsive Multi-Pane Stage
                let avail_w = ui.available_width();
                let is_wide = avail_w >= 920.0;
                let show_split_drawer =
                    is_wide && self.state.dashboard_mode == DashboardMode::Dynamic;

                if show_split_drawer {
                    // Two-Column Desktop Layout: Center Controls (Max 640px) + Right Preset Drawer (320px)
                    ui.columns(2, |cols| {
                        // Left Column: Interactive Dial & Controls
                        cols[0].vertical_centered(|ui| {
                            egui::Frame::none()
                                .fill(Theme::CARD_BG)
                                .rounding(Rounding::same(20.0))
                                .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                                .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                                .show(ui, |ui| {
                                    self.render_active_screen_content(ui);
                                });
                        });

                        // Right Column: Full 146-Effect Preset Catalog
                        cols[1].vertical(|ui| {
                            egui::Frame::none()
                                .fill(Theme::CARD_BG)
                                .rounding(Rounding::same(20.0))
                                .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                                .inner_margin(egui::Margin::symmetric(20.0, 16.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new("Lighting Styles Catalog (146)")
                                            .strong()
                                            .size(15.0)
                                            .color(Theme::TEXT_PRIMARY),
                                    );
                                    ui.add_space(6.0);
                                    if let Some(code) = render_effect_drawer(ui, &mut self.state) {
                                        self.state.ui_effect = code;
                                        self.send(GuiCommand::SendCommand(Command::SetEffect(
                                            code,
                                        )));
                                    }
                                });
                        });
                    });
                } else {
                    // Centered Single Stage: Max width constrained to 720px
                    ui.vertical_centered(|ui| {
                        let stage_w = avail_w.min(720.0);
                        ui.allocate_ui_with_layout(
                            Vec2::new(stage_w, ui.available_height()),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                egui::Frame::none()
                                    .fill(Theme::CARD_BG)
                                    .rounding(Rounding::same(20.0))
                                    .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                                    .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                                    .show(ui, |ui| {
                                        self.render_active_screen_content(ui);
                                    });
                            },
                        );
                    });
                }
            });

        // 3. Modals and Drawers
        if self.state.show_settings_modal && render_settings_modal(ctx, &mut self.state) {
            // Apply EEPROM parameters
            self.send(GuiCommand::SendCommand(Command::SetLength(
                self.state.ui_length,
            )));
            self.send(GuiCommand::SendCommand(Command::SetChipOrder(
                self.state.ui_chip_order,
            )));
        }

        if self.state.show_help_modal {
            self.render_help_modal(ctx);
        }

        // Floating Effect Drawer Modal (when not in wide two-column mode)
        let avail_w = ctx.screen_rect().width();
        if self.state.show_effect_drawer && avail_w < 1140.0 {
            egui::Window::new("146 Lighting Styles Preset Drawer")
                .collapsible(false)
                .resizable(true)
                .default_size(Vec2::new(380.0, 520.0))
                .anchor(egui::Align2::RIGHT_TOP, Vec2::new(-20.0, 60.0))
                .show(ctx, |ui| {
                    if let Some(code) = render_effect_drawer(ui, &mut self.state) {
                        self.state.ui_effect = code;
                        self.send(GuiCommand::SendCommand(Command::SetEffect(code)));
                    }
                });
        }

        // Request 60 FPS continuous animation redraws
        ctx.request_repaint();
    }
}

impl PyxillumApp {
    fn render_top_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Screen Back / Header
            if let AppScreen::Dashboard(_) = &self.state.current_screen {
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("<")
                                .size(16.0)
                                .color(Theme::TEXT_SECONDARY),
                        )
                        .fill(Color32::from_rgb(241, 245, 249))
                        .rounding(Rounding::same(8.0)),
                    )
                    .clicked()
                {
                    self.state.current_screen = AppScreen::DeviceHub;
                }
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new("SP530E (Bluetooth)")
                        .strong()
                        .size(17.0)
                        .color(Theme::TEXT_PRIMARY),
                );
            } else {
                ui.label(
                    egui::RichText::new("Pyxillum Device Studio")
                        .strong()
                        .size(17.0)
                        .color(Theme::TEXT_PRIMARY),
                );
            }

            // Mode Selector Tabs in Header (when in Dashboard)
            if let AppScreen::Dashboard(_) = &self.state.current_screen {
                ui.add_space(20.0);
                let modes = [
                    (DashboardMode::Dynamic, "Dynamic"),
                    (DashboardMode::Solid, "Solid"),
                    (DashboardMode::Music, "Music"),
                    (DashboardMode::Diy, "DIY"),
                ];

                for (m, lbl) in modes {
                    let is_sel = self.state.dashboard_mode == m;
                    if ui
                        .selectable_label(
                            is_sel,
                            egui::RichText::new(lbl)
                                .size(14.0)
                                .strong()
                                .color(if is_sel {
                                    Theme::ACCENT_BLUE
                                } else {
                                    Theme::TEXT_SECONDARY
                                }),
                        )
                        .clicked()
                    {
                        self.state.dashboard_mode = m;
                    }
                    ui.add_space(4.0);
                }
            }

            // Right-Hand Controls: Help, Settings, and Master Red Power Switch
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Master Power Button
                let (pwr_rect, pwr_resp) =
                    ui.allocate_exact_size(Vec2::splat(36.0), egui::Sense::click());
                let p = ui.painter_at(pwr_rect);
                let pwr_col = if self.state.ui_power {
                    Theme::ACCENT_RED
                } else {
                    Color32::from_rgb(148, 163, 184)
                };
                p.circle_filled(pwr_rect.center(), 17.0, pwr_col);
                p.text(
                    pwr_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "P",
                    egui::FontId::proportional(15.0),
                    Color32::WHITE,
                );

                if pwr_resp.clicked() {
                    self.state.ui_power = !self.state.ui_power;
                    self.send(GuiCommand::SendCommand(Command::Power(self.state.ui_power)));
                }

                ui.add_space(8.0);

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("Settings").size(12.0))
                            .fill(Color32::from_rgb(241, 245, 249))
                            .rounding(Rounding::same(8.0)),
                    )
                    .clicked()
                {
                    self.state.show_settings_modal = true;
                }

                ui.add_space(4.0);

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("?").size(13.0).strong())
                            .fill(Color32::from_rgb(241, 245, 249))
                            .rounding(Rounding::same(14.0)),
                    )
                    .clicked()
                {
                    self.state.show_help_modal = true;
                }
            });
        });
    }

    fn render_active_screen_content(&mut self, ui: &mut egui::Ui) {
        match &self.state.current_screen {
            AppScreen::DeviceHub => {
                if let Some(dev_id) = render_device_hub(ui, &mut self.state, &self.saved_devices) {
                    self.state.active_device_id = dev_id.clone();
                    self.state.current_screen = AppScreen::Dashboard(dev_id);
                }
            }
            AppScreen::Dashboard(_) => match self.state.dashboard_mode {
                DashboardMode::Dynamic => {
                    if render_dynamic_mode(ui, &mut self.state) {
                        self.send(GuiCommand::SendCommand(Command::SetEffect(
                            self.state.ui_effect,
                        )));
                        self.send(GuiCommand::SendCommand(Command::SetSpeed(
                            self.state.ui_speed,
                        )));
                        self.send(GuiCommand::SendCommand(Command::SetBrightness(
                            self.state.ui_brightness,
                        )));
                        let light_mode = if self.state.is_loop_all {
                            LightMode::AutoDynamic
                        } else {
                            LightMode::Single
                        };
                        self.send(GuiCommand::SendCommand(Command::SetLightMode(light_mode)));
                    }
                }
                DashboardMode::Solid => {
                    if render_solid_mode(ui, &mut self.state) {
                        let r = (self.state.ui_color[0] * 255.0) as u8;
                        let g = (self.state.ui_color[1] * 255.0) as u8;
                        let b = (self.state.ui_color[2] * 255.0) as u8;
                        self.send(GuiCommand::SendCommand(Command::SetColor {
                            color: RgbColor::new(r, g, b),
                            level: self.state.ui_brightness,
                        }));
                    }
                }
                DashboardMode::Music => {
                    if render_music_mode(ui, &mut self.state, &self.now_playing) {
                        self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                            self.state.ui_audio_input,
                        )));
                        self.send(GuiCommand::SendCommand(Command::SetSensitivity(
                            self.state.ui_sensitivity,
                        )));
                    }
                }
                DashboardMode::Diy => {
                    if render_diy_mode(ui, &mut self.state) {
                        self.send(GuiCommand::SendCommand(Command::SetSpeed(
                            self.state.diy_speed,
                        )));
                        self.send(GuiCommand::SendCommand(Command::SetBrightness(
                            self.state.diy_brightness,
                        )));
                    }
                }
            },
        }
    }

    fn render_help_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("Pyxillum Help & Architecture Guide")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(460.0)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("About Pyxillum")
                        .strong()
                        .size(15.0)
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(4.0);
                ui.label(
                    "Pyxillum is a native, offline-first Windows controller for SP621E/SP6xxE Bluetooth Low Energy pixel LED devices.",
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Protocol Tenets")
                        .strong()
                        .size(13.0)
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.label("- Write Pacing: 30 ms automatic delay between outbound GATT frames.");
                ui.label("- Clean-Room Interoperability: Direct implementation of unencrypted GATT framing.");
                ui.label("- Hardware Safety: EEPROM register updates are gated behind safety flags.");
                ui.add_space(14.0);

                if ui.button("Close").clicked() {
                    self.state.show_help_modal = false;
                }
            });
    }
}
