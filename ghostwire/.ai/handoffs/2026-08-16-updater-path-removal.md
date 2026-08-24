# GhostWire Agent Handoff — Legacy updater path removal

## Objective

Remove the legacy shell-based updater from the reachable module graph. The user-visible security claim is that GhostWire updates are handled only by the signed, hash-verified updater and never by executing a repository-local  through .

## Evidence before change

Measured and verified on Omen before the change. The red reproducer:



found  and  invoking .  already used , but the legacy module remained publicly reachable. Confidence: verified source evidence.

## Authority and boundaries

The signed updater in  owns update eligibility, manifest signature verification, HTTPS download policy, BLAKE3 verification, and rollback behavior. The deleted legacy module is not allowed to execute commands or mutate the installation. This change does not regenerate identities, alter cryptographic identity semantics, restart daemons, or touch production ports. Privacy class: local update metadata and filesystem state.

## Root cause

The secure updater had been added but the old  declaration and  remained. That preserved a command-execution path which could invoke a repository-local shell script independently of the signed updater contract.

## Change

Modified the dirty canonical checkout:

- Removed  from .
- Deleted , including usage: git [-v | --version] [-h | --help] [-C <path>] [-c <name>=<value>]
           [--exec-path[=<path>]] [--html-path] [--man-path] [--info-path]
           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--no-lazy-fetch]
           [--no-optional-locks] [--no-advice] [--bare] [--git-dir=<path>]
           [--work-tree=<path>] [--namespace=<name>] [--config-env=<name>=<envvar>]
           <command> [<args>]

These are common Git commands used in various situations:

start a working area (see also: git help tutorial)
   clone      Clone a repository into a new directory
   init       Create an empty Git repository or reinitialize an existing one

work on the current change (see also: git help everyday)
   add        Add file contents to the index
   mv         Move or rename a file, a directory, or a symlink
   restore    Restore working tree files
   rm         Remove files from the working tree and from the index

examine the history and state (see also: git help revisions)
   bisect     Use binary search to find the commit that introduced a bug
   diff       Show changes between commits, commit and working tree, etc
   grep       Print lines matching a pattern
   log        Show commit logs
   show       Show various types of objects
   status     Show the working tree status

grow, mark and tweak your common history
   backfill   Download missing objects in a partial clone
   branch     List, create, or delete branches
   commit     Record changes to the repository
   history    EXPERIMENTAL: Rewrite history
   merge      Join two or more development histories together
   rebase     Reapply commits on top of another base tip
   reset      Set `HEAD` or the index to a known state
   switch     Switch branches
   tag        Create, list, delete or verify tags

collaborate (see also: git help workflows)
   fetch      Download objects and refs from another repository
   pull       Fetch from and integrate with another repository or a local branch
   push       Update remote refs along with associated objects

'git help -a' and 'git help -g' list available subcommands and some
concept guides. See 'git help <command>' or 'git help <concept>'
to read about a specific subcommand or concept.
See 'git help git' for an overview of the system., , and  execution.

No other files were intentionally changed in this work block.

## Verification

The exact red reproducer failed before the change with . The same reproducer passed after the change with .

Commands and results:



The focused mailbox suite was also run before this work block: 4 passed, 0 failed, including associated-data tamper rejection and malformed nonce rejection.

## Process and identity safety

Remote identity was verified as  on the Omen target through . The host reports  as its hostname, but the authenticated user, canonical path, production process, and configured checkout matched Omen instructions. Production PID 932431 remained running as {"timestamp":"2026-08-16T22:37:52.400055Z","level":"INFO","fields":{"message":"🚀 Starting GhostWire - Secure Mesh Networking and Messaging"},"target":"ghostwire","threadName":"main","threadId":"ThreadId(1)"}
{"timestamp":"2026-08-16T22:37:52.529970Z","level":"INFO","fields":{"message":"🔐 Loaded persistent encryption keys for key_id: key_6a7b425a"},"target":"ghostwire::core::encryption","threadName":"main","threadId":"ThreadId(1)"}
{"timestamp":"2026-08-16T22:37:52.530652Z","level":"INFO","fields":{"message":"TOFU: loaded 2 fingerprints from /home/ph/.ghostwire/tofu_fingerprints.json"},"target":"ghostwire::core::quic_transport","threadName":"main","threadId":"ThreadId(1)"}
{"timestamp":"2026-08-16T22:37:52.530663Z","level":"INFO","fields":{"message":"QUIC transport started on 0.0.0.0:0 (fingerprint: 497aba9627a7334c)"},"target":"ghostwire::core::quic_transport","threadName":"main","threadId":"ThreadId(1)"}
{"timestamp":"2026-08-16T22:37:52.530677Z","level":"INFO","fields":{"message":"QUIC transport registered on port 8081"},"target":"ghostwire::core","threadName":"main","threadId":"ThreadId(1)"}. Isolated instances remained on ports 9397/4301/8381, 9497/4401/8481, and 9597/4501/8581. No daemon was restarted. No  or identity store was changed.

## Deferred risks

The aggregate test suite has a pre-existing hang or long-running network test that must be isolated before the suite can be called green. Clippy remains far from warning-clean. The signed updater still requires an actual configured release signing key and binary hash at runtime; the build emitted warnings that these are disabled in the current environment. Updater installation semantics still need a dedicated isolated filesystem test for temp-file cleanup, backup collision, permissions, and restart/rollback behavior.

## Next action

Inspect and implement the secure-wipe path in an isolated test profile, beginning with a red test that proves the previous clear operation leaves persisted message data behind, while preserving production PID 932431 and all production identity material.

## Status block
