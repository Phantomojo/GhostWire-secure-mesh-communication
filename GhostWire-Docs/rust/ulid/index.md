Crate ulid Copy item path

<span class="sub-heading"><a href="../src/ulid/lib.rs.html#1-439" class="src">Source</a>
</span>

Expand description

# <a href="#ulid-rs" class="doc-anchor">§</a>ulid-rs

This is a Rust implementation of the
[ulid](https://github.com/ulid/spec) project which provides Universally
Unique Lexicographically Sortable Identifiers.

## <a href="#quickstart" class="doc-anchor">§</a>Quickstart

``` {.rust .rust-example-rendered}
// Generate a ulid
let ulid = Ulid::new();

// Generate a string for a ulid
let s = ulid.to_string();

// Create from a String
let res = Ulid::from_string(&s);
assert_eq!(ulid, res.unwrap());

// Or using FromStr
let res = s.parse();
assert_eq!(ulid, res.unwrap());
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Generator.html" class="struct"
title="struct ulid::Generator">Generator</a>  
A Ulid generator that provides monotonically increasing Ulids

<a href="struct.Ulid.html" class="struct"
title="struct ulid::Ulid">Ulid</a>  
A Ulid is a unique 128-bit lexicographically sortable identifier

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DecodeError.html" class="enum"
title="enum ulid::DecodeError">DecodeError</a>  
An error that can occur when decoding a base32 string

<a href="enum.EncodeError.html" class="enum"
title="enum ulid::EncodeError">EncodeError</a>  
An error that can occur when encoding a base32 string

<a href="enum.MonotonicError.html" class="enum"
title="enum ulid::MonotonicError">MonotonicError</a>  
Error while trying to generate a monotonic increment in the same
millisecond

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ULID_LEN.html" class="constant"
title="constant ulid::ULID_LEN">ULID_LEN</a>  
Length of a string-encoded Ulid
