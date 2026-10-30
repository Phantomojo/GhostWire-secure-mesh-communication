# TLA+ Specification Outline — GhostWire Message Path

## Purpose

Machine-checked invariants for the GhostWire message delivery path. Targets
the "no message delivered" class of bug. Uses TLC model checker on a bounded
model.

## Scope

The spec covers:
1. Handshake (key exchange, peer discovery)
2. DHT (peer routing, message routing)
3. Message delivery (send, receive, ACK, retry)
4. Store-and-forward (offline delivery)

## State Variables

```
VARIABLES
    peers,          \* Set of known peers
    peerKeys,       \* Map: peer -> public key
    messages,       \* Set of messages in flight
    delivered,      \* Set of delivered message IDs
    mailboxes,      \* Map: peer -> list of pending messages
    connections,    \* Map: peer -> connection state
    retryCount      \* Map: message -> retry count
```

## Invariants

### Safety Invariants

1. **No message loss**: Every message sent is either delivered or in a mailbox
   ```
   \A m \in messages : m.id \in delivered \/ \E p \in peers : m \in mailboxes[p]
   ```

2. **No duplicate delivery**: A message is delivered at most once
   ```
   \A m \in messages : Cardinality({p \in peers : m.id \in delivered[p]}) <= 1
   ```

3. **Key consistency**: Peer keys are consistent across all nodes
   ```
   \A p1, p2 \in peers : peerKeys[p1][p2] = peerKeys[p2][p2]
   ```

4. **Encryption integrity**: Delivered messages match sent messages
   ```
   \A m \in delivered : m.content = originalContent[m.id]
   ```

### Liveness Properties

1. **Eventually delivered**: If recipient is online, message is eventually delivered
   ```
   \A m \in messages : (recipientOnline(m.recipient) ~> m.id \in delivered)
   ```

2. **Mailbox drain**: When recipient comes online, mailbox is drained
   ```
   recipientComesOnline(p) ~> mailboxes[p] = {}
   ```

### Bounded Properties

1. **Retry bound**: Retry count never exceeds MAX_RETRIES
   ```
   \A m \in messages : retryCount[m] <= MAX_RETRIES
   ```

2. **Mailbox bound**: Mailbox size never exceeds MAX_MAILBOX_SIZE
   ```
   \A p \in peers : Len(mailboxes[p]) <= MAX_MAILBOX_SIZE
   ```

3. **Reconnect exposure**: Reconnect exposure <= QCAP + PCAP + BURST + RATE*WINDOW
   ```
   reconnectExposure <= QCAP + PCAP + BURST + RATE * WINDOW
   ```

## Actions

### Handshake
```
Handshake(p1, p2) ==
    /\ p1 \in peers /\ p2 \in peers
    /\ p1 # p2
    /\ connections' = [connections EXCEPT ![p1][p2] = "connected"]
    /\ peerKeys' = [peerKeys EXCEPT ![p1][p2] = generateKey(p1, p2)]
    /\ UNCHANGED <<messages, delivered, mailboxes, retryCount>>
```

### SendMessage
```
SendMessage(sender, recipient, content) ==
    /\ sender \in peers /\ recipient \in peers
    /\ LET m == [id |-> newId, sender |-> sender, recipient |-> recipient, content |-> content]
       IN messages' = messages \cup {m}
    /\ UNCHANGED <<delivered, mailboxes, connections, retryCount, peerKeys>>
```

### DeliverMessage
```
DeliverMessage(m) ==
    /\ m \in messages
    /\ connections[m.sender][m.recipient] = "connected"
    /\ delivered' = delivered \cup {m.id}
    /\ UNCHANGED <<messages, mailboxes, connections, retryCount, peerKeys>>
```

### StoreInMailbox
```
StoreInMailbox(m) ==
    /\ m \in messages
    /\ connections[m.sender][m.recipient] # "connected"
    /\ mailboxes' = [mailboxes EXCEPT ![m.recipient] = Append(mailboxes[m.recipient], m)]
    /\ UNCHANGED <<messages, delivered, connections, retryCount, peerKeys>>
```

### DrainMailbox
```
DrainMailbox(p) ==
    /\ p \in peers
    /\ connections[p][*] = "connected"  \* p is online
    /\ delivered' = delivered \cup {m.id : m \in mailboxes[p]}
    /\ mailboxes' = [mailboxes EXCEPT ![p] = <<>>]
    /\ UNCHANGED <<messages, connections, retryCount, peerKeys>>
```

### RetryMessage
```
RetryMessage(m) ==
    /\ m \in messages
    /\ m.id \notin delivered
    /\ retryCount[m] < MAX_RETRIES
    /\ retryCount' = [retryCount EXCEPT ![m] = retryCount[m] + 1]
    /\ UNCHANGED <<messages, delivered, mailboxes, connections, peerKeys>>
```

## Fairness

```
Fairness ==
    /\ \A m \in messages : WF_vars(DeliverMessage(m))
    /\ \A p \in peers : WF_vars(DrainMailbox(p))
```

## Model Checking Parameters

- Peers: 2-3 (small model)
- Messages: 2-3
- MAX_RETRIES: 3
- MAX_MAILBOX_SIZE: 5

## Implementation Notes

1. Use TLA+ Toolbox or VS Code TLA+ extension
2. Run TLC with the above parameters
3. Check all invariants
4. If invariant violated, examine counterexample trace
5. Fix code, re-verify

## References

- TLA+ / TLC (signal-fish-server formal-verification.md)
- Tamarin/ProVerif for crypto protocol verification
- ADR-0003 rationale (from signal-fish-server)
EOF
echo "created"