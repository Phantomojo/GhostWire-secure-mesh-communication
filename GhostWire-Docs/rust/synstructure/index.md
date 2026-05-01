Crate synstructure Copy item path

<span class="sub-heading"><a href="../src/synstructure/lib.rs.html#1-2559" class="src">Source</a>
</span>

Expand description

This crate provides helper types for matching against enum variants, and
extracting bindings to each of the fields in the deriving Struct or Enum
in a generic way.

If you are writing a `#[derive]` which needs to perform some operation
on every field, then you have come to the right place!

# <a href="#example-walkfields" class="doc-anchor">§</a>Example: `WalkFields`

### <a href="#trait-implementation" class="doc-anchor">§</a>Trait Implementation

``` {.rust .rust-example-rendered}
pub trait WalkFields: std::any::Any {
    fn walk_fields(&self, walk: &mut FnMut(&WalkFields));
}
impl WalkFields for i32 {
    fn walk_fields(&self, _walk: &mut FnMut(&WalkFields)) {}
}
```

### <a href="#custom-derive" class="doc-anchor">§</a>Custom Derive

``` {.rust .rust-example-rendered}
fn walkfields_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| quote!{
        walk(#bi)
    });

    s.gen_impl(quote! {
        extern crate synstructure_test_traits;

        gen impl synstructure_test_traits::WalkFields for @Self {
            fn walk_fields(&self, walk: &mut FnMut(&synstructure_test_traits::WalkFields)) {
                match *self { #body }
            }
        }
    })
}
synstructure::decl_derive!([WalkFields] => walkfields_derive);

/*
 * Test Case
 */
fn main() {
    synstructure::test_derive! {
        walkfields_derive {
            enum A<T> {
                B(i32, T),
                C(i32),
            }
        }
        expands to {
            const _: () = {
                extern crate synstructure_test_traits;
                impl<T> synstructure_test_traits::WalkFields for A<T>
                    where T: synstructure_test_traits::WalkFields
                {
                    fn walk_fields(&self, walk: &mut FnMut(&synstructure_test_traits::WalkFields)) {
                        match *self {
                            A::B(ref __binding_0, ref __binding_1,) => {
                                { walk(__binding_0) }
                                { walk(__binding_1) }
                            }
                            A::C(ref __binding_0,) => {
                                { walk(__binding_0) }
                            }
                        }
                    }
                }
            };
        }
    }
}
```

# <a href="#example-interest" class="doc-anchor">§</a>Example: `Interest`

### <a href="#trait-implementation-1" class="doc-anchor">§</a>Trait Implementation

``` {.rust .rust-example-rendered}
pub trait Interest {
    fn interesting(&self) -> bool;
}
impl Interest for i32 {
    fn interesting(&self) -> bool { *self > 0 }
}
```

### <a href="#custom-derive-1" class="doc-anchor">§</a>Custom Derive

``` {.rust .rust-example-rendered}
fn interest_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.fold(false, |acc, bi| quote!{
        #acc || synstructure_test_traits::Interest::interesting(#bi)
    });

    s.gen_impl(quote! {
        extern crate synstructure_test_traits;
        gen impl synstructure_test_traits::Interest for @Self {
            fn interesting(&self) -> bool {
                match *self {
                    #body
                }
            }
        }
    })
}
synstructure::decl_derive!([Interest] => interest_derive);

/*
 * Test Case
 */
fn main() {
    synstructure::test_derive!{
        interest_derive {
            enum A<T> {
                B(i32, T),
                C(i32),
            }
        }
        expands to {
            const _: () = {
                extern crate synstructure_test_traits;
                impl<T> synstructure_test_traits::Interest for A<T>
                    where T: synstructure_test_traits::Interest
                {
                    fn interesting(&self) -> bool {
                        match *self {
                            A::B(ref __binding_0, ref __binding_1,) => {
                                false ||
                                    synstructure_test_traits::Interest::interesting(__binding_0) ||
                                    synstructure_test_traits::Interest::interesting(__binding_1)
                            }
                            A::C(ref __binding_0,) => {
                                false ||
                                    synstructure_test_traits::Interest::interesting(__binding_0)
                            }
                        }
                    }
                }
            };
        }
    }
}
```

For more example usage, consider investigating the `abomonation_derive`
crate, which makes use of this crate, and is fairly simple.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.decl_attribute.html" class="macro"
title="macro synstructure::decl_attribute">decl_attribute</a>  
The `decl_attribute!` macro declares a custom attribute wrapper. It will
parse the incoming `TokenStream` into a `synstructure::Structure`
object, and pass it into the inner function.

<a href="macro.decl_derive.html" class="macro"
title="macro synstructure::decl_derive">decl_derive</a>  
The `decl_derive!` macro declares a custom derive wrapper. It will parse
the incoming `TokenStream` into a `synstructure::Structure` object, and
pass it into the inner function.

<a href="macro.test_derive.html" class="macro"
title="macro synstructure::test_derive">test_derive</a>  
Run a test on a custom derive. This macro expands both the original
struct and the expansion to ensure that they compile correctly, and
confirms that feeding the original struct into the named derive will
produce the written output.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BindingInfo.html" class="struct"
title="struct synstructure::BindingInfo">BindingInfo</a>  
Information about a specific binding. This contains both an `Ident`
reference to the given field, and the syn `&'a Field` descriptor for
that field.

<a href="struct.Structure.html" class="struct"
title="struct synstructure::Structure">Structure</a>  
A wrapper around a `syn::DeriveInput` which provides utilities for
creating custom derive trait implementations.

<a href="struct.VariantAst.html" class="struct"
title="struct synstructure::VariantAst">VariantAst</a>  
This type is similar to `syn`’s `Variant` type, however each of the
fields are references rather than owned. When this is used as the AST
for a real variant, this struct simply borrows the fields of the
`syn::Variant`, however this type may also be used as the sole variant
for a struct.

<a href="struct.VariantInfo.html" class="struct"
title="struct synstructure::VariantInfo">VariantInfo</a>  
A wrapper around a `syn::DeriveInput`’s variant which provides utilities
for destructuring `Variant`s with `match` expressions.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AddBounds.html" class="enum"
title="enum synstructure::AddBounds">AddBounds</a>  
Changes how bounds are added

<a href="enum.BindStyle.html" class="enum"
title="enum synstructure::BindStyle">BindStyle</a>  
The type of binding to use when generating a pattern.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.MacroResult.html" class="trait"
title="trait synstructure::MacroResult">MacroResult</a>  
Helper trait describing values which may be returned by macro
implementation methods used by this crate’s macros.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.unpretty_print.html" class="fn"
title="fn synstructure::unpretty_print">unpretty_print</a>  
Dumps an unpretty version of a tokenstream. Takes any type which
implements `Display`.
