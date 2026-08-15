# GhostWire License and Dependency Compliance Audit

**Status:** Preliminary engineering compliance audit.
**Project declaration:** `AGPL-3.0-or-later` in `ghostwire/Cargo.toml`; repository root describes an AGPL/commercial dual-license model.
**Audit limitation:** This is not legal advice and is not a complete clearance. Registry DNS was unavailable during the audit, so full crate-metadata resolution and `cargo-license` installation could not complete.

## Executive conclusion

The portion of the resolved Rust dependency graph that could be mechanically inspected presents **no confirmed third-party GPL, AGPL, or LGPL conflict** with GhostWire’s `AGPL-3.0-or-later` declaration. The majority of reviewed crates use MIT, Apache-2.0, or dual MIT/Apache licensing. Those licenses are generally compatible with GPLv3-family combination, subject to preserving notices and the terms selected by the relevant SPDX expression.[1]

The audit is nevertheless **incomplete**: the Cargo.lock contains **848 packages**, of which **618** had locally cached manifests available for inspection and **230** did not. An incomplete license report is not a release approval. In particular, optional and platform-specific dependency subgraphs must be resolved and checked online before release.

Three cached dependencies declare MPL-2.0 (`attohttpc 0.30.1`, `option-ext 0.2.0`, and `webpki-roots 0.25.4`). MPL-2.0 can be combined with AGPLv3-or-later under the secondary-license mechanism if the source is not marked **Incompatible With Secondary Licenses** and MPL notice/source obligations are honored. The cached Rust source files were searched and contained no such opt-out header. This is a positive technical check, not legal clearance; it must be repeated for the final lockfile/release sources.[1] [2]

| Audit result | Status | Meaning |
|---|---|---|
| Root license declaration | **Confirmed** | `AGPL-3.0-or-later` appears in the Cargo manifest; root LICENSE presents AGPL plus a commercial path. |
| Cached dependency licenses | **Partially inspected** | 618 of 848 Cargo.lock packages were inspected from local manifests. |
| Confirmed GPL/AGPL/LGPL third-party dependency | **None in cached inventory** | No such third-party SPDX expression was found; this is not proof for 230 unverified packages. |
| MPL-2.0 source header review | **Positive but conditional** | No cached `.rs` file contained the secondary-license opt-out notice. |
| Full tool-backed release inventory | **Blocked** | `cargo install cargo-license` failed due to registry DNS; full `cargo metadata --offline` required an uncached Android package. |
| Release readiness | **Not approved** | Re-run online in CI and review all features/platforms before any formal release. |

## Evidence and methodology

The audit generated `ghostwire_audit/license_inventory_lockfile.md` from `Cargo.lock` and locally cached crate manifests. It captures package name, version, source, declared SPDX expression, and whether the manifest was actually available. The fallback was necessary because Cargo’s full offline metadata resolution stopped at missing `android_system_properties v0.1.5`, and crates.io DNS resolution failed while installing `cargo-license`.

Cargo interprets a package’s `license` as an SPDX expression. In particular, `OR` permits a choice of licenses, `AND` requires simultaneous compliance with every listed license, and `WITH` denotes a license exception. The manifest field is valuable machine-readable metadata, but it does not replace source-level copyright notices, embedded license files, generated-code licenses, native/system-library licenses, or contractual terms.[3]

| Declared SPDX expression in cached/local inventory | Count |
|---|---:|
| `MIT OR Apache-2.0` | 265 |
| `MIT` | 155 |
| `Apache-2.0 OR MIT` | 62 |
| `MIT/Apache-2.0` | 34 |
| `Unicode-3.0` | 18 |
| `Apache-2.0` | 16 |
| Other permissive/notice-bearing expressions | 65 |
| `MPL-2.0` | 3 |
| `AGPL-3.0-or-later` | 1 (GhostWire itself) |
| Not verified because manifest was not cached | 230 |

The last row is deliberately not labeled `MISSING` in a legal sense. It means the audit did not obtain the crate manifest locally; it says nothing by itself about the crate’s license.

## Key dependency observations

