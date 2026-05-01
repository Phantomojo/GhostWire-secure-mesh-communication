Crate regex\_automata Copy item path

<span class="sub-heading"><a href="../src/regex_automata/lib.rs.html#1-651" class="src">Source</a>
</span>

Expand description

This crate exposes a variety of regex engines used by the `regex` crate.
It provides a vast, sprawling and “expert” level API to each regex
engine. The regex engines provided by this crate focus heavily on finite
automata implementations and specifically guarantee worst case
`O(m * n)` time complexity for all searches. (Where `m ~ len(regex)` and
`n ~ len(haystack)`.)

The primary goal of this crate is to serve as an implementation detail
for the `regex` crate. A secondary goal is to make its internals
available for use by others.

# <a href="#table-of-contents" class="doc-anchor">§</a>Table of contents

-   [Should I be using this crate?](#should-i-be-using-this-crate) gives
    some reasons for and against using this crate.
-   [Examples](#examples) provides a small selection of things you can
    do with this crate.
-   [Available regex engines](#available-regex-engines) provides a
    hyperlinked list of all regex engines in this crate.
-   [API themes](#api-themes) discusses common elements used throughout
    this crate.
-   [Crate features](#crate-features) documents the extensive list of
    Cargo features available.

# <a href="#should-i-be-using-this-crate" class="doc-anchor">§</a>Should I be using this crate?

If you find yourself here because you just want to use regexes, then you
should first check out whether the [`regex`
crate](https://docs.rs/regex) meets your needs. It provides a
streamlined and difficult-to-misuse API for regex searching.

If you’re here because there is something specific you want to do that
can’t be easily done with `regex` crate, then you are perhaps in the
right place. It’s most likely that the first stop you’ll want to make is
to explore the [`meta` regex
APIs](meta/index.html "mod regex_automata::meta"). Namely, the `regex`
crate is just a light wrapper over a
[`meta::Regex`](meta/struct.Regex.html "struct regex_automata::meta::Regex"),
so its API will probably be the easiest to transition to. In contrast to
the `regex` crate, the `meta::Regex` API supports more search parameters
and does multi-pattern searches. However, it isn’t quite as ergonomic.

Otherwise, the following is an inexhaustive list of reasons to use this
crate:

-   You want to analyze or use a [Thompson
    `NFA`](nfa/thompson/struct.NFA.html "struct regex_automata::nfa::thompson::NFA")
    directly.
-   You want more powerful multi-pattern search than what is provided by
    `RegexSet` in the `regex` crate. All regex engines in this crate
    support multi-pattern searches.
-   You want to use one of the `regex` crate’s internal engines directly
    because of some interesting configuration that isn’t possible via
    the `regex` crate. For example, a [lazy DFA’s
    configuration](hybrid/dfa/struct.Config.html "struct regex_automata::hybrid::dfa::Config")
    exposes a dizzying number of options for controlling its execution.
-   You want to use the lower level search APIs. For example, both the
    [lazy DFA](hybrid/dfa/index.html "mod regex_automata::hybrid::dfa")
    and [fully compiled DFAs](dfa/index.html "mod regex_automata::dfa")
    support searching by exploring the automaton one state at a time.
    This might be useful, for example, for stream searches or searches
    of strings stored in non-contiguous in memory.
-   You want to build a fully compiled DFA and then [use zero-copy
    deserialization](dfa/dense/struct.DFA.html#method.from_bytes "associated function regex_automata::dfa::dense::DFA::from_bytes")
    to load it into memory and use it for searching. This use case is
    supported in core-only no-std/no-alloc environments.
-   You want to run [anchored
    searches](struct.Input.html#method.anchored "method regex_automata::Input::anchored")
    without using the `^` anchor in your regex pattern.
-   You need to work-around contention issues with sharing a regex
    across multiple threads. The
    [`meta::Regex::search_with`](meta/struct.Regex.html#method.search_with "method regex_automata::meta::Regex::search_with")
    API permits bypassing any kind of synchronization at all by
    requiring the caller to provide the mutable scratch spaced needed
    during a search.
-   You want to build your own regex engine on top of the `regex`
    crate’s infrastructure.

# <a href="#examples" class="doc-anchor">§</a>Examples

This section tries to identify a few interesting things you can do with
this crate and demonstrates them.

### <a href="#multi-pattern-searches-with-capture-groups"
class="doc-anchor">§</a>Multi-pattern searches with capture groups

One of the more frustrating limitations of `RegexSet` in the `regex`
crate (at the time of writing) is that it doesn’t report match
positions. With this crate, multi-pattern support was intentionally
designed in from the beginning, which means it works in all regex
engines and even for capture groups as well.

This example shows how to search for matches of multiple regexes, where
each regex uses the same capture group names to parse different
key-value formats.

``` {.rust .rust-example-rendered}
use regex_automata::{meta::Regex, PatternID};

let re = Regex::new_many(&[
    r#"(?m)^(?<key>[[:word:]]+)=(?<val>[[:word:]]+)$"#,
    r#"(?m)^(?<key>[[:word:]]+)="(?<val>[^"]+)"$"#,
    r#"(?m)^(?<key>[[:word:]]+)='(?<val>[^']+)'$"#,
    r#"(?m)^(?<key>[[:word:]]+):\s*(?<val>[[:word:]]+)$"#,
])?;
let hay = r#"
best_album="Blow Your Face Out"
best_quote='"then as it was, then again it will be"'
best_year=1973
best_simpsons_episode: HOMR
"#;
let mut kvs = vec![];
for caps in re.captures_iter(hay) {
    // N.B. One could use capture indices '1' and '2' here
    // as well. Capture indices are local to each pattern.
    // (Just like names are.)
    let key = &hay[caps.get_group_by_name("key").unwrap()];
    let val = &hay[caps.get_group_by_name("val").unwrap()];
    kvs.push((key, val));
}
assert_eq!(kvs, vec![
    ("best_album", "Blow Your Face Out"),
    ("best_quote", "\"then as it was, then again it will be\""),
    ("best_year", "1973"),
    ("best_simpsons_episode", "HOMR"),
]);
```

### <a href="#build-a-full-dfa-and-walk-it-manually"
class="doc-anchor">§</a>Build a full DFA and walk it manually

One of the regex engines in this crate is a fully compiled DFA. It takes
worst case exponential time to build, but once built, it can be easily
explored and used for searches. Here’s a simple example that uses its
lower level APIs to implement a simple anchored search by hand.

``` {.rust .rust-example-rendered}
use regex_automata::{dfa::{Automaton, dense}, Input};

let dfa = dense::DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let haystack = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&Input::new(haystack))?;
// Walk all the bytes in the haystack.
for &b in haystack.as_bytes().iter() {
    state = dfa.next_state(state, b);
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(state);
assert!(dfa.is_match_state(state));
```

Or do the same with a lazy DFA that avoids exponential worst case
compile time, but requires mutable scratch space to lazily build the DFA
during the search.

``` {.rust .rust-example-rendered}
use regex_automata::{hybrid::dfa::DFA, Input};

let dfa = DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let mut cache = dfa.create_cache();
let hay = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&mut cache, &Input::new(hay))?;
// Walk all the bytes in the haystack.
for &b in hay.as_bytes().iter() {
    state = dfa.next_state(&mut cache, state, b)?;
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(&mut cache, state)?;
assert!(state.is_match());
```

### <a href="#find-all-overlapping-matches" class="doc-anchor">§</a>Find all overlapping matches

This example shows how to build a DFA and use it to find all possible
matches, including overlapping matches. A similar example will work with
a lazy DFA as well. This also works with multiple patterns and will
report all matches at the same position where multiple patterns match.

``` {.rust .rust-example-rendered}
use regex_automata::{
    dfa::{dense, Automaton, OverlappingState},
    Input, MatchKind,
};

let dfa = dense::DFA::builder()
    .configure(dense::DFA::config().match_kind(MatchKind::All))
    .build(r"(?-u)\w{3,}")?;
let input = Input::new("homer marge bart lisa maggie");
let mut state = OverlappingState::start();

let mut matches = vec![];
while let Some(hm) = {
    dfa.try_search_overlapping_fwd(&input, &mut state)?;
    state.get_match()
} {
    matches.push(hm.offset());
}
assert_eq!(matches, vec![
    3, 4, 5,        // hom, home, homer
    9, 10, 11,      // mar, marg, marge
    15, 16,         // bar, bart
    20, 21,         // lis, lisa
    25, 26, 27, 28, // mag, magg, maggi, maggie
]);
```

# <a href="#available-regex-engines" class="doc-anchor">§</a>Available regex engines

The following is a complete list of all regex engines provided by this
crate, along with a very brief description of it and why you might want
to use it.

-   [`dfa::regex::Regex`](dfa/regex/struct.Regex.html "struct regex_automata::dfa::regex::Regex")
    is a regex engine that works on top of either
    [dense](dfa/dense/index.html "mod regex_automata::dfa::dense") or
    [sparse](dfa/sparse/index.html "mod regex_automata::dfa::sparse")
    fully compiled DFAs. You might use a DFA if you need the fastest
    possible regex engine in this crate and can afford the exorbitant
    memory usage usually required by DFAs. Low level APIs on fully
    compiled DFAs are provided by the [`Automaton`
    trait](dfa/trait.Automaton.html "trait regex_automata::dfa::Automaton").
    Fully compiled dense DFAs can handle all regexes except for
    searching a regex with a Unicode word boundary on non-ASCII
    haystacks. A fully compiled DFA based regex can only report the
    start and end of each match.
-   [`hybrid::regex::Regex`](hybrid/regex/struct.Regex.html "struct regex_automata::hybrid::regex::Regex")
    is a regex engine that works on top of a lazily built DFA. Its
    performance profile is very similar to that of fully compiled DFAs,
    but can be slower in some pathological cases. Fully compiled DFAs
    are also amenable to more optimizations, such as state acceleration,
    that aren’t available in a lazy DFA. You might use this lazy DFA if
    you can’t abide the worst case exponential compile time of a full
    DFA, but still want the DFA search performance in the vast majority
    of cases. A lazy DFA based regex can only report the start and end
    of each match.
-   [`dfa::onepass::DFA`](dfa/onepass/struct.DFA.html "struct regex_automata::dfa::onepass::DFA")
    is a regex engine that is implemented as a DFA, but can report the
    matches of each capture group in addition to the start and end of
    each match. The catch is that it only works on a somewhat small
    subset of regexes known as “one-pass.” You’ll want to use this for
    cases when you need capture group matches and the regex is one-pass
    since it is likely to be faster than any alternative. A one-pass DFA
    can handle all types of regexes, but does have some reasonable
    limits on the number of capture groups it can handle.
-   [`nfa::thompson::backtrack::BoundedBacktracker`](nfa/thompson/backtrack/struct.BoundedBacktracker.html "struct regex_automata::nfa::thompson::backtrack::BoundedBacktracker")
    is a regex engine that uses backtracking, but keeps track of the
    work it has done to avoid catastrophic backtracking. Like the
    one-pass DFA, it provides the matches of each capture group. It
    retains the `O(m * n)` worst case time bound. This tends to be
    slower than the one-pass DFA regex engine, but faster than the
    PikeVM. It can handle all types of regexes, but usually only works
    well with small haystacks and small regexes due to the memory
    required to avoid redoing work.
-   [`nfa::thompson::pikevm::PikeVM`](nfa/thompson/pikevm/struct.PikeVM.html "struct regex_automata::nfa::thompson::pikevm::PikeVM")
    is a regex engine that can handle all regexes, of all sizes and
    provides capture group matches. It tends to be a tool of last resort
    because it is also usually the slowest regex engine.
-   [`meta::Regex`](meta/struct.Regex.html "struct regex_automata::meta::Regex")
    is the meta regex engine that combines *all* of the above engines
    into one. The reason for this is that each of the engines above have
    their own caveats such as, “only handles a subset of regexes” or “is
    generally slow.” The meta regex engine accounts for all of these
    caveats and composes the engines in a way that attempts to mitigate
    each engine’s weaknesses while emphasizing its strengths. For
    example, it will attempt to run a lazy DFA even if it might fail. In
    which case, it will restart the search with a likely slower but more
    capable regex engine. The meta regex engine is what you should
    default to. Use one of the above engines directly only if you have a
    specific reason to.

# <a href="#api-themes" class="doc-anchor">§</a>API themes

While each regex engine has its own APIs and configuration options,
there are some general themes followed by all of them.

### <a href="#the-input-abstraction" class="doc-anchor">§</a>The `Input` abstraction

Most search routines in this crate accept anything that implements
`Into<Input>`. Both `&str` and `&[u8]` haystacks satisfy this
constraint, which means that things like `engine.search("foo")` will
work as you would expect.

By virtue of accepting an `Into<Input>` though, callers can provide more
than just a haystack. Indeed, the
[`Input`](struct.Input.html "struct regex_automata::Input") type has
more details, but briefly, callers can use it to configure various
aspects of the search:

-   The span of the haystack to search via
    [`Input::span`](struct.Input.html#method.span "method regex_automata::Input::span")
    or
    [`Input::range`](struct.Input.html#method.range "method regex_automata::Input::range"),
    which might be a substring of the haystack.
-   Whether to run an anchored search or not via
    [`Input::anchored`](struct.Input.html#method.anchored "method regex_automata::Input::anchored").
    This permits one to require matches to start at the same offset that
    the search started.
-   Whether to ask the regex engine to stop as soon as a match is seen
    via
    [`Input::earliest`](struct.Input.html#method.earliest "method regex_automata::Input::earliest").
    This can be used to find the offset of a match as soon as it is
    known without waiting for the full leftmost-first match to be found.
    This can also be used to avoid the worst case `O(m * n^2)` time
    complexity of iteration.

Some lower level search routines accept an `&Input` for performance
reasons. In which case, `&Input::new("haystack")` can be used for a
simple search.

### <a href="#error-reporting" class="doc-anchor">§</a>Error reporting

Most, but not all, regex engines in this crate can fail to execute a
search. When a search fails, callers cannot determine whether or not a
match exists. That is, the result is indeterminate.

Search failure, in all cases in this crate, is represented by a
[`MatchError`](struct.MatchError.html "struct regex_automata::MatchError").
Routines that can fail start with the `try_` prefix in their name. For
example,
[`hybrid::regex::Regex::try_search`](hybrid/regex/struct.Regex.html#method.try_search "method regex_automata::hybrid::regex::Regex::try_search")
can fail for a number of reasons. Conversely, routines that either can’t
fail or can panic on failure lack the `try_` prefix. For example,
[`hybrid::regex::Regex::find`](hybrid/regex/struct.Regex.html#method.find "method regex_automata::hybrid::regex::Regex::find")
will panic in cases where
[`hybrid::regex::Regex::try_search`](hybrid/regex/struct.Regex.html#method.try_search "method regex_automata::hybrid::regex::Regex::try_search")
would return an error, and
[`meta::Regex::find`](meta/struct.Regex.html#method.find "method regex_automata::meta::Regex::find")
will never panic. Therefore, callers need to pay close attention to the
panicking conditions in the documentation.

In most cases, the reasons that a search fails are either predictable or
configurable, albeit at some additional cost.

An example of predictable failure is
[`BoundedBacktracker::try_search`](nfa/thompson/backtrack/struct.BoundedBacktracker.html#method.try_search "method regex_automata::nfa::thompson::backtrack::BoundedBacktracker::try_search").
Namely, it fails whenever the multiplication of the haystack, the regex
and some constant exceeds the [configured visited
capacity](nfa/thompson/backtrack/struct.Config.html#method.visited_capacity "method regex_automata::nfa::thompson::backtrack::Config::visited_capacity").
Callers can predict the failure in terms of haystack length via the
[`BoundedBacktracker::max_haystack_len`](nfa/thompson/backtrack/struct.BoundedBacktracker.html#method.max_haystack_len "method regex_automata::nfa::thompson::backtrack::BoundedBacktracker::max_haystack_len")
method. While this form of failure is technically avoidable by
increasing the visited capacity, it isn’t practical to do so for all
inputs because the memory usage required for larger haystacks becomes
impractically large. So in practice, if one is using the bounded
backtracker, you really do have to deal with the failure.

An example of configurable failure happens when one enables heuristic
support for Unicode word boundaries in a DFA. Namely, since the DFAs in
this crate (except for the one-pass DFA) do not support Unicode word
boundaries on non-ASCII haystacks, building a DFA from an NFA that
contains a Unicode word boundary will itself fail. However, one can
configure DFAs to still be built in this case by [configuring heuristic
support for Unicode word
boundaries](hybrid/dfa/struct.Config.html#method.unicode_word_boundary "method regex_automata::hybrid::dfa::Config::unicode_word_boundary").
If the NFA the DFA is built from contains a Unicode word boundary, then
the DFA will still be built, but special transitions will be added to
every state that cause the DFA to fail if any non-ASCII byte is seen.
This failure happens at search time and it requires the caller to opt
into this.

There are other ways for regex engines to fail in this crate, but the
above two should represent the general theme of failures one can find.
Dealing with these failures is, in part, one the responsibilities of the
[meta regex engine](meta/index.html "mod regex_automata::meta"). Notice,
for example, that the meta regex engine exposes an API that never
returns an error nor panics. It carefully manages all of the ways in
which the regex engines can fail and either avoids the predictable ones
entirely (e.g., the bounded backtracker) or reacts to configured
failures by falling back to a different engine (e.g., the lazy DFA
quitting because it saw a non-ASCII byte).

### <a href="#configuration-and-builders" class="doc-anchor">§</a>Configuration and Builders

Most of the regex engines in this crate come with two types to
facilitate building the regex engine: a `Config` and a `Builder`. A
`Config` is usually specific to that particular regex engine, but other
objects such as parsing and NFA compilation have `Config` types too. A
`Builder` is the thing responsible for taking inputs (either pattern
strings or already-parsed patterns or even NFAs directly) and turning
them into an actual regex engine that can be used for searching.

The main reason why building a regex engine is a bit complicated is
because of the desire to permit composition with de-coupled components.
For example, you might want to [manually construct a Thompson
NFA](nfa/thompson/struct.Builder.html "struct regex_automata::nfa::thompson::Builder")
and then build a regex engine from it without ever using a regex parser
at all. On the other hand, you might also want to build a regex engine
directly from the concrete syntax. This demonstrates why regex engine
construction is so flexible: it needs to support not just convenient
construction, but also construction from parts built elsewhere.

This is also in turn why there are many different `Config` structs in
this crate. Let’s look more closely at an example:
[`hybrid::regex::Builder`](hybrid/regex/struct.Builder.html "struct regex_automata::hybrid::regex::Builder").
It accepts three different `Config` types for configuring construction
of a lazy DFA regex:

-   [`hybrid::regex::Builder::syntax`](hybrid/regex/struct.Builder.html#method.syntax "method regex_automata::hybrid::regex::Builder::syntax")
    accepts a
    [`util::syntax::Config`](util/syntax/struct.Config.html "struct regex_automata::util::syntax::Config")
    for configuring the options found in the
    [`regex-syntax`](../regex_syntax/index.html "mod regex_syntax")
    crate. For example, whether to match case insensitively.
-   [`hybrid::regex::Builder::thompson`](hybrid/regex/struct.Builder.html#method.thompson "method regex_automata::hybrid::regex::Builder::thompson")
    accepts a
    [`nfa::thompson::Config`](nfa/thompson/struct.Config.html "struct regex_automata::nfa::thompson::Config")
    for configuring construction of a [Thompson
    NFA](nfa/thompson/struct.NFA.html "struct regex_automata::nfa::thompson::NFA").
    For example, whether to build an NFA that matches the reverse
    language described by the regex.
-   [`hybrid::regex::Builder::dfa`](hybrid/regex/struct.Builder.html#method.dfa "method regex_automata::hybrid::regex::Builder::dfa")
    accept a
    [`hybrid::dfa::Config`](hybrid/dfa/struct.Config.html "struct regex_automata::hybrid::dfa::Config")
    for configuring construction of the pair of underlying lazy DFAs
    that make up the lazy DFA regex engine. For example, changing the
    capacity of the cache used to store the transition table.

The lazy DFA regex engine uses all three of those configuration objects
for methods like
[`hybrid::regex::Builder::build`](hybrid/regex/struct.Builder.html#method.build "method regex_automata::hybrid::regex::Builder::build"),
which accepts a pattern string containing the concrete syntax of your
regex. It uses the syntax configuration to parse it into an AST and
translate it into an HIR. Then the NFA configuration when compiling the
HIR into an NFA. And then finally the DFA configuration when lazily
determinizing the NFA into a DFA.

Notice though that the builder also has a
[`hybrid::regex::Builder::build_from_dfas`](hybrid/regex/struct.Builder.html#method.build_from_dfas "method regex_automata::hybrid::regex::Builder::build_from_dfas")
constructor. This permits callers to build the underlying pair of lazy
DFAs themselves (one for the forward searching to find the end of a
match and one for the reverse searching to find the start of a match),
and then build the regex engine from them. The lazy DFAs, in turn, have
their own builder that permits [construction directly from a Thompson
NFA](hybrid/dfa/struct.Builder.html#method.build_from_nfa "method regex_automata::hybrid::dfa::Builder::build_from_nfa").
Continuing down the rabbit hole, a Thompson NFA has its own compiler
that permits [construction directly from an
HIR](nfa/thompson/struct.Compiler.html#method.build_from_hir "method regex_automata::nfa::thompson::Compiler::build_from_hir").
The lazy DFA regex engine builder lets you follow this rabbit hole all
the way down, but also provides convenience routines that do it for you
when you don’t need precise control over every component.

The [meta regex engine](meta/index.html "mod regex_automata::meta") is a
good example of something that utilizes the full flexibility of these
builders. It often needs not only precise control over each component,
but also shares them across multiple regex engines. (Most sharing is
done by internal reference accounting. For example, an
[`NFA`](nfa/thompson/struct.NFA.html "struct regex_automata::nfa::thompson::NFA")
is reference counted internally which makes cloning cheap.)

### <a href="#size-limits" class="doc-anchor">§</a>Size limits

Unlike the `regex` crate, the `regex-automata` crate specifically does
not enable any size limits by default. That means users of this crate
need to be quite careful when using untrusted patterns. Namely, because
bounded repetitions can grow exponentially by stacking them, it is
possible to build a very large internal regex object from just a small
pattern string. For example, the NFA built from the pattern
`a{10}{10}{10}{10}{10}{10}{10}` is over 240MB.

There are multiple size limit options in this crate. If one or more size
limits are relevant for the object you’re building, they will be
configurable via methods on a corresponding `Config` type.

# <a href="#crate-features" class="doc-anchor">§</a>Crate features

This crate has a dizzying number of features. The main idea is to be
able to control how much stuff you pull in for your specific use case,
since the full crate is quite large and can dramatically increase
compile times and binary size.

The most barebones but useful configuration is to disable all default
features and enable only `dfa-search`. This will bring in just the DFA
deserialization and search routines without any dependency on `std` or
`alloc`. This does require generating and serializing a DFA, and then
storing it somewhere, but it permits regex searches in freestanding or
embedded environments.

Because there are so many features, they are split into a few groups.

The default set of features is: `std`, `syntax`, `perf`, `unicode`,
`meta`, `nfa`, `dfa` and `hybrid`. Basically, the default is to enable
everything except for development related features like `logging`.

### <a href="#ecosystem-features" class="doc-anchor">§</a>Ecosystem features

-   **std** - Enables use of the standard library. In terms of APIs,
    this usually just means that error types implement the
    `std::error::Error` trait. Otherwise, `std` sometimes enables the
    code to be faster, for example, using a `HashMap` instead of a
    `BTreeMap`. (The `std` feature matters more for dependencies like
    `aho-corasick` and `memchr`, where `std` is required to enable
    certain classes of SIMD optimizations.) Enabling `std` automatically
    enables `alloc`.
-   **alloc** - Enables use of the `alloc` library. This is required for
    most APIs in this crate. The main exception is deserializing and
    searching with fully compiled DFAs.
-   **logging** - Adds a dependency on the `log` crate and makes this
    crate emit log messages of varying degrees of utility. The log
    messages are especially useful in trying to understand what the meta
    regex engine is doing.

### <a href="#performance-features" class="doc-anchor">§</a>Performance features

**Note**: To get performance benefits offered by the SIMD, `std` must be
enabled. None of the `perf-*` features will enable `std` implicitly.

-   **perf** - Enables all of the below features.
-   **perf-inline** - When enabled, `inline(always)` is used in (many)
    strategic locations to help performance at the expense of longer
    compile times and increased binary size.
-   **perf-literal** - Enables all literal related optimizations.
    -   **perf-literal-substring** - Enables all single substring
        literal optimizations. This includes adding a dependency on the
        `memchr` crate.
    -   **perf-literal-multisubstring** - Enables all multiple substring
        literal optimizations. This includes adding a dependency on the
        `aho-corasick` crate.

### <a href="#unicode-features" class="doc-anchor">§</a>Unicode features

-   **unicode** - Enables all Unicode features. This feature is enabled
    by default, and will always cover all Unicode features, even if more
    are added in the future.
-   **unicode-age** - Provide the data for the [Unicode `Age`
    property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
    This makes it possible to use classes like `\p{Age:6.0}` to refer to
    all codepoints first introduced in Unicode 6.0
-   **unicode-bool** - Provide the data for numerous Unicode boolean
    properties. The full list is not included here, but contains
    properties like `Alphabetic`, `Emoji`, `Lowercase`, `Math`,
    `Uppercase` and `White_Space`.
-   **unicode-case** - Provide the data for case insensitive matching
    using [Unicode’s “simple loose matches”
    specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
-   **unicode-gencat** - Provide the data for [Unicode general
    categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
    This includes, but is not limited to, `Decimal_Number`, `Letter`,
    `Math_Symbol`, `Number` and `Punctuation`.
-   **unicode-perl** - Provide the data for supporting the Unicode-aware
    Perl character classes, corresponding to `\w`, `\s` and `\d`. This
    is also necessary for using Unicode-aware word boundary assertions.
    Note that if this feature is disabled, the `\s` and `\d` character
    classes are still available if the `unicode-bool` and
    `unicode-gencat` features are enabled, respectively.
-   **unicode-script** - Provide the data for [Unicode scripts and
    script extensions](https://www.unicode.org/reports/tr24/). This
    includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
    `Latin` and `Thai`.
-   **unicode-segment** - Provide the data necessary to provide the
    properties used to implement the [Unicode text segmentation
    algorithms](https://www.unicode.org/reports/tr29/). This enables
    using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
    `\p{sb=ATerm}`.
-   **unicode-word-boundary** - Enables support for Unicode word
    boundaries, i.e., `\b`, in regexes. When this and `unicode-perl` are
    enabled, then data tables from `regex-syntax` are used to implement
    Unicode word boundaries. However, if `regex-syntax` isn’t enabled as
    a dependency then one can still enable this feature. It will cause
    `regex-automata` to bundle its own data table that would otherwise
    be redundant with `regex-syntax`’s table.

### <a href="#regex-engine-features" class="doc-anchor">§</a>Regex engine features

-   **syntax** - Enables a dependency on `regex-syntax`. This makes APIs
    for building regex engines from pattern strings available. Without
    the `regex-syntax` dependency, the only way to build a regex engine
    is generally to deserialize a previously built DFA or to hand
    assemble an NFA using its [builder
    API](nfa/thompson/struct.Builder.html "struct regex_automata::nfa::thompson::Builder").
    Once you have an NFA, you can build any of the regex engines in this
    crate. The `syntax` feature also enables `alloc`.
-   **meta** - Enables the meta regex engine. This also enables the
    `syntax` and `nfa-pikevm` features, as both are the minimal
    requirements needed. The meta regex engine benefits from enabling
    any of the other regex engines and will use them automatically when
    appropriate.
-   **nfa** - Enables all NFA related features below.
    -   **nfa-thompson** - Enables the Thompson NFA APIs. This enables
        `alloc`.
    -   **nfa-pikevm** - Enables the PikeVM regex engine. This enables
        `nfa-thompson`.
    -   **nfa-backtrack** - Enables the bounded backtracker regex
        engine. This enables `nfa-thompson`.
-   **dfa** - Enables all DFA related features below.
    -   **dfa-build** - Enables APIs for determinizing DFAs from NFAs.
        This enables `nfa-thompson` and `dfa-search`.
    -   **dfa-search** - Enables APIs for searching with DFAs.
    -   **dfa-onepass** - Enables the one-pass DFA API. This enables
        `nfa-thompson`.
-   **hybrid** - Enables the hybrid NFA/DFA or “lazy DFA” regex engine.
    This enables `alloc` and `nfa-thompson`.

# Modules<a href="#modules" class="anchor">§</a>

<a href="dfa/index.html" class="mod"
title="mod regex_automata::dfa">dfa</a>  
A module for building and searching with deterministic finite automata
(DFAs).

<a href="hybrid/index.html" class="mod"
title="mod regex_automata::hybrid">hybrid</a>  
A module for building and searching with lazy deterministic finite
automata (DFAs).

<a href="meta/index.html" class="mod"
title="mod regex_automata::meta">meta</a>  
Provides a regex matcher that composes several other regex matchers
automatically.

<a href="nfa/index.html" class="mod"
title="mod regex_automata::nfa">nfa</a>  
Provides non-deterministic finite automata (NFA) and regex engines that
use them.

<a href="util/index.html" class="mod"
title="mod regex_automata::util">util</a>  
A collection of modules that provide APIs that are useful across many
regex engines.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HalfMatch.html" class="struct"
title="struct regex_automata::HalfMatch">HalfMatch</a>  
A representation of “half” of a match reported by a DFA.

<a href="struct.Input.html" class="struct"
title="struct regex_automata::Input">Input</a>  
The parameters for a regex search including the haystack to search.

<a href="struct.Match.html" class="struct"
title="struct regex_automata::Match">Match</a>  
A representation of a match reported by a regex engine.

<a href="struct.MatchError.html" class="struct"
title="struct regex_automata::MatchError">MatchError</a>  
An error indicating that a search stopped before reporting whether a
match exists or not.

<a href="struct.PatternID.html" class="struct"
title="struct regex_automata::PatternID">PatternID</a>  
The identifier of a regex pattern, represented by a
[`SmallIndex`](util/primitives/struct.SmallIndex.html "struct regex_automata::util::primitives::SmallIndex").

<a href="struct.PatternSet.html" class="struct"
title="struct regex_automata::PatternSet">PatternSet</a>  
A set of `PatternID`s.

<a href="struct.PatternSetInsertError.html" class="struct"
title="struct regex_automata::PatternSetInsertError">PatternSetInsertError</a>  
An error that occurs when a `PatternID` failed to insert into a
`PatternSet`.

<a href="struct.PatternSetIter.html" class="struct"
title="struct regex_automata::PatternSetIter">PatternSetIter</a>  
An iterator over all pattern identifiers in a
[`PatternSet`](struct.PatternSet.html "struct regex_automata::PatternSet").

<a href="struct.Span.html" class="struct"
title="struct regex_automata::Span">Span</a>  
A representation of a span reported by a regex engine.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Anchored.html" class="enum"
title="enum regex_automata::Anchored">Anchored</a>  
The type of anchored search to perform.

<a href="enum.MatchErrorKind.html" class="enum"
title="enum regex_automata::MatchErrorKind">MatchErrorKind</a>  
The underlying kind of a
[`MatchError`](struct.MatchError.html "struct regex_automata::MatchError").

<a href="enum.MatchKind.html" class="enum"
title="enum regex_automata::MatchKind">MatchKind</a>  
The kind of match semantics to use for a regex pattern.
