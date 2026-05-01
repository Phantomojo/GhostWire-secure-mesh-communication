Crate assign Copy item path

<span class="sub-heading"><a href="../src/assign/lib.rs.html#1-211" class="src">Source</a>
</span>

Expand description

This module provides the `assign!` macro to allow mutating a struct
value in a declarative style.

It is an alternative to [struct update
syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)
that works with
[non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute)
structs.

It is used as

``` {.rust .rust-example-rendered}
let foo = assign!(init_expression, {
    field: new_field_value,
    // other_field: new_other_field_value,
    // ...
});
```

For details and examples, see the documentation for the macro itself.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.assign.html" class="macro"
title="macro assign::assign">assign</a>  
Mutate a struct value in a declarative style.
