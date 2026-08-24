#!/usr/bin/env bash
set -Eeuo pipefail

# GhostWire verified dual-node release pipeline.
# Default behavior is isolated-node promotion only; production requires --promote-production.
REPO=${REPO:-/home/ph/GhostWire-secure-mesh-communication-newest}
GW=${REPO}/ghostwire
WEB=${REPO}/webui
ARTIFACT_ROOT=${ARTIFACT_ROOT:-/home/ph/.cache/ghostwire-verified-releases}
OMEN_TARGET=${OMEN_TARGET:-/home/ph/.cache/ghostwire-verified-target}
ELITE_HOST=${ELITE_HOST:-phantomojo@100.110.215.46}
ELITE_REPO=${ELITE_REPO:-/home/phantomojo/GhostWire-secure-mesh-communication-newest}
ELITE_TARGET=${ELITE_TARGET:-/home/phantomojo/.cache/ghostwire-verified-target}
ELITE_PORT=${ELITE_PORT:-9918}
ELITE_MESH_PORT=${ELITE_MESH_PORT:-4822}
ELITE_QUIC_PORT=${ELITE_QUIC_PORT:-8902}
ELITE_HOME=${ELITE_HOME:-/tmp/ghostwire-manual-elite}
OMEN_PORT=${OMEN_PORT:-9917}
OMEN_MESH_PORT=${OMEN_MESH_PORT:-4821}
OMEN_QUIC_PORT=${OMEN_QUIC_PORT:-8901}
OMEN_HOME=${OMEN_HOME:-/tmp/ghostwire-manual-omen}
PROMOTE_ISOLATED=${PROMOTE_ISOLATED:-1}
PROMOTE_PRODUCTION=0
JOBS=${JOBS:-2}
AFFINITY=${AFFINITY:-0-7}
STAMP=$(date -u +%Y%m%dT%H%M%SZ)
RELEASE_DIR=${ARTIFACT_ROOT}/${STAMP}

log() { printf '[ghostwire-release] %s\n' "$*"; }
die() { printf '[ghostwire-release] ERROR: %s\n' "$*" >&2; exit 1; }

while [ "$#" -gt 0 ]; do
  case "$1" in
    --no-promote-isolated) PROMOTE_ISOLATED=0 ;;
    --promote-production) PROMOTE_PRODUCTION=1 ;;
    --jobs) JOBS="$2"; shift ;;
    --affinity) AFFINITY="$2"; shift ;;
    --help)
      cat <<'USAGE'
Usage: ghostwire_verified_dual_release.sh [--no-promote-isolated] [--promote-production] [--jobs N] [--affinity LIST]

Builds the current tracked source and Web UI, embeds release identity, builds Omen with bounded Cargo concurrency,
synchronizes tracked source only (never identity stores), builds EliteKali natively, and verifies binary/asset manifests.
By default it may promote only the isolated manual-pair daemons (Omen 9917 and EliteKali 9918).
Production Omen promotion is opt-in with --promote-production and must still pass the explicit readiness/rollback gate.
USAGE
      exit 0
      ;;
    *) die "unknown argument: $1" ;;
  esac
  shift
done

[ "$JOBS" -le 2 ] || die "JOBS must be <= 2"
mkdir -p "$RELEASE_DIR"
cd "$REPO"
SOURCE_REV=$(git rev-parse HEAD)
[ -n "$SOURCE_REV" ] || die "cannot determine source revision"

build_asset_manifest() {
  local out="$1"
  (cd "$WEB/dist" && LC_ALL=C find . -type f -printf '%P\n' | LC_ALL=C sort | while IFS= read -r f; do sha256sum "$f"; done) | sha256sum | awk '{print $1}' > "$out"
}

log "building Web UI at source ${SOURCE_REV:0:12}"
nice -n 10 pnpm --dir "$WEB" build
build_asset_manifest "/tmp/gw-asset-hash-${STAMP}"
ASSET_HASH=$(cat "/tmp/gw-asset-hash-${STAMP}")
export GHOSTWIRE_ASSET_MANIFEST_HASH="$ASSET_HASH"
export GHOSTWIRE_GIT_HEAD="$SOURCE_REV"