| Dependency | Version observed | Declared license evidence | Assessment |
|---|---:|---|---|
| `libp2p` | 0.56.0 | MIT | Compatible with the project’s AGPL distribution model, subject to notice preservation. |
| `bluer` | 0.17.4 | BSD-2-Clause | No GPL-family conflict found. The earlier concern that it might be LGPL was not supported by the cached manifest. |
| `dbus` | 0.9.12 | Apache-2.0/MIT | Compatible on the reviewed evidence; verify system-library and platform packaging obligations separately. |
| `ort` | 2.0.0-rc.12 | MIT OR Apache-2.0 | Compatible on crate metadata; separately review ONNX Runtime binary/model distribution and any provider artifacts. |
| `pqcrypto-mlkem` | 0.1.1 | MIT OR Apache-2.0 | Compatible on reviewed evidence. |
| `sphinx-rs` | 0.1.0 | MIT OR Apache-2.0 | Compatible on reviewed evidence. |
| `shamir_secret_sharing` | 0.1.1 | MIT | Compatible on reviewed evidence. |
| `vodozemac` | 0.7.0 | Apache-2.0 | Compatible on reviewed evidence. A second 0.9.0 lockfile entry was not cached and remains a verification item. |
| `matrix-sdk` | 0.16.1 | Not cached | No conclusion; verify before enabling/releasing `matrix`/`matrix-bridge`. |
| `serialport` | 4.9.0 | Not cached | No conclusion; verify before enabling/releasing Meshtastic/platform serial support. |
| `attohttpc` | 0.30.1 | MPL-2.0 | Conditional compatibility; source-header inspection found no secondary-license opt-out. |
| `option-ext` | 0.2.0 | MPL-2.0 | Conditional compatibility; source-header inspection found no secondary-license opt-out. |
| `webpki-roots` | 0.25.4 | MPL-2.0 | Conditional compatibility; source-header inspection found no secondary-license opt-out. |

## AGPL and distribution obligations

The GNU AGPL is designed for network-server software. A modified version used to provide network interaction must offer the corresponding source to its users. It also places source, notice, and license obligations on conveyed non-source distributions. The public GhostWire web API, WebSocket server, Docker images, binary releases, and any hosted daemon should therefore provide a clear, accurate source-offer/legal-notice path before public deployment.[4]

GhostWire’s root LICENSE states an AGPL/commercial dual-license model. That model needs operational precision. A commercial exception cannot reliably be offered unless the entity offering it has authority to license all applicable project copyright, contributor rights, generated assets, and bundled materials. Establish a contributor policy—such as Developer Certificate of Origin, contributor license agreement, or copyright assignment strategy—before marketing commercial terms broadly. This is a governance recommendation, not a conclusion that the existing terms are invalid.

| Distribution mode | Minimum AGPL/compliance actions |
|---|---|
| Source repository | Preserve license/copyright notices; document exact build instructions; ship dependency notice inventory. |
| Prebuilt binary | Include AGPL text, a clear corresponding-source location matching the binary revision, and third-party notices. |
| Docker image / playground | Include image build source, Dockerfiles, exact source revision, license/notices, and any bundled binary/model licenses. |
| Public web/API service | Add a prominent appropriate legal notice/source link accessible to network users; keep the deployed source revision available. |
| Commercial exception deployment | Maintain separate written terms, provenance/contributor authority record, and third-party attribution compliance. |

## MPL-2.0 handling

Mozilla describes MPL-2.0 as file-level copyleft. It generally creates external-distribution obligations for MPL-covered source and requires recipients be informed how to obtain applicable source. GNU’s license analysis identifies a path to combine MPL-2.0 with AGPLv3-or-later if the MPL code is not marked incompatible with secondary licenses; original MPL files must remain available under MPL terms.[1] [2]

The cached audit searched Rust source files for the exact Exhibit-B phrase `Incompatible With Secondary Licenses` in the three MPL crates and did not find it. A text match in the MPL license document itself is expected and is not an opt-out. The release process must preserve this check and add a manual inspection of the license header/NOTICE/source distribution route for all MPL crates actually shipped.

## Special dependencies and non-Rust artifacts

The Cargo crate license is not the whole compliance picture. GhostWire should inventory artifacts that can be embedded, downloaded, invoked, or redistributed outside ordinary Rust source compilation.

