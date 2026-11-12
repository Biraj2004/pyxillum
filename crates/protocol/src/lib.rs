#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # Pyxillum Protocol
//!
//! Pure Rust codec for Bluetooth Low Energy LED pixel controllers in the SP621E / BanlanX2 family.
//! This crate contains zero I/O, zero asynchronous code, and `#![forbid(unsafe_code)]`.
//!
//! Protocol facts implemented herein are clean-room derived and corroborated against
//! open-source references (`monty68/uniled`, `xevrion/wallglow`).

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Standard Bluetooth SIG Company Identifier for Sperll / SP-series controllers.
pub const MANUFACTURER_ID_SPERLL: u16 = 20563; // 0x5053 ("SP")

/// Alternative manufacturer ID encountered in some firmware revisions.
pub const MANUFACTURER_ID_SPERLL_ALT: u16 = 5053;

/// Primary GATT Service UUID (128-bit).
pub const SERVICE_UUID: &str = "0000ffe0-0000-1000-8000-00805f9b34fb";

/// Data Characteristic UUID for writes and notifications (128-bit).
pub const DATA_CHARACTERISTIC_UUID: &str = "0000ffe1-0000-1000-8000-00805f9b34fb";

/// Command prefix byte for client-to-controller messages.
pub const COMMAND_PREFIX: u8 = 0xA0;

/// Notification magic header bytes from controller.
pub const NOTIFICATION_MAGIC: [u8; 2] = [0x53, 0x43]; // "SC"

/// Solid colour effect ID.
pub const EFFECT_SOLID: u8 = 0xBE;

/// Solid white effect ID.
pub const EFFECT_WHITE: u8 = 0xBF;

/// Audio / sound reactive mode effect base ID.
pub const EFFECT_SOUND_BASE: u8 = 0xC9;

/// Errors arising during protocol encoding or decoding.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    /// Command payload length exceeds valid limits.
    #[error("payload length {0} exceeds capacity")]
    PayloadTooLong(usize),

    /// Byte value out of accepted operational bounds.
    #[error("value {value} out of range ({min}..={max})")]
    ValueOutOfRange {
        /// Provided value.
        value: u32,
        /// Minimum permitted.
        min: u32,
        /// Maximum permitted.
        max: u32,
    },

    /// Packet framing was truncated or invalid.
    #[error("frame truncated: expected at least {expected} bytes, got {actual}")]
    FrameTruncated {
        /// Expected minimum byte length.
        expected: usize,
        /// Actual byte length received.
        actual: usize,
    },

    /// Unrecognized packet magic header.
    #[error("invalid magic header: expected {expected:?}, got {actual:?}")]
    InvalidMagic {
        /// Expected magic bytes.
        expected: Vec<u8>,
        /// Actual magic bytes.
        actual: Vec<u8>,
    },

    /// Notification chunk sequence arrived out of order.
    #[error("chunk out of sequence: expected {expected}, got {actual}")]
    ChunkOutOfSequence {
        /// Expected chunk sequence index.
        expected: u8,
        /// Received chunk sequence index.
        actual: u8,
    },

    /// Assembled payload shorter than expected schema minimum.
    #[error("status payload too short: expected at least 10 bytes, got {0}")]
    StatusPayloadTooShort(usize),

    /// Invalid chip order index.
    #[error("invalid chip order code: {0}")]
    InvalidChipOrder(u8),
}

/// 24-bit RGB colour triplet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RgbColor {
    /// Red component (0–255).
    pub r: u8,
    /// Green component (0–255).
    pub g: u8,
    /// Blue component (0–255).
    pub b: u8,
}

impl RgbColor {
    /// Constructs a new RGB color.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Solid black (all channels off).
    pub const BLACK: Self = Self::new(0, 0, 0);

    /// Solid white (all channels full).
    pub const WHITE: Self = Self::new(255, 255, 255);
}

/// Addressable pixel RGB channel sequence order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChipOrder {
    /// Standard RGB ordering (0).
    Rgb = 0,
    /// Red-Blue-Green ordering (1).
    Rbg = 1,
    /// Green-Red-Blue ordering (2) - most common for WS2812B strips.
    Grb = 2,
    /// Green-Blue-Red ordering (3).
    Gbr = 3,
    /// Blue-Red-Green ordering (4).
    Brg = 4,
    /// Blue-Green-Red ordering (5).
    Bgr = 5,
}

impl ChipOrder {
    /// Converts a raw byte code to a ChipOrder.
    pub fn from_u8(value: u8) -> Result<Self, ProtocolError> {
        match value {
            0 => Ok(Self::Rgb),
            1 => Ok(Self::Rbg),
            2 => Ok(Self::Grb),
            3 => Ok(Self::Gbr),
            4 => Ok(Self::Brg),
            5 => Ok(Self::Bgr),
            other => Err(ProtocolError::InvalidChipOrder(other)),
        }
    }
}

/// Global operating light cycle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum LightMode {
    /// Play a single selected effect.
    Single = 0,
    /// Auto-cycle through dynamic lighting patterns.
    AutoDynamic = 1,
    /// Auto-cycle through sound-reactive lighting patterns.
    AutoSound = 2,
}

