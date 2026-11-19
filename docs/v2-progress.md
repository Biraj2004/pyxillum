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
| **2** | **Audio & FFI Surface** | `crates/audio`, `crates/ffi` | **COMPLETED** | UniFFI proc-macros, C#/Swift bindings, 77 workspace tests passing |
| **3** | **Design Source of Truth** | `design/`, `tools/` | **COMPLETED** | Tokens, strings.toml, icon map, generator scripts, 79 workspace tests passing |
| **4** | **Windows Native Front End** | `apps/windows` (WinUI 3) | **COMPLETED** | C# .NET 8, WinUI 3, Mica, NavigationView, 6 views, 79 tests passing |
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

---

## Phase 2 Execution Summary
- **Audio Capture & Spectrum Analysis Subsystem (`crates/audio`)**:
  - Defined `AudioSource` trait and `AudioSourceKind` enum (`SystemLoopback`, `Microphone`).
  - Implemented 24-band ISO third-octave spectrum processor with fast attack, exponential decay ballistics, and normalization to `0.0..=1.0`.
  - Implemented native Windows WASAPI loopback and microphone endpoint capture via `windows` crate (`IMMDeviceEnumerator`, `IAudioMeterInformation`).
  - Implemented `SimulatedAudioSource` synthesizing dynamic 128 BPM rhythmic acoustic patterns for deterministic offline testing.
  - Scaffolded `LinuxAudioSource` (PulseAudio/PipeWire) and `MacOsAudioSource` (CoreAudio) cleanly gated by `target_os` (marked unverified on physical hardware).
  - Wired real-time audio capture into `Engine::start_audio`, `stop_audio`, and `audio_spectrum` in `crates/core`.
- **UniFFI Language Bindings Surface (`crates/ffi`)**:
  - Exposed `EngineFfi`, `ActionFfi`, `AppStateFfi`, `StripFrameFfi`, `StateListenerCallback`, `PlatformServicesCallback`, and `FfiError` via UniFFI proc-macro mode.
  - Provided `create_engine` factory supporting both simulated and hardware BLE execution backends.
  - Configured `uniffi-bindgen` CLI tool to generate native Swift bindings into `apps/macos/PyxillumCore/Generated/` (`pyxillum_ffi.swift`, `pyxillum_ffiFFI.h`, `pyxillum_ffiFFI.modulemap`).
  - Implemented idiomatic C# (.NET 8 / C# 12) binding declarations in `apps/windows/Pyxillum.Core/Generated/PyxillumCore.cs` for WinUI 3.
  - Implemented end-to-end FFI contract and lifecycle test verifying snapshot queries, frame buffers, action dispatching, and audio controls across the FFI boundary.
- **Repository Automation (`xtask`)**:
  - Added `cargo xtask generate-bindings` for automated code generation of Swift and C# bindings.
  - Added `cargo xtask ffi-windows`, `cargo xtask ffi-macos`, and `cargo xtask ffi-linux` for targeting platform native dynamic libraries.
- **Verification & Quality Gate**:
  - Total workspace tests increased to 77 tests passing (100% pass rate).
  - Formatting (`cargo fmt --check`): PASSED.
  - Clippy (`-D warnings`): PASSED (zero warnings across all workspace targets).

---

## Phase 3 Execution Summary
- **Unified Design Tokens (`design/tokens.json`)**:
  - Established 4px spacing scale (`space_0` through `space_16`), corner radii (`none` through `pill`), typography scale (`caption` through `dial_readout`), motion durations and bezier curves, and semantic colors in OKLCH with hex fallbacks (`success`, `warning`, `danger`, `info`, `led_glow`).
  - Documented that platform host themes supply dynamic surfaces, text, and primary accents.
- **Localisation & Strings Source of Truth (`design/strings.toml`)**:
  - All user-facing strings authored in British/Indian English (`colour`, `optimised`, `initialised`, `behaviour`).
  - Mapped all `ErrorKey::message_key()` entries under `[errors]`.
  - Added full string catalogues for `[navigation]`, `[common]`, `[devices]`, `[dynamic]`, `[solid]`, `[music]`, `[diy]`, `[settings]`.
