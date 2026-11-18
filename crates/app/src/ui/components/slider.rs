//! # Fluent Slider & ColourSlider Component
//!
//! Provides Windows 11 Fluent styled sliders featuring full-width 4px tracks,
//! expanding thumb dots on hover/press, tabular readouts, keyboard arrows, and mouse wheel adjustments.

use crate::ui::theme::ThemeTokens;
use crate::ui::typography::{render_tabular_slot, TypeScale};
use eframe::egui::{self, Color32, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};

/// Renders a full-width Windows 11 Fluent Slider.
pub fn fluent_slider(
    ui: &mut Ui,
    label: &str,
    value: &mut u8,
    min: u8,
    max: u8,
    unit: &str,
) -> Response {
    let tokens = ThemeTokens::default_dark();
    let font = TypeScale::body();

    // Top Row: Label on the left, tabular readout with unit on the right (if label provided)
    if !label.is_empty() {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(label)
                    .font(font.clone())
                    .color(tokens.text.primary),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let val_str = format!("{}{}", value, unit);
                let max_template = format!("{}{}", max, unit);
                render_tabular_slot(ui, &val_str, &max_template, &font, tokens.accent.text_light);
            });
        });
        ui.add_space(4.0);
    }

    // Track & Thumb Row (full available width)
    let full_w = ui.available_width();
    let row_h = 24.0_f32;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(full_w, row_h), Sense::click_and_drag());

    let range = (max - min) as f32;
    let current_norm = if range > 0.0 {
        (*value - min) as f32 / range
    } else {
        0.0
    };

    let track_inset = 10.0_f32;
    let track_w = (rect.width() - track_inset * 2.0).max(10.0);
    let track_y = rect.center().y;

    // Interactive Dragging / Clicking Calculation
    if resp.dragged() || resp.clicked() {
        if let Some(pos) = resp.interact_pointer_pos() {
            let rel_x = (pos.x - (rect.min.x + track_inset)).clamp(0.0, track_w);
            let norm = rel_x / track_w;
            let new_val = (min as f32 + norm * range)
                .round()
                .clamp(min as f32, max as f32) as u8;
            *value = new_val;
        }
    }

    // Keyboard Arrow Keys & Mouse Wheel Support
    if resp.hovered() {
        let wheel_delta = ui.input(|i| i.raw_scroll_delta.y);
        if wheel_delta > 0.0 {
            *value = (*value as i32 + 2).min(max as i32) as u8;
        } else if wheel_delta < 0.0 {
            *value = (*value as i32 - 2).max(min as i32) as u8;
        }

        if ui.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::ArrowUp)) {
            *value = (*value as i32 + 1).min(max as i32) as u8;
        }
        if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::ArrowDown))
        {
            *value = (*value as i32 - 1).max(min as i32) as u8;
        }
    }

    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let track_h = 4.0_f32;
        let track_rect = Rect::from_center_size(
            Pos2::new(rect.center().x, track_y),
            Vec2::new(track_w, track_h),
        );

        // Background track (well)
        p.rect_filled(track_rect, Rounding::same(2.0), tokens.surfaces.well);
        p.rect_stroke(
            track_rect,
            Rounding::same(2.0),
            Stroke::new(1.0_f32, tokens.borders.subtle),
        );

        // Active accent track (filled left portion)
        let fill_w = track_w * current_norm;
        if fill_w > 0.0 {
            let fill_rect = Rect::from_min_size(track_rect.min, Vec2::new(fill_w, track_h));
            p.rect_filled(fill_rect, Rounding::same(2.0), tokens.accent.base);
        }

        // Thumb position & expanding dot
        let thumb_x = track_rect.min.x + fill_w;
        let thumb_center = Pos2::new(thumb_x, track_y);

        let is_interacting = resp.is_pointer_button_down_on() || resp.hovered();
        let outer_r = 10.0_f32;
        let inner_dot_r = if is_interacting { 5.0_f32 } else { 3.5_f32 };

        // Outer white thumb with drop shadow
        p.circle_filled(
            Pos2::new(thumb_center.x, thumb_center.y + 1.0),
            outer_r,
            Color32::from_black_alpha(60),
        );
        p.circle_filled(thumb_center, outer_r, Color32::WHITE);
        p.circle_stroke(
            thumb_center,
            outer_r,
            Stroke::new(1.0_f32, Color32::from_black_alpha(40)),
        );

        // Expanding inner accent dot
        p.circle_filled(thumb_center, inner_dot_r, tokens.accent.base);
    }

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    resp
}

/// Renders a channel-specific colour gradient slider (R, G, or B).
pub fn colour_channel_slider(
    ui: &mut Ui,
    channel_name: &str,
    channel_val: &mut u8,
    base_color: [f32; 3],
    channel_idx: usize,
) -> Response {
    let tokens = ThemeTokens::default_dark();
    let font = TypeScale::body();

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(channel_name)
                .font(font.clone())
                .color(tokens.text.primary),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let val_str = format!("{}", channel_val);
            render_tabular_slot(ui, &val_str, "255", &font, tokens.text.primary);
        });
    });

    ui.add_space(2.0);

    let full_w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(full_w, 22.0), Sense::click_and_drag());
    let track_inset = 8.0_f32;
    let track_w = (rect.width() - track_inset * 2.0).max(10.0);
    let track_y = rect.center().y;

    if resp.dragged() || resp.clicked() {
        if let Some(pos) = resp.interact_pointer_pos() {
            let rel_x = (pos.x - (rect.min.x + track_inset)).clamp(0.0, track_w);
            *channel_val = (rel_x / track_w * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    }

    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let track_h = 6.0_f32;
        let track_rect = Rect::from_center_size(
            Pos2::new(rect.center().x, track_y),
            Vec2::new(track_w, track_h),
        );

        // Sample 16 gradient steps along the channel axis
        let steps = 16;
        let step_w = track_w / steps as f32;
        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;
            let mut col = base_color;
            col[channel_idx] = t;
            let color32 = Color32::from_rgb(
                (col[0] * 255.0) as u8,
                (col[1] * 255.0) as u8,
                (col[2] * 255.0) as u8,
            );
            let s_rect = Rect::from_min_size(
                Pos2::new(track_rect.min.x + i as f32 * step_w, track_rect.min.y),
                Vec2::new(step_w + 1.0, track_h),
            );
            p.rect_filled(s_rect, Rounding::ZERO, color32);
        }
        p.rect_stroke(
            track_rect,
            Rounding::same(3.0),
            Stroke::new(1.0_f32, tokens.borders.subtle),
        );

        // Thumb
        let thumb_x = track_rect.min.x + (*channel_val as f32 / 255.0) * track_w;
        let thumb_center = Pos2::new(thumb_x, track_y);
        p.circle_filled(thumb_center, 8.0, Color32::WHITE);
        p.circle_stroke(
            thumb_center,
            8.0,
            Stroke::new(1.0_f32, Color32::from_black_alpha(80)),
        );
    }

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    resp
}
