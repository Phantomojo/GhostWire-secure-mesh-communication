Crate zerovec Copy item path

<span class="sub-heading"><a href="../src/zerovec/lib.rs.html#5-576" class="src">Source</a>
</span>

Expand description

Zero-copy vector abstractions for arbitrary types, backed by byte
slices.

`zerovec` enables a far wider range of types — beyond just `&[u8]` and
`&str` — to participate in zero-copy deserialization from byte slices.
It is `serde` compatible and comes equipped with proc macros

Clients upgrading to `zerovec` benefit from zero heap allocations when
deserializing read-only data.

This crate has four main types:

-   [`ZeroVec<'a, T>`](struct.ZeroVec.html "struct zerovec::ZeroVec")
    (and
    [`ZeroSlice<T>`](struct.ZeroSlice.html "struct zerovec::ZeroSlice"))
    for fixed-width types like `u32`
-   [`VarZeroVec<'a, T>`](struct.VarZeroVec.html "struct zerovec::VarZeroVec")
    (and
    [`VarZeroSlice<T>`](struct.ZeroSlice.html "struct zerovec::ZeroSlice"))
    for variable-width types like `str`
-   [`ZeroMap<'a, K, V>`](ZeroMap) to map from `K` to `V`
-   [`ZeroMap2d<'a, K0, K1, V>`](ZeroMap2d) to map from the pair
    `(K0, K1)` to `V`

