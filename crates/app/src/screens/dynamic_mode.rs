//! # Dynamic Mode Screen
//!
//! Visualizes and controls the 146 SPI addressable dynamic lighting styles.
//! Features the signature radial arc dial, action controls, and dual metric sliders.

use crate::catalog::{get_lighting_style, get_lighting_title};
use crate::state::AppState;
use crate::ui::components::button::FluentButton;
use crate::ui::components::card::Card;
use crate::ui::components::dial::ArcDial;
use crate::ui::components::slider::labeled_slider;
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use eframe::egui::{self, Color32, Rect, Stroke, Vec2};

pub fn render_dynamic_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let tokens = ThemeTokens::default_dark();
    let mut state_changed = false;

    ui.vertical(|ui| {
        // 1. Arc Dial Hero Card
        Card::base().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                let eff_title = get_lighting_title(state.ui_effect);
                let category_name = get_lighting_style(state.ui_effect)
                    .map(|s| s.category.name())
                    .unwrap_or("Dynamic FX");

                let mut eff_val = state.ui_effect;
                let dial = ArcDial::new(&mut eff_val, 1, 146, &eff_title, category_name);
                if dial.show(ui).changed() {
                    state.ui_effect = eff_val;
                    state_changed = true;
                }

                ui.add_space(16.0);

                // Action Bar: 40px primary action row, all items share one centre line.
                // Hero play/pause circular button is exactly 40px circle.
                ui.horizontal(|ui| {
                    let bar_w = ui.available_width();
                    let total_btn_w = 104.0 + 12.0 + 40.0 + 12.0 + 104.0 + 12.0 + 120.0;
                    ui.add_space(((bar_w - total_btn_w) / 2.0).max(0.0));

                    // Loop Mode Toggle (40px height)
                    let loop_label = if state.is_loop_all {
                        "Loop All"
                    } else {
                        "Play Once"
                    };
                    let loop_icon = if state.is_loop_all {
                        Icon::Play
                    } else {
                        Icon::Once
                    };
                    if FluentButton::secondary(loop_label)
                        .icon(loop_icon)
                        .min_width(104.0)
                        .large()
                        .show(ui)
                        .clicked()
                    {
                        state.is_loop_all = !state.is_loop_all;
                        state_changed = true;
                    }

                    ui.add_space(12.0);

                    // Hero Play/Pause 40px Circular Button
                    let play_icon = if state.is_playing_dynamic {
                        Icon::Pause
                    } else {
                        Icon::Play
                    };
                    let (p_rect, p_resp) =
                        ui.allocate_exact_size(Vec2::splat(40.0), egui::Sense::click());
                    let p_resp = p_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                    let pp = ui.painter_at(p_rect);

                    let p_hover = p_resp.hovered();
                    let p_bg = if p_hover {
                        tokens.accent.hover
                    } else {
                        tokens.accent.base
                    };
                    pp.circle_filled(p_rect.center(), 20.0, p_bg);
                    pp.circle_stroke(
                        p_rect.center(),
                        20.0,
                        Stroke::new(1.0_f32, tokens.accent.text_light),
                    );
                    play_icon.paint(
                        &pp,
                        Rect::from_center_size(p_rect.center(), Vec2::splat(18.0)),
                        Color32::WHITE,
                    );

                    if p_resp.clicked() {
                        state.is_playing_dynamic = !state.is_playing_dynamic;
                        state_changed = true;
                    }

                    ui.add_space(12.0);

                    // Favourite Star Button (40px height)
                    let fav_label = if state.is_favourite {
                        "Favorited"
                    } else {
                        "Favorite"
                    };
                    let fav_icon = if state.is_favourite {
                        Icon::StarFilled
                    } else {
                        Icon::Star
                    };
                    if FluentButton::secondary(fav_label)
                        .icon(fav_icon)
                        .min_width(104.0)
                        .large()
                        .show(ui)
                        .clicked()
                    {
                        state.is_favourite = !state.is_favourite;
                    }

                    ui.add_space(12.0);

                    // Preset Catalog Drawer Toggle (40px height)
                    if FluentButton::secondary("Preset Styles")
                        .icon(Icon::Effects)
                        .min_width(120.0)
                        .large()
                        .show(ui)
                        .clicked()
                    {
                        state.show_effect_drawer = !state.show_effect_drawer;
                    }
                });
            });
        });

        ui.add_space(16.0);

        // 2. Dual Adjustment Metric Cards (Master Brightness and Animation Speed)
        crate::ui::layout::split_2(ui, 0.50, 200.0, 200.0, 16.0, |ui, side| match side {
            crate::ui::layout::SplitSide::Left => {
                Card::base().show(ui, |ui| {
                    let pct = (state.ui_brightness as f32 / 255.0 * 100.0) as u32;
                    let val_str = format!("{}%", pct);
                    if labeled_slider(
                        ui,
                        "Master Brightness",
                        &mut state.ui_brightness,
                        0,
                        255,
                        &val_str,
                    )
                    .changed()
                    {
                        state_changed = true;
                    }
                });
            }
            crate::ui::layout::SplitSide::Right => {
                Card::base().show(ui, |ui| {
                    let val_str = format!("Level {}", state.ui_speed);
                    if labeled_slider(ui, "Animation Speed", &mut state.ui_speed, 1, 10, &val_str)
                        .changed()
                    {
                        state_changed = true;
                    }
                });
            }
        });
    });

    state_changed
}
