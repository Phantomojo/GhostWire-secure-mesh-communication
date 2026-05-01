Crate fs2 Copy item path

<span class="sub-heading"><a href="../src/fs2/lib.rs.html#1-458" class="src">Source</a>
</span>

Expand description

Extended utilities for working with files and filesystems in Rust.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.FsStats.html" class="struct"
title="struct fs2::FsStats">FsStats</a>  
`FsStats` contains some common stats about a file system.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FileExt.html" class="trait"
title="trait fs2::FileExt">FileExt</a>  
Extension trait for `std::fs::File` which provides allocation,
duplication and locking methods.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.allocation_granularity.html" class="fn"
title="fn fs2::allocation_granularity">allocation_granularity</a>  
Returns the filesystem’s disk space allocation granularity in bytes. The
provided path may be for any file in the filesystem.

<a href="fn.available_space.html" class="fn"
title="fn fs2::available_space">available_space</a>  
Returns the available space in bytes to non-priveleged users in the file
system containing the provided path.

<a href="fn.free_space.html" class="fn"
title="fn fs2::free_space">free_space</a>  
Returns the number of free bytes in the file system containing the
provided path.

<a href="fn.lock_contended_error.html" class="fn"
title="fn fs2::lock_contended_error">lock_contended_error</a>  
Returns the error that a call to a try lock method on a contended file
will return.

<a href="fn.statvfs.html" class="fn" title="fn fs2::statvfs">statvfs</a>  
Get the stats of the file system containing the provided path.

<a href="fn.total_space.html" class="fn"
title="fn fs2::total_space">total_space</a>  
Returns the total space in bytes in the file system containing the
provided path.
