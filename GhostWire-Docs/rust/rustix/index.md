Crate rustix Copy item path

<span class="sub-heading"><a href="../src/rustix/lib.rs.html#1-427" class="src">Source</a>
</span>

Expand description

`rustix` provides efficient memory-safe and
[I/O-safe](https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md)
wrappers to POSIX-like, Unix-like, Linux, and Winsock syscall-like APIs,
with configurable backends.

With rustix, you can write code like this:

``` {.rust .rust-example-rendered}
let (nread, _received) = rustix::net::recv(&sock, buf, RecvFlags::PEEK)?;
```

instead of like this:

``` {.rust .rust-example-rendered}
let nread = unsafe {
    #[cfg(any(unix, target_os = "wasi"))]
    let raw = sock.as_raw_fd();
    #[cfg(windows)]
    let raw = sock.as_raw_socket();
    match libc::recv(
        raw as _,
        buf.as_mut_ptr().cast(),
        buf.len().try_into().unwrap_or(i32::MAX as _),
        MSG_PEEK,
    ) {
        -1 => return Err(std::io::Error::last_os_error()),
        nread => nread as usize,
    }
};
```

rustix’s APIs perform the following tasks:

-   Error values are translated to
    [`Result`](https://doc.rust-lang.org/1.93.1/core/result/enum.Result.html "enum core::result::Result")s.
-   Buffers are passed as Rust slices.
-   Out-parameters are presented as return values.
-   Path arguments use
    [`Arg`](path/trait.Arg.html "trait rustix::path::Arg"), so they
    accept any string type.
-   File descriptors are passed and returned via
    [`AsFd`](fd/trait.AsFd.html "trait rustix::fd::AsFd") and
    [`OwnedFd`](fd/struct.OwnedFd.html "struct rustix::fd::OwnedFd")
    instead of bare integers, ensuring I/O safety.
-   Constants use `enum`s and
    [`bitflags`](../bitflags/index.html "mod bitflags") types, and
    enable [support for externally defined
    flags](../bitflags/index.html#externally-defined-flags "mod bitflags").
-   Multiplexed functions (eg. `fcntl`, `ioctl`, etc.) are
    de-multiplexed.
-   Variadic functions (eg. `openat`, etc.) are presented as
    non-variadic.
-   Functions that return strings automatically allocate sufficient
    memory and retry the syscall as needed to determine the needed
    length.
-   Functions and types which need `l` prefixes or `64` suffixes to
    enable large-file support (LFS) are used automatically. File sizes
    and offsets are always presented as `u64` and `i64`.
-   Behaviors that depend on the sizes of C types like `long` are
    hidden.
-   In some places, more human-friendly and less historical-accident
    names are used (and documentation aliases are used so that the
    original names can still be searched for).
-   Provide y2038 compatibility, on platforms which support this.
-   Correct selected platform bugs, such as behavioral differences when
    running under seccomp.
-   Use `timespec` for timestamps and timeouts instead of `timeval` and
    `c_int` milliseconds.

Things they don’t do include:

-   Detecting whether functions are supported at runtime, except in
    specific cases where new interfaces need to be detected to support
    y2038 and LFS.
-   Hiding significant differences between platforms.
-   Restricting ambient authorities.
-   Imposing sandboxing features such as filesystem path or network
    address sandboxing.

See [`cap-std`](https://crates.io/crates/cap-std),
[`system-interface`](https://crates.io/crates/system-interface), and
[`io-streams`](https://crates.io/crates/io-streams) for libraries which
do hide significant differences between platforms, and
[`cap-std`](https://crates.io/crates/cap-std) which does perform
sandboxing and restricts ambient authorities.

# Modules<a href="#modules" class="anchor">§</a>

<a href="buffer/index.html" class="mod"
title="mod rustix::buffer">buffer</a>  
Utilities for functions that return data via buffers.

<a href="fd/index.html" class="mod" title="mod rustix::fd">fd</a>  
Export the `*Fd` types and traits that are used in rustix’s public API.

<a href="ffi/index.html" class="mod" title="mod rustix::ffi">ffi</a>  
Utilities related to FFI bindings.

<a href="fs/index.html" class="mod" title="mod rustix::fs">fs</a>  
Filesystem operations.

<a href="io/index.html" class="mod" title="mod rustix::io">io</a>  
I/O operations.

<a href="ioctl/index.html" class="mod"
title="mod rustix::ioctl">ioctl</a>  
Unsafe `ioctl` API.

<a href="not_implemented/index.html" class="mod"
title="mod rustix::not_implemented">not_implemented</a>  
Documentation about unimplemented functions.

<a href="path/index.html" class="mod" title="mod rustix::path">path</a>  
Filesystem path operations.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cstr.html" class="macro"
title="macro rustix::cstr">cstr</a>  
A macro for [`CStr`](ffi/struct.CStr.html "struct rustix::ffi::CStr")
literals.
