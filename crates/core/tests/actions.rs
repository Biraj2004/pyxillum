//! # Comprehensive Action Test Suite
//!
//! Validates that every variant of `Action` can be dispatched to `Engine` against
//! the in-memory hardware simulator and produces the expected state mutations.

use pyxillum_core::{
    Action, AppState, CustomColorEffect, DefaultPlatformServices, DiyEffectType, DiyPatternStyle,
    EffectCategory, Engine, ScreenId, WindowStateData,
};
use pyxillum_protocol::{AudioInput, LightMode, RgbColor, EFFECT_SOLID};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

struct TestListener {
    call_count: Arc<AtomicUsize>,
}

impl pyxillum_core::StateListener for TestListener {
    fn on_state_changed(&self, _state: &AppState) {
        self.call_count.fetch_add(1, Ordering::SeqCst);
    }
}

fn create_test_engine() -> Engine {
    let platform = Arc::new(DefaultPlatformServices);
    Engine::new_simulator(platform).expect("Engine simulator failed to initialize")
}

#[tokio::test]
async fn test_power_actions() {
    let engine = create_test_engine();

    // 1. Initial power state is true
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(engine.snapshot().device_state.power);

    // 2. Turn off
    engine.dispatch(Action::SetPower(false));
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(!engine.snapshot().device_state.power);

    // 3. Toggle back on
    engine.dispatch(Action::TogglePower);
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(engine.snapshot().device_state.power);
}

#[tokio::test]
async fn test_brightness_and_speed_actions() {
    let engine = create_test_engine();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Brightness
    engine.dispatch(Action::SetBrightness(175));
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.snapshot().device_state.brightness, 175);

    // Speed clamping 1..=10
    engine.dispatch(Action::SetSpeed(8));
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.snapshot().device_state.speed, 8);

    engine.dispatch(Action::SetSpeed(99)); // clamps to 10
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.snapshot().device_state.speed, 10);
}

#[tokio::test]
async fn test_solid_color_and_custom_effects() {
    let engine = create_test_engine();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Set solid colour
    let target = RgbColor::new(0, 200, 255);
    engine.dispatch(Action::SetSolidColor(target));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let snap = engine.snapshot();
    assert_eq!(snap.device_state.color, target);
    assert_eq!(snap.device_state.effect, EFFECT_SOLID);
    assert_eq!(snap.device_state.mode, LightMode::Single);
    assert_eq!(snap.solid_page.current_color, target);
    assert!(snap.solid_page.recent_colors.contains(&target));

    // Custom colour effect (e.g. Breathe)
    engine.dispatch(Action::SetCustomColorEffect(CustomColorEffect::Breathe));
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(
        engine.snapshot().solid_page.active_custom_effect,
        CustomColorEffect::Breathe
    );

    // Colour temperature Kelvin
    engine.dispatch(Action::SetColorTemperature(3000));
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.snapshot().solid_page.color_temperature_k, 3000);
}

#[tokio::test]
async fn test_dynamic_effect_actions() {
    let engine = create_test_engine();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Set effect 42
    engine.dispatch(Action::SetEffect(42));
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.snapshot().device_state.effect, 42);
    assert_eq!(engine.snapshot().dynamic_page.selected_effect, 42);

    // Next effect
    engine.dispatch(Action::NextEffect);
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.snapshot().dynamic_page.selected_effect, 43);

    // Previous effect
    engine.dispatch(Action::PreviousEffect);
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.snapshot().dynamic_page.selected_effect, 42);

    // Category and search
    engine.dispatch(Action::SetEffectCategory(Some(EffectCategory::Rainbow)));
    engine.dispatch(Action::SearchEffects("meteor".to_string()));
    engine.dispatch(Action::ToggleLoopAll(true));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let snap = engine.snapshot();
    assert_eq!(
        snap.dynamic_page.selected_category,
        Some(EffectCategory::Rainbow)
    );
    assert_eq!(snap.dynamic_page.search_query, "meteor");
    assert!(snap.dynamic_page.loop_all);
}

