//! # Desktop Left Navigation Rail (Sidebar)
//!
//! Provides the persistent desktop sidebar for device selection,
//! real-time BLE connection status, and mode switching.

use crate::state::{AppScreen, AppState, DashboardMode};
use crate::theme::Theme;
use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

/// Renders the left-hand navigation sidebar.
pub fn render_sidebar(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // 1. App Header / Branding
        ui.horizontal(|ui| {
            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
            let ip = ui.painter_at(icon_rect);
            ip.circle_filled(icon_rect.center(), 13.0, Theme::ACCENT_BLUE);
            ip.circle_filled(icon_rect.center(), 6.0, Color32::WHITE);

            ui.add_space(6.0);
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("PYXILLUM")
                        .strong()
                        .size(15.0)
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.label(
                    egui::RichText::new("Pixel LED Studio")
                        .size(10.5)
                        .color(Theme::TEXT_MUTED),
                );
            });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(12.0);

        // 2. Hardware Controller Status Section
        ui.label(
            egui::RichText::new("HARDWARE CONTROLLER")
                .size(10.0)
                .strong()
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(6.0);

        let is_hub = matches!(state.current_screen, AppScreen::DeviceHub);

        // Controller Card in Sidebar
        let ctrl_bg = if is_hub {
            Color32::from_rgb(30, 58, 138)
        } else {
            Color32::from_rgb(19, 27, 46)
        };

        egui::Frame::none()
            .fill(ctrl_bg)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(
                1.0_f32,
                if is_hub {
                    Theme::ACCENT_CYAN
                } else {
                    Theme::BORDER_LIGHT
                },
            ))
            .inner_margin(egui::Margin::symmetric(10.0, 8.0))
            .show(ui, |ui| {
                let dev_name = if state.is_connected && !state.active_device_id.is_empty() {
                    format!("Controller ({})", state.active_device_id)
                } else {
                    "No Device Connected".to_string()
                };

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(dev_name).strong().size(12.0).color(
                            if is_hub {
                                Color32::WHITE
                            } else {
                                Theme::TEXT_PRIMARY
                            },
                        ))
                        .fill(Color32::TRANSPARENT),
                    )
                    .clicked()
                {
                    state.current_screen = AppScreen::DeviceHub;
                }

                ui.horizontal(|ui| {
                    let dot_col = if state.is_connected {
                        Theme::ACCENT_EMERALD
                    } else if state.is_scanning {
                        Theme::ACCENT_AMBER
                    } else {
                        Theme::TEXT_MUTED
                    };

                    let (dot_rect, _) =
                        ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                    ui.painter().circle_filled(dot_rect.center(), 4.0, dot_col);

                    let status_lbl = if state.is_connected {
                        "BLE Link Active"
                    } else if state.is_scanning {
                        "Scanning BLE..."
                    } else {
                        "Offline / Standby"
                    };

                    ui.label(
                        egui::RichText::new(status_lbl)
                            .size(10.5)
                            .color(Theme::TEXT_SECONDARY),
                    );
                });
            });

        ui.add_space(8.0);

        // Quick Device Hub shortcut
        if ui
            .add(
                egui::Button::new(
                    egui::RichText::new("Discovered Controllers")
                        .size(12.0)
                        .color(if is_hub {
                            Theme::ACCENT_CYAN
                        } else {
                            Theme::TEXT_SECONDARY
                        }),
                )
                .fill(if is_hub {
                    Color32::from_rgb(23, 32, 54)
                } else {
                    Color32::TRANSPARENT
                })
                .rounding(Rounding::same(6.0))
                .min_size(Vec2::new(ui.available_width(), 28.0)),
            )
            .clicked()
        {
            state.current_screen = AppScreen::DeviceHub;
        }

        ui.add_space(14.0);
        ui.separator();
        ui.add_space(10.0);

        // 3. Navigation Modes Section
        ui.label(
            egui::RichText::new("LIGHTING MODES")
                .size(10.0)
                .strong()
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(6.0);

        let modes = [
            (DashboardMode::Dynamic, "Dynamic Patterns (146)"),
            (DashboardMode::Solid, "Solid Chromatic Wheel"),
            (DashboardMode::Music, "Music Reactive Spectrum"),
            (DashboardMode::Diy, "DIY 14-Node Sequencer"),
        ];

        for (m, label) in modes {
            let is_sel = !is_hub && state.dashboard_mode == m;
            let btn = ui.add(
                egui::Button::new(egui::RichText::new(label).size(12.5).color(if is_sel {
                    Theme::ACCENT_CYAN
                } else {
                    Theme::TEXT_PRIMARY
                }))
                .fill(if is_sel {
                    Color32::from_rgb(30, 58, 138)
                } else {
                    Color32::TRANSPARENT
                })
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(ui.available_width(), 32.0)),
            );

            if btn.clicked() {
                state.current_screen = AppScreen::Dashboard(state.active_device_id.clone());
                state.dashboard_mode = m;
            }
        }

        ui.add_space(14.0);
        ui.separator();
        ui.add_space(10.0);

        // 4. Configuration Utilities
        ui.label(
            egui::RichText::new("CONFIGURATION")
                .size(10.0)
                .strong()
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(6.0);

        if ui
            .add(
                egui::Button::new(
                    egui::RichText::new("Hardware EEPROM Settings")
                        .size(12.0)
                        .color(Theme::TEXT_SECONDARY),
                )
                .fill(Color32::TRANSPARENT)
                .rounding(Rounding::same(6.0))
                .min_size(Vec2::new(ui.available_width(), 28.0)),
            )
            .clicked()
        {
            state.show_settings_modal = true;
        }

        if ui
            .add(
                egui::Button::new(
                    egui::RichText::new("Documentation & Protocol")
                        .size(12.0)
                        .color(Theme::TEXT_SECONDARY),
                )
                .fill(Color32::TRANSPARENT)
                .rounding(Rounding::same(6.0))
                .min_size(Vec2::new(ui.available_width(), 28.0)),
            )
            .clicked()
        {
            state.show_help_modal = true;
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("v0.1.0 Native Windows Engine")
                    .size(9.5)
                    .color(Theme::TEXT_MUTED),
            );
            ui.label(
                egui::RichText::new("SP621E / SP6xxE Hardware")
                    .size(9.5)
                    .color(Theme::TEXT_MUTED),
            );
        });
    });
}
