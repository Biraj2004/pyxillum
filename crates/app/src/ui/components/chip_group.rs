//! # Fluent ChipGroup Component
//!
//! Provides a single horizontally scrollable row of filter chips with gradient edge fades.

use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Sense, Stroke, Ui, Vec2};

pub fn chip_group(ui: &mut Ui, items: &[&str], selected_index: Option<usize>) -> Option<usize> {
    let tokens = ThemeTokens::default_dark();
    let font = TypeScale::caption();
    let mut new_selection = selected_index;

    let area_resp = egui::ScrollArea::horizontal()
        .auto_shrink([false, false])
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;

                for (i, &label) in items.iter().enumerate() {
                    let is_selected = selected_index == Some(i);
                    let text_size = ui.fonts(|f| {
                        f.layout_no_wrap(label.to_owned(), font.clone(), Color32::WHITE)
                            .size()
                    });
                    let chip_w = text_size.x + 20.0;
                    let chip_h = 26.0_f32;

                    let (rect, resp) =
                        ui.allocate_exact_size(Vec2::new(chip_w, chip_h), Sense::click());

                    if resp.clicked() {
                        new_selection = if is_selected { None } else { Some(i) };
                    }

                    if ui.is_rect_visible(rect) {
                        let p = ui.painter();
                        let is_hovered = resp.hovered();

                        // No outline at rest: use control fill (L about 0.26); outline only on hover (about 14% alpha)
                        let (bg, border, text_color) = if is_selected {
                            (tokens.accent.base, Stroke::NONE, Color32::WHITE)
                        } else if is_hovered {
                            (
                                tokens.surfaces.raised_hover,
                                Stroke::new(1.0_f32, tokens.borders.hover),
                                tokens.text.primary,
                            )
                        } else {
                            (
                                tokens.surfaces.raised_hover,
                                Stroke::NONE,
                                tokens.text.secondary,
                            )
                        };

                        p.rect_filled(rect, tokens.radius.chip_rounding(), bg);
                        if border != Stroke::NONE {
                            p.rect_stroke(rect, tokens.radius.chip_rounding(), border);
                        }

                        let text_pos = Pos2::new(
                            rect.center().x - text_size.x / 2.0,
                            rect.center().y - text_size.y / 2.0,
                        );
                        let galley = ui.fonts(|f| {
                            f.layout_no_wrap(label.to_owned(), font.clone(), text_color)
                        });
                        p.galley(text_pos, galley, text_color);
                    }

                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                }
            });
        });

    // Subtle edge fades on the horizontal scroll container
    let container_rect = area_resp.inner_rect;
    let fade_w = 12.0_f32;
    let p = ui.painter();
    let left_fade = egui::Rect::from_min_size(
        container_rect.min,
        Vec2::new(fade_w, container_rect.height()),
    );
    let right_fade = egui::Rect::from_min_size(
        Pos2::new(container_rect.max.x - fade_w, container_rect.min.y),
        Vec2::new(fade_w, container_rect.height()),
    );
    let fade_col = tokens.surfaces.card;
    p.rect_filled(
        left_fade,
        egui::Rounding::ZERO,
        fade_col.linear_multiply(0.4),
    );
    p.rect_filled(
        right_fade,
        egui::Rounding::ZERO,
        fade_col.linear_multiply(0.4),
    );

    new_selection
}
