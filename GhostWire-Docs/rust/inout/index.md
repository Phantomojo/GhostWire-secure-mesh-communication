Crate inout Copy item path

<span class="sub-heading"><a href="../src/inout/lib.rs.html#1-20" class="src">Source</a>
</span>

Expand description

Collection of custom reference types for code generic over in-place and
buffer-to-buffer modes of operation.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.InOut.html" class="struct"
title="struct inout::InOut">InOut</a>  
Custom pointer type which contains one immutable (input) and one mutable
(output) pointer, which are either equal or non-overlapping.

<a href="struct.InOutBuf.html" class="struct"
title="struct inout::InOutBuf">InOutBuf</a>  
Custom slice type which references one immutable (input) slice and one
mutable (output) slice of equal length. Input and output slices are
either the same or do not overlap.

<a href="struct.InOutBufIter.html" class="struct"
title="struct inout::InOutBufIter">InOutBufIter</a>  
Iterator over
[`InOutBuf`](struct.InOutBuf.html "struct inout::InOutBuf").

<a href="struct.InOutBufReserved.html" class="struct"
title="struct inout::InOutBufReserved">InOutBufReserved</a>  
Custom slice type which references one immutable (input) slice and one
mutable (output) slice. Input and output slices are either the same or
do not overlap. Length of the output slice is always equal or bigger
than length of the input slice.

<a href="struct.IntoArrayError.html" class="struct"
title="struct inout::IntoArrayError">IntoArrayError</a>  
The error returned when slice can not be converted into array.

<a href="struct.NotEqualError.html" class="struct"
title="struct inout::NotEqualError">NotEqualError</a>  
The error returned when input and output slices have different length
and thus can not be converted to `InOutBuf`.

<a href="struct.OutIsTooSmallError.html" class="struct"
title="struct inout::OutIsTooSmallError">OutIsTooSmallError</a>  
Output buffer is smaller than input buffer.
