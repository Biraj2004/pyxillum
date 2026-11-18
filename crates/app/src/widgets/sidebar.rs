//! # Desktop Left Navigation Rail (Sidebar)
//!
//! Provides the persistent desktop sidebar for device selection,
//! real-time BLE connection status, and mode switching matching Windows 11 Fluent Design.
//! Supports responsive compact (<1100px) icon-only mode and full (240px) mode.

use crate::state::{AppScreen, AppState, DashboardMode};
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::{reserve_bold_width, TypeScale};
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

            // 2. Compact Device Status Icon
            let (dot_rect, dot_resp) =
                ui.allocate_exact_size(Vec2::splat(36.0), egui::Sense::click());
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

            let conn_state = state.connection_state();
            let (dot_col, icon) = match conn_state {
                crate::state::ConnectionState::Connected => {
                    (tokens.semantic.success, Icon::Bluetooth)
                }
                crate::state::ConnectionState::Scanning => (tokens.semantic.warning, Icon::Devices),
                crate::state::ConnectionState::Connecting => {
                    (tokens.semantic.warning, Icon::Bluetooth)
                }
                crate::state::ConnectionState::Disconnected => {
                    (tokens.text.disabled, Icon::Devices)
                }
            };

            icon.paint(
                &dp,
                Rect::from_center_size(dot_rect.center(), Vec2::splat(18.0)),
                dot_col,
            );

            let tooltip_text = match conn_state {
                crate::state::ConnectionState::Connected => {
                    format!("Connected: {}", state.active_device_id)
                }
                crate::state::ConnectionState::Scanning => format!(
                    "Scanning... ({})",
                    crate::state::format_controller_count(state.discovered_count)
                ),
                crate::state::ConnectionState::Connecting => "Connecting...".to_string(),
                crate::state::ConnectionState::Disconnected => format!(
                    "Not connected ({}) - Click to pair",
                    crate::state::format_controller_count(state.discovered_count)
                ),
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

        // 2. Persistent Single Connection State Chip (56px Container, No nested buttons, Blue border only on focus)
        let container_h = 56.0_f32;
        let (card_rect, card_resp) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), container_h),
            egui::Sense::click(),
        );
        let card_resp = card_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
        let cp = ui.painter_at(card_rect);

        let is_hovered = card_resp.hovered();
        let has_focus = card_resp.has_focus();

        let bg = if is_hovered {
            tokens.surfaces.raised_hover
        } else {
            tokens.surfaces.well
        };
        cp.rect_filled(card_rect, tokens.radius.control_rounding(), bg);

        // Blue border ONLY on focus
        let border_color = if has_focus {
            tokens.borders.focus
        } else {
            tokens.borders.subtle
        };
        let border_width = if has_focus { 2.0_f32 } else { 1.0_f32 };
        cp.rect_stroke(
            card_rect,
            tokens.radius.control_rounding(),
            Stroke::new(border_width, border_color),
        );

        let conn_state = state.connection_state();
        let (icon_col, icon, primary_text, subline_text) = match conn_state {
            crate::state::ConnectionState::Connected => {
                let short_id = if state.active_device_id.len() >= 8 {
                    &state.active_device_id[..8]
                } else if !state.active_device_id.is_empty() {
                    &state.active_device_id
                } else {
                    "SP621E"
                };
                (
                    tokens.semantic.success,
                    Icon::Bluetooth,
                    conn_state.label().to_string(),
                    format!("SP621E ({})", short_id),
                )
            }
            crate::state::ConnectionState::Scanning => (
                tokens.semantic.warning,
                Icon::Devices,
                conn_state.label().to_string(),
                crate::state::format_controller_count(state.discovered_count),
            ),
            crate::state::ConnectionState::Connecting => (
                tokens.semantic.warning,
                Icon::Bluetooth,
                conn_state.label().to_string(),
                "Establishing link".to_string(),
            ),
            crate::state::ConnectionState::Disconnected => (
                tokens.text.disabled,
                Icon::Devices,
                conn_state.label().to_string(),
                if state.discovered_count > 0 {
                    crate::state::format_controller_count(state.discovered_count)
                } else {
                    "1 controller found".to_string()
                },
            ),
        };

        // Render Icon
        let icon_box = Rect::from_center_size(
            Pos2::new(card_rect.min.x + 20.0, card_rect.center().y),
            Vec2::splat(18.0),
        );
        icon.paint(&cp, icon_box, icon_col);

        // Primary Title & Sub-line
        let text_x = card_rect.min.x + 36.0;
        cp.text(
            Pos2::new(text_x, card_rect.center().y - 8.0),
            egui::Align2::LEFT_CENTER,
            primary_text,
            TypeScale::body_strong(),
            tokens.text.primary,
        );
        cp.text(
            Pos2::new(text_x, card_rect.center().y + 8.0),
            egui::Align2::LEFT_CENTER,
            subline_text,
            TypeScale::caption(),
            tokens.text.secondary,
        );

        if card_resp.clicked() {
            state.current_screen = AppScreen::DeviceHub;
        }

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

            // Diagnostics / Status Line: exactly one line with full text in tooltip, no fragments
            if !state.status_message.is_empty() {
                let clean_msg = state
                    .status_message
                    .split('.')
                    .next()
                    .unwrap_or(&state.status_message)
                    .trim();
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(clean_msg)
                            .font(TypeScale::caption())
                            .color(tokens.text.tertiary),
                    )
                    .truncate(),
                )
                .on_hover_text(&state.status_message);
            }

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
