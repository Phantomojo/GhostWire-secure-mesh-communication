[tokio](../index.html)

Module process Copy item path

<span class="sub-heading"><a href="../../src/tokio/process/mod.rs.html#1-1854"
class="src">Source</a> </span>

Expand description

An implementation of asynchronous process management for Tokio.

This module provides a
[`Command`](struct.Command.html "struct tokio::process::Command") struct
that imitates the interface of the
[`std::process::Command`](https://doc.rust-lang.org/1.93.1/std/process/struct.Command.html "struct std::process::Command")
type in the standard library, but provides asynchronous versions of
functions that create processes. These functions (`spawn`, `status`,
`output` and their variants) return “future aware” types that
interoperate with Tokio. The asynchronous process support is provided
through signal handling on Unix and system APIs on Windows.

# <a href="#examples" class="doc-anchor">§</a>Examples

Here’s an example program which will spawn `echo hello world` and then
wait for it complete.

``` {.rust .rust-example-rendered}
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The usage is similar as with the standard library's `Command` type
    let mut child = Command::new("echo")
        .arg("hello")
        .arg("world")
        .spawn()
        .expect("failed to spawn");

    // Await until the command completes
    let status = child.wait().await?;
    println!("the command exited with: {}", status);
    Ok(())
}
```

Next, let’s take a look at an example where we not only spawn
`echo hello world` but we also capture its output.

``` {.rust .rust-example-rendered}
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Like above, but use `output` which returns a future instead of
    // immediately returning the `Child`.
    let output = Command::new("echo").arg("hello").arg("world")
                        .output();

    let output = output.await?;

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello world\n");
    Ok(())
}
```

We can also read input line by line.

``` {.rust .rust-example-rendered}
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cat");

    // Specify that we want the command's standard output piped back to us.
    // By default, standard input/output/error will be inherited from the
    // current process (for example, this means that standard input will
    // come from the keyboard and standard output/error will go directly to
    // the terminal if this process is invoked from the command line).
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()
        .expect("failed to spawn command");

    let stdout = child.stdout.take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async move {
        let status = child.wait().await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        println!("Line: {}", line);
    }

    Ok(())
}
```

Here is another example using `sort` writing into the child process
standard input, capturing the output of the sorted text.

``` {.rust .rust-example-rendered}
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("sort");

    // Specifying that we want pipe both the output and the input.
    // Similarly to capturing the output, by configuring the pipe
    // to stdin it can now be used as an asynchronous writer.
    cmd.stdout(Stdio::piped());
    cmd.stdin(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    // These are the animals we want to sort
    let animals: &[&str] = &["dog", "bird", "frog", "cat", "fish"];

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");

    // Write our animals to the child process
    // Note that the behavior of `sort` is to buffer _all input_ before writing any output.
    // In the general sense, it is recommended to write to the child in a separate task as
    // awaiting its exit (or output) to avoid deadlocks (for example, the child tries to write
    // some output but gets stuck waiting on the parent to read from it, meanwhile the parent
    // is stuck waiting to write its input completely before reading the output).
    stdin
        .write(animals.join("\n").as_bytes())
        .await
        .expect("could not write to stdin");

    // We drop the handle here which signals EOF to the child process.
    // This tells the child process that it there is no more data on the pipe.
    drop(stdin);

    let op = child.wait_with_output().await?;

    // Results should come back in sorted order
    assert_eq!(op.stdout, "bird\ncat\ndog\nfish\nfrog\n".as_bytes());

    Ok(())
}
```

With some coordination, we can also pipe the output of one command into
another.

``` {.rust .rust-example-rendered}
use tokio::join;
use tokio::process::Command;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut echo = Command::new("echo")
        .arg("hello world!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn echo");

    let tr_stdin: Stdio = echo
        .stdout
        .take()
        .unwrap()
        .try_into()
        .expect("failed to convert to Stdio");

    let tr = Command::new("tr")
        .arg("a-z")
        .arg("A-Z")
        .stdin(tr_stdin)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn tr");

    let (echo_result, tr_output) = join!(echo.wait(), tr.wait_with_output());

    assert!(echo_result.unwrap().success());

    let tr_output = tr_output.expect("failed to await tr");
    assert!(tr_output.status.success());

    assert_eq!(tr_output.stdout, b"HELLO WORLD!\n");

    Ok(())
}
```

# <a href="#caveats" class="doc-anchor">§</a>Caveats

## <a href="#droppingcancellation" class="doc-anchor">§</a>Dropping/Cancellation

Similar to the behavior to the standard library, and unlike the futures
paradigm of dropping-implies-cancellation, a spawned process will, by
default, continue to execute even after the `Child` handle has been
dropped.

The
[`Command::kill_on_drop`](struct.Command.html#method.kill_on_drop "method tokio::process::Command::kill_on_drop")
method can be used to modify this behavior and kill the child process if
the `Child` wrapper is dropped before it has exited.

## <a href="#unix-processes" class="doc-anchor">§</a>Unix Processes

On Unix platforms processes must be “reaped” by their parent process
after they have exited in order to release all OS resources. A child
process which has exited, but has not yet been reaped by its parent is
considered a “zombie” process. Such processes continue to count against
limits imposed by the system, and having too many zombie processes
present can prevent additional processes from being spawned.

The tokio runtime will, on a best-effort basis, attempt to reap and
clean up any process which it has spawned. No additional guarantees are
made with regard to how quickly or how often this procedure will take
place.

It is recommended to avoid dropping a
[`Child`](struct.Child.html "struct tokio::process::Child") process
handle before it has been fully `await`ed if stricter cleanup guarantees
are required.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Child.html" class="struct"
title="struct tokio::process::Child">Child</a>  
Representation of a child process spawned onto an event loop.

<a href="struct.ChildStderr.html" class="struct"
title="struct tokio::process::ChildStderr">ChildStderr</a>  
The standard error stream for spawned children.

<a href="struct.ChildStdin.html" class="struct"
title="struct tokio::process::ChildStdin">ChildStdin</a>  
The standard input stream for spawned children.

<a href="struct.ChildStdout.html" class="struct"
title="struct tokio::process::ChildStdout">ChildStdout</a>  
The standard output stream for spawned children.

<a href="struct.Command.html" class="struct"
title="struct tokio::process::Command">Command</a>  
This structure mimics the API of
[`std::process::Command`](https://doc.rust-lang.org/1.93.1/std/process/struct.Command.html "struct std::process::Command")
found in the standard library, but replaces functions that create a
process with an asynchronous variant. The main provided asynchronous
functions are
[spawn](struct.Command.html#method.spawn "method tokio::process::Command::spawn"),
[status](struct.Command.html#method.status "method tokio::process::Command::status"),
and
[output](struct.Command.html#method.output "method tokio::process::Command::output").
