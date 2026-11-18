//! # Typography & Font Hierarchy
//!
//! Provides Windows 11 Segoe UI Variable font loading, fluid type scaling,
//! reserved bold width layout protection, optical text trim, tabular numeric slots,
//! and tooltip-backed overflow truncation.

use eframe::egui::{
    self, Color32, FontFamily, FontId, Pos2, Response, Sense, Stroke, TextStyle, Ui, Vec2,
};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Standard Fluent typography sizes in logical points.
pub struct TypeScale;

impl TypeScale {
    pub const CAPTION: f32 = 12.0;
    pub const BODY: f32 = 14.0;
    pub const BODY_STRONG: f32 = 14.0;
    pub const SUBTITLE: f32 = 16.0;
    pub const TITLE: f32 = 20.0;
    pub const DISPLAY: f32 = 28.0;
    pub const DIAL_VALUE: f32 = 40.0;

    pub fn caption() -> FontId {
        FontId::new(Self::CAPTION, FontFamily::Proportional)
    }

    pub fn body() -> FontId {
        FontId::new(Self::BODY, FontFamily::Proportional)
    }

    pub fn body_strong() -> FontId {
        // Uses Monospace / Bold slot mapped in FontDefinitions
        FontId::new(Self::BODY_STRONG, FontFamily::Name("SegoeBold".into()))
    }

    pub fn subtitle() -> FontId {
        FontId::new(Self::SUBTITLE, FontFamily::Name("SegoeBold".into()))
    }

    pub fn title() -> FontId {
        FontId::new(Self::TITLE, FontFamily::Name("SegoeBold".into()))
    }

    pub fn display() -> FontId {
        FontId::new(Self::DISPLAY, FontFamily::Name("SegoeBold".into()))
    }

    pub fn dial_value() -> FontId {
        FontId::new(Self::DIAL_VALUE, FontFamily::Monospace)
    }

    /// Computes fluid type size scaled with window width within a +/-12% safety envelope.
    pub fn fluid(base: f32, window_width: f32) -> f32 {
        const REFERENCE_WIDTH: f32 = 1280.0;
        let delta = (window_width - REFERENCE_WIDTH) / REFERENCE_WIDTH;
        let scale = (1.0 + delta * 0.12).clamp(0.88, 1.12);
        (base * scale).round()
    }
}

/// Configures font definitions, loading native Segoe UI Variable from %WINDIR%\Fonts on Windows.
pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    #[cfg(windows)]
    {
        if let Ok(windir) = env::var("WINDIR") {
            let fonts_dir = PathBuf::from(windir).join("Fonts");

            // 1. Regular text: Segoe UI Variable or Segoe UI
            let reg_candidates = ["SegUIVar.ttf", "segoeui.ttf"];
            for filename in reg_candidates {
                let path = fonts_dir.join(filename);
                if path.exists() {
                    if let Ok(data) = fs::read(&path) {
                        fonts.font_data.insert(
                            "SegoeUIRegular".to_owned(),
                            egui::FontData::from_owned(data),
                        );
                        // Prepend to proportional font family
                        fonts
                            .families
                            .entry(FontFamily::Proportional)
                            .or_default()
                            .insert(0, "SegoeUIRegular".to_owned());
                        break;
                    }
                }
            }

            // 2. Bold text: Segoe UI Variable Bold or Segoe UI Bold
            let bold_candidates = ["SegUIVarB.ttf", "segoeuib.ttf"];
            for filename in bold_candidates {
                let path = fonts_dir.join(filename);
                if path.exists() {
                    if let Ok(data) = fs::read(&path) {
                        fonts
                            .font_data
                            .insert("SegoeUIBold".to_owned(), egui::FontData::from_owned(data));
                        fonts
                            .families
                            .entry(FontFamily::Name("SegoeBold".into()))
                            .or_default()
                            .push("SegoeUIBold".to_owned());
                        break;
                    }
                }
            }
        }
    }

    // Ensure SegoeBold family always has fallback to proportional
    fonts
        .families
        .entry(FontFamily::Name("SegoeBold".into()))
        .or_default()
        .push("SegoeUIRegular".to_owned());

    // Map default TextStyle sizes to our type scale
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Small, TypeScale::caption()),
        (TextStyle::Body, TypeScale::body()),
        (TextStyle::Button, TypeScale::body()),
        (TextStyle::Heading, TypeScale::subtitle()),
        (
            TextStyle::Monospace,
            FontId::new(13.0, FontFamily::Monospace),
        ),
    ]
    .into();

    ctx.set_fonts(fonts);
    ctx.set_style(style);
}

