//! The mesh must run under the operator's security policy, not a hardcoded
//! default.
//!
//! `Core::new` translates `ghostwire.toml`'s `[security]` section correctly
//! (`security_v2::SecurityConfig::from(&config.security)`) — and then, for the
//! lifetime of the project, threw the translated result away one layer down:
//! production `MeshManager::new` hardcoded `SecurityConfig::default()`.
//!
//! So `max_messages_per_minute`, `max_connections_per_ip`, `blacklist_enabled`,
//! `allowed_ips` and `blocked_ips` were all inert for the mesh transport, while
//! appearing to be configured. An operator who set `max_connections_per_ip = 2`
//! got the default instead.
//!
//! The fix threads the policy in as an explicit parameter, so this test pins
//! three things:
//!
//!   1. `MeshManager::new` no longer hardcodes `SecurityConfig::default()`.
//!   2. Every production call site passes a translated config, not a default.
//!   3. The translation itself maps the operator's values (a behaviour test on
//!      `From<&config::SecurityConfig>`, which is what makes the wiring mean
//!      something).

use std::fs;

fn mesh_source() -> String {
    fs::read_to_string("src/core/mesh.rs").expect("src/core/mesh.rs must be readable")
}

fn core_source() -> String {
    fs::read_to_string("src/core/mod.rs").expect("src/core/mod.rs must be readable")
}

#[test]
fn mesh_manager_new_does_not_hardcode_the_default_policy() {
    let mesh = mesh_source();

    // Scope to MeshManager::new so a legitimate default elsewhere cannot
    // satisfy this, and vice versa.
    let start = mesh
        .find("impl MeshManager {")
        .expect("impl MeshManager must exist");
    let body = &mesh[start..];

    let new_at = body
        .find("pub async fn new(")
        .expect("MeshManager::new must exist");
    let sig = &body[new_at..new_at + 3000];

    assert!(
        !sig.contains("let security_config = SecurityConfig::default()"),
        "MeshManager::new must not hardcode SecurityConfig::default(). That \
         discarded every operator-set security knob (rate limits, per-IP caps, \
         allow/block lists) for the mesh transport."
    );
    assert!(
        sig.contains("security_config: SecurityConfig"),
        "MeshManager::new must take the operator policy as a parameter"
    );
    assert!(
        sig.contains("SecurityManager::new(security_config)"),
        "MeshManager::new must build its SecurityManager from the parameter, \
         not from a fresh default"
    );
}

#[test]
fn every_production_call_site_passes_a_translated_config() {
    let core = core_source();

    let calls = core.matches("MeshManager::new(").count();
    assert!(
        calls >= 3,
        "expected the three production call sites in core/mod.rs, found {calls}"
    );

    // Each call must be followed by a translated config, not a bare default.
    // Count the translations that feed MeshManager::new.
    let translated = core
        .matches("SecurityConfig::from(&config.security)")
        .count()
        + core
            .matches("SecurityConfig::from(&self.config.security)")
            .count();
    assert!(
        translated >= calls,
        "each MeshManager::new call needs a SecurityConfig::from(&...) \
         translation; found {calls} calls and {translated} translations"
    );
}

#[test]
fn no_call_site_passes_a_bare_default_in_production() {
    // Belt and braces: a future call site adding `SecurityConfig::default()`
    // would compile and silently restore the bug. This is the check that
    // catches it, and it is scoped to core/mod.rs (production), not the tests.
    let core = core_source();
    assert!(
        !core.contains("MeshManager::new") || {
            // Look for a default passed within a MeshManager::new argument list.
            let mut offender = false;
            let mut idx = 0;
            while let Some(rel) = core[idx..].find("MeshManager::new(") {
                let at = idx + rel;
                let window = &core[at..(at + 2500).min(core.len())];
                // Stop at the end of this call's argument list.
                let end = window.find(")\n").unwrap_or(window.len());
                let args = &window[..end];
                if args.contains("SecurityConfig::default()") {
                    offender = true;
                }
                idx = at + 1;
            }
            !offender
        },
        "a production MeshManager::new call site passes SecurityConfig::default(); \
         pass SecurityConfig::from(&config.security) so operator settings apply"
    );
}

/// The behavioural half: the translation the wiring depends on actually maps
/// operator values. Without this, the wiring could compile and still mean
/// nothing.
mod translation_is_faithful {
    use ghostwire::config::SecurityConfig as FileSecurityConfig;
    use ghostwire::core::security_v2::SecurityConfig as RuntimeSecurityConfig;

    fn file_config() -> FileSecurityConfig {
        FileSecurityConfig {
            max_messages_per_minute: 7,
            max_connections_per_ip: 3,
            blacklist_enabled: true,
            ..Default::default()
        }
    }

    #[test]
    fn rate_limits_are_carried_into_the_runtime_policy() {
        let file = file_config();
        let runtime = RuntimeSecurityConfig::from(&file);
        assert_eq!(
            runtime.max_messages_per_minute, 7,
            "max_messages_per_minute must survive translation — this is the \
             knob the mesh now actually honours"
        );
    }

    #[test]
    fn per_ip_cap_and_blacklist_are_carried_across() {
        let file = file_config();
        let runtime = RuntimeSecurityConfig::from(&file);
        assert_eq!(
            runtime.max_connections_per_ip, 3,
            "max_connections_per_ip must survive translation"
        );
        assert!(
            runtime.blacklist_enabled,
            "blacklist_enabled must survive translation"
        );
    }

    #[test]
    fn a_non_default_config_does_not_translate_to_the_default() {
        // The regression shape: translation silently yields defaults. This
        // asserts the two differ, so a no-op `From` cannot pass both this and
        // the tests above.
        let file = file_config();
        let runtime = RuntimeSecurityConfig::from(&file);
        let default = RuntimeSecurityConfig::default();
        assert_ne!(
            runtime.max_messages_per_minute,
            default.max_messages_per_minute,
            "a configured policy translated to the default — the From impl is a no-op"
        );
    }
}