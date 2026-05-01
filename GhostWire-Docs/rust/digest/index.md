Crate digest Copy item path

<span class="sub-heading"><a href="../src/digest/lib.rs.html#1-301" class="src">Source</a>
</span>

Expand description

This crate provides traits which describe functionality of cryptographic
hash functions and Message Authentication algorithms.

Traits in this repository are organized into the following levels:

-   **High-level convenience traits**:
    [`Digest`](trait.Digest.html "trait digest::Digest"),
    [`DynDigest`](trait.DynDigest.html "trait digest::DynDigest"),
    [`Mac`](trait.Mac.html "trait digest::Mac"). Wrappers around
    lower-level traits for most common use-cases. Users should usually
    prefer using these traits.
-   **Mid-level traits**:
    [`Update`](trait.Update.html "trait digest::Update"),
    [`FixedOutput`](trait.FixedOutput.html "trait digest::FixedOutput"),
    [`FixedOutputReset`](trait.FixedOutputReset.html "trait digest::FixedOutputReset"),
    [`ExtendableOutput`](trait.ExtendableOutput.html "trait digest::ExtendableOutput"),
    [`ExtendableOutputReset`](trait.ExtendableOutputReset.html "trait digest::ExtendableOutputReset"),
    [`XofReader`](trait.XofReader.html "trait digest::XofReader"),
    [`VariableOutput`](trait.VariableOutput.html "trait digest::VariableOutput"),
    [`Reset`](trait.Reset.html "trait digest::Reset"),
    [`KeyInit`](trait.KeyInit.html "trait digest::KeyInit"), and
    [`InnerInit`](trait.InnerInit.html "trait digest::InnerInit"). These
    traits atomically describe available functionality of an algorithm.
-   **Marker traits**:
    [`HashMarker`](trait.HashMarker.html "trait digest::HashMarker"),
    [`MacMarker`](trait.MacMarker.html "trait digest::MacMarker"). Used
    to distinguish different algorithm classes.
-   **Low-level traits** defined in the
    [`core_api`](core_api/index.html "mod digest::core_api") module.
    These traits operate at a block-level and do not contain any
    built-in buffering. They are intended to be implemented by low-level
    algorithm providers only. Usually they should not be used in
    application-level code.

