//! # Real-time LED Strip Monitor Widget
//!
//! Provides the simulated hardware monitor showing 60 FPS pixel strip states
//! with precision aluminum channel chassis styling, metallic terminal end-caps,
//! optical absorption well, silicone lens diodes, and realistic apex highlights.

use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

/// Renders the 60 FPS animated LED pixel strip monitor with precision fixture chassis.
pub fn render_neon_strip(ui: &mut egui::Ui, pixels: &[RgbColor], power: bool) {
    let avail_w = ui.available_width().max(360.0);
    let height = 56.0_f32;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    // 1. Machined Obsidian/Mica Aluminum Enclosure
    painter.rect_filled(rect, Rounding::same(8.0), Color32::from_rgb(26, 26, 26));
    painter.rect_stroke(
        rect,
        Rounding::same(8.0),
        Stroke::new(1.0_f32, Color32::from_rgb(52, 52, 52)),
    );

    // Specular top edge highlight for extruded chassis depth
    painter.line_segment(
        [
            Pos2::new(rect.min.x + 8.0, rect.min.y + 0.8),
            Pos2::new(rect.max.x - 8.0, rect.min.y + 0.8),
        ],
        Stroke::new(1.0_f32, Color32::from_white_alpha(35)),
    );

    // Subtle ambient shadow along chassis bottom
    painter.line_segment(
        [
            Pos2::new(rect.min.x + 8.0, rect.max.y - 0.8),
            Pos2::new(rect.max.x - 8.0, rect.max.y - 0.8),
        ],
        Stroke::new(1.0_f32, Color32::from_black_alpha(75)),
    );

    let count = if pixels.is_empty() {
        60
    } else {
        pixels.len().min(120)
    };

    // 2. Fixture Status Header Bar
    let status_y = rect.min.y + 11.0;

    // Power / Transmission Indicator LED
    let ind_col = if power {
        Theme::ACCENT_EMERALD
    } else {
        Color32::from_rgb(85, 85, 85)
    };
    let ind_pos = Pos2::new(rect.min.x + 16.0, status_y);
    if power {
        painter.circle_filled(
            ind_pos,
            4.0,
            Color32::from_rgba_premultiplied(16, 124, 65, 55),
        );
    }
    painter.circle_filled(ind_pos, 2.4, ind_col);

    // Left Title Label
    painter.text(
        Pos2::new(rect.min.x + 24.0, status_y),
        egui::Align2::LEFT_CENTER,
        "VIRTUAL PIXEL HARDWARE MONITOR",
        egui::FontId::proportional(9.0),
        Color32::from_rgb(145, 145, 145),
    );

    // Right Protocol Status Label
    let right_status = if power {
        format!("SPI 800 kHz • {} NODES • 60 FPS", count)
    } else {
        format!("STANDBY • {} NODES • OUTPUT OFF", count)
    };
    painter.text(
        Pos2::new(rect.max.x - 14.0, status_y),
        egui::Align2::RIGHT_CENTER,
        right_status,
        egui::FontId::proportional(9.0),
        Color32::from_rgb(115, 115, 115),
    );

    // 3. Recessed Optical Absorption Channel (Track Bed)
    let track_rect = Rect::from_min_max(
        Pos2::new(rect.min.x + 12.0, rect.min.y + 22.0),
        Pos2::new(rect.max.x - 12.0, rect.max.y - 8.0),
    );

    // Deep optical well
    painter.rect_filled(
        track_rect,
        Rounding::same(5.0),
        Color32::from_rgb(10, 10, 10),
    );
    painter.rect_stroke(
        track_rect,
        Rounding::same(5.0),
        Stroke::new(1.0_f32, Color32::from_rgb(38, 38, 38)),
    );

    // Inset groove depth line
    painter.line_segment(
        [
            Pos2::new(track_rect.min.x + 5.0, track_rect.min.y + 1.2),
            Pos2::new(track_rect.max.x - 5.0, track_rect.min.y + 1.2),
        ],
        Stroke::new(1.0_f32, Color32::from_black_alpha(110)),
    );

    // 4. Hardware Metallic Terminal End-Caps (+5V, DIN/DOUT, GND Busbars)
    let term_w = 12.0_f32;
    let term_h = track_rect.height() - 2.0;

    // Left Terminal Block (DIN Input)
    let left_term = Rect::from_min_size(
        Pos2::new(track_rect.min.x + 1.0, track_rect.min.y + 1.0),
        Vec2::new(term_w, term_h),
    );
    painter.rect_filled(
        left_term,
        Rounding {
            nw: 4.0,
            sw: 4.0,
            ne: 0.0,
            se: 0.0,
        },
        Color32::from_rgb(28, 28, 28),
    );
    painter.rect_stroke(
        left_term,
        Rounding {
            nw: 4.0,
            sw: 4.0,
            ne: 0.0,
            se: 0.0,
        },
        Stroke::new(1.0_f32, Color32::from_rgb(46, 46, 46)),
    );

    // Right Terminal Block (DOUT Output)
    let right_term = Rect::from_min_size(
        Pos2::new(track_rect.max.x - 1.0 - term_w, track_rect.min.y + 1.0),
        Vec2::new(term_w, term_h),
    );
    painter.rect_filled(
        right_term,
        Rounding {
            nw: 0.0,
            sw: 0.0,
            ne: 4.0,
            se: 4.0,
        },
        Color32::from_rgb(28, 28, 28),
    );
    painter.rect_stroke(
        right_term,
        Rounding {
            nw: 0.0,
            sw: 0.0,
            ne: 4.0,
            se: 4.0,
        },
        Stroke::new(1.0_f32, Color32::from_rgb(46, 46, 46)),
    );

    // Gold/Copper Busbar Contact Traces
    let cy = track_rect.center().y;
    let pad_gold = Color32::from_rgb(185, 140, 60);
    for offset_y in [-6.0_f32, 0.0_f32, 6.0_f32] {
        painter.circle_filled(
            Pos2::new(left_term.center().x, cy + offset_y),
            1.4,
            pad_gold,
        );
        painter.circle_filled(
            Pos2::new(right_term.center().x, cy + offset_y),
            1.4,
            pad_gold,
        );
    }

    // 5. Pixel Diodes & Continuous Diffuser Light Guide
    let usable_w = track_rect.width() - 44.0;
    let start_x = track_rect.min.x + 22.0;

    if pixels.is_empty() {
        // Standby unlit SMD beads
        let dot_count = (usable_w / 24.0) as usize;
        let spacing = usable_w / (dot_count + 1) as f32;
        for i in 1..=dot_count {
            let cx = start_x + (i as f32) * spacing;
            let pos = Pos2::new(cx, cy);
            painter.circle_filled(pos, 3.2, Color32::from_rgb(34, 34, 34));
            painter.circle_stroke(
                pos,
                3.2,
                Stroke::new(0.8_f32, Color32::from_rgb(46, 46, 46)),
            );
        }
        return;
    }

    let step = usable_w / count as f32;
    let radius = (step * 0.40).clamp(2.8, 5.2);

    // Continuous frosted light guide glow when powered
    if power {
        let guide_rect = Rect::from_center_size(
            Pos2::new(track_rect.center().x, cy),
            Vec2::new(usable_w + radius * 2.0, radius * 2.6),
        );
        painter.rect_filled(
            guide_rect,
            Rounding::same(radius * 1.3),
            Color32::from_white_alpha(10),
        );
    }

    for (i, p) in pixels.iter().take(count).enumerate() {
        let cx = start_x + (i as f32 + 0.5) * step;
        let pos = Pos2::new(cx, cy);

        if power {
            let col = Color32::from_rgb(p.r, p.g, p.b);

            // Layer 1: Ambient soft bloom halo
            if p.r > 15 || p.g > 15 || p.b > 15 {
                let bloom_col = Color32::from_rgba_premultiplied(p.r / 7, p.g / 7, p.b / 7, 24);
                painter.circle_filled(pos, radius * 2.3, bloom_col);
            }

            // Layer 2: Frosted silicone dome lens
            let lens_col = Color32::from_rgba_premultiplied(
                (p.r as u16 + 50).min(255) as u8,
                (p.g as u16 + 50).min(255) as u8,
                (p.b as u16 + 50).min(255) as u8,
                80,
            );
            painter.circle_filled(pos, radius * 1.25, lens_col);

            // Layer 3: Core Phosphor Diode Bead
            painter.circle_filled(pos, radius, col);

            // Layer 4: 3D Dome Specular Highlight
            let glint_pos = Pos2::new(cx - radius * 0.28, cy - radius * 0.28);
            painter.circle_filled(glint_pos, radius * 0.32, Color32::from_white_alpha(165));
        } else {
            // Unlit physical SMD pixel diode
            painter.circle_filled(pos, radius, Color32::from_rgb(34, 34, 34));
            painter.circle_stroke(
                pos,
                radius,
                Stroke::new(0.8_f32, Color32::from_rgb(48, 48, 48)),
            );
            let glint_pos = Pos2::new(cx - radius * 0.25, cy - radius * 0.25);
            painter.circle_filled(glint_pos, radius * 0.28, Color32::from_white_alpha(35));
        }
    }
}
