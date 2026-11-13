//! # 60 FPS Real-Time LED Strip Simulator Widget
//!
//! Renders an addressable pixel LED strip with acrylic diffusion and dynamic glow.

use eframe::egui::{self, Color32, Pos2, Rounding, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

pub fn render_neon_strip(ui: &mut egui::Ui, pixels: &[RgbColor], power: bool) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 44.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    // Dark acrylic track background
    painter.rect_filled(rect, Rounding::same(10.0), Color32::from_rgb(15, 23, 42));
    painter.rect_stroke(
        rect,
        Rounding::same(10.0),
        Stroke::new(1.0_f32, Color32::from_rgb(51, 65, 85)),
    );

    let num_pixels = pixels.len().max(30);
    let step = rect.width() / (num_pixels as f32);

    for (i, p) in pixels.iter().enumerate() {
        let center = Pos2::new(rect.min.x + (i as f32 + 0.5) * step, rect.center().y);
        let radius = (step * 0.38).clamp(3.0, 7.5);

        let col = if power {
            Color32::from_rgb(p.r, p.g, p.b)
        } else {
            Color32::from_rgb(30, 41, 59)
        };

        // Outer glow halo if powered
        if power && (p.r > 20 || p.g > 20 || p.b > 20) {
            let halo = Color32::from_rgba_premultiplied(p.r / 4, p.g / 4, p.b / 4, 35);
            painter.circle_filled(center, radius * 1.8, halo);
        }

        // Core diode
        painter.circle_filled(center, radius, col);

        // Highlight glint
        if power {
            painter.circle_filled(
                Pos2::new(center.x - radius * 0.3, center.y - radius * 0.3),
                radius * 0.28,
                Color32::from_rgba_premultiplied(255, 255, 255, 120),
            );
        }
    }
}
