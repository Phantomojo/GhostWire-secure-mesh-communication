Crate unsigned\_varint Copy item path

<span class="sub-heading"><a href="../src/unsigned_varint/lib.rs.html#20-40"
class="src">Source</a> </span>

Expand description

Unsigned varint encodes unsigned integers in 7-bit groups. The most
significant bit (MSB) in each byte indicates if another byte follows
(MSB = 1), or not (MSB = 0).

# Modules<a href="#modules" class="anchor">§</a>

<a href="decode/index.html" class="mod"
title="mod unsigned_varint::decode">decode</a>  
Basic unsigned-varint decoding.

<a href="encode/index.html" class="mod"
title="mod unsigned_varint::encode">encode</a>  
Basic unsigned-varint encoding.

<a href="io/index.html" class="mod"
title="mod unsigned_varint::io">io</a>  
Decode using
[`std::io::Read`](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read")
types.
