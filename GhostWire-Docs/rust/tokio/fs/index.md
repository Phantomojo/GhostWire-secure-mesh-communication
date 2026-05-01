[tokio](../index.html)

Module fs Copy item path

<span class="sub-heading"><a href="../../src/tokio/fs/mod.rs.html#1-324" class="src">Source</a>
</span>

Expand description

Asynchronous file utilities.

This module contains utility methods for working with the file system
asynchronously. This includes reading/writing to files, and working with
directories.

Be aware that most operating systems do not provide asynchronous file
system APIs. Because of that, Tokio will use ordinary blocking file
operations behind the scenes. This is done using the
[`spawn_blocking`](../task/fn.spawn_blocking.html "fn tokio::task::spawn_blocking")
threadpool to run them in the background.

The `tokio::fs` module should only be used for ordinary files. Trying to
use it with e.g., a named pipe on Linux can result in surprising
behavior, such as hangs during runtime shutdown. For special files, you
should use a dedicated type such as
[`tokio::net::unix::pipe`](../net/unix/pipe/index.html "mod tokio::net::unix::pipe")
or
[`AsyncFd`](../io/unix/struct.AsyncFd.html "struct tokio::io::unix::AsyncFd")
instead.

Currently, Tokio will always use
[`spawn_blocking`](../task/fn.spawn_blocking.html "fn tokio::task::spawn_blocking")
on all platforms, but it may be changed to use asynchronous file system
APIs such as io\_uring in the future.

# <a href="#usage" class="doc-anchor">§</a>Usage

The easiest way to use this module is to use the utility functions that
operate on entire files:

-   [`tokio::fs::read`](fn.read.html "fn tokio::fs::read")
-   [`tokio::fs::read_to_string`](fn.read_to_string.html "fn tokio::fs::read_to_string")
-   [`tokio::fs::write`](fn.write.html "fn tokio::fs::write")

The two `read` functions reads the entire file and returns its contents.
The `write` function takes the contents of the file and writes those
contents to the file. It overwrites the existing file, if any.

For example, to read the file:

``` {.rust .rust-example-rendered}
let contents = tokio::fs::read_to_string("my_file.txt").await?;

println!("File has {} lines.", contents.lines().count());
```

To overwrite the file:

``` {.rust .rust-example-rendered}
let contents = "First line.\nSecond line.\nThird line.\n";

tokio::fs::write("my_file.txt", contents.as_bytes()).await?;
```

## <a href="#using-file" class="doc-anchor">§</a>Using `File`

The main type for interacting with files is
[`File`](struct.File.html "struct tokio::fs::File"). It can be used to
read from and write to a given file. This is done using the
[`AsyncRead`](../io/trait.AsyncRead.html "trait tokio::io::AsyncRead")
and
[`AsyncWrite`](../io/trait.AsyncWrite.html "trait tokio::io::AsyncWrite")
traits. This type is generally used when you want to do something more
complex than just reading or writing the entire contents in one go.

