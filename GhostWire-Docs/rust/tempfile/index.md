Crate tempfile Copy item path

<span class="sub-heading"><a href="../src/tempfile/lib.rs.html#1-757" class="src">Source</a>
</span>

Expand description

This is a library for creating temporary files and directories that are
automatically deleted when no longer referenced (i.e., on drop).

-   Use [`tempfile()`](fn.tempfile.html) when you need a real
    [`std::fs::File`](https://doc.rust-lang.org/1.93.1/std/fs/struct.File.html "struct std::fs::File")
    but don’t need to refer to it by-path.
-   Use
    [`NamedTempFile::new()`](struct.NamedTempFile.html#method.new "associated function tempfile::NamedTempFile::new")
    when you need a *named* temporary file that can be refered to its
    path.
-   Use [`tempdir()`](fn.tempdir.html) when you need a temporary
    directory that will be recursively deleted on drop.
-   Use
    [`spooled_tempfile()`](fn.spooled_tempfile.html "fn tempfile::spooled_tempfile")
    when you need an in-memory buffer that will ultimately be backed by
    a temporary file if it gets too large.

# <a href="#design" class="doc-anchor">§</a>Design

This crate provides several approaches to creating temporary files and
directories. [`tempfile()`](fn.tempfile.html) relies on the OS to remove
the temporary file once the last handle is closed.
[`TempDir`](struct.TempDir.html) and
[`NamedTempFile`](struct.NamedTempFile.html) both rely on Rust
destructors for cleanup.

## <a href="#resource-leaking" class="doc-anchor">§</a>Resource Leaking

`tempfile` will (almost) never fail to cleanup temporary resources.
However `TempDir` and `NamedTempFile` will fail if their destructors
don’t run. This is because `tempfile` relies on the OS to cleanup the
underlying file, while `TempDir` and `NamedTempFile` rely on rust
destructors to do so. Destructors may fail to run if the process exits
through an unhandled signal interrupt (like `SIGINT`), or if the
instance is declared statically (like with
[`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs/issues/62)),
among other possible reasons.

## <a href="#unexpected-file-deletion" class="doc-anchor">§</a>Unexpected File Deletion

Most operating systems periodically clean up temporary files that
haven’t been accessed recently (often on the order of multiple days).
This issue does not affect unnamed temporary files but can invalidate
the paths associated with named temporary files on Unix-like systems
because the temporary file can be unlinked from the filesystem while
still open and in-use. See the [temporary file
cleaner](#temporary-file-cleaners) section for more security
implications.

## <a href="#security" class="doc-anchor">§</a>Security

This section discusses security issues relevant to Unix-like operating
systems that use shared temporary directories by default. Importantly,
it’s not relevant for Windows or macOS as both operating systems use
private per-user temporary directories by default.

Applications can mitigate the issues described below by using
[`env::override_temp_dir`](env/fn.override_temp_dir.html "fn tempfile::env::override_temp_dir")
to change the default temporary directory but should do so if and only
if default the temporary directory
([`env::temp_dir`](env/fn.temp_dir.html "fn tempfile::env::temp_dir"))
is unsuitable (is world readable, world writable, managed by a temporary
file cleaner, etc.).

### <a href="#temporary-file-cleaners" class="doc-anchor">§</a>Temporary File Cleaners

In the presence of pathological temporary file cleaner, relying on file
paths is unsafe because a temporary file cleaner could delete the
temporary file which an attacker could then replace.

This isn’t an issue for
[`tempfile`](fn.tempfile.html "fn tempfile::tempfile") as it doesn’t
rely on file paths. However,
[`NamedTempFile`](struct.NamedTempFile.html) and temporary directories
*do* rely on file paths for *some* operations. See the security
documentation on the [`NamedTempFile`](struct.NamedTempFile.html) and
the [`TempDir`](struct.TempDir.html) types for more information.

Mitigation:

-   This is rarely an issue for short-lived files as temporary file
    cleaners usually only remove temporary files that haven’t been
    modified or accessed within many (10-30) days.
-   Very long lived temporary files should be placed in directories not
    managed by temporary file cleaners.

### <a href="#access-permissions" class="doc-anchor">§</a>Access Permissions

Temporary *files* created with this library are private by default on
all operating systems. However, temporary *directories* are created with
the default permissions and will therefore be world-readable by default
unless the user has changed their umask and/or default temporary
directory.

### <a href="#denial-of-service" class="doc-anchor">§</a>Denial of Service

If the file-name randomness
([`Builder::rand_bytes`](struct.Builder.html#method.rand_bytes "method tempfile::Builder::rand_bytes"))
is too small and/or this crate is built without the `getrandom` feature,
it may be possible for an attacker to predict the random file names
chosen by this library, preventing temporary file creation by creating
temporary files with these predicted file names. By default, this
library mitigates this denial of service attack by:

1.  Defaulting to 6 random characters per temporary file forcing an
    attacker to create billions of files before random collisions are
    expected (at which point you probably have larger problems).
2.  Re-seeding the random filename generator from system randomness
    after 3 failed attempts to create temporary a file (when the
    `getrandom` feature is enabled as it is by default on all major
    platforms).

## <a href="#early-drop-pitfall" class="doc-anchor">§</a>Early drop pitfall

Because `TempDir` and `NamedTempFile` rely on their destructors for
cleanup, this can lead to an unexpected early removal of the
directory/file, usually when working with APIs which are generic over
`AsRef<Path>`. Consider the following example:

``` {.rust .rust-example-rendered}
use tempfile::tempdir;
use std::process::Command;

// Create a directory inside of `env::temp_dir()`.
let temp_dir = tempdir()?;

// Spawn the `touch` command inside the temporary directory and collect the exit status
// Note that `temp_dir` is **not** moved into `current_dir`, but passed as a reference
let exit_status = Command::new("touch").arg("tmp").current_dir(&temp_dir).status()?;
assert!(exit_status.success());
```

This works because a reference to `temp_dir` is passed to `current_dir`,
resulting in the destructor of `temp_dir` being run after the `Command`
has finished execution. Moving the `TempDir` into the `current_dir` call
would result in the `TempDir` being converted into an internal
representation, with the original value being dropped and the directory
thus being deleted, before the command can be executed.

The `touch` command would fail with an `No such file or directory`
error.

## <a href="#examples" class="doc-anchor">§</a>Examples

Create a temporary file and write some data into it:

``` {.rust .rust-example-rendered}
use tempfile::tempfile;
use std::io::Write;

// Create a file inside of `env::temp_dir()`.
let mut file = tempfile()?;

writeln!(file, "Brian was here. Briefly.")?;
```

Create a named temporary file and open an independent file handle:

``` {.rust .rust-example-rendered}
use tempfile::NamedTempFile;
use std::io::{Write, Read};

let text = "Brian was here. Briefly.";

// Create a file inside of `env::temp_dir()`.
let mut file1 = NamedTempFile::new()?;

// Re-open it.
let mut file2 = file1.reopen()?;

// Write some test data to the first handle.
file1.write_all(text.as_bytes())?;

// Read the test data using the second handle.
let mut buf = String::new();
file2.read_to_string(&mut buf)?;
assert_eq!(buf, text);
```

Create a temporary directory and add a file to it:

``` {.rust .rust-example-rendered}
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

// Create a directory inside of `env::temp_dir()`.
let dir = tempdir()?;

let file_path = dir.path().join("my-temporary-note.txt");
let mut file = File::create(file_path)?;
writeln!(file, "Brian was here. Briefly.")?;

// By closing the `TempDir` explicitly, we can check that it has
// been deleted successfully. If we don't close it explicitly,
// the directory will still be deleted when `dir` goes out
// of scope, but we won't know whether deleting the directory
// succeeded.
drop(file);
dir.close()?;
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="env/index.html" class="mod" title="mod tempfile::env">env</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Builder.html" class="struct"
title="struct tempfile::Builder">Builder</a>  
Create a new temporary file or directory with custom options.

<a href="struct.NamedTempFile.html" class="struct"
title="struct tempfile::NamedTempFile">NamedTempFile</a>  
A named temporary file.

<a href="struct.PathPersistError.html" class="struct"
title="struct tempfile::PathPersistError">PathPersistError</a>  
Error returned when persisting a temporary file path fails.

<a href="struct.PersistError.html" class="struct"
title="struct tempfile::PersistError">PersistError</a>  
Error returned when persisting a temporary file fails.

<a href="struct.SpooledTempFile.html" class="struct"
title="struct tempfile::SpooledTempFile">SpooledTempFile</a>  
An object that behaves like a regular temporary file, but keeps data in
memory until it reaches a configured size, at which point the data is
written to a temporary file on disk, and further operations use the file
on disk.

<a href="struct.TempDir.html" class="struct"
title="struct tempfile::TempDir">TempDir</a>  
A directory in the filesystem that is automatically deleted when it goes
out of scope.

<a href="struct.TempPath.html" class="struct"
title="struct tempfile::TempPath">TempPath</a>  
A path to a named temporary file without an open file handle.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.SpooledData.html" class="enum"
title="enum tempfile::SpooledData">SpooledData</a>  
A wrapper for the two states of a
[`SpooledTempFile`](struct.SpooledTempFile.html "struct tempfile::SpooledTempFile").
Either:

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.spooled_tempfile.html" class="fn"
title="fn tempfile::spooled_tempfile">spooled_tempfile</a>  
Create a new
[`SpooledTempFile`](struct.SpooledTempFile.html "struct tempfile::SpooledTempFile").
Also see
[`spooled_tempfile_in`](fn.spooled_tempfile_in.html "fn tempfile::spooled_tempfile_in").

<a href="fn.spooled_tempfile_in.html" class="fn"
title="fn tempfile::spooled_tempfile_in">spooled_tempfile_in</a>  
Construct a new
[`SpooledTempFile`](struct.SpooledTempFile.html "struct tempfile::SpooledTempFile"),
backed by a file in the specified directory. Use this when, e.g., you
need the temporary file to be backed by a specific filesystem (e.g.,
when your default temporary directory is in-memory). Also see
[`spooled_tempfile`](fn.spooled_tempfile.html "fn tempfile::spooled_tempfile").

<a href="fn.tempdir.html" class="fn"
title="fn tempfile::tempdir">tempdir</a>  
Create a new temporary directory. Also see
[`tempdir_in`](fn.tempdir_in.html "fn tempfile::tempdir_in").

<a href="fn.tempdir_in.html" class="fn"
title="fn tempfile::tempdir_in">tempdir_in</a>  
Create a new temporary directory in a specific directory. Also see
[`tempdir`](struct.TempDir.html).

<a href="fn.tempfile.html" class="fn"
title="fn tempfile::tempfile">tempfile</a>  
Create a new temporary file. Also see
[`tempfile_in`](fn.tempfile_in.html "fn tempfile::tempfile_in").

<a href="fn.tempfile_in.html" class="fn"
title="fn tempfile::tempfile_in">tempfile_in</a>  
Create a new temporary file in the specified directory. Also see
[`tempfile`](fn.tempfile.html "fn tempfile::tempfile").
