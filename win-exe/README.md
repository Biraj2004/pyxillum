<div align="center">

<img src="../assets/icon.svg" alt="Pyxillum App Icon" width="96" height="96" />

# Pyxillum Portable Windows Executables

**Ready-to-run 64-bit Windows release binaries compiled directly from source.**

[![Platform: Windows 10 / 11](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20%28x86__64%29-lightgrey?style=flat-square)](../docs/building-windows.md)
[![Profile: Release (ThinLTO)](https://img.shields.io/badge/Profile-Release%20%28ThinLTO%29-brightgreen?style=flat-square)](../Cargo.toml)
[![Subsystem: Windows Native](https://img.shields.io/badge/Subsystem-WinUI%203%20Native-orange?style=flat-square)](../apps/windows/Pyxillum)
[![Installation: Portable Zero--Install](https://img.shields.io/badge/Installation-Portable%20Zero--Install-blue?style=flat-square)](#quickstart)

<p align="center">
  <a href="../README.md"><strong>Project Root</strong></a> &bull;
  <a href="../docs/building-windows.md"><strong>Windows Build Guide</strong></a> &bull;
  <a href="../AGENTS.md"><strong>Technical Guidelines</strong></a>
</p>

</div>

---

## Included Executables

This directory contains standalone, self-contained Windows 64-bit release executables compiled directly from source using the v2 native architecture:

| Binary | Component | Architecture | Purpose |
| :--- | :--- | :--- | :--- |
| **`pyxillum-desktop.exe`** | Desktop Front End | Native WinUI 3 (.NET 8 self-contained) + Rust FFI Core (`pyxillum_ffi.dll`) | Native Windows 11 desktop GUI application featuring Mica backdrop, native navigation view, glowing LED strip monitor canvas, virtual device simulator, and full lighting controls. |
| **`pyxillum.exe`** | Command-Line Interface | Native Rust CLI (ThinLTO, stripped) | Lightweight native terminal tool for scriptable device automation and interactive 7-stage hardware verification runner. |

---

## Quickstart

### Launching the Desktop Application
Double-click `pyxillum-desktop.exe` to run the native application immediately.
- Operates in windowed mode without opening an extraneous console window.
- Defaults to the built-in virtual device simulator when physical Bluetooth hardware is not connected.
- Requires no external runtimes, MSVC redistributables, or installer packages.

### Running the Command-Line Interface
Launch commands against physical hardware or the virtual simulator:

```powershell
# Query device status in virtual simulation mode
.\pyxillum.exe status --sim

# Set solid RGB colour (Amber: 255, 120, 0)
.\pyxillum.exe color 255 120 0 --sim

# Adjust master brightness
.\pyxillum.exe brightness 180 --sim

# Execute the 7-stage hardware verification runner
.\pyxillum.exe hwcheck --sim
```

---

## Maintenance Note for Agents and Contributors

Every build and packaging workflow (`cargo xtask package`, `cargo xtask msi`, or `cargo xtask sync-exe`) automatically updates the executables in this directory.
