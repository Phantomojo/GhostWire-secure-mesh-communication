Crate slab Copy item path

<span class="sub-heading"><a href="../src/slab/lib.rs.html#1-1644" class="src">Source</a>
</span>

Expand description

Pre-allocated storage for a uniform data type.

`Slab` provides pre-allocated storage for a single data type. If many
values of a single type are being allocated, it can be more efficient to
pre-allocate the necessary storage. Since the size of the type is
uniform, memory fragmentation can be avoided. Storing, clearing, and
lookup operations become very cheap.

While `Slab` may look like other Rust collections, it is not intended to
be used as a general purpose collection. The primary difference between
`Slab` and `Vec` is that `Slab` returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a
value associated with a given key is removed from a slab, that key may
be returned from future calls to `insert`.

# <a href="#examples" class="doc-anchor">§</a>Examples

Basic storing and retrieval.

``` {.rust .rust-example-rendered}
let mut slab = Slab::new();

let hello = slab.insert("hello");
let world = slab.insert("world");

assert_eq!(slab[hello], "hello");
assert_eq!(slab[world], "world");

slab[world] = "earth";
assert_eq!(slab[world], "earth");
```

Sometimes it is useful to be able to associate the key with the value
being inserted in the slab. This can be done with the `vacant_entry` API
as such:

``` {.rust .rust-example-rendered}
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab[hello].0);
assert_eq!("hello", slab[hello].1);
```

It is generally a good idea to specify the desired capacity of a slab at
creation time. Note that `Slab` will grow the internal capacity when
attempting to insert a new value once the existing capacity has been
reached. To avoid this, add a check.

``` {.rust .rust-example-rendered}
let mut slab = Slab::with_capacity(1024);

// ... use the slab

if slab.len() == slab.capacity() {
    panic!("slab full");
}

slab.insert("the slab is not at capacity yet");
```

# <a href="#capacity-and-reallocation" class="doc-anchor">§</a>Capacity and reallocation

The capacity of a slab is the amount of space allocated for any future
values that will be inserted in the slab. This is not to be confused
with the *length* of the slab, which specifies the number of actual
values currently being inserted. If a slab’s length is equal to its
capacity, the next value inserted into the slab will require growing the
slab by reallocating.

For example, a slab with capacity 10 and length 0 would be an empty slab
with space for 10 more stored values. Storing 10 or fewer elements into
the slab will not change its capacity or cause reallocation to occur.
However, if the slab length is increased to 11 (due to another
`insert`), it will have to reallocate, which can be slow. For this
reason, it is recommended to use
[`Slab::with_capacity`](struct.Slab.html#with_capacity) whenever
possible to specify how many values the slab is expected to store.

# <a href="#implementation" class="doc-anchor">§</a>Implementation

`Slab` is backed by a `Vec` of slots. Each slot is either occupied or
vacant. `Slab` maintains a stack of vacant slots using a linked list. To
find a vacant slot, the stack is popped. When a slot is released, it is
pushed onto the stack.

If there are no more available slots in the stack, then
`Vec::reserve(1)` is called and a new slot is created.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Drain.html" class="struct"
title="struct slab::Drain">Drain</a>  
A draining iterator for `Slab`

<a href="struct.IntoIter.html" class="struct"
title="struct slab::IntoIter">IntoIter</a>  
A consuming iterator over the values stored in a `Slab`

<a href="struct.Iter.html" class="struct"
title="struct slab::Iter">Iter</a>  
An iterator over the values stored in the `Slab`

<a href="struct.IterMut.html" class="struct"
title="struct slab::IterMut">IterMut</a>  
A mutable iterator over the values stored in the `Slab`

<a href="struct.Slab.html" class="struct"
title="struct slab::Slab">Slab</a>  
Pre-allocated storage for a uniform data type

<a href="struct.VacantEntry.html" class="struct"
title="struct slab::VacantEntry">VacantEntry</a>  
A handle to a vacant entry in a `Slab`.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.GetDisjointMutError.html" class="enum"
title="enum slab::GetDisjointMutError">GetDisjointMutError</a>  
The error type returned by
[`Slab::get_disjoint_mut`](struct.Slab.html#method.get_disjoint_mut "method slab::Slab::get_disjoint_mut").
