use anyhow::Result;
use local_ip_address::list_afinet_netifas;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip: IpAddr,
    pub is_tailscale: bool,
    pub is_loopback: bool,
    pub is_virtual: bool,
    pub subnet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub interfaces: Vec<NetworkInterface>,
    pub active_interfaces: Vec<NetworkInterface>,
    pub tailscale_ip: Option<IpAddr>,
    pub primary_ip: Option<IpAddr>,
    pub hostname: String,
    pub internet_available: bool,
    pub internet_latency_ms: Option<u64>,
}

pub fn scan_interfaces() -> Result<ScanResult> {
    let ifaces = list_afinet_netifas().unwrap_or_default();
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let mut interfaces = Vec::new();
    let mut tailscale_ip = None;
    let mut primary_ip = None;

    for (name, ip) in &ifaces {
        let is_loopback = ip.is_loopback();
        let is_tailscale = name == "tailscale0"
            || name.starts_with("utun")
            || matches!(ip, IpAddr::V4(v) if is_tailscale_range(*v));
        let is_virtual = name.starts_with("docker")
            || name.starts_with("veth")
            || name.starts_with("br-")
            || name.starts_with("virbr")
            || name.starts_with("lo:");

        let subnet = if let IpAddr::V4(v4) = ip {
            if is_private(*v4) {
                Some(format!("{}/24", compute_subnet(*v4)))
            } else {
                Some(format!("{}/32", ip))
            }
        } else {
            None
        };

        let iface = NetworkInterface {
            name: name.clone(),
            ip: *ip,
            is_tailscale,
            is_loopback,
            is_virtual,
            subnet,
        };

        if is_tailscale && tailscale_ip.is_none() {
            tailscale_ip = Some(*ip);
        }

        if !is_loopback && !is_virtual && ip.is_ipv4() && primary_ip.is_none() {
            primary_ip = Some(*ip);
        }

        interfaces.push(iface);
    }

    let active_interfaces: Vec<NetworkInterface> = interfaces
        .iter()
        .filter(|i| !i.is_loopback && !i.is_virtual)
        .cloned()
        .collect();

    let (internet_available, internet_latency_ms) = check_internet();

    tracing::info!(
        "Network scan: {} interfaces ({} active), tailscale={}, primary={:?}, internet={}",
        interfaces.len(),
        active_interfaces.len(),
        tailscale_ip.is_some(),
        primary_ip,
        internet_available
    );

    for iface in &active_interfaces {
        tracing::info!(
            "  {} → {} (tailscale={}, subnet={:?})",
            iface.name,
            iface.ip,
            iface.is_tailscale,
            iface.subnet
        );
    }

    Ok(ScanResult {
        interfaces,
        active_interfaces,
        tailscale_ip,
        primary_ip,
        hostname,
        internet_available,
        internet_latency_ms,
    })
}

/// Get all listen addresses for the mesh (one per active interface).
pub fn get_listen_addresses(port: u16) -> Vec<String> {
    let scan = match scan_interfaces() {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!("Network scan failed: {}, falling back to 0.0.0.0", e);
            return vec![format!("/ip4/0.0.0.0/tcp/{}", port)];
        }
    };

    let mut addrs: Vec<String> = scan
        .active_interfaces
        .iter()
        .map(|i| format!("/ip4/{}/tcp/{}", i.ip, port))
        .collect();

    if !addrs.iter().any(|a| a.contains("0.0.0.0")) {
        addrs.push(format!("/ip4/0.0.0.0/tcp/{}", port));
    }

    addrs
}

fn is_tailscale_range(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    octets[0] == 100 && (octets[1] & 0xC0) == 64
}

fn is_private(ip: Ipv4Addr) -> bool {
    match ip.octets() {
        [10, _, _, _] => true,
        [172, b, _, _] if (16..=31).contains(&b) => true,
        [192, 168, _, _] => true,
        _ => false,
    }
}

fn compute_subnet(ip: Ipv4Addr) -> Ipv4Addr {
    let octets = ip.octets();
    Ipv4Addr::new(octets[0], octets[1], octets[2], 0)
}

fn check_internet() -> (bool, Option<u64>) {
    let start = std::time::Instant::now();
    match TcpStream::connect_timeout(
        &"8.8.8.8:53".parse().unwrap(),
        Duration::from_secs(3),
    ) {
        Ok(_) => (true, Some(start.elapsed().as_millis() as u64)),
        Err(_) => (false, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tailscale_range() {
        assert!(is_tailscale_range(Ipv4Addr::new(100, 64, 0, 1)));
        assert!(is_tailscale_range(Ipv4Addr::new(100, 127, 255, 255)));
        assert!(!is_tailscale_range(Ipv4Addr::new(192, 168, 1, 1)));
        assert!(!is_tailscale_range(Ipv4Addr::new(10, 0, 0, 1)));
    }

    #[test]
    fn test_is_private() {
        assert!(is_private(Ipv4Addr::new(10, 0, 0, 1)));
        assert!(is_private(Ipv4Addr::new(172, 16, 0, 1)));
        assert!(is_private(Ipv4Addr::new(192, 168, 1, 1)));
        assert!(!is_private(Ipv4Addr::new(8, 8, 8, 8)));
    }

    #[test]
    fn test_compute_subnet() {
        assert_eq!(compute_subnet(Ipv4Addr::new(192, 168, 1, 5)), Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(compute_subnet(Ipv4Addr::new(10, 0, 0, 255)), Ipv4Addr::new(10, 0, 0, 0));
    }

    #[test]
    fn test_scan_returns_results() {
        let result = scan_interfaces().unwrap();
        assert!(!result.interfaces.is_empty());
        assert!(!result.hostname.is_empty());
    }
}
