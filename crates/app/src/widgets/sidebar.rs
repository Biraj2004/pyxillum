//! # Desktop Left Navigation Rail (Sidebar)
//!
//! Provides the persistent desktop sidebar for device selection,
//! connection status, and mode switching.

use crate::state::{AppScreen, AppState, DashboardMode};
use crate::theme::Theme;
use eframe::egui::{self, Color32, Rounding, Vec2};

pub fn render_sidebar(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // 1. App Header / Branding
        ui.horizontal(|ui| {
            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
            let ip = ui.painter_at(icon_rect);
            ip.circle_filled(icon_rect.center(), 13.0, Theme::ACCENT_BLUE);
            ip.circle_filled(icon_rect.center(), 6.0, Color32::WHITE);

            ui.add_space(4.0);
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("PYXILLUM")
                        .strong()
                        .size(15.0)
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.label(
                    egui::RichText::new("Pixel LED Studio")
                        .size(10.0)
                        .color(Theme::TEXT_MUTED),
                );
            });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(10.0);

        // 2. Active Devices Section
        ui.label(
            egui::RichText::new("ACTIVE DEVICE")
                .size(10.0)
                .strong()
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(6.0);

        let is_hub = matches!(state.current_screen, AppScreen::DeviceHub);
        let dev_btn = ui.add(
            egui::Button::new(egui::RichText::new("SP530E (Bluetooth)").size(13.0).color(
                if !is_hub {
                    Theme::ACCENT_BLUE
                } else {
                    Theme::TEXT_PRIMARY
                },
            ))
            .fill(if !is_hub {
                Color32::from_rgb(239, 246, 255)
            } else {
                Color32::TRANSPARENT
            })
            .rounding(Rounding::same(8.0))
            .min_size(Vec2::new(ui.available_width(), 32.0)),
        );
        if dev_btn.clicked() {
            state.current_screen = AppScreen::Dashboard(state.active_device_id.clone());
        }

        ui.horizontal(|ui| {
            let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(12.0), egui::Sense::hover());
            ui.painter_at(dot_rect)
                .circle_filled(dot_rect.center(), 4.0, Theme::ACCENT_EMERALD);
            ui.label(
                egui::RichText::new("Connected (-68 dBm)")
                    .size(10.5)
                    .color(Theme::TEXT_SECONDARY),
            );
        });

        ui.add_space(6.0);
        if ui
            .add(
                egui::Button::new(
                    egui::RichText::new("+ Device Hub / Scan")
                        .size(11.5)
                        .color(Theme::TEXT_SECONDARY),
                )
                .fill(if is_hub {
                    Color32::from_rgb(239, 246, 255)
                } else {
                    Color32::TRANSPARENT
                })
                .rounding(Rounding::same(6.0)),
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
                    Theme::ACCENT_BLUE
                } else {
                    Theme::TEXT_PRIMARY
                }))
                .fill(if is_sel {
                    Color32::from_rgb(239, 246, 255)
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

        // 4. Utilities
        ui.label(
            egui::RichText::new("CONFIGURATION")
                .size(10.0)
                .strong()
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(6.0);

        if ui
            .add(
                egui::Button::new(egui::RichText::new("Strip Settings").size(12.5))
                    .fill(Color32::TRANSPARENT)
                    .rounding(Rounding::same(8.0))
                    .min_size(Vec2::new(ui.available_width(), 30.0)),
            )
            .clicked()
        {
            state.show_settings_modal = true;
        }

        if ui
            .add(
                egui::Button::new(egui::RichText::new("Help & Documentation").size(12.5))
                    .fill(Color32::TRANSPARENT)
                    .rounding(Rounding::same(8.0))
                    .min_size(Vec2::new(ui.available_width(), 30.0)),
            )
            .clicked()
        {
            state.show_help_modal = true;
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("v0.1.0 (Clean-Room Engine)")
                    .size(9.5)
                    .color(Theme::TEXT_MUTED),
            );
            ui.label(
                egui::RichText::new("SP621E / SP6xxE Native")
                    .size(9.5)
                    .color(Theme::TEXT_MUTED),
            );
        });
    });
}
