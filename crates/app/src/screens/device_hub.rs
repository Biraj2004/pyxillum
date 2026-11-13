//! # Device Hub Screen
//!
//! Multi-device discovery and management interface. Displays ambient hero illustration,
//! filterable device/group tabs, and grid of hardware controller cards.

use crate::state::{AppState, DeviceCardInfo};
use crate::theme::Theme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

pub fn render_device_hub(
    ui: &mut egui::Ui,
    state: &mut AppState,
    devices: &[DeviceCardInfo],
) -> Option<String> {
    let mut selected_device = None;

    ui.vertical(|ui| {
        // 1. Ambient Hero Header Card
        let hero_h = 100.0;
        let (hero_rect, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), hero_h),
            egui::Sense::hover(),
        );
        let hp = ui.painter_at(hero_rect);

        // Gradient banner
        hp.rect_filled(
            hero_rect,
            Rounding::same(16.0),
            Color32::from_rgb(147, 197, 253),
        );
        hp.rect_stroke(
            hero_rect,
            Rounding::same(16.0),
            Stroke::new(1.0_f32, Color32::from_rgb(191, 219, 254)),
        );

        // Geometric city skyline illustration
        let base_y = hero_rect.max.y - 12.0;
        let house_x = hero_rect.max.x - 110.0;
        let house_w = 46.0;
        let house_h = 56.0;
        let h_rect = Rect::from_min_size(
            Pos2::new(house_x, base_y - house_h),
            Vec2::new(house_w, house_h),
        );
        hp.rect_filled(h_rect, Rounding::same(4.0), Color32::from_rgb(59, 130, 246));

        // Lighted windows on house
        let w_rect = Rect::from_min_size(
            Pos2::new(house_x + 8.0, base_y - 36.0),
            Vec2::new(14.0, 20.0),
        );
        hp.rect_filled(
            w_rect,
            Rounding::same(2.0),
            Color32::from_rgb(254, 240, 138),
        );

        // Hero title text
        hp.text(
            Pos2::new(hero_rect.min.x + 22.0, hero_rect.min.y + 32.0),
            egui::Align2::LEFT_TOP,
            "New experience\nof light control",
            egui::FontId::proportional(20.0),
            Color32::WHITE,
        );

        ui.add_space(18.0);

        // 2. Segmented Devices / Groups Bar
        ui.horizontal(|ui| {
            if ui
                .selectable_label(
                    state.hub_tab_devices,
                    egui::RichText::new("Devices").strong().size(15.0),
                )
                .clicked()
            {
                state.hub_tab_devices = true;
            }
            ui.add_space(8.0);
            if ui
                .selectable_label(
                    !state.hub_tab_devices,
                    egui::RichText::new("Groups").strong().size(15.0),
                )
                .clicked()
            {
                state.hub_tab_devices = false;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("+ Add Device")
                                .size(12.5)
                                .color(Theme::ACCENT_BLUE),
                        )
                        .fill(Color32::from_rgb(239, 246, 255))
                        .rounding(Rounding::same(8.0)),
                    )
                    .clicked()
                {
                    // Trigger scan
                }
            });
        });

        ui.add_space(14.0);

        // 3. Device Cards Grid
        let card_w = ((ui.available_width() - 16.0) / 2.0).max(200.0);
        let card_h = 100.0;

        ui.horizontal_wrapped(|ui| {
            for dev in devices {
                let (rect, resp) =
                    ui.allocate_exact_size(Vec2::new(card_w, card_h), egui::Sense::click());
                let p = ui.painter_at(rect);

                p.rect_filled(rect, Rounding::same(14.0), Color32::WHITE);
                p.rect_stroke(
                    rect,
                    Rounding::same(14.0),
                    Stroke::new(
                        1.0_f32,
                        if dev.is_connected {
                            Theme::ACCENT_BLUE
                        } else {
                            Theme::BORDER_LIGHT
                        },
                    ),
                );

                // Controller graphic thumbnail
                let thumb_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x + 14.0, rect.min.y + 14.0),
                    Vec2::new(32.0, 32.0),
                );
                p.rect_filled(
                    thumb_rect,
                    Rounding::same(8.0),
                    Color32::from_rgb(241, 245, 249),
                );
                p.circle_filled(thumb_rect.center(), 8.0, Theme::ACCENT_BLUE);

                // Device text labels
                p.text(
                    Pos2::new(rect.min.x + 56.0, rect.min.y + 16.0),
                    egui::Align2::LEFT_TOP,
                    &dev.name,
                    egui::FontId::proportional(14.5),
                    Theme::TEXT_PRIMARY,
                );
                p.text(
                    Pos2::new(rect.min.x + 56.0, rect.min.y + 38.0),
                    egui::Align2::LEFT_TOP,
                    format!("{} | {}", dev.model, dev.id),
                    egui::FontId::proportional(11.0),
                    Theme::TEXT_SECONDARY,
                );

                // Connection badge pill
                let status_lbl = if dev.is_connected {
                    "Connected"
                } else {
                    "Standby"
                };
                let bg_col = if dev.is_connected {
                    Color32::from_rgb(220, 252, 231)
                } else {
                    Color32::from_rgb(241, 245, 249)
                };
                let txt_col = if dev.is_connected {
                    Color32::from_rgb(22, 101, 52)
                } else {
                    Theme::TEXT_SECONDARY
                };

                let pill_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x + 56.0, rect.max.y - 28.0),
                    Vec2::new(68.0, 18.0),
                );
                p.rect_filled(pill_rect, Rounding::same(9.0), bg_col);
                p.text(
                    pill_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    status_lbl,
                    egui::FontId::proportional(10.0),
                    txt_col,
                );

                if resp.clicked() {
                    selected_device = Some(dev.id.clone());
                }
            }
        });
    });

    selected_device
}
