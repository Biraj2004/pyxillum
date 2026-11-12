<div align="center">

<img src="../assets/icon.svg" alt="Pyxillum App Icon" width="96" height="96" />

# Legal and Interoperability Notes

**Clean-room engineering rationale, statutory interoperability exemptions, and legal questions.**

[![Licence: AGPL-3.0-or-later](https://img.shields.io/badge/Licence-AGPL--3.0-blue?style=flat-square)](../LICENSE)
[![European Union: Directive 2009/24/EC](https://img.shields.io/badge/EU-Directive%202009%2F24%2FEC%20Art.%206-informational?style=flat-square)](#1-interoperability-rationale)
[![United States: 17 U.S.C. § 1201(f)](https://img.shields.io/badge/US-17%20U.S.C.%20%C2%A7%201201(f)-informational?style=flat-square)](#1-interoperability-rationale)
[![India: Copyright Act § 52(1)(ab)](https://img.shields.io/badge/India-Copyright%20Act%20%C2%A7%2052(1)(ab)-informational?style=flat-square)](#1-interoperability-rationale)

<p align="center">
  <a href="../README.md"><strong>Project Root</strong></a> &bull;
  <a href="protocol.md"><strong>Protocol Spec</strong></a> &bull;
  <a href="../AGENTS.md"><strong>Agent Guidelines</strong></a>
</p>

</div>

---

> [!WARNING]
> **Disclaimer**: This document does not constitute formal legal advice. It outlines the engineering rationale for clean-room interoperability and identifies legal considerations that should be reviewed by a qualified legal professional in the appropriate jurisdiction.

---

## 1. Interoperability Rationale

The development of this software is undertaken strictly for the purpose of enabling independent, cross-platform interoperability between standard consumer computing platforms (Windows, macOS, Linux) and Bluetooth Low Energy (BLE) pixel LED controllers owned by the user.

### Key Factors Supporting Fair Use & Interoperability:
1. **Absence of Vendor Desktop Support**: The original equipment manufacturer provides companion applications exclusively for mobile operating systems (Android, iOS). No native desktop software exists to control these devices.
2. **Access Controls & DRM**: The hardware communication channel relies on standard, unencrypted Bluetooth Low Energy GATT transactions without cryptographic handshakes, copy protection, or digital rights management (DRM) circumvention.
3. **Clean-Room Engineering Methodology**:
   - Decompiled artefacts, reverse-engineering notes, and intermediate decompilations are quarantined in an untracked, gitignored directory (`./re-workspace/`).
   - The production codebase contains exclusively original Rust implementations derived from documented wire protocol specifications.
   - No decompiled source code, proprietary graphic assets, proprietary font files, or copyrighted audio files from the original vendor app are redistributed or committed.
4. **Independent Branding & Non-Affiliation**:
   - The project uses an entirely distinct, original brand name (`Pyxillum`).
   - Third-party mobile application names are strictly avoided. Physical hardware model designations (e.g. "SP621E") are cited solely in a factual nominative sense to indicate device compatibility.
5. **Limitation of Liability & Statutory Immunity**:
   - The software is provided under AGPL-3.0 with complete disclaimer of warranties and limitation of liability (Sections 15, 16, and 17).
   - Under international statutory exemptions (Directive 2009/24/EC Art. 6, US 17 U.S.C. § 1201(f), Indian Copyright Act Section 52(1)(ab)), reverse engineering unencrypted protocols solely to achieve interoperability with independent computer systems is legally protected.

---

## 2. Open Questions for a Qualified Legal Professional

When consulting legal counsel regarding open-source distribution of this interoperability project, the following questions should be reviewed:

1. **Jurisdictional Reverse Engineering Protections**:
   - Under US Law (17 U.S.C. § 1201(f) and relevant fair use precedents such as *Sega v. Accolade* and *Sony v. Connectix*), does the reverse engineering of unencrypted BLE protocol frames for interoperability fall safely within statutory exemptions?
   - Under European Union Directive 2009/24/EC (Articles 5 and 6), are all decompilation steps for obtaining information necessary to achieve interoperability fully compliant?
   - Under Indian Copyright Act, 1957 (Section 52(1)(ab)), is the observation, study, or testing of computer program functioning to determine underlying ideas and principles protected?
2. **Nominative Fair Use of Hardware Identifiers**:
   - Does citing specific model numbers (e.g. "SP621E") in compatibility matrices and documentation qualify as nominative fair use without risk of trademark dilution?
3. **Distribution of Protocol Test Vectors**:
   - Do static-derived protocol test vectors consisting of synthetic hex byte arrays represent functional facts (unprotectable under copyright law) rather than expressive content?

