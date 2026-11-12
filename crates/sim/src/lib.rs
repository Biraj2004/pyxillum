#![warn(missing_docs)]

//! # Pyxillum Simulator
//!
//! Faithful hardware simulator for the SP621E / BanlanX2 Bluetooth Low Energy pixel controller.
//! Provides an accurate virtual state machine, enforces frame timing limits, validates incoming packets,
//! and synthesizes response notification chunks (`0x53 0x43`).

use pyxillum_protocol::{
    AudioInput, ChipOrder, LightMode, ProtocolError, RgbColor, COMMAND_PREFIX, EFFECT_SOLID,
    NOTIFICATION_MAGIC,
};
use std::time::{Duration, Instant};
use thiserror::Error;

/// Errors emitted by the device simulator when receiving malformed or illegal commands.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SimError {
    /// Packet was too short to contain a valid header.
    #[error("packet too short: expected at least 3 bytes, got {0}")]
    PacketTooShort(usize),

    /// Missing or invalid command prefix byte (expected 0xA0).
    #[error("invalid command prefix: expected 0x{:02X}, got 0x{:02X}", COMMAND_PREFIX, .0)]
    InvalidPrefix(u8),

    /// Opcode payload length mismatch.
    #[error("payload length mismatch: opcode 0x{opcode:02X} specified {declared} bytes, packet had {actual}")]
    LengthMismatch {
        /// Command opcode byte.
        opcode: u8,
        /// Length declared in header.
        declared: usize,
        /// Actual payload bytes present.
        actual: usize,
    },

    /// Unrecognized command opcode byte.
    #[error("unsupported command opcode: 0x{0:02X}")]
    UnknownOpcode(u8),

    /// Inbound write rate exceeded the hardware buffer threshold.
    #[error("rate limit violation: writes spaced {actual:?} apart, minimum required is {min:?}")]
    RateLimitViolation {
        /// Actual elapsed duration since last write.
        actual: Duration,
        /// Minimum safe interval required by controller MCU.
        min: Duration,
    },

    /// Under-the-hood protocol parsing error.
    #[error("protocol error: {0}")]
    Protocol(#[from] ProtocolError),
}

/// Simulated hardware instance of an SP621E pixel LED controller.
#[derive(Debug, Clone)]
pub struct DeviceSimulator {
    /// Master power state.
    pub power: bool,
    /// Active lighting mode.
    pub mode: LightMode,
    /// Active effect ID code.
    pub effect: u8,
    /// Configured RGB channel order.
    pub chip_order: ChipOrder,
    /// Brightness level (0–255).
    pub brightness: u8,
    /// Animation speed (1–10).
    pub speed: u8,
    /// Configured strip pixel count (1–150).
    pub length: u8,
    /// Active solid RGB colour.
    pub color: RgbColor,
    /// Audio input mode.
    pub audio_input: AudioInput,
    /// Audio sensitivity gain (1–16).
    pub sensitivity: u8,

    // Timing and internal state
    last_write: Option<Instant>,
    min_write_interval: Duration,
    animation_tick: f32,
}

impl Default for DeviceSimulator {
    fn default() -> Self {
        Self {
            power: true,
            mode: LightMode::Single,
            effect: EFFECT_SOLID,
            chip_order: ChipOrder::Grb,
            brightness: 255,
            speed: 5,
            length: 30,
            color: RgbColor::new(255, 120, 0),
            audio_input: AudioInput::InternalMic,
            sensitivity: 8,
            last_write: None,
            min_write_interval: Duration::from_millis(20), // 20ms threshold
            animation_tick: 0.0,
        }
    }
}

impl DeviceSimulator {
    /// Constructs a new virtual simulator with factory defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the minimum required duration between successive writes.
    pub fn set_min_write_interval(&mut self, interval: Duration) {
        self.min_write_interval = interval;
    }

    /// Advances the simulation clock by the given delta duration.
    pub fn tick(&mut self, dt: Duration) {
        self.animation_tick += dt.as_secs_f32() * (self.speed as f32);
    }

