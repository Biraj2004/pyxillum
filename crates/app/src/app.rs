//! # Pyxillum Desktop Application Coordinator
//!
//! Orchestrates the multi-column desktop layout, screen-wise view routing,
//! real-time 60 FPS strip simulation, and background command dispatch.

use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_core::DeviceState;
use pyxillum_protocol::{AudioInput, Command, LightMode, RgbColor};
use pyxillum_sim::DeviceSimulator;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex};

use crate::screens::{
    render_device_hub, render_diy_mode, render_dynamic_mode, render_music_mode,
    render_settings_modal, render_solid_mode, DeviceHubAction,
};
use crate::state::{
    AppScreen, AppState, CustomColorEffect, DashboardMode, DeviceCardInfo, GuiCommand, GuiEvent,
};
use crate::theme::{render_secondary_button, render_toggle_switch, Theme};
use crate::widgets::{render_effect_drawer, render_neon_strip, render_sidebar};
use crate::windows_media::{NowPlayingInfo, WindowsAudioService};

pub struct PyxillumApp {
    cmd_tx: mpsc::Sender<GuiCommand>,
    event_rx: mpsc::Receiver<GuiEvent>,
    state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
    _simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
    audio_service: WindowsAudioService,

    pub state: AppState,
    pub saved_devices: Vec<DeviceCardInfo>,
    now_playing: NowPlayingInfo,

    last_frame_instant: Instant,

    // Last sent hardware state to prevent redundant packet queuing
    last_sent_effect: u8,
    last_sent_speed: u8,
    last_sent_brightness: u8,
    last_sent_loop_all: bool,
    last_sent_color: [u8; 3],
    last_sent_custom_effect: CustomColorEffect,
    last_sent_audio_input: AudioInput,
    last_sent_sensitivity: u8,
    last_sent_music_mode: u8,

    // Real-time BLE brightness streaming for dynamic custom colour effects
    last_stream_instant: Instant,
    last_streamed_level: u8,

    // Window visibility and session persistence
    has_shown_window: bool,
    monitor_id: Option<String>,
    scale: f64,
    target_rect: crate::window_geom::Rect,
    last_valid_rect: Option<crate::window_geom::Rect>,
    is_maximized: bool,

    // Design system tokens
    #[allow(dead_code)]
    pub tokens: crate::ui::theme::ThemeTokens,
}

