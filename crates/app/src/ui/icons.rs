//! # Microsoft Fluent System Icons Vector Suite
//!
//! Provides mathematically precise vector drawing of Microsoft Fluent System Icons
//! (Regular and Filled variants). Eliminates Unicode font fallbacks, ensures subpixel
//! physical alignment across 100/125/150/200% DPI scales, and provides standard alignment
//! helpers for labels, buttons, and navigation rows.

use eframe::egui::{
    self, Color32, Painter, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2,
};
use std::f32::consts::PI;

/// Canonical Microsoft Fluent System Icons for Pyxillum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    // Navigation
    Effects,
    Palette,
    MusicBars,
    DiySliders,
    Devices,

    // Header & Global Actions
    Settings,
    HardwareChip,
    Back,
    ChevronLeft,
    ChevronRight,
    ChevronDown,
    ChevronUp,
    Play,
    Pause,
    Once,
    Star,
    StarFilled,
    Search,
    Clear,
    Close,
    Copy,
    Bluetooth,
    BluetoothOff,
    Warning,
    Info,
    Success,
    Error,
    Scan,
    Spinner,
    Power,
    Mic,
    Speaker,
    Add,
    Remove,
    Duplicate,
    Reverse,
    Delete,
}

impl Icon {
    /// Renders this icon within the given bounding box using the provided painter and color.
    pub fn paint(&self, p: &Painter, rect: Rect, color: Color32) {
        let size = rect.width().min(rect.height());
        let center = rect.center();
        let stroke_w = (size * 0.08).clamp(1.2, 2.0);
        let stroke = Stroke::new(stroke_w, color);

        match self {
            Icon::Effects => paint_effects(p, center, size, color),
            Icon::Palette => paint_palette(p, center, size, stroke, color),
            Icon::MusicBars => paint_music_bars(p, center, size, color),
            Icon::DiySliders => paint_diy_sliders(p, center, size, stroke, color),
            Icon::Devices => paint_devices(p, center, size, stroke),
            Icon::Settings => paint_settings(p, center, size, stroke),
            Icon::HardwareChip => paint_hardware_chip(p, center, size, stroke, color),
            Icon::Back | Icon::ChevronLeft => {
                paint_chevron(p, center, size, stroke, Direction::Left)
            }
            Icon::ChevronRight => paint_chevron(p, center, size, stroke, Direction::Right),
            Icon::ChevronDown => paint_chevron(p, center, size, stroke, Direction::Down),
            Icon::ChevronUp => paint_chevron(p, center, size, stroke, Direction::Up),
            Icon::Play => paint_play(p, center, size, color),
            Icon::Pause => paint_pause(p, center, size, color),
            Icon::Once => paint_repeat_once(p, center, size, stroke),
            Icon::Star => paint_star(p, center, size, stroke, false),
            Icon::StarFilled => paint_star(p, center, size, stroke, true),
            Icon::Search => paint_search(p, center, size, stroke),
            Icon::Clear | Icon::Close => paint_close(p, center, size, stroke),
            Icon::Copy => paint_copy(p, center, size, stroke),
            Icon::Bluetooth => paint_bluetooth(p, center, size, stroke, false),
            Icon::BluetoothOff => paint_bluetooth(p, center, size, stroke, true),
            Icon::Warning => paint_warning(p, center, size, stroke, color),
            Icon::Info => paint_info(p, center, size, stroke, color),
            Icon::Success => paint_success(p, center, size, stroke),
            Icon::Error => paint_error(p, center, size, stroke, color),
            Icon::Scan => paint_scan(p, center, size, stroke),
            Icon::Spinner => paint_spinner(p, center, size, stroke),
            Icon::Power => paint_power(p, center, size, stroke),
            Icon::Mic => paint_mic(p, center, size, stroke, color),
            Icon::Speaker => paint_speaker(p, center, size, stroke, color),
            Icon::Add => paint_add(p, center, size, stroke),
            Icon::Remove => paint_remove(p, center, size, stroke),
            Icon::Duplicate => paint_duplicate(p, center, size, stroke),
            Icon::Reverse => paint_reverse(p, center, size, stroke),
            Icon::Delete => paint_delete(p, center, size, stroke),
        }
    }
}

