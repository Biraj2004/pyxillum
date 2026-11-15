//! # Music Reactive Lights Screen
//!
//! Synchronizes strip lighting to music via Windows Media Session integration
//! (GSMTC) and WASAPI audio loopback FFT analysis.

use crate::catalog::get_music_mode_name;
use crate::state::AppState;
use crate::theme::{render_card, render_source_pill, Theme};
use crate::widgets::ArcDial;
use crate::windows_media::NowPlayingInfo;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_protocol::AudioInput;

pub fn render_music_mode(
    ui: &mut egui::Ui,
    state: &mut AppState,
    now_playing: &NowPlayingInfo,
) -> bool {
    let mut state_changed = false;

    // Constrain width to 680px centered for balanced desktop proportion
    let max_w = 680.0_f32.min(ui.available_width());
    let pad = ((ui.available_width() - max_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_max_width(max_w);

            // 1. Music Dial Hero Card
            render_card(ui, 8.0, |ui| {
                ui.vertical_centered(|ui| {
                    let mode_name = get_music_mode_name(state.music_mode_index);
                    let mut dial = ArcDial::new(state.music_mode_index as u32, 18, mode_name);
                    if let Some(delta) = dial.show(ui) {
                        let next_val = (state.music_mode_index as i32 + delta).clamp(1, 18) as u8;
                        if next_val != state.music_mode_index {
                            state.music_mode_index = next_val;
                            state_changed = true;
                        }
                    }

                    ui.add_space(10.0);

                    // Audio Source Selector Pills
                    ui.horizontal(|ui| {
                        let bar_w = ui.available_width();
                        ui.add_space(((bar_w - 300.0) / 2.0).max(0.0));

                        let is_stream = state.ui_audio_input == AudioInput::Player;
                        if render_source_pill(ui, "PC Media", is_stream).clicked() {
                            state.ui_audio_input = AudioInput::Player;
                            state_changed = true;
                        }
                        ui.add_space(8.0);

                        let is_int_mic = state.ui_audio_input == AudioInput::InternalMic;
                        if render_source_pill(ui, "Device Mic", is_int_mic).clicked() {
                            state.ui_audio_input = AudioInput::InternalMic;
                            state_changed = true;
                        }
                        ui.add_space(8.0);

                        let is_ext_mic = state.ui_audio_input == AudioInput::ExternalMic;
                        if render_source_pill(ui, "Host Mic", is_ext_mic).clicked() {
                            state.ui_audio_input = AudioInput::ExternalMic;
                            state_changed = true;
                        }
                    });
                });
            });

            ui.add_space(12.0);

            // 2. Windows Media Transport Banner (GSMTC)
            let (banner_rect, _) =
                ui.allocate_exact_size(Vec2::new(max_w, 44.0), egui::Sense::hover());
            let bp = ui.painter_at(banner_rect);

            bp.rect_filled(banner_rect, Rounding::same(8.0), Theme::CARD_BG);
            bp.rect_stroke(
                banner_rect,
                Rounding::same(8.0),
                Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
            );

            let icon_rect = Rect::from_min_size(
                Pos2::new(banner_rect.min.x + 12.0, banner_rect.center().y - 10.0),
                Vec2::splat(20.0),
            );
            bp.rect_filled(icon_rect, Rounding::same(4.0), Theme::ACCENT_BLUE);
            bp.text(
                icon_rect.center(),
                egui::Align2::CENTER_CENTER,
                "M",
                egui::FontId::proportional(11.0),
                Color32::WHITE,
            );

            let track_text = if !now_playing.title.is_empty() {
                format!(
                    "Now Playing: {} - {}",
                    now_playing.title, now_playing.artist
                )
            } else {
                "Windows Audio: Monitoring active system audio playback".to_string()
            };

            bp.text(
                Pos2::new(banner_rect.min.x + 40.0, banner_rect.center().y),
                egui::Align2::LEFT_CENTER,
                track_text,
                egui::FontId::proportional(12.0),
                Theme::TEXT_PRIMARY,
            );

            ui.add_space(12.0);

            // 3. Graphic Equalizer Visualizer Card
            render_card(ui, 8.0, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Acoustic Spectrum")
                            .size(12.0)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.add_space(6.0);
                    render_equalizer_bars(ui, state);
                });
            });

            ui.add_space(12.0);

            // 4. Sensitivity & Brightness Sliders
            ui.columns(2, |cols| {
                // Sensitivity Card
                cols[0].group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Acoustic Sensitivity")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.label(
                            egui::RichText::new(format!("Level {} / 16", state.ui_sensitivity))
                                .strong()
                                .size(18.0)
                                .color(Theme::ACCENT_BLUE),
                        );
                        ui.add_space(4.0);
                        if ui
                            .add(
                                egui::Slider::new(&mut state.ui_sensitivity, 1..=16)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            state_changed = true;
                        }
                    });
                });

                // Brightness Card
                cols[1].group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Master Brightness")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        let pct = (state.ui_brightness as f32 / 255.0 * 100.0) as u32;
                        ui.label(
                            egui::RichText::new(format!("{}%", pct))
                                .strong()
                                .size(18.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(4.0);
                        if ui
                            .add(
                                egui::Slider::new(&mut state.ui_brightness, 0..=255)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            state_changed = true;
                        }
                    });
                });
            });
        });
    });

    state_changed
}

/// Helper that renders 16 animated spectrum bars tracking live audio peak.
fn render_equalizer_bars(ui: &mut egui::Ui, state: &AppState) {
    let avail_w = ui.available_width().max(200.0);
    let h = 48.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, h), egui::Sense::hover());
    let p = ui.painter_at(rect);

    p.rect_filled(rect, Rounding::same(6.0), Theme::WELL_BG);

    let num_bars = 16;
    let bar_w = ((rect.width() - 20.0) / num_bars as f32) - 4.0;
    let max_bar_h = rect.height() - 12.0;

    for i in 0..num_bars {
        let harmonic = ((i as f32 * 0.45 + state.audio_visualizer_timer * 6.0).sin() * 0.5 + 0.5)
            * state.live_audio_peak;
        let bar_h = (max_bar_h * harmonic.clamp(0.08, 1.0)).max(3.0);
        let x = rect.min.x + 10.0 + (i as f32) * (bar_w + 4.0);
        let y = rect.max.y - 6.0 - bar_h;
        let b_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(bar_w, bar_h));

        let col = if i < 10 {
            Theme::ACCENT_BLUE
        } else if i < 13 {
            Theme::ACCENT_CYAN
        } else {
            Theme::ACCENT_AMBER
        };

        p.rect_filled(b_rect, Rounding::same(2.0), col);
    }
}
