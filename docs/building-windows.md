# Building and Optimising Pyxillum on Windows

This guide provides technical instructions for compiling, optimising, packaging, and testing the native Windows desktop application (`pyxillum-desktop.exe`) and command-line tool (`pyxillum.exe`).

---

## 1. Why Native Rust on Windows?

Many cross-platform desktop applications rely on frameworks like Electron, Chromium Embedded Framework, or heavyweight runtimes that bundle an entire web browser, consuming 150–400 MB of RAM on startup and resulting in 100+ MB installers.

Pyxillum adopts a lightweight, native approach:
- **Direct Native Execution**: Compiles to native x86_64 machine code with zero external runtime dependencies.
- **Hardware-Accelerated UI**: Uses `eframe` and `egui` rendered over OpenGL/WGL via `glow`, achieving 60 FPS strip animations and sub-millisecond frame updates.
- **Direct WinRT Bluetooth Integration**: Uses native Windows Runtime Bluetooth APIs (`Windows.Devices.Bluetooth`, `Windows.Devices.Bluetooth.GenericAttributeProfile`) via `btleplug`.
- **Minimal Footprint**: The fully optimised release GUI executable is only **4.69 MB**, launches in under 50 milliseconds, and uses less than 25 MB of resident RAM.
- **Zero-Hardware Simulation**: A complete virtual SP621E controller is built into the binary, allowing full UI and CLI testing even when physical hardware is disconnected.

---

## 2. Prerequisites

### Supported Toolchains

Pyxillum supports both standard Windows Rust targets:

1. **MinGW GNU (UCRT64)** (Default for standalone GCC environments):
   - Target: `x86_64-pc-windows-gnu`
   - Toolchain: MSYS2 UCRT64 (`gcc`, `windres`)
2. **Microsoft Visual C++ (MSVC)**:
   - Target: `x86_64-pc-windows-msvc`
   - Toolchain: Visual Studio 2022 / Build Tools (`link.exe`, `rc.exe`)

### System Requirements

- **Operating System**: Windows 10 (Version 1809 or higher) or Windows 11.
- **Bluetooth Adapter**: Bluetooth 4.0 LE (Low Energy) compatible radio or USB dongle.
- **Rust Toolchain**: Rust 1.80.0 or later (MSRV). Check with `rustc --version`.

---

## 3. Windows-Specific Optimisations

Pyxillum includes dedicated Windows platform enhancements in both its codebase and Cargo configuration:

### A. Windowed Subsystem Suppression
By default, Rust desktop applications compile under the `console` subsystem, which opens an empty command prompt window when the executable is launched. Pyxillum eliminates this in release mode using conditional subsystem configuration:

