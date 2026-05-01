Crate bitflags Copy item path

<span class="sub-heading"><a href="../src/bitflags/lib.rs.html#11-997" class="src">Source</a>
</span>

Expand description

Generate types for C-style flags with ergonomic APIs.

# <a href="#getting-started" class="doc-anchor">§</a>Getting started

Add `bitflags` to your `Cargo.toml`:

``` language-toml
[dependencies.bitflags]
version = "2.10.0"
```

## <a href="#crate-features" class="doc-anchor">§</a>Crate features

The `bitflags` library defines a few Cargo features that you can opt-in
to:

-   `std`: Implement the `Error` trait on error types used by
    `bitflags`.
-   `serde`: Support deriving `serde` traits on generated flags types.
-   `arbitrary`: Support deriving `arbitrary` traits on generated flags
    types.
-   `bytemuck`: Support deriving `bytemuck` traits on generated flags
    types.

## <a href="#generating-flags-types" class="doc-anchor">§</a>Generating flags types

Use the [`bitflags`](macro.bitflags.html "macro bitflags::bitflags")
macro to generate flags types:

``` {.rust .rust-example-rendered}
use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
```

See the docs for the `bitflags` macro for the full syntax.

Also see the [`example_generated`](./example_generated/index.html)
module for an example of what the `bitflags` macro generates for a flags
type.

### <a href="#externally-defined-flags" class="doc-anchor">§</a>Externally defined flags

If you’re generating flags types for an external source, such as a C
API, you can define an extra unnamed flag as a mask of all bits the
external source may ever set. Usually this would be all bits (`!0`):

``` {.rust .rust-example-rendered}
bitflags! {
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;

        // The source may set any bits
        const _ = !0;
    }
}
```

