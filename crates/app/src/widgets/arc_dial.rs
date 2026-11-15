//! # Scalable Windows 11 Arc Dial Widget
//!
//! Provides the signature 270-degree radial arc gauge with authentic
//! Windows 11 Fluent dark styling, interactive draggable thumb, and center readout.

use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

pub struct ArcDial {
    pub current_value: u32,
    pub max_value: u32,
    pub title: String,
    pub radius: f32,
}

impl ArcDial {
    pub fn new(current_value: u32, max_value: u32, title: impl Into<String>) -> Self {
        Self {
            current_value,
            max_value,
            title: title.into(),
            radius: 80.0,
        }
    }

    /// Renders the arc dial and returns an optional step delta: -1 for left, +1 for right.
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<i32> {
        let mut delta = None;

        let total_w = self.radius * 2.0 + 96.0;
        let total_h = self.radius * 2.0 + 64.0;
        let (rect, resp) =
            ui.allocate_exact_size(Vec2::new(total_w, total_h), egui::Sense::click_and_drag());
        let painter = ui.painter_at(rect);

        // Tactile cursor feedback for radial gauge
        if resp.dragged() || resp.is_pointer_button_down_on() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        } else if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        }

        let center = Pos2::new(rect.center().x, rect.min.y + self.radius + 18.0);
        let radius = self.radius;
        let track_thickness = 11.0_f32;

        let start_deg = 135.0f32;
        let sweep_deg = 270.0f32;
        let progress = (self.current_value as f32 / self.max_value as f32).clamp(0.01, 1.0);

        // Handle dragging on the arc dial
        if resp.dragged() || resp.clicked() {
            if let Some(mouse_pos) = resp.interact_pointer_pos() {
                let vec = mouse_pos - center;
                let mut angle_deg = vec.y.atan2(vec.x).to_degrees();
                if angle_deg < 0.0 {
                    angle_deg += 360.0;
                }

                let norm_angle = if angle_deg >= 135.0 {
                    angle_deg - 135.0
                } else if angle_deg <= 45.0 {
                    angle_deg + 225.0
                } else {
                    // In dead zone at bottom
                    if angle_deg > 90.0 {
                        0.0
                    } else {
                        270.0
                    }
                };

                let new_pct = (norm_angle / 270.0).clamp(0.0, 1.0);
                let target_val = (new_pct * (self.max_value - 1) as f32 + 1.0).round() as u32;
                if target_val > self.current_value {
                    delta = Some(1);
                } else if target_val < self.current_value {
                    delta = Some(-1);
                }
            }
        }

        // 1. Inactive Background Track (Neutral Dark Windows Track)
        let segments = 54;
        for i in 0..segments {
            let a1 = (start_deg + sweep_deg * (i as f32 / segments as f32)).to_radians();
            let a2 = (start_deg + sweep_deg * ((i + 1) as f32 / segments as f32)).to_radians();
            let p1 = center + Vec2::new(a1.cos(), a1.sin()) * radius;
            let p2 = center + Vec2::new(a2.cos(), a2.sin()) * radius;
            painter.line_segment(
                [p1, p2],
                Stroke::new(track_thickness, Color32::from_rgb(52, 52, 52)),
            );
        }

        // 2. Active Arc Track (Microsoft Fluent Blue #0078D4 to #389BF2)
        let active_segments = ((segments as f32 * progress) as usize).max(1);
        for i in 0..active_segments {
            let f = i as f32 / segments as f32;
            let a1 = (start_deg + sweep_deg * f).to_radians();
            let a2 = (start_deg + sweep_deg * ((i + 1) as f32 / segments as f32)).to_radians();
            let p1 = center + Vec2::new(a1.cos(), a1.sin()) * radius;
            let p2 = center + Vec2::new(a2.cos(), a2.sin()) * radius;

            let r = (0.0 * (1.0 - f) + 56.0 * f) as u8;
            let g = (120.0 * (1.0 - f) + 155.0 * f) as u8;
            let b = (212.0 * (1.0 - f) + 242.0 * f) as u8;
            let col = Color32::from_rgb(r, g, b);
            painter.line_segment([p1, p2], Stroke::new(track_thickness, col));
        }

        // 3. Thumb Indicator
        let thumb_angle = (start_deg + sweep_deg * progress).to_radians();
        let thumb_pos = center + Vec2::new(thumb_angle.cos(), thumb_angle.sin()) * radius;
        painter.circle_filled(thumb_pos, track_thickness * 0.75, Color32::WHITE);
        painter.circle_stroke(
            thumb_pos,
            track_thickness * 0.75,
            Stroke::new(2.0_f32, Theme::ACCENT_BLUE),
        );

        // 4. Inner Disc
        let inner_r = radius - track_thickness - 6.0;
        painter.circle_filled(center, inner_r, Color32::from_rgb(34, 34, 34));
        painter.circle_stroke(
            center,
            inner_r,
            Stroke::new(1.0_f32, Color32::from_rgb(46, 46, 46)),
        );

        // 5. Center Number Display (Segoe UI Bold)
        painter.text(
            Pos2::new(center.x, center.y - 4.0),
            egui::Align2::CENTER_CENTER,
            format!("{}", self.current_value),
            egui::FontId::proportional(32.0),
            Theme::TEXT_PRIMARY,
        );

        // 6. Sub-title Label Below Number
        let max_label_len = 24;
        let display_title = if self.title.len() > max_label_len {
            format!("{}...", &self.title[..max_label_len - 3])
        } else {
            self.title.clone()
        };

        painter.text(
            Pos2::new(center.x, rect.max.y - 14.0),
            egui::Align2::CENTER_BOTTOM,
            display_title,
            egui::FontId::proportional(12.5),
            Theme::TEXT_SECONDARY,
        );

        // 7. Left Stepper Button (‹)
        let btn_size = Vec2::splat(34.0);
        let left_btn_rect =
            Rect::from_center_size(Pos2::new(rect.min.x + 18.0, center.y), btn_size);
        let left_btn_resp = ui
            .interact(
                left_btn_rect,
                ui.id().with("dial_left"),
                egui::Sense::click(),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        let lb_hovered = left_btn_resp.hovered();
        painter.circle_filled(
            left_btn_rect.center(),
            16.0,
            if lb_hovered {
                Theme::CARD_HOVER
            } else {
                Color32::from_rgb(38, 38, 38)
            },
        );
        painter.circle_stroke(
            left_btn_rect.center(),
            16.0,
            Stroke::new(
                1.0_f32,
                if lb_hovered {
                    Theme::BORDER_HOVER
                } else {
                    Theme::BORDER_LIGHT
                },
            ),
        );
        let lc = left_btn_rect.center();
        let chevron_col = if lb_hovered {
            Theme::TEXT_PRIMARY
        } else {
            Theme::TEXT_SECONDARY
        };
        painter.line_segment(
            [
                Pos2::new(lc.x + 2.5, lc.y - 6.0),
                Pos2::new(lc.x - 3.5, lc.y),
            ],
            Stroke::new(2.0_f32, chevron_col),
        );
        painter.line_segment(
            [
                Pos2::new(lc.x - 3.5, lc.y),
                Pos2::new(lc.x + 2.5, lc.y + 6.0),
            ],
            Stroke::new(2.0_f32, chevron_col),
        );
        if left_btn_resp.clicked() {
            delta = Some(-1);
        }

        // 8. Right Stepper Button (›)
        let right_btn_rect =
            Rect::from_center_size(Pos2::new(rect.max.x - 18.0, center.y), btn_size);
        let right_btn_resp = ui
            .interact(
                right_btn_rect,
                ui.id().with("dial_right"),
                egui::Sense::click(),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        let rb_hovered = right_btn_resp.hovered();
        painter.circle_filled(
            right_btn_rect.center(),
            16.0,
            if rb_hovered {
                Theme::CARD_HOVER
            } else {
                Color32::from_rgb(38, 38, 38)
            },
        );
        painter.circle_stroke(
            right_btn_rect.center(),
            16.0,
            Stroke::new(
                1.0_f32,
                if rb_hovered {
                    Theme::BORDER_HOVER
                } else {
                    Theme::BORDER_LIGHT
                },
            ),
        );
        let rc = right_btn_rect.center();
        let chevron_col_r = if rb_hovered {
            Theme::TEXT_PRIMARY
        } else {
            Theme::TEXT_SECONDARY
        };
        painter.line_segment(
            [
                Pos2::new(rc.x - 2.5, rc.y - 6.0),
                Pos2::new(rc.x + 3.5, rc.y),
            ],
            Stroke::new(2.0_f32, chevron_col_r),
        );
        painter.line_segment(
            [
                Pos2::new(rc.x + 3.5, rc.y),
                Pos2::new(rc.x - 2.5, rc.y + 6.0),
            ],
            Stroke::new(2.0_f32, chevron_col_r),
        );
        if right_btn_resp.clicked() {
            delta = Some(1);
        }

        delta
    }
}
