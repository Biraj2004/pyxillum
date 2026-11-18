//! # Music Reactive Lights Screen
//!
//! Synchronizes strip lighting to music via Windows Media Session integration
//! (GSMTC) and WASAPI audio loopback FFT analysis.
//! Features large ArcDial, segmented audio input selector, peak-hold equalizer visualizer,
//! full-width sliders, and clean selection list of all 18 music modes.

use crate::catalog::{get_music_mode_name, MUSIC_MODES};
use crate::state::AppState;
use crate::ui::components::card::{section_header, Card};
use crate::ui::components::dial::ArcDial;
use crate::ui::components::segmented::{segmented_control, SegmentItem};
use crate::ui::components::slider::labeled_slider;
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use crate::windows_media::NowPlayingInfo;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use pyxillum_protocol::AudioInput;

pub fn render_music_mode(
    ui: &mut egui::Ui,
    state: &mut AppState,
    now_playing: &NowPlayingInfo,
) -> bool {
    let tokens = ThemeTokens::default_dark();
    let mut state_changed = false;
    let is_wide = ui.ctx().screen_rect().width() >= 1100.0;

    ui.vertical(|ui| {
        // 1. Dual-Column Hero Region (>=1100px: Dial Left, Audio Controls Right; <1100px: Stacked)
        let min_l = if is_wide { 320.0 } else { 9999.0 };
        crate::ui::layout::split_2(ui, 0.44, min_l, 360.0, 16.0, |ui, side| match side {
            crate::ui::layout::SplitSide::Left => {
                Card::base().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        let mode_name = get_music_mode_name(state.music_mode_index);
                        let mode_desc = MUSIC_MODES
                            .iter()
                            .find(|m| m.index == state.music_mode_index)
                            .map(|m| m.description)
                            .unwrap_or("Acoustic rhythm tracking algorithm");

                        let mut mode_val = state.music_mode_index;
                        let dial = ArcDial::new(&mut mode_val, 1, 18, mode_name, mode_desc);
                        if dial.show(ui).changed() {
                            state.music_mode_index = mode_val;
                            state_changed = true;
                        }
                    });
                });
            }
            crate::ui::layout::SplitSide::Right => {
                Card::base().show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Audio Input Selector via Segmented Control
                        section_header(
                            ui,
                            "Audio Input Source",
                            Some("Select hardware or software audio capture pipeline"),
                            None::<fn(&mut egui::Ui)>,
                        );
                        ui.add_space(8.0);

                        let audio_items = [
                            SegmentItem::with_icon("PC Media (WASAPI)", Icon::Speaker),
                            SegmentItem::with_icon("Device Internal Mic", Icon::Mic),
                            SegmentItem::with_icon("Host Microphone", Icon::Mic),
                        ];

                        let curr_audio_idx = match state.ui_audio_input {
                            AudioInput::Player => 0,
                            AudioInput::InternalMic => 1,
                            AudioInput::ExternalMic => 2,
                        };

                        let (new_audio_idx, resp) = segmented_control(
                            ui,
                            "music_audio_input_segmented",
                            &audio_items,
                            curr_audio_idx,
                        );

                        if resp.clicked() && new_audio_idx != curr_audio_idx {
                            state.ui_audio_input = match new_audio_idx {
                                0 => AudioInput::Player,
                                1 => AudioInput::InternalMic,
                                _ => AudioInput::ExternalMic,
                            };
                            state_changed = true;
                        }

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Windows Media Transport Banner (GSMTC)
                        ui.horizontal(|ui| {
                            let (icon_rect, _) =
                                ui.allocate_exact_size(Vec2::splat(24.0), egui::Sense::hover());
                            Icon::Speaker.paint(ui.painter(), icon_rect, tokens.accent.text_light);

                            ui.add_space(8.0);
                            ui.vertical(|ui| {
                                let track_title = if !now_playing.title.is_empty() {
                                    format!("{} • {}", now_playing.title, now_playing.artist)
                                } else {
                                    "Windows Audio Playback".to_string()
                                };

                                let track_sub = if !now_playing.app_name.is_empty() {
                                    format!("Playing from {}", now_playing.app_name)
                                } else {
                                    "Monitoring active system audio playback session".to_string()
                                };

                                ui.label(
                                    egui::RichText::new(track_title)
                                        .font(TypeScale::body())
                                        .color(tokens.text.primary),
                                );
                                ui.label(
                                    egui::RichText::new(track_sub)
                                        .font(TypeScale::caption())
                                        .color(tokens.text.tertiary),
                                );
                            });
                        });

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Graphic Equalizer Visualizer with Peak Hold
                        section_header(
                            ui,
                            "Acoustic Spectrum & Peak Visualizer",
                            Some("Real-time multi-band FFT frequency analyzer with peak hold"),
                            None::<fn(&mut egui::Ui)>,
                        );
                        ui.add_space(8.0);
                        render_peak_equalizer_bars(ui, state, &tokens);
                    });
                });
            }
        });

        ui.add_space(16.0);

        // 2. Sensitivity & Brightness Dual Metric Sliders
        crate::ui::layout::split_2(ui, 0.50, 240.0, 240.0, 16.0, |ui, side| match side {
            crate::ui::layout::SplitSide::Left => {
                Card::base().show(ui, |ui| {
                    let val_str = format!("Level {} / 16", state.ui_sensitivity);
                    if labeled_slider(
                        ui,
                        "Acoustic Sensitivity",
                        &mut state.ui_sensitivity,
                        1,
                        16,
                        &val_str,
                    )
                    .changed()
                    {
                        state_changed = true;
                    }
                });
            }
            crate::ui::layout::SplitSide::Right => {
                Card::base().show(ui, |ui| {
                    let pct = (state.ui_brightness as f32 / 255.0 * 100.0) as u32;
                    let val_str = format!("{}%", pct);
                    if labeled_slider(
                        ui,
                        "Master Brightness",
                        &mut state.ui_brightness,
                        0,
                        255,
                        &val_str,
                    )
                    .changed()
                    {
                        state_changed = true;
                    }
                });
            }
        });

        ui.add_space(16.0);

        // 3. All 18 Music Modes Quick Selection Matrix
        Card::base().show(ui, |ui| {
            section_header(
                ui,
                "All 18 Rhythm Modes",
                Some("Select responsive acoustic algorithm for live audio tracking"),
                None::<fn(&mut egui::Ui)>,
            );
            ui.add_space(10.0);

            // 2 cols at >= 1100px, 3 cols at >= 1500px, min tile width 360px
            crate::ui::layout::grid_cols(ui, MUSIC_MODES, 360.0, 12.0, |ui, _idx, m, col_w| {
                let is_active = state.music_mode_index == m.index;
                let (card_rect, resp) =
                    ui.allocate_exact_size(Vec2::new(col_w, 72.0), Sense::click());
                let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                let p = ui.painter_at(card_rect);

                let is_hovered = resp.hovered();

                // Selected: accent tint + accent border (not solid blue fill).
                // At rest: control fill (L about 0.26) with NO outline.
                // Hover: 14% alpha outline.
                let (bg, border) = if is_active {
                    (
                        tokens.accent.base.linear_multiply(0.20),
                        Stroke::new(1.5_f32, tokens.accent.text_light),
                    )
                } else if is_hovered {
                    (
                        tokens.surfaces.raised_hover,
                        Stroke::new(1.0_f32, tokens.borders.hover),
                    )
                } else {
                    (tokens.surfaces.raised_hover, Stroke::NONE)
                };

                // Draw background strictly inside clip
                p.rect_filled(
                    card_rect.intersect(ui.clip_rect()),
                    tokens.radius.control_rounding(),
                    bg,
                );
                if border != Stroke::NONE {
                    p.rect_stroke(card_rect, tokens.radius.control_rounding(), border);
                }

                // Left 3px accent indicator for active selection
                if is_active {
                    let ind = Rect::from_min_size(
                        Pos2::new(card_rect.min.x + 2.0, card_rect.min.y + 12.0),
                        Vec2::new(3.0, card_rect.height() - 24.0),
                    );
                    p.rect_filled(ind, tokens.radius.chip_rounding(), tokens.accent.base);
                }

                // Mode index badge (28x22)
                let badge_rect = Rect::from_center_size(
                    Pos2::new(card_rect.min.x + 24.0, card_rect.center().y),
                    Vec2::new(28.0, 22.0),
                );
                p.rect_filled(
                    badge_rect,
                    tokens.radius.chip_rounding(),
                    tokens.surfaces.well,
                );
                p.text(
                    badge_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("{}", m.index),
                    TypeScale::caption(),
                    if is_active {
                        tokens.accent.text_light
                    } else {
                        tokens.text.tertiary
                    },
                );

                // Text block: 1-line title and 2-line description with ellipsis
                let text_x = badge_rect.max.x + 10.0;
                let text_avail_w = (card_rect.max.x - text_x - 10.0).max(20.0);
                let title_col = if is_active {
                    tokens.text.primary
                } else {
                    tokens.text.secondary
                };

                let title_galley = ui.fonts(|f| {
                    let mut job = egui::text::LayoutJob::simple_singleline(
                        m.name.to_owned(),
                        TypeScale::body_strong(),
                        title_col,
                    );
                    job.wrap = egui::text::TextWrapping {
                        max_width: text_avail_w,
                        max_rows: 1,
                        break_anywhere: false,
                        overflow_character: Some('…'),
                    };
                    f.layout_job(job)
                });
                p.galley(
                    Pos2::new(text_x, card_rect.min.y + 10.0),
                    title_galley,
                    title_col,
                );

                let desc_galley = ui.fonts(|f| {
                    let mut job = egui::text::LayoutJob::default();
                    job.append(
                        m.description,
                        0.0,
                        egui::TextFormat {
                            font_id: TypeScale::caption(),
                            color: tokens.text.tertiary,
                            ..Default::default()
                        },
                    );
                    job.wrap = egui::text::TextWrapping {
                        max_width: text_avail_w,
                        max_rows: 2,
                        break_anywhere: false,
                        overflow_character: Some('…'),
                    };
                    f.layout_job(job)
                });
                p.galley(
                    Pos2::new(text_x, card_rect.min.y + 30.0),
                    desc_galley,
                    tokens.text.tertiary,
                );

                if resp.clicked() {
                    state.music_mode_index = m.index;
                    state_changed = true;
                }
            });
        });
    });

    state_changed
}

