Crate enum\_as\_inner Copy item path

<span class="sub-heading"><a href="../src/enum_as_inner/lib.rs.html#8-398" class="src">Source</a>
</span>

Expand description

# <a href="#enum-as-inner" class="doc-anchor">§</a>enum-as-inner

A deriving proc-macro for generating functions to automatically give
access to the inner members of enum.

## <a href="#basic-unnamed-field-case" class="doc-anchor">§</a>Basic unnamed field case

The basic case is meant for single item enums, like:

``` {.rust .rust-example-rendered}
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
enum OneEnum {
    One(u32),
}

let one = OneEnum::One(1);

assert_eq!(*one.as_one().unwrap(), 1);
assert_eq!(one.into_one().unwrap(), 1);
```

where the result is either a reference for inner items or a tuple
containing the inner items.

## <a href="#unit-case" class="doc-anchor">§</a>Unit case

This will return true if enum’s variant matches the expected type

``` {.rust .rust-example-rendered}
use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
enum UnitVariants {
    Zero,
    One,
    Two,
}

let unit = UnitVariants::Two;

assert!(unit.is_two());
```

## <a href="#mutliple-unnamed-field-case" class="doc-anchor">§</a>Mutliple, unnamed field case

This will return a tuple of the inner types:

``` {.rust .rust-example-rendered}
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
enum ManyVariants {
    One(u32),
    Two(u32, i32),
    Three(bool, u32, i64),
}

let many = ManyVariants::Three(true, 1, 2);

assert!(many.is_three());
assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
assert_eq!(many.into_three().unwrap(), (true, 1_u32, 2_i64));
```

## <a href="#multiple-named-field-case" class="doc-anchor">§</a>Multiple, named field case

This will return a tuple of the inner types, like the unnamed option:

``` {.rust .rust-example-rendered}
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
enum ManyVariants {
    One { one: u32 },
    Two { one: u32, two: i32 },
    Three { one: bool, two: u32, three: i64 },
}

let many = ManyVariants::Three { one: true, two: 1, three: 2 };

assert!(many.is_three());
assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
assert_eq!(many.into_three().unwrap(), (true, 1_u32, 2_i64));
```

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.EnumAsInner.html" class="derive"
title="derive enum_as_inner::EnumAsInner">EnumAsInner</a>  
Derive functions on an Enum for easily accessing individual items in the
Enum
