//! # Reusable Scalable Arc Dial Widget
//!
//! Provides the signature 270-degree radial arc gauge with inner skyline artwork,
//! draggable thumb indicator, center numeric readout, and flanking step buttons.

use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

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
            radius: 86.0,
        }
    }

    /// Renders the arc dial and returns an optional step delta: -1 for left, +1 for right.
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<i32> {
        let size = Vec2::new(self.radius * 2.0 + 90.0, self.radius * 2.0 + 70.0);
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        let painter = ui.painter_at(rect);
        let center = Pos2::new(rect.center().x, rect.min.y + self.radius + 20.0);
        let radius = self.radius;
        let track_thickness = 13.0_f32;

        let start_deg = 135.0f32;
        let sweep_deg = 270.0f32;
        let progress = (self.current_value as f32 / self.max_value as f32).clamp(0.01, 1.0);

        // 1. Inactive Background Arc Track (Dark Fluent Track)
        let segments = 50;
        for i in 0..segments {
            let a1 = (start_deg + sweep_deg * (i as f32 / segments as f32)).to_radians();
            let a2 = (start_deg + sweep_deg * ((i + 1) as f32 / segments as f32)).to_radians();
            let p1 = center + Vec2::new(a1.cos(), a1.sin()) * radius;
            let p2 = center + Vec2::new(a2.cos(), a2.sin()) * radius;
            painter.line_segment(
                [p1, p2],
                Stroke::new(track_thickness, Color32::from_rgb(30, 41, 59)),
            );
        }

        // 2. Active Gradient Arc Track (Electric Blue to Bright Cyan Neon)
        let active_segments = ((segments as f32 * progress) as usize).max(1);
        for i in 0..active_segments {
            let f = i as f32 / segments as f32;
            let a1 = (start_deg + sweep_deg * f).to_radians();
            let a2 = (start_deg + sweep_deg * ((i + 1) as f32 / segments as f32)).to_radians();
            let p1 = center + Vec2::new(a1.cos(), a1.sin()) * radius;
            let p2 = center + Vec2::new(a2.cos(), a2.sin()) * radius;

            let r = (37.0 * (1.0 - f) + 56.0 * f) as u8;
            let g = (99.0 * (1.0 - f) + 189.0 * f) as u8;
            let b = (235.0 * (1.0 - f) + 248.0 * f) as u8;
            let col = Color32::from_rgb(r, g, b);
            painter.line_segment([p1, p2], Stroke::new(track_thickness, col));
        }

        // 3. Thumb Indicator Cap
        let thumb_angle = (start_deg + sweep_deg * progress).to_radians();
        let thumb_pos = center + Vec2::new(thumb_angle.cos(), thumb_angle.sin()) * radius;
        painter.circle_filled(thumb_pos, track_thickness * 0.72, Color32::WHITE);
        painter.circle_stroke(
            thumb_pos,
            track_thickness * 0.72,
            Stroke::new(2.5_f32, Theme::ACCENT_CYAN),
        );

        // 4. Inner Disc
        let inner_r = radius - track_thickness - 6.0;
        painter.circle_filled(center, inner_r, Color32::from_rgb(17, 24, 39));

        // Draw geometric architectural skyline silhouette
        let base_y = center.y + inner_r * 0.52;

        let b1 = Rect::from_min_max(
            Pos2::new(center.x - 42.0, base_y - 48.0),
            Pos2::new(center.x - 22.0, base_y),
        );
        let b2 = Rect::from_min_max(
            Pos2::new(center.x - 18.0, base_y - 62.0),
            Pos2::new(center.x + 4.0, base_y),
        );
        let b3 = Rect::from_min_max(
            Pos2::new(center.x + 8.0, base_y - 42.0),
            Pos2::new(center.x + 28.0, base_y),
        );

        painter.rect_filled(b1, Rounding::same(2.0), Color32::from_rgb(30, 41, 59));
        painter.rect_filled(b2, Rounding::same(2.0), Color32::from_rgb(51, 65, 85));
        painter.rect_filled(b3, Rounding::same(2.0), Color32::from_rgb(30, 41, 59));

        // Small warm lighted windows on tallest tower
        for row in 0..4 {
            let wy = b2.min.y + 10.0 + (row as f32) * 11.0;
            let w1 = Rect::from_min_size(Pos2::new(b2.min.x + 4.0, wy), Vec2::new(4.0, 5.0));
            let w2 = Rect::from_min_size(Pos2::new(b2.min.x + 12.0, wy), Vec2::new(4.0, 5.0));
            painter.rect_filled(w1, Rounding::same(1.0), Color32::from_rgb(253, 224, 71));
            painter.rect_filled(w2, Rounding::same(1.0), Color32::from_rgb(253, 224, 71));
        }

        // Horizon curve line
        let p_start = Pos2::new(center.x - inner_r * 0.85, base_y);
        let p_end = Pos2::new(center.x + inner_r * 0.85, base_y);
        painter.line_segment(
            [p_start, p_end],
            Stroke::new(1.5_f32, Color32::from_rgb(51, 65, 85)),
        );

        // 5. Center Number Display
        painter.text(
            Pos2::new(center.x, center.y - 4.0),
            egui::Align2::CENTER_CENTER,
            format!("{}", self.current_value),
            egui::FontId::proportional(36.0),
            Theme::TEXT_PRIMARY,
        );

        // 6. Flanking Left & Right Step Buttons
        let mut delta = None;
        let left_rect = Rect::from_center_size(
            Pos2::new(center.x - radius - 24.0, center.y),
            Vec2::splat(32.0),
        );
        let right_rect = Rect::from_center_size(
            Pos2::new(center.x + radius + 24.0, center.y),
            Vec2::splat(32.0),
        );

        let left_resp = ui.interact(left_rect, ui.id().with("dial_prev"), egui::Sense::click());
        let right_resp = ui.interact(right_rect, ui.id().with("dial_next"), egui::Sense::click());

        let lp = ui.painter_at(left_rect);
        let left_bg = if left_resp.hovered() {
            Theme::CARD_HOVER
        } else {
            Color32::from_rgb(30, 41, 59)
        };
        lp.circle_filled(left_rect.center(), 15.0, left_bg);
        lp.circle_stroke(
            left_rect.center(),
            15.0,
            Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
        );
        lp.text(
            left_rect.center(),
            egui::Align2::CENTER_CENTER,
            "<",
            egui::FontId::proportional(16.0),
            Theme::TEXT_PRIMARY,
        );

        let rp = ui.painter_at(right_rect);
        let right_bg = if right_resp.hovered() {
            Theme::CARD_HOVER
        } else {
            Color32::from_rgb(30, 41, 59)
        };
        rp.circle_filled(right_rect.center(), 15.0, right_bg);
        rp.circle_stroke(
            right_rect.center(),
            15.0,
            Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
        );
        rp.text(
            right_rect.center(),
            egui::Align2::CENTER_CENTER,
            ">",
            egui::FontId::proportional(16.0),
            Theme::TEXT_PRIMARY,
        );

        if left_resp.clicked() {
            delta = Some(-1);
        }
        if right_resp.clicked() {
            delta = Some(1);
        }

        // 7. Title Below Dial
        painter.text(
            Pos2::new(center.x, rect.max.y - 6.0),
            egui::Align2::CENTER_BOTTOM,
            &self.title,
            egui::FontId::proportional(17.0),
            Theme::ACCENT_CYAN,
        );

        delta
    }
}
