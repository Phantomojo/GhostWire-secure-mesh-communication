Crate matchers Copy item path

<span class="sub-heading"><a href="../src/matchers/lib.rs.html#1-553" class="src">Source</a>
</span>

Expand description

Regex matchers on character and byte streams.

## <a href="#overview" class="doc-anchor">§</a>Overview

The [`regex`](https://crates.io/crates/regex) crate implements regular
expression matching on strings and byte arrays. However, in order to
match the output of implementations of `fmt::Debug` and `fmt::Display`,
or by any code which writes to an instance of `fmt::Write` or
`io::Write`, it is necessary to first allocate a buffer, write to that
buffer, and then match the buffer against a regex.

In cases where it is not necessary to extract substrings, but only to
test whether or not output matches a regex, it is not strictly necessary
to allocate and write this output to a buffer. This crate provides a
simple interface on top of the lower-level
[`regex-automata`](https://crates.io/crates/regex-automata) library that
implements `fmt::Write` and `io::Write` for regex patterns. This may be
used to test whether streaming output matches a pattern without
buffering that output.

Users who need to extract substrings based on a pattern or who already
have buffered data should probably use the
[`regex`](https://crates.io/crates/regex) crate instead.

## <a href="#syntax" class="doc-anchor">§</a>Syntax

This crate uses the same [regex
syntax](https://docs.rs/regex-automata/0.4.3/regex_automata/#syntax) of
the `regex-automata` crate.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BuildError.html" class="struct"
title="struct matchers::BuildError">BuildError</a>  
An error that occurred during the construction of a DFA.

<a href="struct.Matcher.html" class="struct"
title="struct matchers::Matcher">Matcher</a>  
A reference to a [`Pattern`](../struct.Pattern.html) that matches a
single input.

<a href="struct.Pattern.html" class="struct"
title="struct matchers::Pattern">Pattern</a>  
A compiled match pattern that can match multipe inputs, or return a
[`Matcher`](../struct.Matcher.html) that matches a single input.
