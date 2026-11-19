# ADR 0003: Native Desktop Front Ends with Shared UniFFI Rust Core

- **Status**: Accepted
- **Date**: 2026-11-19
- **Deciders**: Software Architecture Team

---

## Context and Problem Statement

Pyxillum initially utilised an immediate-mode graphical user interface based on `egui` and `eframe` rendered over OpenGL via `glow`. While effective for rapid prototyping and virtual strip simulation, immediate-mode toolkits exhibit inherent desktop integration limitations:
1. Non-standard window chrome and absence of native OS title bars, Mica/Vibrancy materials, and backdrop blurs.
2. Limited assistive technology integration (screen readers such as Windows Narrator, macOS VoiceOver, and Linux Orca).
3. Non-native widget rendering and keyboard navigation quirks that deviate from platform Human Interface Guidelines.
4. Inability to leverage platform-native audio loopback capture facilities and system media transport integrations seamlessly.

We require a solution that delivers authentic, platform-native desktop experiences on Windows, macOS, and Linux without sacrificing the clean-room, safe Rust foundation of the protocol engine.

---

## Decision Drivers

- **Zero Webview / Zero Electron**: Maintain a lightweight memory footprint (< 30 MB) and avoid bundling Chromium runtimes.
- **Single Source of Truth**: The Rust core must retain 100% ownership of Bluetooth communication, packet serialization, safety gating, device state caching, and preset management.
- **Native UX**:
  - Windows: WinUI 3 (Windows App SDK) in C# with Mica, Segoe Fluent Icons, and Accessibility Automation.
  - macOS: SwiftUI in Swift with AppKit menu bar, SF Symbols, and standard scenes.
  - Linux: GTK 4 + libadwaita in Rust adhering to GNOME HIG.
- **Interoperability**: High-performance, memory-safe FFI binding generation for C# and Swift via Mozilla's UniFFI.

---

## Considered Options

1. **Option 1**: Retain `egui` and expand custom desktop styling widgets.
2. **Option 2**: Rebuild UI with Tauri or Electron (HTML/CSS/JS front ends).
3. **Option 3**: One shared Rust core with three platform-native front ends:
   - WinUI 3 (C#/.NET 8) via UniFFI on Windows.
   - SwiftUI (Swift) via UniFFI on macOS.
   - GTK 4 / libadwaita (Rust) directly linking to `crates/core` on Linux.

---

## Decision Outcome

Chosen option: **Option 3**.

### Architectural Consequences:
- **Positive**:
  - Delivers genuine native look and feel on all three major desktop operating systems.
  - Full screen reader accessibility and native keyboard shortcuts.
  - Clean boundary: UI front ends are strictly dumb presentation layers rendering `AppState` snapshots and dispatching `Action` enums.
  - Rust core remains reusable by the command-line interface (`crates/cli`), test runners, and automated verification suites.
- **Negative / Trade-offs**:
  - Requires maintaining three front-end codebases.
  - Introduces UniFFI proc-macro generation for C# and Swift FFI layers.
  - Build pipeline must handle .NET SDK (Windows) and Swift/Xcode toolchains (macOS).

---

## Compliance and Validation

- `crates/protocol` retains `#![forbid(unsafe_code)]` and zero platform I/O.
- FFI interfaces in `crates/ffi` undergo contract and roundtrip testing.
- Transitional compatibility: `crates/app` (egui) remains functional until Phase 8.