- **System Icon Cross-Platform Mapping (`design/icons.toml`)**:
  - Mapped core `IconId` definitions to Segoe Fluent glyph hex codes (`\uXXXX`), Apple SF Symbols names, and GNOME Adwaita symbolic icon identifiers.
- **Page-by-Page Specifications (`design/spec/`)**:
  - Created detailed layout specifications and measured ASCII wireframes for `devices.md`, `dynamic.md`, `solid.md`, `music.md`, `diy.md`, and `settings.md`.
  - Resolved historical UI defects: removed duplicated connection badges, enlarged dial readouts, eliminated cramped scroll containers, and fixed modal scrim dimming.
- **Automated Token Generation Tools (`tools/generate_tokens.py`)**:
  - Reads `design/tokens.json` and automatically generates `apps/windows/Pyxillum/Themes/Tokens.xaml` (WinUI 3 ResourceDictionary), `apps/macos/PyxillumCore/Sources/Tokens.swift` (SwiftUI constants), and `apps/linux/data/tokens.css` (GTK4 CSS variables).
- **Automated Verification**:
  - Implemented `crates/core/tests/strings_toml.rs` validating that all `ErrorKey` message keys exist in `design/strings.toml` and all mandatory sections are present.
  - Workspace test count increased to 79 tests passing (100% pass rate).
  - Version updated to `1.0.5` across `Cargo.toml`.

---

## Phase 4 Execution Summary
- **Windows Solution & Project Architecture (`apps/windows/`)**:
  - `Pyxillum.sln`: Visual Studio solution containing `Pyxillum.Core` and `Pyxillum`.
  - `Pyxillum.Core.csproj`: .NET 8 class library consuming generated UniFFI C# bindings (`PyxillumCore.cs`) and staging `pyxillum_ffi.dll`.
  - `Pyxillum.csproj`: WinUI 3 desktop application project referencing Windows App SDK and CommunityToolkit.Mvvm with unpackaged runtime configuration (`WindowsPackageType=None`).
  - `app.manifest`: Per-Monitor V2 High-DPI manifest.
- **Application Shell & Window Hosting (`App.xaml`, `MainWindow.xaml`)**:
  - Mica backdrop (`SystemBackdrop = new MicaBackdrop()`).
  - Native title bar with custom drag region and Segoe Fluent iconography (`ExtendsContentIntoTitleBar = true`).
  - NavigationView with auto-collapse to rail below 1100px and 240px expanded pane width.
  - Single connection-status location in the global header, alongside master power toggle and brightness slider.
  - Top notification InfoBars for Bluetooth Low Energy availability (with button linking to Windows Settings) and dismissible operational error states.
  - Window geometry: Defaulting to 80% x 80% of the active monitor's work area, centered.
- **Zero-CPU 60 FPS LED Strip Virtual Monitor (`Controls/LedStripMonitor.cs`)**:
  - Canvas control subscribing to `CompositionTarget.Rendering` only when `Visibility == Visibility.Visible`.
  - Throttled to 60 FPS (16.6ms frame intervals) pulling `engine.frame()`. Unsubscribes from rendering loop on unload to guarantee zero idle CPU.
- **Dedicated View Implementation (`Views/`)**:
  - `DevicesPage`: Device discovery hub with scanning ProgressRing, empty state guidance card, and controller list.
  - `DynamicPage`: 146+ dynamic animation effects with search filter, velocity/speed slider, luminescence slider, auto-cycle toggle, and effect cards.
  - `SolidPage`: Continuous chromatic hue disc, Kelvin colour temperature slider (2,700 K to 6,500 K), individual RGB channel sliders, hex code input, and curated architectural swatch buttons.
  - `MusicPage`: Audio input pipeline selector (WASAPI loopback, Internal Mic, Aux Mic), sensor sensitivity slider, 24-band spectrum visualizer canvas, and 18 sound tracking algorithm selector.
  - `DiyPage`: 14-step timeline with keyframe node discs, color palette, sequence dynamics (speed, forward/reverse direction), and preview playback.
  - `SettingsPage`: Non-volatile EEPROM hardware configuration with explicit cautionary warning banner, confirmation checkbox (`--i-understand-this-changes-the-device`), pixel count slider/stepper, and RGB calibration sequence selector.
- **Verification & Quality Gate**:
  - `cargo xtask check` PASSED (formatting clean, zero clippy warnings, 79 workspace tests passing).


