Crate walkdir Copy item path

<span class="sub-heading"><a href="../src/walkdir/lib.rs.html#1-1194" class="src">Source</a>
</span>

Expand description

Crate `walkdir` provides an efficient and cross platform implementation
of recursive directory traversal. Several options are exposed to control
iteration, such as whether to follow symbolic links (default off), limit
the maximum number of simultaneous open file descriptors and the ability
to efficiently skip descending into directories.

To use this crate, add `walkdir` as a dependency to your project’s
`Cargo.toml`:

``` language-toml
[dependencies]
walkdir = "2"
```

# <a href="#from-the-top" class="doc-anchor">§</a>From the top

The [`WalkDir`](struct.WalkDir.html) type builds iterators. The
[`DirEntry`](struct.DirEntry.html) type describes values yielded by the
iterator. Finally, the [`Error`](struct.Error.html) type is a small
wrapper around
[`std::io::Error`](https://doc.rust-lang.org/stable/std/io/struct.Error.html)
with additional information, such as if a loop was detected while
following symbolic links (not enabled by default).

# <a href="#example" class="doc-anchor">§</a>Example

The following code recursively iterates over the directory given and
prints the path for each entry:

``` {.rust .rust-example-rendered}
use walkdir::WalkDir;

for entry in WalkDir::new("foo") {
    println!("{}", entry?.path().display());
}
```

Or, if you’d like to iterate over all entries and ignore any errors that
may arise, use
[`filter_map`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter_map).
(e.g., This code below will silently skip directories that the owner of
the running process does not have permission to access.)

``` {.rust .rust-example-rendered}
use walkdir::WalkDir;

for entry in WalkDir::new("foo").into_iter().filter_map(|e| e.ok()) {
    println!("{}", entry.path().display());
}
```

# <a href="#example-follow-symbolic-links" class="doc-anchor">§</a>Example: follow symbolic links

The same code as above, except
[`follow_links`](struct.WalkDir.html#method.follow_links) is enabled:

``` {.rust .rust-example-rendered}
use walkdir::WalkDir;

for entry in WalkDir::new("foo").follow_links(true) {
    println!("{}", entry?.path().display());
}
```

# <a href="#example-skip-hidden-files-and-directories-on-unix"
class="doc-anchor">§</a>Example: skip hidden files and directories on unix

This uses the [`filter_entry`](struct.IntoIter.html#method.filter_entry)
iterator adapter to avoid yielding hidden files and directories
efficiently (i.e. without recursing into hidden directories):

``` {.rust .rust-example-rendered}
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

let walker = WalkDir::new("foo").into_iter();
for entry in walker.filter_entry(|e| !is_hidden(e)) {
    println!("{}", entry?.path().display());
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DirEntry.html" class="struct"
title="struct walkdir::DirEntry">DirEntry</a>  
A directory entry.

<a href="struct.Error.html" class="struct"
title="struct walkdir::Error">Error</a>  
An error produced by recursively walking a directory.

<a href="struct.FilterEntry.html" class="struct"
title="struct walkdir::FilterEntry">FilterEntry</a>  
A recursive directory iterator that skips entries.

<a href="struct.IntoIter.html" class="struct"
title="struct walkdir::IntoIter">IntoIter</a>  
An iterator for recursively descending into a directory.

<a href="struct.WalkDir.html" class="struct"
title="struct walkdir::WalkDir">WalkDir</a>  
A builder to create an iterator for recursively walking a directory.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DirEntryExt.html" class="trait"
title="trait walkdir::DirEntryExt">DirEntryExt</a>  
Unix-specific extension methods for `walkdir::DirEntry`

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type walkdir::Result">Result</a>  
A result type for walkdir operations.
