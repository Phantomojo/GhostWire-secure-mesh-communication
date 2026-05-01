Crate typewit Copy item path

<span class="sub-heading"><a href="../src/typewit/lib.rs.html#1-674" class="src">Source</a>
</span>

Expand description

This crate provides abstractions for creating [type
witnesses](#what-are-type-witnesses).

The inciting motivation for this crate is emulating trait polymorphism
in `const fn` (as of 2025-07-20, it’s not possible to call trait methods
in const contexts on stable).

# <a href="#what-are-type-witnesses" class="doc-anchor">§</a>What are type witnesses

Type witnesses are values that prove the equality of a type parameter to
a fixed set of possible types.

The simplest type witness is
[`TypeEq<L, R>`](struct.TypeEq.html "struct typewit::TypeEq"), which
only proves equality of its `L` and `R` type parameters, and can be used
to coerce between them.

Most type witnesses are enums with
[`TypeEq`](struct.TypeEq.html "struct typewit::TypeEq") fields, which
can coerce between a type parameter and as many types as there are
variants.

# <a href="#examples" class="doc-anchor">§</a>Examples

<span id="example0"></span>

### <a href="#polymorphic-function" class="doc-anchor">§</a>Polymorphic function

This demonstrates how one can write a polymorphic `const fn` (as of
2025-07-20, trait methods can’t be called in const fns on stable)

(this example requires Rust 1.61.0, since it uses trait bounds in const)

``` {.rust .rust-example-rendered}
use typewit::{HasTypeWitness, TypeEq};
 
const VALS: [&str; 6] = [
    message(0),
    message(1),
    message(2),
    message(3),
    message("hi"),
    message("foo"),
];
assert_eq!(VALS, ["A", "B", "C", "A", "hi", "foo"]);
 
 
// A "method" of the `Message` trait (declared below)
const fn message<'a, T: Message<'a>>(val: T) -> &'a str {
    match HasTypeWitness::WITNESS {
        MessageWitness::Usize(te) => {
            // `te` (a `TypeEq<T, usize>`) allows coercing between `T` and `usize`,
            // because `TypeEq` is a value-level proof that both types are the same.
            let index: usize = te.to_right(val);
            ["A", "B", "C"][index % 3]
        }
        MessageWitness::Str(te) => {
            // `te` is a `TypeEq<T, &'a str>`
            te.to_right(val)
        }
    }
}
 
// The trait that we use to emulate polymorphic dispatch,
// the limitation is that it can only emulate it for a limited set of types known
// to the crate that defines the trait, in this case that's `usize` and `&str`.
trait Message<'a>: HasTypeWitness<MessageWitness<'a, Self>> { }
 
// replacing these impls with a blanket impl leads to worse compilation errors
impl<'a> Message<'a> for usize {}
impl<'a> Message<'a> for &'a str {}
 
