# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - Complete Windows 11 Fluent UI Redesign (Phases 1-9)

### Added
- **Phase 1: Design Tokens Engine (`crates/app/src/ui/theme/`)**:
  - `palette.rs`: Pure mathematical OKLCH & Oklab perceptual colour space engine, linear sRGB conversion, and WCAG 2.1 relative luminance and contrast ratio calculation.
  - `tokens.rs`: Layered elevation surfaces (`base`, `sidebar`, `card`, `raised_hover`, `dialog`), 4px spacing grid, concentric radii (`card` 12px, `control` 8px, `chip` 6px, `pill` 999px), subtle translucent borders, and semantic palette (`success`, `warning`, `danger`, `info`).
  - Automated tests verifying WCAG AA compliance (4.5:1 body text, 3:1 large text/boundaries).
- **Phase 2: Windows 11 Typography Scale (`crates/app/src/ui/typography/`)**:
  - `scale.rs`: Segoe UI Variable integration loading from `%WINDIR%\Fonts` with embedded Inter (OFL) fallback.
  - Fluid type scale: caption (12px), body (14px), body-strong (14px semibold), subtitle (16px), title (20px), display (28px+), and dial readouts (40px+).
  - Reserved bold width layout protection for sidebar navigation and segmented controls.
  - Tabular numeric readouts and text-box trim optical centering.
- **Phase 3: Unified Fluent Vector Iconography (`crates/app/src/ui/icons/`)**:
  - Vector painter implementing 33 Microsoft Fluent System Icons on strict 16px, 20px, and 24px physical grids.
  - Sub-pixel snapping (`snap_rect_to_pixels`) preventing bilinear filtering blur across Windows display scaling factors (100%, 125%, 150%, 200%).
  - Zero-Unicode-emoji audit test (`test_zero_unicode_symbol_icons_in_source`).
- **Phase 4: Windows 11 Native Integration (`crates/app/src/platform/windows.rs`)**:
  - Windows High Contrast theme detection (`SPI_GETHIGHCONTRAST`).
  - Windows Visual Effects animation setting query (`SPI_GETCLIENTAREAANIMATION`).
  - System accent colour extraction via `DwmGetColorizationColor`.
  - Native Windows title bar preservation with Snap Layouts support.
- **Phase 5: Fluent Component Library (`crates/app/src/ui/components/`)**:
  - `card.rs`: Standardised elevation surfaces with top highlight and subtle border stroke.
  - `button.rs`: Standardised primary, secondary, subtle, and destructive buttons with optical icon centering and tactile 1px depression.
  - `segmented.rs`: Segmented control and 2D toggle tile grid with reserved bold width.
  - `slider.rs`: Full-width 4px track sliders with expanding hover thumbs, mouse wheel stepping, and tabular readouts.
  - `infobar.rs`: Windows 11 style notification banner with status icons and action links.
  - `toggle.rs`: Fluent toggle switch with spring elastic overshoot.
  - `dial.rs`: 270-degree radial ArcDial with angular dragging and centred readout.
  - `search_box.rs`: Search input with vector icon and instant clear button.
  - `list_row.rs`: Virtualised row widget with animated left accent selection indicator.
  - `chip_group.rs`: Horizontal category filter chips.
  - `dialog.rs`: Modal ContentDialog with backdrop scrim, drop shadow, and error shake animation.
  - `status_chip.rs`: Status indicators, device badges, and centred empty states.
- **Phase 6: Responsive Layout & Adaptive Virtual LED Monitor**:
  - Responsive breakpoints: Compact (<1100px) 56px icon rail, Regular (1100-1500px) 240px sidebar, Wide (>1500px) 1280px capped stage.
  - `crates/app/src/widgets/neon_strip.rs`: Adaptive pixel hardware monitor supporting 30 to 255 LEDs without overflow, multi-layer concentric glow optics, and off-state dark diode rendering. Width matches stage card width.
  - State reconciliation: Disconnected header shows "No Controller Connected (Preview Mode)" with inline "Scan & Connect" InfoBar.
