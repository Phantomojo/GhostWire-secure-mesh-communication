Crate nix Copy item path

<span class="sub-heading"><a href="../src/nix/lib.rs.html#1-382" class="src">Source</a>
</span>

Expand description

Rust friendly bindings to the various \*nix system functions.

Modules are structured according to the C header file that they would be
defined in.

# <a href="#features" class="doc-anchor">§</a>Features

Nix uses the following Cargo features to enable optional functionality.
They may be enabled in any combination.

-   `acct` - Process accounting
-   `aio` - POSIX AIO
-   `dir` - Stuff relating to directory iteration
-   `env` - Manipulate environment variables
-   `event` - Event-driven APIs, like `kqueue` and `epoll`
-   `fanotify` - Linux’s `fanotify` filesystem events monitoring API
-   `feature` - Query characteristics of the OS at runtime
-   `fs` - File system functionality
-   `hostname` - Get and set the system’s hostname
-   `inotify` - Linux’s `inotify` file system notification API
-   `ioctl` - The `ioctl` syscall, and wrappers for many specific
    instances
-   `kmod` - Load and unload kernel modules
-   `mman` - Stuff relating to memory management
-   `mount` - Mount and unmount file systems
-   `mqueue` - POSIX message queues
-   `net` - Networking-related functionality
-   `personality` - Set the process execution domain
-   `poll` - APIs like `poll` and `select`
-   `process` - Stuff relating to running processes
-   `pthread` - POSIX threads
-   `ptrace` - Process tracing and debugging
-   `quota` - File system quotas
-   `reboot` - Reboot the system
-   `resource` - Process resource limits
-   `sched` - Manipulate process’s scheduling
-   `socket` - Sockets, whether for networking or local use
-   `signal` - Send and receive signals to processes
-   `term` - Terminal control APIs
-   `time` - Query the operating system’s clocks
-   `ucontext` - User thread context
-   `uio` - Vectored I/O
-   `user` - Stuff relating to users and groups
-   `zerocopy` - APIs like `sendfile` and `copy_file_range`

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../libc/index.html" class="mod"
title="mod libc"><code>libc</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="errno/index.html" class="mod" title="mod nix::errno">errno</a>  
Safe wrappers around errno functions

<a href="fcntl/index.html" class="mod" title="mod nix::fcntl">fcntl</a>  
file control options

<a href="sys/index.html" class="mod" title="mod nix::sys">sys</a>  
Mostly platform-specific functionality

<a href="unistd/index.html" class="mod"
title="mod nix::unistd">unistd</a>  
Safe wrappers around functions found in libc “unistd.h” header

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.ioctl_none.html" class="macro"
title="macro nix::ioctl_none">ioctl_none</a>  
Generates a wrapper function for an ioctl that passes no data to the
kernel.

<a href="macro.ioctl_none_bad.html" class="macro"
title="macro nix::ioctl_none_bad">ioctl_none_bad</a>  
Generates a wrapper function for a “bad” ioctl that passes no data to
the kernel.

<a href="macro.ioctl_read.html" class="macro"
title="macro nix::ioctl_read">ioctl_read</a>  
Generates a wrapper function for an ioctl that reads data from the
kernel.

<a href="macro.ioctl_read_bad.html" class="macro"
title="macro nix::ioctl_read_bad">ioctl_read_bad</a>  
Generates a wrapper function for a “bad” ioctl that reads data from the
kernel.

<a href="macro.ioctl_read_buf.html" class="macro"
title="macro nix::ioctl_read_buf">ioctl_read_buf</a>  
Generates a wrapper function for an ioctl that reads an array of
elements from the kernel.

<a href="macro.ioctl_readwrite.html" class="macro"
title="macro nix::ioctl_readwrite">ioctl_readwrite</a>  
Generates a wrapper function for an ioctl that reads and writes data to
the kernel.

<a href="macro.ioctl_readwrite_bad.html" class="macro"
title="macro nix::ioctl_readwrite_bad">ioctl_readwrite_bad</a>  
Generates a wrapper function for a “bad” ioctl that reads and writes
data to the kernel.

<a href="macro.ioctl_readwrite_buf.html" class="macro"
title="macro nix::ioctl_readwrite_buf">ioctl_readwrite_buf</a>  
Generates a wrapper function for an ioctl that reads and writes an array
of elements to the kernel.

<a href="macro.ioctl_write_buf.html" class="macro"
title="macro nix::ioctl_write_buf">ioctl_write_buf</a>  
Generates a wrapper function for an ioctl that writes an array of
elements to the kernel.

<a href="macro.ioctl_write_int.html" class="macro"
title="macro nix::ioctl_write_int">ioctl_write_int</a>  
Generates a wrapper function for a ioctl that writes an integer to the
kernel.

<a href="macro.ioctl_write_int_bad.html" class="macro"
title="macro nix::ioctl_write_int_bad">ioctl_write_int_bad</a>  
Generates a wrapper function for a “bad” ioctl that writes an integer to
the kernel.

<a href="macro.ioctl_write_ptr.html" class="macro"
title="macro nix::ioctl_write_ptr">ioctl_write_ptr</a>  
Generates a wrapper function for an ioctl that writes data through a
pointer to the kernel.

<a href="macro.ioctl_write_ptr_bad.html" class="macro"
title="macro nix::ioctl_write_ptr_bad">ioctl_write_ptr_bad</a>  
Generates a wrapper function for a “bad” ioctl that writes data through
a pointer to the kernel.

<a href="macro.request_code_none.html" class="macro"
title="macro nix::request_code_none">request_code_none</a>  
Generate an ioctl request code for a command that passes no data.

<a href="macro.request_code_read.html" class="macro"
title="macro nix::request_code_read">request_code_read</a>  
Generate an ioctl request code for a command that reads.

<a href="macro.request_code_readwrite.html" class="macro"
title="macro nix::request_code_readwrite">request_code_readwrite</a>  
Generate an ioctl request code for a command that reads and writes.

<a href="macro.request_code_write.html" class="macro"
title="macro nix::request_code_write">request_code_write</a>  
Generate an ioctl request code for a command that writes.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.NixPath.html" class="trait"
title="trait nix::NixPath">NixPath</a>  
Common trait used to represent file system paths by many Nix functions.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Error.html" class="type" title="type nix::Error">Error</a>  
Nix’s main error type.

<a href="type.Result.html" class="type"
title="type nix::Result">Result</a>  
Nix Result Type
