//! # Pyxillum Engine Core Orchestrator
//!
//! Owns the background runtime, Bluetooth transport, device synchronization,
//! 60 FPS effects frame rendering, and coalesced state notifications.

use crate::action::Action;
use crate::frame::StripFrame;
use crate::listener::StateListener;
use crate::services::PlatformServices;
use crate::state::{AppState, ConnectionStatus, CustomColorEffect, ErrorKey, ScreenId, UserError};
use crate::{CoreError, DeviceManager, DeviceState, PresetStore, ScenePreset};
use pyxillum_protocol::{Command, LightMode, RgbColor, EFFECT_SOLID};
use pyxillum_sim::DeviceSimulator;
use pyxillum_transport::{BtleplugTransport, SimTransport, Transport};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex, RwLock};
use tracing::{info, warn};

/// High-level engine coordinating Pyxillum domain logic, transport, and state snapshots.
pub struct Engine {
    state: Arc<RwLock<AppState>>,
    manager: Arc<DeviceManager>,
    platform: Arc<dyn PlatformServices>,
    preset_store: Arc<PresetStore>,
    simulator: Option<Arc<Mutex<DeviceSimulator>>>,
    action_tx: mpsc::UnboundedSender<Action>,
    listeners: Arc<Mutex<Vec<Arc<dyn StateListener>>>>,
    state_changed: Arc<AtomicBool>,
    running: Arc<AtomicBool>,
    #[allow(dead_code)]
    runtime: Option<tokio::runtime::Runtime>,
    start_time: Instant,
}

impl Engine {
    /// Returns a thread-safe handle to the internal application state lock.
    pub fn state_handle(&self) -> Arc<RwLock<AppState>> {
        self.state.clone()
    }

    /// Returns a handle to the underlying device manager.
    pub fn manager(&self) -> Arc<DeviceManager> {
        self.manager.clone()
    }

    /// Returns a reference to the active platform services.
    pub fn platform(&self) -> &Arc<dyn PlatformServices> {
        &self.platform
    }

    /// Returns a handle to the scene preset storage.
    pub fn preset_store(&self) -> Arc<PresetStore> {
        self.preset_store.clone()
    }

    /// Returns a reference to the virtual simulator if running in simulation mode.
    pub fn simulator(&self) -> Option<Arc<Mutex<DeviceSimulator>>> {
        self.simulator.clone()
    }

