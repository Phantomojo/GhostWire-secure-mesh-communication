# Tokio crate documentation

## Overview

Tokio is an event-driven, non-blocking I/O runtime for writing reliable network applications in Rust.

## License

MIT

## Website

https://tokio.rs

## Source

https://github.com/tokio-rs/tokio

---

## Core Features

### Async Runtime

Tokio provides a multi-threaded async runtime for executing asynchronous code.

```rust
#[tokio::main]
async fn main() {
    // Your async code here
}
```

### Task Spawning

```rust
use tokio::task;

let handle = tokio::spawn(async {
    "hello from spawned task"
});

let result = handle.await?;
```

### Channels

```rust
use tokio::sync::mpsc;

// Create channel
let (tx, mut rx) = mpsc::channel(100);

// Send
tx.send("hello").await?;

// Receive
while let Some(msg) = rx.recv().await {
    println!("Received: {}", msg);
}
```

### TCP/UDP Networking

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP Server
let listener = TcpListener::bind("127.0.0.1:8080").await?;

while let Ok((mut socket, _)) = listener.accept().await {
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await?;
        socket.write_all(&buf[..n]).await?;
    });
}
```

### Timers

```rust
use tokio::time::{sleep, timeout, Duration};

sleep(Duration::from_secs(1)).await;

// With timeout
let result = timeout(Duration::from_secs(5), some_async_operation()).await;
```

### Mutex

```rust
use tokio::sync::Mutex;

let mutex = Mutex::new(0);

{
    let mut guard = mutex.lock().await;
    *guard += 1;
}
```

---

## Feature Flags

| Feature         | Description                            |
| --------------- | -------------------------------------- |
| rt              | Basic runtime (current-thread)         |
| rt-multi-thread | Multi-threaded work-stealing scheduler |
| io-util         | Async I/O traits                       |
| net             | TCP/UDP/Unix sockets                   |
| time            | Timers and intervals                   |
| process         | Child processes                        |
| sync            | Synchronization primitives             |
| fs              | Async filesystem                       |
| signal          | OS signal handling                     |
| macros          | #[tokio::main], #[tokio::test]         |
| full            | All features                           |

---

## Runtime Types

### Multi-thread Runtime

```rust
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()?;
```

### Current-thread Runtime

```rust
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()?;
```

---

## Modules

### tokio::net

- TcpListener, TcpStream, UdpSocket
- Unix sockets

### tokio::sync

- channels: mpsc, oneshot, watch, broadcast
- Mutex, RwLock, Semaphore
- Barrier

### tokio::time

- sleep, interval, timeout, timeout_at

### tokio::fs

- Async file operations

### tokio::process

- Spawning child processes

### tokio::signal

- Unix/Windows signal handling

---

## GhostWire Usage

GhostWire uses tokio for:

- Multi-threaded async runtime
- TCP networking
- Message passing between components
- Timers for health checks
- File I/O
