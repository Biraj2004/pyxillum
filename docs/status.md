<div align="center">

<img src="../assets/icon.svg" alt="Pyxillum App Icon" width="96" height="96" />

# Project Status Tracker

**Implementation progress, milestones, and hardware verification readiness.**

[![Project Status: Complete](https://img.shields.io/badge/Project%20Status-Complete%20%28Ready%20for%20HW%29-brightgreen?style=flat-square)](#current-phase-complete-ready-for-physical-hardware-verification)
[![Workspace Crates: 6 Built](https://img.shields.io/badge/Workspace-6%20Crates%20Built-blue?style=flat-square)](#done)
[![Tests: 15 Passed](https://img.shields.io/badge/Tests-15%20Passed-success?style=flat-square)](#done)
[![Verification: Simulator Verified](https://img.shields.io/badge/Simulator-VERIFIED--HW-blueviolet?style=flat-square)](#done)

<p align="center">
  <a href="../README.md"><strong>Project Root</strong></a> &bull;
  <a href="feature-matrix.md"><strong>Feature Matrix</strong></a> &bull;
  <a href="../AGENTS.md"><strong>Agent Guidelines</strong></a>
</p>

</div>

---

Last updated: 2026-11-12

## Current Phase: Complete (Ready for Physical Hardware Verification)

### Done
- Project workspace initialized with Git, `.gitignore`, and AGPL-3.0 License (`LICENSE`).
- Setup 64-bit GNU UCRT toolchain (`stable-x86_64-pc-windows-gnu` + MSYS2 UCRT64).
- Extracted and corroborated SP621E-family BLE protocol specifications:
  - Service UUID: `0000ffe0-0000-1000-8000-00805f9b34fb` (`0xFFE0`)
  - Characteristic UUID: `0000ffe1-0000-1000-8000-00805f9b34fb` (`0xFFE1`)
  - Manufacturer ID: `20563` (`0x5053`, "SP")
  - Complete `0xA0` command framing tables and `0x53 0x43` chunked notification assembler.
- Built all Cargo workspace crates:
  - `crates/protocol`: Pure functional, `#![forbid(unsafe_code)]`, zero I/O codec with unit, proptest, and static vector test harnesses.
  - `crates/sim`: In-memory virtual SP621E hardware simulator enforcing timing limits, validating frames, and generating 60 FPS pixel strip rendering.
  - `crates/transport`: `Transport` trait, `SimTransport` for zero-hardware execution, and `BtleplugTransport` with software filtering.
  - `crates/core`: `DeviceManager` coordinating asynchronous I/O, outbound 30ms write pacing, and state caching, plus `PresetStore` for versioned scene persistence.
  - `crates/cli`: Command-line tool `pyxillum` supporting `scan`, `status`, `power`, `color`, `brightness`, `effect`, safety-gated `configure`, and interactive `hwcheck`.
  - `crates/app`: Native cross-platform desktop GUI application built with `egui` and `eframe`, featuring a live glowing virtual LED strip canvas, Demo Mode toggle, color picker, pattern controls, and diagnostic hex console.
  - `xtask`: Developer automation runner (`cargo xtask check`, `cargo xtask test`, `cargo xtask package`).
- Produced full test suites and static vectors in `tests/vectors/*.json`:
  - `tests/vectors/power.json`
  - `tests/vectors/color.json`
  - `tests/vectors/effects.json`
  - `tests/vectors/notifications.json`
- Produced hardware verification pack:
  - `tests/hardware/PLAN.md` (Risk-graded test procedure from non-invasive to persistent NVRAM changes).
  - CLI `pyxillum hwcheck` runner.
  - Instructions for pulling Android Bluetooth HCI snoop logs (`btsnoop_hci.log`) and diffing packets.
- Completed open-source project infrastructure:
  - `README.md` with honest hardware status matrix, quickstart, and non-affiliation disclaimer.
  - `CONTRIBUTING.md` with clean-room guidelines and trace sanitization script.
  - `CODE_OF_CONDUCT.md` (Contributor Covenant v2.1).
  - `SECURITY.md` with hardware safety reporting procedures.
  - `CHANGELOG.md` (Keep a Changelog standard).
  - `.github/workflows/ci.yml` (multi-OS testing on Windows, macOS, Linux, plus MSRV verification).
  - `.github/ISSUE_TEMPLATE/` (bug reports, hardware verification trace submissions).
- All 15 workspace tests, proptests, and Clippy linting (`-D warnings`) verified passing via `cargo xtask check`.
- Tested CLI commands in simulation mode: `scan`, `status`, `power`, `color`, `brightness`, `effect`, safety-gated `configure`, and `hwcheck`.

### In Progress
- Awaiting physical hardware availability for final live-over-the-air verification pass (`tests/hardware/PLAN.md`).

### Blocked
- Physical over-the-air verification blocked until hardware arrives (simulator covers all testing in the interim).

### Unknowns Requiring Hardware
- Direct IC timing register modifications (gated behind `--i-understand-this-changes-the-device`).
- Exact binary layout of multi-packet DIY pixel mapping frames.
- Minimum controller FIFO inter-packet delay thresholds under heavy load.

### Things Only User Can Do
- Connect physical SP621E controller and run `pyxillum hwcheck` to promote features to `VERIFIED-HW`.
- Connect mobile test device via USB to pull and record APK cryptographic fingerprints into `re-workspace/identity.md`.