| Artifact class | Example | Required follow-up |
|---|---|---|
| Native runtime / provider | ONNX Runtime used by `ort` | Verify native binary, execution provider, model, and distribution license separately. |
| AI model / tokenizer | ONNX models or downloaded model assets | Maintain model card, source, license, provenance, redistribution, privacy, and acceptable-use review. |
| System library / driver | Bluetooth, D-Bus, serial ports | Check platform packaging/linkage and vendor terms in each target image. |
| Embedded web asset | UI fonts, icons, JavaScript, docs assets | Inventory and attribute separately; avoid copying assets from GPL projects without review. |
| Protocol or reference implementation code | NomadNet, Reticulum, Signal/libsignal | Product ideas and public protocol specifications are not code licenses. Do not copy implementation/UI assets without an explicit source/license analysis. |
| Docker/base image | Rust, Debian/Alpine, Python images | Pin digest, retain image/license/SBOM data, and comply with base-image notices. |

## Required controls

### 1. Make license verification reproducible and blocking

After registry access is restored, pin a license-analysis tool in CI and run it against the locked graph. `cargo-deny` or `cargo-license` should emit an artifact. The allowlist must explicitly include only approved SPDX expressions and must fail builds for missing/unapproved metadata unless a documented exception has an owner and expiry date.

```toml
# Illustrative policy; validate tool syntax/version before adoption.
[licenses]
allow = [
  "AGPL-3.0-or-later", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause",
  "CC0-1.0", "CDLA-Permissive-2.0", "ISC", "MIT", "MPL-2.0",
  "Unicode-3.0", "Unlicense", "Zlib"
]
```

MPL approval must be paired with an explicit secondary-license compatibility/header check. Any GPL-family, custom, non-SPDX, or unknown license should require manual review—not an automatic deny/allow based only on a name.

### 2. Publish a complete notice artifact

Add a generated, version-controlled or release-attached `THIRD_PARTY_NOTICES.md` containing every resolved package, license expression, copyright/notice requirements where available, and the exact lockfile hash. Attach it to binaries, Docker images, and release archives. Preserve Apache notice/patent terms, BSD/MIT/ISC attribution, MPL source/notice requirements, Unicode terms, and any model/native-library notices.

### 3. Verify every feature profile

The default feature profile is insufficient. Perform license and supply-chain checks for the supported matrix at minimum:

| Profile | Reason |
|---|---|
| Default | Public web/CLI/mesh/security/stealth/QR release graph. |
| `--no-default-features --features reticulum` | Current experimental bridge graph; do not claim support until it compiles. |
| Matrix bridge | Pulls Matrix ecosystem dependencies and requires its own license/security review. |
| Meshtastic/serial | Pulls native/platform serial dependencies. |
| Bluetooth on supported Linux target | Pulls D-Bus/Bluetooth packages and potentially system-layer requirements. |
| Iroh future feature | Requires a fresh resolved graph and relay/discovery privacy review. |

### 4. Add an SBOM and provenance record

A release should include a CycloneDX or SPDX SBOM, `Cargo.lock`, source revision, Rust toolchain version, feature profile, base-image digest, generated notices, and vulnerability/advisory report. This allows users and maintainers to distinguish the reviewed release from an arbitrary later dependency resolution.

## Release gates

| Gate | Required evidence | Owner |
|---|---|---|
| Full dependency resolution | Online locked report resolves every package and none remain unverified. | Release engineering |
| License allow/exception decision | Reviewed policy plus manual review for every exception. | Maintainer + legal reviewer where needed |
| MPL review | Header/notice review for all shipped MPL crates. | Maintainer |
| Native/model artifact review | Separate provenance/license record for each bundled artifact. | Feature owner |
| Third-party notices | Artifact attached to source, binary, Docker, and release distribution. | Release engineering |
| AGPL source offer | Public service/binary source path matches deployed revision. | Deployment owner |
| Contributor/commercial authority | Written contributor and commercial licensing policy. | Project owner |

## References

[1]: https://www.gnu.org/licenses/license-list.html#MPL-2.0 "GNU license list — MPL-2.0 compatibility notes"
[2]: https://www.mozilla.org/en-US/MPL/2.0/FAQ/ "Mozilla Public License 2.0 FAQ"
[3]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields "Cargo manifest license and license-file fields"
[4]: https://www.gnu.org/licenses/agpl-3.0.en.html "GNU Affero General Public License v3"