The first two are intended as close-to-drop-in replacements for `Vec<T>`
in Serde structs. The third and fourth are intended as a replacement for
`HashMap` or [`LiteMap`](https://docs.rs/litemap). When used with Serde
derives, **be sure to apply `#[serde(borrow)]` to these types**, same as
one would for [`Cow<'a, T>`](alloc::borrow::Cow).

[`ZeroVec<'a, T>`](struct.ZeroVec.html "struct zerovec::ZeroVec"),
[`VarZeroVec<'a, T>`](struct.VarZeroVec.html "struct zerovec::VarZeroVec"),
[`ZeroMap<'a, K, V>`](ZeroMap), and
[`ZeroMap2d<'a, K0, K1, V>`](ZeroMap2d) all behave like
[`Cow<'a, T>`](alloc::borrow::Cow) in that they abstract over either
borrowed or owned data. When performing deserialization from
human-readable formats (like `json` and `xml`), typically these types
will allocate and fully own their data, whereas if deserializing from
binary formats like `bincode` and `postcard`, these types will borrow
data directly from the buffer being deserialized from, avoiding
allocations and only performing validity checks. As such, this crate can
be pretty fast (see [below](#Performance) for more information) on
deserialization.

See [the design
doc](https://github.com/unicode-org/icu4x/blob/main/utils/zerovec/design_doc.md)
for details on how this crate works under the hood.

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo features

This crate has several optional Cargo features:

-   `serde`: Allows serializing and deserializing `zerovec`’s
    abstractions via [`serde`](https://docs.rs/serde)
-   `yoke`: Enables implementations of `Yokeable` from the
    [`yoke`](https://docs.rs/yoke/) crate, which is also useful in
    situations involving a lot of zero-copy deserialization.
-   `derive`: Makes it easier to use custom types in these collections
    by providing the
    [`#[make_ule]`](attr.make_ule.html "attr zerovec::make_ule") and
    [`#[make_varule]`](attr.make_varule.html "attr zerovec::make_varule")
    proc macros, which generate appropriate
    [`ULE`](ule/trait.ULE.html "trait zerovec::ule::ULE") and
    [`VarULE`](ule/trait.VarULE.html "trait zerovec::ule::VarULE")-conformant
    types for a given “normal” type.
-   `std`: Enabled `std::Error` implementations for error types. This
    crate is by default `no_std` with a dependency on `alloc`.

# <a href="#examples" class="doc-anchor">§</a>Examples

Serialize and deserialize a struct with ZeroVec and VarZeroVec with
Bincode:

``` {.rust .rust-example-rendered}
use zerovec::{VarZeroVec, ZeroVec};

// This example requires the "serde" feature
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataStruct<'data> {
    #[serde(borrow)]
    nums: ZeroVec<'data, u32>,
    #[serde(borrow)]
    chars: ZeroVec<'data, char>,
    #[serde(borrow)]
    strs: VarZeroVec<'data, str>,
}

let data = DataStruct {
    nums: ZeroVec::from_slice_or_alloc(&[211, 281, 421, 461]),
    chars: ZeroVec::alloc_from_slice(&['ö', '冇', 'म']),
    strs: VarZeroVec::from(&["hello", "world"]),
};
let bincode_bytes =
    bincode::serialize(&data).expect("Serialization should be successful");
assert_eq!(bincode_bytes.len(), 63);

let deserialized: DataStruct = bincode::deserialize(&bincode_bytes)
    .expect("Deserialization should be successful");
assert_eq!(deserialized.nums.first(), Some(211));
assert_eq!(deserialized.chars.get(1), Some('冇'));
assert_eq!(deserialized.strs.get(1), Some("world"));
// The deserialization will not have allocated anything
assert!(!deserialized.nums.is_owned());
```

Use custom types inside of ZeroVec:

``` {.rust .rust-example-rendered}
use zerovec::{ZeroVec, VarZeroVec, ZeroMap};
use std::borrow::Cow;
use zerovec::ule::encode_varule_to_box;

// custom fixed-size ULE type for ZeroVec
#[zerovec::make_ule(DateULE)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Date {
    y: u64,
    m: u8,
    d: u8
}

// custom variable sized VarULE type for VarZeroVec
#[zerovec::make_varule(PersonULE)]
#[zerovec::derive(Serialize, Deserialize)] // add Serde impls to PersonULE
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Person<'a> {
    birthday: Date,
    favorite_character: char,
    #[serde(borrow)]
    name: Cow<'a, str>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data<'a> {
    #[serde(borrow)]
    important_dates: ZeroVec<'a, Date>,
    // note: VarZeroVec always must reference the ULE type directly
    #[serde(borrow)]
    important_people: VarZeroVec<'a, PersonULE>,
    #[serde(borrow)]
    birthdays_to_people: ZeroMap<'a, Date, PersonULE>
}


let person1 = Person {
    birthday: Date { y: 1990, m: 9, d: 7},
    favorite_character: 'π',
    name: Cow::from("Kate")
};
let person2 = Person {
    birthday: Date { y: 1960, m: 5, d: 25},
    favorite_character: '冇',
    name: Cow::from("Jesse")
};

let important_dates = ZeroVec::alloc_from_slice(&[Date { y: 1943, m: 3, d: 20}, Date { y: 1976, m: 8, d: 2}, Date { y: 1998, m: 2, d: 15}]);
let important_people = VarZeroVec::from(&[&person1, &person2]);
let mut birthdays_to_people: ZeroMap<Date, PersonULE> = ZeroMap::new();
// `.insert_var_v()` is slightly more convenient over `.insert()` for custom ULE types
birthdays_to_people.insert_var_v(&person1.birthday, &person1);
birthdays_to_people.insert_var_v(&person2.birthday, &person2);

let data = Data { important_dates, important_people, birthdays_to_people };

let bincode_bytes = bincode::serialize(&data)
    .expect("Serialization should be successful");
assert_eq!(bincode_bytes.len(), 160);

let deserialized: Data = bincode::deserialize(&bincode_bytes)
    .expect("Deserialization should be successful");

assert_eq!(deserialized.important_dates.get(0).unwrap().y, 1943);
assert_eq!(&deserialized.important_people.get(1).unwrap().name, "Jesse");
assert_eq!(&deserialized.important_people.get(0).unwrap().name, "Kate");
assert_eq!(&deserialized.birthdays_to_people.get(&person1.birthday).unwrap().name, "Kate");

} // feature = serde and derive
```

# <a href="#performance" class="doc-anchor">§</a>Performance

`zerovec` is designed for fast deserialization from byte buffers with
zero memory allocations while minimizing performance regressions for
common vector operations.

Benchmark results on x86\_64:

| Operation | `Vec<T>` | `zerovec` |
|----|----|----|
| Deserialize vec of 100 `u32` | 233.18 ns | 14.120 ns |
| Compute sum of vec of 100 `u32` (read every element) | 8.7472 ns | 10.775 ns |
| Binary search vec of 1000 `u32` 50 times | 442.80 ns | 472.51 ns |
| Deserialize vec of 100 strings | 7.3740 μs\* | 1.4495 μs |
| Count chars in vec of 100 strings (read every element) | 747.50 ns | 955.28 ns |
| Binary search vec of 500 strings 10 times | 466.09 ns | 790.33 ns |

\* *This result is reported for `Vec<String>`. However, Serde also
supports deserializing to the partially-zero-copy `Vec<&str>`; this
gives 1.8420 μs, much faster than `Vec<String>` but a bit slower than
`zerovec`.*

| Operation | `HashMap<K,V>` | `LiteMap<K,V>` | `ZeroMap<K,V>` |
|----|----|----|----|
| Deserialize a small map | 2.72 μs | 1.28 μs | 480 ns |
| Deserialize a large map | 50.5 ms | 18.3 ms | 3.74 ms |
| Look up from a small deserialized map | 49 ns | 42 ns | 54 ns |
| Look up from a large deserialized map | 51 ns | 155 ns | 213 ns |

Small = 16 elements, large = 131,072 elements. Maps contain
`<String, String>`.

The benches used to generate the above table can be found in the
`benches` directory in the project repository. `zeromap` benches are
named by convention, e.g. `zeromap/deserialize/small`,
`zeromap/lookup/large`. The type is appended for baseline comparisons,
e.g. `zeromap/lookup/small/hashmap`.

# Modules<a href="#modules" class="anchor">§</a>

<a href="ule/index.html" class="mod" title="mod zerovec::ule">ule</a>  
Traits over unaligned little-endian data (ULE, pronounced “yule”).

<a href="vecs/index.html" class="mod" title="mod zerovec::vecs">vecs</a>  
This module contains additional utility types for working with
[`ZeroVec`](struct.ZeroVec.html "struct zerovec::ZeroVec") and
[`VarZeroVec`](struct.VarZeroVec.html "struct zerovec::VarZeroVec"). See
their docs for more details on the general purpose of these types.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.impl_ule_from_array.html" class="macro"
title="macro zerovec::impl_ule_from_array">impl_ule_from_array</a>  
Given `Self` (`$aligned`), `Self::ULE` (`$unaligned`), and a conversion
function (`$single` or `Self::from_aligned`), implement `from_array` for
arrays of `$aligned` to `$unaligned`.

<a href="macro.zeroslice.html" class="macro"
title="macro zerovec::zeroslice">zeroslice</a>  
Convenience wrapper for
[`ZeroSlice::from_ule_slice`](struct.ZeroSlice.html#method.from_ule_slice "associated function zerovec::ZeroSlice::from_ule_slice").
The value will be created at compile-time, meaning that all arguments
must also be constant.

<a href="macro.zerovec.html" class="macro"
title="macro zerovec::zerovec">zerovec</a>  
Creates a borrowed `ZeroVec`. Convenience wrapper for
`zeroslice!(...).as_zerovec()`. The value will be created at
compile-time, meaning that all arguments must also be constant.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.VarZeroCow.html" class="struct"
title="struct zerovec::VarZeroCow">VarZeroCow</a>  
Copy-on-write type that efficiently represents
[`VarULE`](ule/trait.VarULE.html "trait zerovec::ule::VarULE") types as
their bitstream representation.

<a href="struct.VarZeroSlice.html" class="struct"
title="struct zerovec::VarZeroSlice">VarZeroSlice</a>  
A zero-copy “slice”, that works for unsized types, i.e. the zero-copy
version of `[T]` where `T` is not `Sized`.

<a href="struct.VarZeroVec.html" class="struct"
title="struct zerovec::VarZeroVec">VarZeroVec</a>  
A zero-copy, byte-aligned vector for variable-width types.

<a href="struct.ZeroSlice.html" class="struct"
title="struct zerovec::ZeroSlice">ZeroSlice</a>  
A zero-copy “slice”, i.e. the zero-copy version of `[T]`.

<a href="struct.ZeroVec.html" class="struct"
title="struct zerovec::ZeroVec">ZeroVec</a>  
A zero-copy, byte-aligned vector for fixed-width types.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.make_ule.html" class="attr"
title="attr zerovec::make_ule">make_ule</a>  
Generate a corresponding
[`ULE`](ule/trait.ULE.html "trait zerovec::ule::ULE") type and the
relevant [`AsULE`](ule/trait.AsULE.html "trait zerovec::ule::AsULE")
implementations for this type

<a href="attr.make_varule.html" class="attr"
title="attr zerovec::make_varule">make_varule</a>  
Generate a corresponding
[`VarULE`](ule/trait.VarULE.html "trait zerovec::ule::VarULE") type and
the relevant
[`EncodeAsVarULE`](ule/trait.EncodeAsVarULE.html "trait zerovec::ule::EncodeAsVarULE")/[`zerofrom::ZeroFrom`](../zerofrom/zero_from/trait.ZeroFrom.html "trait zerofrom::zero_from::ZeroFrom")
implementations for this type
