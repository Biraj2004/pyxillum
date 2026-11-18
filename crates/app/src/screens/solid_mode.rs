//! # Solid Colour & Dynamic Custom Styles Screen
//!
//! Provides the continuous radial Hue & Saturation colour wheel,
//! hex code validator with clipboard copy, RGB channel fine-tuning gradient sliders,
//! white colour temperature CCT slider, curated rounded preset swatches,
//! and real-time dynamic styles (Breathe, Strobe, Pulse, Wave, Comet, Stars, Sweep).

use crate::catalog::find_closest_hardware_effect;
use crate::state::{AppState, CustomColorEffect, DashboardMode};
use crate::ui::components::button::FluentButton;
use crate::ui::components::card::Card;
use crate::ui::components::segmented::{segmented_control, SegmentItem};
use crate::ui::components::slider::{colour_channel_slider, fluent_slider};
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use crate::widgets::render_color_disc;
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};

pub fn render_solid_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let tokens = ThemeTokens::default_dark();
    let mut state_changed = false;

    ui.vertical(|ui| {
        // 1. Two-Column Main Stage: Colour Tuning & Intensity/Temperature
        ui.columns(2, |cols| {
            // Left Column: Chromatic Wheel, Hex Input, RGB Sliders
            Card::base().show(&mut cols[0], |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Chromatic Colour Wheel")
                            .font(TypeScale::body_strong())
                            .color(tokens.text.primary),
                    );
                    ui.add_space(8.0);

                    if render_color_disc(ui, &mut state.ui_color, 85.0) {
                        state_changed = true;
                    }

                    ui.add_space(10.0);

                    let mut r = (state.ui_color[0] * 255.0).round() as u8;
                    let mut g = (state.ui_color[1] * 255.0).round() as u8;
                    let mut b = (state.ui_color[2] * 255.0).round() as u8;

                    // Hex Swatch Card with Copy Button
                    let hex_str = format!("#{:02X}{:02X}{:02X}", r, g, b);
                    ui.horizontal(|ui| {
                        let full_w = ui.available_width();
                        ui.add_space(((full_w - 220.0) / 2.0).max(0.0));

                        let (swatch_rect, _) =
                            ui.allocate_exact_size(Vec2::new(140.0, 32.0), egui::Sense::hover());
                        let p = ui.painter_at(swatch_rect);
                        p.rect_filled(
                            swatch_rect,
                            tokens.radius.control_rounding(),
                            Color32::from_rgb(r, g, b),
                        );
                        p.rect_stroke(
                            swatch_rect,
                            tokens.radius.control_rounding(),
                            Stroke::new(1.0_f32, tokens.borders.subtle),
                        );

                        // High contrast text inside swatch
                        let text_col =
                            if (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000 > 128 {
                                Color32::BLACK
                            } else {
                                Color32::WHITE
                            };
                        p.text(
                            swatch_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &hex_str,
                            TypeScale::body_strong(),
                            text_col,
                        );

                        ui.add_space(6.0);
                        if FluentButton::secondary("Copy")
                            .icon(Icon::Copy)
                            .min_width(64.0)
                            .show(ui)
                            .clicked()
                        {
                            ui.output_mut(|o| o.copied_text = hex_str.clone());
                        }
                    });

                    ui.add_space(14.0);

                    // RGB Channel Sliders with Live Colour Gradients
                    let mut changed_channel = false;
                    let old_r = r;
                    let old_g = g;
                    let old_b = b;

                    colour_channel_slider(ui, "Red Channel", &mut r, state.ui_color, 0);
                    ui.add_space(4.0);
                    colour_channel_slider(ui, "Green Channel", &mut g, state.ui_color, 1);
                    ui.add_space(4.0);
                    colour_channel_slider(ui, "Blue Channel", &mut b, state.ui_color, 2);

                    if r != old_r {
                        state.ui_color[0] = r as f32 / 255.0;
                        changed_channel = true;
                    }
                    if g != old_g {
                        state.ui_color[1] = g as f32 / 255.0;
                        changed_channel = true;
                    }
                    if b != old_b {
                        state.ui_color[2] = b as f32 / 255.0;
                        changed_channel = true;
                    }
                    if changed_channel {
                        state_changed = true;
                    }
                });
            });

            // Right Column: Master Brightness, CCT Temperature & Curated Swatches
            Card::base().show(&mut cols[1], |ui| {
                ui.vertical(|ui| {
                    // 1. Master Brightness
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Master Brightness")
                                .font(TypeScale::body())
                                .color(tokens.text.secondary),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pct = (state.ui_brightness as f32 / 255.0 * 100.0) as u32;
                            ui.label(
                                egui::RichText::new(format!("{}%", pct))
                                    .font(TypeScale::body_strong())
                                    .color(tokens.text.primary),
                            );
                        });
                    });
                    ui.add_space(8.0);
                    if fluent_slider(ui, "", &mut state.ui_brightness, 0, 255, "%").changed() {
                        state_changed = true;
                    }

                    ui.add_space(16.0);

                    // 2. White Colour Temperature (CCT: 2700K Warm to 6500K Cool)
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("White Temperature (CCT)")
                                .font(TypeScale::body())
                                .color(tokens.text.secondary),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let cct_kelvin =
                                (2700.0 + state.ui_color_temp * (6500.0 - 2700.0)) as u32;
                            ui.label(
                                egui::RichText::new(format!("{} K", cct_kelvin))
                                    .font(TypeScale::caption())
                                    .color(tokens.accent.text_light),
                            );
                        });
                    });
                    ui.add_space(6.0);

                    // Custom CCT Gradient Track
                    let cct_w = ui.available_width();
                    let (cct_rect, cct_resp) =
                        ui.allocate_exact_size(Vec2::new(cct_w, 22.0), Sense::click_and_drag());
                    if cct_resp.dragged() || cct_resp.clicked() {
                        if let Some(pos) = cct_resp.interact_pointer_pos() {
                            let t = ((pos.x - cct_rect.min.x) / cct_rect.width()).clamp(0.0, 1.0);
                            state.ui_color_temp = t;
                            let warm = [1.0_f32, 0.77_f32, 0.5_f32];
                            let cool = [0.8_f32, 0.9_f32, 1.0_f32];
                            state.ui_color = [
                                warm[0] * (1.0 - t) + cool[0] * t,
                                warm[1] * (1.0 - t) + cool[1] * t,
                                warm[2] * (1.0 - t) + cool[2] * t,
                            ];
                            state_changed = true;
                        }
                    }

                    if ui.is_rect_visible(cct_rect) {
                        let p = ui.painter();
                        let track_h = 6.0_f32;
                        let track_r = Rect::from_center_size(
                            cct_rect.center(),
                            Vec2::new(cct_rect.width(), track_h),
                        );
                        p.rect_filled(track_r, tokens.radius.chip_rounding(), tokens.surfaces.well);

                        // Gradient steps from 2700K warm to 6500K cool
                        let steps = 16;
                        let step_w = track_r.width() / steps as f32;
                        for i in 0..steps {
                            let frac = i as f32 / steps as f32;
                            let col = Color32::from_rgb(
                                (255.0 * (1.0 - frac * 0.2)) as u8,
                                (196.0 + frac * 34.0) as u8,
                                (128.0 + frac * 127.0) as u8,
                            );
                            let seg = Rect::from_min_size(
                                Pos2::new(track_r.min.x + i as f32 * step_w, track_r.min.y),
                                Vec2::new(step_w + 1.0, track_h),
                            );
                            p.rect_filled(seg, tokens.radius.chip_rounding(), col);
                        }

                        // Thumb
                        let thumb_x = track_r.min.x + track_r.width() * state.ui_color_temp;
                        let thumb_pos = Pos2::new(thumb_x, track_r.center().y);
                        p.circle_filled(
                            Pos2::new(thumb_pos.x, thumb_pos.y + 1.0),
                            9.0,
                            Color32::from_black_alpha(60),
                        );
                        p.circle_filled(thumb_pos, 8.5, Color32::WHITE);
                        p.circle_filled(thumb_pos, 3.5, tokens.accent.base);
                    }

                    ui.add_space(16.0);

                    // 3. Curated Quick Swatches (Rounded squares with selected ring)
                    ui.label(
                        egui::RichText::new("Preset Swatches")
                            .font(TypeScale::body())
                            .color(tokens.text.secondary),
                    );
                    ui.add_space(8.0);

                    let swatches = [
                        [1.0, 0.2, 0.2],   // Red
                        [1.0, 0.55, 0.0],  // Orange
                        [1.0, 0.85, 0.0],  // Yellow
                        [0.1, 0.85, 0.3],  // Green
                        [0.0, 0.75, 1.0],  // Cyan
                        [0.1, 0.45, 1.0],  // Blue
                        [0.55, 0.2, 1.0],  // Purple
                        [1.0, 0.2, 0.7],   // Magenta
                        [1.0, 1.0, 1.0],   // White
                        [1.0, 0.85, 0.65], // Warm Amber
                    ];

                    ui.horizontal_wrapped(|ui| {
                        for col in &swatches {
                            let is_selected = (state.ui_color[0] - col[0]).abs() < 0.04
                                && (state.ui_color[1] - col[1]).abs() < 0.04
                                && (state.ui_color[2] - col[2]).abs() < 0.04;

                            let (s_rect, s_resp) =
                                ui.allocate_exact_size(Vec2::splat(28.0), Sense::click());
                            let s_resp = s_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                            let p = ui.painter_at(s_rect);

                            let r = (col[0] * 255.0) as u8;
                            let g = (col[1] * 255.0) as u8;
                            let b = (col[2] * 255.0) as u8;

                            p.rect_filled(
                                s_rect,
                                tokens.radius.control_rounding(),
                                Color32::from_rgb(r, g, b),
                            );

                            if is_selected {
                                p.rect_stroke(
                                    s_rect.expand(2.0),
                                    tokens.radius.card_rounding(),
                                    Stroke::new(2.0_f32, tokens.accent.text_light),
                                );
                            } else {
                                p.rect_stroke(
                                    s_rect,
                                    tokens.radius.control_rounding(),
                                    Stroke::new(1.0_f32, tokens.borders.subtle),
                                );
                            }

                            if s_resp.clicked() {
                                state.ui_color = *col;
                                state_changed = true;
                            }
                        }
                    });
                });
            });
        });

        ui.add_space(16.0);

        // 2. Custom Colour Dynamic Styles Card
        Card::base().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Custom Colour Dynamic Styles")
                            .font(TypeScale::subtitle())
                            .color(tokens.text.primary),
                    );
                });

                ui.label(
                    egui::RichText::new(state.custom_color_effect.description())
                        .font(TypeScale::caption())
                        .color(tokens.text.tertiary),
                );

                ui.add_space(12.0);

                // Style Selector via Segmented Control (Solid, Breathe, Strobe, Pulse, Wave, Comet, Stars, Sweep)
                let all_effects = CustomColorEffect::all();
                let segment_items: Vec<SegmentItem> = all_effects
                    .iter()
                    .map(|e| SegmentItem::new(e.name()))
                    .collect();

                let curr_idx = all_effects
                    .iter()
                    .position(|e| *e == state.custom_color_effect)
                    .unwrap_or(0);

                let (new_idx, resp) = segmented_control(
                    ui,
                    "solid_dynamic_styles_segmented",
                    &segment_items,
                    curr_idx,
                );

                if resp.clicked() && new_idx != curr_idx {
                    if let Some(&eff) = all_effects.get(new_idx) {
                        state.custom_color_effect = eff;
                        state_changed = true;
                    }
                }

                // Additional parameters when dynamic style is selected (not static Solid)
                if state.custom_color_effect != CustomColorEffect::Solid {
                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.columns(2, |cols| {
                        // Animation Speed Slider
                        cols[0].vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Animation Speed")
                                        .font(TypeScale::body())
                                        .color(tokens.text.secondary),
                                );
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "Level {}",
                                                state.custom_effect_speed
                                            ))
                                            .font(TypeScale::body_strong())
                                            .color(tokens.accent.text_light),
                                        );
                                    },
                                );
                            });
                            ui.add_space(8.0);
                            if fluent_slider(ui, "", &mut state.custom_effect_speed, 1, 10, "Level")
                                .changed()
                            {
                                state_changed = true;
                            }
                        });

                        // On-Chip Hardware Sync Card
                        cols[1].vertical(|ui| {
                            let r = (state.ui_color[0] * 255.0) as u8;
                            let g = (state.ui_color[1] * 255.0) as u8;
                            let b = (state.ui_color[2] * 255.0) as u8;
                            let (chip_code, chip_name) =
                                find_closest_hardware_effect(r, g, b, state.custom_color_effect);

                            ui.label(
                                egui::RichText::new("On-Chip Hardware Sync")
                                    .font(TypeScale::body())
                                    .color(tokens.text.secondary),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "#{} {} (0x{:02X})",
                                    chip_code, chip_name, chip_code
                                ))
                                .font(TypeScale::caption())
                                .color(tokens.text.tertiary),
                            );
                            ui.add_space(8.0);

                            if FluentButton::primary("Sync Directly to Chip")
                                .icon(Icon::HardwareChip)
                                .show(ui)
                                .clicked()
                            {
                                state.ui_effect = chip_code;
                                state.dashboard_mode = DashboardMode::Dynamic;
                                state_changed = true;
                            }
                        });
                    });
                }
            });
        });
    });

    state_changed
}