```rust
// crates/app/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

In `debug` mode, console logging remains active for developers; in `release` mode, the binary launches purely as a native windowed application without command prompt flashing.

### B. High-DPI Per-Monitor V2 Awareness & Common Controls v6
To prevent blurry text and bitmap scaling on modern high-DPI displays (such as 125%, 150%, or 4K monitors), Pyxillum embeds a Windows application manifest via `crates/app/build.rs`:

- **Per-Monitor V2 DPI Awareness**: Instructs Windows Desktop Window Manager (DWM) to render fonts at physical display pixel resolution.
- **Common Controls v6**: Enables modern Windows visual styling.
- **UTF-8 Active Code Page**: Forces Windows API calls to use native UTF-8 encoding.
- **Long Path Awareness**: Removes the legacy 260-character `MAX_PATH` limitation.

The manifest (`crates/app/windows/pyxillum.manifest`) and resource script (`crates/app/windows/pyxillum.rc`) are automatically compiled during build using `windres` (GNU) or `rc.exe` (MSVC).

### C. Link-Time Optimisation & Symbol Stripping
The root `Cargo.toml` is tuned for maximum native performance and minimal binary footprint:

```toml
[profile.release]
opt-level = 3          # Maximum LLVM optimisations
lto = "thin"           # Thin Link-Time Optimisation across all crate boundaries
codegen-units = 1      # Single code generation unit for superior inlining
panic = "abort"        # Drops C++ unwinding tables and landing pads
strip = true           # Automatically strips debug symbols and relocations
```

#### Binary Size Comparison

| Build Profile | Binary Name | Executable Size |
| :--- | :--- | :--- |
| **Debug (Unoptimised)** | `pyxillum-desktop.exe` | ~158.0 MB |
| **Release (Optimised + ThinLTO + Stripped)** | `pyxillum-desktop.exe` | **4.69 MB** |
| **Release (Optimised + ThinLTO + Stripped)** | `pyxillum.exe` (CLI) | **1.47 MB** |

---

## 4. Step-by-Step Build Instructions

### Step 1: Ensure Toolchain Environment is Configured

When building with the MinGW UCRT64 toolchain, verify that your MSYS2 `bin` directory and Cargo binaries are on your PATH:

```powershell
$env:Path = "C:\msys64\ucrt64\bin;C:\Users\$env:USERNAME\.cargo\bin;$env:Path"
```

Verify your active toolchain:

```powershell
rustc -V
cargo -V
```

### Step 2: Compile the Debug Desktop Application

To compile for active development and local testing:

```powershell
cargo build -p pyxillum-app
```

The resulting executable will be placed in `target/debug/pyxillum-desktop.exe`.

### Step 3: Compile the Optimised Release Binaries

To produce the stripped, ThinLTO-optimised release executables:

```powershell
cargo build --release -p pyxillum-app -p pyxillum-cli
```

The executables will be generated in `target/release/pyxillum-desktop.exe` and `target/release/pyxillum.exe`.

### Step 4: Automated Packaging via `xtask`

Pyxillum includes an automated developer workflow tool (`xtask`). To compile and package release binaries into a self-contained distribution folder:

```powershell
cargo xtask package
```

This command:
1. Builds all workspace targets in `--release` profile.
2. Creates the `dist/` directory.
3. Stages `pyxillum-desktop.exe` and `pyxillum.exe`.
4. Copies `README.md` and `LICENSE`.
5. Prints the size of each packaged artifact.

### Step 5: Building Windows MSI Installer via WiX (`cargo xtask msi`)

To compile a native Windows Installer (`.msi`) package with branded banners, desktop/start-menu shortcuts, and Add/Remove Programs icons:

```powershell
cargo xtask msi
```

If WiX Toolset is installed (`winget install WiX.Toolset`), this builds `dist/pyxillum-setup-0.1.0.msi` incorporating:
- WiX definition: `crates/app/windows/pyxillum.wxs`
- Application Icon: `crates/app/windows/pyxillum.ico`
- Wizard Header Banner: `crates/app/windows/WixUIBannerBmp.bmp` (493 x 58 px)
- Wizard Dialog Banner: `crates/app/windows/WixUIDialogBmp.bmp` (493 x 312 px)

---

## 5. Icon Assets & Visual Identity

Pyxillum uses a clean, geometric, unslop icon mark representing interconnected addressable pixel nodes:

| Asset Path | Format / Size | Purpose |
| :--- | :--- | :--- |
| `assets/icon.svg` | Vector SVG (512x512) | Scalable master vector mark |
| `assets/icon-mono.svg` | Vector SVG (512x512) | High-contrast monochrome variant |
| `assets/icons/icon-*.png` | PNG (16, 24, 32, 48, 64, 128, 256, 512) | Standard desktop, taskbar, and app store rasters |
| `assets/pyxillum.ico` | Multi-layer ICO (16 to 256 px) | Embedded Windows PE application icon |
| `assets/wix/WixUIBannerBmp.bmp` | 24-bit RGB BMP (493x58) | WiX MSI installer top header banner |
| `assets/wix/WixUIDialogBmp.bmp` | 24-bit RGB BMP (493x312) | WiX MSI installer welcome/exit dialog sidebar |

To regenerate all raster icons and MSI assets from the master SVG:

```powershell
python scripts/generate_assets.py
```

---

## 6. Testing and Validation on Windows

### A. Running the Desktop Application

To launch the native GUI directly:

```powershell
# Run via Cargo
cargo run --release -p pyxillum-app

