Crate matrix\_sdk\_common Copy item path

<span class="sub-heading"><a href="../src/matrix_sdk_common/lib.rs.html#15-119"
class="src">Source</a> </span>

Expand description

Common types that are used in the different
[matrix-sdk](https://github.com/matrix-org/matrix-rust-sdk/) layers.

**NOTE:** These are common dependencies for the different
[matrix-sdk](https://github.com/matrix-org/matrix-rust-sdk/) layers,
you’re probably interested in the main
[matrix-sdk](https://docs.rs/matrix-sdk/) crate.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../ruma/index.html" class="mod"
title="mod ruma"><code>ruma</code></a>`;`

`pub use cross_process_lock::`<a href="cross_process_lock/constant.LEASE_DURATION_MS.html"
class="constant"
title="constant matrix_sdk_common::cross_process_lock::LEASE_DURATION_MS"><code>LEASE_DURATION_MS</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="cross_process_lock/index.html" class="mod"
title="mod matrix_sdk_common::cross_process_lock">cross_process_lock</a>  
A cross-process lock implementation.

<a href="debug/index.html" class="mod"
title="mod matrix_sdk_common::debug">debug</a>  
Helpers for creating `std::fmt::Debug` implementations.

<a href="deserialized_responses/index.html" class="mod"
title="mod matrix_sdk_common::deserialized_responses">deserialized_responses</a>  
<a href="executor/index.html" class="mod"
title="mod matrix_sdk_common::executor">executor</a>  
Abstraction over an executor so we can spawn tasks under Wasm the same
way we do usually.

<a href="failures_cache/index.html" class="mod"
title="mod matrix_sdk_common::failures_cache">failures_cache</a>  
A TTL cache which can be used to time out repeated operations that might
experience intermittent failures.

<a href="linked_chunk/index.html" class="mod"
title="mod matrix_sdk_common::linked_chunk">linked_chunk</a>  
A linked chunk is the underlying data structure that holds all events.

<a href="locks/index.html" class="mod"
title="mod matrix_sdk_common::locks">locks</a>  
Simplified locks hat panic instead of returning a `Result` when the lock
is poisoned.

<a href="ring_buffer/index.html" class="mod"
title="mod matrix_sdk_common::ring_buffer">ring_buffer</a>  
<a href="serde_helpers/index.html" class="mod"
title="mod matrix_sdk_common::serde_helpers">serde_helpers</a>  
A collection of serde helpers to avoid having to deserialize an entire
event to access some fields.

<a href="sleep/index.html" class="mod"
title="mod matrix_sdk_common::sleep">sleep</a>  
<a href="stream/index.html" class="mod"
title="mod matrix_sdk_common::stream">stream</a>  
Platform-specific stream utilities.

<a href="timeout/index.html" class="mod"
title="mod matrix_sdk_common::timeout">timeout</a>  
<a href="tracing_timer/index.html" class="mod"
title="mod matrix_sdk_common::tracing_timer">tracing_timer</a>  
<a href="ttl_cache/index.html" class="mod"
title="mod matrix_sdk_common::ttl_cache">ttl_cache</a>  
A TTL cache which can be used to time out repeated operations that might
experience intermittent failures.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.boxed_into_future.html" class="macro"
title="macro matrix_sdk_common::boxed_into_future">boxed_into_future</a>  
<a href="macro.timer.html" class="macro"
title="macro matrix_sdk_common::timer">timer</a>  
Macro to create a RAII timer that will log on `Drop` how long its
covered section took to execute.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ROOM_VERSION_FALLBACK.html" class="constant"
title="constant matrix_sdk_common::ROOM_VERSION_FALLBACK">ROOM_VERSION_FALLBACK</a>  
The room version to use as a fallback when the version of a room is
unknown.

<a href="constant.ROOM_VERSION_RULES_FALLBACK.html" class="constant"
title="constant matrix_sdk_common::ROOM_VERSION_RULES_FALLBACK">ROOM_VERSION_RULES_FALLBACK</a>  
The room version rules to use as a fallback when the version of a room
is unknown or unsupported.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsyncTraitDeps.html" class="trait"
title="trait matrix_sdk_common::AsyncTraitDeps">AsyncTraitDeps</a>  
Super trait that is used for our store traits, this trait will differ if
it’s used on WASM. WASM targets will not require `Send` and `Sync` to
have implemented, while other targets will.

<a href="trait.SendOutsideWasm.html" class="trait"
title="trait matrix_sdk_common::SendOutsideWasm">SendOutsideWasm</a>  
Alias for `Send` on non-wasm, empty trait (implemented by everything) on
wasm.

<a href="trait.SyncOutsideWasm.html" class="trait"
title="trait matrix_sdk_common::SyncOutsideWasm">SyncOutsideWasm</a>  
Alias for `Sync` on non-wasm, empty trait (implemented by everything) on
wasm.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BoxFuture.html" class="type"
title="type matrix_sdk_common::BoxFuture">BoxFuture</a>
