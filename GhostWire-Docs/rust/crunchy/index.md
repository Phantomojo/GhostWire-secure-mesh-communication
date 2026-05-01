Crate crunchy Copy item path

<span class="sub-heading"><a href="../src/crunchy/lib.rs.html#1-36" class="src">Source</a>
</span>

Expand description

The crunchy unroller - deterministically unroll constant loops. For
number “crunching”.

The Rust optimizer will unroll constant loops that don’t use the loop
variable, like this:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
for _ in 0..100 {
    println!("Hello!");
}
```

However, using the loop variable will cause it to never unroll the loop.
This is unfortunate because it means that you can’t constant-fold the
loop variable, and if you end up stomping on the registers it will have
to do a load for each iteration. This crate ensures that your code is
unrolled and const-folded. It only works on literals, unfortunately, but
there’s a work-around:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
debug_assert_eq!(MY_CONSTANT, 100);
unroll! {
    for i in 0..100 {
        println!("Iteration {}", i);
    }
}
```

This means that your tests will catch if you redefine the constant.

To default maximum number of loops to unroll is `64`, but that can be
easily increased using the cargo features:

-   `limit_128`
-   `limit_256`
-   `limit_512`
-   `limit_1024`
-   `limit_2048`

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.unroll.html" class="macro"
title="macro crunchy::unroll">unroll</a>  
Unroll the given for loop
