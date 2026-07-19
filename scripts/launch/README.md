# Launch Scripts

This directory contains GhostWire launch and startup scripts.

## Organization

Scripts are linked from the parent directory for organization:
- `RUN_GHOSTWIRE.sh` → Main launcher
- `launch-ghostwire-working.sh` → Working launcher variant
- `launch_swarm.sh` → Swarm mode launcher
- `start-ghostwire-bg.sh` → Background launcher
- `start-ghostwire-debug.sh` → Debug launcher
- `start-pc-a.sh`, `start-pc-b.sh` → PC-specific launchers

## Usage

```bash
# Standard launch
./scripts/launch/RUN_GHOSTWIRE.sh

# Background launch
./scripts/launch/start-ghostwire-bg.sh

# Debug mode
./scripts/launch/start-ghostwire-debug.sh
```

## Notes

Most launch scripts are variations that configure different runtime modes.
The `RUN_GHOSTWIRE.sh` is the primary entry point.