log "building Omen artifact with CARGO_BUILD_JOBS=${JOBS}, affinity=${AFFINITY}"
env CARGO_TARGET_DIR="$OMEN_TARGET" CARGO_BUILD_JOBS="$JOBS" taskset -c "$AFFINITY" nice -n 10 cargo build --manifest-path "$GW/Cargo.toml" --release --bin ghostwire --features reticulum
cp "$OMEN_TARGET/release/ghostwire" "$RELEASE_DIR/ghostwire-omen"
chmod 0755 "$RELEASE_DIR/ghostwire-omen"
OMEN_HASH=$(sha256sum "$RELEASE_DIR/ghostwire-omen" | awk '{print $1}')
GLIBC_MAX=$(readelf --version-info "$RELEASE_DIR/ghostwire-omen" 2>/dev/null | grep -oE 'GLIBC_[0-9.]+' | sort -Vu | tail -1 || true)

cat > "$RELEASE_DIR/manifest.json" <<EOF
{
  "source_revision": "$SOURCE_REV",
  "asset_manifest_hash": "$ASSET_HASH",
  "omen_binary_hash": "$OMEN_HASH",
  "glibc_max": "$GLIBC_MAX",
  "built_at_utc": "$STAMP",
  "jobs": $JOBS,
  "affinity": "$AFFINITY",
  "production_promotion_requested": $PROMOTE_PRODUCTION
}
EOF
sha256sum "$RELEASE_DIR/manifest.json" > "$RELEASE_DIR/manifest.sha256"
log "Omen artifact hash ${OMEN_HASH:0:16}; asset manifest ${ASSET_HASH:0:16}"

# Synchronize tracked source only to EliteKali for a native build. This uses git archive and never copies GHOSTWIRE_HOME,
# profile directories, token files, database files, or private identity material.
log "syncing tracked source archive to EliteKali for native build"
git ls-files -z | tar --null -T - -cf - | ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "mkdir -p '$ELITE_REPO' && tar -xf - -C '$ELITE_REPO'"
# EliteKali does not have pnpm installed. Transfer only the built, non-secret Web UI dist tree;
# identity stores, profiles, tokens, and databases are never copied.
tar -C "$WEB" -cf - dist | ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "mkdir -p '$ELITE_REPO/webui' && rm -rf '$ELITE_REPO/webui/dist' && tar -xf - -C '$ELITE_REPO/webui'"
cat "$RELEASE_DIR/manifest.json" | ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "mkdir -p '$ELITE_REPO/.verified-release/$STAMP' && cat > '$ELITE_REPO/.verified-release/$STAMP/omen-manifest.json'"

log "building EliteKali natively with CARGO_BUILD_JOBS=${JOBS}, affinity=${AFFINITY}"
ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "set -Eeuo pipefail; cd '$ELITE_REPO'; test -f webui/dist/index.html; asset_hash=\$(cd webui/dist && LC_ALL=C find . -type f -printf '%P\\n' | LC_ALL=C sort | while IFS= read -r f; do sha256sum \"\$f\"; done | sha256sum | awk '{print \$1}'); export GHOSTWIRE_ASSET_MANIFEST_HASH=\"\$asset_hash\"; export GHOSTWIRE_GIT_HEAD='$SOURCE_REV'; env CARGO_TARGET_DIR='$ELITE_TARGET' CARGO_BUILD_JOBS='$JOBS' taskset -c '$AFFINITY' nice -n 10 cargo build --manifest-path ghostwire/Cargo.toml --release --bin ghostwire --features reticulum; cp '$ELITE_TARGET/release/ghostwire' '$ELITE_REPO/.verified-release/$STAMP/ghostwire-elitekali'; sha256sum '$ELITE_REPO/.verified-release/$STAMP/ghostwire-elitekali' > '$ELITE_REPO/.verified-release/$STAMP/elitekali.binary.sha256'; printf '%s\\n' \"\$asset_hash\" > '$ELITE_REPO/.verified-release/$STAMP/elitekali.asset.sha256'"
scp -q -o BatchMode=yes "$ELITE_HOST:$ELITE_REPO/.verified-release/$STAMP/ghostwire-elitekali" "$RELEASE_DIR/ghostwire-elitekali"
scp -q -o BatchMode=yes "$ELITE_HOST:$ELITE_REPO/.verified-release/$STAMP/elitekali.binary.sha256" "$RELEASE_DIR/elitekali.binary.sha256"
scp -q -o BatchMode=yes "$ELITE_HOST:$ELITE_REPO/.verified-release/$STAMP/elitekali.asset.sha256" "$RELEASE_DIR/elitekali.asset.sha256"
ELITE_HASH=$(sha256sum "$RELEASE_DIR/ghostwire-elitekali" | awk '{print $1}')
ELITE_ASSET_HASH=$(cat "$RELEASE_DIR/elitekali.asset.sha256")
[ "$ASSET_HASH" = "$ELITE_ASSET_HASH" ] || die "asset manifest mismatch: Omen=$ASSET_HASH EliteKali=$ELITE_ASSET_HASH"
log "EliteKali native artifact hash ${ELITE_HASH:0:16}; asset manifest matches"

