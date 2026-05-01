Crate fallible\_streaming\_iterator Copy item path

<span class="sub-heading"><a href="../src/fallible_streaming_iterator/lib.rs.html#1-899"
class="src">Source</a> </span>

Expand description

Fallible, streaming iteration.

`FallibleStreamingIterator` differs from the standard library’s
`Iterator` trait in two ways: iteration can fail, resulting in an error,
and only one element of the iteration is available at any time.

While these iterators cannot be used with Rust `for` loops, `while let`
loops offer a similar level of ergonomics:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
while let Some(value) = it.next()? {
    // use value
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Convert.html" class="struct"
title="struct fallible_streaming_iterator::Convert">Convert</a>  
An iterator which wraps a normal `Iterator`.

<a href="struct.Empty.html" class="struct"
title="struct fallible_streaming_iterator::Empty">Empty</a>  
An iterator over no items.

<a href="struct.Filter.html" class="struct"
title="struct fallible_streaming_iterator::Filter">Filter</a>  
An iterator which filters elements with a predicate.

<a href="struct.Fuse.html" class="struct"
title="struct fallible_streaming_iterator::Fuse">Fuse</a>  
An iterator which is well-behaved at the beginning and end of iteration.

<a href="struct.Map.html" class="struct"
title="struct fallible_streaming_iterator::Map">Map</a>  
An iterator which applies a transform to elements.

<a href="struct.MapErr.html" class="struct"
title="struct fallible_streaming_iterator::MapErr">MapErr</a>  
An iterator which applies a transform to errors.

<a href="struct.MapRef.html" class="struct"
title="struct fallible_streaming_iterator::MapRef">MapRef</a>  
An iterator which applies a transform to elements.

<a href="struct.Skip.html" class="struct"
title="struct fallible_streaming_iterator::Skip">Skip</a>  
An iterator which skips a number of initial elements.

<a href="struct.SkipWhile.html" class="struct"
title="struct fallible_streaming_iterator::SkipWhile">SkipWhile</a>  
An iterator which skips initial elements matching a predicate.

<a href="struct.Take.html" class="struct"
title="struct fallible_streaming_iterator::Take">Take</a>  
An iterator which only returns a number of initial elements.

<a href="struct.TakeWhile.html" class="struct"
title="struct fallible_streaming_iterator::TakeWhile">TakeWhile</a>  
An iterator which only returns initial elements matching a predicate.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DoubleEndedFallibleStreamingIterator.html" class="trait"
title="trait fallible_streaming_iterator::DoubleEndedFallibleStreamingIterator">DoubleEndedFallibleStreamingIterator</a>  
A fallible, streaming iterator which can be advanced from either end.

<a href="trait.FallibleStreamingIterator.html" class="trait"
title="trait fallible_streaming_iterator::FallibleStreamingIterator">FallibleStreamingIterator</a>  
A fallible, streaming iterator.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.convert.html" class="fn"
title="fn fallible_streaming_iterator::convert">convert</a>  
Converts a normal `Iterator` over `Results` of references into a
`FallibleStreamingIterator`.

<a href="fn.empty.html" class="fn"
title="fn fallible_streaming_iterator::empty">empty</a>  
Returns an iterator over no items.
