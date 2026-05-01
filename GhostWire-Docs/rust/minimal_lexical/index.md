Crate minimal\_lexical Copy item path

<span class="sub-heading"><a href="../src/minimal_lexical/lib.rs.html#1-68" class="src">Source</a>
</span>

Expand description

Fast, minimal float-parsing algorithm.

minimal-lexical has a simple, high-level API with a single exported
function: [`parse_float`](fn.parse_float.html).

[`parse_float`](fn.parse_float.html) expects a forward iterator for the
integer and fraction digits, as well as a parsed exponent as an
[`i32`](https://doc.rust-lang.org/stable/std/primitive.i32.html).

For more examples, please see
[simple-example](https://github.com/Alexhuszagh/minimal-lexical/blob/master/examples/simple.rs).

## <a href="#examples" class="doc-anchor">§</a>EXAMPLES

``` {.rust .rust-example-rendered}
extern crate minimal_lexical;

// Let's say we want to parse "1.2345".
// First, we need an external parser to extract the integer digits ("1"),
// the fraction digits ("2345"), and then parse the exponent to a 32-bit
// integer (0).
// Warning:
// --------
//  Please note that leading zeros must be trimmed from the integer,
//  and trailing zeros must be trimmed from the fraction. This cannot
//  be handled by minimal-lexical, since we accept iterators.
let integer = b"1";
let fraction = b"2345";
let float: f64 = minimal_lexical::parse_float(integer.iter(), fraction.iter(), 0);
println!("float={:?}", float);    // 1.235
```

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Float.html" class="trait"
title="trait minimal_lexical::Float">Float</a>  
Generic floating-point type, to be used in generic code for parsing.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.parse_float.html" class="fn"
title="fn minimal_lexical::parse_float">parse_float</a>  
Parse float from extracted float components.