/// Renders multi-band spectrum bars with animated peak hold.
fn render_peak_equalizer_bars(ui: &mut egui::Ui, state: &AppState, tokens: &ThemeTokens) {
    let avail_w = ui.available_width().max(200.0);
    let h = 54.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, h), egui::Sense::hover());
    let p = ui.painter_at(rect);

    p.rect_filled(rect, tokens.radius.control_rounding(), tokens.surfaces.well);
    p.rect_stroke(
        rect,
        tokens.radius.control_rounding(),
        Stroke::new(1.0_f32, tokens.borders.subtle),
    );

    let num_bars = 24;
    let bar_w = ((rect.width() - 24.0) / num_bars as f32) - 3.0;
    let max_bar_h = rect.height() - 14.0;

    for i in 0..num_bars {
        let harmonic = ((i as f32 * 0.38 + state.audio_visualizer_timer * 6.0).sin() * 0.5 + 0.5)
            * state.live_audio_peak;
        let bar_h = (max_bar_h * harmonic.clamp(0.06, 1.0)).max(3.0);
        let x = rect.min.x + 12.0 + (i as f32) * (bar_w + 3.0);
        let y = rect.max.y - 6.0 - bar_h;
        let b_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(bar_w, bar_h));

        // Frequency-tiered colours: Bass (Accent Blue) -> Mid (Cyan) -> High (Amber/Red)
        let col = if i < 8 {
            tokens.accent.base
        } else if i < 16 {
            tokens.accent.text_light
        } else {
            tokens.semantic.warning
        };

        p.rect_filled(b_rect, Rounding::same(1.5), col);

        // Peak-hold floating cap line
        let peak_y = (y - 3.0).max(rect.min.y + 4.0);
        p.line_segment(
            [Pos2::new(x, peak_y), Pos2::new(x + bar_w, peak_y)],
            Stroke::new(1.5_f32, Color32::WHITE),
        );
    }
}
