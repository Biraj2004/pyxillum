//! # SegmentedControl & ToggleTileGrid Component
//!
//! Provides the primary Fluent option selector with an animated sliding indicator,
//! replacing disparate pill and rectangular button groups across all dashboard screens.

use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};

/// A single segment item descriptor.
#[derive(Debug, Clone)]
pub struct SegmentItem<'a> {
    pub label: &'a str,
    pub icon: Option<Icon>,
}

impl<'a> SegmentItem<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, icon: None }
    }

    pub fn with_icon(label: &'a str, icon: Icon) -> Self {
        Self {
            label,
            icon: Some(icon),
        }
    }
}

/// Renders a horizontal Fluent SegmentedControl with animated sliding indicator.
pub fn segmented_control(
    ui: &mut Ui,
    id_source: &str,
    items: &[SegmentItem],
    selected_index: usize,
) -> (usize, Response) {
    let tokens = ThemeTokens::default_dark();
    let font = TypeScale::body();
    let num_items = items.len().max(1);
    let height = 34.0_f32;
    let full_w = ui.available_width();

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(full_w, height), Sense::click());
    let mut new_selection = selected_index;

    if ui.is_rect_visible(rect) {
        let p = ui.painter();

        // Inset well container
        p.rect_filled(rect, tokens.radius.control_rounding(), tokens.surfaces.well);
        p.rect_stroke(
            rect,
            tokens.radius.control_rounding(),
            Stroke::new(1.0_f32, tokens.borders.subtle),
        );

        let segment_w = (rect.width() - 4.0) / num_items as f32;

        // Animated sliding indicator target position and width
        let target_x = rect.min.x + 2.0 + selected_index as f32 * segment_w;
        let anim_id = ui.make_persistent_id(format!("{}_anim_x", id_source));
        let curr_x = ui.ctx().animate_value_with_time(anim_id, target_x, 0.18);

        let indicator_rect = Rect::from_min_size(
            Pos2::new(curr_x, rect.min.y + 2.0),
            Vec2::new(segment_w, rect.height() - 4.0),
        );

        // Render animated sliding indicator
        p.rect_filled(indicator_rect, Rounding::same(6.0), tokens.accent.base);

        // Click detection and item rendering
        if resp.clicked() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let rel_x = (pos.x - (rect.min.x + 2.0)).max(0.0);
                let clicked_idx = (rel_x / segment_w) as usize;
                if clicked_idx < num_items {
                    new_selection = clicked_idx;
                }
            }
        }

        for (i, item) in items.iter().enumerate() {
            let seg_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + 2.0 + i as f32 * segment_w, rect.min.y + 2.0),
                Vec2::new(segment_w, rect.height() - 4.0),
            );

            let is_selected = i == selected_index;
            let is_hovered = resp.hovered()
                && ui.input(|inp| {
                    inp.pointer
                        .hover_pos()
                        .is_some_and(|p| seg_rect.contains(p))
                });

            let text_color = if is_selected {
                Color32::WHITE
            } else if is_hovered {
                tokens.text.primary
            } else {
                tokens.text.secondary
            };

            // Icon + Label
            let text_size = ui.fonts(|f| {
                f.layout_no_wrap(item.label.to_owned(), font.clone(), text_color)
                    .size()
            });
            let icon_w = if item.icon.is_some() { 20.0 } else { 0.0 };
            let mut cursor_x = seg_rect.center().x - (text_size.x + icon_w) / 2.0;

            if let Some(icon) = item.icon {
                let icon_box = Rect::from_center_size(
                    Pos2::new(cursor_x + 8.0, seg_rect.center().y),
                    Vec2::splat(16.0),
                );
                icon.paint(p, icon_box, text_color);
                cursor_x += 20.0;
            }

            let text_pos = Pos2::new(cursor_x, seg_rect.center().y - text_size.y / 2.0);
            let galley =
                ui.fonts(|f| f.layout_no_wrap(item.label.to_owned(), font.clone(), text_color));
            p.galley(text_pos, galley, text_color);
        }
    }

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    (new_selection, resp)
}

/// A 2D toggle tile item for complex selectors with icons and descriptive subtitles.
#[derive(Debug, Clone)]
pub struct ToggleTileItem<'a> {
    pub label: &'a str,
    pub description: &'a str,
    pub icon: Icon,
}

/// Renders a responsive 2D grid of toggle tiles (e.g. for DIY Animation Styles).
pub fn toggle_tile_grid(
    ui: &mut Ui,
    items: &[ToggleTileItem],
    selected_index: usize,
    columns: usize,
) -> usize {
    let tokens = ThemeTokens::default_dark();
    let cols = columns.max(1);
    let mut new_selection = selected_index;
    let full_w = ui.available_width();
    let gap = 8.0_f32;
    let item_w = (full_w - gap * (cols - 1) as f32) / cols as f32;
    let item_h = 56.0_f32;

    egui::Grid::new("toggle_tile_grid")
        .spacing(Vec2::new(gap, gap))
        .show(ui, |ui| {
            for (i, item) in items.iter().enumerate() {
                let is_selected = i == selected_index;
                let (rect, resp) =
                    ui.allocate_exact_size(Vec2::new(item_w, item_h), Sense::click());

                if ui.is_rect_visible(rect) {
                    let p = ui.painter();
                    let is_hovered = resp.hovered();

                    let (bg, border, text_color) = if is_selected {
                        (
                            tokens.accent.base,
                            Stroke::new(1.0_f32, tokens.accent.text_light),
                            Color32::WHITE,
                        )
                    } else if is_hovered {
                        (
                            tokens.surfaces.raised_hover,
                            tokens.card_stroke_hover(),
                            tokens.text.primary,
                        )
                    } else {
                        (
                            tokens.surfaces.card,
                            tokens.card_stroke(),
                            tokens.text.secondary,
                        )
                    };

                    p.rect_filled(rect, tokens.radius.control_rounding(), bg);
                    p.rect_stroke(rect, tokens.radius.control_rounding(), border);

                    // Left icon
                    let icon_box = Rect::from_center_size(
                        Pos2::new(rect.min.x + 22.0, rect.center().y),
                        Vec2::splat(20.0),
                    );
                    item.icon.paint(p, icon_box, text_color);

                    // Text block
                    let text_x = rect.min.x + 42.0;
                    let title_font = crate::ui::typography::TypeScale::body_strong();
                    let desc_font = crate::ui::typography::TypeScale::caption();

                    let title_galley = ui
                        .fonts(|f| f.layout_no_wrap(item.label.to_owned(), title_font, text_color));
                    p.galley(
                        Pos2::new(text_x, rect.min.y + 10.0),
                        title_galley,
                        text_color,
                    );

                    let sub_color = if is_selected {
                        Color32::from_white_alpha(200)
                    } else {
                        tokens.text.tertiary
                    };
                    let desc_galley = ui.fonts(|f| {
                        f.layout_no_wrap(item.description.to_owned(), desc_font, sub_color)
                    });
                    p.galley(Pos2::new(text_x, rect.min.y + 28.0), desc_galley, sub_color);
                }

                if resp.clicked() {
                    new_selection = i;
                }
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }

                if (i + 1) % cols == 0 {
                    ui.end_row();
                }
            }
        });

    new_selection
}
