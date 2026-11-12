# ADR 0002: Evaluation of Bluetooth Low Energy (BLE) Libraries

## Status
Proposed (Awaiting User Approval)

## Context
The application must communicate with Bluetooth Low Energy (BLE) peripheral hardware across Windows, macOS, and Linux. The controller protocol runs over a standard GATT service (`0xFFE0` / `0xFFE1`), requiring:
- Adapter discovery and passive/active peripheral scanning.
- Connection establishment and MTU negotiation.
- Characteristic discovery, write-without-response / write-with-response commands, and notification streaming.
- Complete decoupling from UI and domain logic to permit in-memory hardware simulation (`SimTransport`).

## Candidate Libraries Evaluated

### 1. `btleplug` (v0.13)
- **Scope**: Cross-platform (Windows, macOS, Linux, iOS, Android).
- **Backend Implementations**:
  - Windows: Uses official Windows Runtime (WinRT) APIs (`Windows.Devices.Bluetooth.GenericAttributeProfile`).
  - macOS: Uses Apple `CoreBluetooth` via Objective-C bindings.
  - Linux: Uses BlueZ D-Bus system bus interface.
- **Strengths**: The de facto cross-platform standard in the Rust ecosystem. Actively maintained by the `deviceplug` organization. Supports async Tokio streams for discovery and characteristic notifications.
- **Weaknesses / Discovered Quirks**:
  - As observed in `andrewdavidmackenzie/meshchat`, native BLE scan filtering at the adapter level fails silently or behaves inconsistently across different OS Bluetooth stacks (particularly BlueZ vs Windows WinRT).
  - Mitigation: Conduct unfiltered adapter scans and implement manufacturer ID and service UUID matching inside application code.

### 2. Platform-Specific Alternatives
- **Linux (`bluer`)**:
  - Native BlueZ interface with complete coverage (GATT server, client, pairing, agent management).
  - High fidelity on Linux, but completely incompatible with Windows and macOS.
- **Windows (`windows` crate WinRT)**:
  - Direct WinRT bindings to `Windows::Devices::Bluetooth`.
  - Zero third-party abstraction overhead, but requires writing and maintaining platform-specific C#/COM-style Rust code and provides no macOS or Linux support.
- **macOS (`objc2-core-bluetooth`)**:
  - Direct bindings to macOS CoreBluetooth framework. macOS only.

## Decision & Architectural Boundary

### Decision:
Use **`btleplug` (v0.13)** wrapped strictly behind a custom `Transport` trait defined in `crates/transport`.

### Architectural Rules:
1. **Strict Trait Isolation**: Neither `crates/protocol`, `crates/core`, nor `crates/app` may import or reference `btleplug`. All communication occurs through the `Transport` trait:
   ```rust
   #[async_trait::async_trait]
   pub trait Transport: Send + Sync {
       async fn scan(&self) -> Result<Vec<DiscoveredDevice>, TransportError>;
       async fn connect(&mut self, address: &str) -> Result<(), TransportError>;
       async fn disconnect(&mut self) -> Result<(), TransportError>;
       async fn write(&mut self, data: &[u8]) -> Result<(), TransportError>;
       async fn subscribe(&mut self) -> Result<Receiver<Vec<u8>>, TransportError>;
       fn is_connected(&self) -> bool;
   }
   ```
2. **First-Class Simulator Support**: The same `Transport` trait is implemented by `SimTransport`, which routes packets directly to `crates/sim::DeviceSimulator` without touching Bluetooth hardware.
3. **Application-Level Filtering**: When scanning with `btleplug`, request unfiltered discovery and apply MAC/name/manufacturer (`0x5053`) filtering in software to guarantee uniform discovery behaviour across Windows, macOS, and Linux.

