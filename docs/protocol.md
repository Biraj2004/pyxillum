# Protocol Specification: SP621E / BanlanX2 BLE Protocol

**Document Version**: 1.0.0  
**Project**: Pyxillum  
**Confidence Status**: Clean-room derived and corroborated against independent open-source implementations (`monty68/uniled`, `xevrion/wallglow`). Untested on physical hardware.

---

## 1. Physical Layer and Bluetooth Low Energy Profile

The target device family (including the SP621E Mini Bluetooth Pixel LED Controller, SP611E, and SP620E) interfaces as a standard Bluetooth Low Energy (BLE) peripheral exposing a transparent UART-like GATT profile.

### 1.1 Identifiers and Service Topology

| Identifier | Value | Confidence | Notes |
| :--- | :--- | :--- | :--- |
| **Service UUID** | `0000ffe0-0000-1000-8000-00805f9b34fb` (short `0xFFE0`) | `CORROBORATED` | Primary GATT Service |
| **Data Characteristic** | `0000ffe1-0000-1000-8000-00805f9b34fb` (short `0xFFE1`) | `CORROBORATED` | Handles both inbound writes and outbound notifications |
| **Characteristic Properties** | `WRITE_WITHOUT_RESPONSE`, `WRITE`, `NOTIFY` | `CORROBORATED` | High-frequency streaming uses `WRITE_NO_RESP` |
| **Manufacturer ID** | `0x5053` (`20563` decimal) / `5053` | `CORROBORATED` | Bluetooth SIG Company ID ("SP" in ASCII little-endian) |
| **Default ATT MTU** | 23 bytes (20 bytes effective payload) | `CORROBORATED` | Governs notification chunk fragmentation |

### 1.2 Concurrency and Timing Constraints
1. **Connection Exclusivity**: The hardware controller permits only **one active BLE central connection**. If a mobile companion app (`com.spled.scenex` or BanlanX) holds an active link, the controller does not respond to external connection attempts.
2. **Write Inter-packet Delay**:
   - Acknowledged writes (`WRITE` with response) consume ~110 ms round-trip time.
   - Unacknowledged writes (`WRITE_WITHOUT_RESPONSE`) must be paced at >= 40 ms between packets to avoid controller FIFO buffer overruns.

---

## 2. Client-to-Device Command Framing (`0xA0` Family)

All control commands transmitted from the host application to characteristic `0xFFE1` adhere to a uniform fixed-prefix framing structure:

```
+---------------+---------------+---------------+-----------------------+
|  Header (1B)  |  Opcode (1B)  |  Length (1B)  |    Payload (N Bytes)   |
|     0xA0      |    [0x..]     |     0x..      |       [0x.. ...]      |
+---------------+---------------+---------------+-----------------------+
```

### 2.1 Command Opcode Reference Table

| Opcode | Function | Payload Length | Payload Format | Example Hex Frame | Confidence |
| :---: | :--- | :---: | :--- | :--- | :---: |
| `0x62` | **Power State** | 1 byte | `0x01` = On, `0x00` = Off | `a0 62 01 01` (Turn On) | `CORROBORATED` |
| `0x63` | **Effect Selection** | 1 byte | `[Effect ID]` (0x01–0x8E, 0xBE, 0xC9–0xDA) | `a0 63 01 be` (Solid Colour) | `CORROBORATED` |
| `0x64` | **RGB Colour Order** | 1 byte | `[0x00..0x05]` (RGB=0, RBG=1, GRB=2, GBR=3, BRG=4, BGR=5) | `a0 64 01 02` (Set GRB) | `CORROBORATED` |
| `0x66` | **Brightness Level** | 1 byte | `[Level]` (`0x00` = 0% to `0xFF` = 100%) | `a0 66 01 ff` (Max Brightness) | `CORROBORATED` |
| `0x67` | **Effect Speed** | 1 byte | `[Speed]` (`0x01` = Slowest to `0x0A` = Fastest) | `a0 67 01 05` (Speed 5) | `CORROBORATED` |
| `0x68` | **Effect Length** | 1 byte | `[Length]` (`0x01` to `0x96` = 150 pixels) | `a0 68 01 10` (Length 16) | `CORROBORATED` |
| `0x69` | **Static RGB Colour**| 4 bytes | `[Red, Green, Blue, Level]` | `a0 69 04 ff 72 72 70` | `CORROBORATED` |
| `0x6A` | **Light Mode** | 1 byte | `0x00` = Single FX, `0x01` = Cycle FX, `0x02` = Sound FX | `a0 6a 01 00` (Single FX) | `CORROBORATED` |
| `0x6B` | **Mic Sensitivity** | 1 byte | `[Gain]` (`0x01` to `0x10` = 16 levels) | `a0 6b 01 08` (Gain 8) | `CORROBORATED` |
| `0x6C` | **Audio Input** | 1 byte | `0x00` = Internal Mic, `0x01` = Player, `0x02` = Ext Mic | `a0 6c 01 00` (Internal Mic) | `CORROBORATED` |
| `0x70` | **State Query** | 0 bytes | None | `a0 70 00` (Query Status) | `CORROBORATED` |
| `0x76` | **White Level** | 2 bytes | `[White, 0x00]` (For RGBW variants like SP617E) | `a0 76 02 ff 00` | `CORROBORATED` |

---

## 3. Peripheral-to-Client Status Notifications (`0x53 0x43`)

