use pyxillum_protocol::*;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct PowerVector {
    input: PowerInput,
    expected_bytes: Vec<u8>,
}

#[derive(Deserialize)]
struct PowerInput {
    power: bool,
}

#[derive(Deserialize)]
struct ColorVector {
    action: String,
    input: serde_json::Value,
    expected_bytes: Vec<u8>,
}

#[derive(Deserialize)]
struct EffectVector {
    action: String,
    input: serde_json::Value,
    expected_bytes: Vec<u8>,
}

#[derive(Deserialize)]
struct NotificationVector {
    chunks: Vec<NotificationChunk>,
    expected_status: ExpectedStatus,
}

#[derive(Deserialize)]
struct NotificationChunk {
    raw_hex: String,
}

#[derive(Deserialize)]
struct ExpectedStatus {
    power: bool,
    light_mode: u8,
    effect_id: u8,
    chip_order: u8,
    brightness: u8,
    speed: u8,
    length: u8,
    rgb: Vec<u8>,
    audio_input: u8,
    audio_sensitivity: u8,
}

fn vectors_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests")
        .join("vectors")
}

#[test]
fn test_power_vectors() {
    let path = vectors_dir().join("power.json");
    let content = fs::read_to_string(&path).expect("Failed to read power.json");
    let vectors: Vec<PowerVector> = serde_json::from_str(&content).expect("Invalid JSON");

    for v in vectors {
        let cmd = Command::Power(v.input.power);
        assert_eq!(cmd.encode(), v.expected_bytes);
    }
}

#[test]
fn test_color_vectors() {
    let path = vectors_dir().join("color.json");
    let content = fs::read_to_string(&path).expect("Failed to read color.json");
    let vectors: Vec<ColorVector> = serde_json::from_str(&content).expect("Invalid JSON");

    for v in vectors {
        let encoded = match v.action.as_str() {
            "set_color" => {
                let r = v.input["r"].as_u64().unwrap() as u8;
                let g = v.input["g"].as_u64().unwrap() as u8;
                let b = v.input["b"].as_u64().unwrap() as u8;
                let level = v.input["level"].as_u64().unwrap() as u8;
                Command::SetColor {
                    color: RgbColor::new(r, g, b),
                    level,
                }
                .encode()
            }
            "set_brightness" => {
                let level = v.input["level"].as_u64().unwrap() as u8;
                Command::SetBrightness(level).encode()
            }
            other => panic!("unknown action: {}", other),
        };
        assert_eq!(encoded, v.expected_bytes);
    }
}

#[test]
fn test_effects_vectors() {
    let path = vectors_dir().join("effects.json");
    let content = fs::read_to_string(&path).expect("Failed to read effects.json");
    let vectors: Vec<EffectVector> = serde_json::from_str(&content).expect("Invalid JSON");

    for v in vectors {
        let encoded = match v.action.as_str() {
            "set_effect" => {
                let id = v.input["effect_id"].as_u64().unwrap() as u8;
                Command::SetEffect(id).encode()
            }
            "set_speed" => {
                let speed = v.input["speed"].as_u64().unwrap() as u8;
                Command::SetSpeed(speed).encode()
            }
            "set_length" => {
                let length = v.input["length"].as_u64().unwrap() as u8;
                Command::SetLength(length).encode()
            }
            "set_chip_order" => {
                let order = v.input["order"].as_u64().unwrap() as u8;
                Command::SetChipOrder(ChipOrder::from_u8(order).unwrap()).encode()
            }
            "query_state" => Command::QueryState.encode(),
            other => panic!("unknown action: {}", other),
        };
        assert_eq!(encoded, v.expected_bytes);
    }
}

#[test]
fn test_notification_vectors() {
    let path = vectors_dir().join("notifications.json");
    let content = fs::read_to_string(&path).expect("Failed to read notifications.json");
    let vectors: Vec<NotificationVector> = serde_json::from_str(&content).expect("Invalid JSON");

    for v in vectors {
        let mut asm = StatusAssembler::new();
        let mut final_status = None;
        for chunk in &v.chunks {
            let bytes = hex::decode(&chunk.raw_hex).unwrap();
            if let Some(status) = asm.feed(&bytes).unwrap() {
                final_status = Some(status);
            }
        }

        let status = final_status.expect("Assembly did not complete");
        assert_eq!(status.power, v.expected_status.power);
        assert_eq!(status.mode as u8, v.expected_status.light_mode);
        assert_eq!(status.effect, v.expected_status.effect_id);
        assert_eq!(status.chip_order as u8, v.expected_status.chip_order);
        assert_eq!(status.brightness, v.expected_status.brightness);
        assert_eq!(status.speed, v.expected_status.speed);
        assert_eq!(status.length, v.expected_status.length);
        assert_eq!(
            status.color,
            RgbColor::new(
                v.expected_status.rgb[0],
                v.expected_status.rgb[1],
                v.expected_status.rgb[2]
            )
        );
        assert_eq!(status.audio_input as u8, v.expected_status.audio_input);
        assert_eq!(status.sensitivity, v.expected_status.audio_sensitivity);
    }
}

mod hex {
    pub fn decode(s: &str) -> Result<Vec<u8>, ()> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ()))
            .collect()
    }
}
