Crate fallible\_iterator Copy item path

<span class="sub-heading"><a href="../src/fallible_iterator/lib.rs.html#1-2808"
class="src">Source</a> </span>

Expand description

“Fallible” iterators.

The iterator APIs in the Rust standard library do not support iteration
that can fail in a first class manner. These iterators are typically
modeled as iterating over `Result<T, E>` values; for example, the
`Lines` iterator returns `io::Result<String>`s. When simply iterating
over these types, the value being iterated over must be unwrapped in
some way before it can be used:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
for line in reader.lines() {
    let line = line?;
    // work with line
}
```

In addition, many of the additional methods on the `Iterator` trait will
not behave properly in the presence of errors when working with these
kinds of iterators. For example, if one wanted to count the number of
lines of text in a `Read`er, this might be a way to go about it:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
let count = reader.lines().count();
```

This will return the proper value when the reader operates successfully,
but if it encounters an IO error, the result will either be slightly
higher than expected if the error is transient, or it may run forever if
the error is returned repeatedly!

In contrast, a fallible iterator is built around the concept that a call
to `next` can fail. The trait has an additional `Error` associated type
in addition to the `Item` type, and `next` returns
`Result<Option<Self::Item>, Self::Error>` rather than
`Option<Self::Item>`. Methods like `count` return `Result`s as well.

This does mean that fallible iterators are incompatible with Rust’s
`for` loop syntax, but `while let` loops offer a similar level of
ergonomics:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
while let Some(item) = iter.next()? {
    // work with item
}
```

## <a href="#fallible-closure-arguments" class="doc-anchor">§</a>Fallible closure arguments

Like `Iterator`, many `FallibleIterator` methods take closures as
arguments. These use the same signatures as their `Iterator`
counterparts, except that `FallibleIterator` expects the closures to be
fallible: they return `Result<T, Self::Error>` instead of simply `T`.

For example, the standard library’s `Iterator::filter` adapter method
filters the underlying iterator according to a predicate provided by the
user, whose return type is `bool`. In `FallibleIterator::filter`,
however, the predicate returns `Result<bool, Self::Error>`:

``` {.rust .rust-example-rendered}
let numbers = convert("100\n200\nfern\n400".lines().map(Ok::<&str, Box<Error>>));
let big_numbers = numbers.filter(|n| Ok(u64::from_str(n)? > 100));
assert!(big_numbers.count().is_err());
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Chain.html" class="struct"
title="struct fallible_iterator::Chain">Chain</a>  
An iterator which yields the elements of one iterator followed by
another.

<a href="struct.Cloned.html" class="struct"
title="struct fallible_iterator::Cloned">Cloned</a>  
An iterator which clones the elements of the underlying iterator.

<a href="struct.Convert.html" class="struct"
title="struct fallible_iterator::Convert">Convert</a>  
A fallible iterator that wraps a normal iterator over `Result`s.

<a href="struct.Cycle.html" class="struct"
title="struct fallible_iterator::Cycle">Cycle</a>  
An iterator which cycles another endlessly.

<a href="struct.Empty.html" class="struct"
title="struct fallible_iterator::Empty">Empty</a>  
An iterator that yields nothing.

<a href="struct.Enumerate.html" class="struct"
title="struct fallible_iterator::Enumerate">Enumerate</a>  
An iterator that yields the iteration count as well as the values of the
underlying iterator.

<a href="struct.Filter.html" class="struct"
title="struct fallible_iterator::Filter">Filter</a>  
An iterator which uses a fallible predicate to determine which values of
the underlying iterator should be yielded.

<a href="struct.FilterMap.html" class="struct"
title="struct fallible_iterator::FilterMap">FilterMap</a>  
An iterator which both filters and maps the values of the underlying
iterator.

<a href="struct.FlatMap.html" class="struct"
title="struct fallible_iterator::FlatMap">FlatMap</a>  
An iterator which maps each element to another iterator, yielding those
iterator’s elements.

<a href="struct.Flatten.html" class="struct"
title="struct fallible_iterator::Flatten">Flatten</a>  
An iterator which flattens an iterator of iterators, yielding those
iterators’ elements.

<a href="struct.FromFn.html" class="struct"
title="struct fallible_iterator::FromFn">FromFn</a>  
An iterator using a function to generate new values.

<a href="struct.Fuse.html" class="struct"
title="struct fallible_iterator::Fuse">Fuse</a>  
An iterator that yields `Ok(None)` forever after the underlying iterator
yields `Ok(None)` once.

<a href="struct.Inspect.html" class="struct"
title="struct fallible_iterator::Inspect">Inspect</a>  
An iterator which passes each element to a closure before returning it.

<a href="struct.IntoFallible.html" class="struct"
title="struct fallible_iterator::IntoFallible">IntoFallible</a>  
A fallible iterator that wraps a normal iterator over `Result`s.

