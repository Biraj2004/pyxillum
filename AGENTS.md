# Agent & Contributor Technical Guidelines

This document provides complete technical context, architecture specifications, legal boundaries, and operational procedures for software agents, automated tools, and human contributors working on **Pyxillum**.

---

## 1. Project Purpose & Scope

Pyxillum is an open-source, native, cross-platform (Windows, macOS, Linux) desktop application and command-line interface written in Rust to control Bluetooth Low Energy (BLE) pixel LED controllers of the SP621E/SP6xxE hardware family.

Key project tenets:
- **Clean-Room Engineering**: Reverse engineering is conducted exclusively for interoperability under statutory exemptions.
- **Offline-First & Zero Telemetry**: The application operates entirely locally. No cloud accounts, analytics, crash reporters, or background network calls.
- **Simulator-First Development**: Complete in-memory virtual hardware simulation (`crates/sim`) permits 100% of the UI, state machine, and command pipelines to be developed and verified without physical hardware connected.
- **Strict Legal Protection & Immunity**: Third-party mobile companion application names must never be cited or committed. All hardware compatibility citations are strictly factual nominative references. The software is provided under AGPL-3.0 with complete warranty disclaimers and limitations of liability.
- **Tone & Style**: British/Indian English spelling throughout (e.g. "colour", "optimised", "initialised", "behaviour"). Strictly no emojis in code, documentation, or commit messages.

---

## 2. Confidence Tagging System

Because physical hardware may not always be available during development, every protocol fact, command opcode, and feature capability must carry an explicit confidence status:

| Confidence Tag | Meaning | Permitted Claims |
| :--- | :--- | :--- |
| **`OBSERVED-STATIC`** | Observed in static analysis or decompilation notes. | "Implemented, untested on hardware" |
| **`CORROBORATED`** | Confirmed by an independent public source or reference implementation. | "Implemented, untested on hardware" |
| **`VERIFIED-HW`** | Confirmed through live over-the-air execution on physical controller hardware. | "Verified on hardware" / "Supported" |
| **`INFERRED`** | Deduced from surrounding protocol structures or logical pattern analysis. | "Inferred, untested" |
| **`UNKNOWN`** | Undetermined behaviour requiring physical hardware capture or oscilloscope traces. | "Unknown" |

> [!IMPORTANT]
> **Honest Capability Rule**: Under no circumstances may a feature or device model be labelled as "working" or "supported" in documentation or README until it has been confirmed on physical hardware and tagged **`VERIFIED-HW`**. Until verified, all implementations must be explicitly labelled **"implemented, untested on hardware"**.

---

## 3. Hardware Safety & Risky Command Gating

Physical pixel LED controllers store configuration parameters in non-volatile flash/EEPROM registers. Writing corrupt, out-of-range, or excessive values can desynchronise controller timing or cause persistent hardware malfunction.

### Safety Rules:
1. **Safety Confirmation Gate**: Any command that mutates non-volatile hardware parameters (colour sequence/chip order, pixel count, IC timing) must be gated behind an explicit confirmation flag:
   ```bash
   --i-understand-this-changes-the-device
   ```
   Without this flag, the CLI and UI must abort the operation immediately and warn the user.
2. **Write Pacing**: Unacknowledged BLE writes (`WRITE_WITHOUT_RESPONSE`) must be paced with at least 30 ms between consecutive packets. The `DeviceManager` enforces this pacing automatically to prevent controller FIFO buffer overflow.
3. **Firmware Updates & OTA**: Firmware flashing and over-the-air (OTA) update routines are **strictly forbidden**. Under no circumstances should OTA routines be reverse engineered or implemented.

---

## 4. Legal Immunity & Clean-Room Boundaries

To maintain legal immunity and protect developers and contributors:

1. **Quarantine Directory (`re-workspace/`)**:
   - All static analysis notes, decompiled artifacts, raw APK files, and packet captures must reside exclusively in `./re-workspace/`.
   - The `./re-workspace/` folder is gitignored and must never be staged or committed.
2. **Independent Implementation**:
   - All production Rust code in `crates/` is written fresh from functional specifications in `docs/protocol.md`.
   - No third-party proprietary source code, decompiled snippets, graphics, icons, fonts, or audio assets may be copied into the repository.
3. **Proprietary Application Names**:
   - The names of third-party mobile companion applications (such as proprietary Android packages) must **never be mentioned** in README, documentation, code comments, or commit messages.
   - Refer only to generic, factual hardware compatibility terms (e.g. "SP621E-compatible Bluetooth Low Energy pixel LED controllers", "companion mobile applications").
4. **Statutory Interoperability Exemptions**:
   - Reverse engineering of unencrypted GATT protocols for interoperability is protected under Directive 2009/24/EC Article 6 (European Union), 17 U.S.C. § 1201(f) (United States), and Section 52(1)(ab) of the Indian Copyright Act, 1957.
5. **Warranty Disclaimer**:
   - The project is distributed under the GNU Affero General Public License v3.0 (`AGPL-3.0-or-later`). Authors and contributors carry zero liability for hardware damage or misuse.

