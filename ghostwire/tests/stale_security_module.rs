//! `core/security.rs` is a stale predecessor and must not be mistaken for the
//! live security module.
//!
//! ## Why this test exists
//!
//! Four independent sources in Michael's own corpus call it a stale
//! predecessor:
//!
//!   * `GHOSTWIRE_LEDGER.md:613` — "security.rs (755) which is a STALE
//!     PREDECESSOR of the live security_v2.rs"
//!   * `GHOSTWIRE_LEDGER.md:865` (F9) — "a stale uncompiled predecessor
//!     exposing an API that does not exist at runtime"
//!   * `GHOSTWIRE_REMEDIATION_PLAN_20261002.md:310` — "reading it sends you
//!     hunting for methods that don't exist at runtime"
//!
//! And the conclusion seemed obvious: delete it. That conclusion was WRONG on
//! first inspection, and the reason it looked wrong is worth recording.
//!
//! ## The trap
//!
//! Three live files reference `crate::core::security::SecurityManager`,
//! `::SecurityStats` and `::ThreatLevel`:
//!
//!   * `core/mesh.rs:1235`     `-> crate::core::security::SecurityStats`
//!   * `controllers/system.rs:29` `threat_level: crate::core::security::ThreatLevel`
//!
//! So `security.rs` looks very much alive, and a name-level grep cannot decide
//! it: every type it defines (`SecurityManager`, `SecurityConfig`, `ThreatLevel`,
//! `SecurityStats`, `PersistentBlacklistManager`, `ProofOfWorkSybilDefense`,
//! `SlidingWindowQuotaEnforcer`, `EntityType`, `SecurityEvent`) ALSO exists in
//! `security_v2.rs`, and every method name (`block_ip`, `is_ip_allowed`,
//! `record_message`, `get_config`) is defined in both. Bare-name greps return
//! hits on both sides and cannot distinguish them.
//!
//! ## The resolution
//!
//! `core/mod.rs` declares only `pub mod security_v2;` — `security.rs` is NOT
//! declared as a module anywhere. And line 54:
//!
//!     pub use security_v2 as security;
//!
//! `crate::core::security` is an ALIAS for `security_v2`. Every one of those
//! three call sites is using `security_v2` through a renamed path. So the file
//! is genuinely unreferenced — but only knowable by reading the module graph,
//! not by grepping symbols.
//!
//! This test pins the alias, because it is the load-bearing fact. If someone
//! "cleans up" by removing the alias, the three call sites stop resolving. If
//! someone adds `pub mod security;`, two `security` names collide and it is
//! ambiguous which one wins.

use std::fs;

fn core_mod() -> String {
    fs::read_to_string("src/core/mod.rs").expect("src/core/mod.rs must be readable")
}

#[test]
fn security_rs_is_not_declared_as_a_module() {
    let src = core_mod();

    // Matched by LINES THAT DECLARE IT, not by substring. The comment in
    // mod.rs may legitimately name the file, and a bare `contains` would then
    // fail against correct code.
    let declares_v1 = src
        .lines()
        .any(|l| !l.trim_start().starts_with("//") && l.trim() == "pub mod security;");
    assert!(
        !declares_v1,
        "core/security.rs must not be declared as a module — it is a stale \
         predecessor of security_v2 and declaring it would make `security` \
         ambiguous"
    );

    assert!(
        src.lines().any(|l| l.trim() == "pub mod security_v2;"),
        "security_v2 must be declared — it is the live module"
    );
}

#[test]
fn security_is_an_alias_for_security_v2() {
    // THE load-bearing fact. `crate::core::security::SecurityManager` in
    // stealth_tcp.rs, `::SecurityStats` in mesh.rs and `::ThreatLevel` in
    // controllers/system.rs all resolve through this alias.
    //
    // Without it, `crate::core::security` names nothing and those three files
    // fail to compile.
    let src = core_mod();
    assert!(
        src.contains("pub use security_v2 as security;"),
        "core/mod.rs must alias security_v2 as `security`. Three live files \
         (stealth_tcp.rs, mesh.rs, controllers/system.rs) reference \
         crate::core::security::*, which resolves ONLY through this alias — \
         core/security.rs is not a declared module."
    );
}

#[test]
fn the_three_live_call_sites_go_through_the_alias() {
    // Pins WHY the alias matters: if these three stopped referencing
    // `core::security::`, the alias could be removed and the stale file
    // deleted without breaking anything. That is the day the cleanup becomes
    // possible, and this test should be the thing that notices.
    let stealth = fs::read_to_string("src/core/stealth_tcp.rs").expect("stealth_tcp.rs");
    let mesh = fs::read_to_string("src/core/mesh.rs").expect("mesh.rs");
    let system = fs::read_to_string("src/controllers/system.rs").expect("system.rs");

    assert!(
        stealth.contains("crate::core::security::SecurityManager"),
        "stealth_tcp.rs is expected to reach SecurityManager through the alias"
    );
    assert!(
        mesh.contains("crate::core::security::SecurityStats"),
        "mesh.rs is expected to reach SecurityStats through the alias"
    );
    assert!(
        system.contains("crate::core::security::ThreatLevel"),
        "controllers/system.rs is expected to reach ThreatLevel through the alias"
    );
}

/// The stale file must stay a stale file. This asserts the *documentation*
/// property rather than the code property: someone reading `security.rs`
/// should not be able to mistake it for the live module, because every symbol
/// it defines also exists in `security_v2.rs` with the same name.
#[test]
fn the_stale_file_is_marked_as_stale() {
    let src = fs::read_to_string("src/core/security.rs").expect("src/core/security.rs");

    let head: String = src.chars().take(2000).collect();
    let marked = head.contains("STALE")
        || head.contains("superseded")
        || head.contains("Deprecated")
        || head.contains("NOT the live")
        || head.contains("security_v2");

    assert!(
        marked,
        "core/security.rs must be marked STALE/superseded in its first lines. \
         Every type and method it defines also exists in security_v2.rs under \
         the same name, so a reader landing in this file has no way to tell \
         that none of it is compiled or reachable."
    );
}