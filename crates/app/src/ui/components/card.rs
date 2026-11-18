//! # Fluent Card & SectionHeader Component
//!
//! Provides elevation-tiered content cards adhering to the concentric radius rule
//! with 1px inner top specular highlights and white alpha borders.

use crate::ui::theme::ThemeTokens;
use eframe::egui::{self, Pos2, Response, Rounding, Stroke, Ui, Vec2};

/// Elevation tier for Cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardTier {
    Base,
    Raised,
    Modal,
}

/// Fluent container card with top specular highlight and alpha border.
pub struct Card {
    tier: CardTier,
    rounding: Option<Rounding>,
    padding: Option<Vec2>,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
        Self {
            tier: CardTier::Base,
            rounding: None,
            padding: None,
        }
    }

    pub fn base() -> Self {
        Self::new()
    }

    pub fn raised() -> Self {
        Self {
            tier: CardTier::Raised,
            rounding: None,
            padding: None,
        }
    }

    pub fn modal() -> Self {
        Self {
            tier: CardTier::Modal,
            rounding: None,
            padding: None,
        }
    }

    pub fn padding(mut self, padding: Vec2) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn rounding(mut self, rounding: Rounding) -> Self {
        self.rounding = Some(rounding);
        self
    }

    /// Renders the card container and executes the inner content closure.
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> Response {
        let tokens = ThemeTokens::default_dark();
        let bg = match self.tier {
            CardTier::Base => tokens.surfaces.card,
            CardTier::Raised => tokens.surfaces.raised_hover,
            CardTier::Modal => tokens.surfaces.dialog,
        };

        let rounding = self.rounding.unwrap_or(tokens.radius.card_rounding());
        let padding = self.padding.unwrap_or(Vec2::splat(tokens.spacing.lg));

        let frame = egui::Frame::none()
            .fill(bg)
            .rounding(rounding)
            .stroke(tokens.card_stroke())
            .inner_margin(egui::Margin::same(padding.x));

        let resp = frame.show(ui, |ui| add_contents(ui));

        // 1px Inner Top Specular Highlight (Native Windows 11 Fluent Card Detail)
        let rect = resp.response.rect;
        crate::ui::layout::track_widget_rect(ui, "card_outer_frame", rect);
        let p = ui.painter_at(rect);
        let highlight_y = rect.min.y + 1.0;
        let r = rounding.nw.min(6.0);
        p.line_segment(
            [
                Pos2::new(rect.min.x + r, highlight_y),
                Pos2::new(rect.max.x - r, highlight_y),
            ],
            Stroke::new(1.0_f32, tokens.borders.card_highlight),
        );

        resp.response
    }
}

/// Fluent section header with 16px semibold title, optional 12px secondary description,
/// and optional right-aligned action widget.
pub fn section_header(
    ui: &mut Ui,
    title: &str,
    subtitle: Option<&str>,
    action: Option<impl FnOnce(&mut Ui)>,
) {
    let tokens = ThemeTokens::default_dark();
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .font(crate::ui::typography::TypeScale::subtitle())
                    .color(tokens.text.primary),
            );
            if let Some(sub) = subtitle {
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new(sub)
                        .font(crate::ui::typography::TypeScale::caption())
                        .color(tokens.text.secondary),
                );
            }
        });

        if let Some(act) = action {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), act);
        }
    });
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_ban_raw_card_titles_in_screens() {
        let screens_dir = Path::new("src/screens");
        if !screens_dir.exists() {
            return;
        }

        let entries = fs::read_dir(screens_dir).expect("Failed to read screens directory");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let content = fs::read_to_string(&path).expect("Failed to read screen file");
                let lines: Vec<&str> = content.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    if line.contains("Card::base().show") {
                        for j in 1..=6 {
                            if let Some(next_line) = lines.get(i + j) {
                                if next_line.contains("ui.label(egui::RichText::new")
                                    && (next_line.contains("TypeScale::title()")
                                        || next_line.contains("TypeScale::body_strong()"))
                                {
                                    panic!(
                                        "Architecture rule violation in {}:{}: Raw RichText title used inside Card instead of section_header()",
                                        path.display(),
                                        i + j + 1
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
