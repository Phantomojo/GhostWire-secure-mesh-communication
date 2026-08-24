# GhostWire Agent Handoff Template

Copy this template into `.ai/handoffs/<date>-<topic>.md` at the end of every meaningful work block.

## Objective

State the exact objective and the user-visible claim being changed.

## Evidence before change

Record the failing test, runtime probe, log line, source path, or external source that established the problem. State confidence and whether the evidence is measured, verified, inferred, estimated, simulated, unknown, or stale.

## Authority and boundaries

State which subsystem owns the decision and which subsystems are explicitly not allowed to override it. Record privacy class, authorization scope, identity/profile scope, and whether the change touches cryptographic or migration semantics.

## Root cause

Explain the mechanism that was broken. If this is only a workaround, name the real fix, its source location, and why the workaround is temporary.

## Change

List modified files and summarize the semantic change. Do not describe compilation as behavior verification.

## Verification

Record exact commands and observed results for the tight reproducer, relevant tests, aggregate tests, isolated runtime, and two-machine runtime if applicable. Include build warnings or known failures.

## Process and identity safety

Record production PID/ports, isolated PID/ports, `GHOSTWIRE_HOME`, profile, identity persistence, and whether any daemon or persistent store was restarted or mutated.

## Deferred risks

List unresolved defects, false-confidence surfaces, privacy implications, resource/DoS risks, rollout/migration risks, and tests still required.

## Next action

Give one exact next command or one precise implementation step. Do not leave the next agent to infer priorities.

## Status block

```text
STATUS REPORT
─────────────
✅ Done:      [specific behavior verified and how]
⚠️  Issue:    [remaining issue and root cause]
❌ Deferred:  [not done and why]
📋 Next:      [exact next action]

HANDOFF — [UTC date]
Compile status: [clean / failures]
Files modified: [list]
Files needing attention: [list + reason]
Resume from: [exact next action]
```
