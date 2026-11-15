//! # Solid Colour & Dynamic Custom Styles Screen
//!
//! Provides the continuous radial Hue & Saturation colour wheel,
//! RGB channel fine-tuning, white colour temperature slider, hexcode tiles,
//! quick swatches, AND real-time dynamic styles (Breathe, Strobe, Pulse, Wave, Comet, Stars, Sweep).

use crate::catalog::find_closest_hardware_effect;
use crate::state::{AppState, CustomColorEffect, DashboardMode};
use crate::theme::{render_card, render_primary_button, Theme};
use crate::widgets::render_color_disc;
use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

pub fn render_solid_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut state_changed = false;

    let max_w = 720.0_f32.min(ui.available_width());
    let pad = ((ui.available_width() - max_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_max_width(max_w);

            // 1. Custom Colour Wheel & Parameters Card
            render_card(ui, 8.0, |ui| {
                ui.columns(2, |columns| {
                    // Left Column: Chromatic Wheel, Hex Swatch, & RGB Sliders
                    columns[0].vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("Chromatic Colour Wheel")
                                .strong()
                                .size(13.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(6.0);

                        if render_color_disc(ui, &mut state.ui_color, 80.0) {
                            state_changed = true;
                        }

                        ui.add_space(8.0);
                        let mut r = (state.ui_color[0] * 255.0).round() as u8;
                        let mut g = (state.ui_color[1] * 255.0).round() as u8;
                        let mut b = (state.ui_color[2] * 255.0).round() as u8;

                        let (swatch_rect, _) =
                            ui.allocate_exact_size(Vec2::new(210.0, 32.0), egui::Sense::hover());
                        let p = ui.painter_at(swatch_rect);
                        p.rect_filled(swatch_rect, Rounding::same(6.0), Color32::from_rgb(r, g, b));
                        p.rect_stroke(
                            swatch_rect,
                            Rounding::same(6.0),
                            Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
                        );
                        p.text(
                            swatch_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("#{:02X}{:02X}{:02X}  RGB({}, {}, {})", r, g, b, r, g, b),
                            egui::FontId::proportional(11.0),
                            if (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000 > 128 {
                                Color32::BLACK
                            } else {
                                Color32::WHITE
                            },
                        );

                        ui.add_space(8.0);
                        // Micro-tuning channel sliders
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("R")
                                    .size(11.0)
                                    .strong()
                                    .color(Color32::from_rgb(255, 110, 110)),
                            );
                            if ui
                                .add(
                                    egui::Slider::new(&mut r, 0..=255)
                                        .show_value(true)
                                        .clamp_to_range(true),
                                )
                                .changed()
                            {
                                state.ui_color[0] = r as f32 / 255.0;
                                state_changed = true;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("G")
                                    .size(11.0)
                                    .strong()
                                    .color(Color32::from_rgb(110, 255, 110)),
                            );
                            if ui
                                .add(
                                    egui::Slider::new(&mut g, 0..=255)
                                        .show_value(true)
                                        .clamp_to_range(true),
                                )
                                .changed()
                            {
                                state.ui_color[1] = g as f32 / 255.0;
                                state_changed = true;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("B")
                                    .size(11.0)
                                    .strong()
                                    .color(Color32::from_rgb(110, 170, 255)),
                            );
                            if ui
                                .add(
                                    egui::Slider::new(&mut b, 0..=255)
                                        .show_value(true)
                                        .clamp_to_range(true),
                                )
                                .changed()
                            {
                                state.ui_color[2] = b as f32 / 255.0;
                                state_changed = true;
                            }
                        });
                    });

                    // Right Column: Master Brightness, Warmth Slider & Quick Swatches
                    columns[1].vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Master Brightness")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        let pct = (state.ui_brightness as f32 / 255.0 * 100.0) as u32;
                        ui.label(
                            egui::RichText::new(format!("{}%", pct))
                                .strong()
                                .size(18.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(4.0);
                        if ui
                            .add(
                                egui::Slider::new(&mut state.ui_brightness, 0..=255)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            state_changed = true;
                        }

                        ui.add_space(12.0);
                        ui.label(
                            egui::RichText::new("White Temperature (CCT)")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        if ui
                            .add(
                                egui::Slider::new(&mut state.ui_color_temp, 0.0..=1.0)
                                    .text("Warm -> Cool"),
                            )
                            .changed()
                        {
                            let warm = [1.0_f32, 0.77_f32, 0.5_f32];
                            let cool = [0.8_f32, 0.9_f32, 1.0_f32];
                            let t = state.ui_color_temp;
                            state.ui_color = [
                                warm[0] * (1.0 - t) + cool[0] * t,
                                warm[1] * (1.0 - t) + cool[1] * t,
                                warm[2] * (1.0 - t) + cool[2] * t,
                            ];
                            state_changed = true;
                        }

                        ui.add_space(12.0);
                        ui.label(
                            egui::RichText::new("Quick Swatches")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.add_space(6.0);

                        let swatches = [
                            [1.0, 0.2, 0.2],   // Red
                            [1.0, 0.55, 0.0],  // Orange
                            [1.0, 0.85, 0.0],  // Yellow
                            [0.1, 0.85, 0.3],  // Green
                            [0.0, 0.75, 1.0],  // Cyan
                            [0.1, 0.45, 1.0],  // Blue
                            [0.55, 0.2, 1.0],  // Purple
                            [1.0, 0.2, 0.7],   // Magenta
                            [1.0, 1.0, 1.0],   // Pure White
                            [1.0, 0.85, 0.65], // Warm Amber
                        ];

                        ui.horizontal_wrapped(|ui| {
                            for col in &swatches {
                                let (s_rect, s_resp) =
                                    ui.allocate_exact_size(Vec2::splat(26.0), egui::Sense::click());
                                let p = ui.painter_at(s_rect);
                                let r = (col[0] * 255.0) as u8;
                                let g = (col[1] * 255.0) as u8;
                                let b = (col[2] * 255.0) as u8;

                                p.circle_filled(s_rect.center(), 12.0, Color32::from_rgb(r, g, b));
                                p.circle_stroke(
                                    s_rect.center(),
                                    12.0,
                                    Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
                                );

                                if s_resp.clicked() {
                                    state.ui_color = *col;
                                    state_changed = true;
                                }
                            }
                        });
                    });
                });
            });

            ui.add_space(12.0);

            // 2. Custom Colour Dynamic Styles Card
            render_card(ui, 8.0, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Custom Colour Dynamic Styles")
                                .strong()
                                .size(13.5)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let (badge_rect, _) = ui
                                .allocate_exact_size(Vec2::new(130.0, 22.0), egui::Sense::hover());
                            let bp = ui.painter_at(badge_rect);
                            bp.rect_filled(
                                badge_rect,
                                Rounding::same(11.0),
                                Color32::from_rgb(20, 35, 55),
                            );
                            bp.rect_stroke(
                                badge_rect,
                                Rounding::same(11.0),
                                Stroke::new(1.0_f32, Theme::ACCENT_BLUE),
                            );
                            bp.text(
                                badge_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                state.custom_color_effect.name(),
                                egui::FontId::proportional(11.0),
                                Theme::ACCENT_CYAN,
                            );
                        });
                    });

                    ui.label(
                        egui::RichText::new(state.custom_color_effect.description())
                            .size(11.0)
                            .color(Theme::TEXT_MUTED),
                    );

                    ui.add_space(10.0);

                    // Style Selector Pills (8 options)
                    ui.horizontal_wrapped(|ui| {
                        for eff in CustomColorEffect::all() {
                            let is_active = state.custom_color_effect == *eff;
                            let (pill_rect, resp) =
                                ui.allocate_exact_size(Vec2::new(76.0, 28.0), egui::Sense::click());
                            let pp = ui.painter_at(pill_rect);

                            let bg = if is_active {
                                Theme::ACCENT_BLUE
                            } else if resp.hovered() {
                                Theme::CARD_HOVER
                            } else {
                                Theme::WELL_BG
                            };

                            let border = if is_active {
                                Theme::ACCENT_BLUE_HOVER
                            } else {
                                Theme::BORDER_LIGHT
                            };

                            pp.rect_filled(pill_rect, Rounding::same(14.0), bg);
                            pp.rect_stroke(
                                pill_rect,
                                Rounding::same(14.0),
                                Stroke::new(1.0_f32, border),
                            );

                            pp.text(
                                pill_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                eff.name(),
                                egui::FontId::proportional(11.5),
                                if is_active {
                                    Color32::WHITE
                                } else {
                                    Theme::TEXT_SECONDARY
                                },
                            );

                            if resp.clicked() {
                                state.custom_color_effect = *eff;
                                state_changed = true;
                            }
                        }
                    });

                    // Controls when dynamic style is selected (not Solid)
                    if state.custom_color_effect != CustomColorEffect::Solid {
                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.columns(2, |cols| {
                            // Column 0: Animation Speed Slider
                            cols[0].vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Animation Speed")
                                        .size(12.0)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Level {}",
                                        state.custom_effect_speed
                                    ))
                                    .strong()
                                    .size(17.0)
                                    .color(Theme::ACCENT_CYAN),
                                );
                                ui.add_space(4.0);
                                if ui
                                    .add(
                                        egui::Slider::new(&mut state.custom_effect_speed, 1..=10)
                                            .show_value(false),
                                    )
                                    .changed()
                                {
                                    state_changed = true;
                                }
                            });

                            // Column 1: On-Chip Hardware Preset Bridge
                            cols[1].vertical(|ui| {
                                let r = (state.ui_color[0] * 255.0) as u8;
                                let g = (state.ui_color[1] * 255.0) as u8;
                                let b = (state.ui_color[2] * 255.0) as u8;
                                let (chip_code, chip_name) = find_closest_hardware_effect(
                                    r,
                                    g,
                                    b,
                                    state.custom_color_effect,
                                );

                                ui.label(
                                    egui::RichText::new("On-Chip Hardware Sync")
                                        .size(12.0)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "#{} {} (0x{:02X})",
                                        chip_code, chip_name, chip_code
                                    ))
                                    .size(11.5)
                                    .color(Theme::TEXT_MUTED),
                                );
                                ui.add_space(4.0);

                                if render_primary_button(ui, "Sync Directly to Chip", true, 160.0)
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
    });

    state_changed
}
