Crate rustc\_demangle Copy item path

<span class="sub-heading"><a href="../src/rustc_demangle/lib.rs.html#1-588" class="src">Source</a>
</span>

Expand description

Demangle Rust compiler symbol names.

This crate provides a `demangle` function which will return a `Demangle`
sentinel value that can be used to learn about the demangled version of
a symbol name. The demangled representation will be the same as the
original if it doesn’t look like a mangled symbol name.

`Demangle` can be formatted with the `Display` trait. The alternate
modifier (`#`) can be used to format the symbol name without the
trailing hash value.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use rustc_demangle::demangle;

assert_eq!(demangle("_ZN4testE").to_string(), "test");
assert_eq!(demangle("_ZN3foo3barE").to_string(), "foo::bar");
assert_eq!(demangle("foo").to_string(), "foo");
// With hash
assert_eq!(format!("{}", demangle("_ZN3foo17h05af221e174051e9E")), "foo::h05af221e174051e9");
// Without hash
assert_eq!(format!("{:#}", demangle("_ZN3foo17h05af221e174051e9E")), "foo");
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Demangle.html" class="struct"
title="struct rustc_demangle::Demangle">Demangle</a>  
Representation of a demangled symbol name.

<a href="struct.TryDemangleError.html" class="struct"
title="struct rustc_demangle::TryDemangleError">TryDemangleError</a>  
Error returned from the `try_demangle` function below when demangling
fails.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.demangle.html" class="fn"
title="fn rustc_demangle::demangle">demangle</a>  
De-mangles a Rust symbol into a more readable version

<a href="fn.try_demangle.html" class="fn"
title="fn rustc_demangle::try_demangle">try_demangle</a>  
The same as `demangle`, except return an `Err` if the string does not
appear to be a Rust symbol, rather than “demangling” the given string as
a no-op.
