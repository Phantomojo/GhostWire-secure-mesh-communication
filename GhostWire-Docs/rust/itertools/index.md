Crate itertools Copy item path

<span class="sub-heading"><a href="../src/itertools/lib.rs.html#1-4713" class="src">Source</a>
</span>

Expand description

Extra iterator adaptors, functions and macros.

To extend
[`Iterator`](https://doc.rust-lang.org/1.93.1/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")
with methods in this crate, import the
[`Itertools`](trait.Itertools.html "trait itertools::Itertools") trait:

``` {.rust .rust-example-rendered}
use itertools::Itertools;
```

Now, new methods like
[`interleave`](trait.Itertools.html#method.interleave "method itertools::Itertools::interleave")
are available on all iterators:

``` {.rust .rust-example-rendered}
use itertools::Itertools;

let it = (1..3).interleave(vec![-1, -2]);
itertools::assert_equal(it, vec![1, -1, 2, -2]);
```

Most iterator methods are also provided as functions (with the benefit
that they convert parameters using
[`IntoIterator`](https://doc.rust-lang.org/1.93.1/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")):

``` {.rust .rust-example-rendered}
use itertools::interleave;

for elt in interleave(&[1, 2, 3], &[2, 3, 4]) {
    /* loop body */
}
```

## <a href="#crate-features" class="doc-anchor">§</a>Crate Features

-   `use_std`
    -   Enabled by default.
    -   Disable to compile itertools using `#![no_std]`. This disables
        any item that depend on allocations (see the `use_alloc`
        feature) and hash maps (like `unique`, `counts`,
        `into_grouping_map` and more).
-   `use_alloc`
    -   Enabled by default.
    -   Enables any item that depend on allocations (like `chunk_by`,
        `kmerge`, `join` and many more).

## <a href="#rust-version" class="doc-anchor">§</a>Rust Version

This version of itertools requires Rust 1.63.0 or later.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::`<a href="structs/index.html" class="mod"
title="mod itertools::structs"><code>structs</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="structs/index.html" class="mod"
title="mod itertools::structs">structs</a>  
The concrete iterator types.

<a href="traits/index.html" class="mod"
title="mod itertools::traits">traits</a>  
Traits helpful for using certain `Itertools` methods in generic
contexts.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.chain.html" class="macro"
title="macro itertools::chain">chain</a>  
[Chain](https://doc.rust-lang.org/1.93.1/core/iter/traits/iterator/trait.Iterator.html#method.chain "method core::iter::traits::iterator::Iterator::chain")
zero or more iterators together into one sequence.

<a href="macro.iproduct.html" class="macro"
title="macro itertools::iproduct">iproduct</a>  
Create an iterator over the “cartesian product” of iterators.

<a href="macro.izip.html" class="macro"
title="macro itertools::izip">izip</a>  
Create an iterator running multiple iterators in lockstep.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Diff.html" class="enum"
title="enum itertools::Diff">Diff</a>  
A type returned by the
[`diff_with`](fn.diff_with.html "fn itertools::diff_with") function.

<a href="enum.Either.html" class="enum"
title="enum itertools::Either">Either</a>  
The enum `Either` with variants `Left` and `Right` is a general purpose
sum type with two cases.

<a href="enum.EitherOrBoth.html" class="enum"
title="enum itertools::EitherOrBoth">EitherOrBoth</a>  
Value that either holds a single A or B, or both.

<a href="enum.FoldWhile.html" class="enum"
title="enum itertools::FoldWhile">FoldWhile</a>  
An enum used for controlling the execution of `fold_while`.

<a href="enum.MinMaxResult.html" class="enum"
title="enum itertools::MinMaxResult">MinMaxResult</a>  
`MinMaxResult` is an enum returned by `minmax`.

<a href="enum.Position.html" class="enum"
title="enum itertools::Position">Position</a>  
The first component of the value yielded by `WithPosition`. Indicates
the position of this element in the iterator results.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Itertools.html" class="trait"
title="trait itertools::Itertools">Itertools</a>  
An
[`Iterator`](https://doc.rust-lang.org/1.93.1/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")
blanket implementation that provides extra adaptors and methods.

<a href="trait.MultiUnzip.html" class="trait"
title="trait itertools::MultiUnzip">MultiUnzip</a>  
An iterator that can be unzipped into multiple collections.

<a href="trait.PeekingNext.html" class="trait"
title="trait itertools::PeekingNext">PeekingNext</a>  
An iterator that allows peeking at an element before deciding to accept
it.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.all.html" class="fn" title="fn itertools::all">all</a>  
Test whether the predicate holds for all elements in the iterable.

<a href="fn.any.html" class="fn" title="fn itertools::any">any</a>  
Test whether the predicate holds for any elements in the iterable.

<a href="fn.assert_equal.html" class="fn"
title="fn itertools::assert_equal">assert_equal</a>  
Assert that two iterables produce equal sequences, with the same
semantics as [`equal(a, b)`](fn.equal.html "fn itertools::equal").

<a href="fn.chain.html" class="fn" title="fn itertools::chain">chain</a>  
Takes two iterables and creates a new iterator over both in sequence.

<a href="fn.cloned.html" class="fn"
title="fn itertools::cloned">cloned</a>  
Create an iterator that clones each element from `&T` to `T`.

<a href="fn.concat.html" class="fn"
title="fn itertools::concat">concat</a>  
Combine all an iterator’s elements into one element by using
[`Extend`](https://doc.rust-lang.org/1.93.1/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend").

<a href="fn.cons_tuples.html" class="fn"
title="fn itertools::cons_tuples">cons_tuples</a>  
Create an iterator that maps for example iterators of `((A, B), C)` to
`(A, B, C)`.

<a href="fn.diff_with.html" class="fn"
title="fn itertools::diff_with">diff_with</a>  
Compares every element yielded by both `i` and `j` with the given
function in lock-step and returns a
[`Diff`](enum.Diff.html "enum itertools::Diff") which describes how `j`
differs from `i`.

<a href="fn.enumerate.html" class="fn"
title="fn itertools::enumerate">enumerate</a>  
Iterate `iterable` with a running index.

<a href="fn.equal.html" class="fn" title="fn itertools::equal">equal</a>  
Return `true` if both iterables produce equal sequences (elements
pairwise equal and sequences of the same length), `false` otherwise.

<a href="fn.fold.html" class="fn" title="fn itertools::fold">fold</a>  
Perform a fold operation over the iterable.

<a href="fn.interleave.html" class="fn"
title="fn itertools::interleave">interleave</a>  
Create an iterator that interleaves elements in `i` and `j`.

<a href="fn.intersperse.html" class="fn"
title="fn itertools::intersperse">intersperse</a>  
Iterate `iterable` with a particular value inserted between each
element.

<a href="fn.intersperse_with.html" class="fn"
title="fn itertools::intersperse_with">intersperse_with</a>  
Iterate `iterable` with a particular value created by a function
inserted between each element.

<a href="fn.iterate.html" class="fn"
title="fn itertools::iterate">iterate</a>  
Creates a new iterator that infinitely applies function to value and
yields results.

<a href="fn.join.html" class="fn" title="fn itertools::join">join</a>  
Combine all iterator elements into one `String`, separated by `sep`.

<a href="fn.kmerge.html" class="fn"
title="fn itertools::kmerge">kmerge</a>  
Create an iterator that merges elements of the contained iterators using
the ordering function.

<a href="fn.kmerge_by.html" class="fn"
title="fn itertools::kmerge_by">kmerge_by</a>  
Create an iterator that merges elements of the contained iterators.

<a href="fn.max.html" class="fn" title="fn itertools::max">max</a>  
Return the maximum value of the iterable.

<a href="fn.merge.html" class="fn" title="fn itertools::merge">merge</a>  
Create an iterator that merges elements in `i` and `j`.

<a href="fn.merge_join_by.html" class="fn"
title="fn itertools::merge_join_by">merge_join_by</a>  
Return an iterator adaptor that merge-joins items from the two base
iterators in ascending order.

<a href="fn.min.html" class="fn" title="fn itertools::min">min</a>  
Return the minimum value of the iterable.

<a href="fn.multipeek.html" class="fn"
title="fn itertools::multipeek">multipeek</a>  
An iterator adaptor that allows the user to peek at multiple `.next()`
values without advancing the base iterator.

<a href="fn.multiunzip.html" class="fn"
title="fn itertools::multiunzip">multiunzip</a>  
Converts an iterator of tuples into a tuple of containers.

<a href="fn.multizip.html" class="fn"
title="fn itertools::multizip">multizip</a>  
An iterator that generalizes `.zip()` and allows running multiple
iterators in lockstep.

<a href="fn.partition.html" class="fn"
title="fn itertools::partition">partition</a>  
Partition a sequence using predicate `pred` so that elements that map to
`true` are placed before elements which map to `false`.

<a href="fn.peek_nth.html" class="fn"
title="fn itertools::peek_nth">peek_nth</a>  
A drop-in replacement for
[`std::iter::Peekable`](https://doc.rust-lang.org/1.93.1/core/iter/adapters/peekable/struct.Peekable.html "struct core::iter::adapters::peekable::Peekable")
which adds a `peek_nth` method allowing the user to `peek` at a value
several iterations forward without advancing the base iterator.

<a href="fn.process_results.html" class="fn"
title="fn itertools::process_results">process_results</a>  
“Lift” a function of the values of an iterator so that it can process an
iterator of `Result` values instead.

<a href="fn.put_back.html" class="fn"
title="fn itertools::put_back">put_back</a>  
Create an iterator where you can put back a single item

<a href="fn.put_back_n.html" class="fn"
title="fn itertools::put_back_n">put_back_n</a>  
Create an iterator where you can put back multiple values to the front
of the iteration.

<a href="fn.rciter.html" class="fn"
title="fn itertools::rciter">rciter</a>  
Return an iterator inside a `Rc<RefCell<_>>` wrapper.

<a href="fn.repeat_n.html" class="fn"
title="fn itertools::repeat_n">repeat_n</a>  
Create an iterator that produces `n` repetitions of `element`.

<a href="fn.rev.html" class="fn" title="fn itertools::rev">rev</a>  
Iterate `iterable` in reverse.

<a href="fn.sorted.html" class="fn"
title="fn itertools::sorted">sorted</a>  
Sort all iterator elements into a new iterator in ascending order.

<a href="fn.sorted_unstable.html" class="fn"
title="fn itertools::sorted_unstable">sorted_unstable</a>  
Sort all iterator elements into a new iterator in ascending order. This
sort is unstable (i.e., may reorder equal elements).

<a href="fn.unfold.html" class="fn"
title="fn itertools::unfold">unfold</a><span class="stab deprecated" title="">Deprecated</span>  
Creates a new unfold source with the specified closure as the “iterator
function” and an initial state to eventually pass to the closure

<a href="fn.zip.html" class="fn" title="fn itertools::zip">zip</a><span class="stab deprecated" title="">Deprecated</span>  
Converts the arguments to iterators and zips them.

<a href="fn.zip_eq.html" class="fn"
title="fn itertools::zip_eq">zip_eq</a>  
Zips two iterators but **panics** if they are not of the same length.
