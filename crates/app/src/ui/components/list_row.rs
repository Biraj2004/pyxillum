//! # Fluent ListRow Component
//!
//! Provides fixed-height list rows for virtualized catalogue lists with
//! hover feedback, left accent indicator for active selection, and accessory buttons.

use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Sense, Ui, Vec2};

pub struct ListRow<'a> {
    title: &'a str,
    code: u8,
    is_selected: bool,
    is_favourite: bool,
}

impl<'a> ListRow<'a> {
    pub fn new(title: &'a str, code: u8, is_selected: bool, is_favourite: bool) -> Self {
        Self {
            title,
            code,
            is_selected,
            is_favourite,
        }
    }

    /// Renders the row. Returns `(row_clicked, fav_toggled)`.
    pub fn show(self, ui: &mut Ui) -> (bool, bool) {
        let tokens = ThemeTokens::default_dark();
        let height = 38.0_f32;
        let full_w = ui.available_width();

        let (rect, resp) = ui.allocate_exact_size(Vec2::new(full_w, height), Sense::click());
        let mut row_clicked = false;
        let mut fav_toggled = false;

        if resp.clicked() {
            row_clicked = true;
        }

        if ui.is_rect_visible(rect) {
            let p = ui.painter();
            let is_hovered = resp.hovered();

            // Background fill
            let bg = if self.is_selected {
                tokens.accent.base.linear_multiply(0.20)
            } else if is_hovered {
                tokens.surfaces.raised_hover
            } else {
                Color32::TRANSPARENT
            };

            p.rect_filled(rect, tokens.radius.control_rounding(), bg);

            // Left 3px accent indicator bar for active row (animates in height on selection)
            let anim_id = ui.make_persistent_id(format!("list_row_bar_anim_{}", self.code));
            let client_anim = crate::platform::windows::is_client_animation_enabled();
            let anim_t = if client_anim {
                ui.ctx()
                    .animate_bool_with_time(anim_id, self.is_selected, 0.16)
            } else if self.is_selected {
                1.0
            } else {
                0.0
            };

            if anim_t > 0.01 {
                let max_h = rect.height() - 12.0;
                let bar_h = (max_h * anim_t).max(3.0);
                let bar = Rect::from_center_size(
                    Pos2::new(rect.min.x + 3.5, rect.center().y),
                    Vec2::new(3.0, bar_h),
                );
                p.rect_filled(bar, Rounding::same(1.5), tokens.accent.base);
            }

            // Numeric ID tag (e.g. "#146")
            let id_str = format!("{:03}", self.code);
            let id_galley =
                ui.fonts(|f| f.layout_no_wrap(id_str, TypeScale::caption(), tokens.text.tertiary));
            p.galley(
                Pos2::new(rect.min.x + 16.0, rect.center().y - 6.0),
                id_galley,
                tokens.text.tertiary,
            );

            // Title
            let text_color = if self.is_selected {
                tokens.accent.text_light
            } else if is_hovered {
                tokens.text.primary
            } else {
                tokens.text.secondary
            };

            let title_font = if self.is_selected {
                TypeScale::body_strong()
            } else {
                TypeScale::body()
            };
            let title_galley =
                ui.fonts(|f| f.layout_no_wrap(self.title.to_owned(), title_font, text_color));
            p.galley(
                Pos2::new(rect.min.x + 52.0, rect.center().y - 7.0),
                title_galley,
                text_color,
            );

            // Favorite star on the right
            let fav_rect = Rect::from_center_size(
                Pos2::new(rect.max.x - 20.0, rect.center().y),
                Vec2::splat(24.0),
            );
            let fav_resp = ui.interact(
                fav_rect,
                ui.make_persistent_id(format!("fav_row_{}", self.code)),
                Sense::click(),
            );
            let star_color = if self.is_favourite {
                tokens.semantic.warning
            } else if fav_resp.hovered() {
                tokens.text.primary
            } else {
                tokens.text.disabled
            };

            let star_icon = if self.is_favourite {
                Icon::StarFilled
            } else {
                Icon::Star
            };
            star_icon.paint(
                p,
                Rect::from_center_size(fav_rect.center(), Vec2::splat(16.0)),
                star_color,
            );

            if fav_resp.clicked() {
                fav_toggled = true;
            }
        }

        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        (row_clicked, fav_toggled)
    }
}
