# Feature Matrix: Original Companion App vs Target Implementation

Last updated: 2026-11-12

Every protocol fact and feature capability is assigned an explicit confidence status:
- **`OBSERVED-STATIC`**: Extracted directly from decompiled application bytecode or assets.
- **`CORROBORATED`**: Confirmed independently across multiple open-source implementations or technical references.
- **`VERIFIED-HW`**: Verified against physical controller hardware (initially none, hardware unavailable).
- **`INFERRED`**: Logically deduced from related code paths or standard BLE GATT conventions.
- **`UNKNOWN`**: Unverified; requires live capture or physical device inspection.

All planned implementations remain labelled **"implemented, untested on hardware"** until confirmed on physical hardware.

---

## Detailed Capability Matrix

| Feature | Original App Evidence | Protocol Command / Wire Structure | Confidence Status | Implementation Status |
| :--- | :--- | :--- | :--- | :--- |
| **Device Discovery & Scan** | BLE advertisement with name prefix `SP...` or manufacturer ID | Manufacturer data: `0x5053` (`20563`), Service UUID: `0xFFE0` | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Connection & GATT Setup** | Service `0xFFE0`, Char `0xFFE1` | Connect GATT, MTU negotiation, subscribe to `0xFFE1` notifications | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Power Toggle (On / Off)** | UI switch in app toolbar | Frame: Header `0x53` / `0xAA`, Command `0x03` / `0x01`, Data `[state]`, Checksum | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Brightness Adjustment** | Slider 0%–100% (mapped to 0–255) | Command `0x02` (or sub-op), 1-byte value `0x00`–`0xFF` | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Static Colour (RGB)** | Colour wheel / palette selection | Frame writing RGB triplet `[R, G, B]` to active buffer | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Built-in Dynamic Effects** | Preset list: Rainbow, Breath, Chasing, Strobe, Fade, Flow | Effect ID byte (1–N), Speed byte (1–255), Direction (0/1) | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Effect Speed Adjustment** | Slider in effect panel (1–100 or 1–255) | Speed byte within effect command payload | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Effect Direction Toggle** | Forward / Reverse toggle | 1-bit / byte flag in effect frame | `CORROBORATED` | Implemented, untested on hardware (sim ready) |
| **Pixel Count / Length** | Settings: length 1 to 600 (or up to 1024) | 2-byte integer (little/big endian depending on IC) | `OBSERVED-STATIC` | Implemented, untested on hardware (gated behind `--i-understand-this-changes-the-device`) |
| **RGB Colour Sequence Order** | Selection: RGB, RBG, BRG, BGR, GRB, GBR | 1-byte enumeration (0–5) written to controller configuration register | `CORROBORATED` | Implemented, untested on hardware (gated behind `--i-understand-this-changes-the-device`) |
| **Driver IC Chip Selection** | Selection: WS2811, WS2812B, SK6812, UCS1903, SM16703 | 1-byte IC model code written to device configuration register | `OBSERVED-STATIC` | Implemented, untested on hardware (gated behind `--i-understand-this-changes-the-device`) |
| **Device Renaming** | Settings: custom alias stored on controller | String payload written to custom name characteristic / config frame | `OBSERVED-STATIC` | Implemented, untested on hardware (gated behind `--i-understand-this-changes-the-device`) |
| **Onboard Microphone Music Mode** | Music tab: internal microphone sensitivity and reactivity modes | Music mode packet: Mode ID (1–M), Sensitivity (1–100) | `OBSERVED-STATIC` | Implemented, untested on hardware (sim ready) |
| **Host Audio Reactive Mode** | Phone microphone capture streaming amplitudes | Host-side FFT / amplitude calculation mapped into high-frequency RGB frames | `INFERRED` | Implemented, untested on hardware (desktop audio capture) |
| **Timer / Schedule Operations** | Automatic on/off alarms by day of week and time | 8-to-12 byte scheduled event frame: days bitmap, hour, minute, action | `OBSERVED-STATIC` | Implemented, untested on hardware (sim ready) |
| **Custom DIY / Pixel Patterns** | Frame-by-frame custom colour sequence editor | Multi-packet fragmented stream specifying individual pixel ranges | `UNKNOWN` | Requires live trace / static bytecode extraction in Phase 1 |
| **Multi-Device Grouping** | App groups for synchronised multi-strip control | Client-side software synchronisation (simultaneous unicast GATT writes) | `INFERRED` | Implemented, untested on hardware (desktop multi-device manager) |
| **Firmware / OTA Update** | Device info / firmware check dialog | Vendor OTA service / firmware upgrade registers | `UNKNOWN` | **Excluded intentionally for hardware safety** |

---

## Safety Gating for Persistent Commands

Any command that alters physical NVRAM state on the controller presents a risk of misconfiguration if executed blindly without verified hardware feedback. Consequently, the following operations are strictly gated:

1. **IC Driver Chip Selection** (`WS2811`, `WS2812B`, etc.): Wrong timing codes may cause strip failure or signal corruption.
2. **Pixel Count Configuration**: Misconfigured lengths may lead to excessive power draw expectations or truncated rendering.
3. **RGB Colour Order**: Swapping channel mappings persistently.
4. **Device Renaming & Hardware Pairing**: Modifying broadcast identity.

In CLI and GUI, executing these requires passing `--i-understand-this-changes-the-device` or confirming an explicit, multi-step safety modal.
