Crate addr2line Copy item path

<span class="sub-heading"><a href="../src/addr2line/lib.rs.html#1-418" class="src">Source</a>
</span>

Expand description

`addr2line` provides a cross-platform library for retrieving per-address
debug information from files with DWARF debug information. Given an
address, it can return the file name, line number, and function name
associated with that address, as well as the inline call stack leading
to that address.

At the lowest level, the library uses a
[`Context`](struct.Context.html "struct addr2line::Context") to cache
parsed information so that multiple lookups are efficient. To create a
`Context`, you first need to open and parse the file using an object
file parser such as [`object`](https://github.com/gimli-rs/object),
create a
[`gimli::Dwarf`](../gimli/read/dwarf/struct.Dwarf.html "struct gimli::read::dwarf::Dwarf"),
and finally call
[`Context::from_dwarf`](struct.Context.html#method.from_dwarf "associated function addr2line::Context::from_dwarf").

Location information is obtained with
[`Context::find_location`](struct.Context.html#method.find_location "method addr2line::Context::find_location")
or
[`Context::find_location_range`](struct.Context.html#method.find_location_range "method addr2line::Context::find_location_range").
Function information is obtained with
[`Context::find_frames`](struct.Context.html#method.find_frames "method addr2line::Context::find_frames"),
which returns a frame for each inline function. Each frame contains both
name and location.

The library also provides a \[`Loader`\] which internally memory maps
the files, uses the `object` crate to do the parsing, and creates a
`Context`. The `Context` is not exposed, but the `Loader` provides the
same functionality via \[`Loader::find_location`\],
\[`Loader::find_location_range`\], and \[`Loader::find_frames`\]. The
`Loader` also provides \[`Loader::find_symbol`\] to use the symbol table
instead of DWARF debugging information. The `Loader` will load Mach-O
dSYM files and split DWARF files as needed.

The crate has a CLI wrapper around the library which provides some of
the functionality of the `addr2line` command line tool distributed with
[GNU
binutils](https://sourceware.org/binutils/docs/binutils/addr2line.html).

# Re-exports<a href="#reexports" class="anchor">§</a>

pub extern crate
<a href="../gimli/index.html" class="mod" title="mod gimli">gimli</a>;

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Context.html" class="struct"
title="struct addr2line::Context">Context</a>  
The state necessary to perform address to line translation.

<a href="struct.Frame.html" class="struct"
title="struct addr2line::Frame">Frame</a>  
A function frame.

<a href="struct.FrameIter.html" class="struct"
title="struct addr2line::FrameIter">FrameIter</a>  
An iterator over function frames.

<a href="struct.FunctionName.html" class="struct"
title="struct addr2line::FunctionName">FunctionName</a>  
A function name.

<a href="struct.Location.html" class="struct"
title="struct addr2line::Location">Location</a>  
A source location.

<a href="struct.LocationRangeIter.html" class="struct"
title="struct addr2line::LocationRangeIter">LocationRangeIter</a>  
Iterator over `Location`s in a range of addresses, returned by
`Context::find_location_range`.

<a href="struct.SplitDwarfLoad.html" class="struct"
title="struct addr2line::SplitDwarfLoad">SplitDwarfLoad</a>  
This struct contains the information needed to find split DWARF data and
to produce a `gimli::Dwarf<R>` for it.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.LookupResult.html" class="enum"
title="enum addr2line::LookupResult">LookupResult</a>  
Operations that consult debug information may require additional files
to be loaded if split DWARF is being used. This enum returns the result
of the operation in the `Output` variant, or information about the split
DWARF that is required and a continuation to invoke once it is available
in the `Load` variant.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.LookupContinuation.html" class="trait"
title="trait addr2line::LookupContinuation">LookupContinuation</a>  
This trait represents a partially complete operation that can be resumed
once a load of needed split DWARF data is completed or abandoned by the
API consumer.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.demangle.html" class="fn"
title="fn addr2line::demangle">demangle</a>  
Demangle a symbol name using the demangling scheme for the given
language.

<a href="fn.demangle_auto.html" class="fn"
title="fn addr2line::demangle_auto">demangle_auto</a>  
Apply ‘best effort’ demangling of a symbol name.
