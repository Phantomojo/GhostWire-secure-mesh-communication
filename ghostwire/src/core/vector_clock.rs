use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Truncate a node identifier to a non-reversible 8-byte hash for wire privacy.
/// Prevents peer_id leakage in vector clocks sent over GossipSub.
pub fn redact_node_id(node_id: &str) -> String {
    let hash = blake3::hash(node_id.as_bytes());
    hex::encode(&hash.as_bytes()[..8])
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VectorClock {
    clocks: HashMap<String, u64>,
}

/// Upper bound on how many distinct nodes a single vector clock will track.
///
/// A vector clock only needs an entry per node we have actually heard from, so
/// this is generous for a real mesh while preventing the unbounded growth that
/// let one peer brick a node's ability to send (see `merge`).
///
/// Sized so the serialized clock stays comfortably under `MAX_FRAME_SIZE`
/// (65_536 bytes). Each entry is a 16-hex-char redacted node id plus a small
/// integer, so ~2_000 entries serializes to well under 32 KiB even with
/// bincode overhead — leaving headroom for message content in the same frame.
pub const MAX_TRACKED_NODES: usize = 2_000;

impl VectorClock {
    pub fn new() -> Self {
        Self {
            clocks: HashMap::new(),
        }
    }

    pub fn increment(&mut self, node_id: &str) {
        *self.clocks.entry(node_id.to_string()).or_insert(0) += 1;
    }

    /// Merge another clock into this one, bounded.
    ///
    /// SECURITY: `merge` is called with an ATTACKER-CONTROLLED clock —
    /// `handle_gossipsub_message` does `self.vector_clock.merge(&msg.vector_clock)`
    /// on a frame from any peer. The keys are `String`, so a peer could insert
    /// unlimited novel keys and there was no pruning anywhere in the type
    /// (grep for gc/prune/retain/truncate against every `vector_clock`
    /// reference finds nothing).
    ///
    /// The damage compounds because every outbound message copies the whole
    /// clock: `message.vector_clock = self.vector_clock.clone()` at mesh.rs:1113
    /// and :1246. Once the clock grows past `MAX_FRAME_SIZE` (65_536, see
    /// wire_protocol.rs:19) `Frame::encode` fails, so the node can no longer
    /// SEND ANYTHING. One peer, no authentication, permanent self-DoS.
    ///
    /// Bounding here is the fix. `MAX_TRACKED_NODES` is far above any honest
    /// mesh size (a clock tracks nodes we have actually heard from), and the
    /// oldest-style trim keeps the map from growing without limit.
    pub fn merge(&mut self, other: &VectorClock) {
        for (node, &count) in &other.clocks {
            if self.clocks.len() >= MAX_TRACKED_NODES && !self.clocks.contains_key(node) {
                tracing::warn!(
                    target: "vector_clock",
                    "clock at capacity ({MAX_TRACKED_NODES}); ignoring a clock entry for an untracked node"
                );
                break;
            }
            let entry = self.clocks.entry(node.clone()).or_insert(0);
            *entry = (*entry).max(count);
        }
    }

    /// Drop the lowest-counter entries until the clock is back under
    /// `MAX_TRACKED_NODES`.
    ///
    /// Used as the recovery path after a merge refusal, so a node that has
    /// already accumulated excess keys can shed them rather than staying
    /// permanently unable to send. Drops the least-advanced entries, which are
    /// the least informative for causality comparisons.
    pub fn prune_to_capacity(&mut self) {
        if self.clocks.len() <= MAX_TRACKED_NODES {
            return;
        }
        let excess = self.clocks.len() - MAX_TRACKED_NODES;
        let mut by_counter: Vec<(String, u64)> =
            self.clocks.iter().map(|(k, v)| (k.clone(), *v)).collect();
        by_counter.sort_by_key(|(_, c)| *c);
        for (node, _) in by_counter.into_iter().take(excess) {
            self.clocks.remove(&node);
        }
        tracing::warn!(
            target: "vector_clock",
            "pruned {excess} entries from an oversized vector clock"
        );
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

    /// How many distinct nodes this clock tracks.
    ///
    /// Bounded by `MAX_TRACKED_NODES` for clocks built through `merge`, since
    /// `merge` is fed by untrusted peers. Direct `increment` calls are not
    /// bounded, so a caller that loops `increment` on peer-supplied ids must
    /// call `prune_to_capacity`.
    pub fn len(&self) -> usize {
        self.clocks.len()
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

    /// DoS REGRESSION: a peer must not be able to inflate our clock until the
    /// node can no longer send.
    ///
    /// `handle_gossipsub_message` merges an UNTRUSTED `msg.vector_clock` into
    /// the driver's long-lived clock. The clock is then copied into every
    /// outbound message (mesh.rs:1113 / :1246), so unbounded growth eventually
    /// pushes `Frame::encode` past `MAX_FRAME_SIZE` (65_536) and the node
    /// silently becomes unable to send anything. One peer, no auth.
    #[test]
    fn merge_from_peer_cannot_grow_the_clock_without_bound() {
        let mut ours = VectorClock::new();
        ours.increment("honest-node");

        // A hostile peer sends a frame claiming thousands of distinct nodes.
        let mut hostile = VectorClock::new();
        for i in 0..50_000 {
            hostile.increment(&format!("attacker-node-{i}"));
        }
        assert_eq!(hostile.len(), 50_000);

        ours.merge(&hostile);
        ours.prune_to_capacity();

        assert!(
            ours.len() <= MAX_TRACKED_NODES,
            "SECURITY REGRESSION: clock grew to {} entries from a single peer; \
             it is serialized into every outbound message and would exceed \
             MAX_FRAME_SIZE, permanently bricking send",
            ours.len()
        );

        // The honest node we already knew about must survive the merge.
        assert_eq!(
            ours.get("honest-node"),
            1,
            "pruning must not drop entries we legitimately track"
        );
    }

    /// The bound must not be a cliff: an existing tracked node can always have
    /// its counter advanced, even at capacity, or honest messages from known
    /// peers would be silently dropped.
    #[test]
    fn merge_at_capacity_still_advances_known_nodes() {
        let mut ours = VectorClock::new();
        for i in 0..MAX_TRACKED_NODES {
            ours.increment(&format!("node-{i}"));
        }
        assert_eq!(ours.len(), MAX_TRACKED_NODES);

        // A clock that advances a node we ALREADY track must be accepted.
        let mut advance = VectorClock::new();
        advance.increment("node-7");
        for _ in 0..9 {
            advance.increment("node-7");
        }
        ours.merge(&advance);
        assert_eq!(
            ours.get("node-7"),
            10,
            "a known node's counter must still advance at capacity"
        );
        assert_eq!(ours.len(), MAX_TRACKED_NODES, "no new keys admitted");

        // But a genuinely new node is refused once full.
        let mut fresh = VectorClock::new();
        fresh.increment("someone-new");
        ours.merge(&fresh);
        assert_eq!(
            ours.len(),
            MAX_TRACKED_NODES,
            "no new key may be admitted at capacity"
        );
    }

    /// `prune_to_capacity` is the recovery path for a node that already
    /// accumulated excess keys before the bound existed.
    #[test]
    fn prune_shrinks_an_already_oversized_clock() {
        let mut clock = VectorClock::new();
        for i in 0..(MAX_TRACKED_NODES + 500) {
            clock.increment(&format!("n{i}"));
        }
        assert!(clock.len() > MAX_TRACKED_NODES);

        clock.prune_to_capacity();
        assert_eq!(clock.len(), MAX_TRACKED_NODES);

        // Pruning a healthy clock must be a no-op, not destructive.
        let mut healthy = VectorClock::new();
        healthy.increment("only-node");
        healthy.prune_to_capacity();
        assert_eq!(healthy.len(), 1, "pruning must not touch a healthy clock");
    }
}