/// Snaps a rectangle to integer physical device pixels for zero blurriness.
#[inline]
pub fn snap_rect_to_pixels(rect: Rect, ppp: f32) -> Rect {
    let min_x = (rect.min.x * ppp).round() / ppp;
    let min_y = (rect.min.y * ppp).round() / ppp;
    let max_x = (rect.max.x * ppp).round() / ppp;
    let max_y = (rect.max.y * ppp).round() / ppp;
    Rect::from_min_max(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y))
}

// ====================================================================
// VECTOR RENDERING IMPLEMENTATIONS
// ====================================================================

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

fn paint_effects(p: &Painter, center: Pos2, size: f32, color: Color32) {
    let r_out = size * 0.44;
    let r_in = size * 0.12;

    // 4-point sparkle diamond
    let points = vec![
        Pos2::new(center.x, center.y - r_out),
        Pos2::new(center.x + r_in, center.y - r_in),
        Pos2::new(center.x + r_out, center.y),
        Pos2::new(center.x + r_in, center.y + r_in),
        Pos2::new(center.x, center.y + r_out),
        Pos2::new(center.x - r_in, center.y + r_in),
        Pos2::new(center.x - r_out, center.y),
        Pos2::new(center.x - r_in, center.y - r_in),
    ];
    p.add(egui::epaint::PathShape::convex_polygon(
        points,
        color,
        Stroke::NONE,
    ));

    // Secondary smaller sparkle in top-right
    let c2 = Pos2::new(center.x + size * 0.30, center.y - size * 0.30);
    let r2_out = size * 0.16;
    let r2_in = size * 0.05;
    let points2 = vec![
        Pos2::new(c2.x, c2.y - r2_out),
        Pos2::new(c2.x + r2_in, c2.y - r2_in),
        Pos2::new(c2.x + r2_out, c2.y),
        Pos2::new(c2.x + r2_in, c2.y + r2_in),
        Pos2::new(c2.x, c2.y + r2_out),
        Pos2::new(c2.x - r2_in, c2.y + r2_in),
        Pos2::new(c2.x - r2_out, c2.y),
        Pos2::new(c2.x - r2_in, c2.y - r2_in),
    ];
    p.add(egui::epaint::PathShape::convex_polygon(
        points2,
        color,
        Stroke::NONE,
    ));
}

fn paint_palette(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let r = size * 0.40;
    p.circle_stroke(center, r, stroke);

    // Three pigment wells
    let r_dot = size * 0.07;
    p.circle_filled(
        Pos2::new(center.x - r * 0.35, center.y - r * 0.25),
        r_dot,
        color,
    );
    p.circle_filled(
        Pos2::new(center.x + r * 0.30, center.y - r * 0.30),
        r_dot,
        color,
    );
    p.circle_filled(
        Pos2::new(center.x - r * 0.30, center.y + r * 0.25),
        r_dot,
        color,
    );

    // Thumb hole
    p.circle_stroke(
        Pos2::new(center.x + r * 0.30, center.y + r * 0.25),
        r * 0.20,
        stroke,
    );
}

fn paint_music_bars(p: &Painter, center: Pos2, size: f32, color: Color32) {
    let w = (size * 0.14).max(2.0);
    let gap = size * 0.08;
    let total_w = w * 4.0 + gap * 3.0;
    let start_x = center.x - total_w / 2.0 + w / 2.0;

    let heights = [0.45, 0.85, 0.60, 0.95];
    for (i, &h_rel) in heights.iter().enumerate() {
        let x = start_x + i as f32 * (w + gap);
        let h = size * 0.70 * h_rel;
        let rect = Rect::from_center_size(
            Pos2::new(x, center.y + (size * 0.35 - h / 2.0)),
            Vec2::new(w, h),
        );
        p.rect_filled(rect, Rounding::same(w / 2.0), color);
    }
}