// This macro declares `enum MessageWitness<'a, __Wit>`, a type witness enum,
// where each variant requires and then guarantees `__Wit` to be a particular type.
// (the `__Wit` type parameter is implicitly added after all generics)
typewit::simple_type_witness! {
    enum MessageWitness<'a> {
        // This variant requires `__Wit == usize`
        Usize = usize,
    
        // This variant requires `__Wit == &'a str`
        Str = &'a str,
    }
}
```

<span id="example-uses-type-fn"></span>

### <a href="#indexing-polymorphism" class="doc-anchor">§</a>Indexing polymorphism

This function demonstrates const fn polymorphism and projecting
[`TypeEq`](struct.TypeEq.html "struct typewit::TypeEq") by implementing
[`TypeFn`](type_fn/trait.TypeFn.html "trait typewit::type_fn::TypeFn").

(this example requires Rust 1.71.0, because it uses `<[T]>::split_at` in
a const context.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use std::ops::Range;
 
use typewit::{HasTypeWitness, TypeEq};
 
fn main() {
    let array = [3, 5, 8, 13, 21, 34, 55, 89];
 
    assert_eq!(index(&array, 0), &3);
    assert_eq!(index(&array, 3), &13);
    assert_eq!(index(&array, 0..4), [3, 5, 8, 13]);
    assert_eq!(index(&array, 3..5), [13, 21]);
}
 
const fn index<T, I>(slice: &[T], idx: I) -> &SliceIndexRet<I, T>
where
    I: SliceIndex<T>,
{
    // `I::WITNESS` is `<I as HasTypeWitness<IndexWitness<I>>>::WITNESS`,
    match I::WITNESS {
        IndexWitness::Usize(arg_te) => {
            // `arg_te` (a `TypeEq<I, usize>`) allows coercing between `I` and `usize`,
            // because `TypeEq` is a value-level proof that both types are the same.
            let idx: usize = arg_te.to_right(idx);
 
            // using the `TypeFn` impl for `FnSliceIndexRet<T>` to 
            // map `TypeEq<I, usize>` 
            // to  `TypeEq<SliceIndexRet<I, T>, SliceIndexRet<usize, T>>`
            arg_te.project::<FnSliceIndexRet<T>>()
                // converts`TypeEq<SliceIndexRet<I, T>, T>` 
                //      to `TypeEq<&SliceIndexRet<I, T>, &T>`
                .in_ref()
                .to_left(&slice[idx])
        }
        IndexWitness::Range(arg_te) => {
            let range: Range<usize> = arg_te.to_right(idx);
            let ret: &[T] = slice_range(slice, range);
            arg_te.project::<FnSliceIndexRet<T>>().in_ref().to_left(ret)
        }
    }
}
 
// This macro declares a type witness enum
typewit::simple_type_witness! {
    // Declares `enum IndexWitness<__Wit>` 
    // (the `__Wit` type parameter is implicitly added after all generics)
    enum IndexWitness {
        // This variant requires `__Wit == usize`
        Usize = usize,
    
        // This variant requires `__Wit == Range<usize>`
        Range = Range<usize>,
    }
}
 
/// Trait for all types that can be used as slice indices
/// 
/// The `HasTypeWitness` supertrait allows getting a `IndexWitness<Self>`
/// with its `WITNESS` associated constant.
trait SliceIndex<T>: HasTypeWitness<IndexWitness<Self>> + Sized {
    type Returns: ?Sized;
}
impl<T> SliceIndex<T> for usize {
    type Returns = T;
}
impl<T> SliceIndex<T> for Range<usize> {
    type Returns = [T];
}
 
type SliceIndexRet<I, T> = <I as SliceIndex<T>>::Returns;
 
// Declares `struct FnSliceIndexRet<T>`
// a type-level function (TypeFn implementor) from `I` to `SliceIndexRet<I, T>`
typewit::type_fn! {
    struct FnSliceIndexRet<T>;

    impl<I: SliceIndex<T>> I => SliceIndexRet<I, T>
}
 
const fn slice_range<T>(slice: &[T], range: Range<usize>) -> &[T] {
    let suffix = slice.split_at(range.start).1;
    suffix.split_at(range.end - range.start).0
}
 
```

When the wrong type is passed for the index, the compile-time error is
the same as with normal generic functions:

``` language-text
error[E0277]: the trait bound `RangeFull: SliceIndex<{integer}>` is not satisfied
  --> src/main.rs:43:30
   |
13 |     assert_eq!(index(&array, ..), [13, 21]);
   |                -----         ^^ the trait `SliceIndex<{integer}>` is not implemented for `RangeFull`
   |                |
   |                required by a bound introduced by this call
   |
   = help: the following other types implement trait `SliceIndex<T>`:
             std::ops::Range<usize>
             usize
```

### <a href="#downcasting-const-generic-type" class="doc-anchor">§</a>Downcasting const generic type

This example demonstrates “downcasting” from a type with a const
parameter to a concrete instance of that type.

