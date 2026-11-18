//! # Resolution-Independent Vector UI Icons
//!
//! Provides pure vector-drawn icons using `egui::Painter` for all sidebar navigation,
//! action bars, and modal controls. Completely eliminates missing Unicode glyphs and
//! Windows font tofu boxes (`☐`).

#![allow(dead_code)]

use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use std::f32::consts::PI;

/// Sidebar and navigation item icons.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavIcon {
    DynamicFx,
    SolidColour,
    MusicRhythm,
    DiyCustom,
    DeviceHub,
    Settings,
}

/// Dynamic action bar icons.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionIcon {
    None,
    Loop(bool), // true = loop all, false = once
    Star(bool), // true = filled, false = outline
    List,
}

/// Paints the high-tech Pyxillum app branding emblem: a 3x3 addressable LED matrix cluster.
pub fn paint_pyxillum_logo(p: &egui::Painter, rect: Rect) {
    // Fluent Blue container with subtle 1px cyan highlight
    p.rect_filled(rect, Rounding::same(7.0), Theme::ACCENT_BLUE);
    p.rect_stroke(
        rect,
        Rounding::same(7.0),
        Stroke::new(1.0_f32, Color32::from_rgb(76, 194, 255)),
    );

    let center = rect.center();
    let step = 5.2_f32;

    // 3x3 Pixel LED Matrix
    for row in -1..=1 {
        for col in -1..=1 {
            let pt = Pos2::new(center.x + col as f32 * step, center.y + row as f32 * step);
            if row == 0 && col == 0 {
                // Central Master LED (Blazing White)
                p.circle_filled(pt, 2.4, Color32::WHITE);
            } else if row == 0 || col == 0 {
                // Cardinal LEDs (Vibrant Cyan)
                p.circle_filled(pt, 1.8, Color32::from_rgb(175, 235, 255));
            } else {
                // Corner LEDs (Deep Cyan Glow)
                p.circle_filled(pt, 1.3, Color32::from_rgb(90, 180, 240));
            }
        }
    }
}

/// Paints the 4-point sparkle star cluster (Dynamic FX).
pub fn paint_sparkle_icon(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let r_out = size * 0.44;
    let r_in = size * 0.13;

    // Primary 4-point sparkle formed by two overlapping convex diamonds
    let vert_diamond = vec![
        Pos2::new(center.x, center.y - r_out),
        Pos2::new(center.x + r_in, center.y),
        Pos2::new(center.x, center.y + r_out),
        Pos2::new(center.x - r_in, center.y),
    ];
    let horiz_diamond = vec![
        Pos2::new(center.x - r_out, center.y),
        Pos2::new(center.x, center.y - r_in),
        Pos2::new(center.x + r_out, center.y),
        Pos2::new(center.x, center.y + r_in),
    ];

    p.add(egui::Shape::convex_polygon(
        vert_diamond,
        color,
        Stroke::NONE,
    ));
    p.add(egui::Shape::convex_polygon(
        horiz_diamond,
        color,
        Stroke::NONE,
    ));

    // Secondary micro-sparkle at top-right
    let c2 = Pos2::new(center.x + size * 0.34, center.y - size * 0.30);
    let r2_out = size * 0.20;
    let r2_in = size * 0.06;

    let v2 = vec![
        Pos2::new(c2.x, c2.y - r2_out),
        Pos2::new(c2.x + r2_in, c2.y),
        Pos2::new(c2.x, c2.y + r2_out),
        Pos2::new(c2.x - r2_in, c2.y),
    ];
    let h2 = vec![
        Pos2::new(c2.x - r2_out, c2.y),
        Pos2::new(c2.x, c2.y - r2_in),
        Pos2::new(c2.x + r2_out, c2.y),
        Pos2::new(c2.x, c2.y + r2_in),
    ];
    p.add(egui::Shape::convex_polygon(v2, color, Stroke::NONE));
    p.add(egui::Shape::convex_polygon(h2, color, Stroke::NONE));
}

/// Paints the chromatic colour palette icon with pigment swatches (Solid Colour).
pub fn paint_palette_icon(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let r = size * 0.42;

    // Palette disc contour
    p.circle_stroke(center, r, Stroke::new(1.4_f32, color));

    // Three chromatic pigment dots (Red, Green, Blue) positioned along top-left arc
    let dot_r = 1.6_f32;
    let dot_offset = r * 0.52;

    let red_dot = Pos2::new(center.x - dot_offset, center.y - dot_offset * 0.5);
    let green_dot = Pos2::new(center.x - dot_offset * 0.1, center.y - dot_offset);
    let blue_dot = Pos2::new(center.x + dot_offset * 0.7, center.y - dot_offset * 0.6);

    p.circle_filled(red_dot, dot_r, Color32::from_rgb(255, 90, 90));
    p.circle_filled(green_dot, dot_r, Color32::from_rgb(90, 240, 90));
    p.circle_filled(blue_dot, dot_r, Color32::from_rgb(90, 160, 255));

    // Thumb hole cutout at bottom-right
    let thumb_center = Pos2::new(center.x + dot_offset * 0.45, center.y + dot_offset * 0.45);
    p.circle_stroke(thumb_center, 2.2, Stroke::new(1.2_f32, color));
}

