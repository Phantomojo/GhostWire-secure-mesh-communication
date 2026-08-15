use crate::core::message::Message;
use crate::core::vector_clock::VectorClock;
use std::collections::VecDeque;

/// CausalBuffer ensures messages are delivered in causal order.
/// If a message arrives but its dependencies (as tracked by its vector clock)
/// are not yet met, it is held in the buffer.
pub struct CausalBuffer {
    buffer: VecDeque<Message>,
    max_size: usize,
}

impl CausalBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            max_size,
        }
    }

    /// Add a message to the buffer.
    pub fn push(&mut self, message: Message) {
        if self.buffer.len() >= self.max_size {
            // Drop oldest if full (simple eviction)
            self.buffer.pop_front();
        }
        self.buffer.push_back(message);
    }

    /// Check if a message is ready for delivery based on the current vector clock.
    pub fn is_ready(&self, message: &Message, current_clock: &VectorClock) -> bool {
        let sender_redacted = crate::core::vector_clock::redact_node_id(&message.sender);
        let sender_count = message.vector_clock.get(&sender_redacted);

        // A valid causal message must advance its own sender component by
        // exactly one. Accepting an older value would re-deliver replays;
        // accepting a larger value would skip an unsatisfied dependency.
        if sender_count != current_clock.get(&sender_redacted) + 1 {
            return false;
        }

        for (node, &count) in message.vector_clock.clocks() {
            if node != &sender_redacted && count > current_clock.get(node) {
                // Dependencies from other senders must already be delivered.
                return false;
            }
        }
        true
    }

    /// Replayed/stale and malformed vector clocks cannot become deliverable.
    /// Remove them instead of allowing a remote sender to exhaust the bounded
    /// causal buffer with messages that will never be released.
    fn should_discard(&self, message: &Message, current_clock: &VectorClock) -> bool {
        let sender_redacted = crate::core::vector_clock::redact_node_id(&message.sender);
        let sender_count = message.vector_clock.get(&sender_redacted);
        sender_count == 0 || sender_count <= current_clock.get(&sender_redacted)
    }

    /// Drain all messages that are now ready for delivery.
    pub fn drain_ready(&mut self, current_clock: &mut VectorClock) -> Vec<Message> {
        let mut ready = Vec::new();
        let mut changed = true;

        while changed {
            changed = false;
            let mut i = 0;
            while i < self.buffer.len() {
                if self.should_discard(&self.buffer[i], current_clock) {
                    self.buffer.remove(i);
                    changed = true;
                } else if self.is_ready(&self.buffer[i], current_clock) {
                    let msg = self.buffer.remove(i).unwrap();
                    // Update our clock with the delivered message's clock
                    current_clock.merge(&msg.vector_clock);
                    ready.push(msg);
                    changed = true;
                } else {
                    i += 1;
                }
            }
        }
        ready
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::vector_clock::{redact_node_id, VectorClock};
    use std::collections::HashMap;

    fn message(sender: &str, content: &str, clock: HashMap<String, u64>) -> Message {
        let mut message = Message::new(
            sender.to_string(),
            "recipient".to_string(),
            content.to_string(),
        );
        message.vector_clock = VectorClock::from_map(clock);
        message
    }

    #[test]
    fn gap_is_buffered_then_released_in_sender_order() {
        let alice = redact_node_id("alice");
        let mut first_clock = HashMap::new();
        first_clock.insert(alice.clone(), 1);
        let mut second_clock = HashMap::new();
        second_clock.insert(alice, 2);

        let first = message("alice", "first", first_clock);
        let second = message("alice", "second", second_clock);
        let mut current_clock = VectorClock::new();
        let mut buffer = CausalBuffer::new(8);

        // The second message arrives first and must wait for counter 1.
        buffer.push(second);
        assert!(buffer.drain_ready(&mut current_clock).is_empty());
        assert_eq!(buffer.len(), 1);

        // Once the predecessor arrives, one drain operation delivers both in
        // causal order because `drain_ready` rescans after each release.
        buffer.push(first);
        let delivered = buffer.drain_ready(&mut current_clock);
        assert_eq!(
            delivered
                .iter()
                .map(|item| item.content.as_str())
                .collect::<Vec<_>>(),
            vec!["first", "second"]
        );
        assert_eq!(buffer.len(), 0);
        assert_eq!(current_clock.get(&redact_node_id("alice")), 2);
    }

    #[test]
    fn bounded_buffer_evicts_oldest_message() {
        let alice = redact_node_id("alice");
        let mut buffer = CausalBuffer::new(2);

        for (counter, content) in [(1, "first"), (2, "second"), (3, "third")] {
            let mut clock = HashMap::new();
            clock.insert(alice.clone(), counter);
            buffer.push(message("alice", content, clock));
        }
        assert_eq!(buffer.len(), 2);

        // Counter 1 was evicted. Simulate that it had been delivered through a
        // different path; the retained counter 2 and 3 messages then release.
        let mut current_clock = VectorClock::new();
        current_clock.increment(&alice);
        let delivered = buffer.drain_ready(&mut current_clock);
        assert_eq!(
            delivered
                .iter()
                .map(|item| item.content.as_str())
                .collect::<Vec<_>>(),
            vec!["second", "third"]
        );
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn replayed_message_is_discarded_without_redelivery() {
        let alice = redact_node_id("alice");
        let mut clock = HashMap::new();
        clock.insert(alice.clone(), 1);
        let replay = message("alice", "replay", clock);
        let mut current_clock = VectorClock::new();
        current_clock.increment(&alice);
        let mut buffer = CausalBuffer::new(8);

        buffer.push(replay);
        assert!(buffer.drain_ready(&mut current_clock).is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(current_clock.get(&alice), 1);
    }

    #[test]
    fn cross_sender_dependency_is_not_delivered_early() {
        let alice = redact_node_id("alice");
        let bob = redact_node_id("bob");
        let mut alice_clock = HashMap::new();
        alice_clock.insert(alice.clone(), 1);
        let mut bob_clock = HashMap::new();
        bob_clock.insert(alice, 1);
        bob_clock.insert(bob.clone(), 1);

        let predecessor = message("alice", "alice-first", alice_clock);
        let dependent = message("bob", "bob-after-alice", bob_clock);
        let mut current_clock = VectorClock::new();
        let mut buffer = CausalBuffer::new(8);

        buffer.push(dependent);
        assert!(buffer.drain_ready(&mut current_clock).is_empty());

        buffer.push(predecessor);
        let delivered = buffer.drain_ready(&mut current_clock);
        assert_eq!(delivered.len(), 2);
        assert_eq!(delivered[0].sender, "alice");
        assert_eq!(delivered[1].sender, "bob");
        assert_eq!(current_clock.get(&bob), 1);
    }
}