``` {.rust .rust-example-rendered}
use typewit::{const_marker::Usize, TypeCmp, TypeEq};
 
assert_eq!(*mutate(&mut Arr([])), Arr([]));
assert_eq!(*mutate(&mut Arr([1])), Arr([1]));
assert_eq!(*mutate(&mut Arr([1, 2])), Arr([1, 2]));
assert_eq!(*mutate(&mut Arr([1, 2, 3])), Arr([1, 3, 6])); // this is different!
assert_eq!(*mutate(&mut Arr([1, 2, 3, 4])), Arr([1, 2, 3, 4])); 
 
#[derive(Debug, PartialEq)]
struct Arr<const N: usize>([u8; N]);
 
fn mutate<const N: usize>(arr: &mut Arr<N>) -> &mut Arr<N> {
    if let TypeCmp::Eq(te) =  Usize::<N>.equals(Usize::<3>) {
        let tem = te // `te` is a `TypeEq<Usize<N>, Usize<3>>`
            .project::<GArr>() // returns `TypeEq<Arr<N>, Arr<3>>`
            .in_mut(); // returns `TypeEq<&mut Arr<N>, &mut Arr<3>>`
 
        // `tem.to_right(arr)` downcasts `arr` to `&mut Arr<3>`
        tetra_sum(tem.to_right(arr));
    }
 
    arr
}
 
fn tetra_sum(arr: &mut Arr<3>) {
    arr.0[1] += arr.0[0];
    arr.0[2] += arr.0[1];
}
 
// Declares `struct GArr`
// a type-level function (TypeFn implementor) from `Usize<N>` to `Arr<N>`
typewit::type_fn!{
    struct GArr;
 
    impl<const N: usize> Usize<N> => Arr<N>
}
```

### <a href="#builder" class="doc-anchor">§</a>Builder

Using a type witness to help encode a type-level enum, and to match on
that type-level enum inside of a function.

The type-level enum is used to track the initialization of fields in a
builder.

This example requires Rust 1.65.0, because it uses Generic Associated
Types.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use typewit::HasTypeWitness;
 
fn main() {
    // all default fields
    assert_eq!(
        StructBuilder::new().build(), 
        Struct{foo: "default value".into(), bar: vec![3, 5, 8]},
    );
 
    // defaulted bar field
    assert_eq!(
        StructBuilder::new().foo("hello").build(), 
        Struct{foo: "hello".into(), bar: vec![3, 5, 8]},
    );
 
    // defaulted foo field
    assert_eq!(
        StructBuilder::new().bar([13, 21, 34]).build(), 
        Struct{foo: "default value".into(), bar: vec![13, 21, 34]},
    );
 
    // all initialized fields
    assert_eq!(
        StructBuilder::new().foo("world").bar([55, 89]).build(), 
        Struct{foo: "world".into(), bar: vec![55, 89]},
    );
}
 
 
#[derive(Debug, PartialEq, Eq)]
struct Struct {
    foo: String,
    bar: Vec<u32>,
}
 
struct StructBuilder<FooInit: InitState, BarInit: InitState> {
    // If `FooInit` is `Uninit`, then this field is a `()`
    // If `FooInit` is `Init`, then this field is a `String`
    foo: BuilderField<FooInit, String>,

    // If `BarInit` is `Uninit`, then this field is a `()`
    // If `BarInit` is `Init`, then this field is a `Vec<u32>`
    bar: BuilderField<BarInit, Vec<u32>>,
}
 
impl StructBuilder<Uninit, Uninit> {
    pub const fn new() -> Self {
        Self {
            foo: (),
            bar: (),
        }
    }
}
 
impl<FooInit: InitState, BarInit: InitState> StructBuilder<FooInit, BarInit> {
    /// Sets the `foo` field
    pub fn foo(self, foo: impl Into<String>) -> StructBuilder<Init, BarInit> {
        StructBuilder {
            foo: foo.into(),
            bar: self.bar,
        }
    }

    /// Sets the `bar` field
    pub fn bar(self, bar: impl Into<Vec<u32>>) -> StructBuilder<FooInit, Init> {
        StructBuilder {
            foo: self.foo,
            bar: bar.into(),
        }
    }
 
    /// Builds `Struct`, 
    /// providing default values for fields that haven't been set.
    pub fn build(self) -> Struct {
        Struct {
            foo: init_or_else::<FooInit, _, _>(self.foo, || "default value".to_string()),
            bar: init_or_else::<BarInit, _, _>(self.bar, || vec![3, 5, 8]),
        }
    }
}
 