Why should you do this? Generated methods like `all` and truncating
operators like `!` only consider bits in defined flags. Adding an
unnamed flag makes those methods consider additional bits, without
generating additional constants for them. It helps compatibility when
the external source may start setting additional bits at any time. The
[known and unknown bits](#known-and-unknown-bits) section has more
details on this behavior.

### <a href="#custom-derives" class="doc-anchor">§</a>Custom derives

You can derive some traits on generated flags types if you enable Cargo
features. The following libraries are currently supported:

-   `serde`: Support `#[derive(Serialize, Deserialize)]`, using text for
    human-readable formats, and a raw number for binary formats.
-   `arbitrary`: Support `#[derive(Arbitrary)]`, only generating flags
    values with known bits.
-   `bytemuck`: Support `#[derive(Pod, Zeroable)]`, for casting between
    flags values and their underlying bits values.

You can also define your own flags type outside of the
[`bitflags`](macro.bitflags.html "macro bitflags::bitflags") macro and
then use it to generate methods. This can be useful if you need a custom
`#[derive]` attribute for a library that `bitflags` doesn’t natively
support:

``` {.rust .rust-example-rendered}
#[derive(SomeTrait)]
pub struct Flags(u32);

bitflags! {
    impl Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
```

### <a href="#adding-custom-methods" class="doc-anchor">§</a>Adding custom methods

The [`bitflags`](macro.bitflags.html "macro bitflags::bitflags") macro
supports attributes on generated flags types within the macro itself,
while `impl` blocks can be added outside of it:

``` {.rust .rust-example-rendered}
bitflags! {
    // Attributes can be applied to flags types
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}

// Impl blocks can be added to flags types
impl Flags {
    pub fn as_u64(&self) -> u64 {
        self.bits() as u64
    }
}
```

## <a href="#working-with-flags-values" class="doc-anchor">§</a>Working with flags values

Use generated constants and standard bitwise operators to interact with
flags values:

``` {.rust .rust-example-rendered}
// union
let ab = Flags::A | Flags::B;

// intersection
let a = ab & Flags::A;

// difference
let b = ab - Flags::A;

// complement
let c = !ab;
```

See the docs for the [`Flags`](trait.Flags.html "trait bitflags::Flags")
trait for more details on operators and how they behave.

# <a href="#formatting-and-parsing" class="doc-anchor">§</a>Formatting and parsing

`bitflags` defines a text format that can be used to convert any flags
value to and from strings.

See the [`parser`](parser/index.html "mod bitflags::parser") module for
more details.

# <a href="#specification" class="doc-anchor">§</a>Specification

The terminology and behavior of generated flags types is [specified in
the source
repository](https://github.com/bitflags/bitflags/blob/main/spec.md).
Details are repeated in these docs where appropriate, but is
exhaustively listed in the spec. Some things are worth calling out
explicitly here.

## <a href="#flags-types-flags-values-flags" class="doc-anchor">§</a>Flags types, flags values, flags

The spec and these docs use consistent terminology to refer to things in
the bitflags domain:

-   **Bits type**: A type that defines a fixed number of bits at
    specific locations.
-   **Flag**: A set of bits in a bits type that may have a unique name.
-   **Flags type**: A set of defined flags over a specific bits type.
-   **Flags value**: An instance of a flags type using its specific bits
    value for storage.

``` {.rust .rust-example-rendered}
bitflags! {
    struct FlagsType: u8 {
//                    -- Bits type
//         --------- Flags type
        const A = 1;
//            ----- Flag
    }
}

let flag = FlagsType::A;
//  ---- Flags value
```

## <a href="#known-and-unknown-bits" class="doc-anchor">§</a>Known and unknown bits

Any bits in a flag you define are called *known bits*. Any other bits
are *unknown bits*. In the following flags type:

``` {.rust .rust-example-rendered}
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}
```

The known bits are `0b0000_0111` and the unknown bits are `0b1111_1000`.

`bitflags` doesn’t guarantee that a flags value will only ever have
known bits set, but some operators will unset any unknown bits they
encounter. In a future version of `bitflags`, all operators will unset
unknown bits.

If you’re using `bitflags` for flags types defined externally, such as
from C, you probably want all bits to be considered known, in case that
external source changes. You can do this using an unnamed flag, as
described in [externally defined flags](#externally-defined-flags).

## <a href="#zero-bit-flags" class="doc-anchor">§</a>Zero-bit flags

Flags with no bits set should be avoided because they interact strangely
with
[`Flags::contains`](trait.Flags.html#method.contains "method bitflags::Flags::contains")
and
[`Flags::intersects`](trait.Flags.html#method.intersects "method bitflags::Flags::intersects").
A zero-bit flag is always contained, but is never intersected. The names
of zero-bit flags can be parsed, but are never formatted.

## <a href="#multi-bit-flags" class="doc-anchor">§</a>Multi-bit flags

Flags that set multiple bits should be avoided unless each bit is also
in a single-bit flag. Take the following flags type as an example:

``` {.rust .rust-example-rendered}
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 | 1 << 1;
    }
}
```

The result of `Flags::A ^ Flags::B` is `0b0000_0010`, which doesn’t
correspond to either `Flags::A` or `Flags::B` even though it’s still a
known bit.

# Modules<a href="#modules" class="anchor">§</a>

<a href="iter/index.html" class="mod"
title="mod bitflags::iter">iter</a>  
Yield the bits of a source flags value in a set of contained flags
values.

<a href="parser/index.html" class="mod"
title="mod bitflags::parser">parser</a>  
Parsing flags from text.

<a href="serde/index.html" class="mod"
title="mod bitflags::serde">serde</a>  
Specialized serialization for flags types using `serde`.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.bitflags.html" class="macro"
title="macro bitflags::bitflags">bitflags</a>  
Generate a flags type.

<a href="macro.bitflags_match.html" class="macro"
title="macro bitflags::bitflags_match">bitflags_match</a>  
A macro that matches flags values, similar to Rust’s `match` statement.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Flag.html" class="struct"
title="struct bitflags::Flag">Flag</a>  
A defined flags value that may be named or unnamed.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Bits.html" class="trait"
title="trait bitflags::Bits">Bits</a>  
A bits type that can be used as storage for a flags type.

<a href="trait.Flags.html" class="trait"
title="trait bitflags::Flags">Flags</a>  
A set of defined flags using a bits type as storage.
