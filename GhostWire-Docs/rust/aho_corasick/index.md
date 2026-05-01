Crate aho\_corasick Copy item path

<span class="sub-heading"><a href="../src/aho_corasick/lib.rs.html#1-326" class="src">Source</a>
</span>

Expand description

A library for finding occurrences of many patterns at once. This library
provides multiple pattern search principally through an implementation
of the [Aho-Corasick
algorithm](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm),
which builds a fast finite state machine for executing searches in
linear time.

Additionally, this library provides a number of configuration options
for building the automaton that permit controlling the space versus time
trade off. Other features include simple ASCII case insensitive
matching, finding overlapping matches, replacements, searching streams
and even searching and replacing text in streams.

Finally, unlike most other Aho-Corasick implementations, this one
supports enabling
[leftmost-first](enum.MatchKind.html#variant.LeftmostFirst "variant aho_corasick::MatchKind::LeftmostFirst")
or
[leftmost-longest](enum.MatchKind.html#variant.LeftmostLongest "variant aho_corasick::MatchKind::LeftmostLongest")
match semantics, using a (seemingly) novel alternative construction
algorithm. For more details on what match semantics means, see the
[`MatchKind`](enum.MatchKind.html "enum aho_corasick::MatchKind") type.

# <a href="#overview" class="doc-anchor">§</a>Overview

This section gives a brief overview of the primary types in this crate:

-   [`AhoCorasick`](struct.AhoCorasick.html "struct aho_corasick::AhoCorasick")
    is the primary type and represents an Aho-Corasick automaton. This
    is the type you use to execute searches.
-   [`AhoCorasickBuilder`](struct.AhoCorasickBuilder.html "struct aho_corasick::AhoCorasickBuilder")
    can be used to build an Aho-Corasick automaton, and supports
    configuring a number of options.
-   [`Match`](struct.Match.html "struct aho_corasick::Match") represents
    a single match reported by an Aho-Corasick automaton. Each match has
    two pieces of information: the pattern that matched and the start
    and end byte offsets corresponding to the position in the haystack
    at which it matched.

# <a href="#example-basic-searching" class="doc-anchor">§</a>Example: basic searching

This example shows how to search for occurrences of multiple patterns
simultaneously. Each match includes the pattern that matched along with
the byte offsets of the match.

``` {.rust .rust-example-rendered}
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "Snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::new(patterns).unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

# <a href="#example-case-insensitivity" class="doc-anchor">§</a>Example: case insensitivity

This is like the previous example, but matches `Snapple` case
insensitively using `AhoCorasickBuilder`:

``` {.rust .rust-example-rendered}
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::builder()
    .ascii_case_insensitive(true)
    .build(patterns)
    .unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

# <a href="#example-replacing-matches-in-a-stream"
class="doc-anchor">§</a>Example: replacing matches in a stream

This example shows how to execute a search and replace on a stream
without loading the entire stream into memory first.

``` {.rust .rust-example-rendered}
use aho_corasick::AhoCorasick;

let patterns = &["fox", "brown", "quick"];
let replace_with = &["sloth", "grey", "slow"];

// In a real example, these might be `std::fs::File`s instead. All you need to
// do is supply a pair of `std::io::Read` and `std::io::Write` implementations.
let rdr = "The quick brown fox.";
let mut wtr = vec![];

let ac = AhoCorasick::new(patterns).unwrap();
ac.try_stream_replace_all(rdr.as_bytes(), &mut wtr, replace_with)?;
assert_eq!(b"The slow grey sloth.".to_vec(), wtr);
```

# <a href="#example-finding-the-leftmost-first-match"
class="doc-anchor">§</a>Example: finding the leftmost first match

In the textbook description of Aho-Corasick, its formulation is
typically structured such that it reports all possible matches, even
when they overlap with another. In many cases, overlapping matches may
not be desired, such as the case of finding all successive
non-overlapping matches like you might with a standard regular
expression.

Unfortunately the “obvious” way to modify the Aho-Corasick algorithm to
do this doesn’t always work in the expected way, since it will report
matches as soon as they are seen. For example, consider matching the
regex `Samwise|Sam` against the text `Samwise`. Most regex engines (that
are Perl-like, or non-POSIX) will report `Samwise` as a match, but the
standard Aho-Corasick algorithm modified for reporting non-overlapping
matches will report `Sam`.

A novel contribution of this library is the ability to change the match
semantics of Aho-Corasick (without additional search time overhead) such
that `Samwise` is reported instead. For example, here’s the standard
approach:

``` {.rust .rust-example-rendered}
use aho_corasick::AhoCorasick;

let patterns = &["Samwise", "Sam"];
let haystack = "Samwise";

let ac = AhoCorasick::new(patterns).unwrap();
let mat = ac.find(haystack).expect("should have a match");
assert_eq!("Sam", &haystack[mat.start()..mat.end()]);
```

And now here’s the leftmost-first version, which matches how a Perl-like
regex will work:

``` {.rust .rust-example-rendered}
use aho_corasick::{AhoCorasick, MatchKind};

let patterns = &["Samwise", "Sam"];
let haystack = "Samwise";

let ac = AhoCorasick::builder()
    .match_kind(MatchKind::LeftmostFirst)
    .build(patterns)
    .unwrap();
let mat = ac.find(haystack).expect("should have a match");
assert_eq!("Samwise", &haystack[mat.start()..mat.end()]);
```

In addition to leftmost-first semantics, this library also supports
leftmost-longest semantics, which match the POSIX behavior of a regular
expression alternation. See
[`MatchKind`](enum.MatchKind.html "enum aho_corasick::MatchKind") for
more details.

# <a href="#prefilters" class="doc-anchor">§</a>Prefilters

While an Aho-Corasick automaton can perform admirably when compared to
more naive solutions, it is generally slower than more specialized
algorithms that are accelerated using vector instructions such as SIMD.

For that reason, this library will internally use a “prefilter” to
attempt to accelerate searches when possible. Currently, this library
has several different algorithms it might use depending on the patterns
provided. Once the number of patterns gets too big, prefilters are no
longer used.

While a prefilter is generally good to have on by default since it works
well in the common case, it can lead to less predictable or even
sub-optimal performance in some cases. For that reason, prefilters can
be explicitly disabled via
[`AhoCorasickBuilder::prefilter`](struct.AhoCorasickBuilder.html#method.prefilter "method aho_corasick::AhoCorasickBuilder::prefilter").

# <a href="#lower-level-apis" class="doc-anchor">§</a>Lower level APIs

This crate also provides several sub-modules that collectively expose
many of the implementation details of the main
[`AhoCorasick`](struct.AhoCorasick.html "struct aho_corasick::AhoCorasick")
type. Most users of this library can completely ignore the submodules
and their contents, but if you needed finer grained control, some parts
of them may be useful to you. Here is a brief overview of each and why
you might want to use them:

-   The [`packed`](packed/index.html "mod aho_corasick::packed")
    sub-module contains a lower level API for using fast vectorized
    routines for finding a small number of patterns in a haystack. You
    might want to use this API when you want to completely side-step
    using Aho-Corasick automata. Otherwise, the fast vectorized routines
    are used automatically as prefilters for `AhoCorasick` searches
    whenever possible.
-   The
    [`automaton`](automaton/index.html "mod aho_corasick::automaton")
    sub-module provides a lower level finite state machine interface
    that the various Aho-Corasick implementations in this crate
    implement. This sub-module’s main contribution is the
    [`Automaton`](automaton/trait.Automaton.html "trait aho_corasick::automaton::Automaton")
    trait, which permits manually walking the state transitions of an
    Aho-Corasick automaton.
-   The [`dfa`](dfa/index.html "mod aho_corasick::dfa") and
    [`nfa`](nfa/index.html "mod aho_corasick::nfa") sub-modules provide
    DFA and NFA implementations of the aforementioned `Automaton` trait.
    The main reason one might want to use these sub-modules is to get
    access to a type that implements the `Automaton` trait. (The
    top-level `AhoCorasick` type does not implement the `Automaton`
    trait.)

As mentioned above, if you aren’t sure whether you need these
sub-modules, you should be able to safely ignore them and just focus on
the
[`AhoCorasick`](struct.AhoCorasick.html "struct aho_corasick::AhoCorasick")
type.

# <a href="#crate-features" class="doc-anchor">§</a>Crate features

This crate exposes a few features for controlling dependency usage and
whether this crate can be used without the standard library.

-   **std** - Enables support for the standard library. This feature is
    enabled by default. When disabled, only `core` and `alloc` are used.
    At an API level, enabling `std` enables `std::error::Error` trait
    impls for the various error types, and higher level stream search
    routines such as
    [`AhoCorasick::try_stream_find_iter`](struct.AhoCorasick.html#method.try_stream_find_iter "method aho_corasick::AhoCorasick::try_stream_find_iter").
    But the `std` feature is also required to enable vectorized
    prefilters. Prefilters can greatly accelerate searches, but
    generally only apply when the number of patterns is small (less than
    ~100).
-   **perf-literal** - Enables support for literal prefilters that use
    vectorized routines from external crates. This feature is enabled by
    default. If you’re only using Aho-Corasick for large numbers of
    patterns or otherwise can abide lower throughput when searching with
    a small number of patterns, then it is reasonable to disable this
    feature.
-   **logging** - Enables a dependency on the `log` crate and emits
    messages to aide in diagnostics. This feature is disabled by
    default.

# Modules<a href="#modules" class="anchor">§</a>

<a href="automaton/index.html" class="mod"
title="mod aho_corasick::automaton">automaton</a>  
Provides
[`Automaton`](automaton/trait.Automaton.html "trait aho_corasick::automaton::Automaton")
trait for abstracting over Aho-Corasick automata.

<a href="dfa/index.html" class="mod"
title="mod aho_corasick::dfa">dfa</a>  
Provides direct access to a DFA implementation of Aho-Corasick.

<a href="nfa/index.html" class="mod"
title="mod aho_corasick::nfa">nfa</a>  
Provides direct access to NFA implementations of Aho-Corasick.

<a href="packed/index.html" class="mod"
title="mod aho_corasick::packed">packed</a>  
Provides packed multiple substring search, principally for a small
number of patterns.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AhoCorasick.html" class="struct"
title="struct aho_corasick::AhoCorasick">AhoCorasick</a>  
An automaton for searching multiple strings in linear time.

<a href="struct.AhoCorasickBuilder.html" class="struct"
title="struct aho_corasick::AhoCorasickBuilder">AhoCorasickBuilder</a>  
A builder for configuring an Aho-Corasick automaton.

<a href="struct.BuildError.html" class="struct"
title="struct aho_corasick::BuildError">BuildError</a>  
An error that occurred during the construction of an Aho-Corasick
automaton.

<a href="struct.FindIter.html" class="struct"
title="struct aho_corasick::FindIter">FindIter</a>  
An iterator of non-overlapping matches in a particular haystack.

<a href="struct.FindOverlappingIter.html" class="struct"
title="struct aho_corasick::FindOverlappingIter">FindOverlappingIter</a>  
An iterator of overlapping matches in a particular haystack.

<a href="struct.Input.html" class="struct"
title="struct aho_corasick::Input">Input</a>  
The configuration and the haystack to use for an Aho-Corasick search.

<a href="struct.Match.html" class="struct"
title="struct aho_corasick::Match">Match</a>  
A representation of a match reported by an Aho-Corasick searcher.

<a href="struct.MatchError.html" class="struct"
title="struct aho_corasick::MatchError">MatchError</a>  
An error that occurred during an Aho-Corasick search.

<a href="struct.PatternID.html" class="struct"
title="struct aho_corasick::PatternID">PatternID</a>  
The identifier of a pattern in an Aho-Corasick automaton.

<a href="struct.PatternIDError.html" class="struct"
title="struct aho_corasick::PatternIDError">PatternIDError</a>  
This error occurs when an ID could not be constructed.

<a href="struct.Span.html" class="struct"
title="struct aho_corasick::Span">Span</a>  
A representation of a range in a haystack.

<a href="struct.StreamFindIter.html" class="struct"
title="struct aho_corasick::StreamFindIter">StreamFindIter</a>  
An iterator that reports Aho-Corasick matches in a stream.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AhoCorasickKind.html" class="enum"
title="enum aho_corasick::AhoCorasickKind">AhoCorasickKind</a>  
The type of Aho-Corasick implementation to use in an
[`AhoCorasick`](struct.AhoCorasick.html "struct aho_corasick::AhoCorasick")
searcher.

<a href="enum.Anchored.html" class="enum"
title="enum aho_corasick::Anchored">Anchored</a>  
The type of anchored search to perform.

<a href="enum.MatchErrorKind.html" class="enum"
title="enum aho_corasick::MatchErrorKind">MatchErrorKind</a>  
The underlying kind of a
[`MatchError`](struct.MatchError.html "struct aho_corasick::MatchError").

<a href="enum.MatchKind.html" class="enum"
title="enum aho_corasick::MatchKind">MatchKind</a>  
A knob for controlling the match semantics of an Aho-Corasick automaton.

<a href="enum.StartKind.html" class="enum"
title="enum aho_corasick::StartKind">StartKind</a>  
The kind of anchored starting configurations to support in an
Aho-Corasick searcher.
