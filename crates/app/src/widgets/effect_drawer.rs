//! # 146-Effect Preset Catalog Drawer
//!
//! Provides a searchable, filterable grid of all 146 dynamic lighting presets.

use crate::catalog::{EffectCategory, LightingStyle, LIGHTING_STYLES};
use crate::state::AppState;
use crate::theme::Theme;
use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

pub fn render_effect_drawer(ui: &mut egui::Ui, state: &mut AppState) -> Option<u8> {
    let mut selected_effect = None;

    ui.vertical(|ui| {
        // Search Input
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Search:")
                    .size(12.0)
                    .color(Theme::TEXT_SECONDARY),
            );
            ui.add(
                egui::TextEdit::singleline(&mut state.effect_search_query)
                    .hint_text("Search 146 effects by name or ID...")
                    .desired_width(ui.available_width() - 10.0),
            );
        });

        ui.add_space(8.0);

        // Category Filter Pills
        ui.horizontal_wrapped(|ui| {
            let is_all = state.effect_category_filter.is_none();
            let all_btn =
                ui.add(
                    egui::Button::new(egui::RichText::new("All (146)").size(11.0).color(
                        if is_all {
                            Color32::WHITE
                        } else {
                            Theme::TEXT_SECONDARY
                        },
                    ))
                    .fill(if is_all {
                        Theme::ACCENT_BLUE
                    } else {
                        Color32::from_rgb(23, 32, 54)
                    })
                    .rounding(Rounding::same(12.0)),
                );
            if all_btn.clicked() {
                state.effect_category_filter = None;
            }

            for cat in EffectCategory::all() {
                let is_sel = state.effect_category_filter == Some(*cat);
                let btn = ui.add(
                    egui::Button::new(egui::RichText::new(cat.name()).size(11.0).color(
                        if is_sel {
                            Color32::WHITE
                        } else {
                            Theme::TEXT_SECONDARY
                        },
                    ))
                    .fill(if is_sel {
                        Theme::ACCENT_BLUE
                    } else {
                        Color32::from_rgb(23, 32, 54)
                    })
                    .rounding(Rounding::same(12.0)),
                );
                if btn.clicked() {
                    state.effect_category_filter = Some(*cat);
                }
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        // Scrollable List of Filtered Effects
        let query = state.effect_search_query.to_lowercase();
        let cat_filter = state.effect_category_filter;

        let filtered: Vec<&LightingStyle> = LIGHTING_STYLES
            .iter()
            .filter(|s| {
                if let Some(cat) = cat_filter {
                    if s.category != cat {
                        return false;
                    }
                }
                if !query.is_empty() {
                    let matches_name = s.name.to_lowercase().contains(&query);
                    let matches_code = format!("{}", s.code).contains(&query);
                    let matches_cat = s.category.name().to_lowercase().contains(&query);
                    matches_name || matches_code || matches_cat
                } else {
                    true
                }
            })
            .collect();

        ui.label(
            egui::RichText::new(format!("Showing {} of 146 Lighting Styles", filtered.len()))
                .size(11.0)
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 20.0)
            .show(ui, |ui| {
                for style in filtered {
                    let is_active = state.ui_effect == style.code;

                    let row_resp = egui::Frame::none()
                        .fill(if is_active {
                            Color32::from_rgb(30, 58, 138)
                        } else {
                            Color32::from_rgb(19, 27, 46)
                        })
                        .stroke(Stroke::new(
                            1.0_f32,
                            if is_active {
                                Theme::ACCENT_CYAN
                            } else {
                                Theme::BORDER_LIGHT
                            },
                        ))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                // Code Badge
                                let badge_text = format!("{:03}", style.code);
                                let (badge_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(34.0, 22.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter_at(badge_rect).rect_filled(
                                    badge_rect,
                                    Rounding::same(5.0),
                                    if is_active {
                                        Theme::ACCENT_CYAN
                                    } else {
                                        Color32::from_rgb(30, 41, 59)
                                    },
                                );
                                ui.painter_at(badge_rect).text(
                                    badge_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    badge_text,
                                    egui::FontId::proportional(11.0),
                                    if is_active {
                                        Color32::BLACK
                                    } else {
                                        Theme::TEXT_PRIMARY
                                    },
                                );

                                ui.add_space(8.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(style.name)
                                            .strong()
                                            .size(13.0)
                                            .color(Theme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        egui::RichText::new(style.description)
                                            .size(10.5)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                });
                            });
                        });

                    if row_resp.response.interact(egui::Sense::click()).clicked() {
                        selected_effect = Some(style.code);
                    }
                    ui.add_space(4.0);
                }
            });
    });

    selected_effect
}
