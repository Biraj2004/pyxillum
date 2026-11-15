//! # Real-time LED Strip Monitor Widget
//!
//! Provides the simulated hardware monitor showing 60 FPS pixel strip states
//! with clean Windows 11 desktop styling, lens diodes, and subtle diffusion.

use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rounding, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

/// Renders the 60 FPS animated LED pixel strip monitor.
pub fn render_neon_strip(ui: &mut egui::Ui, pixels: &[RgbColor], power: bool) {
    let avail_w = ui.available_width().max(320.0);
    let height = 44.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    // Outer Dark Mica Container
    painter.rect_filled(rect, Rounding::same(8.0), Theme::WELL_BG);
    painter.rect_stroke(
        rect,
        Rounding::same(8.0),
        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
    );

    if pixels.is_empty() {
        // Draw subtle standby indicators
        let dot_count = (avail_w / 28.0) as usize;
        let spacing = avail_w / (dot_count + 1) as f32;
        for i in 1..=dot_count {
            let cx = rect.min.x + (i as f32) * spacing;
            let cy = rect.center().y;
            painter.circle_filled(Pos2::new(cx, cy), 3.0, Color32::from_rgb(48, 48, 48));
        }
        return;
    }

    let count = pixels.len().min(120);
    let usable_w = rect.width() - 24.0;
    let step = usable_w / count as f32;
    let radius = (step * 0.38).clamp(2.5, 5.0);

    for (i, p) in pixels.iter().take(count).enumerate() {
        let cx = rect.min.x + 12.0 + (i as f32 + 0.5) * step;
        let cy = rect.center().y;
        let pos = Pos2::new(cx, cy);

        let col = if power {
            Color32::from_rgb(p.r, p.g, p.b)
        } else {
            Color32::from_rgb(44, 44, 44)
        };

        // Outer soft glow lens
        if power && (p.r > 20 || p.g > 20 || p.b > 20) {
            let glow_col = Color32::from_rgba_premultiplied(p.r / 4, p.g / 4, p.b / 4, 30);
            painter.circle_filled(pos, radius * 1.8, glow_col);
        }

        // Core diode bead
        painter.circle_filled(pos, radius, col);
    }
}
