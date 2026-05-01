Crate hkdf Copy item path

<span class="sub-heading"><a href="../src/hkdf/lib.rs.html#1-290" class="src">Source</a>
</span>

Expand description

An implementation of HKDF, the [HMAC-based Extract-and-Expand Key
Derivation Function](https://tools.ietf.org/html/rfc5869).

# <a href="#usage" class="doc-anchor">§</a>Usage

The most common way to use HKDF is as follows: you provide the Initial
Key Material (IKM) and an optional salt, then you expand it (perhaps
multiple times) into some Output Key Material (OKM) bound to an “info”
context string.

There are two usage options for the salt:

-   [`None`](https://doc.rust-lang.org/1.93.1/core/option/enum.Option.html#variant.None "variant core::option::Option::None")
    or static for domain separation in a private setting
-   guaranteed to be uniformly-distributed and unique in a public
    setting

Other non fitting data should be added to the `IKM` or `info`.

``` {.rust .rust-example-rendered}
use sha2::Sha256;
use hkdf::Hkdf;
use hex_literal::hex;

let ikm = hex!("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");
let salt = hex!("000102030405060708090a0b0c");
let info = hex!("f0f1f2f3f4f5f6f7f8f9");

let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &ikm);
let mut okm = [0u8; 42];
hk.expand(&info, &mut okm)
    .expect("42 is a valid length for Sha256 to output");

let expected = hex!("
    3cb25f25faacd57a90434f64d0362f2a
    2d2d0a90cf1a5a4c5db02d56ecc4c5bf
    34007208d5b887185865
");
assert_eq!(okm[..], expected[..]);
```

Normally the PRK (Pseudo-Random Key) remains hidden within the HKDF
object, but if you need to access it, use
[`Hkdf::extract`](struct.Hkdf.html#method.extract "associated function hkdf::Hkdf::extract")
instead of
[`Hkdf::new`](struct.Hkdf.html#method.new "associated function hkdf::Hkdf::new").

``` {.rust .rust-example-rendered}
let (prk, hk) = Hkdf::<Sha256>::extract(Some(&salt[..]), &ikm);
let expected = hex!("
    077709362c2e32df0ddc3f0dc47bba63
    90b6c73bb50f9c3122ec844ad7c2b3e5
");
assert_eq!(prk[..], expected[..]);
```

If you already have a strong key to work from (uniformly-distributed and
long enough), you can save a tiny amount of time by skipping the extract
step. In this case, you pass a Pseudo-Random Key (PRK) into the
[`Hkdf::from_prk`](struct.Hkdf.html#method.from_prk "associated function hkdf::Hkdf::from_prk")
constructor, then use the resulting
[`Hkdf`](struct.Hkdf.html "struct hkdf::Hkdf") object as usual.

``` {.rust .rust-example-rendered}
let prk = hex!("
    077709362c2e32df0ddc3f0dc47bba63
    90b6c73bb50f9c3122ec844ad7c2b3e5
");

let hk = Hkdf::<Sha256>::from_prk(&prk).expect("PRK should be large enough");
let mut okm = [0u8; 42];
hk.expand(&info, &mut okm)
    .expect("42 is a valid length for Sha256 to output");

let expected = hex!("
    3cb25f25faacd57a90434f64d0362f2a
    2d2d0a90cf1a5a4c5db02d56ecc4c5bf
    34007208d5b887185865
");
assert_eq!(okm[..], expected[..]);
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../hmac/index.html" class="mod"
title="mod hmac"><code>hmac</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Hkdf.html" class="struct"
title="struct hkdf::Hkdf">Hkdf</a>  
Structure representing the HKDF, capable of HKDF-Expand and HKDF-Extract
operations. Recommendations for the correct usage of the parameters can
be found in the [crate root](index.html#usage).

<a href="struct.HkdfExtract.html" class="struct"
title="struct hkdf::HkdfExtract">HkdfExtract</a>  
Structure representing the streaming context of an HKDF-Extract
operation

<a href="struct.InvalidLength.html" class="struct"
title="struct hkdf::InvalidLength">InvalidLength</a>  
Structure for InvalidLength, used for output error handling.

<a href="struct.InvalidPrkLength.html" class="struct"
title="struct hkdf::InvalidPrkLength">InvalidPrkLength</a>  
Error that is returned when supplied pseudorandom key (PRK) is not long
enough.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.HmacImpl.html" class="trait"
title="trait hkdf::HmacImpl">HmacImpl</a>  
Sealed trait implemented for
[`Hmac`](../hmac/optim/type.Hmac.html "type hmac::optim::Hmac") and
[`SimpleHmac`](../hmac/simple/struct.SimpleHmac.html "struct hmac::simple::SimpleHmac").

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.SimpleHkdf.html" class="type"
title="type hkdf::SimpleHkdf">SimpleHkdf</a>  
[`Hkdf`](struct.Hkdf.html "struct hkdf::Hkdf") variant which uses
[`SimpleHmac`](../hmac/simple/struct.SimpleHmac.html "struct hmac::simple::SimpleHmac")
for underlying HMAC implementation.

<a href="type.SimpleHkdfExtract.html" class="type"
title="type hkdf::SimpleHkdfExtract">SimpleHkdfExtract</a>  
[`HkdfExtract`](struct.HkdfExtract.html "struct hkdf::HkdfExtract")
variant which uses
[`SimpleHmac`](../hmac/simple/struct.SimpleHmac.html "struct hmac::simple::SimpleHmac")
for underlying HMAC implementation.
