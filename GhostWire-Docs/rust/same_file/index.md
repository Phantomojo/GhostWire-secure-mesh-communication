Crate same\_file Copy item path

<span class="sub-heading"><a href="../src/same_file/lib.rs.html#1-572" class="src">Source</a>
</span>

Expand description

This crate provides a safe and simple **cross platform** way to
determine whether two file paths refer to the same file or directory.

Most uses of this crate should be limited to the top-level
[`is_same_file`](fn.is_same_file.html) function, which takes two file
paths and returns true if they refer to the same file or directory:

``` {.rust .rust-example-rendered}
use same_file::is_same_file;

assert!(is_same_file("/bin/sh", "/usr/bin/sh")?);
```

Additionally, this crate provides a [`Handle`](struct.Handle.html) type
that permits a more efficient equality check depending on your access
pattern. For example, if one wanted to check whether any path in a list
of paths corresponded to the process’ stdout handle, then one could
build a handle once for stdout. The equality check for each file in the
list then only requires one stat call instead of two. The code might
look like this:

``` {.rust .rust-example-rendered}
use same_file::Handle;

let candidates = &[
    "examples/is_same_file.rs",
    "examples/is_stderr.rs",
    "examples/stderr",
];
let stdout_handle = Handle::stdout()?;
for candidate in candidates {
    let handle = Handle::from_path(candidate)?;
    if stdout_handle == handle {
        println!("{:?} is stdout!", candidate);
    } else {
        println!("{:?} is NOT stdout!", candidate);
    }
}
```

See
[`examples/is_stderr.rs`](https://github.com/BurntSushi/same-file/blob/master/examples/is_same_file.rs)
for a runnable example and compare the output of:

-   `cargo run --example is_stderr 2> examples/stderr` and
-   `cargo run --example is_stderr`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Handle.html" class="struct"
title="struct same_file::Handle">Handle</a>  
A handle to a file that can be tested for equality with other handles.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.is_same_file.html" class="fn"
title="fn same_file::is_same_file">is_same_file</a>  
Returns true if the two file paths may correspond to the same file.