Additionally hash functions implement traits from the standard library:
[`Default`](https://doc.rust-lang.org/1.93.1/core/default/trait.Default.html "trait core::default::Default"),
[`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone"),
[`Write`](https://doc.rust-lang.org/1.93.1/std/io/trait.Write.html "trait std::io::Write").
The latter is feature-gated behind `std` feature, which is usually
enabled by default by hash implementation crates.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use block_buffer;`

`pub use `<a href="../const_oid/index.html" class="mod"
title="mod const_oid"><code>const_oid</code></a>`;`

`pub use crypto_common;`

`pub use crypto_common::generic_array;`

`pub use crypto_common::`<a href="https://docs.rs/typenum/1.19.0/typenum/index.html" class="mod"
title="mod typenum"><code>typenum</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod digest::consts">consts</a>  
<a href="core_api/index.html" class="mod"
title="mod digest::core_api">core_api</a>  
Low-level traits operating on blocks and wrappers around them.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.impl_oid_carrier.html" class="macro"
title="macro digest::impl_oid_carrier">impl_oid_carrier</a>  
Implement dummy type with hidden docs which is used to “carry” hasher
OID for
[`CtVariableCoreWrapper`](core_api/struct.CtVariableCoreWrapper.html "struct digest::core_api::CtVariableCoreWrapper").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CtOutput.html" class="struct"
title="struct digest::CtOutput">CtOutput</a>  
Fixed size output value which provides a safe
[`Eq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.Eq.html "trait core::cmp::Eq")
implementation that runs in constant time.

<a href="struct.InvalidBufferSize.html" class="struct"
title="struct digest::InvalidBufferSize">InvalidBufferSize</a>  
Buffer length is not equal to hash output size.

<a href="struct.InvalidLength.html" class="struct"
title="struct digest::InvalidLength">InvalidLength</a>  
The error type returned when key and/or IV used in the
[`KeyInit`](trait.KeyInit.html "trait digest::KeyInit"),
\[`KeyIvInit`\], and \[`InnerIvInit`\] slice-based methods had an
invalid length.

<a href="struct.InvalidOutputSize.html" class="struct"
title="struct digest::InvalidOutputSize">InvalidOutputSize</a>  
The error type used in variable hash traits.

<a href="struct.MacError.html" class="struct"
title="struct digest::MacError">MacError</a>  
Error type for when the
[`Output`](type.Output.html "type digest::Output") of a
[`Mac`](trait.Mac.html "trait digest::Mac") is not equal to the expected
value.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Digest.html" class="trait"
title="trait digest::Digest">Digest</a>  
Convenience wrapper trait covering functionality of cryptographic hash
functions with fixed output size.

<a href="trait.DynDigest.html" class="trait"
title="trait digest::DynDigest">DynDigest</a>  
Modification of the [`Digest`](trait.Digest.html "trait digest::Digest")
trait suitable for trait objects.

<a href="trait.ExtendableOutput.html" class="trait"
title="trait digest::ExtendableOutput">ExtendableOutput</a>  
Trait for hash functions with extendable-output (XOF).

<a href="trait.ExtendableOutputReset.html" class="trait"
title="trait digest::ExtendableOutputReset">ExtendableOutputReset</a>  
Trait for hash functions with extendable-output (XOF) able to reset
themselves.

<a href="trait.FixedOutput.html" class="trait"
title="trait digest::FixedOutput">FixedOutput</a>  
Trait for hash functions with fixed-size output.

<a href="trait.FixedOutputReset.html" class="trait"
title="trait digest::FixedOutputReset">FixedOutputReset</a>  
Trait for hash functions with fixed-size output able to reset
themselves.

<a href="trait.HashMarker.html" class="trait"
title="trait digest::HashMarker">HashMarker</a>  
Marker trait for cryptographic hash functions.

<a href="trait.InnerInit.html" class="trait"
title="trait digest::InnerInit">InnerInit</a>  
Types which can be initialized from another type (usually block
ciphers).

<a href="trait.KeyInit.html" class="trait"
title="trait digest::KeyInit">KeyInit</a>  
Types which can be initialized from key.

<a href="trait.Mac.html" class="trait" title="trait digest::Mac">Mac</a>  
Convenience wrapper trait covering functionality of Message
Authentication algorithms.

<a href="trait.MacMarker.html" class="trait"
title="trait digest::MacMarker">MacMarker</a>  
Marker trait for Message Authentication algorithms.

<a href="trait.OutputSizeUser.html" class="trait"
title="trait digest::OutputSizeUser">OutputSizeUser</a>  
Types which return data with the given size.

<a href="trait.Reset.html" class="trait"
title="trait digest::Reset">Reset</a>  
Resettable types.

<a href="trait.Update.html" class="trait"
title="trait digest::Update">Update</a>  
Types which consume data with byte granularity.

<a href="trait.VariableOutput.html" class="trait"
title="trait digest::VariableOutput">VariableOutput</a>  
Trait for hash functions with variable-size output.

<a href="trait.VariableOutputReset.html" class="trait"
title="trait digest::VariableOutputReset">VariableOutputReset</a>  
Trait for hash functions with variable-size output able to reset
themselves.

<a href="trait.XofReader.html" class="trait"
title="trait digest::XofReader">XofReader</a>  
Trait for reader types which are used to extract extendable output from
a XOF (extendable-output function) result.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Key.html" class="type" title="type digest::Key">Key</a>  
Key used by \[`KeySizeUser`\] implementors.

<a href="type.Output.html" class="type"
title="type digest::Output">Output</a>  
Output array of
[`OutputSizeUser`](trait.OutputSizeUser.html "trait digest::OutputSizeUser")
implementors.
