//! # DIY Custom Lighting Mode Screen
//!
//! 14-node colour sequencer supporting Solid/Gradient interpolation,
//! multi-algorithm chase modes (Forward, Backward, Middle, Out), and real-time preview.

use crate::state::{AppState, DiyChaseMode, DiyEffectType};
use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

pub fn render_diy_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut state_changed = false;

    ui.vertical(|ui| {
        // 1. DIY Effect Header & Solid / Gradient Toggle
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("DIY Effect")
                    .strong()
                    .size(17.0)
                    .color(Theme::TEXT_PRIMARY),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let is_grad = state.diy_effect_type == DiyEffectType::Gradient;
                let (tog_rect, _) =
                    ui.allocate_exact_size(Vec2::new(140.0, 32.0), egui::Sense::hover());
                let tp = ui.painter_at(tog_rect);
                tp.rect_filled(
                    tog_rect,
                    Rounding::same(16.0),
                    Color32::from_rgb(241, 245, 249),
                );

                let half_w = tog_rect.width() / 2.0;
                let solid_rect =
                    Rect::from_min_size(tog_rect.min, Vec2::new(half_w, tog_rect.height()));
                let grad_rect = Rect::from_min_size(
                    Pos2::new(tog_rect.min.x + half_w, tog_rect.min.y),
                    Vec2::new(half_w, tog_rect.height()),
                );

                if !is_grad {
                    tp.rect_filled(solid_rect, Rounding::same(16.0), Color32::WHITE);
                    tp.rect_stroke(
                        solid_rect,
                        Rounding::same(16.0),
                        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
                    );
                } else {
                    tp.rect_filled(grad_rect, Rounding::same(16.0), Color32::WHITE);
                    tp.rect_stroke(
                        grad_rect,
                        Rounding::same(16.0),
                        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
                    );
                }

                tp.text(
                    solid_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Solid",
                    egui::FontId::proportional(12.0),
                    if !is_grad {
                        Theme::TEXT_PRIMARY
                    } else {
                        Theme::TEXT_SECONDARY
                    },
                );
                tp.text(
                    grad_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Gradient",
                    egui::FontId::proportional(12.0),
                    if is_grad {
                        Theme::TEXT_PRIMARY
                    } else {
                        Theme::TEXT_SECONDARY
                    },
                );

                if ui
                    .interact(solid_rect, ui.id().with("tog_solid"), egui::Sense::click())
                    .clicked()
                {
                    state.diy_effect_type = DiyEffectType::Solid;
                    state_changed = true;
                }
                if ui
                    .interact(grad_rect, ui.id().with("tog_grad"), egui::Sense::click())
                    .clicked()
                {
                    state.diy_effect_type = DiyEffectType::Gradient;
                    state_changed = true;
                }
            });
        });

        ui.add_space(16.0);

        // 2. 14-Node Colour Grid Container (2x7 Grid)
        egui::Frame::none()
            .fill(Color32::from_rgb(248, 250, 252))
            .rounding(Rounding::same(14.0))
            .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
            .inner_margin(egui::Margin::symmetric(16.0, 14.0))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Color")
                        .size(13.0)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(10.0);

                let available_w = ui.available_width();
                let cols = 7;
                let cell_w = (available_w / cols as f32).min(44.0);
                let circle_r = 17.0;

                // Row 1 (Nodes 0..7)
                ui.horizontal(|ui| {
                    for i in 0..7 {
                        render_color_node(ui, state, i, cell_w, circle_r, &mut state_changed);
                    }
                });

                ui.add_space(10.0);

                // Row 2 (Nodes 7..14)
                ui.horizontal(|ui| {
                    for i in 7..14 {
                        render_color_node(ui, state, i, cell_w, circle_r, &mut state_changed);
                    }
                });
            });

        ui.add_space(16.0);

        // 3. Chase Mode Dropdown Card
        let (mode_rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), egui::Sense::hover());
        let mp = ui.painter_at(mode_rect);
        mp.rect_filled(
            mode_rect,
            Rounding::same(12.0),
            Color32::from_rgb(248, 250, 252),
        );
        mp.rect_stroke(
            mode_rect,
            Rounding::same(12.0),
            Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
        );

        mp.text(
            Pos2::new(mode_rect.min.x + 14.0, mode_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Mode",
            egui::FontId::proportional(14.0),
            Theme::TEXT_PRIMARY,
        );

        let mode_lbl = match state.diy_chase_mode {
            DiyChaseMode::Forward => "Chase forward",
            DiyChaseMode::Backward => "Chase backward",
            DiyChaseMode::Middle => "Chase middle",
            DiyChaseMode::Out => "Chase out",
        };

        ui.allocate_ui_at_rect(
            Rect::from_min_max(
                Pos2::new(mode_rect.max.x - 170.0, mode_rect.min.y + 8.0),
                Pos2::new(mode_rect.max.x - 10.0, mode_rect.max.y - 8.0),
            ),
            |ui| {
                egui::ComboBox::from_id_source("diy_chase_mode_cb")
                    .selected_text(mode_lbl)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut state.diy_chase_mode,
                                DiyChaseMode::Forward,
                                "Chase forward",
                            )
                            .clicked()
                        {
                            state_changed = true;
                        }
                        if ui
                            .selectable_value(
                                &mut state.diy_chase_mode,
                                DiyChaseMode::Backward,
                                "Chase backward",
                            )
                            .clicked()
                        {
                            state_changed = true;
                        }
                        if ui
                            .selectable_value(
                                &mut state.diy_chase_mode,
                                DiyChaseMode::Middle,
                                "Chase middle",
                            )
                            .clicked()
                        {
                            state_changed = true;
                        }
                        if ui
                            .selectable_value(
                                &mut state.diy_chase_mode,
                                DiyChaseMode::Out,
                                "Chase out",
                            )
                            .clicked()
                        {
                            state_changed = true;
                        }
                    });
            },
        );

        ui.add_space(16.0);

        // 4. Dual Metric Sliders: Brightness & Speed
        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(
                    egui::RichText::new("Brightness")
                        .size(12.0)
                        .color(Theme::TEXT_SECONDARY),
                );
                let pct = (state.diy_brightness as f32 / 255.0 * 100.0) as u32;
                ui.label(
                    egui::RichText::new(format!("{}%", pct))
                        .strong()
                        .size(16.0)
                        .color(Theme::TEXT_PRIMARY),
                );
                if ui
                    .add(egui::Slider::new(&mut state.diy_brightness, 0..=255).show_value(false))
                    .changed()
                {
                    state_changed = true;
                }
            });

            cols[1].group(|ui| {
                ui.label(
                    egui::RichText::new("Speed")
                        .size(12.0)
                        .color(Theme::TEXT_SECONDARY),
                );
                ui.label(
                    egui::RichText::new(format!("{}", state.diy_speed))
                        .strong()
                        .size(16.0)
                        .color(Theme::ACCENT_EMERALD),
                );
                if ui
                    .add(egui::Slider::new(&mut state.diy_speed, 1..=10).show_value(false))
                    .changed()
                {
                    state_changed = true;
                }
            });
        });

        ui.add_space(16.0);

        // 5. Multi-Colour Chase Running Preview Bar
        let (prev_rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 38.0), egui::Sense::hover());
        let painter = ui.painter_at(prev_rect);

        let step_w = prev_rect.width() / 14.0;
        let offset = (state.animation_timer * 4.0) as usize % 14;

        for i in 0..14 {
            let node_idx = match state.diy_chase_mode {
                DiyChaseMode::Backward => (14 + offset - (i % 14)) % 14,
                _ => (i + offset) % 14,
            };
            let n = &state.diy_nodes[node_idx];
            let col = Color32::from_rgb(n.color.r, n.color.g, n.color.b);
            let seg = Rect::from_min_max(
                Pos2::new(prev_rect.min.x + (i as f32) * step_w, prev_rect.min.y),
                Pos2::new(prev_rect.min.x + ((i + 1) as f32) * step_w, prev_rect.max.y),
            );
            painter.rect_filled(seg, Rounding::same(4.0), col);
        }
        painter.rect_stroke(
            prev_rect,
            Rounding::same(19.0),
            Stroke::new(1.0_f32, Color32::WHITE),
        );
    });

    state_changed
}

