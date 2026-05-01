Crate eyeball Copy item path

<span class="sub-heading"><a href="../src/eyeball/lib.rs.html#1-93" class="src">Source</a>
</span>

Expand description

Add observability to your Rust types!

This crate implements a basic form of the [Observer
pattern](https://en.wikipedia.org/wiki/Observer_pattern) for Rust. It
provides
[`Observable<T>`](struct.Observable.html "struct eyeball::Observable")
as a type that semi-transparently wraps an inner value `T` and
broadcasts changes to any associated
[`Subscriber<T>`](struct.Subscriber.html "struct eyeball::Subscriber")s.
`Subscriber`s can currently only be polled for updates using `async` /
`.await`, but this may change in the future.

There is also
[`SharedObservable<T>`](struct.SharedObservable.html "struct eyeball::SharedObservable")
as another variation which implements
[`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone")
but not
[`Deref`](https://doc.rust-lang.org/1.93.1/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref").
It is more ergonomic and efficient than putting a `Observable` inside of
`Arc<RwLock<_>>` for updating the value from multiple places in the
code.

Here is a quick walk-through:

``` {.rust .rust-example-rendered}
use eyeball::Observable;

let mut observable = Observable::new("A".to_owned());
// Observable has no methods of its own, as those could conflict
// with methods of the inner type, which it `Deref`erences to.
let mut subscriber1 = Observable::subscribe(&observable);
let mut subscriber2 = Observable::subscribe(&observable);

// You can get the current value from a subscriber without waiting
// for updates.
assert_eq!(subscriber1.get(), "A");

Observable::set(&mut observable, "B".to_owned());
// `.next().await` will wait for the next update, then return the
// new value.
assert_eq!(subscriber1.next().await, Some("B".to_owned()));

// If multiple updates have happened without the subscriber being
// polled as is the case for subscriber2 here, the next poll will
// skip all but the latest.
Observable::set(&mut observable, "C".to_owned());
assert_eq!(subscriber1.next().await, Some("C".to_owned()));
assert_eq!(subscriber2.next().await, Some("C".to_owned()));

// You can even obtain the value without cloning the value, by
// using `.read()` (no waiting) or `.next_ref().await` (waits for
// the next update).
// If you restrict yourself to these methods, you can even use
// `Observable` with inner types that don't implement the `Clone`
// trait.
// However, note that while a read guard returned by `.read()` or
// `.next_ref().await` is alive, updating the observable is
// blocked.
Observable::set(&mut observable, "D".to_owned());
{
    let guard = subscriber1.next_ref().await.unwrap();
    assert_eq!(*guard, "D");
}

// The latest value is kept alive by subscribers when the
// `Observable` is dropped.
drop(observable);
assert_eq!(subscriber1.get(), "D");
assert_eq!(*subscriber2.read(), "D");
```

Cargo features:

-   `tracing`: Emit [tracing](../tracing/index.html "mod tracing")
    events when updates are sent out

# Modules<a href="#modules" class="anchor">§</a>

<a href="subscriber/index.html" class="mod"
title="mod eyeball::subscriber">subscriber</a>  
Details of observable
[`Subscriber`](struct.Subscriber.html "struct eyeball::Subscriber")s.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Observable.html" class="struct"
title="struct eyeball::Observable">Observable</a>  
A value whose changes will be broadcast to subscribers.

<a href="struct.ObservableReadGuard.html" class="struct"
title="struct eyeball::ObservableReadGuard">ObservableReadGuard</a>  
A read guard for the inner value of an observable.

<a href="struct.ObservableWriteGuard.html" class="struct"
title="struct eyeball::ObservableWriteGuard">ObservableWriteGuard</a>  
A write guard for the inner value of an observable.

<a href="struct.SharedObservable.html" class="struct"
title="struct eyeball::SharedObservable">SharedObservable</a>  
A value whose changes will be broadcast to subscribers.

<a href="struct.Subscriber.html" class="struct"
title="struct eyeball::Subscriber">Subscriber</a>  
A subscriber for updates of an `Observable`.

<a href="struct.WeakObservable.html" class="struct"
title="struct eyeball::WeakObservable">WeakObservable</a>  
A weak reference to a
[`SharedObservable`](struct.SharedObservable.html "struct eyeball::SharedObservable").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AsyncLock.html" class="enum"
title="enum eyeball::AsyncLock">AsyncLock</a>  
Marker type for using an asynchronous lock for the inner value.

<a href="enum.SyncLock.html" class="enum"
title="enum eyeball::SyncLock">SyncLock</a>  
Marker type for using a synchronous lock for the inner value.
