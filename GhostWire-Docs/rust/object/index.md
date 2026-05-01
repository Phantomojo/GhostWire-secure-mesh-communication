Crate object Copy item path

<span class="sub-heading"><a href="../src/object/lib.rs.html#1-94" class="src">Source</a>
</span>

Expand description

# <a href="#object" class="doc-anchor">§</a>`object`

The `object` crate provides a unified interface to working with object
files across platforms. It supports reading relocatable object files and
executable files, and writing relocatable object files and some
executable files.

## <a href="#raw-struct-definitions" class="doc-anchor">§</a>Raw struct definitions

Raw structs are defined for: [ELF](elf/index.html "mod object::elf"),
[Mach-O](macho/index.html "mod object::macho"),
[PE/COFF](pe/index.html "mod object::pe"),
[XCOFF](xcoff/index.html "mod object::xcoff"),
[archive](archive/index.html "mod object::archive"). Types and traits
for zerocopy support are defined in the
[`pod`](pod/index.html "mod object::pod") and
[`endian`](endian/index.html "mod object::endian") modules.

## <a href="#unified-read-api" class="doc-anchor">§</a>Unified read API

The [`read`](read/index.html "mod object::read") module provides a
unified read API using the
[`read::Object`](read/trait.Object.html "trait object::read::Object")
trait. There is an implementation of this trait for
[`read::File`](read/enum.File.html "enum object::read::File"), which
allows reading any file format, as well as implementations for each file
format.

## <a href="#low-level-read-api" class="doc-anchor">§</a>Low level read API

The [`read`](read/index.html#modules "mod object::read") submodules
define helpers that operate on the raw structs. These can be used
instead of the unified API, or in conjunction with it to access details
that are not available via the unified API.

## <a href="#unified-write-api" class="doc-anchor">§</a>Unified write API

The \[`mod@write`\] module provides a unified write API for relocatable
object files using \[`write::Object`\]. This does not support writing
executable files.

## <a href="#low-level-write-api" class="doc-anchor">§</a>Low level write API

The \[`mod@write#modules`\] submodules define helpers for writing the
raw structs.

## <a href="#build-api" class="doc-anchor">§</a>Build API

The \[`mod@build`\] submodules define helpers for building object files,
either from scratch or by modifying existing files.

## <a href="#shared-definitions" class="doc-anchor">§</a>Shared definitions

The crate provides a number of definitions that are used by both the
read and write APIs. These are defined at the top level module, but none
of these are the main entry points of the crate.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="endian/index.html" class="mod"
title="mod object::endian"><code>endian</code></a>`::*;`

`pub use `<a href="pod/index.html" class="mod"
title="mod object::pod"><code>pod</code></a>`::*;`

`pub use `<a href="read/index.html" class="mod"
title="mod object::read"><code>read</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="archive/index.html" class="mod"
title="mod object::archive">archive</a>  
Archive definitions.

<a href="elf/index.html" class="mod" title="mod object::elf">elf</a>  
ELF definitions.

<a href="endian/index.html" class="mod"
title="mod object::endian">endian</a>  
Types for compile-time and run-time endianness.

<a href="macho/index.html" class="mod"
title="mod object::macho">macho</a>  
Mach-O definitions.

<a href="pe/index.html" class="mod" title="mod object::pe">pe</a>  
PE/COFF definitions.

<a href="pod/index.html" class="mod" title="mod object::pod">pod</a>  
Tools for converting file format structures to and from bytes.

<a href="read/index.html" class="mod" title="mod object::read">read</a>  
Interface for reading object files.

<a href="xcoff/index.html" class="mod"
title="mod object::xcoff">xcoff</a>  
XCOFF definitions

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AddressSize.html" class="enum"
title="enum object::AddressSize">AddressSize</a>  
The size of an address value for an architecture.

<a href="enum.Architecture.html" class="enum"
title="enum object::Architecture">Architecture</a>  
A CPU architecture.

<a href="enum.BinaryFormat.html" class="enum"
title="enum object::BinaryFormat">BinaryFormat</a>  
A binary file format.

<a href="enum.ComdatKind.html" class="enum"
title="enum object::ComdatKind">ComdatKind</a>  
The selection kind for a COMDAT section group.

<a href="enum.FileFlags.html" class="enum"
title="enum object::FileFlags">FileFlags</a>  
File flags that are specific to each file format.

<a href="enum.RelocationEncoding.html" class="enum"
title="enum object::RelocationEncoding">RelocationEncoding</a>  
Information about how the result of the relocation operation is encoded
in the place.

<a href="enum.RelocationFlags.html" class="enum"
title="enum object::RelocationFlags">RelocationFlags</a>  
Relocation fields that are specific to each file format and
architecture.

<a href="enum.RelocationKind.html" class="enum"
title="enum object::RelocationKind">RelocationKind</a>  
The operation used to calculate the result of the relocation.

<a href="enum.SectionFlags.html" class="enum"
title="enum object::SectionFlags">SectionFlags</a>  
Section flags that are specific to each file format.

<a href="enum.SectionKind.html" class="enum"
title="enum object::SectionKind">SectionKind</a>  
The kind of a section.

<a href="enum.SegmentFlags.html" class="enum"
title="enum object::SegmentFlags">SegmentFlags</a>  
Segment flags that are specific to each file format.

<a href="enum.SubArchitecture.html" class="enum"
title="enum object::SubArchitecture">SubArchitecture</a>  
A CPU sub-architecture.

<a href="enum.SymbolFlags.html" class="enum"
title="enum object::SymbolFlags">SymbolFlags</a>  
Symbol flags that are specific to each file format.

<a href="enum.SymbolKind.html" class="enum"
title="enum object::SymbolKind">SymbolKind</a>  
The kind of a symbol.

<a href="enum.SymbolScope.html" class="enum"
title="enum object::SymbolScope">SymbolScope</a>  
A symbol scope.