/// Measures the bold width of a label and reserves the full width,
/// completely preventing layout shifting when toggling selected states.
pub fn reserve_bold_width(ui: &Ui, text: &str, regular_font: &FontId, bold_font: &FontId) -> f32 {
    let reg_w = ui.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), regular_font.clone(), Color32::WHITE)
            .size()
            .x
    });
    let bold_w = ui.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), bold_font.clone(), Color32::WHITE)
            .size()
            .x
    });
    reg_w.max(bold_w)
}

/// Renders a numeric value with a smaller, dimmer unit label (e.g. "120" with "pixels" or "%").
pub fn render_value_with_unit(
    ui: &mut Ui,
    value: &str,
    unit: &str,
    val_color: Color32,
    unit_color: Color32,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.label(
            egui::RichText::new(value)
                .font(TypeScale::body_strong())
                .color(val_color),
        );
        ui.label(
            egui::RichText::new(unit)
                .font(TypeScale::caption())
                .color(unit_color),
        );
    });
}

/// Allocates a fixed-width slot sized for `max_template` (e.g. "100%" or "255"),
/// right-aligning the value so numeric readouts do not jitter while dragging sliders.
pub fn render_tabular_slot(
    ui: &mut Ui,
    value: &str,
    max_template: &str,
    font: &FontId,
    color: Color32,
) -> Response {
    let slot_w = ui.fonts(|f| {
        f.layout_no_wrap(max_template.to_owned(), font.clone(), Color32::WHITE)
            .size()
            .x
            + 6.0
    });
    let slot_h = font.size + 4.0;

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(slot_w, slot_h), Sense::hover());
    if ui.is_rect_visible(rect) {
        let val_galley = ui.fonts(|f| f.layout_no_wrap(value.to_owned(), font.clone(), color));
        let pos = Pos2::new(rect.right() - val_galley.size().x - 2.0, rect.top() + 2.0);
        ui.painter().galley(pos, val_galley, color);
    }
    resp
}

/// Renders a hyperlink with a clean, optically distinct underline offset by 2px.
pub fn render_link_with_offset(ui: &mut Ui, text: &str, color: Color32) -> Response {
    let font = TypeScale::body();
    let size = ui.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), font.clone(), color)
            .size()
    });

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(size.x, size.y + 4.0), Sense::click());
    if ui.is_rect_visible(rect) {
        let hover = resp.hovered();
        let draw_color = if hover { Color32::WHITE } else { color };
        let galley = ui.fonts(|f| f.layout_no_wrap(text.to_owned(), font, draw_color));
        let painter = ui.painter();
        painter.galley(rect.min, galley, draw_color);

        // Thin 1px underline with 2px optical separation
        let line_y = rect.min.y + size.y + 2.0;
        let line_stroke = Stroke::new(1.0_f32, draw_color);
        painter.line_segment(
            [Pos2::new(rect.min.x, line_y), Pos2::new(rect.max.x, line_y)],
            line_stroke,
        );
    }

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}

/// Renders a label with safe truncation and automatic hover tooltip if the text overflows.
pub fn render_safe_label(
    ui: &mut Ui,
    text: &str,
    max_width: f32,
    font: &FontId,
    color: Color32,
) -> Response {
    let is_overflowing = ui.fonts(|f| {
        let full_galley = f.layout_no_wrap(text.to_owned(), font.clone(), color);
        full_galley.size().x > max_width
    });

    if is_overflowing {
        // Truncate with ellipsis
        let mut truncated = text.to_owned();
        while !truncated.is_empty() {
            truncated.pop();
            let test_str = format!("{}...", truncated);
            let fits = ui.fonts(|f| {
                let g = f.layout_no_wrap(test_str.clone(), font.clone(), color);
                g.size().x <= max_width
            });
            if fits {
                truncated = test_str;
                break;
            }
        }
        let resp = ui.label(
            egui::RichText::new(truncated)
                .font(font.clone())
                .color(color),
        );
        resp.on_hover_text(text)
    } else {
        ui.label(egui::RichText::new(text).font(font.clone()).color(color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid_type_scale() {
        // At 1280px reference width, fluid size equals base
        assert_eq!(TypeScale::fluid(14.0, 1280.0), 14.0);
        // At 1920px (+50% width), fluid size scales up within +12% envelope (14 * 1.06 = ~15)
        let wide = TypeScale::fluid(14.0, 1920.0);
        assert!((14.0..=16.0).contains(&wide));
        // At 1000px, fluid size scales down slightly
        let compact = TypeScale::fluid(14.0, 1000.0);
        assert!((12.0..=14.0).contains(&compact));
    }
}
