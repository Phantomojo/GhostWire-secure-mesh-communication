Crate backon Copy item path

<span class="sub-heading"><a href="../src/backon/lib.rs.html#1-258" class="src">Source</a>
</span>

Expand description

[![Build
Status](https://img.shields.io/github/actions/workflow/status/Xuanwo/backon/ci.yml?branch=main)](https://github.com/Xuanwo/backon/actions?query=branch%3Amain)
[![Latest
Version](https://img.shields.io/crates/v/backon.svg)](https://crates.io/crates/backon)
[![](https://img.shields.io/discord/1111711408875393035?logo=discord&label=discord)](https://discord.gg/8ARnvtJePD)

<img
src="https://raw.githubusercontent.com/Xuanwo/backon/main/.github/assets/logo.jpeg"
style="width:38.2%" alt="BackON" />

Make **retry** like a built-in feature provided by Rust.

-   **Simple**: Just like a built-in feature:
    `your_fn.retry(ExponentialBuilder::default()).await`.
-   **Flexible**: Supports both blocking and async functions.
-   **Powerful**: Allows control over retry behavior such as
    [`when`](https://docs.rs/backon/latest/backon/struct.Retry.html#method.when)
    and
    [`notify`](https://docs.rs/backon/latest/backon/struct.Retry.html#method.notify).
-   **Customizable**: Supports custom retry strategies like
    [exponential](https://docs.rs/backon/latest/backon/struct.ExponentialBuilder.html),
    [constant](https://docs.rs/backon/latest/backon/struct.ConstantBuilder.html),
    etc.

# <a href="#backoff" class="doc-anchor">§</a>Backoff

Retry in BackON requires a backoff strategy. BackON will accept a
[`BackoffBuilder`](trait.BackoffBuilder.html "trait backon::BackoffBuilder")
which will generate a new
[`Backoff`](trait.Backoff.html "trait backon::Backoff") for each retry.
It also accepts any object that implements
[`Backoff`](trait.Backoff.html "trait backon::Backoff"). You can
therefore easily implement your own custom backoff strategy.

BackON provides several backoff implementations with reasonable
defaults:

-   [`ConstantBuilder`](struct.ConstantBuilder.html "struct backon::ConstantBuilder"):
    backoff with a constant delay, limited to a specific number of
    attempts.
-   [`ExponentialBuilder`](struct.ExponentialBuilder.html "struct backon::ExponentialBuilder"):
    backoff with an exponential delay, also supports jitter.
-   [`FibonacciBuilder`](struct.FibonacciBuilder.html "struct backon::FibonacciBuilder"):
    backoff with a fibonacci delay, also supports jitter.

# <a href="#sleep" class="doc-anchor">§</a>Sleep

Retry in BackON requires an implementation for sleeping, such an
implementation is called a Sleeper, it will implement
[`Sleeper`](trait.Sleeper.html "trait backon::Sleeper") or
[`BlockingSleeper`](trait.BlockingSleeper.html "trait backon::BlockingSleeper")
depending on if it is going to be used in an asynchronous context.

## <a href="#default-sleeper" class="doc-anchor">§</a>Default Sleeper

Currently, BackON has 3 built-in Sleeper implementations for different
environments, they are gated under their own features, which are enabled
by default:

| `Sleeper` | feature | Environment | Asynchronous |
|----|----|----|----|
| [`TokioSleeper`](struct.TokioSleeper.html "struct backon::TokioSleeper") | tokio-sleep | non-wasm32 | Yes |
| \[`GlooTimersSleep`\] | gloo-timers-sleep | wasm32 | Yes |
| \[`FuturesTimerSleeper`\] | futures-timer-sleep | wasm/non-wasm | Yes |
| \[`EmbassySleep`\] | embassy-sleep | no\_std | Yes |
| [`StdSleeper`](struct.StdSleeper.html "struct backon::StdSleeper") | std-blocking-sleep | std | No |

## <a href="#custom-sleeper" class="doc-anchor">§</a>Custom Sleeper

If you do not want to use the built-in Sleeper, you CAN provide a custom
implementation, let’s implement an asynchronous dummy Sleeper that does
not sleep at all. You will find it pretty similar when you implement a
blocking one.

``` {.rust .rust-example-rendered}
use std::time::Duration;

use backon::Sleeper;

/// A dummy `Sleeper` impl that prints then becomes ready!
struct DummySleeper;

impl Sleeper for DummySleeper {
    type Sleep = std::future::Ready<()>;

    fn sleep(&self, dur: Duration) -> Self::Sleep {
        println!("Hello from DummySleeper!");
        std::future::ready(())
    }
}
```

## <a href="#the-empty-sleeper" class="doc-anchor">§</a>The empty Sleeper

If neither feature is enabled nor a custom implementation is provided,
BackON will fallback to the empty sleeper, in which case, a compile-time
error that
`PleaseEnableAFeatureOrProvideACustomSleeper needs to implement Sleeper or BlockingSleeper`
will be raised to remind you to choose or bring a real Sleeper
implementation.

# <a href="#retry" class="doc-anchor">§</a>Retry

For additional examples, please visit \[`docs::examples`\].

## <a href="#retry-an-async-function" class="doc-anchor">§</a>Retry an async function

``` {.rust .rust-example-rendered}
use anyhow::Result;
use backon::ExponentialBuilder;
use backon::Retryable;
use core::time::Duration;

async fn fetch() -> Result<String> {
    Ok("hello, world!".to_string())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let content = fetch
        // Retry with exponential backoff
        .retry(ExponentialBuilder::default())
        // Sleep implementation, default to tokio::time::sleep if `tokio-sleep` has been enabled.
        .sleep(tokio::time::sleep)
        // When to retry
        .when(|e| e.to_string() == "EOF")
        // Notify when retrying
        .notify(|err: &anyhow::Error, dur: Duration| {
            println!("retrying {:?} after {:?}", err, dur);
        })
        .await?;
    println!("fetch succeeded: {}", content);

    Ok(())
}
```

## <a href="#retry-a-blocking-function" class="doc-anchor">§</a>Retry a blocking function

``` {.rust .rust-example-rendered}
use anyhow::Result;
use backon::BlockingRetryable;
use backon::ExponentialBuilder;
use core::time::Duration;

fn fetch() -> Result<String> {
    Ok("hello, world!".to_string())
}

fn main() -> Result<()> {
    let content = fetch
        // Retry with exponential backoff
        .retry(ExponentialBuilder::default())
        // Sleep implementation, default to std::thread::sleep if `std-blocking-sleep` has been enabled.
        .sleep(std::thread::sleep)
        // When to retry
        .when(|e| e.to_string() == "EOF")
        // Notify when retrying
        .notify(|err: &anyhow::Error, dur: Duration| {
            println!("retrying {:?} after {:?}", err, dur);
        })
        .call()?;
    println!("fetch succeeded: {}", content);

    Ok(())
}
```

## <a href="#retry-an-async-function-with-context" class="doc-anchor">§</a>Retry an async function with context

Sometimes users can meet the problem that the async function is needs to
take `FnMut`:

``` language-shell
error: captured variable cannot escape `FnMut` closure body
   --> src/retry.rs:404:27
    |
400 |         let mut test = Test;
    |             -------- variable defined here
...
404 |         let result = { || async { test.hello().await } }
    |                         - ^^^^^^^^----^^^^^^^^^^^^^^^^
    |                         | |       |
    |                         | |       variable captured here
    |                         | returns an `async` block that contains a reference to a captured variable, which then escapes the closure body
    |                         inferred to be a `FnMut` closure
    |
    = note: `FnMut` closures only have access to their captured variables while they are executing...
    = note: ...therefore, they cannot allow references to captured variables to escape
```

`RetryableWithContext` is designed for this, it allows you to pass a
context to the retry function, and return it back after the retry is
done.

``` {.rust .rust-example-rendered}
use anyhow::anyhow;
use anyhow::Result;
use backon::ExponentialBuilder;
use backon::RetryableWithContext;

struct Test;

impl Test {
    async fn hello(&mut self) -> Result<usize> {
        Err(anyhow!("not retryable"))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut test = Test;

    // (Test, Result<usize>)
    let (_, result) = {
        |mut v: Test| async {
            let res = v.hello().await;
            (v, res)
        }
    }
    .retry(ExponentialBuilder::default())
    .context(test)
    .await;

    Ok(())
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BlockingRetry.html" class="struct"
title="struct backon::BlockingRetry">BlockingRetry</a>  
Retry structure generated by
[`BlockingRetryable`](trait.BlockingRetryable.html "trait backon::BlockingRetryable").

<a href="struct.BlockingRetryWithContext.html" class="struct"
title="struct backon::BlockingRetryWithContext">BlockingRetryWithContext</a>  
Retry structure generated by
[`BlockingRetryableWithContext`](trait.BlockingRetryableWithContext.html "trait backon::BlockingRetryableWithContext").

<a href="struct.ConstantBuilder.html" class="struct"
title="struct backon::ConstantBuilder">ConstantBuilder</a>  
ConstantBuilder is used to create a \[`ConstantBackoff`\], providing a
steady delay with a fixed number of retries.

<a href="struct.ExponentialBuilder.html" class="struct"
title="struct backon::ExponentialBuilder">ExponentialBuilder</a>  
ExponentialBuilder is used to construct an \[`ExponentialBackoff`\] that
offers delays with exponential retries.

<a href="struct.FibonacciBuilder.html" class="struct"
title="struct backon::FibonacciBuilder">FibonacciBuilder</a>  
FibonacciBuilder is used to build a \[`FibonacciBackoff`\] which offers
a delay with Fibonacci-based retries.

<a href="struct.Retry.html" class="struct"
title="struct backon::Retry">Retry</a>  
Struct generated by
[`Retryable`](trait.Retryable.html "trait backon::Retryable").

<a href="struct.RetryWithContext.html" class="struct"
title="struct backon::RetryWithContext">RetryWithContext</a>  
Retry struct generated by
[`RetryableWithContext`](trait.RetryableWithContext.html "trait backon::RetryableWithContext").

<a href="struct.StdSleeper.html" class="struct"
title="struct backon::StdSleeper">StdSleeper</a>  
The implementation of `StdSleeper` uses
[`std::thread::sleep`](https://doc.rust-lang.org/1.93.1/std/thread/functions/fn.sleep.html "fn std::thread::functions::sleep").

<a href="struct.TokioSleeper.html" class="struct"
title="struct backon::TokioSleeper">TokioSleeper</a>  
The default implementation of `Sleeper` uses `tokio::time::sleep`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Backoff.html" class="trait"
title="trait backon::Backoff">Backoff</a>  
Backoff is an
[`Iterator`](https://doc.rust-lang.org/1.93.1/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")
that returns
[`Duration`](https://doc.rust-lang.org/1.93.1/core/time/struct.Duration.html "struct core::time::Duration").

<a href="trait.BackoffBuilder.html" class="trait"
title="trait backon::BackoffBuilder">BackoffBuilder</a>  
BackoffBuilder is utilized to construct a new backoff.

<a href="trait.BlockingRetryable.html" class="trait"
title="trait backon::BlockingRetryable">BlockingRetryable</a>  
BlockingRetryable adds retry support for blocking functions.

<a href="trait.BlockingRetryableWithContext.html" class="trait"
title="trait backon::BlockingRetryableWithContext">BlockingRetryableWithContext</a>  
BlockingRetryableWithContext adds retry support for blocking functions.

<a href="trait.BlockingSleeper.html" class="trait"
title="trait backon::BlockingSleeper">BlockingSleeper</a>  
A sleeper is used sleep for a specified duration.

<a href="trait.Retryable.html" class="trait"
title="trait backon::Retryable">Retryable</a>  
Retryable will add retry support for functions that produce futures with
results.

<a href="trait.RetryableWithContext.html" class="trait"
title="trait backon::RetryableWithContext">RetryableWithContext</a>  
`RetryableWithContext` adds retry support for functions that produce
futures with results and context.

<a href="trait.Sleeper.html" class="trait"
title="trait backon::Sleeper">Sleeper</a>  
A sleeper is used to generate a future that completes after a specified
duration.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.DefaultBlockingSleeper.html" class="type"
title="type backon::DefaultBlockingSleeper">DefaultBlockingSleeper</a>  
The default implementation of `Sleeper` while feature
`std-blocking-sleep` enabled.

<a href="type.DefaultSleeper.html" class="type"
title="type backon::DefaultSleeper">DefaultSleeper</a>  
The default implementation of `Sleeper` while feature `tokio-sleep`
enabled.
