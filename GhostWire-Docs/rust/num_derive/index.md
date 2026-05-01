Crate num\_derive Copy item path

<span class="sub-heading"><a href="../src/num_derive/lib.rs.html#11-1004" class="src">Source</a>
</span>

Expand description

Procedural macros to derive numeric traits in Rust.

## <a href="#usage" class="doc-anchor">§</a>Usage

Add this to your `Cargo.toml`:

``` language-toml
[dependencies]
num-traits = "0.2"
num-derive = "0.3"
```

Then you can derive traits on your own types:

``` {.rust .rust-example-rendered}
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive, ToPrimitive)]
enum Color {
    Red,
    Blue,
    Green,
}
```

## <a href="#explicit-import" class="doc-anchor">§</a>Explicit import

By default the `num_derive` procedural macros assume that the
`num_traits` crate is a direct dependency. If `num_traits` is instead a
transitive dependency, the `num_traits` helper attribute can be used to
tell `num_derive` to use a specific identifier for its imports.

``` {.rust .rust-example-rendered}
#[macro_use]
extern crate num_derive;
// Lets pretend this is a transitive dependency from another crate
// reexported as `some_other_ident`.
extern crate num_traits as some_other_ident;

#[derive(FromPrimitive, ToPrimitive)]
#[num_traits = "some_other_ident"]
enum Color {
    Red,
    Blue,
    Green,
}
```

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Float.html" class="derive"
title="derive num_derive::Float">Float</a>  
Derives
[`num_traits::Float`](https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html)
for newtypes. The inner type must already implement `Float`.

<a href="derive.FromPrimitive.html" class="derive"
title="derive num_derive::FromPrimitive">FromPrimitive</a>  
Derives
[`num_traits::FromPrimitive`](https://docs.rs/num-traits/0.2/num_traits/cast/trait.FromPrimitive.html)
for simple enums and newtypes.

<a href="derive.Num.html" class="derive"
title="derive num_derive::Num">Num</a>  
Derives
[`num_traits::Num`](https://docs.rs/num-traits/0.2/num_traits/trait.Num.html)
for newtypes. The inner type must already implement `Num`.

<a href="derive.NumCast.html" class="derive"
title="derive num_derive::NumCast">NumCast</a>  
Derives
[`num_traits::NumCast`](https://docs.rs/num-traits/0.2/num_traits/cast/trait.NumCast.html)
for newtypes. The inner type must already implement `NumCast`.

<a href="derive.NumOps.html" class="derive"
title="derive num_derive::NumOps">NumOps</a>  
Derives
[`num_traits::NumOps`](https://docs.rs/num-traits/0.2/num_traits/trait.NumOps.html)
for newtypes. The inner type must already implement `NumOps`.

<a href="derive.One.html" class="derive"
title="derive num_derive::One">One</a>  
Derives
[`num_traits::One`](https://docs.rs/num-traits/0.2/num_traits/identities/trait.One.html)
for newtypes. The inner type must already implement `One`.

<a href="derive.Signed.html" class="derive"
title="derive num_derive::Signed">Signed</a>  
Derives
[`num_traits::Signed`](https://docs.rs/num-traits/0.2/num_traits/sign/trait.Signed.html)
for newtypes. The inner type must already implement `Signed`.

<a href="derive.ToPrimitive.html" class="derive"
title="derive num_derive::ToPrimitive">ToPrimitive</a>  
Derives
[`num_traits::ToPrimitive`](https://docs.rs/num-traits/0.2/num_traits/cast/trait.ToPrimitive.html)
for simple enums and newtypes.

<a href="derive.Unsigned.html" class="derive"
title="derive num_derive::Unsigned">Unsigned</a>  
Derives
[`num_traits::Unsigned`](https://docs.rs/num/latest/num/traits/trait.Unsigned.html).
The inner type must already implement `Unsigned`.

<a href="derive.Zero.html" class="derive"
title="derive num_derive::Zero">Zero</a>  
Derives
[`num_traits::Zero`](https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html)
for newtypes. The inner type must already implement `Zero`.
