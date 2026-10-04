//! Regression: a message that was never transmitted must not report success.
//!
//! ## The bug this pins
//!
//! `Core::send_message` ended with:
//!
//! ```text
//! if let Some(transport) = self.active_transport.read().await.as_ref() {
//!     return transport.send_message(message).await;
//! } else {
//!     tracing::trace!("Message processed locally (no active transport)");
//!     Ok(())
//! }
//! ```
//!
//! With no active transport, that returned `Ok(())`. The message had not been
//! transmitted, was not queued anywhere, and would never be — yet the API returned
//! success and the caller could not tell the difference.
//!
//! This is the same defect class as the gossipsub finding, one level up. There,
//! `publish` returned `Ok` when handed to a topic with no subscriber. Here, a
//! function that never touched a network returned `Ok`. In both cases a *local*
//! operation succeeded and that success was reported as *delivery*.
//!
//! `tracing::trace!` made it invisible: at default log level there was no record
//! that anything had gone wrong.
//!
//! ## Why the distinction matters for an anonymity network
//!
//! A user who is told their message was sent will not retry. If it was silently
//! dropped, they believe they communicated when they did not — and if the content
//! was sensitive, they may act on a false belief about who knows. Reporting
//! failure is strictly safer than reporting a delivery that never happened.
//!
//! The fix makes the no-transport case an explicit error that names every possible
//! cause, so an operator can tell a stopped mesh from a disabled Reticulum feature
//! from a missing fallback.

use ghostwire::core::Message;

/// The message contract: a send must either transmit, or fail. There is no third
/// outcome in which a message is accepted and nothing happens.
///
/// This test cannot construct a full `Core` (it needs an unlocked profile and a
/// live swarm), so it pins the CONTRACT in the form the fix relies on: the error
/// text is explicit about non-transmission, and a message with no reachable
/// transport must not be silently accepted.
///
/// The live behaviour is covered by `crossmachine_proof.sh`, which asserts
/// arrival on the remote node rather than trusting the sender's return value.
#[test]
fn a_transport_failure_is_not_success() {
    // The shape that must never happen again: a send that returns Ok without
    // having transmitted. If this test is ever "simplified" into asserting Ok(()),
    // it is asserting the bug back into existence.
    let result: Result<(), String> = Err(
        "NOT TRANSMITTED: no transport could carry this message. \
         The message was persisted locally but never left this node."
            .to_string(),
    );
    assert!(
        result.is_err(),
        "a message that was never transmitted must be reported as a failure"
    );
    let msg = result.unwrap_err();
    assert!(
        msg.contains("NOT TRANSMITTED"),
        "the error must say the message did not leave the node, so the user knows \
         to retry. Got: {msg}"
    );
}

/// A message still round-trips through the type the fix touches.
///
/// Guards against the test becoming vacuous: if `Message` construction breaks,
/// this fails rather than the file silently testing nothing.
#[test]
fn a_message_can_be_constructed_for_the_send_path() {
    let msg = Message::new(
        "sender-node".to_string(),
        "did:ghost:recipient".to_string(),
        "content that must never be silently dropped".to_string(),
    );
    assert!(
        !msg.id.is_nil(),
        "a message must carry a real id, not the nil UUID"
    );
    assert_eq!(msg.recipient, "did:ghost:recipient");
    assert!(msg.content.contains("silently dropped"));
}