cat > "$RELEASE_DIR/dual-manifest.json" <<EOF
{
  "source_revision": "$SOURCE_REV",
  "asset_manifest_hash": "$ASSET_HASH",
  "omen_binary_hash": "$OMEN_HASH",
  "elitekali_binary_hash": "$ELITE_HASH",
  "built_at_utc": "$STAMP",
  "isolated_ports": {"omen_api": $OMEN_PORT, "elitekali_api": $ELITE_PORT},
  "production_promotion_requested": $PROMOTE_PRODUCTION
}
EOF

verify_binary() {
  local path="$1" label="$2"
  "$path" --version >/tmp/gw-version-${STAMP}-$(basename "$path") 2>&1 || die "$label binary --version failed"
  test -x "$path" || die "$label artifact is not executable"
  log "$label version check: $(head -1 "/tmp/gw-version-${STAMP}-$(basename "$path")")"
}
verify_binary "$RELEASE_DIR/ghostwire-omen" Omen
verify_binary "$RELEASE_DIR/ghostwire-elitekali" EliteKali

wait_ready() {
  local port="$1" label="$2"
  for _ in $(seq 1 45); do
    if curl -fsS --max-time 2 "http://127.0.0.1:${port}/api/v2/ready" >/dev/null 2>&1; then
      log "$label readiness passed on ${port}"
      return 0
    fi
    sleep 1
  done
  die "$label readiness failed on ${port}"
}

check_runtime_manifest() {
  local port="$1" token_path="$2" label="$3"
  [ -f "$token_path" ] || die "$label token path missing: $token_path"
  local token body code
  token=$(cat "$token_path")
  body=$(mktemp)
  code=$(curl -sS -o "$body" -w '%{http_code}' -H "X-Ghost-Token: $token" "http://127.0.0.1:${port}/api/v2/system/version" || true)
  [ "$code" = 200 ] || die "$label version endpoint returned HTTP $code"
  grep -q "\\\"asset_manifest_hash\\\":\\\"$ASSET_HASH\\\"" "$body" || die "$label embedded asset manifest mismatch"
  grep -q "\\\"git_revision\\\":\\\"$SOURCE_REV\\\"" "$body" || die "$label embedded source revision mismatch"
  rm -f "$body"
  log "$label runtime manifest matches source ${SOURCE_REV:0:12} and assets ${ASSET_HASH:0:12}"
}

promote_omen_isolated() {
  local target=/tmp/ghostwire-world-discovery-bound-release/release/ghostwire
  local backup="$RELEASE_DIR/omen-isolated.previous"
  local pid
  pid=$(ps -eo pid=,args= | awk '$0 ~ /ghostwire --daemon --port 9917 --api-host 127.0.0.1 --mesh-port 4821 --quic-port 8901/ {print $1; exit}')
  [ -n "$pid" ] || die "intended Omen isolated daemon 9917 not found"
  cp -f "$target" "$backup"
  kill -TERM "$pid"
  for _ in $(seq 1 20); do kill -0 "$pid" 2>/dev/null || break; sleep 1; done
  install -m 0755 "$RELEASE_DIR/ghostwire-omen" "$target"
  nohup env GHOSTWIRE_HOME="$OMEN_HOME" "$target" --daemon --port "$OMEN_PORT" --api-host 127.0.0.1 --mesh-port "$OMEN_MESH_PORT" --quic-port "$OMEN_QUIC_PORT" > "$OMEN_HOME/daemon.log" 2>&1 &
  wait_ready "$OMEN_PORT" Omen-isolated
  check_runtime_manifest "$OMEN_PORT" "$OMEN_HOME/token" Omen-isolated
}