fn paint_diy_sliders(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let w = size * 0.75;
    let h_span = size * 0.50;
    let y1 = center.y - h_span / 2.0;
    let y2 = center.y + h_span / 2.0;
    let x_min = center.x - w / 2.0;
    let x_max = center.x + w / 2.0;

    // Track 1
    p.line_segment([Pos2::new(x_min, y1), Pos2::new(x_max, y1)], stroke);
    let thumb1_x = center.x - w * 0.15;
    p.circle_filled(Pos2::new(thumb1_x, y1), size * 0.14, color);

    // Track 2
    p.line_segment([Pos2::new(x_min, y2), Pos2::new(x_max, y2)], stroke);
    let thumb2_x = center.x + w * 0.20;
    p.circle_filled(Pos2::new(thumb2_x, y2), size * 0.14, color);
}

fn paint_devices(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    // Monitor / Laptop frame
    let w = size * 0.72;
    let h = size * 0.48;
    let screen =
        Rect::from_center_size(Pos2::new(center.x, center.y - size * 0.06), Vec2::new(w, h));
    p.rect_stroke(screen, Rounding::same(size * 0.08), stroke);

    // Stand base
    let base_y = screen.bottom() + size * 0.14;
    p.line_segment(
        [
            Pos2::new(center.x - size * 0.08, screen.bottom()),
            Pos2::new(center.x - size * 0.14, base_y),
        ],
        stroke,
    );
    p.line_segment(
        [
            Pos2::new(center.x + size * 0.08, screen.bottom()),
            Pos2::new(center.x + size * 0.14, base_y),
        ],
        stroke,
    );
    p.line_segment(
        [
            Pos2::new(center.x - size * 0.25, base_y),
            Pos2::new(center.x + size * 0.25, base_y),
        ],
        stroke,
    );
}

fn paint_settings(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let r = size * 0.32;
    p.circle_stroke(center, r, stroke);

    // 6 cogs
    for i in 0..6 {
        let angle = i as f32 * (PI / 3.0);
        let p_in = Pos2::new(
            center.x + (r - 0.5) * angle.cos(),
            center.y + (r - 0.5) * angle.sin(),
        );
        let p_out = Pos2::new(
            center.x + (r + size * 0.10) * angle.cos(),
            center.y + (r + size * 0.10) * angle.sin(),
        );
        p.line_segment([p_in, p_out], stroke);
    }
}

fn paint_hardware_chip(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let chip_w = size * 0.48;
    let chip = Rect::from_center_size(center, Vec2::splat(chip_w));
    p.rect_filled(chip, Rounding::same(size * 0.06), color);
    p.rect_stroke(chip, Rounding::same(size * 0.06), stroke);

    // Pins extending on all 4 sides
    let pin_len = size * 0.12;
    for side in 0..4 {
        for slot in [-0.25, 0.25] {
            let offset = chip_w * slot;
            let (p1, p2) = match side {
                0 => (
                    Pos2::new(center.x + offset, chip.top()),
                    Pos2::new(center.x + offset, chip.top() - pin_len),
                ),
                1 => (
                    Pos2::new(center.x + offset, chip.bottom()),
                    Pos2::new(center.x + offset, chip.bottom() + pin_len),
                ),
                2 => (
                    Pos2::new(chip.left(), center.y + offset),
                    Pos2::new(chip.left() - pin_len, center.y + offset),
                ),
                _ => (
                    Pos2::new(chip.right(), center.y + offset),
                    Pos2::new(chip.right() + pin_len, center.y + offset),
                ),
            };
            p.line_segment([p1, p2], stroke);
        }
    }
}

fn paint_chevron(p: &Painter, center: Pos2, size: f32, stroke: Stroke, dir: Direction) {
    let d = size * 0.22;
    let (p1, p2, p3) = match dir {
        Direction::Left => (
            Pos2::new(center.x + d * 0.5, center.y - d),
            Pos2::new(center.x - d * 0.5, center.y),
            Pos2::new(center.x + d * 0.5, center.y + d),
        ),
        Direction::Right => (
            Pos2::new(center.x - d * 0.5, center.y - d),
            Pos2::new(center.x + d * 0.5, center.y),
            Pos2::new(center.x - d * 0.5, center.y + d),
        ),
        Direction::Down => (
            Pos2::new(center.x - d, center.y - d * 0.5),
            Pos2::new(center.x, center.y + d * 0.5),
            Pos2::new(center.x + d, center.y - d * 0.5),
        ),
        Direction::Up => (
            Pos2::new(center.x - d, center.y + d * 0.5),
            Pos2::new(center.x, center.y - d * 0.5),
            Pos2::new(center.x + d, center.y + d * 0.5),
        ),
    };
    p.line_segment([p1, p2], stroke);
    p.line_segment([p2, p3], stroke);
}

