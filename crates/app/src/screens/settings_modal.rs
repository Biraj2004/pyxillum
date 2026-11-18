//! # Controller Settings Modal Sheet
//!
//! Provides hardware-level EEPROM configuration for pixel count and colour channel sequence,
//! guarded behind an explicit safety verification gate with authentic Windows 11 Fluent
//! styling, 45% alpha backdrop scrim, and decelerating enter animation.

use crate::state::AppState;
use crate::ui::components::button::FluentButton;
use crate::ui::components::checkbox::fluent_checkbox;
use crate::ui::components::segmented::{segmented_control, SegmentItem};
use crate::ui::components::slider::fluent_slider;
use crate::ui::icons::{header_tile_icon, icon_button_32, Icon};
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rounding, Stroke, Vec2};
use pyxillum_protocol::ChipOrder;

/// Pure animation helper that computes modal enter animation progress.
///
/// Duration: 180 ms.
/// Scale: 0.97 to 1.0.
/// Opacity: 0.0 to 1.0.
/// Decelerate cubic ease-out: `f(t) = 1 - (1 - t)^3`.
///
/// Guaranteed to land on exactly 1.0 when `elapsed >= duration`, and requests one final repaint.
pub fn compute_dialog_enter_animation(
    elapsed: f32,
    duration: f32,
    already_finalized: bool,
) -> (f32, f32, bool, bool) {
    if duration <= 0.0 || elapsed >= duration {
        let needs_final = !already_finalized;
        (1.0, 1.0, false, needs_final)
    } else if elapsed <= 0.0 {
        (0.97, 0.0, true, false)
    } else {
        let linear_t = (elapsed / duration).clamp(0.0, 1.0);
        let inv = 1.0 - linear_t;
        let progress = 1.0 - (inv * inv * inv);

        let scale = 0.97 + (1.0 - 0.97) * progress;
        let opacity = progress;
        (scale, opacity, true, false)
    }
}

