<div align="center">

<img src="../assets/icon.svg" alt="Pyxillum App Icon" width="96" height="96" />

# Workspace Dependency Audit and Justification

**Catalogue of all third-party dependencies, rationale, footprint, and licence compatibility.**

[![Audit: Zero Dependency Bloat](https://img.shields.io/badge/Audit-Zero%20Dependency%20Bloat-brightgreen?style=flat-square)](#1-core-dependencies-and-justifications)
[![Licensing: MIT / Apache-2.0 Compatible](https://img.shields.io/badge/Licence%20Compatibility-MIT%20%2F%20Apache--2.0-blue?style=flat-square)](#1-core-dependencies-and-justifications)
[![Unsafe Code: Safe Protocol](https://img.shields.io/badge/Protocol%20Crate-%23!%5Bforbid(unsafe__code)%5D-success?style=flat-square)](#1-core-dependencies-and-justifications)

<p align="center">
  <a href="../README.md"><strong>Project Root</strong></a> &bull;
  <a href="architecture.md"><strong>Architecture</strong></a> &bull;
  <a href="../AGENTS.md"><strong>Agent Guidelines</strong></a>
</p>

</div>

---

All dependencies are audited for minimal footprint, memory safety, permissive or compatible licensing, and absence of redundant bloat.

---

## 1. Core Dependencies and Justifications

| Crate | Target Crates | Version | Licence | Rationale / Justification |
| :--- | :--- | :---: | :--- | :--- |
| **`thiserror`** | `protocol`, `sim`, `transport`, `core` | `^1.0` | MIT OR Apache-2.0 | Derives idiomatic, zero-overhead standard `Error` implementations for domain types without stringly-typed errors. |
| **`anyhow`** | `cli`, `app`, `xtask` | `^1.0` | MIT OR Apache-2.0 | High-ergonomics contextual error handling strictly at binary boundary entry points (`main()`). |
| **`serde`** | All crates | `^1.0` | MIT OR Apache-2.0 | De facto standard for serializing static vectors, state snapshots, and configuration files. |
| **`serde_json`**| `protocol`, `core`, `cli` | `^1.0` | MIT OR Apache-2.0 | Parses test vectors and serializes persistent user presets. |
| **`tokio`** | `transport`, `core`, `cli` | `^1.38` | MIT | Industry-standard async runtime required for `btleplug` and background I/O tasks. Kept isolated from `protocol` and `app` GUI loop. |
| **`async-trait`** | `transport` | `^0.1` | MIT OR Apache-2.0 | Allows dynamic dispatch and async methods on the `Transport` trait. |
| **`btleplug`** | `transport` | `^0.13` | MIT/Apache-2.0 | Cross-platform Bluetooth Low Energy host library (Windows WinRT, macOS CoreBluetooth, Linux BlueZ). Optional backend behind trait. |
| **`eframe` / `egui`** | `app` | `^0.28` | MIT OR Apache-2.0 | Native cross-platform immediate-mode GUI engine. Renders custom 60 FPS pixel strip canvases natively with zero webview bloat. |
| **`clap`** | `cli`, `xtask` | `^4.5` | MIT OR Apache-2.0 | Robust CLI argument parsing with derive macros and clear help messages. |
| **`tracing`** | All crates | `^0.1` | MIT | Structured diagnostics and non-blocking logging. |
| **`tracing-subscriber`** | `cli`, `app` | `^0.3` | MIT | Configures terminal/GUI logging formatters. |
| **`proptest`** | `protocol` (dev) | `^1.5` | MIT OR Apache-2.0 | Property-based randomized fuzzing for packet encoders and notification chunk reassembly. |
| **`directories`**| `core` | `^5.0` | MIT OR Apache-2.0 | Cross-platform standard paths for configuration and preset directories (`AppData/Roaming` on Windows, `Library/Application Support` on macOS, `~/.config` on Linux). |

---

## 2. Pinned Minimum Supported Rust Version (MSRV)

The project pins its Minimum Supported Rust Version (MSRV) to **Rust 1.80.0**. This guarantees compatibility across major Linux distributions, macOS package managers, and standard Windows toolchains.

