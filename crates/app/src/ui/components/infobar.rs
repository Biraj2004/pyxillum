//! # Fluent InfoBar Component
//!
//! Provides inline status alerts with semantic icon tiles, action buttons,
//! dismiss buttons, and animated height reveals.

use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Rect, Rounding, Stroke, Ui, Vec2};

/// Severity level for InfoBars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoBarSeverity {
    Info,
    Success,
    Warning,
    Danger,
}

pub struct InfoBar<'a> {
    severity: InfoBarSeverity,
    title: &'a str,
    message: &'a str,
    action_label: Option<&'a str>,
}

impl<'a> InfoBar<'a> {
    pub fn new(severity: InfoBarSeverity, title: &'a str, message: &'a str) -> Self {
        Self {
            severity,
            title,
            message,
            action_label: None,
        }
    }

    pub fn info(title: &'a str, message: &'a str) -> Self {
        Self::new(InfoBarSeverity::Info, title, message)
    }

    pub fn success(title: &'a str, message: &'a str) -> Self {
        Self::new(InfoBarSeverity::Success, title, message)
    }

    pub fn warning(title: &'a str, message: &'a str) -> Self {
        Self::new(InfoBarSeverity::Warning, title, message)
    }

    pub fn danger(title: &'a str, message: &'a str) -> Self {
        Self::new(InfoBarSeverity::Danger, title, message)
    }

    pub fn action(mut self, label: &'a str) -> Self {
        self.action_label = Some(label);
        self
    }

    /// Renders the InfoBar. Returns `(action_clicked, dismiss_clicked)`.
    pub fn show(self, ui: &mut Ui) -> (bool, bool) {
        let tokens = ThemeTokens::default_dark();

        let (fg, bg, icon) = match self.severity {
            InfoBarSeverity::Info => (tokens.semantic.info, tokens.semantic.info_bg, Icon::Info),
            InfoBarSeverity::Success => (
                tokens.semantic.success,
                tokens.semantic.success_bg,
                Icon::Success,
            ),
            InfoBarSeverity::Warning => (
                tokens.semantic.warning,
                tokens.semantic.warning_bg,
                Icon::Warning,
            ),
            InfoBarSeverity::Danger => (
                tokens.semantic.danger,
                tokens.semantic.danger_bg,
                Icon::Error,
            ),
        };

        let mut action_clicked = false;
        let mut dismiss_clicked = false;

        let frame = egui::Frame::none()
            .fill(bg)
            .rounding(tokens.radius.card_rounding())
            .stroke(Stroke::new(1.0_f32, fg))
            .inner_margin(egui::Margin::symmetric(14.0, 10.0));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Leading 32px rounded tile with semantic icon
                let (tile_rect, _) =
                    ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
                let p = ui.painter_at(tile_rect);
                p.rect_filled(tile_rect, Rounding::same(6.0), fg.linear_multiply(0.2));
                let icon_box = Rect::from_center_size(tile_rect.center(), Vec2::splat(16.0));
                icon.paint(&p, icon_box, fg);

                ui.add_space(8.0);

                // Title and message
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(self.title)
                            .font(TypeScale::body_strong())
                            .color(tokens.text.primary),
                    );
                    ui.label(
                        egui::RichText::new(self.message)
                            .font(TypeScale::caption())
                            .color(tokens.text.secondary),
                    );
                });

                // Optional Action Button & Dismiss Close Button on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (close_rect, close_resp) =
                        ui.allocate_exact_size(Vec2::splat(24.0), egui::Sense::click());
                    if ui.is_rect_visible(close_rect) {
                        let cp = ui.painter_at(close_rect);
                        let close_color = if close_resp.hovered() {
                            Color32::WHITE
                        } else {
                            tokens.text.tertiary
                        };
                        let close_box =
                            Rect::from_center_size(close_rect.center(), Vec2::splat(12.0));
                        Icon::Close.paint(&cp, close_box, close_color);
                    }
                    if close_resp.clicked() {
                        dismiss_clicked = true;
                    }

                    if let Some(act) = self.action_label {
                        ui.add_space(6.0);
                        if ui
                            .add(egui::Button::new(
                                egui::RichText::new(act)
                                    .font(TypeScale::caption())
                                    .color(fg),
                            ))
                            .clicked()
                        {
                            action_clicked = true;
                        }
                    }
                });
            });
        });

        (action_clicked, dismiss_clicked)
    }
}
