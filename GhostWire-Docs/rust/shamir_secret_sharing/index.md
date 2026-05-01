Crate shamir\_secret\_sharing Copy item path

<span class="sub-heading"><a href="../src/shamir_secret_sharing/lib.rs.html#1-219"
class="src">Source</a> </span>

Expand description

A rust implementation of Shamir Secret Sharing over Finite Field.

The lib support large field charactirics `prime` by taking advantage of
`num_bigint`. It’s not optimized for production purpose, which can be
improved in several aspects:

1.  replace the `extended_euclid_algo` with machine-friendly
    `stein_algo` to calculate the modulo inverse;
2.  add commitment scheme to make it verifiable

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use shamir_secret_sharing::ShamirSecretSharing as SSS;
use num_bigint::{BigInt, BigUint};
use num_bigint::Sign::*;
let sss = SSS {
    threshold: 3,
    share_amount: 5,
    prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap()
    };

let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();

let shares = sss.split(secret.clone());

println!("shares: {:?}", shares);
assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../num_bigint/index.html" class="mod"
title="mod num_bigint"><code>num_bigint</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ShamirSecretSharing.html" class="struct"
title="struct shamir_secret_sharing::ShamirSecretSharing">ShamirSecretSharing</a>
