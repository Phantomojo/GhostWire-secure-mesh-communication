# Security Policy

Canonical policy: [`ghostwire/SECURITY.md`](../ghostwire/SECURITY.md). This file exists so GitHub's security tab
has a supported entry point; it does not replace that document's details.

## Supported Versions
The only currently supported development lineage is the repo's current rescue
lineage. All previously shipped or public artefacts are unsupported and should be
replaced.

## Reporting a Vulnerability
1. Prefer GitHub private vulnerability reporting or email `security@ghostwire.cc`.
2. Do not open a public issue and wait for a fix.
3. Include description, reproduction, impact, OS/GhostWire version, PoC, and
   disclosure timeline.

## Security Reality
- GhostWire is pre-1.0 and has had no external security review.
- Treat claims in this project as design intent, not verified properties.
- The current codebase is experimental. There is no deployed network or released
  security-supported product here.

## Response Expectations
- Acknowledgement: within 48 hours
- Assessment: within 7 days
- Low: next release; medium: 30 days; high: 14 days; critical: 7 days
- Do not create a public issue before a fix exists.

## Advisories
We aim to request a CVE through private GitHub advisories only when a fix is
available and described accurately.