/// Paints the 4-bar audio spectrum equalizer (Music Rhythm).
pub fn paint_equalizer_icon(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let bar_w = 2.2_f32;
    let gap = 2.0_f32;
    let total_w = 4.0 * bar_w + 3.0 * gap;
    let start_x = center.x - total_w / 2.0 + bar_w / 2.0;
    let base_y = center.y + size * 0.38;

    // 4 vertical bars with authentic live audio spectrum heights
    let heights = [
        size * 0.48, // Bar 1 (Bass)
        size * 0.78, // Bar 2 (Mid-High Peak)
        size * 0.62, // Bar 3 (Vocal Mid)
        size * 0.36, // Bar 4 (Treble)
    ];

    for (i, &h) in heights.iter().enumerate() {
        let x = start_x + i as f32 * (bar_w + gap);
        let top_y = base_y - h;
        let rect = Rect::from_min_max(
            Pos2::new(x - bar_w / 2.0, top_y),
            Pos2::new(x + bar_w / 2.0, base_y),
        );
        p.rect_filled(rect, Rounding::same(1.1), color);
    }
}

/// Paints the 3-track timeline node sequencer (DIY Custom).
pub fn paint_sequencer_icon(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let half_w = size * 0.42;
    let stroke = Stroke::new(1.2_f32, color);
    let y_step = size * 0.30;

    // Track 1 (Top)
    let y1 = center.y - y_step;
    p.line_segment(
        [
            Pos2::new(center.x - half_w, y1),
            Pos2::new(center.x + half_w, y1),
        ],
        stroke,
    );
    p.circle_filled(Pos2::new(center.x - half_w * 0.35, y1), 2.2, color);

    // Track 2 (Center)
    let y2 = center.y;
    p.line_segment(
        [
            Pos2::new(center.x - half_w, y2),
            Pos2::new(center.x + half_w, y2),
        ],
        stroke,
    );
    p.circle_filled(Pos2::new(center.x + half_w * 0.45, y2), 2.2, color);

    // Track 3 (Bottom)
    let y3 = center.y + y_step;
    p.line_segment(
        [
            Pos2::new(center.x - half_w, y3),
            Pos2::new(center.x + half_w, y3),
        ],
        stroke,
    );
    p.circle_filled(Pos2::new(center.x - half_w * 0.15, y3), 2.2, color);
}

/// Paints the official Bluetooth rune (Devices & Pairing).
pub fn paint_bluetooth_icon(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let h = size * 0.82;
    let w = size * 0.40;
    let stroke = Stroke::new(1.6_f32, color);

    let top = Pos2::new(center.x, center.y - h * 0.5);
    let bot = Pos2::new(center.x, center.y + h * 0.5);
    let mid = Pos2::new(center.x, center.y);

    let right_top = Pos2::new(center.x + w, center.y - h * 0.25);
    let right_bot = Pos2::new(center.x + w, center.y + h * 0.25);

    let left_top = Pos2::new(center.x - w * 0.85, center.y - h * 0.25);
    let left_bot = Pos2::new(center.x - w * 0.85, center.y + h * 0.25);

    // Central vertical spine
    p.line_segment([top, bot], stroke);

    // Upper chevron loop
    p.line_segment([left_bot, right_top], stroke);
    p.line_segment([right_top, mid], stroke);

    // Lower chevron loop
    p.line_segment([left_top, right_bot], stroke);
    p.line_segment([right_bot, mid], stroke);
}

/// Paints the settings cog / gear icon (Settings & Hardware).
pub fn paint_gear_icon(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new(1.4_f32, color);
    let r_ring = size * 0.30;
    let r_inner = size * 0.14;
    let r_tooth_out = size * 0.44;

    // Center circular ring
    p.circle_stroke(center, r_ring, stroke);
    p.circle_stroke(center, r_inner, Stroke::new(1.0_f32, color));

    // 6 Radiating gear teeth
    for i in 0..6 {
        let angle = i as f32 * (PI / 3.0);
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let pt1 = Pos2::new(center.x + cos_a * r_ring, center.y + sin_a * r_ring);
        let pt2 = Pos2::new(
            center.x + cos_a * r_tooth_out,
            center.y + sin_a * r_tooth_out,
        );
        p.line_segment([pt1, pt2], Stroke::new(2.4_f32, color));
    }
}

