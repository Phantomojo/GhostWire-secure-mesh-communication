#!/usr/bin/env bash
# sync-crates.sh — Sync standalone crates INTO ghostwire/
#
# Approach A: ghostwire-libs/crates/ is the canonical source.
# Changes to crates auto-publish, then sync into ghostwire/.
#
# Usage:
#   ./scripts/sync-crates.sh              # Sync all crates into ghostwire/
#   ./scripts/sync-crates.sh sphinx-rs    # Sync only sphinx-rs
#   ./scripts/sync-crates.sh --check      # Check if ghostwire/ is up to date
#
# After syncing, ghostwire/ uses the updated crate code via path dependencies
# in ghostwire/Cargo.toml.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
GW_DIR="$ROOT_DIR/ghostwire"
LIBS_DIR="$ROOT_DIR/ghostwire-libs"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${GREEN}[sync]${NC} $*"; }
warn() { echo -e "${YELLOW}[sync]${NC} $*"; }
err()  { echo -e "${RED}[sync]${NC} $*" >&2; }

# Crate -> destination in ghostwire/
declare -A CRATE_DESTS=(
    ["sphinx-rs"]="src/core/routing/sphinx.rs"
    ["ghostwire-dtn"]="src/core/dtn/bundle.rs src/core/dtn/epidemic.rs"
    ["hlc-rs"]="src/core/hlc.rs"
    ["trust-store"]="src/core/reputation.rs"
)

CHECK_MODE=false
TARGET_CRATE=""

for arg in "$@"; do
    case "$arg" in
        --check) CHECK_MODE=true ;;
        *) TARGET_CRATE="$arg" ;;
    esac
done

sync_crate() {
    local crate="$1"
    local dests="${CRATE_DESTS[$crate]}"
    local crate_dir="$LIBS_DIR/crates/$crate"

    if [ ! -d "$crate_dir" ]; then
        err "Crate directory not found: $crate_dir"
        return 1
    fi

    local changed=false

    for dest in $dests; do
        local src_file="$crate_dir/src/$(basename "$dest")"
        local dest_file="$GW_DIR/$dest"

        if [ ! -f "$src_file" ]; then
            warn "Crate source not found: $src_file (skipping)"
            continue
        fi

        # Ensure destination directory exists
        mkdir -p "$(dirname "$dest_file")"

        if [ "$CHECK_MODE" = true ]; then
            if [ ! -f "$dest_file" ] || ! diff -q "$src_file" "$dest_file" > /dev/null 2>&1; then
                warn "$crate: $dest is out of sync"
                changed=true
            fi
        else
            if [ ! -f "$dest_file" ] || ! diff -q "$src_file" "$dest_file" > /dev/null 2>&1; then
                log "Syncing $crate/src/$(basename "$dest") -> $dest"
                cp "$src_file" "$dest_file"
                changed=true
            fi
        fi
    done

    if [ "$CHECK_MODE" = true ] && [ "$changed" = true ]; then
        return 1
    fi
    return 0
}

if [ "$CHECK_MODE" = true ]; then
    log "Checking if ghostwire/ is in sync with crates..."
    exit_code=0

    if [ -n "$TARGET_CRATE" ]; then
        sync_crate "$TARGET_CRATE" || exit_code=1
    else
        for crate in "${!CRATE_DESTS[@]}"; do
            sync_crate "$crate" || exit_code=1
        done
    fi

    if [ $exit_code -eq 0 ]; then
        log "ghostwire/ is in sync with all crates"
    fi
    exit $exit_code
fi

# Sync mode
if [ -n "$TARGET_CRATE" ]; then
    log "Syncing $TARGET_CRATE into ghostwire/..."
    sync_crate "$TARGET_CRATE"
else
    log "Syncing all crates into ghostwire/..."
    for crate in "${!CRATE_DESTS[@]}"; do
        sync_crate "$crate"
    done
fi

log "Done"
