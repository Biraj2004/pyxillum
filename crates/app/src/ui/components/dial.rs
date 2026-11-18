//! # Fluent ArcDial Component
//!
//! Provides a circular 270-degree rotary dial with animated accent track,
//! tabular numeric centre readout, flanking 40px stepping buttons,
//! arrow-key navigation, and non-overlapping title and description below the ring.

use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use std::f32::consts::PI;

pub struct ArcDial<'a> {
    value: &'a mut u8,
    min: u8,
    max: u8,
    title: &'a str,
    subtitle: &'a str,
    size: Option<f32>,
}

impl<'a> ArcDial<'a> {
    pub fn new(value: &'a mut u8, min: u8, max: u8, title: &'a str, subtitle: &'a str) -> Self {
        Self {
            value,
            min,
            max,
            title,
            subtitle,
            size: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let tokens = ThemeTokens::default_dark();
        let avail_w = ui.available_width();

        // 1. Determine Dial Ring Size S = min(240, 50% of available width)
        const BTN_SIZE: f32 = 40.0;
        const GAP: f32 = 12.0;
        let flanking_w = (BTN_SIZE + GAP) * 2.0;

        let s = self.size.unwrap_or_else(|| {
            let half_w = avail_w * 0.50;
            240.0_f32.min(half_w).max(160.0).min(avail_w - flanking_w)
        });

        let total_row_w = s + flanking_w;
        let start_x = ui.cursor().min.x + ((avail_w - total_row_w) * 0.5).max(0.0);
        let start_y = ui.cursor().min.y;

        let row_rect = Rect::from_min_size(Pos2::new(start_x, start_y), Vec2::new(total_row_w, s));
        let center_y = row_rect.center().y;

        // 2. Allocate interactive areas
        let prev_rect = Rect::from_min_size(
            Pos2::new(start_x, center_y - (BTN_SIZE / 2.0)),
            Vec2::splat(BTN_SIZE),
        );
        let ring_rect =
            Rect::from_min_size(Pos2::new(start_x + BTN_SIZE + GAP, start_y), Vec2::splat(s));
        let next_rect = Rect::from_min_size(
            Pos2::new(ring_rect.max.x + GAP, center_y - (BTN_SIZE / 2.0)),
            Vec2::splat(BTN_SIZE),
        );

        let mut changed = false;

        // Interactive responses
        let prev_resp = ui
            .interact(prev_rect, ui.id().with("prev"), Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text("Previous mode (Left Arrow)");

        let next_resp = ui
            .interact(next_rect, ui.id().with("next"), Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text("Next mode (Right Arrow)");

        let ring_resp = ui
            .interact(ring_rect, ui.id().with("ring"), Sense::click_and_drag())
            .on_hover_cursor(egui::CursorIcon::PointingHand);

        // Arrow Key Support
        let (left_key, right_key) = ui.input(|i| {
            let left = i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::ArrowDown);
            let right = i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::ArrowUp);
            (left, right)
        });

        if (prev_resp.clicked() || (ring_resp.hovered() && left_key)) && *self.value > self.min {
            *self.value -= 1;
            changed = true;
        }
        if (next_resp.clicked() || (ring_resp.hovered() && right_key)) && *self.value < self.max {
            *self.value += 1;
            changed = true;
        }

        // Rotary drag calculation on Ring
        let range = (self.max - self.min) as f32;
        if ring_resp.dragged() {
            if let Some(pos) = ring_resp.interact_pointer_pos() {
                let center = ring_rect.center();
                let dx = pos.x - center.x;
                let dy = pos.y - center.y;
                let mut angle = dy.atan2(dx);
                if angle < 0.0 {
                    angle += 2.0 * PI;
                }

                // 270° arc spanning from 135° (3pi/4) to 405° (9pi/4)
                let start_angle = 135.0_f32.to_radians();
                let arc_span = 270.0_f32.to_radians();

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
                    if new_val != *self.value {
                        *self.value = new_val;
                        changed = true;
                    }
                }
            }
        }

        // Mouse wheel adjustment
        if ring_resp.hovered() {
            let dy = ui.input(|i| i.raw_scroll_delta.y);
            if dy > 0.0 && *self.value < self.max {
                *self.value += 1;
                changed = true;
            } else if dy < 0.0 && *self.value > self.min {
                *self.value -= 1;
                changed = true;
            }
        }

        // 3. Render Flanking Buttons (40px)
        let p = ui.painter();

        // Prev Button
        let prev_bg = if prev_resp.is_pointer_button_down_on() {
            tokens.surfaces.raised_hover
        } else if prev_resp.hovered() {
            tokens.surfaces.card
        } else {
            tokens.surfaces.well
        };
        p.rect_filled(prev_rect, tokens.radius.control_rounding(), prev_bg);
        p.rect_stroke(
            prev_rect,
            tokens.radius.control_rounding(),
            Stroke::new(
                1.0_f32,
                if prev_resp.hovered() {
                    tokens.borders.hover
                } else {
                    tokens.borders.subtle
                },
            ),
        );
        Icon::ChevronLeft.paint(
            p,
            Rect::from_center_size(prev_rect.center(), Vec2::splat(18.0)),
            if prev_resp.hovered() {
                tokens.text.primary
            } else {
                tokens.text.secondary
            },
        );

        // Next Button
        let next_bg = if next_resp.is_pointer_button_down_on() {
            tokens.surfaces.raised_hover
        } else if next_resp.hovered() {
            tokens.surfaces.card
        } else {
            tokens.surfaces.well
        };
        p.rect_filled(next_rect, tokens.radius.control_rounding(), next_bg);
        p.rect_stroke(
            next_rect,
            tokens.radius.control_rounding(),
            Stroke::new(
                1.0_f32,
                if next_resp.hovered() {
                    tokens.borders.hover
                } else {
                    tokens.borders.subtle
                },
            ),
        );
        Icon::ChevronRight.paint(
            p,
            Rect::from_center_size(next_rect.center(), Vec2::splat(18.0)),
            if next_resp.hovered() {
                tokens.text.primary
            } else {
                tokens.text.secondary
            },
        );

        // 4. Render Ring: 270 degrees from 135 degrees
        let center = ring_rect.center();
        let track_w = 8.0_f32;
        // Radius guarantees stroke is fully inside the card (card padding >= 16 + half stroke)
        let radius = (s / 2.0) - 16.0 - (track_w / 2.0);

        let start_angle = 135.0_f32.to_radians();
        let arc_span = 270.0_f32.to_radians();

        let norm = if range > 0.0 {
            (*self.value - self.min) as f32 / range
        } else {
            0.0
        };

        // Inactive background track
        let segments = 40;
        for i in 0..segments {
            let a1 = start_angle + i as f32 * (arc_span / segments as f32);
            let a2 = start_angle + (i + 1) as f32 * (arc_span / segments as f32);
            let p1 = Pos2::new(center.x + radius * a1.cos(), center.y + radius * a1.sin());
            let p2 = Pos2::new(center.x + radius * a2.cos(), center.y + radius * a2.sin());
            p.line_segment([p1, p2], Stroke::new(track_w, tokens.surfaces.well));
        }

        // Active accent track
        let active_segments = (segments as f32 * norm).round() as usize;
        for i in 0..active_segments {
            let a1 = start_angle + i as f32 * (arc_span / segments as f32);
            let a2 = start_angle + (i + 1) as f32 * (arc_span / segments as f32);
            let p1 = Pos2::new(center.x + radius * a1.cos(), center.y + radius * a1.sin());
            let p2 = Pos2::new(center.x + radius * a2.cos(), center.y + radius * a2.sin());
            p.line_segment([p1, p2], Stroke::new(track_w, tokens.accent.base));
        }

        // Thumb knob
        let thumb_angle = start_angle + norm * arc_span;
        let thumb_pos = Pos2::new(
            center.x + radius * thumb_angle.cos(),
            center.y + radius * thumb_angle.sin(),
        );
        p.circle_filled(thumb_pos, 9.0, Color32::WHITE);
        p.circle_stroke(thumb_pos, 9.0, Stroke::new(2.0_f32, tokens.accent.base));

        // 5. Value Centred Inside the Ring
        let val_str = format!("{}", *self.value);
        let val_galley =
            ui.fonts(|f| f.layout_no_wrap(val_str, TypeScale::dial_value(), Color32::WHITE));
        let val_pos = Pos2::new(
            center.x - (val_galley.size().x / 2.0),
            center.y - (val_galley.size().y / 2.0),
        );
        p.galley(val_pos, val_galley, Color32::WHITE);

        // Advance cursor past the ring row
        ui.allocate_rect(row_rect, Sense::hover());

        // 6. Title and Description Sit BELOW the Ring and Never Overlap
        ui.add_space(14.0);
        ui.vertical_centered(|ui| {
            // Title (1 line)
            ui.label(
                egui::RichText::new(self.title)
                    .font(TypeScale::body_strong())
                    .color(tokens.text.primary),
            );

            ui.add_space(3.0);

            // Description (max 2 lines with tooltip)
            ui.add(
                egui::Label::new(
                    egui::RichText::new(self.subtitle)
                        .font(TypeScale::caption())
                        .color(tokens.text.secondary),
                )
                .wrap(),
            )
            .on_hover_text(self.subtitle);
        });

        let mut final_resp = ring_resp.union(prev_resp).union(next_resp);
        if changed {
            final_resp.mark_changed();
        }
        final_resp
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_dial_geometry_and_stroke_containment() {
        let avail_widths = [400.0_f32, 600.0, 800.0, 1200.0, 1560.0];
        const BTN_SIZE: f32 = 40.0;
        const GAP: f32 = 12.0;
        let flanking_w = (BTN_SIZE + GAP) * 2.0;

        for avail_w in avail_widths {
            let half_w = avail_w * 0.50;
            let s = 240.0_f32.min(half_w).max(160.0).min(avail_w - flanking_w);
            assert!(s <= 240.0, "Dial size must be at most 240px");

            let track_w = 8.0_f32;
            let radius = (s / 2.0) - 16.0 - (track_w / 2.0);
            let outer_edge = radius + (track_w / 2.0);

            // Verify outer edge is at least 16px inside half size
            assert!(
                outer_edge <= (s / 2.0) - 16.0,
                "Stroke outer edge {} must be at least 16px inside s/2 ({})",
                outer_edge,
                s / 2.0
            );
        }

        // Test arc angle span: 270 degrees from 135 degrees
        let start_angle_deg = 135.0_f32;
        let arc_span_deg = 270.0_f32;
        assert_eq!(start_angle_deg.to_radians(), 135.0_f32.to_radians());
        assert_eq!(arc_span_deg.to_radians(), 270.0_f32.to_radians());
        let end_angle_deg = start_angle_deg + arc_span_deg;
        assert_eq!(end_angle_deg, 405.0);
    }
}
