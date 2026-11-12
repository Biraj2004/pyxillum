# Phase 0 Research Report: Clean-Room BLE Pixel LED Desktop Controller

**Date**: 2026-11-12  
**Target Hardware Family**: SP621E / SP6xxE Mini Bluetooth Pixel LED Controllers (`com.spled.scenex`)  
**Methodology**: Clean-Room Reverse Engineering, Public Protocol Corroboration, Hardware Simulator  
**Style Guidelines**: British/Indian English, no colloquialisms, strict absence of emojis  

---

## 1. GitHub Ecosystem Survey (Last 12 Months)

This survey examines active, representative open-source projects across three domains to extract architectural patterns, implementation lessons, and pitfalls to avoid.

### 1.1 Category A: Rust Desktop Applications

#### Project 1: `rerun-io/rerun`
- **Repository**: `rerun-io/rerun`
- **Licence**: MIT OR Apache-2.0 (Dual License)
- **Last Commit**: Active (within last 30 days)
- **Stars**: ~9.5k+
- **Technology Stack**: Rust, `egui`, `eframe`, `wgpu`, `winit`, Tokio
- **Architecture**: Modular workspace decomposing visual presentation (`re_ui`, `re_viewer`), time-series streaming engines, and type definitions (`re_types`). Decouples asynchronous data ingestion workers from the immediate-mode rendering loop using crossbeam channels.
- **What to Learn**: Immediate-mode GUI (`egui`) can render high-frequency real-time visualisations (60 FPS) with low overhead when decoupled from I/O workers.
- **What to Avoid**: Unrestricted repainting loops. Without `request_repaint_after()` or event-driven repainting, immediate-mode GUIs can needlessly consume CPU cycles when idle.

#### Project 2: `spacedriveapp/spacedrive`
- **Repository**: `spacedriveapp/spacedrive`
- **Licence**: Apache-2.0
- **Last Commit**: Active (within last 60 days)
- **Stars**: ~32k+
- **Technology Stack**: Rust, Tauri v2, React, TypeScript, Prisma
- **Architecture**: Virtual Distributed File System (VDFS) core implemented in Rust, communicating across Tauri IPC to a web-based frontend.
- **What to Learn**: Strict boundaries between underlying storage/protocol crates and UI presentation.
- **What to Avoid**: The complexity and bundle overhead of maintaining a hybrid web/Rust toolchain when pure Rust tooling suffices for desktop tooling.

#### Project 3: `clash-verge-rev/clash-verge-rev`
- **Repository**: `clash-verge-rev/clash-verge-rev`
- **Licence**: GPL-3.0
- **Last Commit**: Active (within last 30 days)
- **Stars**: ~45k+
- **Technology Stack**: Rust, Tauri v2, React, TypeScript
- **Architecture**: Cross-platform system tray utility managing background daemons, network routing configurations, and system-level event hooks.
- **What to Learn**: Elegant system tray integration, clean desktop notification handling, and reliable auto-start configurations on Windows, macOS, and Linux.
- **What to Avoid**: Copyleft licence entanglement; hard-coupling configuration schema directly to presentation state.

---

### 1.2 Category B: BLE LED Controller Clients & Reverse-Engineered Protocols

#### Project 1: `xevrion/wallglow`
- **Repository**: `xevrion/wallglow`
- **Licence**: MIT
- **Last Commit**: Active (late 2025/2026)
- **Stars**: ~50+
- **Technology Stack**: Python 3.11+, Bleak, BlueZ, Noctalia
- **Architecture**: Minimal CLI daemon that extracts dominant colours from desktop wallpapers and transmits raw Bluetooth Low Energy (BLE) control frames to SP621E, SP611E, and SP620E controllers.
- **What to Learn**: Confirms exact compatibility with the SP621E hardware family. Proves that connection persistence and connection pooling achieve sub-120ms colour-shift latency. Highlights that BanlanX/SceneX controllers permit only **one active Bluetooth connection at a time**.
- **What to Avoid**: Fragile hardcoded GATT payloads without formal frame validation, type-safe error boundaries, or checksum verification.

#### Project 2: `monty68/uniled`
- **Repository**: `monty68/uniled`
- **Licence**: MIT
- **Last Commit**: Active (within last 90 days)
- **Stars**: ~160+
- **Technology Stack**: Python, Bleak, Home Assistant Core Custom Components
- **Architecture**: Universal abstraction layer for Bluetooth and local Wi-Fi LED controllers. Maps distinct device families (including BanlanX SP621E, SP611E, SP608E, SP601E, and SP107E) into standardized light primitives (brightness, RGB, effect, speed).
- **What to Learn**: Clean separation between high-level logical light state and low-level protocol frame assemblers. Accurate mapping of manufacturer IDs (`0x5053` / `20563`) and GATT characteristic endpoints (`0xFFE0` / `0xFFE1`).
- **What to Avoid**: Large monolithic Python files with complex conditional branching for device-specific quirks; lack of strict compile-time packet schemas.

