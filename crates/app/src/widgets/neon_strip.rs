//! # Real-time LED Strip Monitor Widget
//!
//! Provides the simulated hardware monitor showing 60 FPS pixel strip states
//! with precision aluminum channel chassis styling, metallic terminal end-caps,
//! optical absorption well, silicone lens diodes, multi-layer concentric glow rings,
//! and dynamic node scaling for 30 to 255 addressable pixels.

use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

/// Renders the 60 FPS animated LED pixel strip monitor with precision fixture chassis.
pub fn render_neon_strip(ui: &mut egui::Ui, pixels: &[RgbColor], power: bool, compact: bool) {
    let tokens = ThemeTokens::default_dark();
    let avail_w = ui.available_width().max(360.0);
    let height = if compact { 40.0_f32 } else { 56.0_f32 };
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    // 1. Machined Obsidian/Mica Aluminum Enclosure
    painter.rect_filled(rect, tokens.radius.control_rounding(), tokens.surfaces.card);
    painter.rect_stroke(
        rect,
        tokens.radius.control_rounding(),
        Stroke::new(1.0_f32, tokens.borders.subtle),
    );

    // Specular top edge highlight for extruded chassis depth (1px inner highlight)
    painter.line_segment(
        [
            Pos2::new(rect.min.x + 8.0, rect.min.y + 1.0),
            Pos2::new(rect.max.x - 8.0, rect.min.y + 1.0),
        ],
        Stroke::new(1.0_f32, Color32::from_white_alpha(15)),
    );

    // Ambient shadow along chassis bottom
    painter.line_segment(
        [
            Pos2::new(rect.min.x + 8.0, rect.max.y - 1.0),
            Pos2::new(rect.max.x - 8.0, rect.max.y - 1.0),
        ],
        Stroke::new(1.0_f32, Color32::from_black_alpha(80)),
    );

    let count = if pixels.is_empty() {
        60
    } else {
        pixels.len().clamp(30, 255)
    };

    // 2. Fixture Status Header Bar
    let status_y = if compact {
        rect.min.y + 8.0
    } else {
        rect.min.y + 11.0
    };

    // Power / Transmission Indicator LED
    let ind_col = if power {
        tokens.semantic.success
    } else {
        tokens.text.disabled
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
        Pos2::new(rect.min.x + 26.0, status_y),
        egui::Align2::LEFT_CENTER,
        "VIRTUAL PIXEL HARDWARE MONITOR",
        TypeScale::caption(),
        tokens.text.secondary,
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
        TypeScale::caption(),
        tokens.text.tertiary,
    );

    // 3. Recessed Optical Absorption Channel (Track Bed)
    let track_rect = if compact {
        Rect::from_min_max(
            Pos2::new(rect.min.x + 12.0, rect.min.y + 18.0),
            Pos2::new(rect.max.x - 12.0, rect.max.y - 4.0),
        )
    } else {
        Rect::from_min_max(
            Pos2::new(rect.min.x + 12.0, rect.min.y + 22.0),
            Pos2::new(rect.max.x - 12.0, rect.max.y - 8.0),
        )
    };

    // Deep optical well
    painter.rect_filled(
        track_rect,
        tokens.radius.chip_rounding(),
        tokens.surfaces.well,
    );
    painter.rect_stroke(
        track_rect,
        tokens.radius.chip_rounding(),
        Stroke::new(1.0_f32, tokens.borders.subtle),
    );

    // Inset groove inner shadow
    painter.line_segment(
        [
            Pos2::new(track_rect.min.x + 4.0, track_rect.min.y + 1.2),
            Pos2::new(track_rect.max.x - 4.0, track_rect.min.y + 1.2),
        ],
        Stroke::new(1.0_f32, Color32::from_black_alpha(120)),
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
        tokens.surfaces.raised_hover,
    );
    painter.rect_stroke(
        left_term,
        Rounding {
            nw: 4.0,
            sw: 4.0,
            ne: 0.0,
            se: 0.0,
        },
        Stroke::new(1.0_f32, tokens.borders.subtle),
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
        tokens.surfaces.raised_hover,
    );
    painter.rect_stroke(
        right_term,
        Rounding {
            nw: 0.0,
            sw: 0.0,
            ne: 4.0,
            se: 4.0,
        },
        Stroke::new(1.0_f32, tokens.borders.subtle),
    );

    // Gold/Copper Busbar Contact Traces
    let cy = track_rect.center().y;
    let pad_gold = Color32::from_rgb(185, 140, 60);
    let offsets = if compact {
        [-4.0_f32, 0.0_f32, 4.0_f32]
    } else {
        [-6.0_f32, 0.0_f32, 6.0_f32]
    };
    for offset_y in offsets {
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

    // 5. Pixel Diodes (Adaptive sizing from 30 to 255 LEDs)
    let usable_w = track_rect.width() - 44.0;
    let start_x = track_rect.min.x + 22.0;

    let step = usable_w / count as f32;
    let max_radius = if compact { 3.6 } else { 5.0 };
    let radius = (step * 0.38).clamp(1.2, max_radius);

    for i in 0..count {
        let cx = start_x + (i as f32 + 0.5) * step;
        let pos = Pos2::new(cx, cy);

        let p = pixels.get(i).copied().unwrap_or(RgbColor::BLACK);
        let is_lit = power && (p.r > 15 || p.g > 15 || p.b > 15);

        if is_lit {
            // Multi-layer concentric glow rings (no blur pass, clean & non-straining)
            let glow_outer = Color32::from_rgba_premultiplied(p.r / 6, p.g / 6, p.b / 6, 20);
            let glow_inner = Color32::from_rgba_premultiplied(p.r / 3, p.g / 3, p.b / 3, 40);
            painter.circle_filled(pos, radius * 2.2, glow_outer);
            painter.circle_filled(pos, radius * 1.5, glow_inner);

            // Core solid diode
            painter.circle_filled(pos, radius, Color32::from_rgb(p.r, p.g, p.b));
        } else {
            // Off-state LEDs: subtle dark circles at ~8% alpha (never pure black or invisible)
            painter.circle_filled(pos, radius, Color32::from_white_alpha(20));
        }
    }
}
