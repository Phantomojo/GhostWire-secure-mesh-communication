use ghostwire::config::Config;

#[test]
fn config_has_reticulum_enabled() {
    let config = Config::default();
    assert!(
        config.network.reticulum_enabled,
        "reticulum should be enabled by default"
    );
}

#[test]
fn config_has_reticulum_interfaces() {
    let config = Config::default();
    let ifaces = &config.network.reticulum_interfaces;
    assert!(
        ifaces.contains("AutoInterface"),
        "should have AutoInterface"
    );
    assert!(
        ifaces.contains("TCPClientInterface"),
        "should have TCPClientInterface"
    );
    assert!(
        ifaces.contains("rmap.world:4242"),
        "should target rmap.world backbone"
    );
}

#[test]
fn reticulum_feature_is_compiled() {
    let result = std::panic::catch_unwind(|| {
        let config = Config::default();
        config.network.reticulum_enabled
    });
    assert!(result.is_ok(), "reticulum config should not panic");
    assert!(result.unwrap(), "reticulum should be enabled");
}