// Emulates a type-level `enum InitState { Init, Uninit }`
trait InitState: Sized + HasTypeWitness<InitWit<Self>> {
    // How a builder represents an initialized/uninitialized field.
    // If `Self` is `Uninit`, then this is `()`.
    // If `Self` is `Init`, then this is `T`.
    type BuilderField<T>;
}
 
// If `I` is `Uninit`, then this evaluates to `()`
// If `I` is `Init`, then this evaluates to `T`
type BuilderField<I, T> = <I as InitState>::BuilderField::<T>;
 
/// Gets `T` out of `maybe_init` if it's actually initialized,
/// otherwise returns `else_()`.
fn init_or_else<I, T, F>(maybe_init: BuilderField<I, T>, else_: F) -> T
where
    I: InitState,
    F: FnOnce() -> T
{
    typewit::type_fn! {
        // Declares the `HelperFn` type-level function (TypeFn implementor)
        // from `I` to `BuilderField<I, T>`
        struct HelperFn<T>;
        impl<I: InitState> I => BuilderField<I, T>
    }
 
    // matching on the type-level `InitState` enum by using `InitWit`.
    // `WITNESS` comes from the `HasTypeWitness` trait
    match I::WITNESS {
        // `te: TypeEq<FooInit, Init>`
        InitWit::InitW(te) => {
            te.map(HelperFn::NEW) //: TypeEq<BuilderField<I, T>, T>
              .to_right(maybe_init)
        }
        InitWit::UninitW(_) => else_(),
    }
}
 
// Emulates a type-level `InitState::Init` variant.
// Marks a field as initialized.
enum Init {}
 
impl InitState for Init {
    type BuilderField<T> = T;
}
 
// Emulates a type-level `InitState::Uninit` variant.
// Marks a field as uninitialized.
enum Uninit {}
 
impl InitState for Uninit {
    type BuilderField<T> = ();
}
 
typewit::simple_type_witness! {
    // Declares `enum InitWit<__Wit>`, a type witness.
    // (the `__Wit` type parameter is implicitly added after all generics)
    enum InitWit {
        // This variant requires `__Wit == Init`
        InitW = Init,
        // This variant requires `__Wit == Uninit`
        UninitW = Uninit,
    }
}
```

### <a href="#generic-const-expressions" class="doc-anchor">§</a>Generic Const Expressions

This example uses
[`Usize`](const_marker/struct.Usize.html "struct typewit::const_marker::Usize")
to coerce an arrays whose length is generic to another generic, but
equal, length.

This example requires the `"generic_const_exprs"` crate feature because
it uses the currently-unstable
[`generic_const_exprs`](https://doc.rust-lang.org/unstable-book/language-features/generic-const-exprs.html)
language feature.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
#![feature(generic_const_exprs)]
 
use typewit::{const_marker::Usize, TypeCmp, TypeEq};
 
 
let mut arrays = Arrays::<1, 3> { a: [3, 5, 8], b: [13, 21, 34] };
 
arrays.swap_inner();
 
assert_eq!(arrays.a, [13, 21, 34]);
assert_eq!(arrays.b, [3, 5, 8]);
 
 
struct Arrays<const A: usize, const B: usize> 
where
    [u8; A * B]:, 
    [u8; B * A]:,
{
    a: [u8; A * B],
    b: [u8; B * A],
}
 
impl<const A: usize, const B: usize> Arrays<A, B> 
where
    [u8; A * B]:, 
    [u8; B * A]:,
{
    // Swaps the two array fields
    const fn swap_inner(&mut self) {
        let a = TypeEq::new::<u8>() // : TypeEq<u8, u8>
            .in_array(commutative_proof::<A, B>()) // : TypeEq<[u8; A * B], [u8; B * A]>
            .in_mut() // : TypeEq<&mut [u8; A * B], &mut [u8; B * A]>
            .to_right(
                &mut self.a // : &mut [u8; A * B] 
            ); // : &mut [u8; B * A] 
         
        core::mem::swap(a, &mut self.b);
    }
}
 
const fn commutative_proof<const A: usize, const B: usize>(
) -> TypeEq<Usize<{A * B}>, Usize<{B * A}>>
{
    // panic-safety: A * B == B * A always holds, so this `unwrap_eq` can never panic
    Usize::<{A * B}>.equals(Usize::<{B * A}>).unwrap_eq()
}
 
```

