//! # Continuous Radial Color Disc Widget
//!
//! High-performance Hue & Saturation color wheel rendered via triangle mesh.

use crate::theme::Theme;
use eframe::egui::{self, Color32, Stroke, Vec2};

pub fn render_color_disc(ui: &mut egui::Ui, current_color: &mut [f32; 3], radius: f32) -> bool {
    let size = Vec2::splat(radius * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    let center = rect.center();

    let segments = 64;
    let mut mesh = egui::Mesh::default();
    let center_idx = mesh.vertices.len() as u32;
    mesh.colored_vertex(center, Color32::WHITE);

    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let pos = center + Vec2::new(angle.cos(), angle.sin()) * radius;

        let hue = (i as f32 / segments as f32) * 360.0;
        let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
        let col = Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);

        mesh.colored_vertex(pos, col);

        if i > 0 {
            mesh.add_triangle(center_idx, center_idx + i as u32, center_idx + i as u32 + 1);
        }
    }

    painter.add(egui::Shape::mesh(mesh));

    // Outer border ring
    painter.circle_stroke(center, radius, Stroke::new(1.5_f32, Theme::BORDER_LIGHT));

    let (curr_h, curr_s, _) = rgb_to_hsv(current_color[0], current_color[1], current_color[2]);
    let curr_angle = curr_h.to_radians();
    let indicator_pos = center + Vec2::new(curr_angle.cos(), curr_angle.sin()) * (curr_s * radius);

    painter.circle_stroke(indicator_pos, 8.5, Stroke::new(2.5_f32, Color32::WHITE));
    painter.circle_stroke(indicator_pos, 10.0, Stroke::new(1.0_f32, Color32::BLACK));

    let mut changed = false;
    if response.dragged() || response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            let offset = pos - center;
            let dist = offset.length();
            let sat = (dist / radius).clamp(0.0, 1.0);
            let mut angle_rad = offset.y.atan2(offset.x);
            if angle_rad < 0.0 {
                angle_rad += std::f32::consts::TAU;
            }
            let hue_deg = angle_rad.to_degrees();

            let (r, g, b) = hsv_to_rgb(hue_deg, sat, 1.0);
            current_color[0] = r;
            current_color[1] = g;
            current_color[2] = b;
            changed = true;
        }
    }

    changed
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = match (h / 60.0) as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (r_prime + m, g_prime + m, b_prime + m)
}

pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta <= 0.0001 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h_normalized = if h < 0.0 { h + 360.0 } else { h };
    let s = if max <= 0.0001 { 0.0 } else { delta / max };
    let v = max;

    (h_normalized, s, v)
}