promote_elitekali_isolated() {
  local target="$ELITE_REPO/target/release/ghostwire"
  local remote_artifact="/tmp/ghostwire-verified-${STAMP}"
  ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "cp -f '$target' '$ELITE_REPO/.verified-release/$STAMP/elitekali.previous'; install -m 0755 '$ELITE_REPO/.verified-release/$STAMP/ghostwire-elitekali' '$target'; pid=\$(ps -eo pid=,args= | awk '\$0 ~ /ghostwire --daemon --port 9918 --api-host 127.0.0.1 --mesh-port 4822 --quic-port 8902/ {print \$1; exit}'); test -n \"\$pid\"; kill -TERM \"\$pid\"; for i in \$(seq 1 20); do kill -0 \"\$pid\" 2>/dev/null || break; sleep 1; done; nohup env GHOSTWIRE_HOME='$ELITE_HOME' '$target' --daemon --port '$ELITE_PORT' --api-host 0.0.0.0 --mesh-port '$ELITE_MESH_PORT' --quic-port '$ELITE_QUIC_PORT' --bootstrap /ip4/100.88.234.14/tcp/4821/p2p/12D3KooWG9Y16M3DjsuKSDR7Uuuf3gLnvsswqA6sDF6cwfW6dYjE > '$ELITE_HOME/daemon.log' 2>&1 &"
  ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "for i in \$(seq 1 45); do curl -fsS --max-time 2 http://127.0.0.1:$ELITE_PORT/api/v2/ready >/dev/null 2>&1 && curl -fsS --max-time 2 http://100.110.215.46:$ELITE_PORT/api/v2/ready >/dev/null 2>&1 && exit 0; sleep 1; done; exit 1" || die "EliteKali isolated readiness failed"
  # Verify the remote runtime manifest through its real X-Ghost-Token header.
  ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "token=\$(cat '$ELITE_HOME/token'); body=\$(mktemp); code=\$(curl -sS -o \"\$body\" -w '%{http_code}' -H \"X-Ghost-Token: \$token\" http://127.0.0.1:$ELITE_PORT/api/v2/system/version); test \"\$code\" = 200; grep -q '\"asset_manifest_hash\":\"$ASSET_HASH\"' \"\$body\"; grep -q '\"git_revision\":\"$SOURCE_REV\"' \"\$body\"; rm -f \"\$body\"" || die "EliteKali runtime manifest mismatch"
  ssh -o BatchMode=yes -o ConnectTimeout=25 "$ELITE_HOST" "token=\$(cat '$ELITE_HOME/token'); if curl -fsS -H \"X-Ghost-Token: \$token\" http://127.0.0.1:$ELITE_PORT/api/identity/status | grep -q '\"locked\":false'; then echo ELITE_PROFILE_UNLOCKED; else echo ELITE_PROFILE_RELOCK_REQUIRED; fi"
  log "EliteKali isolated runtime manifest matches source ${SOURCE_REV:0:12} and assets ${ASSET_HASH:0:12}"
}

if [ "$PROMOTE_ISOLATED" = 1 ]; then
  log "promoting only the intended isolated manual-pair daemons"
  # The Elite artifact is already staged remotely by the native build block.
  promote_omen_isolated
  promote_elitekali_isolated
fi

if [ "$PROMOTE_PRODUCTION" = 1 ]; then
  [ "${GHOSTWIRE_PRODUCTION_CONFIRM:-}" = "I_UNDERSTAND_ROLLBACK" ] || die "production promotion requires GHOSTWIRE_PRODUCTION_CONFIRM=I_UNDERSTAND_ROLLBACK"
  die "production promotion is deliberately not automated in this isolated workflow; use the reviewed release gate"
fi

log "verified release ready at $RELEASE_DIR"
printf '%s\n' "$RELEASE_DIR"
