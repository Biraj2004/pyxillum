use pyxillum_core::state::ErrorKey;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_strings_toml_contains_all_error_keys() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let strings_path = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("design")
        .join("strings.toml");

    assert!(
        strings_path.exists(),
        "design/strings.toml must exist at {:?}",
        strings_path
    );

    let content = fs::read_to_string(&strings_path)
        .expect("Failed to read design/strings.toml for verification");

    let toml_value: toml::Value =
        toml::from_str(&content).expect("Failed to parse design/strings.toml");

    let errors_table = toml_value
        .get("errors")
        .and_then(|v| v.as_table())
        .expect("design/strings.toml must have an [errors] table");

    let all_error_keys = vec![
        ErrorKey::BluetoothUnavailable,
        ErrorKey::ScanFailed,
        ErrorKey::ConnectionFailed,
        ErrorKey::DeviceDisconnected,
        ErrorKey::CommandFailed,
        ErrorKey::SafetyGateRequiresConfirmation,
        ErrorKey::PresetNotFound,
        ErrorKey::PersistenceFailed,
        ErrorKey::InvalidParameter,
    ];

    let mut missing_keys = Vec::new();
    for error_key in all_error_keys {
        let key_str = error_key.message_key();
        if !errors_table.contains_key(key_str) {
            missing_keys.push(key_str);
        }
    }

    assert!(
        missing_keys.is_empty(),
        "Missing error keys in design/strings.toml [errors] section: {:?}",
        missing_keys
    );
}

#[test]
fn test_strings_toml_has_required_sections() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let strings_path = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("design")
        .join("strings.toml");

    let content = fs::read_to_string(&strings_path).expect("Failed to read design/strings.toml");
    let toml_value: toml::Value = toml::from_str(&content).expect("Failed to parse strings.toml");
    let table = toml_value.as_table().expect("Root must be table");

    let expected_sections: HashSet<&str> = [
        "app",
        "navigation",
        "common",
        "devices",
        "dynamic",
        "solid",
        "music",
        "diy",
        "settings",
        "errors",
    ]
    .iter()
    .copied()
    .collect();

    for section in expected_sections {
        assert!(
            table.contains_key(section),
            "design/strings.toml missing mandatory section: [{}]",
            section
        );
    }
}