<a href="struct.Iterator.html" class="struct"
title="struct fallible_iterator::Iterator">Iterator</a>  
A normal (non-fallible) iterator which wraps a fallible iterator.

<a href="struct.Map.html" class="struct"
title="struct fallible_iterator::Map">Map</a>  
An iterator which applies a fallible transform to the elements of the
underlying iterator.

<a href="struct.MapErr.html" class="struct"
title="struct fallible_iterator::MapErr">MapErr</a>  
An iterator which applies a transform to the errors of the underlying
iterator.

<a href="struct.Once.html" class="struct"
title="struct fallible_iterator::Once">Once</a>  
An iterator that yields something exactly once.

<a href="struct.OnceErr.html" class="struct"
title="struct fallible_iterator::OnceErr">OnceErr</a>  
An iterator that fails with a predetermined error exactly once.

<a href="struct.Peekable.html" class="struct"
title="struct fallible_iterator::Peekable">Peekable</a>  
An iterator which can look at the next element without consuming it.

<a href="struct.Repeat.html" class="struct"
title="struct fallible_iterator::Repeat">Repeat</a>  
An iterator that endlessly repeats a single element.

<a href="struct.RepeatErr.html" class="struct"
title="struct fallible_iterator::RepeatErr">RepeatErr</a>  
An iterator that endlessly repeats a single error.

<a href="struct.Rev.html" class="struct"
title="struct fallible_iterator::Rev">Rev</a>  
An iterator which yields elements of the underlying iterator in reverse
order.

<a href="struct.Scan.html" class="struct"
title="struct fallible_iterator::Scan">Scan</a>  
An iterator which applies a stateful closure.

<a href="struct.Skip.html" class="struct"
title="struct fallible_iterator::Skip">Skip</a>  
An iterator which skips initial elements.

<a href="struct.SkipWhile.html" class="struct"
title="struct fallible_iterator::SkipWhile">SkipWhile</a>  
An iterator which skips initial elements based on a predicate.

<a href="struct.StepBy.html" class="struct"
title="struct fallible_iterator::StepBy">StepBy</a>  
An iterator which steps through the elements of the underlying iterator
by a certain amount.

<a href="struct.Take.html" class="struct"
title="struct fallible_iterator::Take">Take</a>  
An iterator which yields a limited number of elements from the
underlying iterator.

<a href="struct.TakeWhile.html" class="struct"
title="struct fallible_iterator::TakeWhile">TakeWhile</a>  
An iterator which yields elements based on a predicate.

<a href="struct.Unwrap.html" class="struct"
title="struct fallible_iterator::Unwrap">Unwrap</a>  
An iterator that unwraps every element yielded by the underlying
FallibleIterator

<a href="struct.Zip.html" class="struct"
title="struct fallible_iterator::Zip">Zip</a>  
An iterator that yields pairs of this iterator’s and another iterator’s
values.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DoubleEndedFallibleIterator.html" class="trait"
title="trait fallible_iterator::DoubleEndedFallibleIterator">DoubleEndedFallibleIterator</a>  
A fallible iterator able to yield elements from both ends.

<a href="trait.FallibleIterator.html" class="trait"
title="trait fallible_iterator::FallibleIterator">FallibleIterator</a>  
An `Iterator`-like trait that allows for calculation of items to fail.

<a href="trait.IntoFallibleIterator.html" class="trait"
title="trait fallible_iterator::IntoFallibleIterator">IntoFallibleIterator</a>  
Conversion into a `FallibleIterator`.

<a href="trait.IteratorExt.html" class="trait"
title="trait fallible_iterator::IteratorExt">IteratorExt</a>  
An extnsion-trait with set of useful methods to convert
[`core::iter::Iterator`](https://doc.rust-lang.org/1.93.1/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")
into
[`FallibleIterator`](trait.FallibleIterator.html "trait fallible_iterator::FallibleIterator")

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.convert.html" class="fn"
title="fn fallible_iterator::convert">convert</a>  
Converts an `Iterator<Item = Result<T, E>>` into a
`FallibleIterator<Item = T, Error = E>`.

<a href="fn.empty.html" class="fn"
title="fn fallible_iterator::empty">empty</a>  
Creates an iterator that yields nothing.

<a href="fn.from_fn.html" class="fn"
title="fn fallible_iterator::from_fn">from_fn</a>  
Creates an iterator from a fallible function generating values.

<a href="fn.once.html" class="fn"
title="fn fallible_iterator::once">once</a>  
Creates an iterator that yields an element exactly once.

<a href="fn.once_err.html" class="fn"
title="fn fallible_iterator::once_err">once_err</a>  
Creates an iterator that fails with a predetermined error exactly once.

<a href="fn.repeat.html" class="fn"
title="fn fallible_iterator::repeat">repeat</a>  
Creates an iterator that endlessly repeats a single element.

<a href="fn.repeat_err.html" class="fn"
title="fn fallible_iterator::repeat_err">repeat_err</a>  
Creates an iterator that endlessly repeats a single error.
