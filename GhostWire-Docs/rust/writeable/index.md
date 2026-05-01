Crate writeable Copy item path

<span class="sub-heading"><a href="../src/writeable/lib.rs.html#6-509" class="src">Source</a>
</span>

Expand description

This crate defines
[`Writeable`](trait.Writeable.html "trait writeable::Writeable"), a
trait representing an object that can be written to a sink implementing
`std::fmt::Write`. It is an alternative to `std::fmt::Display` with the
addition of a function indicating the number of bytes to be written.

`Writeable` improves upon `std::fmt::Display` in two ways:

1.  More efficient, since the sink can pre-allocate bytes.
2.  Smaller code, since the format machinery can be short-circuited.

This crate also exports
[`TryWriteable`](trait.TryWriteable.html "trait writeable::TryWriteable"),
a writeable that supports a custom error.

# <a href="#benchmarks" class="doc-anchor">§</a>Benchmarks

The benchmarks to generate the following data can be found in the
`benches` directory.

| Case                                                 | `Writeable` | `Display` |
|------------------------------------------------------|-------------|-----------|
| Create string from single-string message (139 chars) | 15.642 ns   | 19.251 ns |
| Create string from complex message                   | 35.830 ns   | 89.478 ns |
| Write complex message to buffer                      | 57.336 ns   | 64.408 ns |

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use std::fmt;
use writeable::assert_writeable_eq;
use writeable::LengthHint;
use writeable::Writeable;

struct WelcomeMessage<'s> {
    pub name: &'s str,
}

impl<'s> Writeable for WelcomeMessage<'s> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str("Hello, ")?;
        sink.write_str(self.name)?;
        sink.write_char('!')?;
        Ok(())
    }

    fn writeable_length_hint(&self) -> LengthHint {
        // "Hello, " + '!' + length of name
        LengthHint::exact(8 + self.name.len())
    }
}

let message = WelcomeMessage { name: "Alice" };
assert_writeable_eq!(&message, "Hello, Alice!");

// Types implementing `Writeable` are recommended to also implement `fmt::Display`.
// This can be simply done by redirecting to the `Writeable` implementation:
writeable::impl_display_with_writeable!(WelcomeMessage<'_>);
assert_eq!(message.to_string(), "Hello, Alice!");
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="adapters/index.html" class="mod"
title="mod writeable::adapters">adapters</a>  
Helper types for trait impls.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.assert_try_writeable_eq.html" class="macro"
title="macro writeable::assert_try_writeable_eq">assert_try_writeable_eq</a>  
Testing macros for types implementing
[`TryWriteable`](trait.TryWriteable.html "trait writeable::TryWriteable").

<a href="macro.assert_try_writeable_parts_eq.html" class="macro"
title="macro writeable::assert_try_writeable_parts_eq">assert_try_writeable_parts_eq</a>  
See
[`assert_try_writeable_eq`](macro.assert_try_writeable_eq.html "macro writeable::assert_try_writeable_eq").

<a href="macro.impl_display_with_writeable.html" class="macro"
title="macro writeable::impl_display_with_writeable">impl_display_with_writeable</a>  
Implements
[`Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display")
for types that implement
[`Writeable`](trait.Writeable.html "trait writeable::Writeable").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.LengthHint.html" class="struct"
title="struct writeable::LengthHint">LengthHint</a>  
A hint to help consumers of `Writeable` pre-allocate bytes before they
call
[`write_to`](trait.Writeable.html#method.write_to "method writeable::Writeable::write_to").

<a href="struct.Part.html" class="struct"
title="struct writeable::Part">Part</a>  
[`Part`](struct.Part.html "struct writeable::Part")s are used as
annotations for formatted strings.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.PartsWrite.html" class="trait"
title="trait writeable::PartsWrite">PartsWrite</a>  
A sink that supports annotating parts of the string with
[`Part`](struct.Part.html "struct writeable::Part")s.

<a href="trait.TryWriteable.html" class="trait"
title="trait writeable::TryWriteable">TryWriteable</a>  
A writeable object that can fail while writing.

<a href="trait.Writeable.html" class="trait"
title="trait writeable::Writeable">Writeable</a>  
`Writeable` is an alternative to `std::fmt::Display` with the addition
of a length function.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.cmp_str.html" class="fn"
title="fn writeable::cmp_str">cmp_str</a>  
Compares the contents of a `Writeable` to the given bytes without
allocating a String to hold the `Writeable` contents.

<a href="fn.cmp_utf8.html" class="fn"
title="fn writeable::cmp_utf8">cmp_utf8</a>  
Compares the contents of a
[`Writeable`](trait.Writeable.html "trait writeable::Writeable") to the
given UTF-8 bytes without allocating memory.
