# Installation Scripts

This directory contains GhostWire installation scripts.

## Organization

Scripts are linked from the parent directory for organization:
- `install.sh` → Main installer (in parent)
- `install-for-omarchy.sh` → Platform-specific installer
- `install-system-monitor.sh` → System monitoring setup
- `install-dev-tools.sh` → Developer tools installation
- `install_hostname.sh` → Hostname configuration

## Usage

```bash
# Standard installation
./scripts/install.sh

# Platform-specific
./scripts/install/install-for-omarchy.sh

# With system monitoring
./scripts/install/install-system-monitor.sh
```

## Notes

The main `install.sh` is the canonical installer. Other scripts are specialized variants.
