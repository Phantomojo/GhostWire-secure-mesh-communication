# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅        |

## Reporting a Vulnerability

We take the security of GhostWire seriously. If you believe you have found a security vulnerability, please report it responsibly:

1. **Email**: mirungu015@proton.me
2. **PGP Key**: Request via email
3. **Response time**: Within 48 hours

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### What to Expect

- **Acknowledgment**: Within 48 hours
- **Assessment**: Within 7 days
- **Fix timeline**: Depends on severity
  - Critical: 7 days
  - High: 14 days
  - Medium: 30 days
  - Low: Next release

### Scope

- Cryptographic implementations (Sphinx, DTN encryption)
- CI/CD pipeline security
- Crate publishing integrity
- Secret management
- Dependency supply chain

### Out of Scope

- Issues in dependencies (report to upstream maintainers)
- Social engineering attacks
- Physical security

## Security Best Practices

- All actions in CI/CD are SHA-pinned
- Branch protection enabled on `main`
- Least-privilege workflow permissions
- Egress monitoring via `step-security/harden-runner`
- No self-hosted runners (GitHub-hosted only)