    /// Manually flags that application state has changed and requests listener notification.
    pub fn notify_changed(&self) {
        self.state_changed.store(true, Ordering::SeqCst);
    }
    /// Creates a new Engine instance using the specified transport and platform services.
    /// Spawns a dedicated Tokio multi-thread runtime internally if not already within one.
    pub fn new(
        transport: Box<dyn Transport>,
        simulator: Option<Arc<Mutex<DeviceSimulator>>>,
        platform: Arc<dyn PlatformServices>,
    ) -> Result<Self, CoreError> {
        let (runtime, handle) = match tokio::runtime::Handle::try_current() {
            Ok(h) => (None, h),
            Err(_) => {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(2)
                    .enable_all()
                    .build()?;
                let handle = rt.handle().clone();
                (Some(rt), handle)
            }
        };

        let manager = Arc::new(DeviceManager::new(transport));
        let preset_store = Arc::new(PresetStore::with_dir(platform.data_dir()));
        let mut initial_state = AppState::default();

        // Load saved presets if available
        if let Ok(presets) = preset_store.load_presets() {
            initial_state.saved_presets = presets;
        }

        if simulator.is_some() {
            initial_state.preview_mode = true;
        }

        let state = Arc::new(RwLock::new(initial_state));
        let listeners: Arc<Mutex<Vec<Arc<dyn StateListener>>>> = Arc::new(Mutex::new(Vec::new()));
        let state_changed = Arc::new(AtomicBool::new(false));
        let running = Arc::new(AtomicBool::new(true));

        let (action_tx, mut action_rx) = mpsc::unbounded_channel::<Action>();

        // 1. Spawn Action Processor Task
        {
            let state_clone = state.clone();
            let manager_clone = manager.clone();
            let platform_clone = platform.clone();
            let preset_store_clone = preset_store.clone();
            let state_changed_clone = state_changed.clone();
            let running_clone = running.clone();

            handle.spawn(async move {
                while running_clone.load(Ordering::Relaxed) {
                    if let Some(action) = action_rx.recv().await {
                        Self::process_action(
                            action,
                            &state_clone,
                            &manager_clone,
                            &platform_clone,
                            &preset_store_clone,
                            &state_changed_clone,
                        )
                        .await;
                    } else {
                        break;
                    }
                }
            });
        }

        // 2. Spawn Notification Coalescing Broadcaster (~60 Hz / 16 ms)
        {
            let state_clone = state.clone();
            let listeners_clone = listeners.clone();
            let state_changed_clone = state_changed.clone();
            let running_clone = running.clone();

            handle.spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(16));
                while running_clone.load(Ordering::Relaxed) {
                    interval.tick().await;

                    if state_changed_clone.swap(false, Ordering::SeqCst) {
                        let snapshot = {
                            let s = state_clone.read().await;
                            s.clone()
                        };

                        let list = {
                            let l = listeners_clone.lock().await;
                            l.clone()
                        };

                        for listener in list {
                            listener.on_state_changed(&snapshot);
                        }
                    }
                }
            });
        }

        // 3. Spawn Hardware Status Sync Task
        {
            let state_clone = state.clone();
            let dev_state_handle = manager.state();
            let state_changed_clone = state_changed.clone();
            let running_clone = running.clone();

            handle.spawn(async move {
                let mut last_seen = DeviceState::default();
                while running_clone.load(Ordering::Relaxed) {
                    tokio::time::sleep(Duration::from_millis(30)).await;
                    let current = {
                        let guard = dev_state_handle.read().await;
                        guard.clone()
                    };

                    if current != last_seen {
                        last_seen = current.clone();
                        let mut s = state_clone.write().await;
                        s.device_state = current;
                        s.strip_config.chip_order = s.device_state.chip_order;
                        s.strip_config.length = s.device_state.length;
                        state_changed_clone.store(true, Ordering::SeqCst);
                    }
                }
            });
        }

        Ok(Self {
            state,
            manager,
            platform,
            preset_store,
            simulator,
            action_tx,
            listeners,
            state_changed,
            running,
            runtime,
            start_time: Instant::now(),
        })
    }

    /// Convenience constructor targeting the virtual in-memory simulator.
    pub fn new_simulator(platform: Arc<dyn PlatformServices>) -> Result<Self, CoreError> {
        let sim = Arc::new(Mutex::new(DeviceSimulator::new()));
        let transport = Box::new(SimTransport::new(sim.clone()));
        let engine = Self::new(transport, Some(sim), platform)?;
        engine.dispatch(Action::Connect {
            address: "SIM:00:11:22:33:44:55".to_string(),
        });
        Ok(engine)
    }

    /// Convenience constructor targeting real Bluetooth Low Energy hardware.
    pub fn new_ble(platform: Arc<dyn PlatformServices>) -> Result<Self, CoreError> {
        let transport = Box::new(BtleplugTransport::new());
        Self::new(transport, None, platform)
    }

    /// Dispatches a strongly typed user action into the engine's event queue.
    pub fn dispatch(&self, action: Action) {
        let _ = self.action_tx.send(action);
    }

    /// Registers a state listener to receive coalesced application state snapshots.
    pub fn subscribe(&self, listener: Arc<dyn StateListener>) {
        // Send initial state snapshot immediately to new listener
        {
            let state_handle = self.state.clone();
            let listener_clone = listener.clone();
            tokio::spawn(async move {
                let s = state_handle.read().await;
                listener_clone.on_state_changed(&s);
            });
        }

        let listeners_handle = self.listeners.clone();
        tokio::spawn(async move {
            let mut list = listeners_handle.lock().await;
            list.push(listener);
        });
    }

    /// Returns a synchronous clone snapshot of the current application state.
    pub fn snapshot(&self) -> AppState {
        // Safe fast read using blocking if inside runtime or direct lock
        if let Ok(guard) = self.state.try_read() {
            guard.clone()
        } else {
            // Fallback for heavy contention
            AppState::default()
        }
    }

    /// Computes the active 60 FPS pixel strip rendering frame.
    pub fn frame(&self) -> StripFrame {
        let state = self.snapshot();
        let pixel_count = state.strip_config.preview_pixel_count.max(1);

        if !state.device_state.power {
            return StripFrame::new_black(pixel_count);
        }

        let elapsed = self.start_time.elapsed().as_secs_f32();
        let speed_factor = state.device_state.speed.max(1) as f32;
        let brightness_factor = state.device_state.brightness as f32 / 255.0;

        match state.active_screen {
            ScreenId::Solid => {
                let base = state.solid_page.current_color;
                match state.solid_page.active_custom_effect {
                    CustomColorEffect::Solid => {
                        let c = RgbColor::new(
                            (base.r as f32 * brightness_factor) as u8,
                            (base.g as f32 * brightness_factor) as u8,
                            (base.b as f32 * brightness_factor) as u8,
                        );
                        StripFrame::new_solid(pixel_count, c)
                    }
                    CustomColorEffect::Breathe => {
                        let phase = (elapsed * speed_factor * 1.5).sin() * 0.5 + 0.5;
                        let b = brightness_factor * (0.15 + 0.85 * phase);
                        let c = RgbColor::new(
                            (base.r as f32 * b) as u8,
                            (base.g as f32 * b) as u8,
                            (base.b as f32 * b) as u8,
                        );
                        StripFrame::new_solid(pixel_count, c)
                    }
                    _ => {
                        let t = elapsed * speed_factor;
                        let buffer = pyxillum_sim::render::render_effect_frame(
                            state.device_state.effect,
                            t,
                            pixel_count,
                            brightness_factor,
                        );
                        StripFrame::new(buffer)
                    }
                }
            }
            ScreenId::Dynamic => {
                let t = elapsed * speed_factor;
                let buffer = pyxillum_sim::render::render_effect_frame(
                    state.dynamic_page.selected_effect,
                    t,
                    pixel_count,
                    brightness_factor,
                );
                StripFrame::new(buffer)
            }
            ScreenId::Music => {
                let bands = state.music_page.spectrum_bands;
                let mut pixels = Vec::with_capacity(pixel_count);
                for i in 0..pixel_count {
                    let band_idx = (i * 16 / pixel_count).min(15);
                    let energy = bands[band_idx];
                    let r = ((i as f32 / pixel_count as f32 * 255.0) * energy * brightness_factor)
                        as u8;
                    let g = (((1.0 - i as f32 / pixel_count as f32) * 200.0)
                        * energy
                        * brightness_factor) as u8;
                    let b = (255.0 * energy * brightness_factor) as u8;
                    pixels.push(RgbColor::new(r, g, b));
                }
                StripFrame::new(pixels)
            }
            ScreenId::Diy => {
                let steps = &state.diy_page.steps;
                if steps.is_empty() {
                    return StripFrame::new_black(pixel_count);
                }
                let mut pixels = Vec::with_capacity(pixel_count);
                let shift = if state.diy_page.is_playing {
                    (elapsed * speed_factor * 2.0) as usize
                } else {
                    0
                };
                for i in 0..pixel_count {
                    let node_idx = (i + shift) % steps.len();
                    let c = steps[node_idx];
                    pixels.push(RgbColor::new(
                        (c.r as f32 * brightness_factor) as u8,
                        (c.g as f32 * brightness_factor) as u8,
                        (c.b as f32 * brightness_factor) as u8,
                    ));
                }
                StripFrame::new(pixels)
            }
            _ => {
                // If running against simulator, delegate to simulator's renderer
                if let Some(sim_ref) = &self.simulator {
                    if let Ok(sim) = sim_ref.try_lock() {
                        let buffer = sim.render_strip();
                        return StripFrame::new(buffer);
                    }
                }

                let t = elapsed * speed_factor;
                let buffer = pyxillum_sim::render::render_effect_frame(
                    state.device_state.effect,
                    t,
                    pixel_count,
                    brightness_factor,
                );
                StripFrame::new(buffer)
            }
        }
    }

    /// Internal core action execution router.
    async fn process_action(
        action: Action,
        state: &Arc<RwLock<AppState>>,
        manager: &Arc<DeviceManager>,
        platform: &Arc<dyn PlatformServices>,
        preset_store: &Arc<PresetStore>,
        state_changed: &Arc<AtomicBool>,
    ) {
        match action {
            Action::StartScan => {
                {
                    let mut s = state.write().await;
                    s.connection_status = ConnectionStatus::Scanning;
                }
                state_changed.store(true, Ordering::SeqCst);

                match manager.scan().await {
                    Ok(devices) => {
                        let mut s = state.write().await;
                        s.discovered_devices = devices;
                        s.connection_status = ConnectionStatus::Disconnected;
                    }
                    Err(e) => {
                        let mut s = state.write().await;
                        s.user_error = Some(UserError::new(ErrorKey::ScanFailed, e.to_string()));
                        s.connection_status = ConnectionStatus::Error(e.to_string());
                    }
                }
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::StopScan => {
                let mut s = state.write().await;
                if s.connection_status == ConnectionStatus::Scanning {
                    s.connection_status = ConnectionStatus::Disconnected;
                    state_changed.store(true, Ordering::SeqCst);
                }
            }

            Action::Connect { address } => {
                {
                    let mut s = state.write().await;
                    s.connection_status = ConnectionStatus::Connecting {
                        address: address.clone(),
                    };
                }
                state_changed.store(true, Ordering::SeqCst);

                match manager.connect(&address).await {
                    Ok(_) => {
                        let mut s = state.write().await;
                        s.connection_status = ConnectionStatus::Connected {
                            address: address.clone(),
                            name: "SP621E Controller".to_string(),
                        };
                        s.user_error = None;
                    }
                    Err(e) => {
                        let mut s = state.write().await;
                        s.user_error =
                            Some(UserError::new(ErrorKey::ConnectionFailed, e.to_string()));
                        s.connection_status = ConnectionStatus::Error(e.to_string());
                    }
                }
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::Disconnect => {
                let _ = manager.disconnect().await;
                let mut s = state.write().await;
                s.connection_status = ConnectionStatus::Disconnected;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetPreviewMode(enabled) => {
                let mut s = state.write().await;
                s.preview_mode = enabled;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::TogglePreviewMode => {
                let mut s = state.write().await;
                s.preview_mode = !s.preview_mode;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::QueryStatus => {
                let _ = manager.send_command(Command::QueryState).await;
            }

            Action::OpenBluetoothSettings => {
                if let Err(e) = platform.open_bluetooth_settings() {
                    let mut s = state.write().await;
                    s.user_error = Some(UserError::new(
                        ErrorKey::BluetoothUnavailable,
                        e.to_string(),
                    ));
                    state_changed.store(true, Ordering::SeqCst);
                }
            }

            Action::SelectScreen(id) => {
                let mut s = state.write().await;
                s.active_screen = id;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetPower(on) => {
                let _ = manager.send_command(Command::Power(on)).await;
                let mut s = state.write().await;
                s.device_state.power = on;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::TogglePower => {
                let current_power = {
                    let s = state.read().await;
                    s.device_state.power
                };
                let new_power = !current_power;
                let _ = manager.send_command(Command::Power(new_power)).await;
                let mut s = state.write().await;
                s.device_state.power = new_power;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetBrightness(level) => {
                let _ = manager.send_command(Command::SetBrightness(level)).await;
                let mut s = state.write().await;
                s.device_state.brightness = level;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetSpeed(speed) => {
                let clamped = speed.clamp(1, 10);
                let _ = manager.send_command(Command::SetSpeed(clamped)).await;
                let mut s = state.write().await;
                s.device_state.speed = clamped;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetSolidColor(color) => {
                let level = { state.read().await.device_state.brightness };
                let _ = manager
                    .send_command(Command::SetColor { color, level })
                    .await;
                let mut s = state.write().await;
                s.device_state.color = color;
                s.device_state.mode = LightMode::Single;
                s.device_state.effect = EFFECT_SOLID;
                s.solid_page.current_color = color;
                s.solid_page.active_custom_effect = CustomColorEffect::Solid;

                // Push to recent colors history if distinct
                if !s.solid_page.recent_colors.contains(&color) {
                    s.solid_page.recent_colors.insert(0, color);
                    s.solid_page.recent_colors.truncate(8);
                }
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetCustomColorEffect(effect) => {
                let (color, speed, brightness) = {
                    let s = state.read().await;
                    (
                        s.solid_page.current_color,
                        s.device_state.speed,
                        s.device_state.brightness,
                    )
                };
                let _ = (speed, brightness);
                let (code, _) =
                    crate::catalog::find_closest_hardware_effect(color.r, color.g, color.b, effect);

                let _ = manager.send_command(Command::SetEffect(code)).await;
                let mut s = state.write().await;
                s.device_state.effect = code;
                s.solid_page.active_custom_effect = effect;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetColorTemperature(kelvin) => {
                let c = kelvin_to_rgb(kelvin);
                let level = { state.read().await.device_state.brightness };
                let _ = manager
                    .send_command(Command::SetColor { color: c, level })
                    .await;
                let mut s = state.write().await;
                s.solid_page.color_temperature_k = kelvin;
                s.solid_page.current_color = c;
                s.device_state.color = c;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetEffect(code) => {
                let clamped = code.clamp(1, 146);
                let _ = manager.send_command(Command::SetEffect(clamped)).await;
                let mut s = state.write().await;
                s.device_state.effect = clamped;
                s.dynamic_page.selected_effect = clamped;
                s.device_state.mode = LightMode::Single;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::NextEffect => {
                let current = {
                    let s = state.read().await;
                    s.dynamic_page.selected_effect
                };
                let next = if current >= 146 { 1 } else { current + 1 };
                let _ = manager.send_command(Command::SetEffect(next)).await;
                let mut s = state.write().await;
                s.device_state.effect = next;
                s.dynamic_page.selected_effect = next;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::PreviousEffect => {
                let current = {
                    let s = state.read().await;
                    s.dynamic_page.selected_effect
                };
                let prev = if current <= 1 { 146 } else { current - 1 };
                let _ = manager.send_command(Command::SetEffect(prev)).await;
                let mut s = state.write().await;
                s.device_state.effect = prev;
                s.dynamic_page.selected_effect = prev;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetEffectCategory(cat) => {
                let mut s = state.write().await;
                s.dynamic_page.selected_category = cat;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SearchEffects(query) => {
                let mut s = state.write().await;
                s.dynamic_page.search_query = query;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::ToggleLoopAll(loop_all) => {
                let mode = if loop_all {
                    LightMode::AutoDynamic
                } else {
                    LightMode::Single
                };
                let _ = manager.send_command(Command::SetLightMode(mode)).await;
                let mut s = state.write().await;
                s.dynamic_page.loop_all = loop_all;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetMusicMode(mode) => {
                let clamped = mode.clamp(1, 18);
                let eff_code = pyxillum_protocol::EFFECT_SOUND_BASE + clamped - 1;
                let _ = manager.send_command(Command::SetEffect(eff_code)).await;
                let _ = manager
                    .send_command(Command::SetLightMode(LightMode::AutoSound))
                    .await;
                let mut s = state.write().await;
                s.music_page.selected_music_mode = clamped;
                s.device_state.mode = LightMode::AutoSound;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetAudioInput(input) => {
                let _ = manager.send_command(Command::SetAudioInput(input)).await;
                let mut s = state.write().await;
                s.music_page.audio_input = input;
                s.device_state.audio_input = input;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetSensitivity(sens) => {
                let clamped = sens.clamp(1, 16);
                let _ = manager.send_command(Command::SetSensitivity(clamped)).await;
                let mut s = state.write().await;
                s.music_page.sensitivity = clamped;
                s.device_state.sensitivity = clamped;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetMicActive(active) => {
                let mut s = state.write().await;
                s.music_page.is_mic_active = active;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::UpdateSpectrumBands(bands) => {
                let mut s = state.write().await;
                s.music_page.spectrum_bands = bands;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetDiyStepColor { index, color } => {
                let mut s = state.write().await;
                if index < s.diy_page.steps.len() {
                    s.diy_page.steps[index] = color;
                    state_changed.store(true, Ordering::SeqCst);
                }
            }

            Action::SetDiySteps(steps) => {
                let mut s = state.write().await;
                s.diy_page.steps = steps;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetDiyPatternStyle(style) => {
                let mut s = state.write().await;
                s.diy_page.pattern_style = style;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetDiyEffectType(eff) => {
                let mut s = state.write().await;
                s.diy_page.effect_type = eff;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SetDiyActiveStep(idx) => {
                let mut s = state.write().await;
                s.diy_page.active_step_index = idx;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::ToggleDiyPlayback(playing) => {
                let mut s = state.write().await;
                s.diy_page.is_playing = playing;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::SaveDiyPreset { name } => {
                let steps = {
                    let s = state.read().await;
                    s.diy_page.steps.clone()
                };
                let preset = ScenePreset::new(&name, steps[0], 0x07, 5, 255);
                let _ = preset_store.save_preset(&preset);
                if let Ok(presets) = preset_store.load_presets() {
                    let mut s = state.write().await;
                    s.saved_presets = presets;
                    state_changed.store(true, Ordering::SeqCst);
                }
            }

            Action::LoadDiyPreset { name } => {
                if let Ok(preset) = preset_store.load_preset(&name) {
                    let mut s = state.write().await;
                    s.diy_page.steps = vec![preset.color];
                    state_changed.store(true, Ordering::SeqCst);
                }
            }

            Action::SavePreset { name } => {
                let current = {
                    let s = state.read().await;
                    (
                        s.device_state.color,
                        s.device_state.effect,
                        s.device_state.speed,
                        s.device_state.brightness,
                    )
                };
                let preset = ScenePreset::new(&name, current.0, current.1, current.2, current.3);
                if let Err(e) = preset_store.save_preset(&preset) {
                    let mut s = state.write().await;
                    s.user_error = Some(UserError::new(ErrorKey::PersistenceFailed, e.to_string()));
                } else if let Ok(presets) = preset_store.load_presets() {
                    let mut s = state.write().await;
                    s.saved_presets = presets;
                }
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::ApplyPreset { name } => {
                match preset_store.load_preset(&name) {
                    Ok(preset) => {
                        let _ = manager
                            .send_command(Command::SetColor {
                                color: preset.color,
                                level: preset.brightness,
                            })
                            .await;
                        let _ = manager
                            .send_command(Command::SetEffect(preset.effect))
                            .await;
                        let _ = manager.send_command(Command::SetSpeed(preset.speed)).await;
                        let _ = manager
                            .send_command(Command::SetBrightness(preset.brightness))
                            .await;

                        let mut s = state.write().await;
                        s.device_state.color = preset.color;
                        s.device_state.effect = preset.effect;
                        s.device_state.speed = preset.speed;
                        s.device_state.brightness = preset.brightness;
                    }
                    Err(_) => {
                        let mut s = state.write().await;
                        s.user_error = Some(UserError::from_key(ErrorKey::PresetNotFound));
                    }
                }
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::DeletePreset { name } => {
                let _ = preset_store.delete_preset(&name);
                if let Ok(presets) = preset_store.load_presets() {
                    let mut s = state.write().await;
                    s.saved_presets = presets;
                    state_changed.store(true, Ordering::SeqCst);
                }
            }

            // === CRITICAL SAFETY GATE ENFORCEMENT ===
            Action::ConfigureHardware {
                chip_order,
                length,
                confirmed,
            } => {
                if !confirmed {
                    warn!("REJECTED: Modifying non-volatile registers without safety confirmation flag.");
                    let mut s = state.write().await;
                    s.user_error = Some(UserError::from_key(
                        ErrorKey::SafetyGateRequiresConfirmation,
                    ));
                    state_changed.store(true, Ordering::SeqCst);
                    return;
                }

                info!(
                    "SAFETY GATE AUTHORISED: Writing non-volatile chip order {:?} and length {}.",
                    chip_order, length
                );

                let _ = manager
                    .send_command(Command::SetChipOrder(chip_order))
                    .await;
                let _ = manager.send_command(Command::SetLength(length)).await;

                let mut s = state.write().await;
                s.strip_config.chip_order = chip_order;
                s.strip_config.length = length;
                s.device_state.chip_order = chip_order;
                s.device_state.length = length;
                s.user_error = None;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::UpdateWindowState(data) => {
                let mut s = state.write().await;
                s.window_state = data;
                state_changed.store(true, Ordering::SeqCst);
            }

            Action::DismissError => {
                let mut s = state.write().await;
                s.user_error = None;
                state_changed.store(true, Ordering::SeqCst);
            }
        }
    }
}

/// Helper function converting Kelvin colour temperature to RGB.
#[allow(clippy::excessive_precision)]
fn kelvin_to_rgb(kelvin: u32) -> RgbColor {
    let temp = (kelvin as f32).clamp(1000.0, 40000.0) / 100.0;

    let red = if temp <= 66.0 {
        255.0
    } else {
        (temp - 60.0).powf(-0.1332047592) * 329.698727446
    };

    let green = if temp <= 66.0 {
        temp.ln() * 99.4708025861 - 161.1195681661
    } else {
        (temp - 60.0).powf(-0.0755148492) * 288.1221695283
    };

    let blue = if temp >= 66.0 {
        255.0
    } else if temp <= 19.0 {
        0.0
    } else {
        (temp - 10.0).ln() * 138.5177312231 - 305.0447927307
    };

    RgbColor::new(
        red.clamp(0.0, 255.0) as u8,
        green.clamp(0.0, 255.0) as u8,
        blue.clamp(0.0, 255.0) as u8,
    )
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
