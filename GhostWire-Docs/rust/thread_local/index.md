Crate thread\_local Copy item path

<span class="sub-heading"><a href="../src/thread_local/lib.rs.html#8-683" class="src">Source</a>
</span>

Expand description

Per-object thread-local storage

This library provides the `ThreadLocal` type which allows a separate
copy of an object to be used for each thread. This allows for per-object
thread-local storage, unlike the standard library’s `thread_local!`
macro which only allows static thread-local storage.

Per-thread objects are not destroyed when a thread exits. Instead,
objects are only destroyed when the `ThreadLocal` containing them is
destroyed.

You can also iterate over the thread-local values of all thread in a
`ThreadLocal` object using the `iter_mut` and `into_iter` methods. This
can only be done if you have mutable access to the `ThreadLocal` object,
which guarantees that you are the only thread currently accessing it.

Note that since thread IDs are recycled when a thread exits, it is
possible for one thread to retrieve the object of another thread. Since
this can only occur after a thread has exited this does not lead to any
race conditions.

# <a href="#examples" class="doc-anchor">§</a>Examples

Basic usage of `ThreadLocal`:

``` {.rust .rust-example-rendered}
use thread_local::ThreadLocal;
let tls: ThreadLocal<u32> = ThreadLocal::new();
assert_eq!(tls.get(), None);
assert_eq!(tls.get_or(|| 5), &5);
assert_eq!(tls.get(), Some(&5));
```

Combining thread-local values into a single result:

``` {.rust .rust-example-rendered}
use thread_local::ThreadLocal;
use std::sync::Arc;
use std::cell::Cell;
use std::thread;

let tls = Arc::new(ThreadLocal::new());

// Create a bunch of threads to do stuff
for _ in 0..5 {
    let tls2 = tls.clone();
    thread::spawn(move || {
        // Increment a counter to count some event...
        let cell = tls2.get_or(|| Cell::new(0));
        cell.set(cell.get() + 1);
    }).join().unwrap();
}

// Once all threads are done, collect the counter values and return the
// sum of all thread-local counter values.
let tls = Arc::try_unwrap(tls).unwrap();
let total = tls.into_iter().fold(0, |x, y| x + y.get());
assert_eq!(total, 5);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CachedIntoIter.html" class="struct"
title="struct thread_local::CachedIntoIter">CachedIntoIter</a><span class="stab deprecated" title="">Deprecated</span>  
An iterator that moves out of a `CachedThreadLocal`.

<a href="struct.CachedIterMut.html" class="struct"
title="struct thread_local::CachedIterMut">CachedIterMut</a><span class="stab deprecated" title="">Deprecated</span>  
Mutable iterator over the contents of a `CachedThreadLocal`.

<a href="struct.CachedThreadLocal.html" class="struct"
title="struct thread_local::CachedThreadLocal">CachedThreadLocal</a><span class="stab deprecated" title="">Deprecated</span>  
Wrapper around
[`ThreadLocal`](struct.ThreadLocal.html "struct thread_local::ThreadLocal").

<a href="struct.IntoIter.html" class="struct"
title="struct thread_local::IntoIter">IntoIter</a>  
An iterator that moves out of a `ThreadLocal`.

<a href="struct.Iter.html" class="struct"
title="struct thread_local::Iter">Iter</a>  
Iterator over the contents of a `ThreadLocal`.

<a href="struct.IterMut.html" class="struct"
title="struct thread_local::IterMut">IterMut</a>  
Mutable iterator over the contents of a `ThreadLocal`.

<a href="struct.ThreadLocal.html" class="struct"
title="struct thread_local::ThreadLocal">ThreadLocal</a>  
Thread-local variable wrapper
