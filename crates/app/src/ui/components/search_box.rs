//! # Fluent SearchBox Component
//!
//! Provides a 32px height text search input with leading search icon,
//! placeholder text, focus ring, and clear button.

use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Renders a 32px Fluent Search Box.
pub fn search_box(ui: &mut Ui, query: &mut String, placeholder: &str) -> Response {
    let tokens = ThemeTokens::default_dark();
    let font = TypeScale::body();
    let height = 32.0_f32;
    let full_w = ui.available_width();

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(full_w, height), Sense::click());

    if ui.is_rect_visible(rect) {
        let is_hovered = resp.hovered();

        // Well container with subtle border
        let border = if is_hovered {
            tokens.borders.hover
        } else {
            tokens.borders.subtle
        };
        {
            let p = ui.painter();
            p.rect_filled(rect, tokens.radius.control_rounding(), tokens.surfaces.well);
            p.rect_stroke(
                rect,
                tokens.radius.control_rounding(),
                Stroke::new(1.0_f32, border),
            );

            // Leading Search Icon
            let icon_box = Rect::from_center_size(
                Pos2::new(rect.min.x + 16.0, rect.center().y),
                Vec2::splat(16.0),
            );
            Icon::Search.paint(p, icon_box, tokens.text.secondary);
        }

        // Clear button on the right if query is not empty
        let right_offset = if !query.is_empty() { 32.0 } else { 8.0 };

        // Input rect
        let input_rect = Rect::from_min_max(
            Pos2::new(rect.min.x + 32.0, rect.min.y + 2.0),
            Pos2::new(rect.max.x - right_offset, rect.max.y - 2.0),
        );

        let mut child_ui = ui.child_ui(
            input_rect,
            egui::Layout::left_to_right(egui::Align::Center),
            None,
        );
        child_ui.add(
            egui::TextEdit::singleline(query)
                .font(font)
                .hint_text(egui::RichText::new(placeholder).color(tokens.text.tertiary))
                .frame(false)
                .text_color(tokens.text.primary),
        );

        if !query.is_empty() {
            let clear_rect = Rect::from_center_size(
                Pos2::new(rect.max.x - 16.0, rect.center().y),
                Vec2::splat(20.0),
            );
            let clear_resp = ui.interact(
                clear_rect,
                ui.make_persistent_id("search_clear_btn"),
                Sense::click(),
            );
            let clear_color = if clear_resp.hovered() {
                tokens.text.primary
            } else {
                tokens.text.tertiary
            };
            Icon::Close.paint(
                ui.painter(),
                Rect::from_center_size(clear_rect.center(), Vec2::splat(12.0)),
                clear_color,
            );
            if clear_resp.clicked() {
                query.clear();
            }
        }
    }

    resp
}
