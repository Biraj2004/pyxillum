<div align="center">

<img src="../assets/icon.svg" alt="Pyxillum App Icon" width="96" height="96" />

# Pyxillum Portable Windows Executables

**Ready-to-run 64-bit Windows release binaries compiled directly from source.**

[![Platform: Windows 10 / 11](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20%28x86__64%29-lightgrey?style=flat-square)](../docs/building-windows.md)
[![Profile: Release (ThinLTO)](https://img.shields.io/badge/Profile-Release%20%28ThinLTO%29-brightgreen?style=flat-square)](../Cargo.toml)
[![Subsystem: Windows Native](https://img.shields.io/badge/Subsystem-Native%20Windowed-orange?style=flat-square)](../crates/app/src/main.rs)
[![Installation: Portable Zero--Install](https://img.shields.io/badge/Installation-Portable%20Zero--Install-blue?style=flat-square)](#quickstart)

<p align="center">
  <a href="../README.md"><strong>Project Root</strong></a> &bull;
  <a href="../docs/building-windows.md"><strong>Windows Build Guide</strong></a> &bull;
  <a href="../AGENTS.md"><strong>Technical Guidelines</strong></a>
</p>

</div>

---

## Included Executables

This directory contains standalone, statically linked Windows 64-bit release executables compiled with full platform optimisations (ThinLTO, symbol stripping, Per-Monitor v2 High-DPI manifest, Common Controls v6, and embedded application icon):

| Binary | Size | Purpose |
| :--- | :---: | :--- |
| **`pyxillum-desktop.exe`** | ~4.7 MB | Native desktop GUI application with hardware-accelerated OpenGL rendering, glowing LED strip canvas, virtual device simulator, and full lighting controls. |
| **`pyxillum.exe`** | ~1.5 MB | Native command-line interface tool and interactive 7-stage hardware verification runner. |

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
