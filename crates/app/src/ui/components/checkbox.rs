//! # Fluent Checkbox Component
//!
//! Provides an authentic 20px Windows 11 Fluent checkbox with checkmark vector icon.
//! The entire allocated row (checkbox box + label text) constitutes the interactive click target.

use crate::ui::theme::ThemeTokens;
use eframe::egui::{
    self, Color32, Pos2, Rect, Response, Sense, Stroke, TextStyle, Ui, Vec2, WidgetText,
};

/// Renders a 20px Fluent checkbox with a vector check icon and interactive label.
///
/// Clicking anywhere on the checkbox or label toggles `checked`.
pub fn fluent_checkbox(ui: &mut Ui, checked: &mut bool, label: impl Into<WidgetText>) -> Response {
    let tokens = ThemeTokens::default_dark();
    let text: WidgetText = label.into();

    const BOX_SIZE: f32 = 20.0;
    const SPACING: f32 = 10.0;

    let galley = text.into_galley(ui, None, f32::INFINITY, TextStyle::Body);
    let galley_size = galley.size();

    let total_width = BOX_SIZE + SPACING + galley_size.x;
    let height = BOX_SIZE.max(galley_size.y).max(24.0);

    let (rect, mut response) =
        ui.allocate_exact_size(Vec2::new(total_width, height), Sense::click());

    if response.clicked() {
        *checked = !*checked;
        response.mark_changed();
    }

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let box_y = rect.center().y - (BOX_SIZE / 2.0);
        let box_rect = Rect::from_min_size(Pos2::new(rect.min.x, box_y), Vec2::splat(BOX_SIZE));
        let rounding = tokens.radius.control_rounding();

        let is_hovered = response.hovered();

        if *checked {
            // Checked state: filled with accent colour and checkmark
            let bg_color = if is_hovered {
                tokens.accent.hover
            } else {
                tokens.accent.base
            };
            p.rect_filled(box_rect, rounding, bg_color);

            // Vector checkmark inside the 20px box
            let center = box_rect.center();
            let check_p1 = Pos2::new(center.x - 4.5, center.y);
            let check_p2 = Pos2::new(center.x - 1.5, center.y + 3.5);
            let check_p3 = Pos2::new(center.x + 4.5, center.y - 3.5);

            let stroke = Stroke::new(2.0_f32, Color32::WHITE);
            p.line_segment([check_p1, check_p2], stroke);
            p.line_segment([check_p2, check_p3], stroke);
        } else {
            // Unchecked state: translucent dark fill with subtle/hover border
            let bg_color = tokens.surfaces.well;
            let border_color = if is_hovered {
                tokens.text.secondary
            } else {
                tokens.borders.subtle
            };
            p.rect_filled(box_rect, rounding, bg_color);
            p.rect_stroke(box_rect, rounding, Stroke::new(1.2_f32, border_color));
        }

        // Draw label text
        let text_x = rect.min.x + BOX_SIZE + SPACING;
        let text_y = rect.center().y - (galley_size.y / 2.0);
        let text_pos = Pos2::new(text_x, text_y);

        p.galley(text_pos, galley, tokens.text.primary);
    }

    response
}
