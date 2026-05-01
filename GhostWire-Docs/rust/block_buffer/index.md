Crate block\_buffer Copy item path

<span class="sub-heading"><a href="../src/block_buffer/lib.rs.html#1-424" class="src">Source</a>
</span>

Expand description

Fixed size buffer for block processing of data.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use block_buffer::{EagerBuffer, array::typenum::U4};

let mut buf = EagerBuffer::<U4>::default();

let mut accum = Vec::new();
let msg1: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
let msg2: &[u8] = &[10, 11, 12];

buf.digest_blocks(msg1, |blocks| accum.extend_from_slice(blocks));
buf.digest_blocks(msg2, |blocks| accum.extend_from_slice(blocks));

assert_eq!(accum.len(), 3);
assert_eq!(accum[0], [0, 1, 2, 3]);
assert_eq!(accum[1], [4, 5, 6, 7]);
assert_eq!(accum[2], [8, 9, 10, 11]);

let padded_block = buf.pad_with_zeros();
assert_eq!(padded_block, [12, 0, 0, 0]);
```

Note that block size used with buffers MUST be bigger than zero and
smaller than 256. You will get a compilation error with an invalid block
size:

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use block_buffer::{EagerBuffer, array::typenum::U0};
let buf = EagerBuffer::<U0>::default();
```

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use block_buffer::{EagerBuffer, array::typenum::U256};
let buf = EagerBuffer::<U256>::default();
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../hybrid_array/index.html" class="mod"
title="mod hybrid_array"><code>hybrid_array</code></a>` as array;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BlockBuffer.html" class="struct"
title="struct block_buffer::BlockBuffer">BlockBuffer</a>  
Buffer for block processing of data.

<a href="struct.Eager.html" class="struct"
title="struct block_buffer::Eager">Eager</a>  
Eager block buffer kind, which guarantees that buffer position always
lies in the range of `0..BlockSize`.

<a href="struct.Error.html" class="struct"
title="struct block_buffer::Error">Error</a>  
Block buffer error.

<a href="struct.Lazy.html" class="struct"
title="struct block_buffer::Lazy">Lazy</a>  
Lazy block buffer kind, which guarantees that buffer position always
lies in the range of `0..=BlockSize`.

<a href="struct.ReadBuffer.html" class="struct"
title="struct block_buffer::ReadBuffer">ReadBuffer</a>  
Buffer for reading block-generated data.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BlockSizes.html" class="trait"
title="trait block_buffer::BlockSizes">BlockSizes</a>  
Trait implemented for supported block sizes, i.e. for types from `U1` to
`U255`.

<a href="trait.BufferKind.html" class="trait"
title="trait block_buffer::BufferKind">BufferKind</a>  
Trait for buffer kinds.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.EagerBuffer.html" class="type"
title="type block_buffer::EagerBuffer">EagerBuffer</a>  
Eager block buffer.

<a href="type.LazyBuffer.html" class="type"
title="type block_buffer::LazyBuffer">LazyBuffer</a>  
Lazy block buffer.

<a href="type.SerializedBuffer.html" class="type"
title="type block_buffer::SerializedBuffer">SerializedBuffer</a>  
`BlockBuffer` serialized as a byte array.

<a href="type.SerializedBufferSize.html" class="type"
title="type block_buffer::SerializedBufferSize">SerializedBufferSize</a>  
Size of serialized `BlockBuffer` in bytes.
