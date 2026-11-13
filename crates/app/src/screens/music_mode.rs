//! # Music Reactive Lights Screen
//!
//! Synchronizes strip lighting to music via Windows Media Session integration
//! (GSMTC) and WASAPI audio loopback FFT analysis.

use crate::catalog::get_music_mode_name;
use crate::state::AppState;
use crate::theme::{render_source_pill, Theme};
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

    ui.vertical_centered(|ui| {
        // 1. Large Circular Music Dial
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

        // 2. Windows System Media Transport Controls (GSMTC) Banner
        let banner_w = (ui.available_width() - 40.0).clamp(280.0, 480.0);
        let (banner_rect, _) =
            ui.allocate_exact_size(Vec2::new(banner_w, 42.0), egui::Sense::hover());
        let bp = ui.painter_at(banner_rect);

        bp.rect_filled(
            banner_rect,
            Rounding::same(10.0),
            Color32::from_rgb(241, 245, 249),
        );
        bp.rect_stroke(
            banner_rect,
            Rounding::same(10.0),
            Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
        );

        let icon_rect = Rect::from_min_size(
            Pos2::new(banner_rect.min.x + 10.0, banner_rect.min.y + 10.0),
            Vec2::splat(22.0),
        );
        bp.rect_filled(icon_rect, Rounding::same(5.0), Theme::ACCENT_BLUE);
        bp.text(
            icon_rect.center(),
            egui::Align2::CENTER_CENTER,
            "P",
            egui::FontId::proportional(11.0),
            Color32::WHITE,
        );

        let track_text = if !now_playing.title.is_empty() {
            format!(
                "Now Playing: {} - {}",
                now_playing.title, now_playing.artist
            )
        } else {
            "Windows Audio: Ready / Monitoring WASAPI Output".to_string()
        };

        bp.text(
            Pos2::new(banner_rect.min.x + 40.0, banner_rect.center().y),
            egui::Align2::LEFT_CENTER,
            track_text,
            egui::FontId::proportional(11.5),
            Theme::TEXT_PRIMARY,
        );

        ui.add_space(12.0);

        // 3. 16-Band Graphic Equalizer
        render_equalizer_bars(ui, state);

        ui.add_space(14.0);

        // 4. Audio Input Source Selector
        ui.horizontal(|ui| {
            let bar_w = ui.available_width();
            ui.add_space(((bar_w - 320.0) / 2.0).max(0.0));

            if render_source_pill(ui, "Auto Cycle", false).clicked() {
                // Auto cycle
            }
            ui.add_space(8.0);

            let is_stream = state.ui_audio_input == AudioInput::Player;
            if render_source_pill(ui, "PC Stream", is_stream).clicked() {
                state.ui_audio_input = AudioInput::Player;
                state.sync_with_windows_audio = true;
                state_changed = true;
            }
            ui.add_space(8.0);

            let is_host = state.ui_audio_input == AudioInput::ExternalMic;
            if render_source_pill(ui, "Host Mic", is_host).clicked() {
                state.ui_audio_input = AudioInput::ExternalMic;
                state.sync_with_windows_audio = false;
                state_changed = true;
            }
            ui.add_space(8.0);

            let is_int = state.ui_audio_input == AudioInput::InternalMic;
            if render_source_pill(ui, "Device Mic", is_int).clicked() {
                state.ui_audio_input = AudioInput::InternalMic;
                state.sync_with_windows_audio = false;
                state_changed = true;
            }
        });

        ui.add_space(12.0);

        // 5. Sensitivity Slider
        ui.horizontal(|ui| {
            let w = ui.available_width();
            ui.add_space(((w - 280.0) / 2.0).max(0.0));
            ui.label(
                egui::RichText::new("Mic Sensitivity:")
                    .size(12.0)
                    .color(Theme::TEXT_SECONDARY),
            );
            if ui
                .add(egui::Slider::new(&mut state.ui_sensitivity, 1..=16).show_value(true))
                .changed()
            {
                state_changed = true;
            }
        });
    });

    state_changed
}

fn render_equalizer_bars(ui: &mut egui::Ui, state: &AppState) {
    let eq_w = (ui.available_width() - 40.0).clamp(280.0, 480.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(eq_w, 60.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, Rounding::same(10.0), Color32::from_rgb(248, 250, 252));
    painter.rect_stroke(
        rect,
        Rounding::same(10.0),
        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
    );

    let bars = 16;
    let spacing = rect.width() / (bars as f32);
    let bar_w = spacing * 0.62;

    for i in 0..bars {
        let freq = i as f32 * 1.35;
        let height_factor = if state.ui_power {
            if state.sync_with_windows_audio && state.live_audio_peak > 0.01 {
                let harmonic = (state.audio_visualizer_timer * 12.0 + freq).sin() * 0.35 + 0.65;
                (state.live_audio_peak * harmonic).clamp(0.08, 0.98)
            } else {
                let wave = ((state.audio_visualizer_timer * 8.0 + freq).sin() * 0.4 + 0.5)
                    * ((state.audio_visualizer_timer * 3.7 + freq * 2.1).cos() * 0.3 + 0.7);
                wave.clamp(0.1, 0.95)
            }
        } else {
            0.04
        };

        let bar_h = rect.height() * height_factor * (state.ui_sensitivity as f32 / 16.0);
        let bar_rect = Rect::from_min_size(
            Pos2::new(
                rect.min.x + spacing * (i as f32 + 0.18),
                rect.max.y - bar_h - 2.0,
            ),
            Vec2::new(bar_w, bar_h),
        );

        let col = if i < 6 {
            Theme::ACCENT_BLUE
        } else if i < 11 {
            Theme::ACCENT_EMERALD
        } else {
            Theme::ACCENT_AMBER
        };

        painter.rect_filled(bar_rect, Rounding::same(3.0), col);
    }
}