fn paint_play(p: &Painter, center: Pos2, size: f32, color: Color32) {
    let w = size * 0.40;
    let h = size * 0.48;
    let x_left = center.x - w / 2.0 + 1.0;
    let points = vec![
        Pos2::new(x_left, center.y - h / 2.0),
        Pos2::new(center.x + w / 2.0 + 1.0, center.y),
        Pos2::new(x_left, center.y + h / 2.0),
    ];
    p.add(egui::epaint::PathShape::convex_polygon(
        points,
        color,
        Stroke::NONE,
    ));
}

fn paint_pause(p: &Painter, center: Pos2, size: f32, color: Color32) {
    let bar_w = (size * 0.12).max(2.0);
    let bar_h = size * 0.48;
    let gap = size * 0.14;
    let b1 = Rect::from_center_size(
        Pos2::new(center.x - gap / 2.0 - bar_w / 2.0, center.y),
        Vec2::new(bar_w, bar_h),
    );
    let b2 = Rect::from_center_size(
        Pos2::new(center.x + gap / 2.0 + bar_w / 2.0, center.y),
        Vec2::new(bar_w, bar_h),
    );
    p.rect_filled(b1, Rounding::same(1.0), color);
    p.rect_filled(b2, Rounding::same(1.0), color);
}

fn paint_repeat_once(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let r = size * 0.32;
    p.circle_stroke(center, r, stroke);
    // Inner digit 1
    p.line_segment(
        [
            Pos2::new(center.x, center.y - r * 0.5),
            Pos2::new(center.x, center.y + r * 0.5),
        ],
        stroke,
    );
}

fn paint_star(p: &Painter, center: Pos2, size: f32, stroke: Stroke, filled: bool) {
    let mut points = Vec::new();
    let r_out = size * 0.42;
    let r_in = size * 0.18;
    for i in 0..10 {
        let angle = i as f32 * (PI / 5.0) - PI / 2.0;
        let r = if i % 2 == 0 { r_out } else { r_in };
        points.push(Pos2::new(
            center.x + r * angle.cos(),
            center.y + r * angle.sin(),
        ));
    }
    if filled {
        p.add(egui::epaint::PathShape::convex_polygon(
            points,
            stroke.color,
            Stroke::NONE,
        ));
    } else {
        p.add(egui::epaint::PathShape::closed_line(points, stroke));
    }
}

fn paint_search(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let r = size * 0.28;
    let lens_c = Pos2::new(center.x - size * 0.08, center.y - size * 0.08);
    p.circle_stroke(lens_c, r, stroke);
    let handle_start = Pos2::new(lens_c.x + r * 0.707, lens_c.y + r * 0.707);
    let handle_end = Pos2::new(center.x + size * 0.36, center.y + size * 0.36);
    p.line_segment([handle_start, handle_end], stroke);
}

fn paint_close(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let d = size * 0.28;
    p.line_segment(
        [
            Pos2::new(center.x - d, center.y - d),
            Pos2::new(center.x + d, center.y + d),
        ],
        stroke,
    );
    p.line_segment(
        [
            Pos2::new(center.x + d, center.y - d),
            Pos2::new(center.x - d, center.y + d),
        ],
        stroke,
    );
}

fn paint_copy(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let doc_w = size * 0.42;
    let doc_h = size * 0.52;
    let btm_doc = Rect::from_center_size(
        Pos2::new(center.x + size * 0.08, center.y + size * 0.08),
        Vec2::new(doc_w, doc_h),
    );
    p.rect_stroke(btm_doc, Rounding::same(2.0), stroke);
    let top_doc = Rect::from_center_size(
        Pos2::new(center.x - size * 0.08, center.y - size * 0.08),
        Vec2::new(doc_w, doc_h),
    );
    p.rect_stroke(top_doc, Rounding::same(2.0), stroke);
}

