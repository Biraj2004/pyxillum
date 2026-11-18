//! # Fluent ListRow Component
//!
//! Provides fixed-height list rows for virtualized catalogue lists with
//! hover feedback, left accent indicator for active selection, and accessory buttons.

use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Rounding, Sense, Ui, Vec2};

pub struct ListRow<'a> {
    title: &'a str,
    code: u8,
    category: Option<&'a str>,
    is_selected: bool,
    is_favourite: bool,
}

impl<'a> ListRow<'a> {
    pub fn new(
        title: &'a str,
        code: u8,
        category: Option<&'a str>,
        is_selected: bool,
        is_favourite: bool,
    ) -> Self {
        Self {
            title,
            code,
            category,
            is_selected,
            is_favourite,
        }
    }

    /// Renders the 48px row. Returns `(row_clicked, fav_toggled, response)`.
    pub fn show(self, ui: &mut Ui) -> (bool, bool, Response) {
        let tokens = ThemeTokens::default_dark();
        let height = 48.0_f32;
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

            // Background fill (clipped strictly inside ui.clip_rect() to avoid bleed)
            let draw_rect = rect.intersect(ui.clip_rect());
            let bg = if self.is_selected {
                tokens.accent.base.linear_multiply(0.20)
            } else if is_hovered {
                tokens.surfaces.raised_hover
            } else {
                Color32::TRANSPARENT
            };

            p.rect_filled(draw_rect, tokens.radius.control_rounding(), bg);

            // Left 3px accent indicator bar for active selection
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
                let max_h = rect.height() - 14.0;
                let bar_h = (max_h * anim_t).max(4.0);
                let bar = Rect::from_center_size(
                    Pos2::new(rect.min.x + 2.5, rect.center().y),
                    Vec2::new(3.0, bar_h),
                );
                p.rect_filled(bar, Rounding::same(1.5), tokens.accent.base);
            }

            // Number badge container (32x22 rounded chip)
            let badge_rect = Rect::from_center_size(
                Pos2::new(rect.min.x + 24.0, rect.center().y),
                Vec2::new(32.0, 22.0),
            );
            p.rect_filled(
                badge_rect,
                tokens.radius.chip_rounding(),
                tokens.surfaces.well,
            );
            let badge_text_col = if self.is_selected {
                tokens.accent.text_light
            } else {
                tokens.text.tertiary
            };
            p.text(
                badge_rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{:03}", self.code),
                TypeScale::caption(),
                badge_text_col,
            );

            // Title and Category Caption
            let text_x = badge_rect.max.x + 10.0;
            let max_text_w = (rect.max.x - text_x - 38.0).max(20.0);

            let title_color = if self.is_selected {
                tokens.accent.text_light
            } else if is_hovered {
                tokens.text.primary
            } else {
                tokens.text.secondary
            };

            let title_galley = ui.fonts(|f| {
                let mut job = egui::text::LayoutJob::simple_singleline(
                    self.title.to_owned(),
                    TypeScale::body_strong(),
                    title_color,
                );
                job.wrap = egui::text::TextWrapping {
                    max_width: max_text_w,
                    max_rows: 1,
                    break_anywhere: false,
                    overflow_character: Some('…'),
                };
                f.layout_job(job)
            });
            p.galley(
                Pos2::new(text_x, rect.min.y + 6.0),
                title_galley,
                title_color,
            );

            // Category caption
            if let Some(cat) = self.category {
                let cat_galley = ui.fonts(|f| {
                    let mut job = egui::text::LayoutJob::simple_singleline(
                        cat.to_owned(),
                        TypeScale::caption(),
                        tokens.text.tertiary,
                    );
                    job.wrap = egui::text::TextWrapping {
                        max_width: max_text_w,
                        max_rows: 1,
                        break_anywhere: false,
                        overflow_character: Some('…'),
                    };
                    f.layout_job(job)
                });
                p.galley(
                    Pos2::new(text_x, rect.min.y + 26.0),
                    cat_galley,
                    tokens.text.tertiary,
                );
            }

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

        let resp = if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            resp.on_hover_text(format!("#{:03} {}", self.code, self.title))
        } else {
            resp
        };

        (row_clicked, fav_toggled, resp)
    }
}
