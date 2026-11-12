# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-12

### Added
- **Clean-room Protocol Codec (`pyxillum-protocol`)**:
  - Strongly typed commands (`Power`, `SetColor`, `SetBrightness`, `SetEffect`, `SetSpeed`, `SetLength`, `SetChipOrder`, `SetLightMode`, `SetSensitivity`, `QueryState`).
  - Wire serializer for the `0xA0` command family.
  - Chunked notification assembler and parser for `0x53 0x43` status responses.
  - Unit tests, `proptest` generative fuzzing, and static vector test suites.
- **Hardware Simulator (`pyxillum-sim`)**:
  - In-memory virtual SP621E controller state machine.
  - Rate limiting enforcement and packet format validation.
  - Dynamic 60 FPS simulated pixel strip frame generation.
- **Transport Abstraction (`pyxillum-transport`)**:
  - Asynchronous `Transport` trait.
  - `SimTransport` routing directly to virtual simulator without hardware dependencies.
  - `BtleplugTransport` with software-level manufacturer and service filtering.
- **Core Domain & State Coordination (`pyxillum-core`)**:
  - Background asynchronous device manager with state synchronization.
  - Versioned scene preset storage in OS configuration folders (`PresetStore`).
- **Command-Line Interface (`pyxillum-cli`)**:
  - Commands for `scan`, `status`, `power`, `color`, `brightness`, `effect`, and `configure`.
  - Safety gating on persistent configuration via `--i-understand-this-changes-the-device`.
  - Interactive hardware verification pack runner (`hwcheck`).
- **Native Desktop GUI (`pyxillum-app`)**:
  - Immediate-mode cross-platform desktop application built with `egui` and `eframe`.
  - Live virtual LED strip monitor with glowing circular diode rendering.
  - One-click "Demo Device" simulator toggle for immediate hardware-free exploration.
  - Real-time colour wheel, brightness/speed sliders, curated presets, and live diagnostic hex log.
- **Developer Automation (`xtask`)**:
  - `cargo xtask check`, `cargo xtask test`, and `cargo xtask package`.
- **Documentation & Verification Pack**:
  - Protocol specification (`docs/protocol.md`), architecture guide (`docs/architecture.md`), risk register (`docs/unknowns.md`), and legal notes (`docs/legal-notes.md`).
  - Hardware verification plan (`tests/hardware/PLAN.md`).
