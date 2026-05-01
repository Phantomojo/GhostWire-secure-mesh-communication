Crate foreign\_types Copy item path

<span class="sub-heading"><a href="../src/foreign_types/lib.rs.html#1-306" class="src">Source</a>
</span>

Expand description

A framework for Rust wrappers over C APIs.

Ownership is as important in C as it is in Rust, but the semantics are
often implicit. In particular, pointer-to-value is commonly used to pass
C values both when transferring ownership or a borrow.

This crate provides a framework to define a Rust wrapper over these
kinds of raw C APIs in a way that allows ownership semantics to be
expressed in an ergonomic manner. The framework takes a dual-type
approach similar to APIs in the standard library such as
`PathBuf`/`Path` or `String`/ `str`. One type represents an owned value
and references to the other represent borrowed values.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use foreign_types::{ForeignType, ForeignTypeRef, Opaque};
use std::ops::{Deref, DerefMut};

mod foo_sys {
    pub enum FOO {}

    extern {
        pub fn FOO_free(foo: *mut FOO);
    }
}

// The borrowed type is a newtype wrapper around an `Opaque` value.
//
// `FooRef` values never exist; we instead create references to `FooRef`s
// from raw C pointers.
pub struct FooRef(Opaque);

impl ForeignTypeRef for FooRef {
    type CType = foo_sys::FOO;
}

// The owned type is simply a newtype wrapper around the raw C type.
//
// It dereferences to `FooRef`, so methods that do not require ownership
// should be defined there.
pub struct Foo(*mut foo_sys::FOO);

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe { foo_sys::FOO_free(self.0) }
    }
}

impl ForeignType for Foo {
    type CType = foo_sys::FOO;
    type Ref = FooRef;

    unsafe fn from_ptr(ptr: *mut foo_sys::FOO) -> Foo {
        Foo(ptr)
    }

    fn as_ptr(&self) -> *mut foo_sys::FOO {
        self.0
    }
}

impl Deref for Foo {
    type Target = FooRef;

    fn deref(&self) -> &FooRef {
        unsafe { FooRef::from_ptr(self.0) }
    }
}

impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut FooRef {
        unsafe { FooRef::from_ptr_mut(self.0) }
    }
}
```

The `foreign_type!` macro can generate this boilerplate for you:

``` {.rust .rust-example-rendered}
#[macro_use]
extern crate foreign_types;

mod foo_sys {
    pub enum FOO {}

    extern {
        pub fn FOO_free(foo: *mut FOO);
        pub fn FOO_duplicate(foo: *mut FOO) -> *mut FOO; // Optional
    }
}

foreign_type! {
    type CType = foo_sys::FOO;
    fn drop = foo_sys::FOO_free;
    fn clone = foo_sys::FOO_duplicate; // Optional
    /// A Foo.
    pub struct Foo;
    /// A borrowed Foo.
    pub struct FooRef;
}
```

If `fn clone` is specified, then it must take `CType` as an argument and
return a copy of it as `CType`. It will be used to implement `ToOwned`
and `Clone`.

`#[derive(…)] is permitted before the lines with `pub
struct`. `\#\[doc(hidden)\]`before the`type
CType`line will hide the`foreign\_type!\` implementations from
documentation.

Say we then have a separate type in our C API that contains a `FOO`:

``` {.rust .rust-example-rendered}
mod foo_sys {
    pub enum FOO {}
    pub enum BAR {}

    extern {
        pub fn FOO_free(foo: *mut FOO);
        pub fn BAR_free(bar: *mut BAR);
        pub fn BAR_get_foo(bar: *mut BAR) -> *mut FOO;
    }
}
```

The documentation for the C library states that `BAR_get_foo` returns a
reference into the `BAR` passed to it, which translates into a reference
in Rust. It also says that we’re allowed to modify the `FOO`, so we’ll
define a pair of accessor methods, one immutable and one mutable:

``` {.rust .rust-example-rendered}
#[macro_use]
extern crate foreign_types;

use foreign_types::ForeignTypeRef;

mod foo_sys {
    pub enum FOO {}
    pub enum BAR {}

    extern {
        pub fn FOO_free(foo: *mut FOO);
        pub fn BAR_free(bar: *mut BAR);
        pub fn BAR_get_foo(bar: *mut BAR) -> *mut FOO;
    }
}

foreign_type! {
    #[doc(hidden)]
    type CType = foo_sys::FOO;
    fn drop = foo_sys::FOO_free;
    /// A Foo.
    pub struct Foo;
    /// A borrowed Foo.
    pub struct FooRef;
}

foreign_type! {
    type CType = foo_sys::BAR;
    fn drop = foo_sys::BAR_free;
    /// A Foo.
    pub struct Bar;
    /// A borrowed Bar.
    pub struct BarRef;
}

impl BarRef {
    fn foo(&self) -> &FooRef {
        unsafe { FooRef::from_ptr(foo_sys::BAR_get_foo(self.as_ptr())) }
    }

    fn foo_mut(&mut self) -> &mut FooRef {
        unsafe { FooRef::from_ptr_mut(foo_sys::BAR_get_foo(self.as_ptr())) }
    }
}
```

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.foreign_type.html" class="macro"
title="macro foreign_types::foreign_type">foreign_type</a>  
A macro to easily define wrappers for foreign types.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Opaque.html" class="struct"
title="struct foreign_types::Opaque">Opaque</a>  
An opaque type used to define `ForeignTypeRef` types.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ForeignType.html" class="trait"
title="trait foreign_types::ForeignType">ForeignType</a>  
A type implemented by wrappers over foreign types.

<a href="trait.ForeignTypeRef.html" class="trait"
title="trait foreign_types::ForeignTypeRef">ForeignTypeRef</a>  
A trait implemented by types which reference borrowed foreign types.