# Or execute the packaged binary directly
.\dist\pyxillum-desktop.exe
```

#### What to Verify in the GUI:
1. **Window Rendering**: The dark-themed GUI appears with the title `Pyxillum - Pixel LED Controller`.
2. **Demo Mode (Simulator)**: By default, the application connects to the built-in virtual device (`SIM:00:11:22:33:44:55`).
3. **Strip Visualiser**: The glowing 30-pixel virtual LED strip responds to colour, brightness, and pattern changes in real time.
4. **Interactive Controls**:
   - Change colour using the colour picker (e.g. RGB(0, 200, 255)).
   - Adjust the brightness slider (0–255).
   - Adjust the speed slider (1–10).
   - Select pattern presets (Rainbow Sweep, Fire Glow, Meteor Trail).
   - Toggle power off and on.
5. **Command Log**: The diagnostic hex log at the bottom displays every outbound frame (`0xA0 ...`) and assembled notification (`0x53 0x43 ...`).

### B. Testing the Command-Line Interface (`pyxillum.exe`)

You can test every feature of the controller from PowerShell using the built-in virtual simulator (`--sim`):

```powershell
# Scan for controllers (discovers the virtual simulator)
.\dist\pyxillum.exe scan --sim

# Query status
.\dist\pyxillum.exe status --sim

# Set solid colour (e.g. Amber: RGB 255, 120, 0)
.\dist\pyxillum.exe color 255 120 0 --sim

# Adjust brightness (0-255)
.\dist\pyxillum.exe brightness 180 --sim

# Select effect pattern (code 1-218)
.\dist\pyxillum.exe effect 5 --speed 8 --sim

# Toggle power
.\dist\pyxillum.exe power off --sim
.\dist\pyxillum.exe power on --sim
```

### C. Testing Hardware Safety Gates

Persistent configuration parameters (such as colour sequence and strip length) write to non-volatile controller memory. Verify that safety gating functions correctly:

```powershell
# Attempting without confirmation flag: Operation aborted
.\dist\pyxillum.exe configure --length 60 --sim

# Providing the required confirmation: Operation succeeds
.\dist\pyxillum.exe configure --length 60 --i-understand-this-changes-the-device --sim
```

### D. Running the Interactive Hardware Verification Checklist

```powershell
.\dist\pyxillum.exe hwcheck --sim
```

This runs test sequence `HW-01` through `HW-07`, confirms each visual state, and generates a structured verification report.

---

## 6. Troubleshooting Common Windows Issues

### Issue 1: `linker link.exe not found`
- **Cause**: The default MSVC target (`x86_64-pc-windows-msvc`) was selected, but Visual Studio C++ build tools are not installed.
- **Solution**: Switch to the GNU toolchain:
  ```powershell
  rustup default stable-x86_64-pc-windows-gnu
  ```

### Issue 2: `Invalid bfd target` during MinGW compilation
- **Cause**: A 32-bit `C:\MinGW\bin\dlltool.exe` is present earlier in your PATH than the 64-bit MSYS2 UCRT64 toolchain.
- **Solution**: Prepend the 64-bit MSYS2 UCRT64 directory to your PATH:
  ```powershell
  $env:Path = "C:\msys64\ucrt64\bin;$env:Path"
  ```

### Issue 3: Bluetooth Adapter Not Discovered
- **Cause**: Windows Bluetooth radio is switched off, or Bluetooth permissions are restricted.
- **Solution**:
  1. Open Windows **Settings** > **Bluetooth & devices** and ensure Bluetooth is toggled **On**.
  2. Verify that your Bluetooth adapter supports Bluetooth Low Energy (4.0+).
  3. Close any active companion mobile apps (the SP621E only permits one active Bluetooth connection).

