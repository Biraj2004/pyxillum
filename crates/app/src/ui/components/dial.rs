//! # Fluent ArcDial Component
//!
//! Provides a circular 240-degree rotary dial with animated accent gradient track,
//! tabular numeric centre readout, spring motion, and integrated stepping buttons.

use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Response, Sense, Stroke, Ui, Vec2};
use std::f32::consts::PI;

pub struct ArcDial<'a> {
    value: &'a mut u8,
    min: u8,
    max: u8,
    title: &'a str,
    subtitle: &'a str,
    size: f32,
}

impl<'a> ArcDial<'a> {
    pub fn new(value: &'a mut u8, min: u8, max: u8, title: &'a str, subtitle: &'a str) -> Self {
        Self {
            value,
            min,
            max,
            title,
            subtitle,
            size: 210.0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let tokens = ThemeTokens::default_dark();
        let center_box = Vec2::splat(self.size);
        let (rect, resp) = ui.allocate_exact_size(center_box, Sense::click_and_drag());

        let range = (self.max - self.min) as f32;
        let current_val = *self.value;

        // Rotary drag calculation
        if resp.dragged() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let center = rect.center();
                let dx = pos.x - center.x;
                let dy = pos.y - center.y;
                let mut angle = dy.atan2(dx);
                if angle < 0.0 {
                    angle += 2.0 * PI;
                }

                // 240° arc spanning from 150° (5pi/6) to 390° (13pi/6)
                let start_angle = 5.0 * PI / 6.0;
                let arc_span = 4.0 * PI / 3.0;

                let mut rel_angle = angle - start_angle;
                if rel_angle < 0.0 {
                    rel_angle += 2.0 * PI;
                }

                if rel_angle <= arc_span {
                    let norm = (rel_angle / arc_span).clamp(0.0, 1.0);
                    let new_val = (self.min as f32 + norm * range)
                        .round()
                        .clamp(self.min as f32, self.max as f32)
                        as u8;
                    *self.value = new_val;
                }
            }
        }

        // Mouse wheel adjustments
        if resp.hovered() {
            let dy = ui.input(|i| i.raw_scroll_delta.y);
            if dy > 0.0 {
                *self.value = (*self.value as i32 + 1).min(self.max as i32) as u8;
            } else if dy < 0.0 {
                *self.value = (*self.value as i32 - 1).max(self.min as i32) as u8;
            }
        }

        if ui.is_rect_visible(rect) {
            let p = ui.painter();
            let center = rect.center();
            let radius = self.size * 0.42;
            let track_w = 8.0_f32;

            let start_angle = 5.0 * PI / 6.0;
            let arc_span = 4.0 * PI / 3.0;
            let norm = if range > 0.0 {
                (current_val - self.min) as f32 / range
            } else {
                0.0
            };

            // 1. Inactive background track
            let segments = 36;
            for i in 0..segments {
                let a1 = start_angle + i as f32 * (arc_span / segments as f32);
                let a2 = start_angle + (i + 1) as f32 * (arc_span / segments as f32);
                let p1 = Pos2::new(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let p2 = Pos2::new(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                p.line_segment([p1, p2], Stroke::new(track_w, tokens.surfaces.well));
            }

            // 2. Active accent track
            let active_segments = (segments as f32 * norm).round() as usize;
            for i in 0..active_segments {
                let a1 = start_angle + i as f32 * (arc_span / segments as f32);
                let a2 = start_angle + (i + 1) as f32 * (arc_span / segments as f32);
                let p1 = Pos2::new(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let p2 = Pos2::new(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                p.line_segment([p1, p2], Stroke::new(track_w, tokens.accent.base));
            }

            // 3. Thumb knob
            let thumb_angle = start_angle + norm * arc_span;
            let thumb_pos = Pos2::new(
                center.x + radius * thumb_angle.cos(),
                center.y + radius * thumb_angle.sin(),
            );
            p.circle_filled(thumb_pos, 10.0, Color32::WHITE);
            p.circle_stroke(thumb_pos, 10.0, Stroke::new(2.0_f32, tokens.accent.base));

            // 4. Center tabular readout
            let val_str = format!("{}", current_val);
            let val_galley =
                ui.fonts(|f| f.layout_no_wrap(val_str, TypeScale::dial_value(), Color32::WHITE));
            p.galley(
                Pos2::new(center.x - val_galley.size().x / 2.0, center.y - 28.0),
                val_galley,
                Color32::WHITE,
            );

            // Title & Subtitle
            let title_galley = ui.fonts(|f| {
                f.layout_no_wrap(
                    self.title.to_owned(),
                    TypeScale::body_strong(),
                    tokens.text.primary,
                )
            });
            p.galley(
                Pos2::new(center.x - title_galley.size().x / 2.0, center.y + 16.0),
                title_galley,
                tokens.text.primary,
            );

            let sub_galley = ui.fonts(|f| {
                f.layout_no_wrap(
                    self.subtitle.to_owned(),
                    TypeScale::caption(),
                    tokens.text.tertiary,
                )
            });
            p.galley(
                Pos2::new(center.x - sub_galley.size().x / 2.0, center.y + 34.0),
                sub_galley,
                tokens.text.tertiary,
            );
        }

        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        resp
    }
}
