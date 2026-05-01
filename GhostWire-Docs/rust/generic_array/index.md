Crate generic\_array Copy item path

<span class="sub-heading"><a href="../src/generic_array/lib.rs.html#1-690" class="src">Source</a>
</span>

Expand description

This crate implements a structure that can be used as a generic array
type. Core Rust array types `[T; N]` can’t be used generically with
respect to `N`, so for example this:

    struct Foo<T, N> {
        data: [T; N]
    }

won’t work.

**generic-array** exports a `GenericArray<T,N>` type, which lets the
above be implemented as:

``` {.rust .rust-example-rendered}
use generic_array::{ArrayLength, GenericArray};

struct Foo<T, N: ArrayLength<T>> {
    data: GenericArray<T,N>
}
```

The `ArrayLength<T>` trait is implemented by default for [unsigned
integer types](../typenum/uint/index.html) from
[typenum](../typenum/index.html):

``` {.rust .rust-example-rendered}
use generic_array::typenum::U5;

struct Foo<N: ArrayLength<i32>> {
    data: GenericArray<i32, N>
}

let foo = Foo::<U5>{data: GenericArray::default()};
```

For example, `GenericArray<T, U5>` would work almost like `[T; 5]`:

``` {.rust .rust-example-rendered}
use generic_array::typenum::U5;

struct Foo<T, N: ArrayLength<T>> {
    data: GenericArray<T, N>
}

let foo = Foo::<i32, U5>{data: GenericArray::default()};
```

For ease of use, an `arr!` macro is provided - example below:

``` {.rust .rust-example-rendered}
let array = arr![u32; 1, 2, 3];
assert_eq!(array[2], 3);
```

# Re-exports<a href="#reexports" class="anchor">§</a>

pub extern crate <a href="../typenum/index.html" class="mod"
title="mod typenum">typenum</a>;

# Re-exports<a href="#reexports-1" class="anchor">§</a>

`pub use self::iter::`<a href="iter/struct.GenericArrayIter.html" class="struct"
title="struct generic_array::iter::GenericArrayIter"><code>GenericArrayIter</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="arr/index.html" class="mod"
title="mod generic_array::arr">arr</a>  
Implementation for `arr!` macro.

<a href="functional/index.html" class="mod"
title="mod generic_array::functional">functional</a>  
Functional programming with generic sequences

<a href="iter/index.html" class="mod"
title="mod generic_array::iter">iter</a>  
`GenericArray` iterator implementation.

<a href="sequence/index.html" class="mod"
title="mod generic_array::sequence">sequence</a>  
Useful traits for manipulating sequences of data stored in
`GenericArray`s

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.arr.html" class="macro"
title="macro generic_array::arr">arr</a>  
Macro allowing for easy generation of Generic Arrays. Example:
`let test = arr![u32; 1, 2, 3];`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.GenericArray.html" class="struct"
title="struct generic_array::GenericArray">GenericArray</a>  
Struct representing a generic array - `GenericArray<T, N>` works like
\[T; N\]

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ArrayLength.html" class="trait"
title="trait generic_array::ArrayLength">ArrayLength</a>  
Trait making `GenericArray` work, marking types to be used as length of
an array