impl LightMode {
    /// Converts a raw byte code to a LightMode.
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::AutoDynamic,
            2 => Self::AutoSound,
            _ => Self::Single,
        }
    }
}

/// Audio input source selection for sound-reactive modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum AudioInput {
    /// Internal hardware microphone on controller.
    InternalMic = 0,
    /// Software audio stream from host/player.
    Player = 1,
    /// Auxiliary/external 3.5mm microphone jack.
    ExternalMic = 2,
}

impl AudioInput {
    /// Converts a raw byte code to an AudioInput source.
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Player,
            2 => Self::ExternalMic,
            _ => Self::InternalMic,
        }
    }
}

/// Strongly-typed command instructions dispatched to the controller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    /// Set master power state (On or Off).
    Power(bool),
    /// Set active effect pattern ID (0x01..0xDA).
    SetEffect(u8),
    /// Reconfigure physical strip color channel sequence.
    SetChipOrder(ChipOrder),
    /// Adjust master brightness (0–255).
    SetBrightness(u8),
    /// Set animation playback speed (1–10).
    SetSpeed(u8),
    /// Set active pixel count / length (1–150).
    SetLength(u8),
    /// Set static RGB color with independent brightness level.
    SetColor {
        /// RGB colour triplet.
        color: RgbColor,
        /// Brightness level (0–255).
        level: u8,
    },
    /// Set global light operating mode.
    SetLightMode(LightMode),
    /// Adjust audio reactivity sensitivity / gain (1–16).
    SetSensitivity(u8),
    /// Select audio input source.
    SetAudioInput(AudioInput),
    /// Query the current device state.
    QueryState,
    /// Set cool white level on RGBW strips.
    SetWhite(u8),
}

impl Command {
    /// Encodes the command into its raw wire byte representation (`0xA0` family).
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::Power(on) => vec![COMMAND_PREFIX, 0x62, 0x01, if *on { 0x01 } else { 0x00 }],
            Self::SetEffect(code) => vec![COMMAND_PREFIX, 0x63, 0x01, *code],
            Self::SetChipOrder(order) => vec![COMMAND_PREFIX, 0x64, 0x01, *order as u8],
            Self::SetBrightness(level) => vec![COMMAND_PREFIX, 0x66, 0x01, *level],
            Self::SetSpeed(speed) => vec![COMMAND_PREFIX, 0x67, 0x01, *speed],
            Self::SetLength(len) => vec![COMMAND_PREFIX, 0x68, 0x01, *len],
            Self::SetColor { color, level } => {
                vec![
                    COMMAND_PREFIX,
                    0x69,
                    0x04,
                    color.r,
                    color.g,
                    color.b,
                    *level,
                ]
            }
            Self::SetLightMode(mode) => vec![COMMAND_PREFIX, 0x6A, 0x01, *mode as u8],
            Self::SetSensitivity(gain) => vec![COMMAND_PREFIX, 0x6B, 0x01, *gain],
            Self::SetAudioInput(input) => vec![COMMAND_PREFIX, 0x6C, 0x01, *input as u8],
            Self::QueryState => vec![COMMAND_PREFIX, 0x70, 0x00],
            Self::SetWhite(level) => vec![COMMAND_PREFIX, 0x76, 0x02, *level, 0x00],
        }
    }
}

/// Fully assembled status response emitted by the controller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceStatus {
    /// Hardware power state.
    pub power: bool,
    /// Active operating mode.
    pub mode: LightMode,
    /// Current effect pattern code.
    pub effect: u8,
    /// Configured RGB channel sequence.
    pub chip_order: ChipOrder,
    /// Current brightness level (0–255).
    pub brightness: u8,
    /// Animation speed (1–10).
    pub speed: u8,
    /// Strip effect length in pixels (1–150).
    pub length: u8,
    /// Active RGB colour.
    pub color: RgbColor,
    /// Audio input source.
    pub audio_input: AudioInput,
    /// Microphone sensitivity gain (1–16).
    pub sensitivity: u8,
    /// Count of active programmed timers.
    pub timer_count: u8,
}

impl DeviceStatus {
    /// Parses a reassembled contiguous status payload.
    pub fn parse(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() < 10 {
            return Err(ProtocolError::StatusPayloadTooShort(payload.len()));
        }

        let power = payload[0] != 0;
        let mode = LightMode::from_u8(payload[1]);
        let effect = payload[2];
        let chip_order = ChipOrder::from_u8(payload[3]).unwrap_or(ChipOrder::Grb);
        let brightness = payload[4];
        let speed = payload[5];
        let length = payload[6];
        let color = RgbColor::new(payload[7], payload[8], payload[9]);
        let audio_input = if payload.len() > 10 {
            AudioInput::from_u8(payload[10])
        } else {
            AudioInput::InternalMic
        };
        let sensitivity = if payload.len() > 11 { payload[11] } else { 8 };
        let timer_count = if payload.len() > 22 { payload[22] } else { 0 };

        Ok(Self {
            power,
            mode,
            effect,
            chip_order,
            brightness,
            speed,
            length,
            color,
            audio_input,
            sensitivity,
            timer_count,
        })
    }
}

