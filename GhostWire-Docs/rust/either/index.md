Crate either Copy item path

<span class="sub-heading"><a href="../src/either/lib.rs.html#1-1561" class="src">Source</a>
</span>

Expand description

The enum [`Either`](enum.Either.html) with variants `Left` and `Right`
is a general purpose sum type with two cases.

**Crate features:**

-   `"std"` Enabled by default. Disable to make the library
    `#![no_std]`.

-   `"serde"` Disabled by default. Enable to
    `#[derive(Serialize, Deserialize)]` for `Either`

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::Either::`<a href="enum.Either.html#variant.Left" class="enum"
title="enum either::Either"><code>Left</code></a>`;`

`pub use crate::Either::`<a href="enum.Either.html#variant.Right" class="enum"
title="enum either::Either"><code>Right</code></a>`;`

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.for_both.html" class="macro"
title="macro either::for_both">for_both</a>  
Evaluate the provided expression for both
[`Either::Left`](enum.Either.html#variant.Left "variant either::Either::Left")
and
[`Either::Right`](enum.Either.html#variant.Right "variant either::Either::Right").

<a href="macro.try_left.html" class="macro"
title="macro either::try_left">try_left</a>  
Macro for unwrapping the left side of an
[`Either`](enum.Either.html "enum either::Either"), which fails early
with the opposite side. Can only be used in functions that return
`Either` because of the early return of `Right` that it provides.

<a href="macro.try_right.html" class="macro"
title="macro either::try_right">try_right</a>  
Dual to [`try_left!`](macro.try_left.html "macro either::try_left"), see
its documentation for more information.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.IterEither.html" class="struct"
title="struct either::IterEither">IterEither</a>  
Iterator that maps left or right iterators to corresponding
`Either`-wrapped items.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Either.html" class="enum"
title="enum either::Either">Either</a>  
The enum `Either` with variants `Left` and `Right` is a general purpose
sum type with two cases.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IntoEither.html" class="trait"
title="trait either::IntoEither">IntoEither</a>  
Provides methods for converting a type `Self` into either a
[`Left`](enum.Either.html#variant.Left "variant either::Either::Left")
or
[`Right`](enum.Either.html#variant.Right "variant either::Either::Right")
variant of
[`Either<Self, Self>`](enum.Either.html "enum either::Either").
