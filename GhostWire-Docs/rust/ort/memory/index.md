[ort](../index.html)

Module memory Copy item path

<span class="sub-heading"><a href="../../src/ort/memory.rs.html#1-580" class="src">Source</a>
</span>

Expand description

Types for managing memory & device allocations.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AllocatedBlock.html" class="struct"
title="struct ort::memory::AllocatedBlock">AllocatedBlock</a>  
A block of memory allocated by an
[`Allocator`](struct.Allocator.html "struct ort::memory::Allocator").

<a href="struct.AllocationDevice.html" class="struct"
title="struct ort::memory::AllocationDevice">AllocationDevice</a>  
Represents possible devices that have their own device allocator.

<a href="struct.Allocator.html" class="struct"
title="struct ort::memory::Allocator">Allocator</a>  
A device allocator used to manage the allocation of
[`Value`](../value/struct.Value.html "struct ort::value::Value")s.

<a href="struct.MemoryInfo.html" class="struct"
title="struct ort::memory::MemoryInfo">MemoryInfo</a>  
Describes allocation properties for value memory.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AllocatorType.html" class="enum"
title="enum ort::memory::AllocatorType">AllocatorType</a>  
Execution provider allocator type.

<a href="enum.DeviceType.html" class="enum"
title="enum ort::memory::DeviceType">DeviceType</a>  
<a href="enum.MemoryType.html" class="enum"
title="enum ort::memory::MemoryType">MemoryType</a>  
Memory types for allocated memory.