/// State machine that collects fragmented notification chunks into a complete status payload.
#[derive(Debug, Default)]
pub struct StatusAssembler {
    accumulated: Vec<u8>,
    expected_total: usize,
    last_chunk: u8,
    completed: bool,
}

impl StatusAssembler {
    /// Creates a new, empty assembler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds an incoming notification packet slice into the assembler.
    ///
    /// Returns `Ok(Some(DeviceStatus))` if this chunk completed a full message.
    pub fn feed(&mut self, packet: &[u8]) -> Result<Option<DeviceStatus>, ProtocolError> {
        if packet.len() < 5 {
            return Err(ProtocolError::FrameTruncated {
                expected: 5,
                actual: packet.len(),
            });
        }

        if packet[0] != NOTIFICATION_MAGIC[0] || packet[1] != NOTIFICATION_MAGIC[1] {
            return Err(ProtocolError::InvalidMagic {
                expected: NOTIFICATION_MAGIC.to_vec(),
                actual: packet[0..2].to_vec(),
            });
        }

        let chunk_num = packet[2];
        let total_len = packet[3] as usize;
        let chunk_len = packet[4] as usize;

        if packet.len() < 5 + chunk_len {
            return Err(ProtocolError::FrameTruncated {
                expected: 5 + chunk_len,
                actual: packet.len(),
            });
        }

        let chunk_data = &packet[5..5 + chunk_len];

        if chunk_num == 1 {
            self.accumulated.clear();
            self.expected_total = total_len;
            self.last_chunk = 1;
            self.completed = false;
        } else if self.accumulated.is_empty() || chunk_num != self.last_chunk + 1 {
            return Err(ProtocolError::ChunkOutOfSequence {
                expected: self.last_chunk + 1,
                actual: chunk_num,
            });
        } else {
            self.last_chunk = chunk_num;
        }

        self.accumulated.extend_from_slice(chunk_data);

        if self.accumulated.len() >= self.expected_total {
            self.completed = true;
            let status = DeviceStatus::parse(&self.accumulated[..self.expected_total])?;
            Ok(Some(status))
        } else {
            Ok(None)
        }
    }

    /// Indicates whether a complete status reply has been assembled.
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Clears internal buffers to receive a new sequence.
    pub fn reset(&mut self) {
        self.accumulated.clear();
        self.expected_total = 0;
        self.last_chunk = 0;
        self.completed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_encoding() {
        assert_eq!(Command::Power(true).encode(), vec![0xA0, 0x62, 0x01, 0x01]);
        assert_eq!(Command::Power(false).encode(), vec![0xA0, 0x62, 0x01, 0x00]);
    }

    #[test]
    fn test_color_encoding() {
        let cmd = Command::SetColor {
            color: RgbColor::new(0xFF, 0x72, 0x72),
            level: 0x70,
        };
        assert_eq!(cmd.encode(), vec![0xA0, 0x69, 0x04, 0xFF, 0x72, 0x72, 0x70]);
    }

    #[test]
    fn test_effect_encoding() {
        assert_eq!(
            Command::SetEffect(EFFECT_SOLID).encode(),
            vec![0xA0, 0x63, 0x01, 0xBE]
        );
        assert_eq!(
            Command::SetBrightness(128).encode(),
            vec![0xA0, 0x66, 0x01, 0x80]
        );
        assert_eq!(Command::SetSpeed(5).encode(), vec![0xA0, 0x67, 0x01, 0x05]);
        assert_eq!(
            Command::SetLength(16).encode(),
            vec![0xA0, 0x68, 0x01, 0x10]
        );
        assert_eq!(Command::QueryState.encode(), vec![0xA0, 0x70, 0x00]);
    }

    #[test]
    fn test_assembler_real_capture() {
        let chunk1 = [
            0x53, 0x43, 0x01, 0x17, 0x0f, 0x01, 0x00, 0xbe, 0x02, 0xff, 0x0a, 0x10, 0xff, 0x00,
            0x5e, 0x01, 0x10, 0x09, 0x0b, 0x14,
        ];
        let chunk2 = [
            0x53, 0x43, 0x02, 0x17, 0x08, 0x1a, 0x32, 0x37, 0x53, 0x73, 0x85, 0x1b, 0x00,
        ];

        let mut asm = StatusAssembler::new();
        let res1 = asm.feed(&chunk1).unwrap();
        assert!(res1.is_none());
        assert!(!asm.is_completed());

        let res2 = asm.feed(&chunk2).unwrap();
        assert!(res2.is_some());
        assert!(asm.is_completed());

        let status = res2.unwrap();
        assert!(status.power);
        assert_eq!(status.mode, LightMode::Single);
        assert_eq!(status.effect, EFFECT_SOLID);
        assert_eq!(status.chip_order, ChipOrder::Grb);
        assert_eq!(status.brightness, 255);
        assert_eq!(status.speed, 10);
        assert_eq!(status.length, 16);
        assert_eq!(status.color, RgbColor::new(255, 0, 94));
    }
}
