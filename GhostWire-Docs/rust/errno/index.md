Crate errno Copy item path

<span class="sub-heading"><a href="../src/errno/lib.rs.html#1-160" class="src">Source</a>
</span>

Expand description

Cross-platform interface to the `errno` variable.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use errno::{Errno, errno, set_errno};

// Get the current value of errno
let e = errno();

// Set the current value of errno
set_errno(e);

// Extract the error code as an i32
let code = e.0;

// Display a human-friendly error message
println!("Error {}: {}", code, e);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Errno.html" class="struct"
title="struct errno::Errno">Errno</a>  
Wraps a platform-specific error code.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.errno.html" class="fn" title="fn errno::errno">errno</a>  
Returns the platform-specific value of `errno`.

<a href="fn.set_errno.html" class="fn"
title="fn errno::set_errno">set_errno</a>  
Sets the platform-specific value of `errno`.