fn render_color_node(
    ui: &mut egui::Ui,
    state: &mut AppState,
    idx: usize,
    cell_w: f32,
    circle_r: f32,
    state_changed: &mut bool,
) {
    let (node_rect, resp) = ui.allocate_exact_size(Vec2::new(cell_w, 36.0), egui::Sense::click());
    let p = ui.painter_at(node_rect);
    let center = node_rect.center();

    let node = &state.diy_nodes[idx];
    let fill = Color32::from_rgb(node.color.r, node.color.g, node.color.b);
    let is_selected = state.selected_diy_index == Some(idx);

    p.circle_filled(center, circle_r, fill);

    if is_selected {
        p.circle_stroke(
            center,
            circle_r + 2.0,
            Stroke::new(2.0_f32, Theme::ACCENT_BLUE),
        );
    } else {
        p.circle_stroke(
            center,
            circle_r,
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0, 0, 0, 40)),
        );
    }

    let val_str = format!("{}", node.value);
    let is_dark =
        (node.color.r as u32 * 299 + node.color.g as u32 * 587 + node.color.b as u32 * 114) / 1000
            < 128;
    p.text(
        center,
        egui::Align2::CENTER_CENTER,
        val_str,
        egui::FontId::proportional(11.0),
        if is_dark {
            Color32::WHITE
        } else {
            Color32::BLACK
        },
    );

    if resp.clicked() {
        state.selected_diy_index = Some(idx);
        // Cycle node color on click
        let next_rgb = match idx % 4 {
            0 => RgbColor::new(239, 68, 68),
            1 => RgbColor::new(59, 130, 246),
            2 => RgbColor::new(34, 197, 94),
            _ => RgbColor::new(234, 179, 8),
        };
        state.diy_nodes[idx].color = next_rgb;
        *state_changed = true;
    }
}