fn paint_bluetooth(p: &Painter, center: Pos2, size: f32, stroke: Stroke, disabled: bool) {
    let h = size * 0.65;
    let w = size * 0.32;
    let top = Pos2::new(center.x, center.y - h / 2.0);
    let bot = Pos2::new(center.x, center.y + h / 2.0);
    p.line_segment([top, bot], stroke);

    let right_top = Pos2::new(center.x + w, center.y - h * 0.25);
    let right_bot = Pos2::new(center.x + w, center.y + h * 0.25);
    let left_top = Pos2::new(center.x - w * 0.6, center.y - h * 0.20);
    let left_bot = Pos2::new(center.x - w * 0.6, center.y + h * 0.20);

    p.line_segment([left_bot, right_top], stroke);
    p.line_segment([right_top, top], stroke);
    p.line_segment([left_top, right_bot], stroke);
    p.line_segment([right_bot, bot], stroke);

    if disabled {
        let slash_stroke = Stroke::new(stroke.width * 1.2, Color32::from_rgb(232, 17, 35));
        p.line_segment(
            [
                Pos2::new(center.x - size * 0.40, center.y - size * 0.40),
                Pos2::new(center.x + size * 0.40, center.y + size * 0.40),
            ],
            slash_stroke,
        );
    }
}

fn paint_warning(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let h = size * 0.70;
    let w = size * 0.76;
    let points = vec![
        Pos2::new(center.x, center.y - h / 2.0),
        Pos2::new(center.x + w / 2.0, center.y + h / 2.0),
        Pos2::new(center.x - w / 2.0, center.y + h / 2.0),
    ];
    p.add(egui::epaint::PathShape::closed_line(points, stroke));
    // Exclamation point
    p.line_segment(
        [
            Pos2::new(center.x, center.y - h * 0.15),
            Pos2::new(center.x, center.y + h * 0.12),
        ],
        stroke,
    );
    p.circle_filled(
        Pos2::new(center.x, center.y + h * 0.32),
        stroke.width,
        color,
    );
}

fn paint_info(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let r = size * 0.40;
    p.circle_stroke(center, r, stroke);
    p.circle_filled(
        Pos2::new(center.x, center.y - r * 0.40),
        stroke.width,
        color,
    );
    p.line_segment(
        [
            Pos2::new(center.x, center.y - r * 0.10),
            Pos2::new(center.x, center.y + r * 0.45),
        ],
        stroke,
    );
}

fn paint_success(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let d = size * 0.28;
    let p1 = Pos2::new(center.x - d, center.y);
    let p2 = Pos2::new(center.x - d * 0.25, center.y + d * 0.75);
    let p3 = Pos2::new(center.x + d, center.y - d * 0.60);
    p.line_segment([p1, p2], stroke);
    p.line_segment([p2, p3], stroke);
}

fn paint_error(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let r = size * 0.40;
    p.circle_stroke(center, r, Stroke::new(stroke.width, color));
    paint_close(p, center, size * 0.70, stroke);
}

fn paint_scan(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let r = size * 0.32;
    // Circular arc
    let n = 12;
    for i in 0..n {
        let a1 = i as f32 * (1.6 * PI / n as f32) - PI * 0.8;
        let a2 = (i + 1) as f32 * (1.6 * PI / n as f32) - PI * 0.8;
        let pt1 = Pos2::new(center.x + r * a1.cos(), center.y + r * a1.sin());
        let pt2 = Pos2::new(center.x + r * a2.cos(), center.y + r * a2.sin());
        p.line_segment([pt1, pt2], stroke);
    }
    // Arrow head
    let tip = Pos2::new(
        center.x + r * (0.8 * PI).cos(),
        center.y + r * (0.8 * PI).sin(),
    );
    p.line_segment(
        [tip, Pos2::new(tip.x - size * 0.12, tip.y - size * 0.08)],
        stroke,
    );
    p.line_segment(
        [tip, Pos2::new(tip.x + size * 0.02, tip.y - size * 0.14)],
        stroke,
    );
}

fn paint_spinner(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let r = size * 0.35;
    for i in 0..8 {
        let angle = i as f32 * (PI / 4.0);
        let alpha = ((i + 1) as f32 / 8.0 * 255.0) as u8;
        let pt = Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin());
        p.circle_filled(
            pt,
            stroke.width,
            Stroke::new(1.0_f32, stroke.color)
                .color
                .linear_multiply(alpha as f32 / 255.0),
        );
    }
}

