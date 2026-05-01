Crate stable\_deref\_trait Copy item path

<span class="sub-heading"><a href="../src/stable_deref_trait/lib.rs.html#9-202"
class="src">Source</a> </span>

Expand description

This module defines an unsafe marker trait, StableDeref, for container
types that deref to a fixed address which is valid even when the
containing type is moved. For example, Box, Vec, Rc, Arc and String
implement this trait. Additionally, it defines CloneStableDeref for
types like Rc where clones deref to the same address.

It is intended to be used by crates such as
[owning\_ref](https://crates.io/crates/owning_ref) and
[rental](https://crates.io/crates/rental), as well as library authors
who wish to make their code interoperable with such crates. For example,
if you write a custom Vec type, you can implement StableDeref, and then
users will be able to use your custom type together with owning\_ref and
rental.

no\_std support can be enabled by disabling default features
(specifically “std”). In this case, the trait will not be implemented
for the std types mentioned above, but you can still use it for your own
types.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CloneStableDeref.html" class="trait"
title="trait stable_deref_trait::CloneStableDeref">CloneStableDeref</a>  
An unsafe marker trait for types where clones deref to the same address.
This has all the requirements of StableDeref, and additionally requires
that after calling clone(), both the old and new value deref to the same
address. For example, Rc and Arc implement CloneStableDeref, but Box and
Vec do not.

<a href="trait.StableDeref.html" class="trait"
title="trait stable_deref_trait::StableDeref">StableDeref</a>  
An unsafe marker trait for types that deref to a stable address, even
when moved. For example, this is implemented by Box, Vec, Rc, Arc and
String, among others. Even when a Box is moved, the underlying storage
remains at a fixed location.