pub fn render_settings_modal(ctx: &egui::Context, state: &mut AppState) -> bool {
    let tokens = ThemeTokens::default_dark();
    let mut commit_changes = false;

    // Check Windows accessibility setting for animations
    let client_anim = crate::platform::windows::is_client_animation_enabled();
    let start_id = egui::Id::new("settings_modal_enter_start_time");
    let finalized_id = egui::Id::new("settings_modal_enter_finalized");

    let now = ctx.input(|i| i.time);
    let start_time: f64 = ctx.data_mut(|d| *d.get_temp_mut_or_insert_with(start_id, || now));
    let already_finalized: bool = ctx.data(|d| d.get_temp(finalized_id)).unwrap_or(false);

    let elapsed = (now - start_time) as f32;
    let (scale, opacity, is_animating, needs_final) = if client_anim {
        compute_dialog_enter_animation(elapsed, 0.180, already_finalized)
    } else {
        (1.0, 1.0, false, false)
    };

    if is_animating {
        ctx.request_repaint();
    } else if needs_final {
        ctx.data_mut(|d| d.insert_temp(finalized_id, true));
        ctx.request_repaint();
    }

    // 1. Fullscreen dark backdrop scrim on Middle order (above background page content, strictly below dialog)
    let screen_rect = ctx.screen_rect();
    let scrim_alpha = (115.0 * opacity).round() as u8; // 45% black alpha = 115
    egui::Area::new(egui::Id::new("settings_modal_scrim_area"))
        .fixed_pos(Pos2::ZERO)
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            let resp = ui.allocate_response(screen_rect.size(), egui::Sense::click());
            ui.painter().rect_filled(
                screen_rect,
                Rounding::ZERO,
                Color32::from_black_alpha(scrim_alpha),
            );

            // Esc key closes modal
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                state.show_settings_modal = false;
            }

            // Clicking scrim closes dialog
            if resp.clicked() {
                state.show_settings_modal = false;
            }
        });

    // 2. Centered Windows 11 Fluent Content Dialog on Foreground order (strictly above scrim)
    let shake_offset_x = crate::ui::components::dialog::get_dialog_shake_offset(ctx);
    let base_width = 540.0_f32;

    egui::Area::new(egui::Id::new("settings_modal_dialog_area"))
        .anchor(egui::Align2::CENTER_CENTER, Vec2::new(shake_offset_x, 0.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            ui.multiply_opacity(opacity);

            let frame = egui::Frame::none()
                .fill(tokens.surfaces.dialog)
                .rounding(tokens.radius.card_rounding())
                .stroke(Stroke::new(1.0_f32, Color32::from_white_alpha(35))) // 1px alpha border
                .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                .shadow(egui::epaint::Shadow {
                    offset: Vec2::new(0.0, 16.0),
                    blur: 32.0,
                    spread: 0.0,
                    color: Color32::from_black_alpha(120),
                });

            frame.show(ui, |ui| {
                ui.set_width(base_width * scale);
                ui.set_max_width(base_width);

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
                            .color(tokens.text.secondary),
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
                                .color(tokens.text.secondary),
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
                // Section 3: Hardware Safety Gate Card (Danger-tint bg + danger-text)
                // =================================================================
                egui::Frame::none()
                    .fill(tokens.semantic.danger_bg)
                    .rounding(tokens.radius.control_rounding())
                    .stroke(Stroke::new(1.0_f32, tokens.semantic.danger.gamma_multiply(0.4)))
                    .inner_margin(egui::Margin::symmetric(14.0, 12.0))
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
                                    .color(tokens.text.secondary),
                                );
                            });
                        });

                        ui.add_space(10.0);
                        let is_confirmed = state.ui_confirm_safety;
                        fluent_checkbox(
                            ui,
                            &mut state.ui_confirm_safety,
                            egui::RichText::new("I understand this mutates hardware registers")
                                .font(TypeScale::body())
                                .color(if is_confirmed {
                                    tokens.text.primary
                                } else {
                                    tokens.text.secondary
                                }),
                        );
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
                                .color(tokens.text.secondary),
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
                                .color(tokens.text.secondary),
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
        });

    if !state.show_settings_modal {
        ctx.data_mut(|d| {
            d.remove_temp::<f64>(start_id);
            d.remove_temp::<bool>(finalized_id);
        });
    }

    commit_changes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_dialog_enter_animation_reaches_target_within_epsilon() {
        const DURATION: f32 = 0.180;

        // 1. Start of animation (t = 0.0s)
        let (s0, op0, anim0, final0) = compute_dialog_enter_animation(0.0, DURATION, false);
        assert!((s0 - 0.97).abs() < 1e-6);
        assert!((op0 - 0.0).abs() < 1e-6);
        assert!(anim0);
        assert!(!final0);

        // 2. Midpoint of animation (t = 0.09s, 50% elapsed)
        let (s_mid, op_mid, anim_mid, _) = compute_dialog_enter_animation(0.09, DURATION, false);
        // Ease-out cubic: at 50% linear progress, eased progress is 1 - 0.5^3 = 0.875
        let expected_eased = 1.0 - (0.5_f32 * 0.5 * 0.5);
        let expected_s = 0.97 + (1.0 - 0.97) * expected_eased;
        assert!((s_mid - expected_s).abs() < 1e-5);
        assert!((op_mid - expected_eased).abs() < 1e-5);
        assert!(anim_mid);

        // 3. Exact completion (t = 0.180s)
        let (s_end, op_end, anim_end, final_end) =
            compute_dialog_enter_animation(0.180, DURATION, false);
        assert!(
            (s_end - 1.0).abs() < f32::EPSILON,
            "Scale must land on exactly 1.0 (difference: {})",
            (s_end - 1.0).abs()
        );
        assert!(
            (op_end - 1.0).abs() < f32::EPSILON,
            "Opacity must land on exactly 1.0 (difference: {})",
            (op_end - 1.0).abs()
        );
        assert!(!anim_end);
        assert!(
            final_end,
            "Must request one final repaint upon reaching target"
        );

        // 4. Stable after 500 ms with mouse still (already finalized)
        let (s_500, op_500, anim_500, final_500) =
            compute_dialog_enter_animation(0.500, DURATION, true);
        assert_eq!(s_500, 1.0);
        assert_eq!(op_500, 1.0);
        assert!(!anim_500);
        assert!(
            !final_500,
            "Must not request further repaints when already finalized"
        );

        // 5. Stable after 2000 ms with mouse still
        let (s_2000, op_2000, anim_2000, final_2000) =
            compute_dialog_enter_animation(2.000, DURATION, true);
        assert_eq!(s_2000, 1.0);
        assert_eq!(op_2000, 1.0);
        assert!(!anim_2000);
        assert!(!final_2000);
    }
}
