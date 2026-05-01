Crate sync\_wrapper Copy item path

<span class="sub-heading"><a href="../src/sync_wrapper/lib.rs.html#16-258" class="src">Source</a>
</span>

Expand description

A mutual exclusion primitive that relies on static type information only

This library is inspired by [this
discussion](https://internals.rust-lang.org/t/what-shall-sync-mean-across-an-await/12020/2).

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SyncFuture.html" class="struct"
title="struct sync_wrapper::SyncFuture">SyncFuture</a>  
`Future` which is `Sync`.

<a href="struct.SyncStream.html" class="struct"
title="struct sync_wrapper::SyncStream">SyncStream</a>  
`Stream` which is `Sync`.

<a href="struct.SyncWrapper.html" class="struct"
title="struct sync_wrapper::SyncWrapper">SyncWrapper</a>  
A mutual exclusion primitive that relies on static type information only
