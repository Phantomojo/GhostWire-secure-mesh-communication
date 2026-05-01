Crate futures\_task Copy item path

<span class="sub-heading"><a href="../src/futures_task/lib.rs.html#1-50" class="src">Source</a>
</span>

Expand description

Tools for working with tasks.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use core::task::`<a
href="https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Context.html"
class="struct"
title="struct core::task::wake::Context"><code>Context</code></a>`;`

`pub use core::task::`<a href="https://doc.rust-lang.org/1.93.1/core/task/poll/enum.Poll.html"
class="enum" title="enum core::task::poll::Poll"><code>Poll</code></a>`;`

`pub use core::task::`<a
href="https://doc.rust-lang.org/1.93.1/core/task/wake/struct.RawWaker.html"
class="struct"
title="struct core::task::wake::RawWaker"><code>RawWaker</code></a>`;`

`pub use core::task::`<a
href="https://doc.rust-lang.org/1.93.1/core/task/wake/struct.RawWakerVTable.html"
class="struct"
title="struct core::task::wake::RawWakerVTable"><code>RawWakerVTable</code></a>`;`

`pub use core::task::`<a
href="https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Waker.html"
class="struct"
title="struct core::task::wake::Waker"><code>Waker</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.FutureObj.html" class="struct"
title="struct futures_task::FutureObj">FutureObj</a>  
A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + Send + 'a>`.

<a href="struct.LocalFutureObj.html" class="struct"
title="struct futures_task::LocalFutureObj">LocalFutureObj</a>  
A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + 'a>`.

<a href="struct.SpawnError.html" class="struct"
title="struct futures_task::SpawnError">SpawnError</a>  
An error that occurred during spawning.

<a href="struct.WakerRef.html" class="struct"
title="struct futures_task::WakerRef">WakerRef</a>  
A
[`Waker`](https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Waker.html "struct core::task::wake::Waker")
that is only valid for a given lifetime.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ArcWake.html" class="trait"
title="trait futures_task::ArcWake">ArcWake</a>  
A way of waking up a specific task.

<a href="trait.LocalSpawn.html" class="trait"
title="trait futures_task::LocalSpawn">LocalSpawn</a>  
The `LocalSpawn` is similar to
[`Spawn`](trait.Spawn.html "trait futures_task::Spawn"), but allows
spawning futures that don’t implement `Send`.

<a href="trait.Spawn.html" class="trait"
title="trait futures_task::Spawn">Spawn</a>  
The `Spawn` trait allows for pushing futures onto an executor that will
run them to completion.

<a href="trait.UnsafeFutureObj.html" class="trait"
title="trait futures_task::UnsafeFutureObj">UnsafeFutureObj</a>  
A custom implementation of a future trait object for `FutureObj`,
providing a vtable with drop support.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.noop_waker.html" class="fn"
title="fn futures_task::noop_waker">noop_waker</a>  
Create a new
[`Waker`](https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Waker.html "struct core::task::wake::Waker")
which does nothing when `wake()` is called on it.

<a href="fn.noop_waker_ref.html" class="fn"
title="fn futures_task::noop_waker_ref">noop_waker_ref</a>  
Get a static reference to a
[`Waker`](https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Waker.html "struct core::task::wake::Waker")
which does nothing when `wake()` is called on it.

<a href="fn.waker.html" class="fn"
title="fn futures_task::waker">waker</a>  
Creates a
[`Waker`](https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Waker.html "struct core::task::wake::Waker")
from an `Arc<impl ArcWake>`.

<a href="fn.waker_ref.html" class="fn"
title="fn futures_task::waker_ref">waker_ref</a>  
Creates a reference to a
[`Waker`](https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Waker.html "struct core::task::wake::Waker")
from a reference to `Arc<impl ArcWake>`.
