//! # Fluent ToggleSwitch Component
//!
//! Provides a 40x20px native Windows 11 Fluent toggle switch with smooth animated thumb travel.

use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};

/// Renders a 40x20px Fluent Toggle Switch.
pub fn toggle_switch(
    ui: &mut Ui,
    id_source: &str,
    is_on: &mut bool,
    label: Option<&str>,
) -> Response {
    let tokens = ThemeTokens::default_dark();
    let width = 40.0_f32;
    let height = 20.0_f32;

    let mut total_w = width;
    if let Some(lbl) = label {
        let text_size = ui.fonts(|f| {
            f.layout_no_wrap(lbl.to_owned(), TypeScale::body(), Color32::WHITE)
                .size()
        });
        total_w += text_size.x + 8.0;
    }

    let (rect, mut resp) = ui.allocate_exact_size(Vec2::new(total_w, height), Sense::click());

    if resp.clicked() {
        *is_on = !*is_on;
        resp.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let switch_rect = Rect::from_min_size(rect.min, Vec2::new(width, height));

        // Animated travel factor [0.0 = Off, 1.0 = On]
        let anim_id = ui.make_persistent_id(format!("{}_switch_anim", id_source));
        let t = ui.ctx().animate_bool_with_time(anim_id, *is_on, 0.14);

        let (bg, border, thumb_color) = if t > 0.5 {
            (tokens.accent.base, Stroke::NONE, Color32::WHITE)
        } else {
            (
                tokens.surfaces.well,
                Stroke::new(1.0_f32, tokens.borders.subtle),
                tokens.text.secondary,
            )
        };

        // Switch pill track
        p.rect_filled(switch_rect, Rounding::same(10.0), bg);
        if border != Stroke::NONE {
            p.rect_stroke(switch_rect, Rounding::same(10.0), border);
        }

        // Sliding circular thumb
        let thumb_r = 7.0_f32;
        let min_center_x = switch_rect.min.x + 10.0;
        let max_center_x = switch_rect.max.x - 10.0;
        let thumb_x = min_center_x + (max_center_x - min_center_x) * t;
        let thumb_center = Pos2::new(thumb_x, switch_rect.center().y);

        p.circle_filled(thumb_center, thumb_r, thumb_color);

        // Optional label on the right
        if let Some(lbl) = label {
            let text_pos = Pos2::new(switch_rect.max.x + 8.0, rect.center().y - 7.0);
            let galley = ui.fonts(|f| {
                f.layout_no_wrap(lbl.to_owned(), TypeScale::body(), tokens.text.primary)
            });
            p.galley(text_pos, galley, tokens.text.primary);
        }
    }

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    resp
}
