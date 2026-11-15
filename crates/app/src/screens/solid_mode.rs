//! # Solid Colour Screen
//!
//! Provides the continuous radial Hue & Saturation colour wheel,
//! white colour temperature slider, hexcode tiles, and quick swatches.

use crate::state::AppState;
use crate::theme::{render_card, Theme};
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

            render_card(ui, 8.0, |ui| {
                ui.columns(2, |columns| {
                    // Left Column: Chromatic Wheel & Hex Tile
                    columns[0].vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("Chromatic Colour Wheel")
                                .strong()
                                .size(13.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(8.0);

                        if render_color_disc(ui, &mut state.ui_color, 82.0) {
                            state_changed = true;
                        }

                        ui.add_space(10.0);
                        let r = (state.ui_color[0] * 255.0) as u8;
                        let g = (state.ui_color[1] * 255.0) as u8;
                        let b = (state.ui_color[2] * 255.0) as u8;

                        let (swatch_rect, _) =
                            ui.allocate_exact_size(Vec2::new(190.0, 32.0), egui::Sense::hover());
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
                    });

                    // Right Column: Brightness, Warmth Slider & Quick Swatches
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

                        ui.add_space(14.0);
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

                        ui.add_space(14.0);
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
        });
    });

    state_changed
}