fn paint_power(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let r = size * 0.34;
    // Broken circle (top open)
    let n = 14;
    for i in 0..n {
        let a1 = i as f32 * (1.6 * PI / n as f32) + PI * 0.7;
        let a2 = (i + 1) as f32 * (1.6 * PI / n as f32) + PI * 0.7;
        let pt1 = Pos2::new(center.x + r * a1.cos(), center.y + r * a1.sin());
        let pt2 = Pos2::new(center.x + r * a2.cos(), center.y + r * a2.sin());
        p.line_segment([pt1, pt2], stroke);
    }
    // Vertical switch line
    p.line_segment(
        [
            Pos2::new(center.x, center.y - r * 1.1),
            Pos2::new(center.x, center.y - r * 0.1),
        ],
        stroke,
    );
}

fn paint_mic(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let w = size * 0.24;
    let h = size * 0.44;
    let capsule =
        Rect::from_center_size(Pos2::new(center.x, center.y - size * 0.08), Vec2::new(w, h));
    p.rect_filled(capsule, Rounding::same(w / 2.0), color);

    // Cradle arc
    let r = size * 0.28;
    let n = 8;
    for i in 0..n {
        let a1 = i as f32 * (PI / n as f32);
        let a2 = (i + 1) as f32 * (PI / n as f32);
        let pt1 = Pos2::new(
            center.x + r * a1.cos(),
            center.y - size * 0.04 + r * a1.sin(),
        );
        let pt2 = Pos2::new(
            center.x + r * a2.cos(),
            center.y - size * 0.04 + r * a2.sin(),
        );
        p.line_segment([pt1, pt2], stroke);
    }
    p.line_segment(
        [
            Pos2::new(center.x, center.y + r - size * 0.04),
            Pos2::new(center.x, center.y + size * 0.38),
        ],
        stroke,
    );
}

fn paint_speaker(p: &Painter, center: Pos2, size: f32, stroke: Stroke, color: Color32) {
    let box_w = size * 0.20;
    let box_h = size * 0.32;
    let b = Rect::from_center_size(
        Pos2::new(center.x - size * 0.16, center.y),
        Vec2::new(box_w, box_h),
    );
    p.rect_filled(b, Rounding::same(1.0), color);

    // Cone
    let points = vec![
        Pos2::new(b.right(), b.top()),
        Pos2::new(center.x + size * 0.12, center.y - size * 0.32),
        Pos2::new(center.x + size * 0.12, center.y + size * 0.32),
        Pos2::new(b.right(), b.bottom()),
    ];
    p.add(egui::epaint::PathShape::convex_polygon(
        points,
        color,
        Stroke::NONE,
    ));

    // Sound waves
    let wave_r = size * 0.26;
    let n = 6;
    for i in 0..n {
        let a1 = (i as f32 * (PI * 0.6 / n as f32)) - PI * 0.3;
        let a2 = ((i + 1) as f32 * (PI * 0.6 / n as f32)) - PI * 0.3;
        let pt1 = Pos2::new(
            center.x + size * 0.14 + wave_r * a1.cos(),
            center.y + wave_r * a1.sin(),
        );
        let pt2 = Pos2::new(
            center.x + size * 0.14 + wave_r * a2.cos(),
            center.y + wave_r * a2.sin(),
        );
        p.line_segment([pt1, pt2], stroke);
    }
}

fn paint_add(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let d = size * 0.30;
    p.line_segment(
        [
            Pos2::new(center.x - d, center.y),
            Pos2::new(center.x + d, center.y),
        ],
        stroke,
    );
    p.line_segment(
        [
            Pos2::new(center.x, center.y - d),
            Pos2::new(center.x, center.y + d),
        ],
        stroke,
    );
}

fn paint_remove(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let d = size * 0.30;
    p.line_segment(
        [
            Pos2::new(center.x - d, center.y),
            Pos2::new(center.x + d, center.y),
        ],
        stroke,
    );
}

fn paint_duplicate(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    paint_copy(p, center, size, stroke);
}

