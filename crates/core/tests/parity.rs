use std::fs;
use std::path::PathBuf;

#[test]
fn test_parity_matrix_exists_and_covers_all_domains() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.parent().unwrap().parent().unwrap();
    let parity_path = root.join("docs").join("parity.md");

    assert!(
        parity_path.exists(),
        "docs/parity.md must exist at {:?}",
        parity_path
    );

    let content = fs::read_to_string(&parity_path).expect("Failed to read docs/parity.md");

    let required_domains = [
        "Device Discovery",
        "GATT Connection",
        "Master Power Toggle",
        "Master Luminescence / Brightness",
        "Dynamic FX",
        "Solid Chromatic Colour Disc",
        "RGB Channel Sliders",
        "Kelvin Temperature",
        "Music Rhythm",
        "Audio Capture Pipeline",
        "24-Band Spectrum Visualizer",
        "14-Step DIY Keyframe Timeline",
        "Non-Volatile Hardware EEPROM",
        "60 FPS Live Strip Simulation",
        "Zero Idle CPU Monitor Suspend",
    ];

    for domain in required_domains {
        assert!(
            content.contains(domain),
            "docs/parity.md must contain verification domain: {}",
            domain
        );
    }
}

#[test]
fn test_frontends_reference_core_actions() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.parent().unwrap().parent().unwrap();

    let windows_dir = root.join("apps").join("windows");
    let linux_dir = root.join("apps").join("linux");
    let macos_dir = root.join("apps").join("macos");

    assert!(
        windows_dir.exists(),
        "apps/windows must exist for Windows WinUI 3 front end"
    );
    assert!(
        linux_dir.exists(),
        "apps/linux must exist for Linux GTK4 front end"
    );
    assert!(
        macos_dir.exists(),
        "apps/macos must exist for macOS SwiftUI front end"
    );

    // Verify key action keywords across Windows front-end source
    let win_vm = fs::read_to_string(
        windows_dir
            .join("Pyxillum")
            .join("ViewModels")
            .join("AppViewModel.cs"),
    )
    .expect("Failed to read AppViewModel.cs");
    assert!(win_vm.contains("TogglePower"));
    assert!(win_vm.contains("SetBrightness"));
    assert!(win_vm.contains("SetSpeed"));
    assert!(win_vm.contains("Navigate"));

    // Verify Linux UI references core actions
    let linux_ui = fs::read_to_string(linux_dir.join("src").join("ui.rs"))
        .expect("Failed to read linux ui.rs");
    assert!(linux_ui.contains("TogglePower"));

    // Verify macOS ViewModel references core action dispatches
    let macos_vm = fs::read_to_string(
        macos_dir
            .join("Pyxillum")
            .join("ViewModels")
            .join("AppViewModel.swift"),
    )
    .expect("Failed to read macOS AppViewModel.swift");
    assert!(macos_vm.contains("dispatch(action: action)"));
}
