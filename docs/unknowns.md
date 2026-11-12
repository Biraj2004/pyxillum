# Protocol Unknowns and Risk Register

This document tracks unresolved protocol behaviours that require confirmation on physical hardware or live Bluetooth HCI snoop captures. Items are prioritized by the operational risk they pose to physical controller hardware.

---

## Unknowns Categorised by Risk

### Severity P0: High Risk (Hardware / NVRAM Corruption Risk)
*These commands modify persistent non-volatile registers or IC driving signals. They must NEVER be written automatically without hardware verification and safety gating.*

| Unknown ID | Description | Potential Hardware Risk | Mitigation / Quarantine |
| :---: | :--- | :--- | :--- |
| **UNK-P0-01** | **Direct Driver IC Reconfiguration Byte Code** | Inappropriate timing signals or high-frequency clock pulses may destabilise or damage sensitive LED driver ICs (e.g. SK6812 vs WS2811 vs APA102 clock pins). | Gated behind `--i-understand-this-changes-the-device`. Omitted from standard UI controls until verified on hardware. |
| **UNK-P0-02** | **Maximum Pixel Limit Rejection Boundary** | Behavior when setting lengths exceeding physical controller limits (e.g. 600 vs 1024 pixels). Unchecked limits could overflow memory or trigger controller brownouts. | Strictly clamp pixel length in software to safe conservative defaults (1 to 150/300) in both simulator and transport. |
| **UNK-P0-03** | **Firmware / OTA Memory Registers** | Accidental invocation of vendor bootloader or OTA flash routines can permanently brick the controller MCU. | **Completely excluded from the codebase.** No OTA code paths will be written. |

### Severity P1: Medium Risk (Hardware Lockout / Operational Disruption)
*Behaviours that may disrupt controller pairing, discovery, or normal BLE connectivity.*

| Unknown ID | Description | Operational Risk | Mitigation |
| :---: | :--- | :--- | :--- |
| **UNK-P1-01** | **Controller Reset / Restore Factory Settings Command** | Could wipe user settings, Bluetooth pairing keys, or device alias. | Do not implement factory reset without explicit hardware trace confirmation and dedicated warning modals. |
| **UNK-P1-02** | **Broadcast Renaming Encoding** | Writing malformed UTF-8 or oversized strings to device alias registers may render the peripheral un-scannable. | Gated behind confirmation flag; validate string length (max 16 ASCII characters). |

### Severity P2: Low Risk (Functional Discrepancies)
*Non-damaging protocol quirks that affect visual fidelity or features.*

| Unknown ID | Description | Impact | Resolution Path |
| :---: | :--- | :--- | :--- |
| **UNK-P2-01** | **DIY / Custom Pixel Pattern Wire Format** | Custom pixel animations may use a multi-packet streaming protocol or an internal script buffer format not yet fully documented. | Marked as `UNKNOWN` in feature matrix. Feature disabled until live trace with original app is captured. |
| **UNK-P2-02** | **Audio Reactive Streaming Packets** | Real-time audio streaming from the phone may use raw frequency spectrum bins or rapid static color overrides. | Provide host-side audio visualizer running through standard high-frequency unacknowledged color frames. |
| **UNK-P2-03** | **Notification Chunk Ordering on High Packet Loss** | If chunk 2 arrives before chunk 1, or is dropped by the host BLE stack. | Assembler drops out-of-order chunks and awaits the next fresh sequence starting at chunk 1. |

### Severity P3: Informational / Cosmetic
*Metadata and visual nuances.*

| Unknown ID | Description | Impact |
| :---: | :--- | :--- |
| **UNK-P3-01** | **Trailing Firmware Bytes Semantics** | Exact decoding of bytes 12..21 in status payloads (hardware revision codes vs calibration numbers). | Display as hexadecimal raw bytes in debug panels. |
| **UNK-P3-02** | **Built-in Microphone AGC (Automatic Gain Control) Curve** | Exact mapping of sensitivity values 1..16 to dB gain thresholds. | Map linearly in the UI. |
