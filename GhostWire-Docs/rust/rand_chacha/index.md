Crate rand\_chacha Copy item path

<span class="sub-heading"><a href="../src/rand_chacha/lib.rs.html#9-106" class="src">Source</a>
</span>

Expand description

The ChaCha random number generators.

These are native Rust implementations of RNGs derived from the [ChaCha
stream ciphers](https://cr.yp.to/chacha.html) by D J Bernstein.

## <a href="#generators" class="doc-anchor">§</a>Generators

This crate provides 8-, 12- and 20-round variants of generators via a
“core” implementation (of
[`BlockRngCore`](../rand_core/block/trait.BlockRngCore.html "trait rand_core::block::BlockRngCore")),
each with an associated “RNG” type (implementing
[`RngCore`](../rand_core/trait.RngCore.html "trait rand_core::RngCore")).

These generators are all deterministic and portable (see
[Reproducibility](https://rust-random.github.io/book/crate-reprod.html)
in the book), with testing against reference vectors.

## <a href="#cryptographic-secure-usage" class="doc-anchor">§</a>Cryptographic (secure) usage

Where secure unpredictable generators are required, it is suggested to
use
[`ChaCha12Rng`](struct.ChaCha12Rng.html "struct rand_chacha::ChaCha12Rng")
or
[`ChaCha20Rng`](struct.ChaCha20Rng.html "struct rand_chacha::ChaCha20Rng")
and to seed via
[`SeedableRng::from_os_rng`](../rand_core/trait.SeedableRng.html#method.from_os_rng "associated function rand_core::SeedableRng::from_os_rng").

See also the
[Security](https://rust-random.github.io/book/guide-rngs.html#security)
chapter in the rand book. The crate is provided “as is”, without any
form of guarantee, and without a security audit.

## <a href="#seeding-construction" class="doc-anchor">§</a>Seeding (construction)

Generators implement the
[`SeedableRng`](../rand_core/trait.SeedableRng.html "trait rand_core::SeedableRng")
trait. Any method may be used, but note that `seed_from_u64` is not
suitable for usage where security is important. Some suggestions:

1.  With a fresh seed, **direct from the OS** (implies a syscall):
    ``` {.rust .rust-example-rendered}
    let rng = ChaCha12Rng::from_os_rng();
    ```
2.  **From a master generator.** This could be
    [`rand::rng`](https://docs.rs/rand/latest/rand/fn.rng.html)
    (effectively a fresh seed without the need for a syscall on each
    usage) or a deterministic generator such as
    [`ChaCha20Rng`](struct.ChaCha20Rng.html "struct rand_chacha::ChaCha20Rng").
    Beware that should a weak master generator be used, correlations may
    be detectable between the outputs of its child generators.
    <a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
    ``` {.rust .rust-example-rendered}
    let rng = ChaCha12Rng::from_rng(&mut rand::rng());
    ```

See also [Seeding
RNGs](https://rust-random.github.io/book/guide-seeding.html) in the
book.

## <a href="#generation" class="doc-anchor">§</a>Generation

Generators implement
[`RngCore`](../rand_core/trait.RngCore.html "trait rand_core::RngCore"),
whose methods may be used directly to generate unbounded integer or byte
values.

``` {.rust .rust-example-rendered}
use rand_core::{SeedableRng, RngCore};
use rand_chacha::ChaCha12Rng;

let mut rng = ChaCha12Rng::from_seed(Default::default());
let x = rng.next_u64();
assert_eq!(x, 0x53f955076a9af49b);
```

It is often more convenient to use the
[`rand::Rng`](https://docs.rs/rand/latest/rand/fn.rng.html) trait, which
provides further functionality. See also the [Random
Values](https://rust-random.github.io/book/guide-values.html) chapter in
the book.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../rand_core/index.html" class="mod"
title="mod rand_core"><code>rand_core</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ChaCha8Core.html" class="struct"
title="struct rand_chacha::ChaCha8Core">ChaCha8Core</a>  
ChaCha with 8 rounds

<a href="struct.ChaCha8Rng.html" class="struct"
title="struct rand_chacha::ChaCha8Rng">ChaCha8Rng</a>  
A cryptographically secure random number generator that uses the ChaCha
algorithm.

<a href="struct.ChaCha12Core.html" class="struct"
title="struct rand_chacha::ChaCha12Core">ChaCha12Core</a>  
ChaCha with 12 rounds

<a href="struct.ChaCha12Rng.html" class="struct"
title="struct rand_chacha::ChaCha12Rng">ChaCha12Rng</a>  
A cryptographically secure random number generator that uses the ChaCha
algorithm.

<a href="struct.ChaCha20Core.html" class="struct"
title="struct rand_chacha::ChaCha20Core">ChaCha20Core</a>  
ChaCha with 20 rounds

<a href="struct.ChaCha20Rng.html" class="struct"
title="struct rand_chacha::ChaCha20Rng">ChaCha20Rng</a>  
A cryptographically secure random number generator that uses the ChaCha
algorithm.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ChaChaCore.html" class="type"
title="type rand_chacha::ChaChaCore">ChaChaCore</a>  
ChaCha with 20 rounds, low-level interface

<a href="type.ChaChaRng.html" class="type"
title="type rand_chacha::ChaChaRng">ChaChaRng</a>  
ChaCha with 20 rounds
