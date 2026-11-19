//! # Hardware Safety Gate Test Suite
//!
//! Validates that non-volatile EEPROM/flash hardware reconfigurations (pixel length,
//! chip sequence ordering) are strictly guarded behind explicit user confirmation.

use pyxillum_core::{Action, DefaultPlatformServices, Engine, ErrorKey};
use pyxillum_protocol::ChipOrder;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_safety_gate_rejection_without_confirmation() {
    let platform = Arc::new(DefaultPlatformServices);
    let engine = Engine::new_simulator(platform).expect("Engine failed to initialize");
    tokio::time::sleep(Duration::from_millis(50)).await;

    let initial_order = engine.snapshot().strip_config.chip_order;
    let initial_length = engine.snapshot().strip_config.length;

    // 1. Attempt non-volatile re-configuration WITHOUT confirmation flag
    engine.dispatch(Action::ConfigureHardware {
        chip_order: ChipOrder::Bgr,
        length: 120,
        confirmed: false,
    });

    tokio::time::sleep(Duration::from_millis(100)).await;
    let snap = engine.snapshot();

    // 2. Guarantee parameters were NOT mutated
    assert_eq!(snap.strip_config.chip_order, initial_order);
    assert_eq!(snap.strip_config.length, initial_length);
    assert_eq!(snap.device_state.chip_order, initial_order);
    assert_eq!(snap.device_state.length, initial_length);

    // 3. Guarantee user error is raised with localized safety gate key
    assert!(snap.user_error.is_some());
    let err = snap.user_error.unwrap();
    assert_eq!(err.key, ErrorKey::SafetyGateRequiresConfirmation);

    // 4. Test error dismissal
    engine.dispatch(Action::DismissError);
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(engine.snapshot().user_error.is_none());
}

#[tokio::test]
async fn test_safety_gate_success_with_confirmation() {
    let platform = Arc::new(DefaultPlatformServices);
    let engine = Engine::new_simulator(platform).expect("Engine failed to initialize");
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Authorize non-volatile re-configuration WITH confirmation flag
    engine.dispatch(Action::ConfigureHardware {
        chip_order: ChipOrder::Rbg,
        length: 90,
        confirmed: true,
    });

    tokio::time::sleep(Duration::from_millis(150)).await;
    let snap = engine.snapshot();

    // Guarantee parameters WERE updated
    assert_eq!(snap.strip_config.chip_order, ChipOrder::Rbg);
    assert_eq!(snap.strip_config.length, 90);
    assert_eq!(snap.device_state.chip_order, ChipOrder::Rbg);
    assert_eq!(snap.device_state.length, 90);
    assert!(snap.user_error.is_none());
}