#### Project 3: `phhusson/ha-banlanx`
- **Repository**: `phhusson/ha-banlanx`
- **Licence**: GPL-3.0
- **Last Commit**: Reference repository
- **Stars**: ~20+
- **Technology Stack**: Python, Bleak, Home Assistant
- **Architecture**: Early reverse-engineered proof-of-concept for BanlanX controllers. Accompanied by documentation in `REVERSE-NOTES`.
- **What to Learn**: Foundational documentation of the byte framing structure and connection exclusivity requirement.
- **What to Avoid**: Copyleft code borrowing; lack of connection retry backoff; hardcoded hardware addresses.

---

### 1.3 Category C: Rust BLE Applications (`btleplug`)

#### Project 1: `andrewdavidmackenzie/meshchat`
- **Repository**: `andrewdavidmackenzie/meshchat`
- **Licence**: GPL-3.0
- **Last Commit**: Active (within last 60 days)
- **Stars**: ~100+
- **Technology Stack**: Rust, Iced GUI, `btleplug` 0.13, Tokio
- **Architecture**: Cross-platform desktop interface for communicating with Meshtastic LoRa radios over BLE.
- **What to Learn**: **Crucial discovery**: Native `btleplug` scan filters fail silently or behave inconsistently across different operating systems (particularly between Linux BlueZ and Windows WinRT). The robust solution is to scan unfiltered and filter discovered advertisements in application logic.
- **What to Avoid**: Binding GUI presentation state directly to the raw `btleplug` event stream without intermediate buffer channels.

#### Project 2: `Neewer-Bridge/Neewer-Bridge`
- **Repository**: `Fohdeesha/Neewer-Bridge`
- **Licence**: MIT
- **Last Commit**: Active (within last 12 months)
- **Stars**: ~60+
- **Technology Stack**: Rust, Tokio, `btleplug` 0.11, Clap
- **Architecture**: Background network daemon translating ArtNet DMX packets over UDP into BLE GATT commands for Neewer studio lights.
- **What to Learn**: Asynchronous worker thread architecture: command queuing, reconnection backoff, and packet rate limiting.
- **What to Avoid**: Legacy `btleplug` 0.11 dependencies; absence of an in-memory mock transport for non-hardware development.

#### Project 3: `MnlPhlp/tauri-plugin-blec`
- **Repository**: `MnlPhlp/tauri-plugin-blec`
- **Licence**: MIT
- **Last Commit**: Active (within last 12 months)
- **Stars**: ~40+
- **Technology Stack**: Rust, `btleplug` 0.13, Tauri v2
- **Architecture**: Tauri plugin wrapping `btleplug` discovery, connection, and GATT characteristic read/write/notify operations behind IPC calls.
- **What to Learn**: Clean serialisation patterns for passing binary characteristic events across asynchronous boundaries.
- **What to Avoid**: Passing raw byte buffers without typed message structures.

---

## 2. Public Knowledge & Attribution of the SP6xxE Family Protocol

### 2.1 Bluetooth Profile & GATT Topology
Corroborated by `monty68/uniled`, `xevrion/wallglow`, and community reverse-engineering notes:
- **Service UUID**: Standard transparent 16-bit UUID mapped to 128-bit:
  - `0000ffe0-0000-1000-8000-00805f9b34fb` (short `0xFFE0`).
  - Alternative firmware revisions expose `0000e0ff-...` or `0000ffb0-...`.
- **Characteristic UUID**:
  - `0000ffe1-0000-1000-8000-00805f9b34fb` (short `0xFFE1`).
  - Permissions: Write Without Response (`WRITE_NO_RESP`), Write (`WRITE`), and Notify (`NOTIFY`).
- **Bluetooth SIG Manufacturer Identifier**:
  - `0x5053` (`20563` decimal), representing Shenzhen Sperll Optoelectronic Technology ("PS" in ASCII little-endian).

### 2.2 Critical Physical Hardware Constraints
1. **Single Active Connection Exclusivity**:
   The controller hardware allows only one active BLE central connection at any moment. If the original smartphone app (`com.spled.scenex` or BanlanX) is connected or operating in the background on a paired mobile device, connection requests from the desktop host will consistently time out.
2. **MTU Limitations**:
   The default BLE MTU is 23 bytes (yielding a 20-byte ATT payload limit). High-speed updates and pixel maps must adhere to chunked framing or negotiate an extended MTU.
