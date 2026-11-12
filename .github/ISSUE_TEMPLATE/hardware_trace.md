---
name: Hardware Verification Trace
about: Submit a verified hardware trace from a physical LED controller
title: "[HW-TRACE] Controller Model: <Model>"
labels: hardware-verification, protocol
assignees: ''
---

### Physical Hardware Details
- **Controller Model**: (e.g. SP621E, SP611E, SP620E)
- **Reported Bluetooth Name**: (e.g. `SP621E_...`)
- **Firmware Version (if known)**:
- **Connected LED Strip**: (e.g. WS2812B, SK6812, 30 LEDs, 5V)

### Verification Checklist Results
Please mark verified items from `tests/hardware/PLAN.md`:
- [ ] `HW-01`: BLE Discovery
- [ ] `HW-02`: Connection & Status Query
- [ ] `HW-03`: Power On / Off Switching
- [ ] `HW-04`: Static RGB Primary Alignment (Red, Green, Blue)
- [ ] `HW-05`: Brightness Dimming
- [ ] `HW-06`: Dynamic Flow Patterns & Speed
- [ ] `HW-07`: Colour Order Configuration (GRB)

### Packet Trace / Log (Sanitized)
> Please ensure personal MAC addresses are scrubbed (`XX:XX:XX:XX:XX:XX`) before posting.

```hex
<Paste raw Wireshark / pyxillum hwcheck log output here>
```

### Visual Observations & Anomalies
Describe any unexpected behavior, timing issues, or flickering.
