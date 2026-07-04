use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VectorClock {
    clocks: HashMap<String, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        Self {
            clocks: HashMap::new(),
        }
    }

    pub fn increment(&mut self, node_id: &str) {
        *self.clocks.entry(node_id.to_string()).or_insert(0) += 1;
    }

    pub fn merge(&mut self, other: &VectorClock) {
        for (node, &count) in &other.clocks {
            let entry = self.clocks.entry(node.clone()).or_insert(0);
            *entry = (*entry).max(count);
        }
    }

    /// Returns ordering between two clocks.
    /// Less = self happened before other.
    /// Greater = self happened after other.
    /// Equal = concurrent or identical.
    pub fn partial_cmp(&self, other: &VectorClock) -> Option<std::cmp::Ordering> {
        let mut less = false;
        let mut greater = false;
        let all_nodes: Vec<&String> = self.clocks.keys().chain(other.clocks.keys()).collect();
        for node in all_nodes {
            let a = self.clocks.get(node).copied().unwrap_or(0);
            let b = other.clocks.get(node).copied().unwrap_or(0);
            if a < b {
                less = true;
            }
            if a > b {
                greater = true;
            }
        }
        match (less, greater) {
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            (false, false) => Some(std::cmp::Ordering::Equal),
            (true, true) => None, // concurrent
        }
    }

    pub fn get(&self, node_id: &str) -> u64 {
        self.clocks.get(node_id).copied().unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.clocks.is_empty()
    }

    pub fn node_count(&self) -> usize {
        self.clocks.len()
    }
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_and_get() {
        let mut vc = VectorClock::new();
        vc.increment("alice");
        vc.increment("alice");
        vc.increment("bob");
        assert_eq!(vc.get("alice"), 2);
        assert_eq!(vc.get("bob"), 1);
        assert_eq!(vc.get("charlie"), 0);
    }

    #[test]
    fn merge_takes_max() {
        let mut a = VectorClock::new();
        a.increment("alice");
        a.increment("alice");
        a.increment("bob");

        let mut b = VectorClock::new();
        b.increment("alice");
        b.increment("charlie");

        a.merge(&b);
        assert_eq!(a.get("alice"), 2);
        assert_eq!(a.get("bob"), 1);
        assert_eq!(a.get("charlie"), 1);
    }

    #[test]
    fn ordering_less() {
        let mut a = VectorClock::new();
        a.increment("alice");

        let mut b = VectorClock::new();
        b.increment("alice");
        b.increment("bob");

        assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
    }

    #[test]
    fn ordering_greater() {
        let mut a = VectorClock::new();
        a.increment("alice");
        a.increment("bob");

        let mut b = VectorClock::new();
        b.increment("alice");

        assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Greater));
    }

    #[test]
    fn ordering_concurrent() {
        let mut a = VectorClock::new();
        a.increment("alice");

        let mut b = VectorClock::new();
        b.increment("bob");

        assert_eq!(a.partial_cmp(&b), None);
    }

    #[test]
    fn ordering_equal() {
        let mut a = VectorClock::new();
        a.increment("alice");
        a.increment("bob");

        let mut b = VectorClock::new();
        b.increment("alice");
        b.increment("bob");

        assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn empty_clocks_equal() {
        let a = VectorClock::new();
        let b = VectorClock::new();
        assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Equal));
        assert!(a.is_empty());
    }
}