If you tried to swap the fields directly, you’d get this error:

``` language-text
error[E0308]: mismatched types
  --> src/lib.rs:437:38
   |
42 |         core::mem::swap(&mut self.a, &mut self.b);
   |                                      ^^^^^^^^^^^ expected `A * B`, found `B * A`
   |
   = note: expected constant `A * B`
              found constant `B * A`
```

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo features

These are the features of this crate.

### <a href="#default-features" class="doc-anchor">§</a>Default-features

These features are enabled by default:

-   `"proc_macros"`: uses proc macros to improve compile-errors
    involving macro-generated impls.

### <a href="#rust-versions-and-standard-crates" class="doc-anchor">§</a>Rust-versions and standard crates

These features enable items that have a minimum Rust version:

-   `"rust_stable"`: enables all the `"rust_1_*"` features.

-   `"rust_1_83"`: turns functions that take mutable references into
    `const fn`s, enables
    [`const_marker`](const_marker/index.html "mod typewit::const_marker")
    items for comparing
    [`ConstMarker`](const_marker/trait.ConstMarker.html "trait typewit::const_marker::ConstMarker")s,
    and enables the `"rust_1_65"` feature.

-   `"rust_1_65"`: enables the
    [`type_constructors`](crate::type_constructors) module, the
    [`methods`](crate::methods) module, and the `"rust_1_61"` feature.

-   `"rust_1_61"`: enables
    [`MetaBaseTypeWit`](enum.MetaBaseTypeWit.html "enum typewit::MetaBaseTypeWit"),
    [`BaseTypeWitness`](trait.BaseTypeWitness.html "trait typewit::BaseTypeWitness"),
    and the `{TypeCmp, TypeNe}::{zip*, in_array}` methods.

These features enable items that require a non-`core` standard crate:

-   `"alloc"`: enable items that use anything from the standard `alloc`
    crate.

### <a href="#nightly-features" class="doc-anchor">§</a>Nightly features

These features require the nightly Rust compiler:

-   `"adt_const_marker"`: enables the `"rust_stable"` crate feature, and
    marker types in the
    [`const_marker`](const_marker/index.html "mod typewit::const_marker")
    module that have non-primitive `const` parameters.

