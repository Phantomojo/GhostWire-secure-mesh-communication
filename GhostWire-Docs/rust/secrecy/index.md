Crate secrecy Copy item path

<span class="sub-heading"><a href="../src/secrecy/lib.rs.html#1-285" class="src">Source</a>
</span>

Expand description

[`Secret`](struct.Secret.html "struct secrecy::Secret") wrapper type for
more carefully handling secret values (e.g. passwords, cryptographic
keys, access tokens or other credentials)

# <a href="#goals" class="doc-anchor">§</a>Goals

-   Make secret access explicit and easy-to-audit via the
    [`ExposeSecret`](trait.ExposeSecret.html "trait secrecy::ExposeSecret")
    trait. This also makes secret values immutable which helps avoid
    making accidental copies (e.g. reallocating the backing buffer for a
    `Vec`)
-   Prevent accidental leakage of secrets via channels like debug
    logging
-   Ensure secrets are wiped from memory on drop securely (using the
    [`zeroize`](../zeroize/index.html "mod zeroize") crate)

Presently this crate favors a simple, `no_std`-friendly, safe i.e.
`forbid(unsafe_code)`-based implementation and does not provide more
advanced memory protection mechanisms e.g. ones based on
`mlock(2)`/`mprotect(2)`. We may explore more advanced protection
mechanisms in the future.

# <a href="#box-string-and-vec-wrappers" class="doc-anchor">§</a>`Box`, `String`, and `Vec` wrappers

Most users of this crate will simply want
[`Secret`](struct.Secret.html "struct secrecy::Secret") wrappers around
Rust’s core collection types: i.e. `Box`, `String`, and `Vec`.

When the `alloc` feature of this crate is enabled (which it is by
default), [`SecretBox`](type.SecretBox.html "type secrecy::SecretBox"),
[`SecretString`](type.SecretString.html "type secrecy::SecretString"),
and [`SecretVec`](type.SecretVec.html "type secrecy::SecretVec") type
aliases are available.

There’s nothing particularly fancy about these: they’re just the simple
composition of `Secret<Box<_>>`, `Secret<String>`, and `Secret<Vec<_>>`!
However, in many cases they’re all you will need.

# <a href="#advanced-usage" class="doc-anchor">§</a>Advanced usage

If you are hitting limitations on what’s possible with the collection
type wrappers, you’ll want to define your own newtype which lets you
customize the implementation:

``` {.rust .rust-example-rendered}
use secrecy::{CloneableSecret, DebugSecret, Secret, Zeroize};

#[derive(Clone)]
pub struct AccountNumber(String);

impl Zeroize for AccountNumber {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

/// Permits cloning
impl CloneableSecret for AccountNumber {}

/// Provides a `Debug` impl (by default `[[REDACTED]]`)
impl DebugSecret for AccountNumber {}

/// Use this alias when storing secret values
pub type SecretAccountNumber = Secret<AccountNumber>;
```

# <a href="#serde-support" class="doc-anchor">§</a>`serde` support

When the `serde` feature of this crate is enabled, the
[`Secret`](struct.Secret.html "struct secrecy::Secret") type will
receive a \[`Deserialize`\] impl for all `Secret<T>` types where
`T: DeserializeOwned`. This allows *loading* secret values from data
deserialized from `serde` (be careful to clean up any intermediate
secrets when doing this, e.g. the unparsed input!)

To prevent exfiltration of secret values via `serde`, by default
`Secret<T>` does *not* receive a corresponding \[`Serialize`\] impl. If
you would like types of `Secret<T>` to be serializable with `serde`, you
will need to impl the \[`SerializableSecret`\] marker trait on `T`.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../zeroize/index.html" class="mod"
title="mod zeroize"><code>zeroize</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Secret.html" class="struct"
title="struct secrecy::Secret">Secret</a>  
Wrapper type for values that contains secrets, which attempts to limit
accidental exposure and ensure secrets are wiped from memory when
dropped. (e.g. passwords, cryptographic keys, access tokens or other
credentials)

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CloneableSecret.html" class="trait"
title="trait secrecy::CloneableSecret">CloneableSecret</a>  
Marker trait for secrets which are allowed to be cloned

<a href="trait.DebugSecret.html" class="trait"
title="trait secrecy::DebugSecret">DebugSecret</a>  
Debugging trait which is specialized for handling secret values

<a href="trait.ExposeSecret.html" class="trait"
title="trait secrecy::ExposeSecret">ExposeSecret</a>  
Expose a reference to an inner secret

<a href="trait.Zeroize.html" class="trait"
title="trait secrecy::Zeroize">Zeroize</a>  
Trait for securely erasing values from memory.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.SecretBox.html" class="type"
title="type secrecy::SecretBox">SecretBox</a>  
`Box` types containing a secret value

<a href="type.SecretString.html" class="type"
title="type secrecy::SecretString">SecretString</a>  
Secret strings

<a href="type.SecretVec.html" class="type"
title="type secrecy::SecretVec">SecretVec</a>  
`Vec` types containing secret value

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Zeroize.html" class="derive"
title="derive secrecy::Zeroize">Zeroize</a>  
Derive the `Zeroize` trait.
