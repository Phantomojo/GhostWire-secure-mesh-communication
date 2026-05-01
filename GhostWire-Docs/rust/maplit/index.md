Crate maplit Copy item path

<span class="sub-heading"><a href="../src/maplit/lib.rs.html#1-324" class="src">Source</a>
</span>

Expand description

Macros for container literals with specific type.

``` {.rust .rust-example-rendered}
#[macro_use] extern crate maplit;

let map = hashmap!{
    "a" => 1,
    "b" => 2,
};
```

The **maplit** crate uses `=>` syntax to separate the key and value for
the mapping macros. (It was not possible to use `:` as separator due to
syntactic restrictions in regular `macro_rules!` macros.)

Note that rust macros are flexible in which brackets you use for the
invocation. You can use them as `hashmap!{}` or `hashmap![]` or
`hashmap!()`.

Generic container macros already exist elsewhere, so those are not
provided here at the moment.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.btreemap.html" class="macro"
title="macro maplit::btreemap">btreemap</a>  
Create a **BTreeMap** from a list of key-value pairs

<a href="macro.btreeset.html" class="macro"
title="macro maplit::btreeset">btreeset</a>  
Create a **BTreeSet** from a list of elements.

<a href="macro.convert_args.html" class="macro"
title="macro maplit::convert_args">convert_args</a>  
Macro that converts the keys or key-value pairs passed to another maplit
macro. The default conversion is to use the
[`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html) trait,
if no custom conversion is passed.

<a href="macro.hashmap.html" class="macro"
title="macro maplit::hashmap">hashmap</a>  
Create a **HashMap** from a list of key-value pairs

<a href="macro.hashset.html" class="macro"
title="macro maplit::hashset">hashset</a>  
Create a **HashSet** from a list of elements.