fn paint_reverse(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let d = size * 0.30;
    // Top arrow (pointing right)
    p.line_segment(
        [
            Pos2::new(center.x - d, center.y - size * 0.14),
            Pos2::new(center.x + d, center.y - size * 0.14),
        ],
        stroke,
    );
    p.line_segment(
        [
            Pos2::new(center.x + d, center.y - size * 0.14),
            Pos2::new(center.x + d * 0.4, center.y - size * 0.26),
        ],
        stroke,
    );

    // Bottom arrow (pointing left)
    p.line_segment(
        [
            Pos2::new(center.x + d, center.y + size * 0.14),
            Pos2::new(center.x - d, center.y + size * 0.14),
        ],
        stroke,
    );
    p.line_segment(
        [
            Pos2::new(center.x - d, center.y + size * 0.14),
            Pos2::new(center.x - d * 0.4, center.y + size * 0.26),
        ],
        stroke,
    );
}

fn paint_delete(p: &Painter, center: Pos2, size: f32, stroke: Stroke) {
    let w = size * 0.44;
    let h = size * 0.50;
    let body = Rect::from_center_size(Pos2::new(center.x, center.y + size * 0.08), Vec2::new(w, h));
    p.rect_stroke(body, Rounding::same(2.0), stroke);

    // Can lid
    p.line_segment(
        [
            Pos2::new(center.x - size * 0.30, body.top() - 1.0),
            Pos2::new(center.x + size * 0.30, body.top() - 1.0),
        ],
        stroke,
    );
    // Lid handle
    p.line_segment(
        [
            Pos2::new(center.x - size * 0.10, body.top() - size * 0.10),
            Pos2::new(center.x + size * 0.10, body.top() - size * 0.10),
        ],
        stroke,
    );
}

// ====================================================================
// ALIGNMENT HELPERS
// ====================================================================

/// Renders an icon and text row with optical center alignment and 8px gap.
pub fn icon_label(ui: &mut Ui, icon: Icon, text: &str, size: f32, color: Color32) -> Response {
    let ppp = ui.ctx().pixels_per_point();
    let row_height = 32.0_f32;
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), row_height), Sense::hover());

    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let icon_box = Rect::from_center_size(
            Pos2::new(rect.min.x + size / 2.0, rect.center().y),
            Vec2::splat(size),
        );
        let snapped_icon = snap_rect_to_pixels(icon_box, ppp);
        icon.paint(p, snapped_icon, color);

        let text_pos = Pos2::new(rect.min.x + size + 8.0, rect.center().y - 7.0);
        let font = crate::ui::typography::TypeScale::body();
        let galley = ui.fonts(|f| f.layout_no_wrap(text.to_owned(), font, color));
        p.galley(text_pos, galley, color);
    }
    resp
}

/// Renders a standardized 32x32 icon button with hover and pressed visual states.
pub fn icon_button_32(ui: &mut Ui, icon: Icon, tooltip: &str, is_active: bool) -> Response {
    let ppp = ui.ctx().pixels_per_point();
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::click());

    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let tokens = crate::ui::theme::ThemeTokens::default_dark();

        let (bg_fill, fg_color) = if resp.is_pointer_button_down_on() {
            (tokens.accent.pressed, Color32::WHITE)
        } else if resp.hovered() {
            (tokens.surfaces.raised_hover, tokens.text.primary)
        } else if is_active {
            (tokens.accent.base, Color32::WHITE)
        } else {
            (Color32::TRANSPARENT, tokens.text.secondary)
        };

        if bg_fill != Color32::TRANSPARENT {
            p.rect_filled(rect, Rounding::same(6.0), bg_fill);
            if resp.hovered() && !is_active {
                p.rect_stroke(
                    rect,
                    Rounding::same(6.0),
                    Stroke::new(1.0_f32, tokens.borders.hover),
                );
            }
        }

        let icon_box = Rect::from_center_size(rect.center(), Vec2::splat(16.0));
        let snapped = snap_rect_to_pixels(icon_box, ppp);
        icon.paint(p, snapped, fg_color);
    }

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.on_hover_text(tooltip)
}

/// Renders a 32x32 rounded tile with tinted semantic background for modal/InfoBar headers.
pub fn header_tile_icon(ui: &mut Ui, icon: Icon, fg: Color32, bg: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, Rounding::same(8.0), bg);
    let icon_box = Rect::from_center_size(rect.center(), Vec2::splat(18.0));
    icon.paint(p, icon_box, fg);
}

