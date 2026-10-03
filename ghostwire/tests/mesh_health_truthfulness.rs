//! Health must report the mesh that is actually running.
//!
//! ## The defect
//!
//! `Core` carries TWO mesh fields:
//!
//! ```ignore
//! pub mesh_manager: Option<Arc<RwLock<MeshManager>>>,        // :347  built at startup
//! pub late_mesh_manager: Arc<RwLock<Option<Arc<RwLock<MeshManager>>>>>, // :351 built after unlock
//! ```
//!
//! The mesh cannot be built before the profile is unlocked — it needs the
//! identity — so unlock installs it into `late_mesh_manager`. That is what
//! `effective_mesh_manager()` exists for: it checks `mesh_manager` first, then
//! falls back to `late_mesh_manager`.
//!
//! `controllers/health.rs:29` reads only `core.mesh_manager`:
//!
//! ```ignore
//! match &core.mesh_manager {
//!     Some(_) => (true, "initialized".to_string()),
//!     None    => ... "not initialized" ...
//! }
//! ```
//!
//! So on an unlocked node running a live mesh, health reported:
//!
//! ```text
//! mesh port 4201 is listening          <- ss, real socket
//! mesh_enabled=false
//! mesh_status="not initialized"       <- health.rs, false
//! ```
//!
//! A live swarm on one side, "not initialized" on the other. Observed live in
//! `~/kenya-invest/analysis/live_verify.sh`, after a successful
//! `POST /api/v2/profile/unlock` returning HTTP 200 on both nodes.
//!
//! ## Why this matters beyond tidiness
//!
//! `mesh_health.rs` computes `rns_ready` and `mod.rs` feeds mesh readiness into
//! bootstrap. A health surface that reports the wrong mesh is the same
//! dishonesty class as an endpoint returning a stub version: a caller that
//! trusts it is told the node has no mesh when it has one. Anything gating on
//! health — an operator, a supervisor, a readiness probe — gets the wrong
//! answer.

use std::fs;

fn health_source() -> String {
    fs::read_to_string("src/controllers/health.rs").expect("src/controllers/health.rs readable")
}

#[test]
fn health_reads_the_effective_mesh_manager_not_the_startup_field() {
    let src = health_source();

    assert!(
        src.contains("effective_mesh_manager"),
        "controllers/health.rs must call core.effective_mesh_manager(). It reads \
         only core.mesh_manager, which is None on any node whose mesh was built \
         after unlock — so a LIVE mesh is reported as 'not initialized'."
    );
}

/// Pin the reason `effective_mesh_manager` is the correct accessor: the second
/// field exists because the mesh is built after unlock.
#[test]
fn the_late_mesh_field_is_real_and_populated_by_unlock() {
    let core = fs::read_to_string("src/core/mod.rs").expect("src/core/mod.rs readable");

    assert!(
        core.contains("pub late_mesh_manager"),
        "Core must carry late_mesh_manager — the post-unlock mesh slot"
    );
    assert!(
        core.contains("pub async fn effective_mesh_manager"),
        "Core must expose effective_mesh_manager(), which consults both slots"
    );

    // And it must actually prefer the late slot, not just exist.
    let at = core
        .find("pub async fn effective_mesh_manager")
        .expect("accessor must exist");
    let body = &core[at..(at + 320).min(core.len())];
    assert!(
        body.contains("late_mesh_manager"),
        "effective_mesh_manager must fall back to late_mesh_manager — that \
         fallback is the whole point, and without it the accessor is equivalent \
         to reading mesh_manager directly."
    );
}

/// The pre-fix shape is a bare `match &core.mesh_manager`. Assert it is gone,
/// anchored to the match so an unrelated mention of the field cannot satisfy
/// this.
#[test]
fn the_bare_mesh_manager_match_is_gone() {
    let src = health_source();
    let bare = src
        .lines()
        .any(|l| !l.trim_start().starts_with("//") && l.contains("match &core.mesh_manager {"));

    assert!(
        !bare,
        "controllers/health.rs still matches on &core.mesh_manager directly. \
         That field is None whenever the mesh was installed by unlock, so the \
         node reports 'not initialized' while its swarm is live."
    );
}