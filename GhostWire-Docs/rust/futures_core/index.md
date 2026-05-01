Crate futures\_core Copy item path

<span class="sub-heading"><a href="../src/futures_core/lib.rs.html#1-27" class="src">Source</a>
</span>

Expand description

Core traits and types for asynchronous operations in Rust.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::future::`<a href="future/trait.FusedFuture.html" class="trait"
title="trait futures_core::future::FusedFuture"><code>FusedFuture</code></a>`;`

`pub use self::future::`<a
href="https://doc.rust-lang.org/1.93.1/core/future/future/trait.Future.html"
class="trait"
title="trait core::future::future::Future"><code>Future</code></a>`;`

`pub use self::future::`<a href="future/trait.TryFuture.html" class="trait"
title="trait futures_core::future::TryFuture"><code>TryFuture</code></a>`;`

`pub use self::stream::`<a href="stream/trait.FusedStream.html" class="trait"
title="trait futures_core::stream::FusedStream"><code>FusedStream</code></a>`;`

`pub use self::stream::`<a href="stream/trait.Stream.html" class="trait"
title="trait futures_core::stream::Stream"><code>Stream</code></a>`;`

`pub use self::stream::`<a href="stream/trait.TryStream.html" class="trait"
title="trait futures_core::stream::TryStream"><code>TryStream</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="future/index.html" class="mod"
title="mod futures_core::future">future</a>  
Futures.

<a href="stream/index.html" class="mod"
title="mod futures_core::stream">stream</a>  
Asynchronous streams.

<a href="task/index.html" class="mod"
title="mod futures_core::task">task</a>  
Task notification.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.ready.html" class="macro"
title="macro futures_core::ready">ready</a>  
Extracts the successful type of a `Poll<T>`.
