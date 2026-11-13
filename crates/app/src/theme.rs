//! # Visual Styling & Theme Definitions
//!
//! Provides the desktop Fluent/Acrylic styling, colour constants,
//! vector drawing helpers, and responsive card containers.

use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

#[allow(dead_code)]
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // Primary Brand & Accent
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(37, 99, 235);
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(6, 182, 212);
    pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(16, 185, 129);
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(245, 158, 11);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(239, 68, 68);

    // Desktop Canvas & Surface
    pub const CANVAS_BG: Color32 = Color32::from_rgb(236, 243, 252);
    pub const CARD_BG: Color32 = Color32::from_rgb(255, 255, 255);
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(244, 247, 251);
    pub const SIDEBAR_BORDER: Color32 = Color32::from_rgb(226, 232, 240);

    // Neutral Text Hierarchy
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(15, 23, 42);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(100, 116, 139);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);

    // Card Borders
    pub const BORDER_LIGHT: Color32 = Color32::from_rgb(226, 232, 240);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(59, 130, 246);
}

/// Helper that renders a circular action button (e.g. Loop, Play/Pause, Fav, List).
pub fn render_round_action_button(
    ui: &mut egui::Ui,
    text: &str,
    is_active: bool,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(40.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let bg = if is_active {
        Color32::from_rgb(239, 246, 255)
    } else {
        Color32::from_rgb(248, 250, 252)
    };
    let border = if is_active {
        Theme::ACCENT_BLUE
    } else {
        Theme::BORDER_LIGHT
    };
    let txt_col = if is_active {
        Theme::ACCENT_BLUE
    } else {
        Theme::TEXT_SECONDARY
    };

    p.circle_filled(rect.center(), 19.0, bg);
    p.circle_stroke(rect.center(), 19.0, Stroke::new(1.0_f32, border));
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(12.0),
        txt_col,
    );

    resp
}

/// Helper that renders a segmented pill or source switcher tab.
pub fn render_source_pill(ui: &mut egui::Ui, text: &str, is_selected: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(76.0, 32.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let bg = if is_selected {
        Theme::ACCENT_BLUE
    } else {
        Color32::from_rgb(241, 245, 249)
    };
    let txt_col = if is_selected {
        Color32::WHITE
    } else {
        Theme::TEXT_SECONDARY
    };

    p.rect_filled(rect, Rounding::same(16.0), bg);
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(10.5),
        txt_col,
    );

    resp
}

/// Helper to render a desktop card container with subtle shadow and border.
#[allow(dead_code)]
pub fn render_card<R>(
    ui: &mut egui::Ui,
    rounding: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::none()
        .fill(Theme::CARD_BG)
        .rounding(Rounding::same(rounding))
        .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
        .inner_margin(egui::Margin::symmetric(20.0, 18.0))
        .show(ui, add_contents)
}
