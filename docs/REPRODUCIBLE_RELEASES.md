# Reproducible Release and Checksum Policy

GhostWire must not publish stable binaries merely because a local build succeeds. Every release is a provenance bundle.

Each versioned release must include the final artefact or container digest, a SHA-256 manifest, a signed release manifest, exact source commit, build toolchain and target, SBOM, and verification instructions. Checksums are calculated before upload and verified again after download. Versioned immutable links are required; a moving `latest` tag is not sufficient evidence.

Cloudflare R2 is suitable for immutable versioned artefacts and manifests after the release gates pass. Containers must be published by immutable digest with source commit and SBOM. A development image that has not passed application tests is not a production image.

Current state: no stable GhostWire binary or signed release manifest is published. The Codeberg review snapshot is source continuity only.
