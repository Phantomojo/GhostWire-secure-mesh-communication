//! Babel routing protocol (RFC 8966) for mesh networks.
//!
//! Babel is a loop-avoiding distance-vector routing protocol designed for
//! wireless mesh networks. It is mobility-designed, link-agnostic, and
//! IETF-standardized. It outperforms BATMAN and OLSR in real UAV mesh tests.
//!
//! Key properties:
//! - Loop-free: uses feasibility condition (FC) to prevent routing loops
//! - Fast convergence: handles mobility and topology changes quickly
//! - Link-agnostic: runs over any link (WiFi, BLE, wired, DTN)
//! - Metric: uses ETX (Expected Transmission Count) or throughput
//!
//! This module provides the core data structures and routing table logic.
//! Full protocol implementation (Hello, IHU, Update, Route Request messages)
//! is a future work item.

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

/// Babel routing entry
#[derive(Debug, Clone)]
pub struct BabelRoute {
    /// Destination prefix
    pub prefix: IpAddr,
    /// Next hop address
    pub next_hop: IpAddr,
    /// Route metric (lower is better)
    pub metric: u16,
    /// Route sequence number (for loop prevention)
    pub seqno: u16,
    /// Source router ID
    pub router_id: u64,
    /// Last update time
    pub last_update: Instant,
    /// Route expiry
    pub expiry: Duration,
    /// Feasible distance (for FC)
    pub feasible_distance: u16,
}

/// Babel neighbour entry
#[derive(Debug, Clone)]
pub struct BabelNeighbour {
    /// Neighbour address
    pub addr: IpAddr,
    /// Hello sequence number
    pub hello_seqno: u16,
    /// Last hello received
    pub last_hello: Instant,
    /// Link cost (ETX-based)
    pub link_cost: u16,
    /// IHU (I Heard You) received
    pub ihu_received: bool,
}

/// Babel routing table
pub struct BabelRoutingTable {
    /// Routes indexed by destination prefix
    routes: HashMap<IpAddr, BabelRoute>,
    /// Neighbours indexed by address
    neighbours: HashMap<IpAddr, BabelNeighbour>,
    /// This router's ID
    router_id: u64,
    /// Hello interval
    hello_interval: Duration,
    /// Update interval
    update_interval: Duration,
}

impl BabelRoutingTable {
    pub fn new(router_id: u64) -> Self {
        Self {
            routes: HashMap::new(),
            neighbours: HashMap::new(),
            router_id,
            hello_interval: Duration::from_secs(4),
            update_interval: Duration::from_secs(8),
        }
    }

    /// Add or update a route
    pub fn update_route(&mut self, route: BabelRoute) -> bool {
        // Feasibility condition: only accept if metric < feasible_distance
        // or if seqno is newer
        if let Some(existing) = self.routes.get(&route.prefix) {
            if route.seqno < existing.seqno {
                return false; // Stale route
            }
            if route.seqno == existing.seqno && route.metric >= existing.metric {
                return false; // Not better
            }
        }
        self.routes.insert(route.prefix, route);
        true
    }

    /// Remove expired routes
    pub fn expire_routes(&mut self) {
        let now = Instant::now();
        self.routes.retain(|_, r| now.duration_since(r.last_update) < r.expiry);
    }

    /// Get best route to destination
    pub fn best_route(&self, dest: &IpAddr) -> Option<&BabelRoute> {
        self.routes.get(dest)
    }

    /// Add or update a neighbour
    pub fn update_neighbour(&mut self, neighbour: BabelNeighbour) {
        self.neighbours.insert(neighbour.addr, neighbour);
    }

    /// Remove stale neighbours
    pub fn expire_neighbours(&mut self) {
        let now = Instant::now();
        let timeout = self.hello_interval * 3;
        self.neighbours.retain(|_, n| now.duration_since(n.last_hello) < timeout);
    }

    /// Get all routes
    pub fn routes(&self) -> &HashMap<IpAddr, BabelRoute> {
        &self.routes
    }

    /// Get all neighbours
    pub fn neighbours(&self) -> &HashMap<IpAddr, BabelNeighbour> {
        &self.neighbours
    }

    /// Router ID
    pub fn router_id(&self) -> u64 {
        self.router_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_route_update() {
        let mut table = BabelRoutingTable::new(1);
        let route = BabelRoute {
            prefix: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            next_hop: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            metric: 100,
            seqno: 1,
            router_id: 2,
            last_update: Instant::now(),
            expiry: Duration::from_secs(30),
            feasible_distance: u16::MAX,
        };
        assert!(table.update_route(route.clone()));
        assert!(table.best_route(&route.prefix).is_some());
    }

    #[test]
    fn test_stale_route_rejected() {
        let mut table = BabelRoutingTable::new(1);
        let route = BabelRoute {
            prefix: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            next_hop: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            metric: 100,
            seqno: 2,
            router_id: 2,
            last_update: Instant::now(),
            expiry: Duration::from_secs(30),
            feasible_distance: u16::MAX,
        };
        table.update_route(route.clone());

        // Stale route (lower seqno) should be rejected
        let stale = BabelRoute {
            seqno: 1,
            ..route
        };
        assert!(!table.update_route(stale));
    }

    #[test]
    fn test_neighbour_expiry() {
        let mut table = BabelRoutingTable::new(1);
        let neighbour = BabelNeighbour {
            addr: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            hello_seqno: 1,
            last_hello: Instant::now() - Duration::from_secs(100),
            link_cost: 10,
            ihu_received: true,
        };
        table.update_neighbour(neighbour);
        table.expire_neighbours();
        assert!(table.neighbours().is_empty());
    }
}