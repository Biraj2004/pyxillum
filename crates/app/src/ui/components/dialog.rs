//! # Fluent ContentDialog Component
//!
//! Provides modal dialogs with backdrop scrim, header icon tile, Esc dismissal,
//! and standard primary/secondary action buttons.

use crate::ui::components::button::FluentButton;
use crate::ui::icons::{header_tile_icon, icon_button_32, Icon};
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rounding, Ui, Vec2};

/// Triggers an error shake animation (+/-6px decaying over 300ms) on active modal dialogs.
pub fn trigger_dialog_shake(ctx: &egui::Context) {
    let shake_id = egui::Id::new("content_dialog_shake_timestamp");
    let now = ctx.input(|i| i.time);
    ctx.data_mut(|d| d.insert_temp(shake_id, now));
    ctx.request_repaint();
}

pub struct ContentDialog<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    icon: Option<Icon>,
    primary_text: Option<&'a str>,
    secondary_text: Option<&'a str>,
    max_width: f32,
}

impl<'a> ContentDialog<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            icon: None,
            primary_text: Some("OK"),
            secondary_text: Some("Cancel"),
            max_width: 520.0,
        }
    }

    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn primary_text(mut self, text: &'a str) -> Self {
        self.primary_text = Some(text);
        self
    }

    pub fn secondary_text(mut self, text: &'a str) -> Self {
        self.secondary_text = Some(text);
        self
    }

    pub fn max_width(mut self, max_w: f32) -> Self {
        self.max_width = max_w;
        self
    }

    /// Renders the modal dialog over a dark scrim. Returns `(primary_clicked, secondary_or_dismiss_clicked)`.
    pub fn show<R>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> (bool, bool) {
        let tokens = ThemeTokens::default_dark();
        let mut primary_clicked = false;
        let mut dismiss_clicked = false;

        // Esc key closes modal
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            dismiss_clicked = true;
        }

        // Fullscreen Scrim
        egui::Area::new(egui::Id::new("dialog_scrim"))
            .fixed_pos(Pos2::ZERO)
            .order(egui::Order::Middle)
            .show(ctx, |ui| {
                let screen_rect = ctx.screen_rect();
                ui.painter().rect_filled(
                    screen_rect,
                    Rounding::ZERO,
                    Color32::from_black_alpha(150),
                );
            });

        // Compute shake offset if recently triggered
        let shake_id = egui::Id::new("content_dialog_shake_timestamp");
        let now = ctx.input(|i| i.time);
        let shake_start: Option<f64> = ctx.data(|d| d.get_temp(shake_id));
        let shake_offset_x = if let Some(start) = shake_start {
            let elapsed = (now - start) as f32;
            if elapsed < 0.30 && crate::platform::windows::is_client_animation_enabled() {
                ctx.request_repaint();
                let progress = elapsed / 0.30;
                let decay = 1.0 - progress;
                (progress * std::f32::consts::PI * 8.0).sin() * 6.0 * decay
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Centered Content Dialog Window (with optional error shake offset)
        egui::Area::new(egui::Id::new("dialog_window"))
            .anchor(egui::Align2::CENTER_CENTER, Vec2::new(shake_offset_x, 0.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let frame = egui::Frame::none()
                    .fill(tokens.surfaces.dialog)
                    .rounding(tokens.radius.card_rounding())
                    .stroke(tokens.card_stroke())
                    .inner_margin(egui::Margin::same(20.0))
                    .shadow(egui::epaint::Shadow {
                        offset: Vec2::new(0.0, 16.0),
                        blur: 32.0,
                        spread: 0.0,
                        color: Color32::from_black_alpha(120),
                    });

                frame.show(ui, |ui| {
                    ui.set_max_width(self.max_width);
                    ui.set_min_width(380.0);

                    // Header Row: Icon Tile + Title/Subtitle + Close Button
                    ui.horizontal(|ui| {
                        if let Some(icon) = self.icon {
                            header_tile_icon(ui, icon, Color32::WHITE, tokens.accent.base);
                            ui.add_space(10.0);
                        }

                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(self.title)
                                    .font(TypeScale::title())
                                    .color(tokens.text.primary),
                            );
                            if let Some(sub) = self.subtitle {
                                ui.label(
                                    egui::RichText::new(sub)
                                        .font(TypeScale::caption())
                                        .color(tokens.text.secondary),
                                );
                            }
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if icon_button_32(ui, Icon::Close, "Close (Esc)", false).clicked() {
                                dismiss_clicked = true;
                            }
                        });
                    });

                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(14.0);

                    // Dialog Body
                    add_contents(ui);

                    ui.add_space(18.0);
                    ui.separator();
                    ui.add_space(14.0);

                    // Footer Buttons
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if let Some(prim_text) = self.primary_text {
                                if FluentButton::primary(prim_text).show(ui).clicked() {
                                    primary_clicked = true;
                                }
                            }
                            if let Some(sec_text) = self.secondary_text {
                                ui.add_space(8.0);
                                if FluentButton::new(sec_text).show(ui).clicked() {
                                    dismiss_clicked = true;
                                }
                            }
                        });
                    });
                });
            });

        (primary_clicked, dismiss_clicked)
    }
}
