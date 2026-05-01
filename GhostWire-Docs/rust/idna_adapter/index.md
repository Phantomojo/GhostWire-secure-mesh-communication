Crate idna\_adapter Copy item path

<span class="sub-heading"><a href="../src/idna_adapter/lib.rs.html#9-282" class="src">Source</a>
</span>

Expand description

This crate abstracts over a Unicode back end for the
[`idna`](https://docs.rs/crate/idna/latest) crate.

To work around the lack of
[`global-features`](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)
in Cargo, this crate allows the top level `Cargo.lock` to choose an
alternative Unicode back end for the `idna` crate by pinning a version
of this crate.

See the [README of the latest
version](https://docs.rs/crate/idna_adapter/latest) for more details.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Adapter.html" class="struct"
title="struct idna_adapter::Adapter">Adapter</a>  
An adapter between a Unicode back end an the `idna` crate.

<a href="struct.BidiClass.html" class="struct"
title="struct idna_adapter::BidiClass">BidiClass</a>  
Value for the Bidi\_Class Unicode property.

<a href="struct.BidiClassMask.html" class="struct"
title="struct idna_adapter::BidiClassMask">BidiClassMask</a>  
A mask representing potentially multiple `BidiClass` values.

<a href="struct.JoiningType.html" class="struct"
title="struct idna_adapter::JoiningType">JoiningType</a>  
Value for the Joining\_Type Unicode property.

<a href="struct.JoiningTypeMask.html" class="struct"
title="struct idna_adapter::JoiningTypeMask">JoiningTypeMask</a>  
A mask representing potentially multiple `JoiningType` values.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.FIRST_BC_MASK.html" class="constant"
title="constant idna_adapter::FIRST_BC_MASK">FIRST_BC_MASK</a>  
Mask for allowable bidi classes in the first character of a label
(either LTR or RTL) in a bidi domain.

<a href="constant.LAST_LTR_MASK.html" class="constant"
title="constant idna_adapter::LAST_LTR_MASK">LAST_LTR_MASK</a>  
<a href="constant.LAST_RTL_MASK.html" class="constant"
title="constant idna_adapter::LAST_RTL_MASK">LAST_RTL_MASK</a>  
<a href="constant.LEFT_OR_DUAL_JOINING_MASK.html" class="constant"
title="constant idna_adapter::LEFT_OR_DUAL_JOINING_MASK">LEFT_OR_DUAL_JOINING_MASK</a>  
Mask for checking for both left and dual joining.

<a href="constant.MIDDLE_LTR_MASK.html" class="constant"
title="constant idna_adapter::MIDDLE_LTR_MASK">MIDDLE_LTR_MASK</a>  
<a href="constant.MIDDLE_RTL_MASK.html" class="constant"
title="constant idna_adapter::MIDDLE_RTL_MASK">MIDDLE_RTL_MASK</a>  
<a href="constant.RIGHT_OR_DUAL_JOINING_MASK.html" class="constant"
title="constant idna_adapter::RIGHT_OR_DUAL_JOINING_MASK">RIGHT_OR_DUAL_JOINING_MASK</a>  
Mask for checking for both left and dual joining.

<a href="constant.RTL_MASK.html" class="constant"
title="constant idna_adapter::RTL_MASK">RTL_MASK</a>  
Mask for checking if the domain is a bidi domain.