3. **Write Type**:
   High-frequency streaming (music mode, dynamic colour sweeps) relies on `Write Without Response` to avoid round-trip latency bottlenecks.

---

## 3. Architecture Evaluations Summary

### 3.1 GUI Stack (Refer to `docs/adr/0001-gui-stack.md`)
- **Evaluated**: Tauri v2, Iced, egui/eframe, Slint, Dioxus.
- **Recommendation**: **`egui` with `eframe`** (Primary).
  - Rationale: Single-language pure Rust toolchain (`cargo build` only, zero Node/npm requirements); immediate-mode custom canvas rendering (`egui::Painter`) effortlessly displays live, animated 60 FPS pixel strip simulations without hardware; native colour picker and sliders available immediately.
  - Secondary alternative: Tauri v2 (if web-standard CSS/HTML UI is demanded).

### 3.2 BLE Library (Refer to `docs/adr/0002-ble-library.md`)
- **Evaluated**: `btleplug` vs platform-specific native libraries (`bluer`, `windows` WinRT, Apple `CoreBluetooth`).
- **Recommendation**: **`btleplug` v0.13** isolated behind a custom `Transport` trait in `crates/transport`.
  - Software filtering: Scan unfiltered at the adapter level to bypass known multi-platform filter bugs, then filter in application logic by manufacturer ID `0x5053` and service UUID `0xFFE0`.
  - Zero-hardware isolation: Implement `SimTransport` alongside `BtleplugTransport` to route all packets to the virtual device simulator (`crates/sim`).

---

## 4. Feature Matrix Summary (Refer to `docs/feature-matrix.md`)
- Core controls (Power, Brightness, Static RGB, Dynamic Effects, Speed, Direction) are `CORROBORATED` and fully simulated.
- Hardware configuration parameters (IC Chip, Pixel Length, Colour Sequence) are `OBSERVED-STATIC` and strictly gated behind safety confirmations.
- Firmware update / OTA functionality is deliberately excluded to protect user hardware from bricking.

---

## 5. Candidate Name Proposals & Collision Audits

In accordance with strict branding and trademark requirements:
- The project name must be original, memorable, and distinct.
- Must not resemble "SceneX", "SP6xxE", "SPLED", "BanlanX", "Sperll", or related trademarks.
- Checked against crates.io, npm, GitHub, Google Play, Apple App Store, and general web search.

### Candidate 1: `Pyxillum`
- **Etymology**: Derived from *pyxis* (capsule/pixel) and *illuminare* (to illuminate).
- **Collisions**:
  - **crates.io**: 0 matches. Name completely available.
  - **npm**: 0 packages found. Name completely available.
  - **GitHub**: 0 matching repositories.
  - **Google Play**: 0 applications found.
  - **Apple App Store**: 0 applications found.
  - **Web Search**: No commercial products, software trademarks, or lighting brands.

### Candidate 2: `Chromavane`
- **Etymology**: Derived from *chroma* (colour) and *vane* (weather vane / directional pointer, symbolizing directed light flows).
- **Collisions**:
  - **crates.io**: 0 matches. Name completely available.
  - **npm**: 0 packages found. Name completely available.
  - **GitHub**: 0 matching repositories.
  - **Google Play**: 0 applications found.
  - **Apple App Store**: 0 applications found.
  - **Web Search**: No active software products or registered lighting trademarks.

### Candidate 3: `Lumenstrand`
- **Etymology**: Derived from *lumen* (measure of light flux) and *strand* (continuous line of addressable pixel LEDs).
- **Collisions**:
  - **crates.io**: 0 matches. Name completely available.
  - **npm**: 0 packages found. Name completely available.
  - **GitHub**: 0 matching repositories.
  - **Google Play**: 0 applications found.
  - **Apple App Store**: 0 applications found.
  - **Web Search**: No active software packages or consumer lighting brands.

---

## 6. Clean-Room Separation & Legal Principles

1. **Interoperability Rationale**:
   The objective of this project is purely interoperability with consumer LED controllers purchased by the user. Analysis is conducted for protocol compatibility only.
2. **Untracked Analysis Boundary**:
   All reverse-engineering artefacts, raw APK files, intermediate decompilations, and identity records reside exclusively in `./re-workspace/`, which is ignored by version control.
3. **Fresh Implementation**:
   Crates in `crates/` contain solely fresh, original Rust code written to conform with protocol specifications documented in `docs/protocol.md`. No proprietary code or assets are incorporated.
4. **Attribution**:
   References to external open-source projects (`monty68/uniled`, `xevrion/wallglow`) are documented factually with respective MIT licences acknowledged.
