Crate byteorder\_lite Copy item path

<span class="sub-heading"><a href="../src/byteorder_lite/lib.rs.html#1-3983"
class="src">Source</a> </span>

Expand description

This crate is a fork of the
[`byteorder`](https://crates.io/crates/byteorder) crate which sets
`#![forbid(unsafe_code)]`. It includes all traits and most methods from
the original crate, but the `ReadBytesExt::read_*_into` family of
methods had to be removed because they currently cannot be implemented
without unsafe code.

The organization of the crate is pretty simple. A trait,
[`ByteOrder`](https://crates.io/crates/byteorder), specifies byte
conversion methods for each type of number in Rust (sans numbers that
have a platform dependent size like `usize` and `isize`). Two types,
[`BigEndian`](enum.BigEndian.html) and
[`LittleEndian`](enum.LittleEndian.html) implement these methods.
Finally, [`ReadBytesExt`](trait.ReadBytesExt.html) and
[`WriteBytesExt`](trait.WriteBytesExt.html) provide convenience methods
available to all types that implement
[`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) and
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html).

An alias, [`NetworkEndian`](type.NetworkEndian.html), for
[`BigEndian`](enum.BigEndian.html) is provided to help improve code
clarity.

An additional alias, [`NativeEndian`](type.NativeEndian.html), is
provided for the endianness of the local platform. This is convenient
when serializing data for use and conversions are not desired.

# <a href="#examples" class="doc-anchor">§</a>Examples

Read unsigned 16 bit big-endian integers from a
[`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) type:

``` {.rust .rust-example-rendered}
use std::io::Cursor;
use byteorder_lite::{BigEndian, ReadBytesExt};

let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
```

Write unsigned 16 bit little-endian integers to a
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) type:

``` {.rust .rust-example-rendered}
use byteorder_lite::{LittleEndian, WriteBytesExt};

let mut wtr = vec![];
wtr.write_u16::<LittleEndian>(517).unwrap();
wtr.write_u16::<LittleEndian>(768).unwrap();
assert_eq!(wtr, vec![5, 2, 0, 3]);
```

# <a href="#optional-features" class="doc-anchor">§</a>Optional Features

This crate can also be used without the standard library.

# <a href="#alternatives" class="doc-anchor">§</a>Alternatives

The standard numeric types provide built-in methods like `to_le_bytes`
and `from_le_bytes`, which support some of the same use cases.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.BigEndian.html" class="enum"
title="enum byteorder_lite::BigEndian">BigEndian</a>  
Defines big-endian serialization.

<a href="enum.LittleEndian.html" class="enum"
title="enum byteorder_lite::LittleEndian">LittleEndian</a>  
Defines little-endian serialization.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ByteOrder.html" class="trait"
title="trait byteorder_lite::ByteOrder">ByteOrder</a>  
`ByteOrder` describes types that can serialize integers as bytes.

<a href="trait.ReadBytesExt.html" class="trait"
title="trait byteorder_lite::ReadBytesExt">ReadBytesExt</a>  
Extends [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) with
methods for reading numbers. (For `std::io`.)

<a href="trait.WriteBytesExt.html" class="trait"
title="trait byteorder_lite::WriteBytesExt">WriteBytesExt</a>  
Extends [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html)
with methods for writing numbers. (For `std::io`.)

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BE.html" class="type"
title="type byteorder_lite::BE">BE</a>  
A type alias for [`BigEndian`](enum.BigEndian.html).

<a href="type.LE.html" class="type"
title="type byteorder_lite::LE">LE</a>  
A type alias for [`LittleEndian`](enum.LittleEndian.html).

<a href="type.NativeEndian.html" class="type"
title="type byteorder_lite::NativeEndian">NativeEndian</a>  
Defines system native-endian serialization.

<a href="type.NetworkEndian.html" class="type"
title="type byteorder_lite::NetworkEndian">NetworkEndian</a>  
Defines network byte order serialization.