**Note:** It is important to use
[`flush`](../io/trait.AsyncWriteExt.html#method.flush "method tokio::io::AsyncWriteExt::flush")
when writing to a Tokio
[`File`](struct.File.html "struct tokio::fs::File"). This is because
calls to `write` will return before the write has finished, and
[`flush`](../io/trait.AsyncWriteExt.html#method.flush "method tokio::io::AsyncWriteExt::flush")
will wait for the write to finish. (The write will happen even if you
don’t flush; it will just happen later.) This is different from
[`std::fs::File`](https://doc.rust-lang.org/1.93.1/std/fs/struct.File.html "struct std::fs::File"),
and is due to the fact that `File` uses `spawn_blocking` behind the
scenes.

For example, to count the number of lines in a file without loading the
entire file into memory:

``` {.rust .rust-example-rendered}
use tokio::fs::File;
use tokio::io::AsyncReadExt;

let mut file = File::open("my_file.txt").await?;

let mut chunk = vec![0; 4096];
let mut number_of_lines = 0;
loop {
    let len = file.read(&mut chunk).await?;
    if len == 0 {
        // Length of zero means end of file.
        break;
    }
    for &b in &chunk[..len] {
        if b == b'\n' {
            number_of_lines += 1;
        }
    }
}

println!("File has {} lines.", number_of_lines);
```

For example, to write a file line-by-line:

``` {.rust .rust-example-rendered}
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

let mut file = File::create("my_file.txt").await?;

file.write_all(b"First line.\n").await?;
file.write_all(b"Second line.\n").await?;
file.write_all(b"Third line.\n").await?;

// Remember to call `flush` after writing!
file.flush().await?;
```

## <a href="#tuning-your-file-io" class="doc-anchor">§</a>Tuning your file IO

Tokio’s file uses
[`spawn_blocking`](../task/fn.spawn_blocking.html "fn tokio::task::spawn_blocking")
behind the scenes, and this has serious performance consequences. To get
good performance with file IO on Tokio, it is recommended to batch your
operations into as few `spawn_blocking` calls as possible.

One example of this difference can be seen by comparing the two reading
examples above. The first example uses
[`tokio::fs::read`](fn.read.html "fn tokio::fs::read"), which reads the
entire file in a single `spawn_blocking` call, and then returns it. The
second example will read the file in chunks using many `spawn_blocking`
calls. This means that the second example will most likely be more
expensive for large files. (Of course, using chunks may be necessary for
very large files that don’t fit in memory.)

The following examples will show some strategies for this:

When creating a file, write the data to a `String` or `Vec<u8>` and then
write the entire file in a single `spawn_blocking` call with
`tokio::fs::write`.

``` {.rust .rust-example-rendered}
let mut contents = String::new();

contents.push_str("First line.\n");
contents.push_str("Second line.\n");
contents.push_str("Third line.\n");

tokio::fs::write("my_file.txt", contents.as_bytes()).await?;
```

Use
[`BufReader`](../io/struct.BufReader.html "struct tokio::io::BufReader")
and
[`BufWriter`](../io/struct.BufWriter.html "struct tokio::io::BufWriter")
to buffer many small reads or writes into a few large ones. This example
will most likely only perform one `spawn_blocking` call.

``` {.rust .rust-example-rendered}
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

let mut file = BufWriter::new(File::create("my_file.txt").await?);

file.write_all(b"First line.\n").await?;
file.write_all(b"Second line.\n").await?;
file.write_all(b"Third line.\n").await?;

// Due to the BufWriter, the actual write and spawn_blocking
// call happens when you flush.
file.flush().await?;
```

Manually use
[`std::fs`](https://doc.rust-lang.org/1.93.1/std/fs/index.html "mod std::fs")
inside
[`spawn_blocking`](../task/fn.spawn_blocking.html "fn tokio::task::spawn_blocking").

``` {.rust .rust-example-rendered}
use std::fs::File;
use std::io::{self, Write};
use tokio::task::spawn_blocking;

spawn_blocking(move || {
    let mut file = File::create("my_file.txt")?;

    file.write_all(b"First line.\n")?;
    file.write_all(b"Second line.\n")?;
    file.write_all(b"Third line.\n")?;

    // Unlike Tokio's file, the std::fs file does
    // not need flush.

    io::Result::Ok(())
}).await.unwrap()?;
```

It’s also good to be aware of
[`File::set_max_buf_size`](struct.File.html#method.set_max_buf_size "method tokio::fs::File::set_max_buf_size"),
which controls the maximum amount of bytes that Tokio’s
[`File`](struct.File.html "struct tokio::fs::File") will read or write
in a single
[`spawn_blocking`](../task/fn.spawn_blocking.html "fn tokio::task::spawn_blocking")
call. The default is two megabytes, but this is subject to change.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DirBuilder.html" class="struct"
title="struct tokio::fs::DirBuilder">DirBuilder</a>  
A builder for creating directories in various manners.

<a href="struct.DirEntry.html" class="struct"
title="struct tokio::fs::DirEntry">DirEntry</a>  
Entries returned by the
[`ReadDir`](struct.ReadDir.html "struct tokio::fs::ReadDir") stream.

<a href="struct.File.html" class="struct"
title="struct tokio::fs::File">File</a>  
A reference to an open file on the filesystem.

<a href="struct.OpenOptions.html" class="struct"
title="struct tokio::fs::OpenOptions">OpenOptions</a>  
Options and flags which can be used to configure how a file is opened.

<a href="struct.ReadDir.html" class="struct"
title="struct tokio::fs::ReadDir">ReadDir</a>  
Reads the entries in a directory.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.canonicalize.html" class="fn"
title="fn tokio::fs::canonicalize">canonicalize</a>  
Returns the canonical, absolute form of a path with all intermediate
components normalized and symbolic links resolved.

<a href="fn.copy.html" class="fn" title="fn tokio::fs::copy">copy</a>  
Copies the contents of one file to another. This function will also copy
the permission bits of the original file to the destination file. This
function will overwrite the contents of to.

<a href="fn.create_dir.html" class="fn"
title="fn tokio::fs::create_dir">create_dir</a>  
Creates a new, empty directory at the provided path.

<a href="fn.create_dir_all.html" class="fn"
title="fn tokio::fs::create_dir_all">create_dir_all</a>  
Recursively creates a directory and all of its parent components if they
are missing.

<a href="fn.hard_link.html" class="fn"
title="fn tokio::fs::hard_link">hard_link</a>  
Creates a new hard link on the filesystem.

<a href="fn.metadata.html" class="fn"
title="fn tokio::fs::metadata">metadata</a>  
Given a path, queries the file system to get information about a file,
directory, etc.

<a href="fn.read.html" class="fn" title="fn tokio::fs::read">read</a>  
Reads the entire contents of a file into a bytes vector.

<a href="fn.read_dir.html" class="fn"
title="fn tokio::fs::read_dir">read_dir</a>  
Returns a stream over the entries within a directory.

<a href="fn.read_link.html" class="fn"
title="fn tokio::fs::read_link">read_link</a>  
Reads a symbolic link, returning the file that the link points to.

<a href="fn.read_to_string.html" class="fn"
title="fn tokio::fs::read_to_string">read_to_string</a>  
Creates a future which will open a file for reading and read the entire
contents into a string and return said string.

<a href="fn.remove_dir.html" class="fn"
title="fn tokio::fs::remove_dir">remove_dir</a>  
Removes an existing, empty directory.

<a href="fn.remove_dir_all.html" class="fn"
title="fn tokio::fs::remove_dir_all">remove_dir_all</a>  
Removes a directory at this path, after removing all its contents. Use
carefully!

<a href="fn.remove_file.html" class="fn"
title="fn tokio::fs::remove_file">remove_file</a>  
Removes a file from the filesystem.

<a href="fn.rename.html" class="fn"
title="fn tokio::fs::rename">rename</a>  
Renames a file or directory to a new name, replacing the original file
if `to` already exists.

<a href="fn.set_permissions.html" class="fn"
title="fn tokio::fs::set_permissions">set_permissions</a>  
Changes the permissions found on a file or a directory.

<a href="fn.symlink.html" class="fn"
title="fn tokio::fs::symlink">symlink</a>  
Creates a new symbolic link on the filesystem.

<a href="fn.symlink_metadata.html" class="fn"
title="fn tokio::fs::symlink_metadata">symlink_metadata</a>  
Queries the file system metadata for a path.

<a href="fn.try_exists.html" class="fn"
title="fn tokio::fs::try_exists">try_exists</a>  
Returns `Ok(true)` if the path points at an existing entity.

<a href="fn.write.html" class="fn" title="fn tokio::fs::write">write</a>  
Creates a future that will open a file for writing and write the entire
contents of `contents` to it.