- **Phase 7: Page-by-Page Screen Redesigns (`crates/app/src/screens/`)**:
  - `device_hub.rs`: Single diagnostic InfoBar with `ms-settings:bluetooth` link, scanning spinner banner, empty state, honest capability chips, and 4-bar RSSI meter.
  - `dynamic_mode.rs`: ArcDial preset selector, full-width brightness & speed sliders, playback controls, and split 146-effect preset drawer.
  - `solid_mode.rs`: HSV colour disc, hex input with clipboard copy, RGB channel sliders, 2700K-6500K CCT slider, curated swatches, and on-chip hardware effect matching.
  - `music_mode.rs`: Segmented input source selector, Windows Media Session (GSMTC) now playing banner, 24-band peak-hold audio spectrum visualizer, and 18-mode responsive matrix.
  - `diy_mode.rs`: 2-32 node sequencer, non-breaking bounds-checked node operations, node inspector, 8-style 2D toggle tile grid, and 60 FPS real-time chromatic preview.
  - `settings_modal.rs`: ContentDialog with length slider, preset pills, chip order selector, and non-volatile EEPROM hardware safety gate.
- **Phase 8: Design, Motion & Animation Hacks**:
  - Perceptual Oklab gradient interpolation for UI surfaces (`interpolate_oklab`).
  - Concentric radii audit: `inner_radius = max(outer_radius - padding, 4.0)` with automated tests.
  - Optical alignment: -0.5px vertical icon offset to align with font x-height.
  - Micro-interactions: 1px button depression, toggle elastic overshoot, and animated list row selection indicator.
  - Error shake: +/-6px decaying sinusoidal shake on unauthorized actions.
  - Windows accessibility respect via `is_client_animation_enabled()`.
- **Phase 9: Performance Optimisation & Executable Staging**:
  - Intelligent repaint pacing: 60 FPS continuous redraws only during active visualizers; idle pacing drops CPU usage to < 1%.
  - Production binaries compiled and staged into `win-exe/`: `pyxillum-desktop.exe` (5.65 MB) and `pyxillum.exe` (1.76 MB).

### New Files Added
- `crates/app/src/ui/mod.rs`
- `crates/app/src/ui/theme/mod.rs`
- `crates/app/src/ui/theme/palette.rs`
- `crates/app/src/ui/theme/tokens.rs`
- `crates/app/src/ui/typography/mod.rs`
- `crates/app/src/ui/typography/scale.rs`
- `crates/app/src/ui/icons/mod.rs`
- `crates/app/src/ui/icons/painter.rs`
- `crates/app/src/ui/components/mod.rs`
- `crates/app/src/ui/components/card.rs`
- `crates/app/src/ui/components/button.rs`
- `crates/app/src/ui/components/segmented.rs`
- `crates/app/src/ui/components/slider.rs`
- `crates/app/src/ui/components/infobar.rs`
- `crates/app/src/ui/components/toggle.rs`
- `crates/app/src/ui/components/dial.rs`
- `crates/app/src/ui/components/search_box.rs`
- `crates/app/src/ui/components/list_row.rs`
- `crates/app/src/ui/components/chip_group.rs`
- `crates/app/src/ui/components/dialog.rs`
- `crates/app/src/ui/components/status_chip.rs`
- `crates/app/src/platform/mod.rs`
- `crates/app/src/platform/windows.rs`
- `docs/design-system.md`

### QA Verification Checklist
- [x] **Native Windows 11 Frame**: Standard title bar, minimize, maximize/Snap Layouts, and close buttons work without visual glitching.
- [x] **Autofit & Centering**: Launches centered at 80%x80% work area; respects OS-level min window size (1000x650); restores saved position on restart.
- [x] **Responsive Adaptation**: Rail collapses cleanly at <1100px; expands at 1100-1500px; caps content width at >1500px.
- [x] **LED Strip Monitor**: Smoothly renders any length from 30 to 255 pixels; matches stage card width; concentric glow rings eliminate eye strain.
- [x] **State Honesty**: Physical hardware models explicitly labelled "implemented, untested on hardware"; disconnected header states "Preview Mode".
- [x] **Single BLE InfoBar**: BLE error messages appear once with actionable link to Windows Settings (`ms-settings:bluetooth`).
- [x] **DIY Sequencer Robustness**: Adding, removing, duplicating, and reversing nodes preserves at least 2 nodes and never causes panic.
- [x] **Full-Width Sliders**: Master brightness, speed, and sensitivity sliders span full card width with tabular units.
- [x] **EEPROM Safety Gate**: Applying configuration is locked until user acknowledges non-volatile mutation; attempts trigger dialog shake.
- [x] **Near-Zero Idle CPU**: Static screens sleep frame pacing up to 100ms, consuming near-zero CPU when idle.

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

