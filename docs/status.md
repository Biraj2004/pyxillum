# Project Status Tracker

Last updated: 2026-11-12

## Current Phase: Phase 0 (Research & Feasibility)

### Done
- Project workspace initialized with Git and untracked `.gitignore` covering `re-workspace/`, `target/`, and binary artefacts.
- Evaluated local toolchain and environment:
  - Node.js version is `v22.13.0` (npm `10.9.2`).
  - ADB installed (`v1.0.32`).
  - Rust toolchain (`rustup`) package identified via winget (`Rustlang.Rustup`).
  - Checked `rea-agents@latest doctor`: notified of Node engine requirement (>=22.23.4).
  - Checked `unslop` skill: confirmed not installed; intent adopted directly.
- Completed Phase 0 GitHub ecosystem survey across 9 relevant repositories:
  - Rust desktop applications: `rerun-io/rerun`, `spacedriveapp/spacedrive`, `clash-verge-rev/clash-verge-rev`.
  - BLE LED controller and reverse-engineered projects: `xevrion/wallglow`, `monty68/uniled`, `phhusson/ha-banlanx`.
  - Rust BLE implementations with `btleplug`: `andrewdavidmackenzie/meshchat`, `Neewer-Bridge/Neewer-Bridge`, `MnlPhlp/tauri-plugin-blec`.
- Completed SP6xxE / BanlanX / SceneX protocol open-source research and corroborated GATT profile (`0xFFE0` service, `0xFFE1` characteristic, `0x5053` manufacturer ID).
- Produced GUI Stack ADR (`docs/adr/0001-gui-stack.md`) and BLE Library ADR (`docs/adr/0002-ble-library.md`).
- Completed feature inventory and initial matrix (`docs/feature-matrix.md`).
- Conducted collision checks for 3 original candidate names (`Pyxillum`, `Chromavane`, `Lumenstrand`).
- Authored comprehensive Implementation Plan artifact.

### In Progress
- Awaiting user selection of project candidate name.
- Awaiting user decision on GUI framework recommendation (`egui` vs `Tauri v2`).
- Awaiting confirmation of dual MIT/Apache-2.0 licence.

### Blocked
- None currently.

### Unknowns Requiring Hardware
- Precise binary layout of custom DIY pixel-map frames and scene sequences.
- Timing requirements: minimum inter-packet delay before the SP621E MCU drops frames.
- Behaviour on invalid IC driver or colour order codes (rejection vs silent fallback).
- Built-in microphone sensitivity curve and auxiliary audio command format.

### Things Only User Can Do
- Select preferred project name from proposed candidates.
- Confirm GUI framework selection.
- Connect Android phone with `com.spled.scenex` installed via USB and run ADB pull command.
