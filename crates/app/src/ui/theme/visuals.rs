//! # Egui Visuals & Style Application
//!
//! Applies the Windows 11 Fluent tokens into `egui::Visuals`, `egui::Style`,
//! and widget interaction defaults.

use super::tokens::ThemeTokens;
use eframe::egui::{self, Color32, Stroke, Vec2};

/// Configures egui context visuals and spacing according to theme tokens.
pub fn configure_egui_theme(ctx: &egui::Context, tokens: &ThemeTokens) {
    let mut visuals = egui::Visuals::dark();

    visuals.dark_mode = true;
    visuals.override_text_color = Some(tokens.text.primary);

    // Canvas / Panel / Window fills
    visuals.panel_fill = tokens.surfaces.canvas;
    visuals.window_fill = tokens.surfaces.dialog;
    visuals.window_stroke = tokens.card_stroke();
    visuals.window_rounding = tokens.radius.card_rounding();
    visuals.extreme_bg_color = tokens.surfaces.well;

    // Non-interactive widgets (frames, separators, labels)
    visuals.widgets.noninteractive.bg_fill = tokens.surfaces.card;
    visuals.widgets.noninteractive.bg_stroke = tokens.card_stroke();
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, tokens.text.secondary);
    visuals.widgets.noninteractive.rounding = tokens.radius.control_rounding();

    // Inactive interactive widgets (standard buttons, inputs)
    visuals.widgets.inactive.bg_fill = tokens.surfaces.card;
    visuals.widgets.inactive.bg_stroke = tokens.card_stroke();
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, tokens.text.primary);
    visuals.widgets.inactive.rounding = tokens.radius.control_rounding();

    // Hovered interactive widgets
    visuals.widgets.hovered.bg_fill = tokens.surfaces.raised_hover;
    visuals.widgets.hovered.bg_stroke = tokens.card_stroke_hover();
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, tokens.text.primary);
    visuals.widgets.hovered.rounding = tokens.radius.control_rounding();

    // Active / Pressed interactive widgets
    visuals.widgets.active.bg_fill = tokens.accent.pressed;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, tokens.accent.base);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.active.rounding = tokens.radius.control_rounding();

    // Open menus / dropdowns
    visuals.widgets.open = visuals.widgets.hovered;

    // Selection highlights
    visuals.selection.bg_fill = tokens.accent.base;
    visuals.selection.stroke = Stroke::new(1.0_f32, tokens.accent.hover);

    // Text cursor & hyperlinks
    visuals.text_cursor.stroke = Stroke::new(2.0_f32, tokens.accent.base);
    visuals.hyperlink_color = tokens.accent.text_light;

    // Fluent top highlight on cards & window drop shadow
    visuals.window_shadow = egui::epaint::Shadow {
        offset: Vec2::new(0.0, 12.0),
        blur: 28.0,
        spread: 0.0,
        color: Color32::from_black_alpha(90),
    };

    ctx.set_visuals(visuals);

    // Spacing configuration (4px grid aligned)
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(tokens.spacing.sm, tokens.spacing.sm);
    style.spacing.window_margin = egui::Margin::same(tokens.spacing.xxl);
    style.spacing.button_padding = Vec2::new(tokens.spacing.md, tokens.spacing.sm);
    style.spacing.icon_spacing = tokens.spacing.sm;
    style.spacing.indent = tokens.spacing.lg;
    style.spacing.interact_size = Vec2::new(tokens.spacing.hero, tokens.spacing.hero);

    ctx.set_style(style);
}
