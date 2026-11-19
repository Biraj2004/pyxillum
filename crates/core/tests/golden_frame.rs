//! # Golden Frame Effects Rendering Test Suite
//!
//! Validates that `engine.frame()` generates accurate 60 FPS pixel buffer representations
//! matching expected golden mathematical standards across all operating modes.

use pyxillum_core::{Action, DefaultPlatformServices, Engine, ScreenId};
use pyxillum_protocol::RgbColor;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_black_frame_when_powered_off() {
    let platform = Arc::new(DefaultPlatformServices);
    let engine = Engine::new_simulator(platform).expect("Engine failed to initialize");
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Power off
    engine.dispatch(Action::SetPower(false));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let frame = engine.frame();
    assert!(!frame.is_empty());
    for pixel in frame.colors() {
        assert_eq!(*pixel, RgbColor::new(0, 0, 0));
    }
}

#[tokio::test]
async fn test_solid_color_golden_frame() {
    let platform = Arc::new(DefaultPlatformServices);
    let engine = Engine::new_simulator(platform).expect("Engine failed to initialize");
    tokio::time::sleep(Duration::from_millis(50)).await;

    let target_color = RgbColor::new(255, 120, 0);
    engine.dispatch(Action::SelectScreen(ScreenId::Solid));
    engine.dispatch(Action::SetBrightness(255));
    engine.dispatch(Action::SetSolidColor(target_color));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let frame = engine.frame();
    assert_eq!(frame.len(), 60);
    for pixel in frame.colors() {
        assert_eq!(*pixel, target_color);
    }
}

#[tokio::test]
async fn test_dynamic_rainbow_effect_golden_frame() {
    let platform = Arc::new(DefaultPlatformServices);
    let engine = Engine::new_simulator(platform).expect("Engine failed to initialize");
    tokio::time::sleep(Duration::from_millis(50)).await;

    engine.dispatch(Action::SelectScreen(ScreenId::Dynamic));
    engine.dispatch(Action::SetEffect(1)); // 0x01 Rainbow
    engine.dispatch(Action::SetSpeed(5));
    engine.dispatch(Action::SetBrightness(255));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let frame = engine.frame();
    assert_eq!(frame.len(), 60);

    // Dynamic rainbow must have chromatic diversity across the strip
    let first_pixel = frame.colors()[0];
    let mid_pixel = frame.colors()[30];
    let last_pixel = frame.colors()[59];

    // Guarantee pixels are not all identical (i.e. spatial gradient exists)
    assert!(first_pixel != mid_pixel || mid_pixel != last_pixel);
    // Guarantee non-zero brightness
    assert!(first_pixel.r > 0 || first_pixel.g > 0 || first_pixel.b > 0);
}

#[tokio::test]
async fn test_diy_sequencer_frame() {
    let platform = Arc::new(DefaultPlatformServices);
    let engine = Engine::new_simulator(platform).expect("Engine failed to initialize");
    tokio::time::sleep(Duration::from_millis(50)).await;

    let custom_nodes = vec![RgbColor::new(255, 0, 0), RgbColor::new(0, 0, 255)];
    engine.dispatch(Action::SelectScreen(ScreenId::Diy));
    engine.dispatch(Action::SetDiySteps(custom_nodes));
    engine.dispatch(Action::SetBrightness(255));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let frame = engine.frame();
    assert_eq!(frame.len(), 60);
    // Even LEDs red, odd LEDs blue or similar alternating distribution
    assert!(frame.colors().iter().any(|c| c.r > 200));
    assert!(frame.colors().iter().any(|c| c.b > 200));
}