/// Debug icon sheet tool rendering every icon at 16, 20, and 24px in every color state.
pub fn render_debug_icon_sheet(ui: &mut Ui) {
    let tokens = crate::ui::theme::ThemeTokens::default_dark();
    let all_icons = [
        Icon::Effects,
        Icon::Palette,
        Icon::MusicBars,
        Icon::DiySliders,
        Icon::Devices,
        Icon::Settings,
        Icon::HardwareChip,
        Icon::Back,
        Icon::ChevronDown,
        Icon::ChevronRight,
        Icon::Play,
        Icon::Pause,
        Icon::Once,
        Icon::Star,
        Icon::StarFilled,
        Icon::Search,
        Icon::Close,
        Icon::Copy,
        Icon::Bluetooth,
        Icon::BluetoothOff,
        Icon::Warning,
        Icon::Info,
        Icon::Success,
        Icon::Error,
        Icon::Scan,
        Icon::Power,
        Icon::Mic,
        Icon::Speaker,
        Icon::Add,
        Icon::Remove,
        Icon::Duplicate,
        Icon::Reverse,
        Icon::Delete,
    ];

    ui.vertical(|ui| {
        ui.heading("Pyxillum Fluent System Icons Debug Sheet");
        ui.label("Rendering all 33 vector icons across 16px, 20px, and 24px physical grids.");
        ui.add_space(12.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for icon in all_icons {
                ui.horizontal(|ui| {
                    ui.label(format!("{:15?}", icon));
                    for size in [16.0, 20.0, 24.0] {
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::splat(size + 4.0), Sense::hover());
                        let p = ui.painter();
                        p.rect_stroke(
                            rect,
                            Rounding::ZERO,
                            Stroke::new(0.5_f32, Color32::from_white_alpha(30)),
                        );
                        let icon_box = Rect::from_center_size(rect.center(), Vec2::splat(size));
                        icon.paint(p, icon_box, tokens.accent.base);
                    }
                });
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_icon_pixel_snapping_at_standard_dpi_scales() {
        let test_scales = [1.0_f32, 1.25, 1.50, 2.0];
        let original_rect = Rect::from_min_size(Pos2::new(12.333, 45.666), Vec2::new(16.0, 16.0));

        for scale in test_scales {
            let snapped = snap_rect_to_pixels(original_rect, scale);
            let phys_min_x = (snapped.min.x * scale).round();
            let phys_min_y = (snapped.min.y * scale).round();

            assert!(
                ((snapped.min.x * scale) - phys_min_x).abs() < 1e-4,
                "Snapping failed at scale {}: x coord {} is not integer aligned",
                scale,
                snapped.min.x * scale
            );
            assert!(
                ((snapped.min.y * scale) - phys_min_y).abs() < 1e-4,
                "Snapping failed at scale {}: y coord {} is not integer aligned",
                scale,
                snapped.min.y * scale
            );
        }
    }

    #[test]
    fn test_zero_unicode_symbol_icons_in_source() {
        // Scans crates/app/src for Unicode symbol glyphs (dingbats, emojis, geometric shapes)
        let forbidden_symbols = ['✕', '⚡', '⚙', '🔍', '🎵', '⭐', '➕', '▶', '⏸', '🔁', '🗑'];

        fn check_dir(dir: &Path, forbidden: &[char]) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        check_dir(&path, forbidden);
                    } else if path.extension().is_some_and(|ext| ext == "rs") {
                        let content = fs::read_to_string(&path).unwrap_or_default();
                        for (line_num, line) in content.lines().enumerate() {
                            let code_only = line.split("//").next().unwrap_or("");
                            for &sym in forbidden {
                                assert!(
                                    !code_only.contains(sym),
                                    "Forbidden Unicode icon '{}' found at {}:{}\nLine: {}",
                                    sym,
                                    path.display(),
                                    line_num + 1,
                                    line
                                );
                            }
                        }
                    }
                }
            }
        }

        let src_dir = Path::new("crates/app/src");
        if src_dir.exists() {
            check_dir(src_dir, &forbidden_symbols);
        }
    }
}