---

## 5. Workspace Topology & Architecture

The Cargo workspace is partitioned into small, decoupled crates with single responsibilities:

```text
pyxillum/
├── crates/
│   ├── protocol/      Pure functional codec (#![forbid(unsafe_code)], zero I/O, proptests)
│   ├── sim/           Virtual SP621E hardware simulator & validator
│   ├── transport/     Abstract Transport trait, SimTransport, and BtleplugTransport
│   ├── core/          Device coordination, write pacing, state sync, and preset store
│   ├── cli/           Command-line interface (pyxillum) and hardware verification wizard
│   └── app/           Native desktop GUI presentation layer (egui / eframe)
├── xtask/             Repository automation runner (check, test, fmt, package, msi)
├── assets/            Vector SVGs, multi-resolution PNGs, Windows .ico, and WiX MSI bitmaps
├── tests/
│   ├── vectors/       Static-derived JSON protocol test vectors
│   └── hardware/      Hardware verification checklist and test plan (PLAN.md)
└── docs/              Architecture, protocol, ADRs, legal notes, Windows build guide
```

### Crate Boundary Responsibilities:
- **`crates/protocol`**: Operates on raw byte slices. Contains packet framing encoders, command builders, and chunked notification reassemblers. No async, no network, no filesystem.
- **`crates/sim`**: Implements the virtual controller state machine. Enforces write rate limits (minimum 20 ms), rejects malformed headers, synthesises response notifications (`0x53 0x43`), and computes 60 FPS pixel strip rendering buffers.
- **`crates/transport`**: Provides `Transport` trait. `SimTransport` routes commands directly to the in-memory simulator. `BtleplugTransport` interacts with real BLE adapters using software-side filtering for manufacturer ID `0x5053` and service UUID `0xFFE0` to circumvent cross-platform driver bugs.
- **`crates/core`**: Manages `DeviceManager`. Enforces 30 ms write pacing between outbound commands, maintains thread-safe `RwLock<DeviceState>`, and persists scene presets to standard OS directories.
- **`crates/app`**: Presentation only. Never executes BLE I/O on the main thread; delegates commands across an async Tokio mpsc channel. Suppresses console windows in release mode via `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`.
- **`crates/cli`**: User-facing terminal tool. Enforces safety gates and provides an interactive hardware verification runner (`pyxillum hwcheck`).

---

## 6. Development & Quality Workflows

All maintenance and CI workflows are orchestrated via `xtask` (`cargo xtask`):

```bash
# 1. Run complete health check (formatting check, Clippy -D warnings, test suite)
cargo xtask check

# 2. Run all workspace tests (unit tests, proptests, static vector harnesses)
cargo xtask test

# 3. Format code according to rustfmt standards
cargo xtask fmt

# 4. Build optimised release binaries and stage into dist/ and win-exe/
cargo xtask package

# 5. Build Windows MSI installer package via WiX Toolset
cargo xtask msi

# 6. Synchronise compiled release binaries directly to win-exe/ and dist/
cargo xtask sync-exe
```

> [!IMPORTANT]
> **Executable Staging Rule (`win-exe/`)**: Whenever release binaries are compiled or updated, the latest built executables (`pyxillum-desktop.exe` and `pyxillum.exe`) must **always** be copied into the dedicated `win-exe/` folder. This ensures ready-to-run executables are always immediately available at the workspace root. This staging is automated via `cargo xtask package`, `cargo xtask msi`, and `cargo xtask sync-exe`.

---

## 7. Testing in Simulation Mode (Zero Hardware)

Developers and agents can test every feature of Pyxillum without physical Bluetooth hardware using the built-in simulator:

```powershell
# Scan for simulated devices
cargo run -p pyxillum-cli -- scan --sim

# Query virtual device status
cargo run -p pyxillum-cli -- status --sim

# Set static colour
cargo run -p pyxillum-cli -- color 255 120 0 --sim

# Adjust brightness (0-255)
cargo run -p pyxillum-cli -- brightness 180 --sim

# Select effect pattern (code 1-218)
cargo run -p pyxillum-cli -- effect 5 --speed 8 --sim

# Test safety gate rejection
cargo run -p pyxillum-cli -- configure --length 60 --sim

# Authorise persistent modification
cargo run -p pyxillum-cli -- configure --length 60 --i-understand-this-changes-the-device --sim

# Run the 7-step hardware verification wizard against the simulator
cargo run -p pyxillum-cli -- hwcheck --sim
```

---

## 8. Physical Hardware Verification Protocol

When a physical SP621E controller becomes available:
1. Close all mobile applications on paired phones (the controller only permits one active connection).
2. Execute the verification runner:
   ```powershell
   cargo run -p pyxillum-cli -- hwcheck
   ```
3. Follow the guided visual prompts (`HW-01` through `HW-07`) detailed in [tests/hardware/PLAN.md](tests/hardware/PLAN.md).
4. Upon successful confirmation, promote the verified features in [docs/feature-matrix.md](docs/feature-matrix.md) and [README.md](README.md) to **`VERIFIED-HW`**.
