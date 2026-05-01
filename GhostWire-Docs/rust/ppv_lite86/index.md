Crate ppv\_lite86 Copy item path

<span class="sub-heading"><a href="../src/ppv_lite86/lib.rs.html#1-42" class="src">Source</a>
</span>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::arch::`<a href="x86_64/union.vec128_storage.html" class="union"
title="union ppv_lite86::x86_64::vec128_storage"><code>vec128_storage</code></a>`;`

`pub use self::arch::`<a href="x86_64/union.vec256_storage.html" class="union"
title="union ppv_lite86::x86_64::vec256_storage"><code>vec256_storage</code></a>`;`

`pub use self::arch::`<a href="x86_64/union.vec512_storage.html" class="union"
title="union ppv_lite86::x86_64::vec512_storage"><code>vec512_storage</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="x86_64/index.html" class="mod"
title="mod ppv_lite86::x86_64">x86_64</a>

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.dispatch.html" class="macro"
title="macro ppv_lite86::dispatch">dispatch</a>  
Generate the full set of optimized implementations to take advantage of
the most important hardware feature sets.

<a href="macro.dispatch_light128.html" class="macro"
title="macro ppv_lite86::dispatch_light128">dispatch_light128</a>  
Generate only the basic implementations necessary to be able to operate
efficiently on 128-bit vectors on this platfrom. For x86-64, that would
mean SSE2 and AVX.

<a href="macro.dispatch_light256.html" class="macro"
title="macro ppv_lite86::dispatch_light256">dispatch_light256</a>  
Generate only the basic implementations necessary to be able to operate
efficiently on 256-bit vectors on this platfrom. For x86-64, that would
mean SSE2, AVX, and AVX2.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AndNot.html" class="trait"
title="trait ppv_lite86::AndNot">AndNot</a>

<a href="trait.ArithOps.html" class="trait"
title="trait ppv_lite86::ArithOps">ArithOps</a>

Ops that depend on word size

<a href="trait.BSwap.html" class="trait"
title="trait ppv_lite86::BSwap">BSwap</a>

<a href="trait.BitOps0.html" class="trait"
title="trait ppv_lite86::BitOps0">BitOps0</a>

Ops that are independent of word size and endian

<a href="trait.BitOps32.html" class="trait"
title="trait ppv_lite86::BitOps32">BitOps32</a>

<a href="trait.BitOps64.html" class="trait"
title="trait ppv_lite86::BitOps64">BitOps64</a>

<a href="trait.BitOps128.html" class="trait"
title="trait ppv_lite86::BitOps128">BitOps128</a>

<a href="trait.LaneWords4.html" class="trait"
title="trait ppv_lite86::LaneWords4">LaneWords4</a>

A vector composed one or more lanes each composed of four words.

<a href="trait.Machine.html" class="trait"
title="trait ppv_lite86::Machine">Machine</a>

<a href="trait.MultiLane.html" class="trait"
title="trait ppv_lite86::MultiLane">MultiLane</a>

A vector composed of multiple 128-bit lanes.

<a href="trait.RotateEachWord32.html" class="trait"
title="trait ppv_lite86::RotateEachWord32">RotateEachWord32</a>

<a href="trait.RotateEachWord64.html" class="trait"
title="trait ppv_lite86::RotateEachWord64">RotateEachWord64</a>

<a href="trait.RotateEachWord128.html" class="trait"
title="trait ppv_lite86::RotateEachWord128">RotateEachWord128</a>

<a href="trait.Store.html" class="trait"
title="trait ppv_lite86::Store">Store</a>

<a href="trait.StoreBytes.html" class="trait"
title="trait ppv_lite86::StoreBytes">StoreBytes</a>

<a href="trait.Swap64.html" class="trait"
title="trait ppv_lite86::Swap64">Swap64</a>

Exchange neigboring ranges of bits of the specified size

<a href="trait.UnsafeFrom.html" class="trait"
title="trait ppv_lite86::UnsafeFrom">UnsafeFrom</a>

<a href="trait.VZip.html" class="trait"
title="trait ppv_lite86::VZip">VZip</a>

Combine single vectors into a multi-lane vector.

<a href="trait.Vec2.html" class="trait"
title="trait ppv_lite86::Vec2">Vec2</a>

A vector composed of two elements, which may be words or themselves
vectors.

<a href="trait.Vec4.html" class="trait"
title="trait ppv_lite86::Vec4">Vec4</a>

A vector composed of four elements, which may be words or themselves
vectors.

<a href="trait.Vec4Ext.html" class="trait"
title="trait ppv_lite86::Vec4Ext">Vec4Ext</a>

Vec4 functions which may not be implemented yet for all Vec4 types.
NOTE: functions in this trait may be moved to Vec4 in any patch release.
To avoid breakage, import Vec4Ext only together with Vec4, and don’t
qualify its methods.

<a href="trait.Vector.html" class="trait"
title="trait ppv_lite86::Vector">Vector</a>

<a href="trait.Words4.html" class="trait"
title="trait ppv_lite86::Words4">Words4</a>

A vector composed of four words; depending on their size, operations may
cross lanes.

<a href="trait.u32x4.html" class="trait"
title="trait ppv_lite86::u32x4">u32x4</a>

<a href="trait.u32x4x2.html" class="trait"
title="trait ppv_lite86::u32x4x2">u32x4x2</a>

<a href="trait.u32x4x4.html" class="trait"
title="trait ppv_lite86::u32x4x4">u32x4x4</a>

<a href="trait.u64x2.html" class="trait"
title="trait ppv_lite86::u64x2">u64x2</a>

<a href="trait.u64x4.html" class="trait"
title="trait ppv_lite86::u64x4">u64x4</a>

<a href="trait.u64x2x2.html" class="trait"
title="trait ppv_lite86::u64x2x2">u64x2x2</a>

<a href="trait.u64x2x4.html" class="trait"
title="trait ppv_lite86::u64x2x4">u64x2x4</a>

<a href="trait.u128x1.html" class="trait"
title="trait ppv_lite86::u128x1">u128x1</a>

<a href="trait.u128x2.html" class="trait"
title="trait ppv_lite86::u128x2">u128x2</a>

<a href="trait.u128x4.html" class="trait"
title="trait ppv_lite86::u128x4">u128x4</a>
