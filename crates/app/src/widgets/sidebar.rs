//! # Desktop Left Navigation Rail (Sidebar)
//!
//! Provides the persistent desktop sidebar for device selection,
//! real-time BLE connection status, and mode switching matching Windows 11 Fluent Design.
//! Supports responsive compact (<1100px) icon-only mode and full (240px) mode.

use crate::state::{AppScreen, AppState, DashboardMode};
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::{render_safe_label, reserve_bold_width, TypeScale};
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

/// Renders the left-hand navigation sidebar with responsive compact rail support.
pub fn render_sidebar(ui: &mut egui::Ui, state: &mut AppState, is_compact: bool) {
    let tokens = ThemeTokens::default_dark();
    let is_hub = matches!(state.current_screen, AppScreen::DeviceHub);

    if is_compact {
        // ====================================================================
        // COMPACT RAIL (56px Width, Icon-Only with Tooltips)
        // ====================================================================
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);

            // 1. App Icon
            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
            crate::widgets::paint_pyxillum_logo(&ui.painter_at(icon_rect), icon_rect);

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            // 2. Compact Device Status Dot
            let (dot_rect, dot_resp) =
                ui.allocate_exact_size(Vec2::splat(32.0), egui::Sense::click());
            let dot_resp = dot_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            let dp = ui.painter_at(dot_rect);

            let dot_hovered = dot_resp.hovered();
            if dot_hovered {
                dp.rect_filled(
                    dot_rect,
                    tokens.radius.control_rounding(),
                    tokens.surfaces.raised_hover,
                );
            }

            let status_dot_color = if state.is_connected {
                tokens.semantic.success
            } else if state.is_scanning {
                tokens.semantic.warning
            } else {
                tokens.text.disabled
            };
            dp.circle_filled(dot_rect.center(), 4.0, status_dot_color);

            let tooltip_text = if state.is_connected {
                format!("Connected: {}", state.active_device_id)
            } else if state.is_scanning {
                "Scanning for BLE controllers...".to_string()
            } else {
                "No Device Connected - Click to Pair".to_string()
            };
            let dot_resp = dot_resp.on_hover_text(tooltip_text);

            if dot_resp.clicked() {
                state.current_screen = AppScreen::DeviceHub;
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // 3. Compact Studio Modes
            let modes = [
                (Icon::Effects, "Dynamic FX (Ctrl+1)", DashboardMode::Dynamic),
                (Icon::Palette, "Solid Colour (Ctrl+2)", DashboardMode::Solid),
                (
                    Icon::MusicBars,
                    "Music Rhythm (Ctrl+3)",
                    DashboardMode::Music,
                ),
                (Icon::DiySliders, "DIY Custom (Ctrl+4)", DashboardMode::Diy),
            ];

            for (icon, label, mode) in modes {
                let is_selected = !is_hub && state.dashboard_mode == mode;
                let (btn_rect, btn_resp) =
                    ui.allocate_exact_size(Vec2::splat(38.0), egui::Sense::click());
                let btn_resp = btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                let bp = ui.painter_at(btn_rect);

                let is_hovered = btn_resp.hovered();
                let bg = if is_selected {
                    tokens.surfaces.card
                } else if is_hovered {
                    tokens.surfaces.raised_hover
                } else {
                    Color32::TRANSPARENT
                };
                bp.rect_filled(btn_rect, tokens.radius.control_rounding(), bg);

                if is_selected {
                    // Left Accent Pill
                    let pill = Rect::from_min_size(
                        Pos2::new(btn_rect.min.x + 1.0, btn_rect.center().y - 8.0),
                        Vec2::new(3.0, 16.0),
                    );
                    bp.rect_filled(pill, Rounding::same(1.5), tokens.accent.base);
                }

                let icon_color = if is_selected {
                    tokens.accent.text_light
                } else if is_hovered {
                    tokens.text.primary
                } else {
                    tokens.text.secondary
                };
                icon.paint(
                    &bp,
                    Rect::from_center_size(btn_rect.center(), Vec2::splat(18.0)),
                    icon_color,
                );

                let btn_resp = btn_resp.on_hover_text(label);

                if btn_resp.clicked() {
                    state.dashboard_mode = mode;
                    if is_hub {
                        state.current_screen = AppScreen::Dashboard(state.active_device_id.clone());
                    }
                }
                ui.add_space(4.0);
            }

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // 4. Compact Devices & Pairing
            let (hub_rect, hub_resp) =
                ui.allocate_exact_size(Vec2::splat(38.0), egui::Sense::click());
            let hub_resp = hub_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            let hp = ui.painter_at(hub_rect);

            let hub_hovered = hub_resp.hovered();
            let hub_bg = if is_hub {
                tokens.surfaces.card
            } else if hub_hovered {
                tokens.surfaces.raised_hover
            } else {
                Color32::TRANSPARENT
            };
            hp.rect_filled(hub_rect, tokens.radius.control_rounding(), hub_bg);

            if is_hub {
                let pill = Rect::from_min_size(
                    Pos2::new(hub_rect.min.x + 1.0, hub_rect.center().y - 8.0),
                    Vec2::new(3.0, 16.0),
                );
                hp.rect_filled(pill, Rounding::same(1.5), tokens.accent.base);
            }

            let hub_icon_color = if is_hub {
                tokens.accent.text_light
            } else if hub_hovered {
                tokens.text.primary
            } else {
                tokens.text.secondary
            };
            Icon::Devices.paint(
                &hp,
                Rect::from_center_size(hub_rect.center(), Vec2::splat(18.0)),
                hub_icon_color,
            );
            let hub_resp = hub_resp.on_hover_text("Devices & Pairing (Ctrl+5)");

            if hub_resp.clicked() {
                state.current_screen = AppScreen::DeviceHub;
            }

            // Bottom Settings
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                let (st_rect, st_resp) =
                    ui.allocate_exact_size(Vec2::splat(38.0), egui::Sense::click());
                let st_resp = st_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                let stp = ui.painter_at(st_rect);

                if st_resp.hovered() {
                    stp.rect_filled(
                        st_rect,
                        tokens.radius.control_rounding(),
                        tokens.surfaces.raised_hover,
                    );
                }
                Icon::Settings.paint(
                    &stp,
                    Rect::from_center_size(st_rect.center(), Vec2::splat(18.0)),
                    tokens.text.secondary,
                );
                let st_resp = st_resp.on_hover_text("Settings & Hardware (Ctrl+,)");

                if st_resp.clicked() {
                    state.show_settings_modal = true;
                }
            });
        });
        return;
    }

    // ========================================================================
    // REGULAR SIDEBAR (240px Width)
    // ========================================================================
    ui.vertical(|ui| {
        // 1. App Header / Branding
        ui.horizontal(|ui| {
            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
            crate::widgets::paint_pyxillum_logo(&ui.painter_at(icon_rect), icon_rect);

            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("PYXILLUM")
                        .font(TypeScale::body_strong())
                        .color(tokens.text.primary),
                );
                ui.label(
                    egui::RichText::new("Pixel LED Studio")
                        .font(TypeScale::caption())
                        .color(tokens.text.tertiary),
                );
            });
        });

        ui.add_space(14.0);
        ui.separator();
        ui.add_space(10.0);

        // 2. Hardware Controller Status Card
        let dev_card_bg = if is_hub {
            tokens.surfaces.raised_hover
        } else {
            tokens.surfaces.well
        };

        egui::Frame::none()
            .fill(dev_card_bg)
            .rounding(tokens.radius.control_rounding())
            .stroke(Stroke::new(
                1.0_f32,
                if is_hub {
                    tokens.borders.focus
                } else {
                    tokens.borders.subtle
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
                        egui::Button::new(
                            egui::RichText::new(dev_name)
                                .font(TypeScale::body_strong())
                                .color(if state.is_connected {
                                    tokens.text.primary
                                } else {
                                    tokens.text.secondary
                                }),
                        )
                        .fill(Color32::TRANSPARENT),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    state.current_screen = AppScreen::DeviceHub;
                }

                ui.horizontal(|ui| {
                    let dot_col = if state.is_connected {
                        tokens.semantic.success
                    } else if state.is_scanning {
                        tokens.semantic.warning
                    } else {
                        tokens.text.disabled
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
                            .font(TypeScale::caption())
                            .color(tokens.text.tertiary),
                    );
                });
            });

        ui.add_space(14.0);

        // 3. Navigation Modes Section Header
        ui.label(
            egui::RichText::new("STUDIO MODES")
                .font(TypeScale::caption())
                .color(tokens.text.tertiary),
        );
        ui.add_space(4.0);

        let nav_items = [
            (
                Icon::Effects,
                "Dynamic FX",
                "146 Presets",
                DashboardMode::Dynamic,
            ),
            (
                Icon::Palette,
                "Solid Colour",
                "Custom & Dynamic",
                DashboardMode::Solid,
            ),
            (
                Icon::MusicBars,
                "Music Rhythm",
                "18 Audio Styles",
                DashboardMode::Music,
            ),
            (
                Icon::DiySliders,
                "DIY Custom",
                "Node Sequencer",
                DashboardMode::Diy,
            ),
        ];

        for (icon, title, sub, mode) in nav_items {
            let is_selected = !is_hub && state.dashboard_mode == mode;
            let item_h = 42.0;
            let (item_rect, resp) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), item_h),
                egui::Sense::click(),
            );
            let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);

            let is_hovered = resp.hovered();
            let p = ui.painter_at(item_rect);

            let bg = if is_selected {
                tokens.surfaces.card
            } else if is_hovered {
                tokens.surfaces.raised_hover
            } else {
                Color32::TRANSPARENT
            };

            p.rect_filled(item_rect, tokens.radius.control_rounding(), bg);

            if is_selected {
                // Windows 11 Fluent Active Indicator (Left Accent Pill)
                let pill_rect = Rect::from_min_size(
                    Pos2::new(item_rect.min.x + 2.0, item_rect.center().y - 10.0),
                    Vec2::new(3.0, 20.0),
                );
                p.rect_filled(pill_rect, Rounding::same(1.5), tokens.accent.base);
            }

            // Resolution-Independent Vector Icon
            let icon_col = if is_selected {
                tokens.accent.text_light
            } else if is_hovered {
                tokens.text.primary
            } else {
                tokens.text.secondary
            };
            let icon_rect = Rect::from_center_size(
                Pos2::new(item_rect.min.x + 20.0, item_rect.center().y),
                Vec2::splat(18.0),
            );
            icon.paint(&p, icon_rect, icon_col);

            // Mode Label with Reserved Bold Width (no layout shift)
            let text_x = item_rect.min.x + 36.0;
            let _ = reserve_bold_width(ui, title, &TypeScale::body(), &TypeScale::body_strong());
            p.text(
                Pos2::new(text_x, item_rect.min.y + 12.0),
                egui::Align2::LEFT_CENTER,
                title,
                if is_selected {
                    TypeScale::body_strong()
                } else {
                    TypeScale::body()
                },
                if is_selected {
                    tokens.text.primary
                } else {
                    tokens.text.secondary
                },
            );

            p.text(
                Pos2::new(text_x, item_rect.min.y + 27.0),
                egui::Align2::LEFT_CENTER,
                sub,
                TypeScale::caption(),
                tokens.text.tertiary,
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
        let hub_resp = hub_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
        let hp = ui.painter_at(hub_rect);
        let hub_hovered = hub_resp.hovered();

        let hub_bg = if is_hub {
            tokens.surfaces.card
        } else if hub_hovered {
            tokens.surfaces.raised_hover
        } else {
            Color32::TRANSPARENT
        };

        hp.rect_filled(hub_rect, tokens.radius.control_rounding(), hub_bg);

        if is_hub {
            let pill_rect = Rect::from_min_size(
                Pos2::new(hub_rect.min.x + 2.0, hub_rect.center().y - 8.0),
                Vec2::new(3.0, 16.0),
            );
            hp.rect_filled(pill_rect, Rounding::same(1.5), tokens.accent.base);
        }

        let hub_icon_col = if is_hub {
            tokens.accent.text_light
        } else if hub_hovered {
            tokens.text.primary
        } else {
            tokens.text.secondary
        };
        let hub_icon_rect = Rect::from_center_size(
            Pos2::new(hub_rect.min.x + 20.0, hub_rect.center().y),
            Vec2::splat(18.0),
        );
        Icon::Devices.paint(&hp, hub_icon_rect, hub_icon_col);

        let _ = reserve_bold_width(
            ui,
            "Devices & Pairing",
            &TypeScale::body(),
            &TypeScale::body_strong(),
        );
        hp.text(
            Pos2::new(hub_rect.min.x + 36.0, hub_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Devices & Pairing",
            if is_hub {
                TypeScale::body_strong()
            } else {
                TypeScale::body()
            },
            if is_hub {
                tokens.text.primary
            } else {
                tokens.text.secondary
            },
        );

        if hub_resp.clicked() {
            state.current_screen = AppScreen::DeviceHub;
        }

        // 5. Bottom Controls & Hardware Settings
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.add_space(6.0);

            // Diagnostics / Status Line with safe tooltip on hover
            let caption_font = TypeScale::caption();
            render_safe_label(
                ui,
                &state.status_message,
                ui.available_width() - 8.0,
                &caption_font,
                tokens.text.tertiary,
            );

            ui.add_space(6.0);

            // Hardware Settings Button with Vector Gear Icon
            let (btn_rect, btn_resp) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), egui::Sense::click());
            let btn_resp = btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            let bp = ui.painter_at(btn_rect);
            let btn_hovered = btn_resp.hovered();
            let btn_bg = if btn_hovered {
                tokens.surfaces.raised_hover
            } else {
                tokens.surfaces.well
            };
            bp.rect_filled(btn_rect, tokens.radius.control_rounding(), btn_bg);
            bp.rect_stroke(
                btn_rect,
                tokens.radius.control_rounding(),
                Stroke::new(
                    1.0_f32,
                    if btn_hovered {
                        tokens.borders.hover
                    } else {
                        tokens.borders.subtle
                    },
                ),
            );

            let gear_col = if btn_hovered {
                tokens.text.primary
            } else {
                tokens.text.secondary
            };
            let gear_rect = Rect::from_center_size(
                Pos2::new(btn_rect.min.x + 18.0, btn_rect.center().y),
                Vec2::splat(16.0),
            );
            Icon::Settings.paint(&bp, gear_rect, gear_col);

            bp.text(
                Pos2::new(btn_rect.min.x + 34.0, btn_rect.center().y),
                egui::Align2::LEFT_CENTER,
                "Settings & Hardware",
                TypeScale::caption(),
                if btn_hovered {
                    tokens.text.primary
                } else {
                    tokens.text.secondary
                },
            );

            if btn_resp.clicked() {
                state.show_settings_modal = true;
            }

            ui.add_space(8.0);
            ui.separator();
        });
    });
}
