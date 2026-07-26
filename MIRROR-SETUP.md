# Mirror Setup Guide

## GitLab Token Fix

Your current token is missing the `Code: Push` permission.

### Steps:

1. Go to **gitlab.com → Settings → Access Tokens**
2. Click **Add new token**
3. Configure:
   - Name: `ghostwire-mirror`
   - Expiration: 1 year from now
   - Resource: **Specific scopes** → **Projects** → select `Phantomojo/GhostWire-secure-mesh-communication`
   - Permission: **Code: Push** ✅
4. Click **Create personal access token**
5. Copy the new token

### Update remote:

```bash
git remote set-url gitlab https://Phantomojo:NEW_TOKEN@gitlab.com/Phantomojo/GhostWire-secure-mesh-communication.git
```

### Test:

```bash
git push gitlab main --force
```

---

## Codeberg Storage Exception

Your repo is 982 MiB. Codeberg limit is 750 MiB.

### Steps:

1. Go to **codeberg.org/Codeberg-e.V./requests**
2. Create a new issue with:

**Title:** Storage exception request for GhostWire-secure-mesh-communication

**Body:**

```
Hello Codeberg team,

I'm requesting a storage exception for my FLOSS project:

- Repository: https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication
- Current size: ~982 MiB (imported from GitHub)
- License: MIT
- Description: Decentralized mesh communication protocol with ML-KEM-768 encryption

The repository was imported from GitHub and contains:
- Rust/TypeScript/Python source code (~340 commits)
- Test data and fixtures
- Documentation and articles

This is a legitimate open-source project for censorship-resistant communication. The size is due to the GitHub import carrying full history. I'm happy to optimize if needed.

Thank you for supporting FLOSS!
```

### Alternative: Reduce repo size

If you want to reduce the size, you can rewrite history to remove large files:

```bash
# Install git-filter-repo
pip install git-filter-repo

# Find large files
git rev-list --objects --all | \
  git cat-file --batch-check='%(objecttype) %(objectsize) %(rest)' | \
  sed -n 's/^blob //p' | \
  sort -rnk1 | head -20

# Remove specific large files from history
git filter-repo --path-glob '*.zip' --invert-paths
git filter-repo --path-glob '*.tar.gz' --invert-paths

# Force push cleaned history
git push origin main --force
git push gitlab main --force
git push codeberg main --force
```

---

## GitHub Actions Auto-Mirror

Already set up in `.github/workflows/mirror.yml`.

### Required secrets:

Add these in **GitHub → Settings → Secrets → Actions**:

1. **GITLAB_TOKEN** — the new fine-grained token with `Code: Push`
2. **CODEBERG_TOKEN** — existing token: `ba184c12445885b43550a41a381dc17d2a143a3b`

### How it works:

Every push to `main`, `develop`, or `security/*` branches automatically triggers:
- Push to GitLab
- Push to Codeberg

No manual mirroring needed.

---

## Legal Compliance Summary

### GitLab ✅
- Encryption export: NIST FIPS standardized, publicly available → exempt under 15 CFR §740.13(e)
- Acceptable Use: No restrictions on encryption tools using standardized algorithms

### Codeberg ⚠️
- AI-generated code: Restricted if "mostly written by AI tools" → you're fine (human-written with AI assistance)
- Harmful code: Only allowed in security research context → GhostWire is communication tool, not malware
- Repo size: Need exception request (see above)

### What to Watch
- Codeberg may ban crypto/mining projects (harmful to reputation clause)
- GhostWire is communication, not crypto — you're fine

---

## Checklist

- [ ] Create new GitLab fine-grained token with `Code: Push`
- [ ] Update git remote with new token
- [ ] Request Codeberg storage exception
- [ ] Add GitHub Actions secrets (GITLAB_TOKEN, CODEBERG_TOKEN)
- [ ] Test push to all three platforms
