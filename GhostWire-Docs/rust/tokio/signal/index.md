[tokio](../index.html)

Module signal Copy item path

<span class="sub-heading"><a href="../../src/tokio/signal/mod.rs.html#1-89" class="src">Source</a>
</span>

Expand description

Asynchronous signal handling for Tokio.

Note that signal handling is in general a very tricky topic and should
be used with great care. This crate attempts to implement ‘best
practice’ for signal handling, but it should be evaluated for your own
applications’ needs to see if it’s suitable.

There are some fundamental limitations of this crate documented on the
OS specific structures, as well.

# <a href="#examples" class="doc-anchor">§</a>Examples

Print on “ctrl-c” notification.

``` {.rust .rust-example-rendered}
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}
```

Wait for `SIGHUP` on Unix

``` {.rust .rust-example-rendered}
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An infinite stream of hangup signals.
    let mut stream = signal(SignalKind::hangup())?;

    // Print whenever a HUP signal is received
    loop {
        stream.recv().await;
        println!("got signal HUP");
    }
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="unix/index.html" class="mod"
title="mod tokio::signal::unix">unix</a>  
Unix-specific types for signal handling.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.ctrl_c.html" class="fn"
title="fn tokio::signal::ctrl_c">ctrl_c</a>  
Completes when a “ctrl-c” notification is sent to the process.
