#!/usr/bin/env bash
# bump-crate.sh — Bump version of a crate and update CHANGELOG
#
# Usage:
#   ./scripts/bump-crate.sh hlc-rs patch    # 0.1.0 -> 0.1.1
#   ./scripts/bump-crate.sh sphinx-rs minor # 0.1.0 -> 0.2.0
#   ./scripts/bump-crate.sh trust-store major # 0.1.0 -> 1.0.0
#   ./scripts/bump-crate.sh ghostwire-dtn 0.2.3  # explicit version

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
LIBS_DIR="$ROOT_DIR/ghostwire-libs"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${GREEN}[bump]${NC} $*"; }
warn() { echo -e "${YELLOW}[bump]${NC} $*"; }
err()  { echo -e "${RED}[bump]${NC} $*" >&2; exit 1; }

CRATE="${1:-}"
BUMP="${2:-patch}"

[ -z "$CRATE" ] && err "Usage: $0 <crate-name> [patch|minor|major|<version>]"

CARGO_TOML="$LIBS_DIR/crates/$CRATE/Cargo.toml"
CHANGELOG="$LIBS_DIR/crates/$CRATE/CHANGELOG.md"

[ ! -f "$CARGO_TOML" ] && err "Cargo.toml not found: $CARGO_TOML"

# Get current version
CURRENT_VERSION=$(grep '^version = ' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/')
[ -z "$CURRENT_VERSION" ] && err "Could not find version in $CARGO_TOML"

# Calculate new version
if [[ "$BUMP" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    NEW_VERSION="$BUMP"
else
    IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"
    case "$BUMP" in
        patch) PATCH=$((PATCH + 1)) ;;
        minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
        major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
        *) err "Invalid bump type: $BUMP (use patch, minor, major, or explicit version)" ;;
    esac
    NEW_VERSION="$MAJOR.$MINOR.$PATCH"
fi

log "Bumping $CRATE: $CURRENT_VERSION -> $NEW_VERSION"

# Update Cargo.toml
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"

# Update CHANGELOG
TODAY=$(date +%Y-%m-%d)
if [ -f "$CHANGELOG" ]; then
    # Add new entry after the header
    sed -i "/^## \[Unreleased\]/a \\\n## [$NEW_VERSION] - $TODAY" "$CHANGELOG"
else
    cat > "$CHANGELOG" << EOF
# Changelog

All notable changes to this project will be documented in this file.

## [$NEW_VERSION] - $TODAY

### Added
- Initial release
EOF
fi

log "Updated $CARGO_TOML and $CHANGELOG"
log "Run 'cargo publish -p $CRATE' to publish"
