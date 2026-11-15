//! # Desktop Left Navigation Rail (Sidebar)
//!
//! Provides the persistent desktop sidebar for device selection,
//! real-time BLE connection status, and mode switching matching Windows 11 Fluent Design.

use crate::state::{AppScreen, AppState, DashboardMode};
use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

/// Renders the left-hand navigation sidebar.
pub fn render_sidebar(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // 1. App Header / Branding
        ui.horizontal(|ui| {
            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
            let ip = ui.painter_at(icon_rect);
            ip.rect_filled(icon_rect, Rounding::same(6.0), Theme::ACCENT_BLUE);
            ip.circle_filled(icon_rect.center(), 5.0, Color32::WHITE);

            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("PYXILLUM")
                        .strong()
                        .size(14.0)
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
        ui.add_space(10.0);

        // 2. Hardware Controller Status Card
        let is_hub = matches!(state.current_screen, AppScreen::DeviceHub);

        let dev_card_bg = if is_hub {
            Color32::from_rgb(38, 38, 38)
        } else {
            Theme::WELL_BG
        };

        egui::Frame::none()
            .fill(dev_card_bg)
            .rounding(Rounding::same(6.0))
            .stroke(Stroke::new(
                1.0_f32,
                if is_hub {
                    Theme::BORDER_FOCUS
                } else {
                    Theme::BORDER_LIGHT
                },
            ))
            .inner_margin(egui::Margin::symmetric(10.0, 8.0))
            .show(ui, |ui| {
                let dev_name = if state.is_connected && !state.active_device_id.is_empty() {
                    let short_id = if state.active_device_id.len() >= 8 {
                        &state.active_device_id[..8]
                    } else {
                        &state.active_device_id
                    };
                    format!("SP621E ({})", short_id)
                } else {
                    "No Device Connected".to_string()
                };

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(dev_name).strong().size(11.5).color(
                            if state.is_connected {
                                Theme::TEXT_PRIMARY
                            } else {
                                Theme::TEXT_SECONDARY
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

                    let (d_rect, _) =
                        ui.allocate_exact_size(Vec2::splat(8.0), egui::Sense::hover());
                    ui.painter().circle_filled(d_rect.center(), 3.0, dot_col);

                    let status_txt = if state.is_connected {
                        "Connected (BLE)"
                    } else if state.is_scanning {
                        "Scanning..."
                    } else {
                        "Click to Pair"
                    };

                    ui.label(
                        egui::RichText::new(status_txt)
                            .size(10.0)
                            .color(Theme::TEXT_MUTED),
                    );
                });
            });

        ui.add_space(14.0);

        // 3. Navigation Modes Section
        ui.label(
            egui::RichText::new("STUDIO MODES")
                .size(10.0)
                .strong()
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(4.0);

        let nav_items = [
            ("✦", "Dynamic FX", "146 Presets", DashboardMode::Dynamic),
            (
                "◉",
                "Solid Colour",
                "Custom & Dynamic",
                DashboardMode::Solid,
            ),
            ("♫", "Music Rhythm", "18 Audio Styles", DashboardMode::Music),
            ("𝌆", "DIY Custom", "14-Node Sequencer", DashboardMode::Diy),
        ];

        for (icon, title, sub, mode) in nav_items {
            let is_selected = !is_hub && state.dashboard_mode == mode;
            let item_h = 42.0;
            let (item_rect, resp) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), item_h),
                egui::Sense::click(),
            );

            let is_hovered = resp.hovered();
            let p = ui.painter_at(item_rect);

            let bg = if is_selected {
                Theme::CARD_BG
            } else if is_hovered {
                Color32::from_rgb(34, 34, 34)
            } else {
                Color32::TRANSPARENT
            };

            p.rect_filled(item_rect, Rounding::same(6.0), bg);

            if is_selected {
                // Windows 11 Fluent Active Indicator (Left Accent Pill)
                let pill_rect = Rect::from_min_size(
                    Pos2::new(item_rect.min.x + 2.0, item_rect.center().y - 10.0),
                    Vec2::new(3.0, 20.0),
                );
                p.rect_filled(pill_rect, Rounding::same(1.5), Theme::ACCENT_BLUE);
            }

            // Mode Icon
            let icon_col = if is_selected {
                Theme::ACCENT_CYAN
            } else if is_hovered {
                Theme::TEXT_PRIMARY
            } else {
                Theme::TEXT_MUTED
            };
            p.text(
                Pos2::new(item_rect.min.x + 14.0, item_rect.center().y),
                egui::Align2::LEFT_CENTER,
                icon,
                egui::FontId::proportional(14.0),
                icon_col,
            );

            // Mode Label & Sub-label
            let text_x = item_rect.min.x + 36.0;
            p.text(
                Pos2::new(text_x, item_rect.min.y + 12.0),
                egui::Align2::LEFT_CENTER,
                title,
                egui::FontId::proportional(12.5),
                if is_selected {
                    Theme::TEXT_PRIMARY
                } else {
                    Theme::TEXT_SECONDARY
                },
            );

            p.text(
                Pos2::new(text_x, item_rect.min.y + 27.0),
                egui::Align2::LEFT_CENTER,
                sub,
                egui::FontId::proportional(10.0),
                Theme::TEXT_MUTED,
            );

            if resp.clicked() {
                state.dashboard_mode = mode;
                if is_hub {
                    state.current_screen = AppScreen::Dashboard(state.active_device_id.clone());
                }
            }

            ui.add_space(2.0);
        }

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        // 4. Device Hub Navigation Item
        let hub_h = 38.0;
        let (hub_rect, hub_resp) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), hub_h), egui::Sense::click());
        let hp = ui.painter_at(hub_rect);
        let hub_hovered = hub_resp.hovered();

        let hub_bg = if is_hub {
            Theme::CARD_BG
        } else if hub_hovered {
            Color32::from_rgb(34, 34, 34)
        } else {
            Color32::TRANSPARENT
        };

        hp.rect_filled(hub_rect, Rounding::same(6.0), hub_bg);

        if is_hub {
            let pill_rect = Rect::from_min_size(
                Pos2::new(hub_rect.min.x + 2.0, hub_rect.center().y - 8.0),
                Vec2::new(3.0, 16.0),
            );
            hp.rect_filled(pill_rect, Rounding::same(1.5), Theme::ACCENT_BLUE);
        }

        let hub_icon_col = if is_hub {
            Theme::ACCENT_CYAN
        } else if hub_hovered {
            Theme::TEXT_PRIMARY
        } else {
            Theme::TEXT_MUTED
        };
        hp.text(
            Pos2::new(hub_rect.min.x + 14.0, hub_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "📶",
            egui::FontId::proportional(13.0),
            hub_icon_col,
        );

        hp.text(
            Pos2::new(hub_rect.min.x + 36.0, hub_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Devices & Pairing",
            egui::FontId::proportional(12.5),
            if is_hub {
                Theme::TEXT_PRIMARY
            } else {
                Theme::TEXT_SECONDARY
            },
        );

        if hub_resp.clicked() {
            state.current_screen = AppScreen::DeviceHub;
        }

        // 5. Spacer & Bottom Controls
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.add_space(6.0);

            // Diagnostics / Status Line
            let short_status = if state.status_message.len() > 32 {
                format!("{}...", &state.status_message[..29])
            } else {
                state.status_message.clone()
            };
            ui.label(
                egui::RichText::new(short_status)
                    .size(9.5)
                    .color(Theme::TEXT_MUTED),
            );

            ui.add_space(6.0);

            // Hardware Settings Button (Safety Gated)
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("⚙ Settings & Hardware")
                            .size(11.5)
                            .color(Theme::TEXT_SECONDARY),
                    )
                    .fill(Color32::from_rgb(34, 34, 34))
                    .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                    .rounding(Rounding::same(6.0))
                    .min_size(Vec2::new(ui.available_width(), 32.0)),
                )
                .clicked()
            {
                state.show_settings_modal = true;
            }

            ui.add_space(8.0);
            ui.separator();
        });
    });
}
