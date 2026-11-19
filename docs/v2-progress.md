# Pyxillum V2 Native Architecture Progress Log

Chronological execution log, architectural decisions, baseline measurements, and verification records for the Pyxillum V2 native desktop reconstruction.

---

## Baseline Measurements (Phase 0)

Recorded on: **2026-11-19**  
Source Commit: `bbeae8c` (`master`)  
Development Branch: `dev-v2`  
Host Operating System: Windows 11 (x86_64)  
Toolchain: `rustc 1.85.0-nightly` / `stable`  

### 1. `cargo xtask check` Results
- **Step 1/3 Formatting (`cargo fmt --check`)**: PASSED (zero discrepancies).
- **Step 2/3 Linting (`cargo clippy --all-targets -- -D warnings`)**: PASSED (zero warnings across all workspace targets).
- **Step 3/3 Test Suite Execution (`cargo test --workspace`)**: PASSED (54 tests passed, 0 failed, 0 ignored).

### 2. Workspace Test Count Breakdown
| Crate / Target | Test Count | Status | Notes |
| :--- | :---: | :---: | :--- |
| `crates/app` (`pyxillum_desktop`) | 35 | Passed | Geometry, layout, palette, tokens, headless screens |
| `crates/cli` (`pyxillum`) | 0 | Passed | Entry point tested via integration harnesses |
| `crates/core` (`pyxillum_core`) | 1 | Passed | End-to-end DeviceManager test with simulator |
| `crates/protocol` (`pyxillum_protocol`) | 10 | Passed | 4 unit tests + 2 proptests + 4 vector suite tests |
| `crates/sim` (`pyxillum_sim`) | 5 | Passed | Rate-limiting, buffer rendering, opcode validation |
| `crates/transport` (`pyxillum_transport`) | 3 | Passed | SimTransport, BLE diagnostics, filtering heuristics |
| **Total Workspace Tests** | **54** | **100% Passed** | **Zero failures, zero flakes** |

### 3. Pre-Compiled Staged Executable Sizes (`win-exe/`)
| Executable | Exact Size (Bytes) | Size (Megabytes) | Description |
| :--- | :---: | :---: | :--- |
| `win-exe/pyxillum-desktop.exe` | 5,697,536 B | ~5.43 MB | Standalone GUI with embedded resources & manifest |
| `win-exe/pyxillum.exe` | 1,842,176 B | ~1.76 MB | Standalone command-line tool & verification wizard |

---

## Phase Execution Tracking Matrix

| Phase | Description | Target Crates / Apps | Status | Verification Gate |
| :---: | :--- | :--- | :---: | :--- |
| **0** | **Baseline & Architecture Docs** | `docs/`, `dev-v2` branch | **COMPLETED** | Baseline verified via `cargo xtask check` |
| **1** | **UI-Agnostic Core** | `crates/core`, `crates/app` | **COMPLETED** | Action/AppState/Engine API; 69 workspace tests passing |
| **2** | **Audio & FFI Surface** | `crates/audio`, `crates/ffi` | Pending | UniFFI proc-macros, C#/Swift bindings, audio loopback |
| **3** | **Design Source of Truth** | `design/`, `tools/` | Pending | Tokens, strings.toml, icon map, generator scripts |
| **4** | **Windows Native Front End** | `apps/windows` (WinUI 3) | Pending | C# .NET 8, Mica, NavigationView, accessible controls |
| **5** | **Linux Native Front End** | `apps/linux` (GTK4 + libadwaita)| Pending | GNOME HIG, AdwNavigationSplitView, Flatpak |
| **6** | **macOS Native Front End** | `apps/macos` (SwiftUI) | Pending | macOS 13+, NavigationSplitView, SF Symbols |
| **7** | **Parity, Accessibility & CI** | `docs/parity.md`, `.github/` | Pending | Full feature checklist, Narrator/VoiceOver/Orca |
| **8** | **Retire Egui Legacy App** | `crates/app` | Pending | Formal removal of crates/app; open PR summary |

---

## Phase 0 Execution Summary
- Created isolated feature branch `dev-v2` branched from `master`.
- Added [docs/pyxillum-native-architecture.md](pyxillum-native-architecture.md) capturing the native architecture specification, component boundaries, and phase deliverables.
- Added [docs/adr/0003-native-frontends-and-uniffi-core.md](adr/0003-native-frontends-and-uniffi-core.md) recording the decision to rebuild with platform-native front ends around a single UniFFI Rust core.
- Recorded comprehensive baseline test metrics (54 tests passing) and release binary sizes.

---

## Phase 1 Execution Summary
- **UI-Agnostic Core Implementation**:
  - Implemented exhaustive `Action` enum in `crates/core/src/action.rs` encompassing all user operations across navigation, connection, power, solid colour, dynamic effects, DIY sequences, music modes, strip configuration, and error dismissal.
  - Implemented immutable `AppState` in `crates/core/src/state.rs` modeling connection status, device metadata, screen mode, individual page state, non-volatile strip configuration, preview mode, and strongly-typed user errors with localisable error keys.
  - Implemented `Engine` runtime coordinator in `crates/core/src/engine.rs` with asynchronous action processing, background command dispatching, ~60 Hz coalesced state change notifications, and 60 FPS `StripFrame` virtual rendering.
  - Implemented `StateListener` and `CallbackListener` in `crates/core/src/listener.rs` for decoupling front-end state rendering from the core engine.
  - Implemented `PlatformServices` and `DefaultPlatformServices` in `crates/core/src/services.rs` for cross-platform OS capability abstraction (Bluetooth settings, OS data directories, audio permission querying).
  - Migrated lighting catalogue presets, colour temperature mappings, and sound modes into `crates/core/src/catalog.rs`.
  - Decoupled `crates/app/src/state.rs` and `crates/app/src/catalog.rs` to re-export and consume shared core definitions without breaking egui application functionality.
- **Verification & Test Coverage**:
  - `tests/actions.rs`: 8 automated tests validating power, solid colours, dynamic effects, music modes, DIY sequences, and listener notification coalescing against the virtual hardware simulator.
  - `tests/golden_frame.rs`: 4 regression tests verifying deterministic 60 FPS pixel strip rendering buffers across power-off, solid colours, dynamic rainbow patterns, and DIY steps.
  - `tests/headless_driver.rs`: Complete headless lifecycle test demonstrating end-to-end action dispatching and frame generation without a graphical display.
  - `tests/safety_gate.rs`: 2 security tests ensuring EEPROM-modifying operations (pixel count, colour sequence) strictly require `--i-understand-this-changes-the-device` confirmation.
  - Total workspace test count increased from 54 to 69 tests passing (100% pass rate).
- **`cargo xtask check` Gate**:
  - Formatting (`cargo fmt --check`): PASSED.
  - Lints (`cargo clippy --all-targets -- -D warnings`): PASSED.
  - Workspace tests (`cargo test --workspace`): 69 passed, 0 failed, 0 ignored.
