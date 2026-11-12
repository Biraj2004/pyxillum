# Contributing to Pyxillum

Thank you for helping build Pyxillum. We welcome contributions from developers, reverse engineers, and lighting enthusiasts.

Because Pyxillum is developed under strict clean-room engineering rules, all contributors must adhere to the protocol derivation and attribution standards detailed below.

---

## 1. Clean-Room Contribution Principles

1. **No Proprietary Code or Binaries**:
   - Do NOT commit decompiled bytecode, APKs, proprietary artwork, screenshots of vendor applications, or proprietary binary blobs.
   - Any reverse-engineering notes or local captures must reside exclusively in gitignored directories (e.g. `./re-workspace/`).
2. **Independent Re-implementation**:
   - Protocol facts must be described in your own words in `docs/protocol.md`. Implementation code in `crates/` must be written cleanly from those specifications.
3. **Honest Confidence Labelling**:
   - Mark all new protocol additions according to their factual verification status: `OBSERVED-STATIC`, `CORROBORATED`, `INFERRED`, or `VERIFIED-HW`.
   - Features cannot be claimed as "supported" until verified on physical hardware.

---

## 2. Development Setup

### Toolchain Requirements
- Rust 1.80.0+ (MSRV)
- Standard Cargo tools: `rustfmt`, `clippy`

### Building and Testing
```bash
# Verify formatting, clippy (-D warnings), and test suite
cargo xtask check

# Run tests across all workspace crates
cargo test --workspace

# Launch the desktop application against the built-in simulator
cargo run -p pyxillum-app
```

---

## 3. Guide: Adding a New Controller or Effect

### Adding a New Effect Pattern
1. **Define the Effect Code**:
   Add the opcode constant and metadata to `crates/protocol/src/lib.rs`.
2. **Update the Simulator**:
   Add animation rendering math or wave generation in `crates/sim::DeviceSimulator::render_strip()`.
3. **Add Test Vectors**:
   Append expected JSON frame encodings to `tests/vectors/effects.json`.
4. **Expose in the Desktop UI**:
   Register the neutral technical name in `crates/app/src/main.rs`.

---

## 4. Guide: Contributing a Verified Protocol Trace

When verifying a physical controller (such as an SP621E, SP611E, or variant):

1. **Scrub Personally Identifiable Information (PII)**:
   Never submit raw traces containing your personal Bluetooth MAC address. Replace addresses with `XX:XX:XX:XX:XX:XX`.
2. **Capture Cleaning Helper Script**:
   Use this Python one-liner to sanitize Wireshark hex logs:
   ```bash
   python -c "import re, sys; print(re.sub(r'([0-9A-Fa-f]{2}[:-]){5}[0-9A-Fa-f]{2}', 'XX:XX:XX:XX:XX:XX', open(sys.argv[1]).read()))" raw_trace.txt > sanitized_trace.txt
   ```
3. **Submit via Issue Template**:
   Open an issue using the [Hardware Trace Submission](.github/ISSUE_TEMPLATE/hardware_trace.md) template and attach your sanitized logs and test results.

---

## 5. Commit Guidelines

We enforce the [Conventional Commits](https://www.conventionalcommits.org/) specification:
- `feat:` New features or user-facing capabilities
- `fix:` Bug fixes or protocol parsing corrections
- `docs:` Documentation and architecture updates
- `test:` Unit tests, property tests, or test vectors
- `refactor:` Code restructuring without behavior changes
- `ci:` CI workflow modifications