    /// Receives and processes a raw incoming GATT write packet.
    ///
    /// Returns `Ok(Some(packets))` containing generated notification response chunks
    /// if the command requests status or mutates state.
    pub fn handle_write(&mut self, packet: &[u8]) -> Result<Option<Vec<Vec<u8>>>, SimError> {
        let now = Instant::now();
        if let Some(prev) = self.last_write {
            let elapsed = now.duration_since(prev);
            if elapsed < self.min_write_interval {
                return Err(SimError::RateLimitViolation {
                    actual: elapsed,
                    min: self.min_write_interval,
                });
            }
        }
        self.last_write = Some(now);

        if packet.len() < 3 {
            return Err(SimError::PacketTooShort(packet.len()));
        }

        if packet[0] != COMMAND_PREFIX {
            return Err(SimError::InvalidPrefix(packet[0]));
        }

        let opcode = packet[1];
        let declared_len = packet[2] as usize;
        let actual_len = packet.len() - 3;

        if declared_len != actual_len {
            return Err(SimError::LengthMismatch {
                opcode,
                declared: declared_len,
                actual: actual_len,
            });
        }

        let payload = &packet[3..];

        let mut status_requested = false;

        match opcode {
            0x62 => {
                // Power
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.power = payload[0] != 0;
            }
            0x63 => {
                // Effect ID
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.effect = payload[0];
            }
            0x64 => {
                // Chip Order
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.chip_order = ChipOrder::from_u8(payload[0])?;
            }
            0x66 => {
                // Brightness
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.brightness = payload[0];
            }
            0x67 => {
                // Speed
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.speed = payload[0].clamp(1, 10);
            }
            0x68 => {
                // Length
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.length = payload[0].clamp(1, 150);
            }
            0x69 => {
                // Color + Level
                if payload.len() < 4 {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 4,
                        actual: payload.len(),
                    });
                }
                self.color = RgbColor::new(payload[0], payload[1], payload[2]);
                self.brightness = payload[3];
                self.effect = EFFECT_SOLID;
            }
            0x6A => {
                // Light Mode
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.mode = LightMode::from_u8(payload[0]);
            }
            0x6B => {
                // Sensitivity
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.sensitivity = payload[0].clamp(1, 16);
            }
            0x6C => {
                // Audio Input
                if payload.is_empty() {
                    return Err(SimError::LengthMismatch {
                        opcode,
                        declared: 1,
                        actual: 0,
                    });
                }
                self.audio_input = AudioInput::from_u8(payload[0]);
            }
            0x70 => {
                // Query state
                status_requested = true;
            }
            0x76 => {
                // White level
                if !payload.is_empty() {
                    self.brightness = payload[0];
                }
            }
            unknown => return Err(SimError::UnknownOpcode(unknown)),
        }

        if status_requested {
            Ok(Some(self.generate_status_chunks()))
        } else {
            Ok(None)
        }
    }

    /// Builds the 2-chunk status response matching real controller notifications.
    pub fn generate_status_chunks(&self) -> Vec<Vec<u8>> {
        // Total payload length is 23 bytes (0x17)
        let total_len: u8 = 23;
        let mut full_payload = vec![0u8; total_len as usize];

        full_payload[0] = if self.power { 1 } else { 0 };
        full_payload[1] = self.mode as u8;
        full_payload[2] = self.effect;
        full_payload[3] = self.chip_order as u8;
        full_payload[4] = self.brightness;
        full_payload[5] = self.speed;
        full_payload[6] = self.length;
        full_payload[7] = self.color.r;
        full_payload[8] = self.color.g;
        full_payload[9] = self.color.b;
        full_payload[10] = self.audio_input as u8;
        full_payload[11] = self.sensitivity;

        // Static firmware flags matching real capture
        let firmware_sig = [0x09, 0x04, 0x0B, 0x14, 0x1A, 0x32, 0x37, 0x50, 0x53, 0x73];
        full_payload[12..22].copy_from_slice(&firmware_sig);
        full_payload[22] = 0; // 0 timers

        // Chunk 1: 15 bytes payload (0x0F)
        let chunk1_len: u8 = 15;
        let mut chunk1 = vec![
            NOTIFICATION_MAGIC[0],
            NOTIFICATION_MAGIC[1],
            0x01,
            total_len,
            chunk1_len,
        ];
        chunk1.extend_from_slice(&full_payload[0..15]);

        // Chunk 2: 8 bytes payload (0x08)
        let chunk2_len: u8 = 8;
        let mut chunk2 = vec![
            NOTIFICATION_MAGIC[0],
            NOTIFICATION_MAGIC[1],
            0x02,
            total_len,
            chunk2_len,
        ];
        chunk2.extend_from_slice(&full_payload[15..23]);

        vec![chunk1, chunk2]
    }

    /// Renders the virtual LED strip's pixel buffer based on the active state and tick count.
    pub fn render_strip(&self) -> Vec<RgbColor> {
        if !self.power {
            return vec![RgbColor::BLACK; self.length as usize];
        }

        let num_pixels = self.length as usize;
        let mut pixels = Vec::with_capacity(num_pixels);

        let factor = (self.brightness as f32) / 255.0;

        match self.effect {
            EFFECT_SOLID => {
                let scaled = RgbColor::new(
                    (self.color.r as f32 * factor) as u8,
                    (self.color.g as f32 * factor) as u8,
                    (self.color.b as f32 * factor) as u8,
                );
                pixels.resize(num_pixels, scaled);
            }
            0x01 => {
                // Rainbow sweep
                for i in 0..num_pixels {
                    let hue = (self.animation_tick * 10.0 + (i as f32 * 360.0 / num_pixels as f32))
                        % 360.0;
                    let (r, g, b) = hsv_to_rgb(hue, 1.0, factor);
                    pixels.push(RgbColor::new(r, g, b));
                }
            }
            _ => {
                // Generic animated dynamic pattern
                for i in 0..num_pixels {
                    let wave =
                        (((self.animation_tick * 5.0 + i as f32).sin() + 1.0) * 0.5) * factor;
                    let r = (self.color.r as f32 * wave) as u8;
                    let g = (self.color.g as f32 * wave) as u8;
                    let b = (self.color.b as f32 * wave) as u8;
                    pixels.push(RgbColor::new(r, g, b));
                }
            }
        }

        pixels
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_prime + m) * 255.0) as u8,
        ((g_prime + m) * 255.0) as u8,
        ((b_prime + m) * 255.0) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyxillum_protocol::{Command, StatusAssembler};

    #[test]
    fn test_simulator_power_and_status_query() {
        let mut sim = DeviceSimulator::new();
        sim.set_min_write_interval(Duration::ZERO);

        let turn_off = Command::Power(false).encode();
        let res = sim.handle_write(&turn_off).unwrap();
        assert!(res.is_none());
        assert!(!sim.power);

        let query = Command::QueryState.encode();
        let chunks = sim.handle_write(&query).unwrap().unwrap();
        assert_eq!(chunks.len(), 2);

        let mut asm = StatusAssembler::new();
        let _ = asm.feed(&chunks[0]).unwrap();
        let status = asm.feed(&chunks[1]).unwrap().unwrap();
        assert!(!status.power);
    }

    #[test]
    fn test_simulator_rejects_bad_prefix() {
        let mut sim = DeviceSimulator::new();
        sim.set_min_write_interval(Duration::ZERO);

        let bad = [0x53, 0x62, 0x01, 0x01];
        let err = sim.handle_write(&bad).unwrap_err();
        assert_eq!(err, SimError::InvalidPrefix(0x53));
    }

    #[test]
    fn test_simulator_rate_limiting() {
        let mut sim = DeviceSimulator::new();
        sim.set_min_write_interval(Duration::from_millis(50));

        let cmd = Command::Power(true).encode();
        assert!(sim.handle_write(&cmd).is_ok());

        // Immediate write should trigger rate limit error
        let err = sim.handle_write(&cmd).unwrap_err();
        match err {
            SimError::RateLimitViolation { .. } => {}
            other => panic!("expected RateLimitViolation, got {:?}", other),
        }
    }
}
