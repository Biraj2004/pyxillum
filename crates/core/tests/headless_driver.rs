//! # Headless Driver Test Suite
//!
//! Validates that headless environments (CLI, daemon workers, test runners) can drive
//! the `Engine` synchronously and asynchronously without any UI toolkit dependencies.

use pyxillum_core::{Action, DefaultPlatformServices, Engine, ScreenId};
use pyxillum_protocol::RgbColor;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_headless_engine_workflow() {
    let platform = Arc::new(DefaultPlatformServices);
    let engine = Engine::new_simulator(platform).expect("Failed to initialize headless engine");
    tokio::time::sleep(Duration::from_millis(50)).await;

    // 1. Initial snapshot inspect
    let initial_snap = engine.snapshot();
    assert!(initial_snap.preview_mode);
    assert!(initial_snap.device_state.power);

    // 2. Perform sequence of headless CLI operations
    engine.dispatch(Action::SelectScreen(ScreenId::Solid));
    engine.dispatch(Action::SetSolidColor(RgbColor::new(255, 120, 0)));
    engine.dispatch(Action::SetBrightness(200));
    engine.dispatch(Action::SetSpeed(7));

    tokio::time::sleep(Duration::from_millis(150)).await;
    let final_snap = engine.snapshot();

    assert_eq!(final_snap.device_state.color, RgbColor::new(255, 120, 0));
    assert_eq!(final_snap.device_state.brightness, 200);
    assert_eq!(final_snap.device_state.speed, 7);

    // 3. Frame pulling works headlessly
    let frame = engine.frame();
    assert_eq!(frame.len(), 60);
    assert_eq!(frame.colors()[0], RgbColor::new(200, 94, 0)); // scaled by 200/255
}
