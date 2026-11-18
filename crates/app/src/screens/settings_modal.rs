//! # Controller Settings Modal Sheet
//!
//! Provides hardware-level EEPROM configuration for pixel count and colour channel sequence,
//! guarded behind an explicit safety verification gate.

use crate::state::AppState;
use crate::ui::components::button::FluentButton;
use crate::ui::components::card::Card;
use crate::ui::components::segmented::{segmented_control, SegmentItem};
use crate::ui::components::slider::fluent_slider;
use crate::ui::icons::{header_tile_icon, icon_button_32, Icon};
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Rounding, Vec2};
use pyxillum_protocol::ChipOrder;

pub fn render_settings_modal(ctx: &egui::Context, state: &mut AppState) -> bool {
    let tokens = ThemeTokens::default_dark();
    let mut commit_changes = false;

    // 1. Fullscreen dark backdrop scrim
    let screen_rect = ctx.screen_rect();
    let scrim_painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Middle,
        egui::Id::new("modal_scrim"),
    ));
    scrim_painter.rect_filled(screen_rect, Rounding::ZERO, Color32::from_black_alpha(150));

    // 2. Centered Windows 11 Fluent Content Dialog
    egui::Window::new("Controller Hardware Settings Window")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .fixed_size(Vec2::new(540.0, 550.0))
        .frame(
            egui::Frame::none()
                .fill(tokens.surfaces.dialog)
                .rounding(tokens.radius.card_rounding())
                .stroke(tokens.card_stroke())
                .inner_margin(egui::Margin::symmetric(24.0, 20.0)),
        )
        .show(ctx, |ui| {
            ui.set_width(ui.available_width());

            // Handle Esc key to close dialog
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                state.show_settings_modal = false;
            }

            // Header Row: Hardware Icon Tile + Title & Subtitle + 32px Close Button
            ui.horizontal(|ui| {
                header_tile_icon(ui, Icon::HardwareChip, Color32::WHITE, tokens.accent.base);

                ui.add_space(10.0);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Controller Hardware Settings")
                            .font(TypeScale::title())
                            .color(tokens.text.primary),
                    );
                    ui.label(
                        egui::RichText::new(
                            "Configure physical LED strip length and chip sequence in EEPROM",
                        )
                        .font(TypeScale::caption())
                        .color(tokens.text.tertiary),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if icon_button_32(ui, Icon::Close, "Close (Esc)", false).clicked() {
                        state.show_settings_modal = false;
                    }
                });
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(12.0);

            // =================================================================
            // Section 1: Pixel Strip Length
            // =================================================================
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Pixel Strip Length")
                        .font(TypeScale::body_strong())
                        .color(tokens.text.primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{} Addressable Pixels", state.ui_length))
                            .font(TypeScale::body_strong())
                            .color(tokens.accent.text_light),
                    );
                });
            });
            ui.add_space(8.0);

            // Full-width slider (30-255 pixels)
            fluent_slider(ui, "", &mut state.ui_length, 30, 255, "Nodes");
            ui.add_space(8.0);

            // Quick preset pills via Segmented Control
            let presets = [30, 60, 90, 120, 144, 240, 255];
            let preset_items: Vec<SegmentItem> = presets
                .iter()
                .map(|p| SegmentItem::new(Box::leak(p.to_string().into_boxed_str())))
                .collect();

            let curr_preset_idx = presets
                .iter()
                .position(|&p| p == state.ui_length)
                .unwrap_or(usize::MAX);

            let (new_p_idx, p_resp) = segmented_control(
                ui,
                "settings_strip_length_presets_segmented",
                &preset_items,
                if curr_preset_idx == usize::MAX { 0 } else { curr_preset_idx },
            );
            if p_resp.clicked() {
                if let Some(&p_val) = presets.get(new_p_idx) {
                    state.ui_length = p_val;
                }
            }

            ui.add_space(16.0);

            // =================================================================
            // Section 2: Colour Sequence (Chip Order)
            // =================================================================
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Colour Sequence (Chip Order)")
                        .font(TypeScale::body_strong())
                        .color(tokens.text.primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let hint = match state.ui_chip_order {
                        ChipOrder::Grb => "WS2812B / WS2811 (Standard)",
                        ChipOrder::Rgb => "Standard RGB LEDs",
                        ChipOrder::Bgr => "APA102 / SK9822 Variant",
                        ChipOrder::Rbg => "RBG LED Modules",
                        ChipOrder::Gbr => "GBR Custom Modules",
                        ChipOrder::Brg => "BRG Custom Modules",
                    };
                    ui.label(
                        egui::RichText::new(hint)
                            .font(TypeScale::caption())
                            .color(tokens.text.tertiary),
                    );
                });
            });
            ui.add_space(8.0);

            let chip_orders = [
                (ChipOrder::Grb, "GRB"),
                (ChipOrder::Rgb, "RGB"),
                (ChipOrder::Bgr, "BGR"),
                (ChipOrder::Rbg, "RBG"),
                (ChipOrder::Gbr, "GBR"),
                (ChipOrder::Brg, "BRG"),
            ];
            let order_items: Vec<SegmentItem> =
                chip_orders.iter().map(|(_, label)| SegmentItem::new(label)).collect();

            let curr_order_idx = chip_orders
                .iter()
                .position(|(ord, _)| *ord == state.ui_chip_order)
                .unwrap_or(0);

            let (new_order_idx, ord_resp) = segmented_control(
                ui,
                "settings_chip_order_segmented",
                &order_items,
                curr_order_idx,
            );
            if ord_resp.clicked() {
                if let Some(&(ord, _)) = chip_orders.get(new_order_idx) {
                    state.ui_chip_order = ord;
                }
            }

            ui.add_space(16.0);

            // =================================================================
            // Section 3: Hardware Safety Gate Card
            // =================================================================
            Card::base()
                .rounding(tokens.radius.control_rounding())
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let (icon_rect, _) =
                            ui.allocate_exact_size(Vec2::splat(22.0), egui::Sense::hover());
                        Icon::Warning.paint(ui.painter(), icon_rect, tokens.semantic.danger);

                        ui.add_space(8.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Hardware Safety Gate (EEPROM Mutating)")
                                    .font(TypeScale::body_strong())
                                    .color(tokens.semantic.danger),
                            );
                            ui.label(
                                egui::RichText::new(
                                    "Applying writes directly to non-volatile registers. Confirm settings match physical LED strip.",
                                )
                                .font(TypeScale::caption())
                                .color(tokens.text.tertiary),
                            );
                        });
                    });

                    ui.add_space(10.0);
                    let is_confirmed = state.ui_confirm_safety;
                    ui.checkbox(
                        &mut state.ui_confirm_safety,
                        egui::RichText::new("I understand this mutates hardware registers")
                            .font(TypeScale::body())
                            .color(if is_confirmed {
                                tokens.text.primary
                            } else {
                                tokens.text.secondary
                            }),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(12.0);

            // =================================================================
            // Section 4: Desktop Window Layout Preferences
            // =================================================================
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Desktop Window Geometry")
                            .font(TypeScale::body_strong())
                            .color(tokens.text.primary),
                    );
                    ui.label(
                        egui::RichText::new("Reset saved window placement to 80% centred default")
                            .font(TypeScale::caption())
                            .color(tokens.text.tertiary),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if FluentButton::secondary("Reset Window")
                        .icon(Icon::Reverse)
                        .show(ui)
                        .clicked()
                    {
                        crate::window_geom::delete_saved_state();
                        let cfg = crate::window_geom::load_or_create_config();
                        let (work, scale, _) =
                            crate::window_geom::get_target_monitor_work_and_scale(cfg.screen);
                        let target = crate::window_geom::compute_rect(work, scale, &cfg, None);
                        let (nc_w, nc_h) =
                            crate::platform::windows::get_non_client_margins_logical(scale);
                        let inner_w = (target.width - nc_w).max(1000.0);
                        let inner_h = (target.height - nc_h).max(650.0);

                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(false));
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::InnerSize(
                            egui::Vec2::new(inner_w as f32, inner_h as f32),
                        ));
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                            egui::Pos2::new(target.x as f32, target.y as f32),
                        ));

                        crate::platform::windows::set_window_outer_rect(
                            "Pyxillum - Pixel LED Studio",
                            target,
                            scale,
                        );
                    }
                });
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(14.0);

            // =================================================================
            // Section 5: Modal Footer Action Row
            // =================================================================
            ui.horizontal(|ui| {
                if state.ui_confirm_safety {
                    ui.label(
                        egui::RichText::new("Ready to commit EEPROM")
                            .font(TypeScale::caption())
                            .color(tokens.semantic.success),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("Safety confirmation required")
                            .font(TypeScale::caption())
                            .color(tokens.text.disabled),
                    );
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let can_apply = state.ui_confirm_safety;

                    let apply_resp = FluentButton::primary("Apply Configuration")
                        .icon(Icon::HardwareChip)
                        .enabled(can_apply)
                        .min_width(150.0)
                        .show(ui);

                    if apply_resp.clicked() {
                        if can_apply {
                            commit_changes = true;
                            state.show_settings_modal = false;
                        } else {
                            crate::ui::components::dialog::trigger_dialog_shake(ui.ctx());
                        }
                    }

                    ui.add_space(10.0);

                    if FluentButton::secondary("Cancel")
                        .min_width(80.0)
                        .show(ui)
                        .clicked()
                    {
                        state.show_settings_modal = false;
                    }
                });
            });
        });

    commit_changes
}
