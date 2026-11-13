//! # Pyxillum Desktop Application Coordinator
//!
//! Orchestrates the multi-column desktop layout, screen-wise view routing,
//! real-time 60 FPS strip simulation, and background command dispatch.

use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_core::DeviceState;
use pyxillum_protocol::{Command, LightMode, RgbColor};
use pyxillum_sim::DeviceSimulator;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex};

use crate::screens::{
    render_device_hub, render_diy_mode, render_dynamic_mode, render_music_mode,
    render_settings_modal, render_solid_mode, DeviceHubAction,
};
use crate::state::{AppScreen, AppState, DashboardMode, DeviceCardInfo, GuiCommand, GuiEvent};
use crate::theme::Theme;
use crate::widgets::{render_effect_drawer, render_neon_strip, render_sidebar};
use crate::windows_media::{NowPlayingInfo, WindowsAudioService};

pub struct PyxillumApp {
    cmd_tx: mpsc::Sender<GuiCommand>,
    event_rx: mpsc::Receiver<GuiEvent>,
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
        cc: &eframe::CreationContext<'_>,
        cmd_tx: mpsc::Sender<GuiCommand>,
        event_rx: mpsc::Receiver<GuiEvent>,
        state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
        simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    ) -> Self {
        // Configure modern dark theme visuals
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Theme::CANVAS_BG;
        visuals.window_fill = Theme::CARD_BG;
        visuals.window_stroke = Stroke::new(1.0_f32, Theme::BORDER_LIGHT);
        visuals.widgets.noninteractive.bg_fill = Theme::CARD_BG;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Theme::BORDER_LIGHT);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Theme::TEXT_PRIMARY);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(23, 32, 54);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Theme::BORDER_LIGHT);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Theme::TEXT_SECONDARY);
        visuals.widgets.hovered.bg_fill = Theme::CARD_HOVER;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Theme::ACCENT_CYAN);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Theme::TEXT_PRIMARY);
        visuals.widgets.active.bg_fill = Color32::from_rgb(30, 58, 138);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, Theme::ACCENT_CYAN);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        cc.egui_ctx.set_visuals(visuals);

        let audio_service = WindowsAudioService::new();

        Self {
            cmd_tx,
            event_rx,
            _state_reader: state_reader,
            _simulator_ref: simulator_ref,
            audio_service,

            state: AppState::default(),
            saved_devices: Vec::new(),
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

        // Drain background worker events (real BLE discoveries and connection updates)
        while let Ok(evt) = self.event_rx.try_recv() {
            match evt {
                GuiEvent::ScanStarted => {
                    self.state.is_scanning = true;
                    self.state.status_message =
                        "Scanning for nearby Bluetooth Low Energy controllers (2.5s)..."
                            .to_string();
                }
                GuiEvent::ScanFinished(devices) => {
                    self.state.is_scanning = false;
                    self.saved_devices.clear();
                    for dev in devices {
                        let model = if let Some(n) = &dev.name {
                            if n.contains("621") {
                                "SP621E"
                            } else if n.contains("530") {
                                "SP530E"
                            } else if n.contains("630") {
                                "SP630E"
                            } else if n.contains("801") {
                                "SP801E"
                            } else {
                                "SP-Family"
                            }
                        } else {
                            "SP-Family"
                        };
                        self.saved_devices.push(DeviceCardInfo {
                            id: dev.address.clone(),
                            name: dev
                                .name
                                .clone()
                                .unwrap_or_else(|| "BLE Controller".to_string()),
                            model: model.to_string(),
                            protocol_type: "BLE GATT (0xFFE0)".to_string(),
                            is_simulated: dev.is_simulated,
                            is_connected: self.state.is_connected
                                && self.state.active_device_id == dev.address,
                            rssi: dev.rssi.unwrap_or(-75) as i8,
                        });
                    }
                    self.state.status_message = if self.saved_devices.is_empty() {
                        "No BLE devices discovered. Ensure controller is powered on.".to_string()
                    } else {
                        format!(
                            "Discovered {} Bluetooth device(s). Ready to pair.",
                            self.saved_devices.len()
                        )
                    };
                }
                GuiEvent::Connected { address, name } => {
                    self.state.is_connected = true;
                    self.state.active_device_id = address.clone();
                    self.state.current_screen = AppScreen::Dashboard(address.clone());
                    self.state.status_message = format!("Connected to {} ({})", name, address);
                    for dev in &mut self.saved_devices {
                        dev.is_connected = dev.id == address;
                    }
                }
                GuiEvent::Disconnected => {
                    self.state.is_connected = false;
                    self.state.current_screen = AppScreen::DeviceHub;
                    self.state.status_message =
                        "Disconnected from Bluetooth controller.".to_string();
                    for dev in &mut self.saved_devices {
                        dev.is_connected = false;
                    }
                }
                GuiEvent::StatusUpdated(status) => {
                    self.state.ui_power = status.power;
                    self.state.ui_brightness = status.brightness;
                    self.state.ui_speed = status.speed;
                    self.state.ui_effect = status.effect;
                    self.state.ui_length = status.length;
                    self.state.ui_chip_order = status.chip_order;
                    self.state.ui_color = [
                        status.color.r as f32 / 255.0,
                        status.color.g as f32 / 255.0,
                        status.color.b as f32 / 255.0,
                    ];
                }
                GuiEvent::Error(err) => {
                    self.state.is_scanning = false;
                    self.state.status_message = format!("Bluetooth: {}", err);
                }
            }
        }

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
                    .inner_margin(egui::Margin::symmetric(20.0, 16.0)),
            )
            .show(ctx, |ui| {
                // Top Header Action Bar
                self.render_top_header(ui);

                ui.add_space(10.0);

                // Embedded 60 FPS Strip Simulation Monitor
                render_neon_strip(ui, &self.state.strip_pixels, self.state.ui_power);

                ui.add_space(14.0);

                // Responsive Multi-Pane Stage inside ScrollArea
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| match &self.state.current_screen {
                        AppScreen::DeviceHub => {
                            if let Some(action) =
                                render_device_hub(ui, &mut self.state, &self.saved_devices)
                            {
                                match action {
                                    DeviceHubAction::Scan => {
                                        self.send(GuiCommand::Scan);
                                    }
                                    DeviceHubAction::Connect(addr) => {
                                        self.send(GuiCommand::Connect(addr));
                                    }
                                    DeviceHubAction::Disconnect => {
                                        self.send(GuiCommand::Disconnect);
                                    }
                                }
                            }
                        }
                        AppScreen::Dashboard(_) => {
                            let avail_w = ui.available_width();
                            let is_wide = avail_w >= 940.0;
                            let show_split_drawer =
                                is_wide && self.state.dashboard_mode == DashboardMode::Dynamic;

                            if show_split_drawer {
                                // Two-Column Desktop Layout: Center Controls + Right Preset Drawer
                                ui.columns(2, |cols| {
                                    // Left Column: Interactive Dial & Controls
                                    cols[0].vertical(|ui| {
                                        egui::Frame::none()
                                            .fill(Theme::CARD_BG)
                                            .rounding(Rounding::same(16.0))
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
                                            .rounding(Rounding::same(16.0))
                                            .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                                            .inner_margin(egui::Margin::symmetric(20.0, 16.0))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    egui::RichText::new(
                                                        "Lighting Styles Catalog (146)",
                                                    )
                                                    .strong()
                                                    .size(15.0)
                                                    .color(Theme::TEXT_PRIMARY),
                                                );
                                                ui.add_space(6.0);
                                                if let Some(code) =
                                                    render_effect_drawer(ui, &mut self.state)
                                                {
                                                    self.state.ui_effect = code;
                                                    self.send(GuiCommand::SendCommand(
                                                        Command::SetEffect(code),
                                                    ));
                                                }
                                            });
                                    });
                                });
                            } else {
                                // Centered Single Stage: Max width constrained to 740px
                                ui.vertical_centered(|ui| {
                                    let stage_w = avail_w.min(740.0);
                                    ui.allocate_ui_with_layout(
                                        Vec2::new(stage_w, ui.available_height()),
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui| {
                                            egui::Frame::none()
                                                .fill(Theme::CARD_BG)
                                                .rounding(Rounding::same(16.0))
                                                .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                                                .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                                                .show(ui, |ui| {
                                                    self.render_active_screen_content(ui);
                                                });
                                        },
                                    );
                                });
                            }
                        }
                    });
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
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), egui::Sense::hover());
        let p = ui.painter_at(rect);
        p.rect_filled(rect, Rounding::same(10.0), Theme::HEADER_BG);
        p.rect_stroke(
            rect,
            Rounding::same(10.0),
            Stroke::new(1.0_f32, Theme::HEADER_BORDER),
        );

        // Left Section: Breadcrumb / Back button & Device title
        let left_w = 280.0;
        let left_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 8.0, rect.min.y),
            Vec2::new(left_w, rect.height()),
        );
        ui.allocate_ui_at_rect(left_rect, |left_ui| {
            left_ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center),
                |left_ui| {
                    if let AppScreen::Dashboard(_) = &self.state.current_screen {
                        if left_ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("< Devices")
                                        .size(12.5)
                                        .color(Theme::ACCENT_CYAN),
                                )
                                .fill(Color32::from_rgb(23, 32, 54))
                                .rounding(Rounding::same(6.0)),
                            )
                            .clicked()
                        {
                            self.state.current_screen = AppScreen::DeviceHub;
                        }

                        left_ui.add_space(8.0);
                        let display_title = if !self.state.active_device_id.is_empty() {
                            format!(
                                "Controller ({})",
                                &self.state.active_device_id
                                    [..self.state.active_device_id.len().min(8)]
                            )
                        } else {
                            "SP621E Controller".to_string()
                        };

                        left_ui.label(
                            egui::RichText::new(display_title)
                                .strong()
                                .size(14.0)
                                .color(Theme::TEXT_PRIMARY),
                        );

                        left_ui.add_space(4.0);
                        let dot_col = if self.state.is_connected {
                            Theme::ACCENT_EMERALD
                        } else {
                            Theme::TEXT_MUTED
                        };
                        let (d_rect, _) =
                            left_ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                        left_ui
                            .painter()
                            .circle_filled(d_rect.center(), 3.5, dot_col);
                    } else {
                        left_ui.label(
                            egui::RichText::new("Pyxillum Device Studio")
                                .strong()
                                .size(15.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        left_ui.add_space(6.0);
                        left_ui.label(
                            egui::RichText::new("Bluetooth Low Energy")
                                .size(11.5)
                                .color(Theme::TEXT_MUTED),
                        );
                    }
                },
            );
        });

        // Center Section: Mode Tabs (when on Dashboard)
        if matches!(self.state.current_screen, AppScreen::Dashboard(_)) {
            let center_x = rect.center().x;
            let tab_bar_w = 340.0;
            let tab_rect = Rect::from_center_size(
                Pos2::new(center_x, rect.center().y),
                Vec2::new(tab_bar_w, 32.0),
            );
            ui.allocate_ui_at_rect(tab_rect, |center_ui| {
                center_ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::Center),
                    |center_ui| {
                        let modes = [
                            (DashboardMode::Dynamic, "Dynamic"),
                            (DashboardMode::Solid, "Solid"),
                            (DashboardMode::Music, "Music"),
                            (DashboardMode::Diy, "DIY"),
                        ];

                        for (m, lbl) in modes {
                            let is_sel = self.state.dashboard_mode == m;
                            let btn = egui::Button::new(
                                egui::RichText::new(lbl)
                                    .size(12.0)
                                    .strong()
                                    .color(if is_sel {
                                        Color32::WHITE
                                    } else {
                                        Theme::TEXT_SECONDARY
                                    }),
                            )
                            .fill(if is_sel {
                                Theme::ACCENT_BLUE
                            } else {
                                Color32::TRANSPARENT
                            })
                            .rounding(Rounding::same(6.0))
                            .min_size(Vec2::new(76.0, 26.0));

                            if center_ui.add(btn).clicked() {
                                self.state.dashboard_mode = m;
                            }
                            center_ui.add_space(4.0);
                        }
                    },
                );
            });
        }

        // Right Section: Power Button & Settings
        let right_rect = Rect::from_min_max(
            Pos2::new(rect.max.x - 200.0, rect.min.y),
            Pos2::new(rect.max.x - 8.0, rect.max.y),
        );
        ui.allocate_ui_at_rect(right_rect, |right_ui| {
            right_ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |right_ui| {
                    // Power Switch Button
                    let (pwr_rect, pwr_resp) =
                        right_ui.allocate_exact_size(Vec2::splat(32.0), egui::Sense::click());
                    let pwr_p = right_ui.painter_at(pwr_rect);
                    let pwr_col = if self.state.ui_power {
                        Theme::ACCENT_RED
                    } else {
                        Color32::from_rgb(71, 85, 105)
                    };
                    pwr_p.circle_filled(pwr_rect.center(), 15.0, pwr_col);
                    pwr_p.text(
                        pwr_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "P",
                        egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );

                    if pwr_resp.clicked() {
                        self.state.ui_power = !self.state.ui_power;
                        self.send(GuiCommand::SendCommand(Command::Power(self.state.ui_power)));
                    }

                    right_ui.add_space(10.0);

                    if right_ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Settings")
                                    .size(12.0)
                                    .color(Theme::TEXT_SECONDARY),
                            )
                            .fill(Color32::from_rgb(23, 32, 54))
                            .rounding(Rounding::same(6.0)),
                        )
                        .clicked()
                    {
                        self.state.show_settings_modal = true;
                    }
                },
            );
        });
    }

    fn render_active_screen_content(&mut self, ui: &mut egui::Ui) {
        match &self.state.current_screen {
            AppScreen::DeviceHub => {
                // Handled in main stage ScrollArea
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
