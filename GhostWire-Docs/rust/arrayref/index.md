Crate arrayref Copy item path

<span class="sub-heading"><a href="../src/arrayref/lib.rs.html#1-504" class="src">Source</a>
</span>

Expand description

This package contains just four macros, which enable the creation of
array references to portions of arrays or slices (or things that can be
sliced).

# <a href="#examples" class="doc-anchor">§</a>Examples

Here is a simple example of slicing and dicing a slice into array
references with these macros. Here we implement a simple little-endian
conversion from bytes to `u16`, and demonstrate code that uses
`array_ref!` to extract an array reference from a larger array. Note
that the documentation for each macro also has an example of its use.

``` {.rust .rust-example-rendered}
#[macro_use]
extern crate arrayref;

fn read_u16(bytes: &[u8; 2]) -> u16 {
     bytes[0] as u16 + ((bytes[1] as u16) << 8)
}
// ...
let data = [0,1,2,3,4,0,6,7,8,9];
assert_eq!(256, read_u16(array_ref![data,0,2]));
assert_eq!(4, read_u16(array_ref![data,4,2]));
```

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.array_mut_ref.html" class="macro"
title="macro arrayref::array_mut_ref">array_mut_ref</a>  
You can use `array_mut_ref` to generate a mutable array reference to a
subset of a sliceable bit of data (which could be an array, or a slice,
or a Vec).

<a href="macro.array_ref.html" class="macro"
title="macro arrayref::array_ref">array_ref</a>  
You can use `array_ref` to generate an array reference to a subset of a
sliceable bit of data (which could be an array, or a slice, or a Vec).

<a href="macro.array_refs.html" class="macro"
title="macro arrayref::array_refs">array_refs</a>  
You can use `array_refs` to generate a series of array references to an
input array reference. The idea is if you want to break an array into a
series of contiguous and non-overlapping arrays. `array_refs` is a bit
funny in that it insists on slicing up the *entire* array. This is
intentional, as I find it handy to make me ensure that my sub-arrays add
up to the entire array. This macro will *never* panic, since the sizes
are all checked at compile time.

<a href="macro.mut_array_refs.html" class="macro"
title="macro arrayref::mut_array_refs">mut_array_refs</a>  
You can use `mut_array_refs` to generate a series of mutable array
references to an input mutable array reference. The idea is if you want
to break an array into a series of contiguous and non-overlapping
mutable array references. Like `array_refs!`, `mut_array_refs!` is a bit
funny in that it insists on slicing up the *entire* array. This is
intentional, as I find it handy to make me ensure that my sub-arrays add
up to the entire array. This macro will *never* panic, since the sizes
are all checked at compile time.
