Crate flagset Copy item path

<span class="sub-heading"><a href="../src/flagset/lib.rs.html#19-1350" class="src">Source</a>
</span>

Expand description

# <a href="#welcome-to-flagset" class="doc-anchor">§</a>Welcome to FlagSet!

FlagSet is a new, ergonomic approach to handling flags that combines the
best of existing crates like `bitflags` and `enumflags` without their
downsides.

## <a href="#existing-implementations" class="doc-anchor">§</a>Existing Implementations

The `bitflags` crate has long been part of the Rust ecosystem.
Unfortunately, it doesn’t feel like natural Rust. The `bitflags` crate
uses a weird struct format to define flags. Flags themselves are just
integers constants, so there is little type-safety involved. But it
doesn’t have any dependencies. It also allows you to define implied
flags (otherwise known as overlapping flags).

The `enumflags` crate tried to improve on `bitflags` by using
enumerations to define flags. This was a big improvement to the natural
feel of the code. Unfortunately, there are some design flaws. To
generate the flags, procedural macros were used. This implied two
separate crates plus additional dependencies. Further, `enumflags`
specifies the size of the flags using a `repr($size)` attribute.
Unfortunately, this attribute cannot resolve type aliases, such as
`c_int`. This makes `enumflags` a poor fit for FFI, which is the most
important place for a flags library. The `enumflags` crate also
disallows overlapping flags and is not maintained.

FlagSet improves on both of these by adopting the `enumflags` natural
feel and the `bitflags` mode of flag generation; as well as additional
API usage niceties. FlagSet has no dependencies and is extensively
documented and tested. It also tries very hard to prevent you from
making mistakes by avoiding external usage of the integer types. FlagSet
is also a zero-cost abstraction: all functions are inlineable and should
reduce to the core integer operations. FlagSet also does not depend on
stdlib, so it can be used in `no_std` libraries and applications.

## <a href="#defining-flags" class="doc-anchor">§</a>Defining Flags

Flags are defined using the `flags!` macro:

``` {.rust .rust-example-rendered}
use flagset::{FlagSet, flags};
use std::os::raw::c_int;

flags! {
    enum FlagsA: u8 {
        Foo,
        Bar,
        Baz,
    }

    enum FlagsB: c_int {
        Foo,
        Bar,
        Baz,
    }
}
```

Notice that a flag definition looks just like a regular enumeration,
with the addition of the field-size type. The field-size type is
required and can be either a type or a type alias. Both examples are
given above.

Also note that the field-size type specifies the size of the
corresponding `FlagSet` type, not size of the enumeration itself. To
specify the size of the enumeration, use the `repr($size)` attribute as
specified below.

## <a href="#flag-values" class="doc-anchor">§</a>Flag Values

Flags often need values assigned to them. This can be done implicitly,
where the value depends on the order of the flags:

``` {.rust .rust-example-rendered}
use flagset::{FlagSet, flags};

flags! {
    enum Flags: u16 {
        Foo, // Implicit Value: 0b0001
        Bar, // Implicit Value: 0b0010
        Baz, // Implicit Value: 0b0100
    }
}
```

Alternatively, flag values can be defined explicitly, by specifying any
`const` expression:

``` {.rust .rust-example-rendered}
use flagset::{FlagSet, flags};

flags! {
    enum Flags: u16 {
        Foo = 0x01,   // Explicit Value: 0b0001
        Bar = 2,      // Explicit Value: 0b0010
        Baz = 0b0100, // Explicit Value: 0b0100
    }
}
```

Flags can also overlap or “imply” other flags:

``` {.rust .rust-example-rendered}
use flagset::{FlagSet, flags};

flags! {
    enum Flags: u16 {
        Foo = 0b0001,
        Bar = 0b0010,
        Baz = 0b0110, // Implies Bar
        All = (Flags::Foo | Flags::Bar | Flags::Baz).bits(),
    }
}
```

## <a href="#specifying-attributes" class="doc-anchor">§</a>Specifying Attributes

Attributes can be used on the enumeration itself or any of the values:

``` {.rust .rust-example-rendered}
use flagset::{FlagSet, flags};

flags! {
    #[derive(PartialOrd, Ord)]
    enum Flags: u8 {
        Foo,
        #[deprecated]
        Bar,
        Baz,
    }
}
```

## <a href="#collections-of-flags" class="doc-anchor">§</a>Collections of Flags

A collection of flags is a `FlagSet<T>`. If you are storing the flags in
memory, the raw `FlagSet<T>` type should be used. However, if you want
to receive flags as an input to a function, you should use
`impl Into<FlagSet<T>>`. This allows for very ergonomic APIs:

``` {.rust .rust-example-rendered}
use flagset::{FlagSet, flags};

flags! {
    enum Flags: u8 {
        Foo,
        Bar,
        Baz,
    }
}

struct Container(FlagSet<Flags>);

impl Container {
    fn new(flags: impl Into<FlagSet<Flags>>) -> Container {
        Container(flags.into())
    }
}

assert_eq!(Container::new(Flags::Foo | Flags::Bar).0.bits(), 0b011);
assert_eq!(Container::new(Flags::Foo).0.bits(), 0b001);
assert_eq!(Container::new(None).0.bits(), 0b000);
```

## <a href="#operations" class="doc-anchor">§</a>Operations

Operations can be performed on a `FlagSet<F>` or on individual flags:

| Operator | Assignment Operator | Meaning                |
|----------|---------------------|------------------------|
| \|       | \|=                 | Union                  |
| &        | &=                  | Intersection           |
| ^        | ^=                  | Toggle specified flags |
| \-       | -=                  | Difference             |
| %        | %=                  | Symmetric difference   |
| !        |                     | Toggle all flags       |

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.flags.html" class="macro"
title="macro flagset::flags">flags</a>  
Define flag value using the `enum` syntax. See below for details.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.FlagSet.html" class="struct"
title="struct flagset::FlagSet">FlagSet</a>  
<a href="struct.InvalidBits.html" class="struct"
title="struct flagset::InvalidBits">InvalidBits</a>  
Error type returned when creating a new flagset from bits is invalid or
undefined.