impl PyxillumApp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        cmd_tx: mpsc::Sender<GuiCommand>,
        event_rx: mpsc::Receiver<GuiEvent>,
        state_reader: Arc<tokio::sync::RwLock<DeviceState>>,
        simulator_ref: Option<Arc<Mutex<DeviceSimulator>>>,
        monitor_id: Option<String>,
        scale: f64,
        target_rect: crate::window_geom::Rect,
    ) -> Self {
        // Configure authentic Windows 11 Fluent dark tokens and typography
        let tokens = crate::ui::theme::ThemeTokens::default_dark();
        crate::ui::theme::configure_egui_theme(&cc.egui_ctx, &tokens);
        crate::ui::typography::configure_fonts(&cc.egui_ctx);

        let audio_service = WindowsAudioService::new();

        Self {
            cmd_tx,
            event_rx,
            state_reader,
            _simulator_ref: simulator_ref,
            audio_service,

            state: AppState::default(),
            saved_devices: Vec::new(),
            now_playing: NowPlayingInfo::default(),

            last_frame_instant: Instant::now(),

            last_sent_effect: 1,
            last_sent_speed: 5,
            last_sent_brightness: 255,
            last_sent_loop_all: false,
            last_sent_color: [255, 120, 0],
            last_sent_custom_effect: CustomColorEffect::Solid,
            last_sent_audio_input: AudioInput::InternalMic,
            last_sent_sensitivity: 8,
            last_sent_music_mode: 1,

            last_stream_instant: Instant::now(),
            last_streamed_level: 255,

            has_shown_window: false,
            monitor_id,
            scale,
            target_rect,
            last_valid_rect: Some(target_rect),
            is_maximized: false,
            tokens,
        }
    }

    fn send(&self, cmd: GuiCommand) {
        let _ = self.cmd_tx.try_send(cmd);
    }

    /// Computes the 60 FPS real-time LED strip rendering buffer.
    fn update_strip_simulation(&mut self, dt: f32) {
        self.state.animation_timer += dt;
        self.state.audio_visualizer_timer += dt;

        let num_pixels = self.state.ui_length as usize;
        let brightness_factor = if self.state.ui_power {
            self.state.ui_brightness as f32 / 255.0
        } else {
            0.0
        };

        self.state.strip_pixels.clear();

        match self.state.dashboard_mode {
            DashboardMode::Solid => {
                let r_base = self.state.ui_color[0] * 255.0;
                let g_base = self.state.ui_color[1] * 255.0;
                let b_base = self.state.ui_color[2] * 255.0;
                let speed_mult = self.state.custom_effect_speed as f32 * 0.4;
                let t = self.state.animation_timer * speed_mult;

                match self.state.custom_color_effect {
                    CustomColorEffect::Solid => {
                        let r = (r_base * brightness_factor) as u8;
                        let g = (g_base * brightness_factor) as u8;
                        let b = (b_base * brightness_factor) as u8;
                        let col = RgbColor::new(r, g, b);
                        for _ in 0..num_pixels {
                            self.state.strip_pixels.push(col);
                        }
                    }
                    CustomColorEffect::Breathe => {
                        let wave = ((t * 2.5).sin() * 0.5 + 0.5).powf(1.8);
                        let factor = (0.12 + 0.88 * wave) * brightness_factor;
                        let col = RgbColor::new(
                            (r_base * factor) as u8,
                            (g_base * factor) as u8,
                            (b_base * factor) as u8,
                        );
                        for _ in 0..num_pixels {
                            self.state.strip_pixels.push(col);
                        }
                    }
                    CustomColorEffect::Strobe => {
                        let phase = (t * 6.0).fract();
                        let flash = if phase < 0.22 { 1.0 } else { 0.04 };
                        let factor = flash * brightness_factor;
                        let col = RgbColor::new(
                            (r_base * factor) as u8,
                            (g_base * factor) as u8,
                            (b_base * factor) as u8,
                        );
                        for _ in 0..num_pixels {
                            self.state.strip_pixels.push(col);
                        }
                    }
                    CustomColorEffect::Pulse => {
                        let cycle = (t * 2.2).fract();
                        let p = if cycle < 0.15 {
                            (cycle / 0.15 * std::f32::consts::PI).sin()
                        } else if (0.24..0.39).contains(&cycle) {
                            ((cycle - 0.24) / 0.15 * std::f32::consts::PI).sin() * 0.7
                        } else {
                            0.05
                        };
                        let factor = (0.08 + 0.92 * p) * brightness_factor;
                        let col = RgbColor::new(
                            (r_base * factor) as u8,
                            (g_base * factor) as u8,
                            (b_base * factor) as u8,
                        );
                        for _ in 0..num_pixels {
                            self.state.strip_pixels.push(col);
                        }
                    }
                    CustomColorEffect::Wave => {
                        for i in 0..num_pixels {
                            let spatial = ((i as f32 * 0.25 - t * 4.0).sin() * 0.5 + 0.5).powi(2);
                            let factor = (0.1 + 0.9 * spatial) * brightness_factor;
                            let col = RgbColor::new(
                                (r_base * factor) as u8,
                                (g_base * factor) as u8,
                                (b_base * factor) as u8,
                            );
                            self.state.strip_pixels.push(col);
                        }
                    }
                    CustomColorEffect::Comet => {
                        let pos = (t * 4.0).rem_euclid(num_pixels as f32 + 12.0) - 6.0;
                        let tail_len = 10.0_f32;
                        for i in 0..num_pixels {
                            let dist = pos - i as f32;
                            let intensity = if dist >= 0.0 && dist <= tail_len {
                                (1.0 - dist / tail_len).powf(1.6)
                            } else {
                                0.03
                            };
                            let factor = intensity * brightness_factor;
                            let col = RgbColor::new(
                                (r_base * factor) as u8,
                                (g_base * factor) as u8,
                                (b_base * factor) as u8,
                            );
                            self.state.strip_pixels.push(col);
                        }
                    }
                    CustomColorEffect::Stars => {
                        for i in 0..num_pixels {
                            let sparkle = ((i as f32 * 23.7 + t * 6.5).sin() * 0.5 + 0.5).powi(3);
                            let factor = (0.06 + 0.94 * sparkle) * brightness_factor;
                            let col = RgbColor::new(
                                (r_base * factor) as u8,
                                (g_base * factor) as u8,
                                (b_base * factor) as u8,
                            );
                            self.state.strip_pixels.push(col);
                        }
                    }
                    CustomColorEffect::Sweep => {
                        let (h_base, s_base, v_base) = pyxillum_sim::render::rgb_to_hsv(
                            self.state.ui_color[0],
                            self.state.ui_color[1],
                            self.state.ui_color[2],
                        );
                        for i in 0..num_pixels {
                            let offset = (i as f32 * 0.15 + t * 2.0).sin() * 35.0;
                            let h = (h_base + offset).rem_euclid(360.0);
                            let col = pyxillum_sim::render::hsv_to_rgb(h, s_base, v_base);
                            self.state
                                .strip_pixels
                                .push(pyxillum_sim::render::scale_color(col, brightness_factor));
                        }
                    }
                }
            }
            DashboardMode::Diy => {
                if self.state.diy_nodes.is_empty() {
                    for _ in 0..num_pixels {
                        self.state
                            .strip_pixels
                            .push(pyxillum_protocol::RgbColor::BLACK);
                    }
                    return;
                }
                let speed_mult = self.state.diy_speed as f32 * 0.4;
                let t = self.state.animation_timer * speed_mult;
                let node_count = self.state.diy_nodes.len();
                let is_gradient =
                    self.state.diy_effect_type == crate::state::DiyEffectType::Gradient;

                match self.state.diy_pattern {
                    crate::state::DiyPatternStyle::Chase => {
                        let float_offset = t * 3.5;
                        for i in 0..num_pixels {
                            let float_idx = match self.state.diy_direction {
                                crate::state::DiyDirection::Forward => {
                                    ((i as f32 * 0.4) + float_offset).abs()
                                }
                                crate::state::DiyDirection::Backward => (10000.0
                                    * node_count as f32
                                    - ((i as f32 * 0.4) - float_offset))
                                    .abs(),
                                crate::state::DiyDirection::MiddleToOut => {
                                    let mid = num_pixels as f32 * 0.5;
                                    let dist = (i as f32 - mid).abs();
                                    (dist * 0.5 + float_offset).abs()
                                }
                                crate::state::DiyDirection::OutToMiddle => {
                                    let mid = num_pixels as f32 * 0.5;
                                    let dist = (i as f32 - mid).abs();
                                    (10000.0 * node_count as f32 - (dist * 0.5 - float_offset))
                                        .abs()
                                }
                            };

                            let idx1 = (float_idx as usize) % node_count;
                            let idx2 = (idx1 + 1) % node_count;
                            let frac = float_idx.fract();

                            let n1 = &self.state.diy_nodes[idx1];
                            let raw_color = if is_gradient {
                                let n2 = &self.state.diy_nodes[idx2];
                                pyxillum_sim::render::lerp_color(n1.color, n2.color, frac)
                            } else {
                                n1.color
                            };

                            let weight_scale = (n1.value as f32 / 255.0).clamp(0.05, 1.0);
                            let final_scale = brightness_factor * weight_scale;
                            let scaled = pyxillum_sim::render::scale_color(raw_color, final_scale);
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                    crate::state::DiyPatternStyle::Flow => {
                        let float_offset = t * 2.0;
                        for i in 0..num_pixels {
                            let norm_pos = match self.state.diy_direction {
                                crate::state::DiyDirection::Forward => {
                                    (i as f32 / num_pixels.max(1) as f32) * (node_count as f32)
                                        + float_offset
                                }
                                crate::state::DiyDirection::Backward => {
                                    10000.0 * node_count as f32
                                        + ((num_pixels - 1 - i) as f32 / num_pixels.max(1) as f32)
                                            * (node_count as f32)
                                        + float_offset
                                }
                                crate::state::DiyDirection::MiddleToOut => {
                                    let mid = num_pixels as f32 * 0.5;
                                    let dist = (i as f32 - mid).abs() / mid.max(1.0);
                                    dist * (node_count as f32) + float_offset
                                }
                                crate::state::DiyDirection::OutToMiddle => {
                                    let mid = num_pixels as f32 * 0.5;
                                    let dist = 1.0 - ((i as f32 - mid).abs() / mid.max(1.0));
                                    dist * (node_count as f32) + float_offset
                                }
                            };

                            let idx1 = (norm_pos as usize) % node_count;
                            let idx2 = (idx1 + 1) % node_count;
                            let frac = norm_pos.fract();

                            let n1 = &self.state.diy_nodes[idx1];
                            let raw_color = if is_gradient {
                                let n2 = &self.state.diy_nodes[idx2];
                                pyxillum_sim::render::lerp_color(n1.color, n2.color, frac)
                            } else {
                                n1.color
                            };

                            let weight_scale = (n1.value as f32 / 255.0).clamp(0.05, 1.0);
                            let final_scale = brightness_factor * weight_scale;
                            let scaled = pyxillum_sim::render::scale_color(raw_color, final_scale);
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                    crate::state::DiyPatternStyle::Breathe => {
                        let cycle = (t * 0.8) % node_count as f32;
                        let idx1 = (cycle as usize) % node_count;
                        let idx2 = (idx1 + 1) % node_count;
                        let frac = cycle.fract();

                        let n1 = &self.state.diy_nodes[idx1];
                        let n2 = &self.state.diy_nodes[idx2];
                        let base_color = if is_gradient {
                            pyxillum_sim::render::lerp_color(n1.color, n2.color, frac)
                        } else {
                            n1.color
                        };

                        let breath_env = (f32::sin(frac * std::f32::consts::PI)).powf(1.6);
                        let final_scale = brightness_factor * breath_env;
                        let scaled = pyxillum_sim::render::scale_color(base_color, final_scale);
                        for _ in 0..num_pixels {
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                    crate::state::DiyPatternStyle::Wave => {
                        let wave_speed = t * 3.0;
                        for i in 0..num_pixels {
                            let phase = match self.state.diy_direction {
                                crate::state::DiyDirection::Forward => {
                                    (i as f32 * 0.15) - wave_speed
                                }
                                crate::state::DiyDirection::Backward => {
                                    (i as f32 * 0.15) + wave_speed
                                }
                                crate::state::DiyDirection::MiddleToOut => {
                                    let mid = num_pixels as f32 * 0.5;
                                    (i as f32 - mid).abs() * 0.2 - wave_speed
                                }
                                crate::state::DiyDirection::OutToMiddle => {
                                    let mid = num_pixels as f32 * 0.5;
                                    (i as f32 - mid).abs() * 0.2 + wave_speed
                                }
                            };
                            let wave_amp = (f32::sin(phase) * 0.5 + 0.5).powf(1.4);

                            let col_pos = (i as f32 / num_pixels.max(1) as f32)
                                * (node_count as f32)
                                + (t * 0.5);
                            let idx1 = (col_pos as usize) % node_count;
                            let idx2 = (idx1 + 1) % node_count;
                            let frac = col_pos.fract();

                            let n1 = &self.state.diy_nodes[idx1];
                            let raw_color = if is_gradient {
                                let n2 = &self.state.diy_nodes[idx2];
                                pyxillum_sim::render::lerp_color(n1.color, n2.color, frac)
                            } else {
                                n1.color
                            };

                            let final_scale = brightness_factor * wave_amp;
                            let scaled = pyxillum_sim::render::scale_color(raw_color, final_scale);
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                    crate::state::DiyPatternStyle::Comet => {
                        let tail_len = 12.0_f32;
                        let head_pos = match self.state.diy_direction {
                            crate::state::DiyDirection::Backward => {
                                (num_pixels as f32 + tail_len)
                                    - ((t * 25.0) % (num_pixels as f32 + tail_len * 2.0))
                            }
                            _ => (t * 25.0) % (num_pixels as f32 + tail_len * 2.0) - tail_len,
                        };
                        let active_node_idx = ((t * 2.0) as usize) % node_count;
                        let comet_color = self.state.diy_nodes[active_node_idx].color;

                        for i in 0..num_pixels {
                            let dist = (head_pos - i as f32).abs();
                            let intensity = if dist < tail_len {
                                (1.0 - (dist / tail_len)).powf(2.0)
                            } else {
                                0.03
                            };

                            let scaled = pyxillum_sim::render::scale_color(
                                comet_color,
                                brightness_factor * intensity,
                            );
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                    crate::state::DiyPatternStyle::Strobe => {
                        let step = ((t * 5.0) as usize) % node_count;
                        let phase_fraction = (t * 5.0).fract();
                        let is_flash_on = phase_fraction < 0.6;

                        let current_col = if is_flash_on {
                            self.state.diy_nodes[step].color
                        } else {
                            pyxillum_protocol::RgbColor::BLACK
                        };
                        let scaled =
                            pyxillum_sim::render::scale_color(current_col, brightness_factor);
                        for _ in 0..num_pixels {
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                    crate::state::DiyPatternStyle::Sparkle => {
                        for i in 0..num_pixels {
                            let hash = ((i as u32 * 1103515245 + 12345 + (t * 8.0) as u32) % 100)
                                as f32
                                / 100.0;
                            let node_idx = (i + (t * 2.0) as usize) % node_count;
                            let base_node_col = self.state.diy_nodes[node_idx].color;

                            let sparkle_val = if hash > 0.82 {
                                ((hash - 0.82) / 0.18).powf(1.5)
                            } else {
                                0.08
                            };
                            let scaled = pyxillum_sim::render::scale_color(
                                base_node_col,
                                brightness_factor * sparkle_val,
                            );
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                    crate::state::DiyPatternStyle::Static => {
                        for i in 0..num_pixels {
                            let norm_pos =
                                (i as f32 / num_pixels.max(1) as f32) * (node_count as f32);
                            let idx1 = (norm_pos as usize).min(node_count - 1);
                            let idx2 = (idx1 + 1).min(node_count - 1);
                            let frac = norm_pos.fract();

                            let n1 = &self.state.diy_nodes[idx1];
                            let raw_color = if is_gradient && idx1 != idx2 {
                                let n2 = &self.state.diy_nodes[idx2];
                                pyxillum_sim::render::lerp_color(n1.color, n2.color, frac)
                            } else {
                                n1.color
                            };

                            let weight_scale = (n1.value as f32 / 255.0).clamp(0.05, 1.0);
                            let scaled = pyxillum_sim::render::scale_color(
                                raw_color,
                                brightness_factor * weight_scale,
                            );
                            self.state.strip_pixels.push(scaled);
                        }
                    }
                }
            }
            DashboardMode::Dynamic => {
                let speed_mult = self.state.ui_speed as f32 * 0.4;
                let t = self.state.animation_timer * speed_mult;
                self.state.strip_pixels = pyxillum_sim::render_effect_frame(
                    self.state.ui_effect,
                    t,
                    num_pixels,
                    brightness_factor,
                );
            }
            DashboardMode::Music => {
                let peak = if self.state.sync_with_windows_audio {
                    self.state.live_audio_peak
                } else {
                    (self.state.audio_visualizer_timer * 6.0).sin() * 0.4 + 0.6
                };

                let t = self.state.audio_visualizer_timer;

                match self.state.music_mode_index {
                    // Mode 5: Strip Music Mode 5 (Energy Gradient expanding symmetrically from center)
                    5 => {
                        let mid = num_pixels as f32 * 0.5;
                        let max_extent = (mid * peak).max(1.0);
                        for i in 0..num_pixels {
                            let dist = (i as f32 - mid).abs();
                            if dist <= max_extent && self.state.ui_power {
                                let ratio = dist / mid;
                                let hue = (180.0 + ratio * 160.0) % 360.0;
                                let col = pyxillum_sim::render::hsv_to_rgb(hue, 1.0, 1.0);
                                self.state
                                    .strip_pixels
                                    .push(pyxillum_sim::render::scale_color(
                                        col,
                                        brightness_factor,
                                    ));
                            } else {
                                self.state.strip_pixels.push(RgbColor::new(26, 26, 26));
                            }
                        }
                    }
                    // Modes 3, 4: Rhythm Stars
                    3 | 4 => {
                        for i in 0..num_pixels {
                            let sparkle = ((i as f32 * 17.3 + t * 8.0).sin() * 0.5 + 0.5).powi(4);
                            if sparkle > 0.3 && peak > 0.25 && self.state.ui_power {
                                let hue = if self.state.music_mode_index == 3 {
                                    (i as f32 * 45.0) % 360.0
                                } else {
                                    195.0
                                };
                                let col =
                                    pyxillum_sim::render::hsv_to_rgb(hue, 1.0, sparkle * peak);
                                self.state
                                    .strip_pixels
                                    .push(pyxillum_sim::render::scale_color(
                                        col,
                                        brightness_factor,
                                    ));
                            } else {
                                self.state.strip_pixels.push(RgbColor::new(26, 26, 26));
                            }
                        }
                    }
                    // Modes 7, 8: Pulse / Energy breathing with music volume
                    7 | 8 => {
                        let intensity = (peak * brightness_factor).clamp(0.0, 1.0);
                        let hue = if self.state.music_mode_index == 7 {
                            (t * 40.0) % 360.0
                        } else {
                            210.0
                        };
                        let col = pyxillum_sim::render::hsv_to_rgb(hue, 1.0, 1.0);
                        let scaled = pyxillum_sim::render::scale_color(col, intensity);
                        for _ in 0..num_pixels {
                            self.state.strip_pixels.push(if self.state.ui_power {
                                scaled
                            } else {
                                RgbColor::new(26, 26, 26)
                            });
                        }
                    }
                    // Default / Modes 1, 2, 6, 9..18: Rhythm VU Spectrum
                    _ => {
                        let active_count = ((num_pixels as f32 * peak) as usize).min(num_pixels);
                        for i in 0..num_pixels {
                            if i < active_count && self.state.ui_power {
                                let ratio = i as f32 / num_pixels as f32;
                                let hue = (120.0 - ratio * 120.0).max(0.0); // Green to Red VU
                                let col = pyxillum_sim::render::hsv_to_rgb(hue, 1.0, 1.0);
                                self.state
                                    .strip_pixels
                                    .push(pyxillum_sim::render::scale_color(
                                        col,
                                        brightness_factor,
                                    ));
                            } else {
                                self.state.strip_pixels.push(RgbColor::new(26, 26, 26));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl eframe::App for PyxillumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::ui::layout::clear_layout_debug_records();

        // Show window on first frame to prevent startup flicker and apply native Windows 11 DWM appearance
        if !self.has_shown_window {
            self.has_shown_window = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            crate::platform::windows::apply_windows_dwm_appearance(
                "Pyxillum - Pixel LED Studio",
                self.tokens.surfaces.canvas,
            );
            crate::platform::windows::set_window_outer_rect(
                "Pyxillum - Pixel LED Studio",
                self.target_rect,
                self.scale,
            );
        }

        // Layout Debugger Hotkey (Ctrl+Shift+L)
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::L)) {
            self.state.layout_debug = crate::ui::layout::toggle_layout_debug();
        }

        // Global Keyboard Shortcuts
        ctx.input(|i| {
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::Num1) {
                    self.state.current_screen =
                        AppScreen::Dashboard(self.state.active_device_id.clone());
                    self.state.dashboard_mode = DashboardMode::Dynamic;
                } else if i.key_pressed(egui::Key::Num2) {
                    self.state.current_screen =
                        AppScreen::Dashboard(self.state.active_device_id.clone());
                    self.state.dashboard_mode = DashboardMode::Solid;
                } else if i.key_pressed(egui::Key::Num3) {
                    self.state.current_screen =
                        AppScreen::Dashboard(self.state.active_device_id.clone());
                    self.state.dashboard_mode = DashboardMode::Music;
                } else if i.key_pressed(egui::Key::Num4) {
                    self.state.current_screen =
                        AppScreen::Dashboard(self.state.active_device_id.clone());
                    self.state.dashboard_mode = DashboardMode::Diy;
                } else if i.key_pressed(egui::Key::Num5) {
                    self.state.current_screen = AppScreen::DeviceHub;
                } else if i.key_pressed(egui::Key::Comma) {
                    self.state.show_settings_modal = true;
                } else if i.key_pressed(egui::Key::F) {
                    self.state.show_effect_drawer = true;
                }
            } else if i.key_pressed(egui::Key::Escape) {
                if self.state.show_settings_modal {
                    self.state.show_settings_modal = false;
                } else if self.state.show_help_modal {
                    self.state.show_help_modal = false;
                } else if self.state.show_effect_drawer {
                    self.state.show_effect_drawer = false;
                } else if self.state.show_icon_sheet {
                    self.state.show_icon_sheet = false;
                }
            } else if i.key_pressed(egui::Key::F5)
                && self.state.current_screen == AppScreen::DeviceHub
                && !self.state.is_scanning
            {
                let _ = self.cmd_tx.try_send(GuiCommand::Scan);
            }
        });

        // Track valid window size, position, and maximised state for persistent session restoration
        ctx.input(|i| {
            let is_max = i.viewport().maximized.unwrap_or(false);
            let is_min = i.viewport().minimized.unwrap_or(false);
            self.is_maximized = is_max;
            if !is_max && !is_min {
                let r_opt = i.viewport().outer_rect.or(i.viewport().inner_rect);
                if let Some(r) = r_opt {
                    if r.width() >= 200.0 && r.height() >= 200.0 {
                        self.last_valid_rect = Some(crate::window_geom::Rect::new(
                            r.min.x as f64,
                            r.min.y as f64,
                            r.width() as f64,
                            r.height() as f64,
                        ));
                    }
                }
            }
        });

        let now = Instant::now();
        let dt = now
            .duration_since(self.last_frame_instant)
            .as_secs_f32()
            .min(0.1);
        self.last_frame_instant = now;

        // Sync connection state with background DeviceState reader
        if let Ok(hw_state) = self.state_reader.try_read() {
            if hw_state.is_connected && !self.state.is_connected {
                self.state.is_connected = true;
            }
        }

        // Drain background worker events (real BLE discoveries and connection updates)
        while let Ok(evt) = self.event_rx.try_recv() {
            match evt {
                GuiEvent::ScanStarted => {
                    self.state.is_scanning = true;
                    self.state.last_ble_diagnostic = None;
                    self.state.status_message =
                        "Scanning for nearby Bluetooth Low Energy controllers (2.5s)..."
                            .to_string();
                }
                GuiEvent::ScanFinished(devices) => {
                    self.state.is_scanning = false;
                    self.state.last_ble_diagnostic = None;
                    self.saved_devices.clear();
                    self.state.discovered_count = 0;
                    for dev in devices {
                        let model = if let Some(n) = &dev.name {
                            if n.contains("621") {
                                "SP621E"
                            } else if n.contains("530") {
                                "SP530E"
                            } else if n.contains("630") {
                                "SP630E"
                            } else if n.contains("801") {
                                "SP801E"
                            } else {
                                "SP-Family"
                            }
                        } else {
                            "SP-Family"
                        };
                        self.saved_devices.push(DeviceCardInfo {
                            id: dev.address.clone(),
                            name: dev
                                .name
                                .clone()
                                .unwrap_or_else(|| "BLE Controller".to_string()),
                            model: model.to_string(),
                            protocol_type: "BLE GATT (0xFFE0)".to_string(),
                            is_simulated: dev.is_simulated,
                            is_connected: self.state.is_connected
                                && self.state.active_device_id == dev.address,
                            rssi: dev.rssi.unwrap_or(-75) as i8,
                        });
                    }
                    self.state.discovered_count = self.saved_devices.len();
                    self.state.status_message = if self.saved_devices.is_empty() {
                        "No BLE devices discovered. Ensure controller is powered on.".to_string()
                    } else {
                        let count = self.saved_devices.len();
                        let word = if count == 1 {
                            "controller"
                        } else {
                            "controllers"
                        };
                        format!("Discovered {} {}. Ready to pair.", count, word)
                    };
                }
                GuiEvent::Connected { address, name } => {
                    self.state.is_connected = true;
                    self.state.last_ble_diagnostic = None;
                    self.state.active_device_id = address.clone();
                    self.state.current_screen = AppScreen::Dashboard(address.clone());
                    self.state.status_message = format!("Connected to {} ({})", name, address);
                    for dev in &mut self.saved_devices {
                        dev.is_connected = dev.id == address;
                    }
                    // Sync active state directly to controller
                    self.send(GuiCommand::SendCommand(Command::Power(self.state.ui_power)));
                    self.send(GuiCommand::SendCommand(Command::SetEffect(
                        self.state.ui_effect,
                    )));
                    self.send(GuiCommand::SendCommand(Command::SetSpeed(
                        self.state.ui_speed,
                    )));
                    self.send(GuiCommand::SendCommand(Command::SetBrightness(
                        self.state.ui_brightness,
                    )));
                    self.last_sent_effect = self.state.ui_effect;
                    self.last_sent_speed = self.state.ui_speed;
                    self.last_sent_brightness = self.state.ui_brightness;
                }
                GuiEvent::Disconnected => {
                    self.state.is_connected = false;
                    self.state.current_screen = AppScreen::DeviceHub;
                    self.state.status_message =
                        "Disconnected from Bluetooth controller.".to_string();
                    for dev in &mut self.saved_devices {
                        dev.is_connected = false;
                    }
                }
                GuiEvent::StatusUpdated(status) => {
                    self.state.ui_power = status.power;
                    self.state.ui_brightness = status.brightness;
                    self.state.ui_speed = status.speed;
                    self.state.ui_effect = status.effect;
                    self.state.ui_length = status.length;
                    self.state.ui_chip_order = status.chip_order;
                    self.state.ui_color = [
                        status.color.r as f32 / 255.0,
                        status.color.g as f32 / 255.0,
                        status.color.b as f32 / 255.0,
                    ];
                    self.last_sent_effect = status.effect;
                    self.last_sent_speed = status.speed;
                    self.last_sent_brightness = status.brightness;
                }
                GuiEvent::Error(err) => {
                    self.state.is_scanning = false;
                    let diag = pyxillum_transport::BleDiagnostic::from_error_str(&err);
                    self.state.status_message = diag.description();
                    self.state.last_ble_diagnostic = Some(diag);
                }
            }
        }

        // Poll live Windows audio metrics
        let (info, peak) = self.audio_service.get_state();
        self.state.live_audio_peak = peak;
        self.now_playing = info;

        // Update real-time LED strip frame
        self.update_strip_simulation(dt);

        // Stream real-time brightness modulation over BLE when in dynamic custom colour style
        if self.state.is_connected
            && self.state.dashboard_mode == DashboardMode::Solid
            && self.state.ui_power
            && self.state.custom_color_effect != CustomColorEffect::Solid
            && self.state.live_color_stream_enabled
            && now.duration_since(self.last_stream_instant) >= std::time::Duration::from_millis(45)
        {
            let speed_mult = self.state.custom_effect_speed as f32 * 0.4;
            let t = self.state.animation_timer * speed_mult;
            let max_lvl = self.state.ui_brightness as f32;

            let stream_lvl = match self.state.custom_color_effect {
                CustomColorEffect::Breathe => {
                    let wave = ((t * 2.5).sin() * 0.5 + 0.5).powf(1.8);
                    ((0.12 + 0.88 * wave) * max_lvl) as u8
                }
                CustomColorEffect::Strobe => {
                    let phase = (t * 6.0).fract();
                    if phase < 0.22 {
                        self.state.ui_brightness
                    } else {
                        (max_lvl * 0.04) as u8
                    }
                }
                CustomColorEffect::Pulse => {
                    let cycle = (t * 2.2).fract();
                    let p = if cycle < 0.15 {
                        (cycle / 0.15 * std::f32::consts::PI).sin()
                    } else if (0.24..0.39).contains(&cycle) {
                        ((cycle - 0.24) / 0.15 * std::f32::consts::PI).sin() * 0.7
                    } else {
                        0.05
                    };
                    ((0.08 + 0.92 * p) * max_lvl) as u8
                }
                _ => self.state.ui_brightness,
            };

            if stream_lvl != self.last_streamed_level {
                self.last_streamed_level = stream_lvl;
                self.send(GuiCommand::SendCommand(Command::SetBrightness(stream_lvl)));
            }
            self.last_stream_instant = now;
        }

        // 1. Responsive Left Navigation Rail (Desktop Sidebar)
        let total_window_w = ctx.screen_rect().width();
        let is_compact = total_window_w < 1100.0;
        let sidebar_w = if is_compact { 56.0 } else { 240.0 };

        egui::SidePanel::left("desktop_sidebar")
            .resizable(false)
            .exact_width(sidebar_w)
            .frame(
                egui::Frame::none()
                    .fill(self.tokens.surfaces.sidebar)
                    .stroke(Stroke::new(1.0_f32, self.tokens.borders.subtle))
                    .inner_margin(if is_compact {
                        egui::Margin::symmetric(4.0, 12.0)
                    } else {
                        egui::Margin::symmetric(14.0, 16.0)
                    }),
            )
            .show(ctx, |ui| {
                render_sidebar(ui, &mut self.state, is_compact);
            });

        // 2. Central Workspace Canvas with Wide (>1500px) Max-Width Container
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(self.tokens.surfaces.canvas)
                    .inner_margin(egui::Margin::symmetric(0.0, 16.0)),
            )
            .show(ctx, |ui| {
                let (content_w, side_margin, gutter) =
                    crate::ui::layout::PageScaffold::compute_geometry(ui.available_width(), is_compact);

                let win_h = ctx.screen_rect().height();
                let is_compact_height = win_h < 800.0;

                // Pinned Top Fixtures: Header, InfoBar, Neon Strip
                ui.horizontal(|ui| {
                    ui.add_space(side_margin);
                    ui.allocate_ui_with_layout(
                        Vec2::new(content_w, 0.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |top_ui| {
                            top_ui.set_max_width(content_w);
                            // Top Header Action Bar (40px)
                            self.render_top_header(top_ui, win_h);

                            top_ui.add_space(8.0);

                            // Disconnected / Preview Mode Warning InfoBar:
                            // One line, 40 px high, dismissible and remembered for the session, hidden if height < 800
                            let show_preview_bar = !self.state.is_connected
                                && !self.state.preview_infobar_dismissed
                                && !is_compact_height
                                && !matches!(self.state.current_screen, AppScreen::DeviceHub);

                            if show_preview_bar {
                                let bar_h = 40.0_f32;
                                let (bar_rect, _) = top_ui
                                    .allocate_exact_size(Vec2::new(content_w, bar_h), egui::Sense::hover());
                                let bp = top_ui.painter_at(bar_rect);
                                bp.rect_filled(
                                    bar_rect,
                                    self.tokens.radius.control_rounding(),
                                    self.tokens.surfaces.card,
                                );
                                bp.rect_stroke(
                                    bar_rect,
                                    self.tokens.radius.control_rounding(),
                                    Stroke::new(1.0_f32, self.tokens.borders.subtle),
                                );

                                top_ui.allocate_ui_at_rect(bar_rect, |bar_ui| {
                                    bar_ui.horizontal_centered(|bar_ui| {
                                        bar_ui.add_space(12.0);
                                        let (icon_rect, _) = bar_ui
                                            .allocate_exact_size(Vec2::splat(16.0), egui::Sense::hover());
                                        crate::ui::icons::Icon::Info.paint(
                                            bar_ui.painter(),
                                            icon_rect,
                                            self.tokens.accent.text_light,
                                        );

                                        bar_ui.add_space(8.0);
                                        bar_ui.label(
                                            egui::RichText::new(
                                                "Preview mode: changes affect the virtual strip only.",
                                            )
                                            .font(crate::ui::typography::TypeScale::caption())
                                            .color(self.tokens.text.primary),
                                        );

                                        bar_ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |right_ui| {
                                                right_ui.add_space(8.0);
                                                // Dismiss 'x' button
                                                let (x_rect, x_resp) = right_ui.allocate_exact_size(
                                                    Vec2::splat(24.0),
                                                    egui::Sense::click(),
                                                );
                                                let x_resp = x_resp
                                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                                    .on_hover_text("Dismiss preview notice for this session");
                                                if x_resp.hovered() {
                                                    right_ui.painter().rect_filled(
                                                        x_rect,
                                                        self.tokens.radius.control_rounding(),
                                                        self.tokens.surfaces.raised_hover,
                                                    );
                                                }
                                                crate::ui::icons::Icon::Close.paint(
                                                    right_ui.painter(),
                                                    Rect::from_center_size(
                                                        x_rect.center(),
                                                        Vec2::splat(12.0),
                                                    ),
                                                    self.tokens.text.secondary,
                                                );
                                                if x_resp.clicked() {
                                                    self.state.preview_infobar_dismissed = true;
                                                }

                                                right_ui.add_space(8.0);
                                                if crate::ui::components::button::FluentButton::subtle(
                                                    "Scan & Connect",
                                                )
                                                .show(right_ui)
                                                .clicked()
                                                {
                                                    self.state.current_screen = AppScreen::DeviceHub;
                                                }
                                            },
                                        );
                                    });
                                });

                                top_ui.add_space(8.0);
                            }

                            // Pinned Adaptive Virtual Pixel Hardware Monitor (56px normal, 40px compact)
                            render_neon_strip(
                                top_ui,
                                &self.state.strip_pixels,
                                self.state.ui_power,
                                is_compact_height,
                            );
                        },
                    );
                    ui.add_space(gutter);
                });

                ui.add_space(16.0);

                // Main Stage inside ScrollArea
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.add_space(side_margin);

                            ui.allocate_ui_with_layout(
                                Vec2::new(content_w, ui.available_height()),
                                egui::Layout::top_down(egui::Align::LEFT),
                                |stage_ui| {
                                    stage_ui.set_max_width(content_w);
                                    match &self.state.current_screen {
                                        AppScreen::DeviceHub => {
                                            if let Some(action) =
                                                render_device_hub(stage_ui, &mut self.state, &self.saved_devices)
                                            {
                                                match action {
                                                    DeviceHubAction::Scan => {
                                                        self.send(GuiCommand::Scan);
                                                    }
                                                    DeviceHubAction::CancelScan => {
                                                        self.state.is_scanning = false;
                                                    }
                                                    DeviceHubAction::Connect(addr) => {
                                                        self.send(GuiCommand::Connect(addr));
                                                    }
                                                    DeviceHubAction::Disconnect => {
                                                        self.send(GuiCommand::Disconnect);
                                                    }
                                                    DeviceHubAction::OpenSystemTarget(target) => {
                                                        #[cfg(windows)]
                                                        {
                                                            let _ = std::process::Command::new("cmd")
                                                                .args(["/C", "start", "", &target])
                                                                .spawn();
                                                        }
                                                        #[cfg(not(windows))]
                                                        {
                                                            let _ = target;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        AppScreen::Dashboard(_) => {
                                            let show_split_drawer = !is_compact
                                                && content_w >= 850.0
                                                && self.state.dashboard_mode == DashboardMode::Dynamic
                                                && self.state.show_effect_drawer;

                                            if show_split_drawer {
                                                // Two-Column Desktop Layout: Center Controls + Right Preset Drawer
                                                let mut selected_effect_code = None;
                                                crate::ui::layout::split_2(
                                                    stage_ui,
                                                    0.55,
                                                    420.0,
                                                    380.0,
                                                    16.0,
                                                    |ui, side| match side {
                                                        crate::ui::layout::SplitSide::Left => {
                                                            self.render_active_screen_content(ui);
                                                        }
                                                        crate::ui::layout::SplitSide::Right => {
                                                            crate::ui::components::card::Card::base().show(ui, |ui| {
                                                                ui.set_min_height(490.0);
                                                                crate::ui::components::card::section_header(
                                                                    ui,
                                                                    "Preset Styles Catalog (146)",
                                                                    Some("Browse, search and filter all 146 hardware dynamic styles"),
                                                                    None::<fn(&mut egui::Ui)>,
                                                                );
                                                                ui.add_space(8.0);
                                                                if let Some(code) =
                                                                    render_effect_drawer(ui, &mut self.state)
                                                                {
                                                                    selected_effect_code = Some(code);
                                                                }
                                                            });
                                                        }
                                                    },
                                                );

                                                if let Some(code) = selected_effect_code {
                                                    self.state.ui_effect = code;
                                                    self.last_sent_effect = code;
                                                    self.send(GuiCommand::SendCommand(
                                                        Command::SetEffect(code),
                                                    ));
                                                }
                                            } else {
                                                // Centered Single Stage
                                                self.render_active_screen_content(stage_ui);
                                            }
                                        }
                                    }
                                },
                            );

                            ui.add_space(gutter);
                        });
                    });
            });

        // 3. Modals and Drawers
        if self.state.show_settings_modal && render_settings_modal(ctx, &mut self.state) {
            // Apply EEPROM parameters
            self.send(GuiCommand::SendCommand(Command::SetLength(
                self.state.ui_length,
            )));
            self.send(GuiCommand::SendCommand(Command::SetChipOrder(
                self.state.ui_chip_order,
            )));
        }

        if self.state.show_help_modal {
            self.render_help_modal(ctx);
        }

        // Floating Effect Drawer Modal (when not in wide two-column mode)
        let avail_w = ctx.screen_rect().width();
        if self.state.show_effect_drawer && avail_w < 1100.0 {
            egui::Window::new("146 Lighting Styles Preset Catalog")
                .collapsible(false)
                .resizable(true)
                .default_size(Vec2::new(380.0, 520.0))
                .anchor(egui::Align2::RIGHT_TOP, Vec2::new(-20.0, 60.0))
                .show(ctx, |ui| {
                    if let Some(code) = render_effect_drawer(ui, &mut self.state) {
                        self.state.ui_effect = code;
                        self.last_sent_effect = code;
                        self.send(GuiCommand::SendCommand(Command::SetEffect(code)));
                    }
                });
        }

        // Debug Icon Sheet (triggered via --icon-sheet or Ctrl+Shift+I)
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::I)) {
            self.state.show_icon_sheet = !self.state.show_icon_sheet;
        }

        if self.state.show_icon_sheet {
            egui::Window::new("Fluent Icons Debug Sheet")
                .open(&mut self.state.show_icon_sheet)
                .default_size([540.0, 620.0])
                .show(ctx, |ui| {
                    crate::ui::icons::render_debug_icon_sheet(ui);
                });
        }

        // Intelligent repaint pacing: 60 FPS continuous repaints ONLY when active visualizers run;
        // otherwise sleep or wake on user input to maintain near-zero idle CPU.
        let needs_continuous_repaint = self.state.is_scanning
            || (self.state.ui_power
                && matches!(self.state.current_screen, AppScreen::Dashboard(_))
                && (self.state.is_playing_dynamic
                    || self.state.dashboard_mode == DashboardMode::Music
                    || (self.state.dashboard_mode == DashboardMode::Diy
                        && self.state.diy_pattern != crate::state::DiyPatternStyle::Static)));

        if needs_continuous_repaint {
            ctx.request_repaint();
        } else {
            // Idle pacing: 10 Hz fallback for background events (BLE notifications / media updates)
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        // Layout Debug Overlays (draws red bounding box over any widget exceeding parent or clip rect)
        crate::ui::layout::render_layout_debug_overlays(ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let win_placement = crate::platform::windows::get_window_normal_placement(
            "Pyxillum - Pixel LED Studio",
            self.scale,
        );
        let final_rect = if let Some((normal_rect, is_max)) = win_placement {
            self.is_maximized = is_max;
            Some(normal_rect)
        } else {
            self.last_valid_rect
        };

        if let Some(rect) = final_rect {
            crate::window_geom::save_window_state(&crate::window_geom::SavedState {
                rect,
                is_maximized: self.is_maximized,
                monitor_id: self.monitor_id.clone(),
            });
        }
    }
}

impl PyxillumApp {
    fn render_top_header(&mut self, ui: &mut egui::Ui, win_h: f32) {
        let tokens = crate::ui::theme::ThemeTokens::default_dark();
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 40.0), egui::Sense::hover());
        let p = ui.painter_at(rect);
        p.rect_filled(rect, tokens.radius.control_rounding(), tokens.surfaces.card);
        p.rect_stroke(
            rect,
            tokens.radius.control_rounding(),
            Stroke::new(1.0_f32, tokens.borders.subtle),
        );

        // Specular top highlight
        p.line_segment(
            [
                Pos2::new(rect.min.x + 8.0, rect.min.y + 1.0),
                Pos2::new(rect.max.x - 8.0, rect.min.y + 1.0),
            ],
            Stroke::new(1.0_f32, Color32::from_white_alpha(15)),
        );

        // Left Section: Breadcrumb & Title
        let left_w = 460.0;
        let left_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 10.0, rect.min.y),
            Vec2::new(left_w, rect.height()),
        );
        ui.allocate_ui_at_rect(left_rect, |left_ui| {
            left_ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center),
                |left_ui| {
                    if let AppScreen::Dashboard(_) = &self.state.current_screen {
                        let (bc_rect, bc_resp) = left_ui
                            .allocate_exact_size(Vec2::new(88.0, 28.0), egui::Sense::click());
                        let bc_resp = bc_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                        let bcp = left_ui.painter_at(bc_rect);
                        let bc_hovered = bc_resp.hovered();
                        let bc_bg = if bc_hovered {
                            tokens.surfaces.raised_hover
                        } else {
                            tokens.surfaces.well
                        };
                        bcp.rect_filled(bc_rect, tokens.radius.control_rounding(), bc_bg);
                        bcp.rect_stroke(
                            bc_rect,
                            tokens.radius.control_rounding(),
                            Stroke::new(
                                1.0_f32,
                                if bc_hovered {
                                    tokens.borders.hover
                                } else {
                                    tokens.borders.subtle
                                },
                            ),
                        );
                        let bc_col = if bc_hovered {
                            tokens.text.primary
                        } else {
                            tokens.text.secondary
                        };
                        crate::ui::icons::Icon::ChevronLeft.paint(
                            &bcp,
                            Rect::from_center_size(
                                Pos2::new(bc_rect.min.x + 14.0, bc_rect.center().y),
                                Vec2::splat(14.0),
                            ),
                            bc_col,
                        );
                        bcp.text(
                            Pos2::new(bc_rect.min.x + 26.0, bc_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "Devices",
                            crate::ui::typography::TypeScale::caption(),
                            bc_col,
                        );
                        if bc_resp.clicked() {
                            self.state.current_screen = AppScreen::DeviceHub;
                        }

                        left_ui.add_space(10.0);

                        // Header title is the page title only
                        let page_title = match self.state.dashboard_mode {
                            DashboardMode::Dynamic => "Dynamic FX",
                            DashboardMode::Solid => "Solid Colour",
                            DashboardMode::Music => "Music Rhythm",
                            DashboardMode::Diy => "DIY Custom",
                        };

                        left_ui.label(
                            egui::RichText::new(page_title)
                                .font(crate::ui::typography::TypeScale::body_strong())
                                .color(tokens.text.primary),
                        );

                        // Device name appears ONLY when truly connected
                        if self.state.is_connected && !self.state.active_device_id.is_empty() {
                            let short_id = if self.state.active_device_id.len() >= 8 {
                                &self.state.active_device_id[..8]
                            } else {
                                &self.state.active_device_id
                            };
                            left_ui.add_space(8.0);
                            left_ui.label(
                                egui::RichText::new(format!("• SP621E ({})", short_id))
                                    .font(crate::ui::typography::TypeScale::caption())
                                    .color(tokens.semantic.success),
                            );
                        } else if self.state.preview_infobar_dismissed || win_h < 800.0 {
                            // Subtle "Preview mode" badge when dismissed or height < 800
                            left_ui.add_space(8.0);
                            let (badge_rect, _) = left_ui
                                .allocate_exact_size(Vec2::new(88.0, 22.0), egui::Sense::hover());
                            let bp = left_ui.painter_at(badge_rect);
                            bp.rect_filled(
                                badge_rect,
                                tokens.radius.chip_rounding(),
                                tokens.surfaces.well,
                            );
                            bp.rect_stroke(
                                badge_rect,
                                tokens.radius.chip_rounding(),
                                Stroke::new(1.0_f32, tokens.borders.subtle),
                            );
                            bp.text(
                                badge_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "Preview mode",
                                crate::ui::typography::TypeScale::caption(),
                                tokens.text.tertiary,
                            );
                        }
                    } else {
                        left_ui.label(
                            egui::RichText::new("Devices & Pairing")
                                .font(crate::ui::typography::TypeScale::body_strong())
                                .color(tokens.text.primary),
                        );
                        left_ui.add_space(8.0);
                        left_ui.label(
                            egui::RichText::new("Bluetooth Low Energy Controller Hub")
                                .font(crate::ui::typography::TypeScale::caption())
                                .color(tokens.text.tertiary),
                        );
                    }
                },
            );
        });

        // Right Section: Power ToggleSwitch & Settings Button
        let right_rect = Rect::from_min_max(
            Pos2::new(rect.max.x - 240.0, rect.min.y),
            Pos2::new(rect.max.x - 10.0, rect.max.y),
        );
        ui.allocate_ui_at_rect(right_rect, |right_ui| {
            right_ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |right_ui| {
                    // Master Power ToggleSwitch
                    let mut power_val = self.state.ui_power;
                    if render_toggle_switch(right_ui, &mut power_val).clicked() {
                        self.state.ui_power = power_val;
                        self.send(GuiCommand::SendCommand(Command::Power(power_val)));
                    }

                    right_ui.add_space(8.0);
                    let power_lbl = if self.state.ui_power { "On" } else { "Off" };
                    right_ui.label(
                        egui::RichText::new(power_lbl)
                            .font(crate::ui::typography::TypeScale::caption())
                            .color(if self.state.ui_power {
                                tokens.text.primary
                            } else {
                                tokens.text.disabled
                            }),
                    );

                    right_ui.add_space(14.0);

                    // Settings Icon Button with Vector Gear
                    let (st_rect, st_resp) =
                        right_ui.allocate_exact_size(Vec2::new(94.0, 30.0), egui::Sense::click());
                    let st_resp = st_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                    let stp = right_ui.painter_at(st_rect);
                    let st_hovered = st_resp.hovered();
                    let st_bg = if st_hovered {
                        tokens.surfaces.raised_hover
                    } else {
                        tokens.surfaces.well
                    };
                    stp.rect_filled(st_rect, tokens.radius.control_rounding(), st_bg);
                    stp.rect_stroke(
                        st_rect,
                        tokens.radius.control_rounding(),
                        Stroke::new(
                            1.0_f32,
                            if st_hovered {
                                tokens.borders.hover
                            } else {
                                tokens.borders.subtle
                            },
                        ),
                    );
                    let st_col = if st_hovered {
                        tokens.text.primary
                    } else {
                        tokens.text.secondary
                    };
                    crate::ui::icons::Icon::Settings.paint(
                        &stp,
                        Rect::from_center_size(
                            Pos2::new(st_rect.min.x + 16.0, st_rect.center().y),
                            Vec2::splat(15.0),
                        ),
                        st_col,
                    );
                    stp.text(
                        Pos2::new(st_rect.min.x + 28.0, st_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        "Settings",
                        crate::ui::typography::TypeScale::caption(),
                        st_col,
                    );
                    if st_resp.clicked() {
                        self.state.show_settings_modal = true;
                    }
                },
            );
        });
    }

    fn render_active_screen_content(&mut self, ui: &mut egui::Ui) {
        match &self.state.current_screen {
            AppScreen::DeviceHub => {
                // Handled in main stage ScrollArea
            }
            AppScreen::Dashboard(_) => match self.state.dashboard_mode {
                DashboardMode::Dynamic => {
                    if render_dynamic_mode(ui, &mut self.state) {
                        if self.state.ui_effect != self.last_sent_effect {
                            self.last_sent_effect = self.state.ui_effect;
                            self.send(GuiCommand::SendCommand(Command::SetEffect(
                                self.state.ui_effect,
                            )));
                        }
                        if self.state.ui_speed != self.last_sent_speed {
                            self.last_sent_speed = self.state.ui_speed;
                            self.send(GuiCommand::SendCommand(Command::SetSpeed(
                                self.state.ui_speed,
                            )));
                        }
                        if self.state.ui_brightness != self.last_sent_brightness {
                            self.last_sent_brightness = self.state.ui_brightness;
                            self.send(GuiCommand::SendCommand(Command::SetBrightness(
                                self.state.ui_brightness,
                            )));
                        }
                        if self.state.is_loop_all != self.last_sent_loop_all {
                            self.last_sent_loop_all = self.state.is_loop_all;
                            let light_mode = if self.state.is_loop_all {
                                LightMode::AutoDynamic
                            } else {
                                LightMode::Single
                            };
                            self.send(GuiCommand::SendCommand(Command::SetLightMode(light_mode)));
                        }
                    }
                }
                DashboardMode::Solid => {
                    if render_solid_mode(ui, &mut self.state) {
                        let r = (self.state.ui_color[0] * 255.0) as u8;
                        let g = (self.state.ui_color[1] * 255.0) as u8;
                        let b = (self.state.ui_color[2] * 255.0) as u8;

                        if self.state.custom_color_effect == CustomColorEffect::Solid {
                            if [r, g, b] != self.last_sent_color
                                || self.state.ui_brightness != self.last_sent_brightness
                                || self.last_sent_custom_effect != CustomColorEffect::Solid
                            {
                                self.last_sent_color = [r, g, b];
                                self.last_sent_brightness = self.state.ui_brightness;
                                self.last_sent_custom_effect = CustomColorEffect::Solid;
                                self.send(GuiCommand::SendCommand(Command::SetEffect(
                                    pyxillum_protocol::EFFECT_SOLID,
                                )));
                                self.send(GuiCommand::SendCommand(Command::SetColor {
                                    color: RgbColor::new(r, g, b),
                                    level: self.state.ui_brightness,
                                }));
                            }
                        } else {
                            if [r, g, b] != self.last_sent_color
                                || self.state.custom_color_effect != self.last_sent_custom_effect
                            {
                                self.last_sent_color = [r, g, b];
                                self.last_sent_custom_effect = self.state.custom_color_effect;
                                self.send(GuiCommand::SendCommand(Command::SetColor {
                                    color: RgbColor::new(r, g, b),
                                    level: self.state.ui_brightness,
                                }));
                                self.send(GuiCommand::SendCommand(Command::SetSpeed(
                                    self.state.custom_effect_speed,
                                )));
                            }
                        }
                    }
                }
                DashboardMode::Music => {
                    if render_music_mode(ui, &mut self.state, &self.now_playing) {
                        if self.state.music_mode_index != self.last_sent_music_mode {
                            self.last_sent_music_mode = self.state.music_mode_index;
                            let eff_code = pyxillum_protocol::EFFECT_SOUND_BASE
                                + self.state.music_mode_index
                                - 1;
                            self.send(GuiCommand::SendCommand(Command::SetEffect(eff_code)));
                            self.send(GuiCommand::SendCommand(Command::SetLightMode(
                                LightMode::AutoSound,
                            )));
                        }
                        if self.state.ui_audio_input != self.last_sent_audio_input {
                            self.last_sent_audio_input = self.state.ui_audio_input;
                            self.send(GuiCommand::SendCommand(Command::SetAudioInput(
                                self.state.ui_audio_input,
                            )));
                        }
                        if self.state.ui_sensitivity != self.last_sent_sensitivity {
                            self.last_sent_sensitivity = self.state.ui_sensitivity;
                            self.send(GuiCommand::SendCommand(Command::SetSensitivity(
                                self.state.ui_sensitivity,
                            )));
                        }
                        if self.state.ui_brightness != self.last_sent_brightness {
                            self.last_sent_brightness = self.state.ui_brightness;
                            self.send(GuiCommand::SendCommand(Command::SetBrightness(
                                self.state.ui_brightness,
                            )));
                        }
                    }
                }
                DashboardMode::Diy => {
                    if render_diy_mode(ui, &mut self.state) {
                        if self.state.diy_speed != self.last_sent_speed {
                            self.last_sent_speed = self.state.diy_speed;
                            self.send(GuiCommand::SendCommand(Command::SetSpeed(
                                self.state.diy_speed,
                            )));
                        }
                        if self.state.diy_brightness != self.last_sent_brightness {
                            self.last_sent_brightness = self.state.diy_brightness;
                            self.send(GuiCommand::SendCommand(Command::SetBrightness(
                                self.state.diy_brightness,
                            )));
                        }
                    }
                }
            },
        }
    }

    fn render_help_modal(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect();
        let scrim_painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Middle,
            egui::Id::new("help_scrim"),
        ));
        scrim_painter.rect_filled(screen_rect, Rounding::ZERO, Color32::from_black_alpha(150));

        egui::Window::new("Pyxillum Help Window")
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(Vec2::new(490.0, 360.0))
            .frame(
                egui::Frame::none()
                    .fill(Theme::CARD_BG)
                    .rounding(Rounding::same(12.0))
                    .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                    .inner_margin(egui::Margin::symmetric(22.0, 18.0)),
            )
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());

                // Header Row
                ui.horizontal(|ui| {
                    let (badge_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
                    let bp = ui.painter_at(badge_rect);
                    bp.rect_filled(badge_rect, Rounding::same(6.0), Theme::ACCENT_CYAN);
                    bp.text(
                        badge_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "i",
                        egui::FontId::proportional(15.0),
                        Color32::from_rgb(20, 20, 20),
                    );

                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("About Pyxillum")
                                .strong()
                                .size(15.5)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.label(
                            egui::RichText::new("Pixel LED Studio & Hardware Architecture Guide")
                                .size(11.0)
                                .color(Theme::TEXT_MUTED),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if crate::ui::icons::icon_button_32(
                            ui,
                            crate::ui::icons::Icon::Close,
                            "Close (Esc)",
                            false,
                        )
                        .clicked()
                        {
                            self.state.show_help_modal = false;
                        }
                    });
                });

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(12.0);

                ui.label(
                    egui::RichText::new(
                        "Pyxillum is a native, offline-first Windows controller for SP621E/SP6xxE Bluetooth Low Energy pixel LED devices.",
                    )
                    .size(12.0)
                    .color(Theme::TEXT_SECONDARY),
                );
                ui.add_space(12.0);

                egui::Frame::none()
                    .fill(Theme::WELL_BG)
                    .rounding(Rounding::same(8.0))
                    .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(
                            egui::RichText::new("Core Architecture Tenets")
                                .strong()
                                .size(12.0)
                                .color(Theme::ACCENT_CYAN),
                        );
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("• Write Pacing: 30 ms automatic delay between outbound GATT frames to prevent buffer overflow.").size(11.0).color(Theme::TEXT_SECONDARY));
                        ui.label(egui::RichText::new("• Clean-Room Interoperability: Pure functional Rust codec adhering to statutory exemptions.").size(11.0).color(Theme::TEXT_SECONDARY));
                        ui.label(egui::RichText::new("• Hardware Safety: Non-volatile EEPROM updates are gated behind explicit confirmation.").size(11.0).color(Theme::TEXT_SECONDARY));
                        ui.label(egui::RichText::new("• Zero Telemetry: Completely local operation with no background network calls.").size(11.0).color(Theme::TEXT_SECONDARY));
                    });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if render_secondary_button(ui, "Close", 90.0).clicked() {
                        self.state.show_help_modal = false;
                    }
                });
            });
    }
}

impl Drop for PyxillumApp {
    fn drop(&mut self) {
        let win_placement = crate::platform::windows::get_window_normal_placement(
            "Pyxillum - Pixel LED Studio",
            self.scale,
        );
        let final_rect = if let Some((normal_rect, is_max)) = win_placement {
            self.is_maximized = is_max;
            Some(normal_rect)
        } else {
            self.last_valid_rect
        };

        if let Some(rect) = final_rect {
            crate::window_geom::save_window_state(&crate::window_geom::SavedState {
                rect,
                is_maximized: self.is_maximized,
                monitor_id: self.monitor_id.clone(),
            });
        }
    }
}
