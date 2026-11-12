# Pyxillum

[![CI](https://github.com/example/pyxillum/actions/workflows/ci.yml/badge.svg)](https://github.com/example/pyxillum/actions)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![MSRV: 1.80.0](https://img.shields.io/badge/MSRV-1.80.0-informational.svg)](https://github.com/rust-lang/rust)

**Pyxillum** is an open-source, cross-platform desktop application and command-line utility written in Rust to control Bluetooth Low Energy (BLE) pixel LED controllers of the family associated with the SP621E Mini Bluetooth Controller.

Engineered using clean-room protocol derivation, Pyxillum is completely offline-first, transmits zero telemetry, and features a built-in virtual hardware simulator allowing complete application evaluation without physical hardware.

---

## Key Features

- **Native Desktop Performance**: 100% Rust architecture with immediate-mode rendering (`egui`/`eframe`) providing responsive 60 FPS animation previews and low memory overhead (~15 MB).
- **Virtual Hardware Simulator**: Test every feature out of the box. Toggle "Demo Device" in the GUI or pass `--sim` in the CLI to simulate real controller behaviour, timing constraints, and notification streaming.
- **Dynamic Lighting & Scene Presets**: Full control over static solid colours, master brightness, animation speeds, built-in dynamic patterns (rainbow, meteor, dual snake, breath), and saved scene presets.
- **Hardware Safety Gating**: Persistent NVRAM changes (chip driver types, pixel strip length, colour sequence order) are protected behind explicit confirmation safeguards (`--i-understand-this-changes-the-device`).
- **Comprehensive Hardware Verification Pack**: Includes an interactive guided testing CLI (`pyxillum hwcheck`) and step-by-step verification plan (`tests/hardware/PLAN.md`) to validate physical controllers.

---

## Hardware Compatibility Matrix

In accordance with strict verification criteria, features are marked as **`VERIFIED-HW`** only after validation on physical controller hardware.

| Controller Model | Protocol Family | Feature Coverage | Status |
| :--- | :---: | :--- | :--- |
| **Virtual Device Simulator** | `SP621E (Sim)` | Power, RGB, Brightness, Speed, Effects, Notifications | **VERIFIED (Simulator)** |
| **SP621E Mini Controller** | `BanlanX2` | Full protocol mapping (`0xA0` opcodes, `0xFFE0` GATT) | **Implemented, untested on hardware** |
| **SP611E SPI Controller** | `BanlanX2` | Full protocol mapping | **Implemented, untested on hardware** |
| **SP620E USB Controller** | `BanlanX2` | Full protocol mapping | **Implemented, untested on hardware** |

---

## Quickstart

### Prerequisites
- Rust 1.80.0 or later (MSRV)
- Windows 10/11, macOS 12+, or Linux with BlueZ D-Bus development headers

### 1. Launch the Desktop GUI in Simulator Demo Mode
```bash
# Clone the repository
git clone https://github.com/example/pyxillum.git
cd pyxillum

# Run the native desktop GUI (defaults to virtual simulator)
cargo run -p pyxillum-app
```

### 2. Command-Line Interface (CLI) Usage
```bash
# Scan for real BLE controllers (or test simulator discovery with --sim)
cargo run -p pyxillum-cli -- scan --sim

# Query virtual controller state
cargo run -p pyxillum-cli -- status --sim

# Set solid RGB colour
cargo run -p pyxillum-cli -- color 255 120 0 --sim

# Trigger dynamic pattern (Effect 1 = Rainbow Sweep at Speed 7)
cargo run -p pyxillum-cli -- effect 1 --speed 7 --sim

# Run the interactive hardware verification wizard
cargo run -p pyxillum-cli -- hwcheck --sim
```

---

## Workspace Structure

The project is structured as a decoupled Cargo workspace:

```text
pyxillum/
├── crates/
│   ├── protocol/      Pure functional codec (#![forbid(unsafe_code)], zero I/O)
│   ├── sim/           In-memory virtual SP621E hardware simulator & validator
│   ├── transport/     Transport trait with SimTransport and BtleplugTransport
│   ├── core/          Device coordination, effects engine, and scene persistence
│   ├── cli/           Command-line application and hwcheck verification runner
│   └── app/           Native desktop GUI presentation layer (eframe/egui)
├── xtask/             Repository check and packaging automation
├── tests/
│   ├── vectors/       Static-derived JSON protocol test vectors
│   └── hardware/      Hardware verification checklist and test plan
└── docs/              Architecture, protocol specification, ADRs, legal notes
```

---

## Developer Workflows (`cargo xtask`)

```bash
# Run all format checks, clippy (-D warnings), and test suites
cargo xtask check

# Run workspace unit and property tests
cargo xtask test

# Compile release binaries
cargo xtask package
```

---

## Compatibility and Disclaimer

**Unofficial Software**: Pyxillum is an independent, clean-room open-source project created strictly for hardware interoperability. It is not affiliated with, authorized by, endorsed by, or in any way officially connected with Shenzhen Sperll Optoelectronic Technology Co., Ltd. or the developers of the companion mobile applications (`SceneX`, `BanlanX`).

All brand names, trademarks, and model designations (such as "SP621E", "SceneX", "Sperll") are used solely in a factual, nominative sense to denote hardware compatibility.

---

## Licence

Pyxillum is licensed under the **GNU Affero General Public License v3.0** (`AGPL-3.0-or-later`). See the [LICENSE](LICENSE) file for details.