/// Paints a left chevron breadcrumb (‹ Devices).
pub fn paint_chevron_left(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let w = size * 0.35;
    let h = size * 0.50;
    let stroke = Stroke::new(1.6_f32, color);

    p.line_segment(
        [
            Pos2::new(center.x + w, center.y - h),
            Pos2::new(center.x - w, center.y),
        ],
        stroke,
    );
    p.line_segment(
        [
            Pos2::new(center.x - w, center.y),
            Pos2::new(center.x + w, center.y + h),
        ],
        stroke,
    );
}

/// Paints the loop / once circular arrow icon.
pub fn paint_loop_icon(
    p: &egui::Painter,
    center: Pos2,
    size: f32,
    is_loop_all: bool,
    color: Color32,
) {
    let r = size * 0.38;
    let stroke = Stroke::new(1.5_f32, color);

    // Draw ~270 degree circular arc with line segments
    let num_segments = 14;
    let start_angle = -PI * 0.75;
    let end_angle = PI * 0.85;

    for i in 0..num_segments {
        let t1 = i as f32 / num_segments as f32;
        let t2 = (i + 1) as f32 / num_segments as f32;
        let a1 = start_angle + (end_angle - start_angle) * t1;
        let a2 = start_angle + (end_angle - start_angle) * t2;

        let p1 = Pos2::new(center.x + a1.cos() * r, center.y + a1.sin() * r);
        let p2 = Pos2::new(center.x + a2.cos() * r, center.y + a2.sin() * r);
        p.line_segment([p1, p2], stroke);
    }

    // Arrowhead at end of arc
    let arrow_tip = Pos2::new(
        center.x + end_angle.cos() * r,
        center.y + end_angle.sin() * r,
    );
    let p_a1 = Pos2::new(arrow_tip.x - 3.5, arrow_tip.y - 4.5);
    let p_a2 = Pos2::new(arrow_tip.x + 4.0, arrow_tip.y - 1.5);
    p.line_segment([p_a1, arrow_tip], stroke);
    p.line_segment([p_a2, arrow_tip], stroke);

    if !is_loop_all {
        // Small "1" in the center for "Once" mode
        let one_top = Pos2::new(center.x, center.y - 3.5);
        let one_bot = Pos2::new(center.x, center.y + 3.5);
        p.line_segment([one_top, one_bot], Stroke::new(1.4_f32, color));
        p.line_segment(
            [Pos2::new(center.x - 1.8, center.y - 1.8), one_top],
            Stroke::new(1.2_f32, color),
        );
    }
}

/// Paints the 5-point favourite star icon (outline or solid filled).
pub fn paint_star_icon(p: &egui::Painter, center: Pos2, size: f32, filled: bool, color: Color32) {
    let r_out = size * 0.44;
    let r_in = size * 0.19;

    let mut pts = Vec::with_capacity(10);
    for i in 0..10 {
        let angle = -PI * 0.5 + i as f32 * (PI / 5.0);
        let r = if i % 2 == 0 { r_out } else { r_in };
        pts.push(Pos2::new(
            center.x + angle.cos() * r,
            center.y + angle.sin() * r,
        ));
    }

    if filled {
        // Render filled 5-point star as 10 triangular fans from centroid
        for i in 0..10 {
            let next_i = (i + 1) % 10;
            let tri = vec![center, pts[i], pts[next_i]];
            p.add(egui::Shape::convex_polygon(tri, color, Stroke::NONE));
        }
    } else {
        // Render star outline
        let stroke = Stroke::new(1.3_f32, color);
        for i in 0..10 {
            let next_i = (i + 1) % 10;
            p.line_segment([pts[i], pts[next_i]], stroke);
        }
    }
}

/// Paints the catalog / presets 3-bar list icon.
pub fn paint_list_icon(p: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let bar_w = size * 0.68;
    let bar_h = 1.8_f32;
    let gap = 3.2_f32;

    for i in -1..=1 {
        let y = center.y + i as f32 * (bar_h + gap);
        let rect = Rect::from_center_size(Pos2::new(center.x, y), Vec2::new(bar_w, bar_h));
        p.rect_filled(rect, Rounding::same(0.9), color);
    }
}

/// Unified dispatcher to paint any navigation icon.
pub fn paint_nav_icon(p: &egui::Painter, icon: NavIcon, center: Pos2, size: f32, color: Color32) {
    match icon {
        NavIcon::DynamicFx => paint_sparkle_icon(p, center, size, color),
        NavIcon::SolidColour => paint_palette_icon(p, center, size, color),
        NavIcon::MusicRhythm => paint_equalizer_icon(p, center, size, color),
        NavIcon::DiyCustom => paint_sequencer_icon(p, center, size, color),
        NavIcon::DeviceHub => paint_bluetooth_icon(p, center, size, color),
        NavIcon::Settings => paint_gear_icon(p, center, size, color),
    }
}
