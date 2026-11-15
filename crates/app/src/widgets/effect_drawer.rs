//! # 146-Effect Preset Catalog Drawer
//!
//! Provides a searchable, filterable grid of all 146 dynamic lighting presets.

use crate::catalog::{EffectCategory, LightingStyle, LIGHTING_STYLES};
use crate::state::AppState;
use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

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
                    .hint_text("Search 146 styles by name or code...")
                    .desired_width(ui.available_width() - 8.0),
            );
        });

        ui.add_space(8.0);

        // Category Filter Pills
        ui.horizontal_wrapped(|ui| {
            let is_all = state.effect_category_filter.is_none();
            let all_btn = ui
                .add(
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
                        Theme::WELL_BG
                    })
                    .stroke(Stroke::new(
                        1.0_f32,
                        if is_all {
                            Theme::ACCENT_BLUE_HOVER
                        } else {
                            Theme::BORDER_LIGHT
                        },
                    ))
                    .rounding(Rounding::same(12.0)),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            if all_btn.clicked() {
                state.effect_category_filter = None;
            }

            for cat in EffectCategory::all() {
                let is_sel = state.effect_category_filter == Some(*cat);
                let btn = ui
                    .add(
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
                            Theme::WELL_BG
                        })
                        .stroke(Stroke::new(
                            1.0_f32,
                            if is_sel {
                                Theme::ACCENT_BLUE_HOVER
                            } else {
                                Theme::BORDER_LIGHT
                            },
                        ))
                        .rounding(Rounding::same(12.0)),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
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
            egui::RichText::new(format!("Showing {} presets", filtered.len()))
                .size(11.0)
                .color(Theme::TEXT_MUTED),
        );
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .max_height(340.0)
            .show(ui, |ui| {
                for style in filtered {
                    let is_active = state.ui_effect == style.code;
                    let item_h = 42.0;
                    let (item_rect, resp) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width() - 8.0, item_h),
                        egui::Sense::click(),
                    );
                    let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                    let p = ui.painter_at(item_rect);

                    let is_hovered = resp.hovered();
                    let bg = if is_active {
                        Theme::CARD_ACTIVE
                    } else if is_hovered {
                        Theme::CARD_HOVER
                    } else {
                        Theme::CARD_BG
                    };

                    let border = if is_active {
                        Theme::BORDER_FOCUS
                    } else if is_hovered {
                        Theme::BORDER_HOVER
                    } else {
                        Theme::BORDER_LIGHT
                    };

                    p.rect_filled(item_rect, Rounding::same(6.0), bg);
                    p.rect_stroke(item_rect, Rounding::same(6.0), Stroke::new(1.0_f32, border));

                    // Left Indicator Pill for Active
                    if is_active {
                        let pill = Rect::from_min_size(
                            Pos2::new(item_rect.min.x + 2.0, item_rect.center().y - 10.0),
                            Vec2::new(3.0, 20.0),
                        );
                        p.rect_filled(pill, Rounding::same(1.5), Theme::ACCENT_BLUE);
                    }

                    // Numeric Badge
                    let badge_rect = Rect::from_min_size(
                        Pos2::new(item_rect.min.x + 12.0, item_rect.center().y - 12.0),
                        Vec2::new(32.0, 24.0),
                    );
                    p.rect_filled(badge_rect, Rounding::same(4.0), Theme::WELL_BG);
                    p.text(
                        badge_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", style.code),
                        egui::FontId::monospace(11.0),
                        if is_active {
                            Theme::ACCENT_CYAN
                        } else {
                            Theme::TEXT_MUTED
                        },
                    );

                    // Name and Description
                    let text_x = badge_rect.max.x + 10.0;
                    p.text(
                        Pos2::new(text_x, item_rect.min.y + 11.0),
                        egui::Align2::LEFT_CENTER,
                        style.name,
                        egui::FontId::proportional(12.5),
                        if is_active {
                            Theme::TEXT_PRIMARY
                        } else {
                            Theme::TEXT_SECONDARY
                        },
                    );

                    p.text(
                        Pos2::new(text_x, item_rect.min.y + 26.0),
                        egui::Align2::LEFT_CENTER,
                        style.description,
                        egui::FontId::proportional(10.0),
                        Theme::TEXT_MUTED,
                    );

                    if resp.clicked() {
                        selected_effect = Some(style.code);
                    }

                    ui.add_space(3.0);
                }
            });
    });

    selected_effect
}
