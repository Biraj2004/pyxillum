//! # DIY Custom Lighting Mode Screen
//!
//! 14-node colour sequencer supporting Solid/Gradient interpolation,
//! multi-algorithm chase modes (Forward, Backward, Middle, Out), and real-time preview.

use crate::state::{AppState, DiyChaseMode, DiyEffectType};
use crate::theme::{render_card, render_segmented_toggle, Theme};
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

pub fn render_diy_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut state_changed = false;

    let max_w = 700.0_f32.min(ui.available_width());
    let pad = ((ui.available_width() - max_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_max_width(max_w);

            // 1. DIY Effect Header Card & Solid / Gradient Toggle
            render_card(ui, 8.0, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("DIY Lighting Sequence")
                            .strong()
                            .size(15.0)
                            .color(Theme::TEXT_PRIMARY),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let is_grad = state.diy_effect_type == DiyEffectType::Gradient;
                        let (_, new_sel) = render_segmented_toggle(
                            ui,
                            "diy_mode_toggle",
                            "Solid",
                            "Gradient",
                            is_grad,
                        );
                        if let Some(to_grad) = new_sel {
                            state.diy_effect_type = if to_grad {
                                DiyEffectType::Gradient
                            } else {
                                DiyEffectType::Solid
                            };
                            state_changed = true;
                        }
                    });
                });
            });

            ui.add_space(12.0);

            // 2. 14-Node Colour Grid Container (2x7 Grid)
            render_card(ui, 8.0, |ui| {
                ui.label(
                    egui::RichText::new("Sequence Palette (14 Nodes)")
                        .size(13.0)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(10.0);

                let available_w = ui.available_width();
                let cols = 7;
                let cell_w = (available_w / cols as f32).max(42.0);

                // Render in 2 rows of 7 nodes
                for row in 0..2 {
                    ui.horizontal(|ui| {
                        for col in 0..cols {
                            let idx = row * cols + col;
                            if idx < state.diy_nodes.len() {
                                let node = &state.diy_nodes[idx];
                                let is_selected = state.selected_diy_index == Some(idx);

                                let (btn_rect, btn_resp) = ui.allocate_exact_size(
                                    Vec2::new(cell_w - 6.0, 48.0),
                                    egui::Sense::click(),
                                );
                                let p = ui.painter_at(btn_rect);

                                let chip_center =
                                    Pos2::new(btn_rect.center().x, btn_rect.min.y + 18.0);
                                let chip_radius = 14.0_f32;

                                let chip_col =
                                    Color32::from_rgb(node.color.r, node.color.g, node.color.b);
                                p.circle_filled(chip_center, chip_radius, chip_col);

                                if is_selected {
                                    p.circle_stroke(
                                        chip_center,
                                        chip_radius + 3.0,
                                        Stroke::new(2.5_f32, Theme::ACCENT_BLUE),
                                    );
                                } else {
                                    p.circle_stroke(
                                        chip_center,
                                        chip_radius,
                                        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
                                    );
                                }

                                // Node numeric readout
                                let txt_col = if (node.color.r as u32 * 299
                                    + node.color.g as u32 * 587
                                    + node.color.b as u32 * 114)
                                    / 1000
                                    > 128
                                {
                                    Color32::BLACK
                                } else {
                                    Color32::WHITE
                                };

                                p.text(
                                    chip_center,
                                    egui::Align2::CENTER_CENTER,
                                    format!("{}", node.value),
                                    egui::FontId::proportional(10.0),
                                    txt_col,
                                );

                                if btn_resp.clicked() {
                                    state.selected_diy_index = Some(idx);
                                }
                            }
                        }
                    });
                    ui.add_space(8.0);
                }

                // Node Editor Controls for Selected Index
                if let Some(sel_idx) = state.selected_diy_index {
                    if sel_idx < state.diy_nodes.len() {
                        ui.add_space(6.0);
                        ui.separator();
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("Editing Node #{}", sel_idx + 1))
                                    .size(12.0)
                                    .strong()
                                    .color(Theme::TEXT_PRIMARY),
                            );

                            let node = &mut state.diy_nodes[sel_idx];
                            let mut node_col = [
                                node.color.r as f32 / 255.0,
                                node.color.g as f32 / 255.0,
                                node.color.b as f32 / 255.0,
                            ];

                            if ui.color_edit_button_rgb(&mut node_col).changed() {
                                node.color = RgbColor::new(
                                    (node_col[0] * 255.0) as u8,
                                    (node_col[1] * 255.0) as u8,
                                    (node_col[2] * 255.0) as u8,
                                );
                                state_changed = true;
                            }

                            ui.add_space(16.0);
                            ui.label(
                                egui::RichText::new("Weight / Value:")
                                    .size(11.5)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            if ui
                                .add(egui::Slider::new(&mut node.value, 0..=255).show_value(true))
                                .changed()
                            {
                                state_changed = true;
                            }
                        });
                    }
                }
            });

            ui.add_space(12.0);

            // 3. Operational Direction & Parameters
            render_card(ui, 8.0, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Chase Mode")
                            .strong()
                            .size(13.0)
                            .color(Theme::TEXT_PRIMARY),
                    );

                    let current_mode_str = match state.diy_chase_mode {
                        DiyChaseMode::Forward => "Chase forward",
                        DiyChaseMode::Backward => "Chase backward",
                        DiyChaseMode::Middle => "Chase middle to out",
                        DiyChaseMode::Out => "Chase out to middle",
                    };

                    egui::ComboBox::from_id_source("diy_chase_select")
                        .selected_text(current_mode_str)
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(
                                    state.diy_chase_mode == DiyChaseMode::Forward,
                                    "Chase forward",
                                )
                                .clicked()
                            {
                                state.diy_chase_mode = DiyChaseMode::Forward;
                                state_changed = true;
                            }
                            if ui
                                .selectable_label(
                                    state.diy_chase_mode == DiyChaseMode::Backward,
                                    "Chase backward",
                                )
                                .clicked()
                            {
                                state.diy_chase_mode = DiyChaseMode::Backward;
                                state_changed = true;
                            }
                            if ui
                                .selectable_label(
                                    state.diy_chase_mode == DiyChaseMode::Middle,
                                    "Chase middle to out",
                                )
                                .clicked()
                            {
                                state.diy_chase_mode = DiyChaseMode::Middle;
                                state_changed = true;
                            }
                            if ui
                                .selectable_label(
                                    state.diy_chase_mode == DiyChaseMode::Out,
                                    "Chase out to middle",
                                )
                                .clicked()
                            {
                                state.diy_chase_mode = DiyChaseMode::Out;
                                state_changed = true;
                            }
                        });
                });

                ui.add_space(14.0);

                // Speed & Brightness Sliders
                ui.columns(2, |cols| {
                    cols[0].vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Sequence Brightness")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        let pct = (state.diy_brightness as f32 / 255.0 * 100.0) as u32;
                        ui.label(
                            egui::RichText::new(format!("{}%", pct))
                                .strong()
                                .size(18.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(4.0);
                        if ui
                            .add(
                                egui::Slider::new(&mut state.diy_brightness, 0..=255)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            state_changed = true;
                        }
                    });

                    cols[1].vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Sequence Speed")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.label(
                            egui::RichText::new(format!("Level {}", state.diy_speed))
                                .strong()
                                .size(18.0)
                                .color(Theme::ACCENT_CYAN),
                        );
                        ui.add_space(4.0);
                        if ui
                            .add(egui::Slider::new(&mut state.diy_speed, 1..=16).show_value(false))
                            .changed()
                        {
                            state_changed = true;
                        }
                    });
                });
            });

            ui.add_space(12.0);

            // 4. Live Strip Visual Preview Bar
            render_card(ui, 8.0, |ui| {
                ui.label(
                    egui::RichText::new("Output Preview")
                        .size(12.0)
                        .color(Theme::TEXT_SECONDARY),
                );
                ui.add_space(6.0);

                let (p_rect, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), 26.0),
                    egui::Sense::hover(),
                );
                let p = ui.painter_at(p_rect);
                p.rect_filled(p_rect, Rounding::same(6.0), Theme::WELL_BG);

                let n_count = state.diy_nodes.len();
                if n_count > 0 {
                    let step_w = p_rect.width() / n_count as f32;
                    for (i, node) in state.diy_nodes.iter().enumerate() {
                        let seg_rect = Rect::from_min_size(
                            Pos2::new(p_rect.min.x + (i as f32) * step_w, p_rect.min.y),
                            Vec2::new(step_w, p_rect.height()),
                        );
                        let col = Color32::from_rgb(node.color.r, node.color.g, node.color.b);
                        p.rect_filled(seg_rect, Rounding::ZERO, col);
                    }
                }
            });
        });
    });

    state_changed
}
