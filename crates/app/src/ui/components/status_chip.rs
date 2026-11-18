//! # Fluent ConnectionStatusChip & EmptyState Components
//!
//! Provides connection status chips with pulsing indicators and unified empty states.

use crate::ui::components::button::FluentButton;
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Connection state variant for status chips.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    NotConnected,
    Scanning,
    Connecting,
    Connected(String),
}

/// Renders an authentic Windows 11 status chip.
pub fn connection_status_chip(ui: &mut Ui, status: &ConnectionStatus) -> Response {
    let tokens = ThemeTokens::default_dark();
    let font = TypeScale::caption();

    let (label_text, dot_color, is_pulsing) = match status {
        ConnectionStatus::NotConnected => ("Not connected", tokens.text.disabled, false),
        ConnectionStatus::Scanning => ("Scanning...", tokens.semantic.warning, true),
        ConnectionStatus::Connecting => ("Connecting...", tokens.accent.text_light, true),
        ConnectionStatus::Connected(dev) => (dev.as_str(), tokens.semantic.success, false),
    };

    let text_size = ui.fonts(|f| {
        f.layout_no_wrap(label_text.to_owned(), font.clone(), Color32::WHITE)
            .size()
    });
    let chip_w = text_size.x + 28.0;
    let chip_h = 24.0_f32;

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(chip_w, chip_h), Sense::hover());

    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        p.rect_filled(rect, tokens.radius.chip_rounding(), tokens.surfaces.well);
        p.rect_stroke(
            rect,
            tokens.radius.chip_rounding(),
            Stroke::new(1.0_f32, tokens.borders.subtle),
        );

        // Status indicator dot
        let dot_center = Pos2::new(rect.min.x + 10.0, rect.center().y);
        let dot_r = 3.5_f32;

        if is_pulsing {
            let time = ui.input(|i| i.time);
            let pulse_r = dot_r + ((time * 4.0).sin().abs() * 2.0) as f32;
            p.circle_filled(dot_center, pulse_r, dot_color.linear_multiply(0.3));
        }

        p.circle_filled(dot_center, dot_r, dot_color);

        // Status label
        let text_pos = Pos2::new(rect.min.x + 20.0, rect.center().y - text_size.y / 2.0);
        let galley =
            ui.fonts(|f| f.layout_no_wrap(label_text.to_owned(), font, tokens.text.secondary));
        p.galley(text_pos, galley, tokens.text.secondary);
    }

    resp
}

/// Renders a unified empty state with hero icon, message, and ONE primary action button.
pub fn empty_state(
    ui: &mut Ui,
    icon: Icon,
    title: &str,
    description: &str,
    action_label: &str,
) -> bool {
    let tokens = ThemeTokens::default_dark();
    let mut action_clicked = false;

    ui.vertical_centered(|ui| {
        ui.add_space(32.0);

        // Hero Icon Circle
        let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
        let p = ui.painter_at(icon_rect);
        p.circle_filled(icon_rect.center(), 28.0, tokens.surfaces.raised_hover);
        p.circle_stroke(
            icon_rect.center(),
            28.0,
            Stroke::new(1.0_f32, tokens.borders.subtle),
        );
        let inner_box = Rect::from_center_size(icon_rect.center(), Vec2::splat(24.0));
        icon.paint(&p, inner_box, tokens.accent.text_light);

        ui.add_space(16.0);

        // Title
        ui.label(
            egui::RichText::new(title)
                .font(TypeScale::subtitle())
                .color(tokens.text.primary),
        );

        ui.add_space(4.0);

        // Description
        ui.label(
            egui::RichText::new(description)
                .font(TypeScale::caption())
                .color(tokens.text.secondary),
        );

        ui.add_space(20.0);

        // Single Primary Action Button
        if FluentButton::primary(action_label).show(ui).clicked() {
            action_clicked = true;
        }

        ui.add_space(32.0);
    });

    action_clicked
}
