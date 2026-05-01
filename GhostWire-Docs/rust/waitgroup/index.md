Crate waitgroup Copy item path

<span class="sub-heading"><a href="../src/waitgroup/lib.rs.html#1-125" class="src">Source</a>
</span>

Expand description

A WaitGroup waits for a collection of task to finish.

## <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use waitgroup::WaitGroup;
use async_std::task;
async {
    let wg = WaitGroup::new();
    for _ in 0..100 {
        let w = wg.worker();
        task::spawn(async move {
            // do work
            drop(w); // drop w means task finished
        });
    }

    wg.wait().await;
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.WaitGroup.html" class="struct"
title="struct waitgroup::WaitGroup">WaitGroup</a>

<a href="struct.WaitGroupFuture.html" class="struct"
title="struct waitgroup::WaitGroupFuture">WaitGroupFuture</a>

<a href="struct.Worker.html" class="struct"
title="struct waitgroup::Worker">Worker</a>
