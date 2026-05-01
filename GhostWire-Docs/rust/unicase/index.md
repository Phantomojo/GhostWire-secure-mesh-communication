Crate unicase Copy item path

<span class="sub-heading"><a href="../src/unicase/lib.rs.html#1-437" class="src">Source</a>
</span>

Expand description

# <a href="#unicase" class="doc-anchor">§</a>UniCase

UniCase provides a way of specifying strings that are case-insensitive.

UniCase supports full [Unicode case
folding](https://www.w3.org/International/wiki/Case_folding). It can
also utilize faster ASCII case comparisons, if both strings are ASCII.

Using the `UniCase::new()` constructor will check the string to see if
it is all ASCII. When a `UniCase` is compared against another, if both
are ASCII, it will use the faster comparison.

There also exists the `Ascii` type in this crate, which will always
assume to use the ASCII case comparisons, if the encoding is already
known.

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use unicase::UniCase;

let a = UniCase::new("Maße");
let b = UniCase::new("MASSE");
let c = UniCase::new("mase");

assert_eq!(a, b);
assert!(b != c);
```

## <a href="#ascii" class="doc-anchor">§</a>Ascii

``` {.rust .rust-example-rendered}
use unicase::Ascii;

let a = Ascii::new("foobar");
let b = Ascii::new("FoObAr");

assert_eq!(a, b);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Ascii.html" class="struct"
title="struct unicase::Ascii">Ascii</a>  
Case Insensitive wrapper of Ascii strings.

<a href="struct.UniCase.html" class="struct"
title="struct unicase::UniCase">UniCase</a>  
Case Insensitive wrapper of strings.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.eq.html" class="fn" title="fn unicase::eq">eq</a>  
Compare two string-like types for case-less equality, using unicode
folding.

<a href="fn.eq_ascii.html" class="fn"
title="fn unicase::eq_ascii">eq_ascii</a>  
Compare two string-like types for case-less equality, ignoring ASCII
case.
