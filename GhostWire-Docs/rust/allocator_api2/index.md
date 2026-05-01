Crate allocator\_api2 Copy item path

<span class="sub-heading"><a href="../src/allocator_api2/lib.rs.html#1-20" class="src">Source</a>
</span>

Expand description

allocator-api2 crate.

# Modules<a href="#modules" class="anchor">§</a>

<a href="alloc/index.html" class="mod"
title="mod allocator_api2::alloc">alloc</a>  
Memory allocation APIs

<a href="boxed/index.html" class="mod"
title="mod allocator_api2::boxed">boxed</a>  
The `Box<T>` type for heap allocation.

<a href="collections/index.html" class="mod"
title="mod allocator_api2::collections">collections</a>  
<a href="vec/index.html" class="mod"
title="mod allocator_api2::vec">vec</a>  
A contiguous growable array type with heap-allocated contents, written
`Vec<T>`.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.unsize_box.html" class="macro"
title="macro allocator_api2::unsize_box">unsize_box</a>  
Allows turning a
[`Box<T: Sized, A>`](boxed/struct.Box.html "struct allocator_api2::boxed::Box")
into a
[`Box<U: ?Sized, A>`](boxed/struct.Box.html "struct allocator_api2::boxed::Box")
where `T` can be unsizing-coerced into a `U`.

<a href="macro.vec.html" class="macro"
title="macro allocator_api2::vec">vec</a>  
Creates a [`Vec`](vec/struct.Vec.html "struct allocator_api2::vec::Vec")
containing the arguments.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.SliceExt.html" class="trait"
title="trait allocator_api2::SliceExt">SliceExt</a>  
Slice methods that use `Box` and `Vec` from this crate.