-   `"generic_const_exprs"`: enables the `"rust_stable"` crate feature,
    and doc examples that use the
    [`generic_const_exprs`](https://doc.rust-lang.org/unstable-book/language-features/generic-const-exprs.html)
    unstable language feature.

# <a href="#no-std-support" class="doc-anchor">§</a>No-std support

`typewit` is `#![no_std]`, it can be used anywhere Rust can be used.

You need to enable the `"alloc"` feature to enable items that use
anything from the standard `alloc` crate.

# <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

`typewit` supports Rust 1.57.0.

Features that require newer versions of Rust, or the nightly compiler,
need to be explicitly enabled with crate features.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::type_fn::`<a href="type_fn/type.CallFn.html" class="type"
title="type typewit::type_fn::CallFn"><code>CallFn</code></a>`;`

`pub use crate::type_fn::`<a href="type_fn/type.CallInjFn.html" class="type"
title="type typewit::type_fn::CallInjFn"><code>CallInjFn</code></a>`;`

`pub use crate::type_fn::`<a href="type_fn/trait.InjTypeFn.html" class="trait"
title="trait typewit::type_fn::InjTypeFn"><code>InjTypeFn</code></a>`;`

`pub use crate::type_fn::`<a href="type_fn/trait.RevTypeFn.html" class="trait"
title="trait typewit::type_fn::RevTypeFn"><code>RevTypeFn</code></a>`;`

`pub use crate::type_fn::`<a href="type_fn/trait.TypeFn.html" class="trait"
title="trait typewit::type_fn::TypeFn"><code>TypeFn</code></a>`;`

`pub use crate::type_fn::`<a href="type_fn/type.UncallFn.html" class="type"
title="type typewit::type_fn::UncallFn"><code>UncallFn</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="const_marker/index.html" class="mod"
title="mod typewit::const_marker">const_marker</a>  
Marker types for passing constants as type arguments.

<a href="type_fn/index.html" class="mod"
title="mod typewit::type_fn">type_fn</a>  
Type-level functions.

<a href="type_ne/index.html" class="mod"
title="mod typewit::type_ne">type_ne</a>  
[`TypeNe`](struct.TypeNe.html "struct typewit::TypeNe")-related items

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.inj_type_fn.html" class="macro"
title="macro typewit::inj_type_fn">inj_type_fn</a>  
Declares an [injective type-level
function](type_fn/trait.InjTypeFn.html "trait typewit::type_fn::InjTypeFn")

<a href="macro.polymatch.html" class="macro"
title="macro typewit::polymatch">polymatch</a>  
Match which expands top-level `|` patterns to multiple match arms.

<a href="macro.simple_type_witness.html" class="macro"
title="macro typewit::simple_type_witness">simple_type_witness</a>  
Declares a [type
witness](index.html#what-are-type-witnesses "mod typewit") enum.

<a href="macro.type_fn.html" class="macro"
title="macro typewit::type_fn">type_fn</a>  
Declares a type-level function (struct that implements
[`TypeFn`](type_fn/trait.TypeFn.html "trait typewit::type_fn::TypeFn"))

<a href="macro.type_ne.html" class="macro"
title="macro typewit::type_ne">type_ne</a>  
Constructs a [`TypeNe`](struct.TypeNe.html "struct typewit::TypeNe") of
types that are statically known to be different.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.TypeEq.html" class="struct"
title="struct typewit::TypeEq">TypeEq</a>  
Value-level proof that `L` is the same type as `R`

<a href="struct.TypeNe.html" class="struct"
title="struct typewit::TypeNe">TypeNe</a>  
Value-level proof that `L` is a different type to `R`

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.MetaBaseTypeWit.html" class="enum"
title="enum typewit::MetaBaseTypeWit">MetaBaseTypeWit</a>  
Type witness for
[`TypeEq`](struct.TypeEq.html "struct typewit::TypeEq")/[`TypeNe`](struct.TypeNe.html "struct typewit::TypeNe")/[`TypeCmp`](enum.TypeCmp.html "enum typewit::TypeCmp").

<a href="enum.TypeCmp.html" class="enum"
title="enum typewit::TypeCmp">TypeCmp</a>  
A witness of whether its `L` and `R` type parameters are the same or
different types.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BaseTypeWitness.html" class="trait"
title="trait typewit::BaseTypeWitness">BaseTypeWitness</a>  
Marker trait for
[`TypeCmp`](enum.TypeCmp.html "enum typewit::TypeCmp")/[`TypeEq`](struct.TypeEq.html "struct typewit::TypeEq")/[`TypeNe`](struct.TypeNe.html "struct typewit::TypeNe").

<a href="trait.HasTypeWitness.html" class="trait"
title="trait typewit::HasTypeWitness">HasTypeWitness</a>  
Gets a [type witness](index.html#what-are-type-witnesses "mod typewit")
for `Self`.

<a href="trait.Identity.html" class="trait"
title="trait typewit::Identity">Identity</a>  
Emulation of `T == U` bounds.

<a href="trait.MakeTypeWitness.html" class="trait"
title="trait typewit::MakeTypeWitness">MakeTypeWitness</a>  
Constructs this [type
witness](index.html#what-are-type-witnesses "mod typewit").

<a href="trait.TypeWitnessTypeArg.html" class="trait"
title="trait typewit::TypeWitnessTypeArg">TypeWitnessTypeArg</a>  
Gets the type argument that this [type
witness](index.html#what-are-type-witnesses "mod typewit") witnesses.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.type_eq.html" class="fn"
title="fn typewit::type_eq">type_eq</a>  
Constructs a
[`TypeEq<T, T>`](struct.TypeEq.html "struct typewit::TypeEq")
