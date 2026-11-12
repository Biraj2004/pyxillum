# ADR 0001: Selection of Desktop GUI Stack

## Status
Proposed (Awaiting User Approval)

## Context
The project requires an open-source, cross-platform desktop application (Windows, macOS, Linux) written in Rust to control Bluetooth Low Energy pixel LED controllers. The user interface must support:
- Live virtual LED strip simulation (rendering dozens to hundreds of addressable RGB pixels dynamically at up to 60 FPS).
- Responsive colour selection (colour wheels, hex/RGB inputs, brightness/speed sliders).
- Dynamic pattern selectors and custom sequence editors.
- Non-blocking integration with an asynchronous Bluetooth LE stack.
- Minimal contributor friction, clean dependency hygiene, and native performance.

Five candidate GUI technologies in the Rust ecosystem were evaluated:
1. **Tauri (v2)**
2. **Iced**
3. **egui / eframe**
4. **Slint**
5. **Dioxus**

## Comparative Evaluation Matrix

| Criterion | Tauri v2 | Iced (0.13) | egui / eframe | Slint | Dioxus (0.6) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Native Performance** | High backend; UI in system WebView | High; native wgpu rendering | High; immediate-mode rendering | Very high; custom native renderer | High; WebView or Winit/Blitz |
| **Colour Picker & Canvas Widgets** | Extensive (web ecosystem, HTML5 Canvas) | Basic; custom canvas requires manual geometry | Built-in color picker; excellent custom painter/mesh for LED strips | Moderate; requires manual custom controls | Good via web canvas |
| **Accessibility (a11y)** | High (browser engine ARIA/DOM) | Partial (AccessKit integration in progress) | Basic (AccessKit integration) | Moderate (growing AccessKit support) | High (inherits webview a11y) |
| **Binary Size** | ~10–15 MB | ~15–25 MB | ~12–18 MB | ~8–15 MB | ~15–20 MB |
| **Packaging & Distribution** | Excellent (built-in NSIS, MSI, DMG, deb, AppImage) | Requires manual scripts or cargo-bundle | Requires manual scripts or cargo-bundle | Good via Slint tooling | Requires Dioxus CLI |
| **BLE Integration** | Decoupled IPC (commands/events over channel) | Elm architecture (Subscriptions / Commands) | Decoupled channels (mpsc/crossbeam from Tokio worker) | Callback / Property bindings | Async coroutine hooks |
| **Toolchain & Contributor Friction** | Dual stack (Rust + Node/npm/TS) | 100% Rust (`cargo build` only) | 100% Rust (`cargo build` only) | Rust + `.slint` DSL compiler | Rust + Dioxus CLI/web tooling |
| **LED Strip Simulator Widget** | HTML5 Canvas / WebGL | `iced::widget::canvas` | Direct `egui::Painter` (instant custom shape drawing) | Custom OpenGL/Skia integration | HTML5 Canvas in webview |

## Detailed Analysis of Candidates

### 1. egui / eframe
- **Strengths**: Pure Rust toolchain. Zero Node.js, Webpack, Vite, or web dependencies needed for building. Running `cargo run` works out of the box on Windows, macOS, and Linux. The immediate-mode model allows drawing live LED strips (e.g. 100+ circles with glowing gradients and dynamic colour updates) using `egui::Painter` with negligible overhead. Built-in colour pickers (`color_edit_button_rgb`) and slider widgets are ready immediately.
- **Weaknesses**: Immediate mode repaints every frame if animations are active, requiring careful use of `request_repaint_after()` when idle to prevent excessive CPU consumption. Custom styling requires code rather than CSS.

### 2. Tauri v2
- **Strengths**: High visual fidelity, native OS integration, best-in-class multi-platform packaging (DMG, NSIS, AppImage), and mature web-based colour pickers and timeline editors.
- **Weaknesses**: Requires maintaining a hybrid Rust + TypeScript/Node.js stack. This increases CI complexity and introduces non-Rust dependencies into the workspace.

### 3. Iced
- **Strengths**: Elegant Elm architecture, type-safe state mutations, 100% Rust.
- **Weaknesses**: API churn between minor releases. Canvas widget requires extensive boilerplate for custom LED animations compared to egui.

### 4. Slint
- **Strengths**: Native styling, small binary footprint, declarative syntax.
- **Weaknesses**: Requires learning a proprietary markup language (`.slint`), and third-party widget components for specialized lighting controls are sparse.

### 5. Dioxus
- **Strengths**: React-like ergonomics in Rust.
- **Weaknesses**: Desktop webview target still undergoes frequent API changes; smaller lighting/audio ecosystem.

## Decision & Recommendation
**Recommended Primary Choice: `egui` with `eframe`** (with `Tauri v2` as the secondary alternative if web-based styling is prioritised).

### Rationale:
1. **Single Toolchain Simplicity**: Enables a 100% Rust workspace where `cargo build`, `cargo test`, and `cargo xtask check` compile the entire application without any Node.js, npm, or web bundler prerequisites.
2. **First-Class Virtual Hardware Simulation**: The virtual LED strip preview is a central requirement because physical hardware is unavailable. Immediate-mode drawing via `egui::Painter` renders individual addressable pixels, chasing animations, rainbow sweeps, and audio visualisations in real time with near-zero latency.
3. **Decoupled Architecture**: By isolating BLE I/O to an asynchronous Tokio worker in `crates/transport` and exchanging typed messages over unbounded crossbeam/mpsc channels, `egui` remains completely non-blocking.

