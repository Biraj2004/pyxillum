# Pyxillum Windows Executables

This directory contains the latest standalone Windows 64-bit release executables compiled directly from source:

- **`pyxillum-desktop.exe`**: Native desktop GUI application with hardware-accelerated rendering, glowing LED strip canvas, virtual device simulator, and full lighting controls.
- **`pyxillum.exe`**: Native command-line interface tool and hardware verification runner.

## Usage

Double-click `pyxillum-desktop.exe` to run the native application. No installation required.
To query or control devices from the terminal:

```powershell
# Virtual device simulation
.\pyxillum.exe status --sim
.\pyxillum.exe color 255 120 0 --sim
```

## Maintenance Note for Agents and Contributors

Every build and packaging workflow (`cargo xtask package`, `cargo xtask msi`, or `cargo xtask sync-exe`) automatically updates the executables in this directory.