#[tokio::test]
async fn test_music_mode_actions() {
    let engine = create_test_engine();
    tokio::time::sleep(Duration::from_millis(50)).await;

    engine.dispatch(Action::SetMusicMode(7));
    engine.dispatch(Action::SetAudioInput(AudioInput::ExternalMic));
    engine.dispatch(Action::SetSensitivity(14));
    engine.dispatch(Action::SetMicActive(true));
    let bands = [0.5; 16];
    engine.dispatch(Action::UpdateSpectrumBands(bands));
    tokio::time::sleep(Duration::from_millis(250)).await;

    let snap = engine.snapshot();
    assert_eq!(snap.music_page.selected_music_mode, 7);
    assert_eq!(snap.device_state.audio_input, AudioInput::ExternalMic);
    assert_eq!(snap.device_state.sensitivity, 14);
    assert!(snap.music_page.is_mic_active);
    assert_eq!(snap.music_page.spectrum_bands, bands);
}

#[tokio::test]
async fn test_diy_sequencer_actions() {
    let engine = create_test_engine();
    tokio::time::sleep(Duration::from_millis(50)).await;

    let new_steps = vec![
        RgbColor::new(255, 0, 0),
        RgbColor::new(0, 255, 0),
        RgbColor::new(0, 0, 255),
    ];
    engine.dispatch(Action::SetDiySteps(new_steps.clone()));
    engine.dispatch(Action::SetDiyPatternStyle(DiyPatternStyle::Flow));
    engine.dispatch(Action::SetDiyEffectType(DiyEffectType::Gradient));
    engine.dispatch(Action::SetDiyActiveStep(1));
    engine.dispatch(Action::ToggleDiyPlayback(false));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let snap = engine.snapshot();
    assert_eq!(snap.diy_page.steps, new_steps);
    assert_eq!(snap.diy_page.pattern_style, DiyPatternStyle::Flow);
    assert_eq!(snap.diy_page.effect_type, DiyEffectType::Gradient);
    assert_eq!(snap.diy_page.active_step_index, 1);
    assert!(!snap.diy_page.is_playing);

    // Update step color
    engine.dispatch(Action::SetDiyStepColor {
        index: 1,
        color: RgbColor::new(255, 255, 0),
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(
        engine.snapshot().diy_page.steps[1],
        RgbColor::new(255, 255, 0)
    );
}

#[tokio::test]
async fn test_navigation_and_window_state() {
    let engine = create_test_engine();
    tokio::time::sleep(Duration::from_millis(50)).await;

    engine.dispatch(Action::SelectScreen(ScreenId::Dynamic));
    let win = WindowStateData {
        x: 200.0,
        y: 150.0,
        width: 1280.0,
        height: 800.0,
        is_maximized: true,
    };
    engine.dispatch(Action::UpdateWindowState(win.clone()));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let snap = engine.snapshot();
    assert_eq!(snap.active_screen, ScreenId::Dynamic);
    assert_eq!(snap.window_state, win);
}

#[tokio::test]
async fn test_state_listener_subscription_and_coalescing() {
    let engine = create_test_engine();
    let call_count = Arc::new(AtomicUsize::new(0));

    let listener = Arc::new(TestListener {
        call_count: call_count.clone(),
    });
    engine.subscribe(listener);

    // Initial delivery
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(call_count.load(Ordering::SeqCst) >= 1);

    // Rapid-fire 10 actions
    let count_before = call_count.load(Ordering::SeqCst);
    for i in 1..=10 {
        engine.dispatch(Action::SetDiyActiveStep(i));
    }

    // Wait 100 ms (~6 frame periods)
    tokio::time::sleep(Duration::from_millis(100)).await;
    let count_after = call_count.load(Ordering::SeqCst);
    let notifications = count_after - count_before;

    // Coalescing guarantee: 10 rapid actions in zero time must coalesce to <= 6 notifications
    assert!(notifications <= 6);
    assert_eq!(engine.snapshot().diy_page.active_step_index, 10);
}

#[tokio::test]
async fn test_engine_audio_lifecycle() {
    let engine = create_test_engine();

    // Start loopback/simulated audio
    assert!(engine
        .start_audio(pyxillum_core::AudioSourceKind::SystemLoopback)
        .await
        .is_ok());

    // Spectrum query should return 24 bands
    let spectrum = engine.audio_spectrum().await;
    assert_eq!(spectrum.len(), 24);

    // Stop audio
    engine.stop_audio().await;
}