When queried via `a0 70 00`, or upon spontaneous state changes, the controller broadcasts notification frames over characteristic `0xFFE1`. Because the status payload exceeds standard 20-byte ATT MTU boundaries, responses are transmitted as multiple fragmented chunk packets.

### 3.1 Chunk Packet Framing

```
+--------------------+--------------------+--------------------+-----------------------+
|  Magic Prefix (2B) | Chunk Number (1B)  | Total Msg Len (1B) | Chunk Payload Len (1B)|
|    0x53  0x43      |    0x01, 0x02..    |      0x17 (23B)    |       0x0F (15B)      |
+--------------------+--------------------+--------------------+-----------------------+
|                                Chunk Data (N Bytes)                                  |
+--------------------------------------------------------------------------------------+
```

- **Header (5 bytes)**:
  - Byte 0: `0x53` ('S')
  - Byte 1: `0x43` ('C')
  - Byte 2: `Chunk Number` (`0x01` for the initial packet, `0x02` for the continuation)
  - Byte 3: `Total Message Length` (Total bytes in reassembled body, typically `0x17` = 23 bytes without timers)
  - Byte 4: `Chunk Payload Length` (Number of data bytes following in this specific packet)
- **Body**: The raw slice of the status payload.

### 3.2 Reassembled Status Payload Layout

Once chunks are assembled to `Total Message Length`, the unified buffer maps as follows:

| Offset | Field Name | Type | Range / Semantics |
| :---: | :--- | :--- | :--- |
| `0` | **Power State** | `u8` | `0x00` = Off, `0x01` = On |
| `1` | **Light Mode** | `u8` | `0x00` = Single FX, `0x01` = Auto Cycle Dynamic, `0x02` = Auto Sound |
| `2` | **Active Effect ID** | `u8` | Effect identifier (`0xBE` = Solid Color, `0x01` = Rainbow, etc.) |
| `3` | **RGB Colour Order** | `u8` | `0x00`–`0x05` (0=RGB, 1=RBG, 2=GRB, 3=GBR, 4=BRG, 5=BGR) |
| `4` | **Brightness** | `u8` | `0x00` to `0xFF` (0–255) |
| `5` | **Speed** | `u8` | `0x01` to `0x0A` (1–10) |
| `6` | **Effect Length** | `u8` | Number of segments (`0x01` to `0x96` = 150) |
| `7` | **Red Component** | `u8` | Current active Red value (`0x00`–`0xFF`) |
| `8` | **Green Component** | `u8` | Current active Green value (`0x00`–`0xFF`) |
| `9` | **Blue Component** | `u8` | Current active Blue value (`0x00`–`0xFF`) |
| `10` | **Audio Input** | `u8` | `0x00` = Internal Mic, `0x01` = Player, `0x02` = External Mic |
| `11` | **Audio Sensitivity**| `u8` | `0x01` to `0x10` (1–16) |
| `12..21` | **Device Firmware Flags** | 10 bytes | Controller firmware signature (e.g. `10 09 04 0b 14 1a 32 37 50 53`) |
| `22` | **Timer Count** | `u8` | Number of active programmed timers ($M$) |
| `23..` | **Timer Records** | $M \times 7$ bytes | 7 bytes per active timer entry |

### 3.3 Timer Record Encoding (7 Bytes per Timer)
- Byte 0: Timer slot index.
- Byte 1: Action (`0x01` = Turn On, `0x00` = Turn Off).
- Byte 2: Day-of-week recurrence bitmask (`0x7F` = Everyday, `0x80` = Run Once, `0x15` = Mon/Wed/Fri, `0x6A` = Sun/Tue/Thu/Sat).
- Bytes 3..5: Scheduled time encoding (seconds from midnight).
- Byte 6: Activation flag (`0x01` = Enabled, `0x00` = Disabled).

---

## 4. Reassembly State Machine

```
              +----------------------+
              |         IDLE         |
              +----------------------+
                         |
           Packet arrives with Chunk = 1
           and Header == 0x53 0x43
                         v
              +----------------------+
              |     COLLECTING       | <---+
              +----------------------+     | More chunks arrive
                         |                 | (Chunk == Prev + 1)
           Accumulated >= Total Length     |
                         |                 +---+
                         v
              +----------------------+
              |       COMPLETE       |
              | Parse Status Payload |
              +----------------------+
```

---

## 5. Corroborated Hex Trace Examples

### 5.1 Real Hardware Capture Trace (Corroborated)
```text
Host -> Peripheral: a0 70 00
Peripheral -> Host (Chunk 1): 53 43 01 17 0f 01 00 be 02 ff 0a 10 ff 00 5e 01 10 09 0b 14
Peripheral -> Host (Chunk 2): 53 43 02 17 08 1a 32 37 53 73 85 1b 00
```
**Interpretation**:
- Chunk 1: Header `53 43`, chunk `01`, total length `0x17` (23 bytes), chunk payload `0x0F` (15 bytes).
- Chunk 2: Header `53 43`, chunk `02`, total length `0x17` (23 bytes), chunk payload `0x08` (8 bytes).
- Decoded Status:
  - Power: `01` (ON)
  - Light Mode: `00` (Single FX)
  - Effect: `be` (Solid Colour)
  - Chip Order: `02` (GRB)
  - Brightness: `ff` (255 / 100%)
  - Speed: `0a` (10)
  - Length: `10` (16 pixels)
  - RGB: `#FF005E`
  - Audio Input: `01` (Player), Sensitivity: `10` (16)
