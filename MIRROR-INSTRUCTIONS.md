# GhostWire Mirror Setup Instructions

This document provides instructions for mirroring the GhostWire repository to multiple platforms for redundancy and censorship resistance.

## Why Mirror?

- **Censorship resistance**: If one platform complies with a takedown, code survives elsewhere
- **Geographic availability**: Some platforms may be blocked in certain regions
- **Backup**: Protect against account suspension or platform issues

## Recommended Mirrors

### 1. GitLab (Primary Mirror)

**Why GitLab**: Largest open-source platform after GitHub, strong community, good CI/CD

**Setup:**
```bash
# 1. Create account at https://gitlab.com
# 2. Create new project: https://gitlab.com/projects/new
# 3. Choose "Import project" → "GitHub"
# 4. Authorize GitLab to access your GitHub repositories
# 5. Select "GhostWire-secure-mesh-communication"
# 6. Click "Import"
```

**Manual mirror (if not using import):**
```bash
# Add GitLab remote
git remote add gitlab https://gitlab.com/Phantomojo/GhostWire-secure-mesh-communication.git

# Push all branches and tags
git push gitlab --all
git push gitlab --tags
```

**CI/CD**: GitLab CI automatically mirrors GitHub Actions workflows.

### 2. Codeberg (Decentralized Mirror)

**Why Codeberg**: Non-profit, EU-based, community-governed, resistant to corporate pressure

**Setup:**
```bash
# 1. Create account at https://codeberg.org
# 2. Create new repository: https://codeberg.org/repos/new
# 3. Name it "GhostWire"
# 4. Push to Codeberg:
git remote add codeberg https://codeberg.org/Phantomojo/GhostWire.git
git push codeberg --all
git push codeberg --tags
```

### 3. IPFS (Permanent Mirror)

**Why IPFS**: Content-addressed, permanent, cannot be removed

**Setup:**
```bash
# Install IPFS Desktop or CLI
# Initialize IPFS node
ipfs init

# Add repository to IPFS
cd /path/to/GhostWire-secure-mesh-communication
ipfs add -r . --pin

# Pin to multiple pinning services for reliability
# Options: Pinata, Infura, Crust, Filecoin
```

### 4. Archive.org (Wayback Machine)

**Why Archive.org**: Non-profit, permanent archive, legal protections

**Setup:**
```bash
# Create archive of repository
# https://archive.org/create

# Or use the Save Page Now feature:
# https://web.archive.org/save/https://github.com/Phantomojo/GhostWire-secure-mesh-communication
```

## Automated Mirror Sync

### GitHub Actions Workflow

Create `.github/workflows/mirror.yml`:

```yaml
name: Mirror to GitLab

on:
  push:
    branches: [main, develop]
  workflow_dispatch:

jobs:
  mirror:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Mirror to GitLab
        uses: wearerequired/git-mirror-action@v1
        env:
          SSH_PRIVATE_KEY: ${{ secrets.GITLAB_SSH_KEY }}
        with:
          source-repo: 'git@github.com:Phantomojo/GhostWire-secure-mesh-communication.git'
          destination-repo: 'git@gitlab.com:Phantomojo/GhostWire-secure-mesh-communication.git'
```

**Setup:**
1. Generate SSH key: `ssh-keygen -t ed25519 -C "mirror"`
2. Add public key to GitLab: Settings → SSH Keys
3. Add private key to GitHub: Settings → Secrets → `GITLAB_SSH_KEY`

## Maintenance

### Weekly sync check
```bash
# Verify all mirrors are current
git remote -v
git fetch --all
git log --oneline origin/main..gitlab/main
git log --oneline origin/main..codeberg/main
```

### After major releases
```bash
# Push tags to all mirrors
git push gitlab --tags
git push codeberg --tags
```

## Legal Considerations

Mirroring does not change licensing terms. All mirrors must maintain:
- AGPL-3.0 license file
- NOTICE file with Genesis ceremony documentation
- Export compliance notice in README
- All copyright notices

## Contact

For mirror issues or requests:
- Email: infrastructure@ghostwire.cc
- GitHub Issues: https://github.com/Phantomojo/GhostWire-secure-mesh-communication/issues

---

**Note:** These mirrors provide redundancy, not anonymity. If you need anonymous distribution, consider using IPFS with privacy-preserving pinning services.
