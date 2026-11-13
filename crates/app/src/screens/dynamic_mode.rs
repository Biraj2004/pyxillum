//! # Dynamic Mode Screen
//!
//! Visualizes and controls the 146 SPI addressable dynamic lighting styles.
//! Features the signature radial arc dial, action controls, and dual metric sliders.

use crate::catalog::{get_lighting_title, EffectCategory};
use crate::state::AppState;
use crate::theme::{render_round_action_button, Theme};
use crate::widgets::ArcDial;
use eframe::egui::{self, Color32};

pub fn render_dynamic_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut state_changed = false;

    ui.vertical_centered(|ui| {
        // 1. Large Circular Arc Dial
        let eff_title = get_lighting_title(state.ui_effect);
        let mut dial = ArcDial::new(state.ui_effect as u32, 146, &eff_title);
        if let Some(delta) = dial.show(ui) {
            let next_val = (state.ui_effect as i32 + delta).clamp(1, 146) as u8;
            if next_val != state.ui_effect {
                state.ui_effect = next_val;
                state_changed = true;
            }
        }

        ui.add_space(14.0);

        // 2. 4-Icon Action Bar (Loop, Play/Pause, Favorite, List)
        ui.horizontal(|ui| {
            let bar_w = ui.available_width();
            ui.add_space(((bar_w - 240.0) / 2.0).max(0.0));

            // Loop Toggle
            let loop_lbl = if state.is_loop_all { "Cycle" } else { "Loop" };
            if render_round_action_button(ui, loop_lbl, state.is_loop_all).clicked() {
                state.is_loop_all = !state.is_loop_all;
                state_changed = true;
            }
            ui.add_space(16.0);

            // Play / Pause Toggle
            let play_lbl = if state.is_playing_dynamic { "||" } else { ">" };
            if render_round_action_button(ui, play_lbl, state.is_playing_dynamic).clicked() {
                state.is_playing_dynamic = !state.is_playing_dynamic;
                state_changed = true;
            }
            ui.add_space(16.0);

            // Favourite Toggle
            if render_round_action_button(ui, "Fav", state.is_favourite).clicked() {
                state.is_favourite = !state.is_favourite;
            }
            ui.add_space(16.0);

            // List / Preset Drawer Toggle
            if render_round_action_button(ui, "List", state.show_effect_drawer).clicked() {
                state.show_effect_drawer = !state.show_effect_drawer;
            }
        });

        ui.add_space(16.0);

        // 3. Dual Adjustment Sliders (Brightness and Speed)
        ui.columns(2, |cols| {
            // Brightness Card
            cols[0].group(|ui| {
                ui.label(
                    egui::RichText::new("Brightness")
                        .size(12.0)
                        .color(Theme::TEXT_SECONDARY),
                );
                let pct = (state.ui_brightness as f32 / 255.0 * 100.0) as u32;
                ui.label(
                    egui::RichText::new(format!("{}%", pct))
                        .strong()
                        .size(16.0)
                        .color(Theme::TEXT_PRIMARY),
                );
                if ui
                    .add(egui::Slider::new(&mut state.ui_brightness, 0..=255).show_value(false))
                    .changed()
                {
                    state_changed = true;
                }
            });

            // Speed Card
            cols[1].group(|ui| {
                ui.label(
                    egui::RichText::new("Speed")
                        .size(12.0)
                        .color(Theme::TEXT_SECONDARY),
                );
                ui.label(
                    egui::RichText::new(format!("{}", state.ui_speed))
                        .strong()
                        .size(16.0)
                        .color(Theme::ACCENT_EMERALD),
                );
                if ui
                    .add(egui::Slider::new(&mut state.ui_speed, 1..=10).show_value(false))
                    .changed()
                {
                    state_changed = true;
                }
            });
        });

        ui.add_space(12.0);

        // 4. Category Quick Select Pills
        ui.horizontal_wrapped(|ui| {
            for cat in &[
                EffectCategory::Rainbow,
                EffectCategory::Fire,
                EffectCategory::Comet,
                EffectCategory::Meteor,
                EffectCategory::DualSnake,
                EffectCategory::Waves,
                EffectCategory::Gradient,
            ] {
                let is_sel = state.effect_category_filter == Some(*cat);
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(cat.name()).size(11.0))
                            .fill(if is_sel {
                                Theme::ACCENT_BLUE
                            } else {
                                Color32::from_rgb(241, 245, 249)
                            })
                            .rounding(egui::Rounding::same(12.0)),
                    )
                    .clicked()
                {
                    state.effect_category_filter = Some(*cat);
                    state.show_effect_drawer = true;
                }
            }
        });
    });

    state_changed
}
