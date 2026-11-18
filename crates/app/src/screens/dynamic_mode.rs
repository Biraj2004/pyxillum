//! # Dynamic Mode Screen
//!
//! Visualizes and controls the 146 SPI addressable dynamic lighting styles.
//! Features the signature radial arc dial, action controls, and dual metric sliders.

use crate::catalog::{get_lighting_style, get_lighting_title};
use crate::state::AppState;
use crate::theme::{render_action_pill, render_card, render_hero_play_button, Theme};
use crate::widgets::{ActionIcon, ArcDial};
use eframe::egui;

pub fn render_dynamic_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut state_changed = false;

    let max_w = (ui.available_width() * 0.78)
        .clamp(760.0, 940.0)
        .min(ui.available_width());
    let pad = ((ui.available_width() - max_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_width(max_w);

            // 1. Arc Dial Hero Card
            render_card(ui, 8.0, |ui| {
                ui.vertical_centered(|ui| {
                    let eff_title = get_lighting_title(state.ui_effect);
                    let mut dial = ArcDial::new(state.ui_effect as u32, 146, &eff_title);
                    if let Some(delta) = dial.show(ui) {
                        let next_val = (state.ui_effect as i32 + delta).clamp(1, 146) as u8;
                        if next_val != state.ui_effect {
                            state.ui_effect = next_val;
                            state_changed = true;
                        }
                    }

                    // Category Pill
                    if let Some(style) = get_lighting_style(state.ui_effect) {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(style.category.name())
                                .size(11.0)
                                .color(Theme::TEXT_MUTED),
                        );
                    }

                    ui.add_space(16.0);

                    // Refined Action Bar: [Loop/Once Pill] [Play Hero Button] [Fav Pill] [Presets Pill]
                    ui.horizontal(|ui| {
                        let bar_w = ui.available_width();
                        // Total bar width: 78 + 12 + 44 + 12 + 74 + 12 + 86 = 318
                        ui.add_space(((bar_w - 318.0) / 2.0).max(0.0));

                        // Loop Mode Toggle Pill
                        let loop_act = ActionIcon::Loop(state.is_loop_all);
                        let loop_text = if state.is_loop_all { "Loop" } else { "Once" };
                        if render_action_pill(ui, loop_act, loop_text, state.is_loop_all, 78.0)
                            .clicked()
                        {
                            state.is_loop_all = !state.is_loop_all;
                            state_changed = true;
                        }
                        ui.add_space(12.0);

                        // Hero Play/Pause Circular Button
                        if render_hero_play_button(ui, state.is_playing_dynamic).clicked() {
                            state.is_playing_dynamic = !state.is_playing_dynamic;
                            state_changed = true;
                        }
                        ui.add_space(12.0);

                        // Favourite Toggle Pill
                        let fav_act = ActionIcon::Star(state.is_favourite);
                        let fav_text = if state.is_favourite { "Saved" } else { "Fav" };
                        if render_action_pill(ui, fav_act, fav_text, state.is_favourite, 74.0)
                            .clicked()
                        {
                            state.is_favourite = !state.is_favourite;
                        }
                        ui.add_space(12.0);

                        // Preset Catalog Drawer Toggle Pill
                        if render_action_pill(
                            ui,
                            ActionIcon::List,
                            "Presets",
                            state.show_effect_drawer,
                            86.0,
                        )
                        .clicked()
                        {
                            state.show_effect_drawer = !state.show_effect_drawer;
                        }
                    });
                });
            });

            ui.add_space(12.0);

            // 2. Dual Adjustment Metric Cards (Brightness and Speed)
            ui.columns(2, |cols| {
                // Brightness Card
                render_card(&mut cols[0], 8.0, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Master Brightness")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pct = (state.ui_brightness as f32 / 255.0 * 100.0) as u32;
                            ui.label(
                                egui::RichText::new(format!("{}%", pct))
                                    .strong()
                                    .size(15.0)
                                    .color(Theme::TEXT_PRIMARY),
                            );
                        });
                    });
                    ui.add_space(8.0);
                    if ui
                        .add(egui::Slider::new(&mut state.ui_brightness, 0..=255).show_value(false))
                        .changed()
                    {
                        state_changed = true;
                    }
                });

                // Speed Card
                render_card(&mut cols[1], 8.0, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Animation Speed")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("Level {}", state.ui_speed))
                                    .strong()
                                    .size(15.0)
                                    .color(Theme::ACCENT_CYAN),
                            );
                        });
                    });
                    ui.add_space(8.0);
                    if ui
                        .add(egui::Slider::new(&mut state.ui_speed, 1..=10).show_value(false))
                        .changed()
                    {
                        state_changed = true;
                    }
                });
            });
        });
    });

    state_changed
}
