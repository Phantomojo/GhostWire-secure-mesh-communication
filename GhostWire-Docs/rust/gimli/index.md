Crate gimli Copy item path

<span class="sub-heading"><a href="../src/gimli/lib.rs.html#1-80" class="src">Source</a>
</span>

Expand description

`gimli` is a library for reading and writing the [DWARF debugging
format](https://dwarfstd.org/).

See the [read](./read/index.html) and [write](./write/index.html)
modules for examples and API documentation.

## <a href="#cargo-features" class="doc-anchor">§</a>Cargo Features

Cargo features that can be enabled with `gimli`:

-   `std`: Enabled by default. Use the `std` library. Disabling this
    feature allows using `gimli` in embedded environments that do not
    have access to `std`. Note that even when `std` is disabled, `gimli`
    still requires an implementation of the `alloc` crate.

-   `read`: Enabled by default. Enables the `read` module. Use of `std`
    is optional.

-   `write`: Enabled by default. Enables the `write` module. Always uses
    the `std` library.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::`<a href="constants/index.html" class="mod"
title="mod gimli::constants"><code>constants</code></a>`::*;`

`pub use crate::`<a href="read/index.html" class="mod"
title="mod gimli::read"><code>read</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="constants/index.html" class="mod"
title="mod gimli::constants">constants</a>  
Constant definitions.

<a href="leb128/index.html" class="mod"
title="mod gimli::leb128">leb128</a>  
Read and write DWARF’s “Little Endian Base 128” (LEB128) variable length
integer encoding.

<a href="read/index.html" class="mod" title="mod gimli::read">read</a>  
Read DWARF debugging information.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AArch64.html" class="struct"
title="struct gimli::AArch64">AArch64</a>  
ARM 64-bit (AArch64) architecture specific definitions.

<a href="struct.Arm.html" class="struct"
title="struct gimli::Arm">Arm</a>  
ARM architecture specific definitions.

<a href="struct.BigEndian.html" class="struct"
title="struct gimli::BigEndian">BigEndian</a>  
Big endian byte order.

<a href="struct.DebugAbbrevOffset.html" class="struct"
title="struct gimli::DebugAbbrevOffset">DebugAbbrevOffset</a>  
An offset into the `.debug_abbrev` section.

<a href="struct.DebugAddrBase.html" class="struct"
title="struct gimli::DebugAddrBase">DebugAddrBase</a>  
An offset to a set of entries in the `.debug_addr` section.

<a href="struct.DebugAddrIndex.html" class="struct"
title="struct gimli::DebugAddrIndex">DebugAddrIndex</a>  
An index into a set of addresses in the `.debug_addr` section.

<a href="struct.DebugAddrOffset.html" class="struct"
title="struct gimli::DebugAddrOffset">DebugAddrOffset</a>  
An offset into the `.debug_addr` section.

<a href="struct.DebugArangesOffset.html" class="struct"
title="struct gimli::DebugArangesOffset">DebugArangesOffset</a>  
An offset into the `.debug_aranges` section.

<a href="struct.DebugFrameOffset.html" class="struct"
title="struct gimli::DebugFrameOffset">DebugFrameOffset</a>  
An offset into the `.debug_frame` section.

<a href="struct.DebugInfoOffset.html" class="struct"
title="struct gimli::DebugInfoOffset">DebugInfoOffset</a>  
An offset into the `.debug_info` section.

<a href="struct.DebugLineOffset.html" class="struct"
title="struct gimli::DebugLineOffset">DebugLineOffset</a>  
An offset into the `.debug_line` section.

<a href="struct.DebugLineStrOffset.html" class="struct"
title="struct gimli::DebugLineStrOffset">DebugLineStrOffset</a>  
An offset into the `.debug_line_str` section.

<a href="struct.DebugLocListsBase.html" class="struct"
title="struct gimli::DebugLocListsBase">DebugLocListsBase</a>  
An offset to a set of location list offsets in the `.debug_loclists`
section.

<a href="struct.DebugLocListsIndex.html" class="struct"
title="struct gimli::DebugLocListsIndex">DebugLocListsIndex</a>  
An index into a set of location list offsets in the `.debug_loclists`
section.

<a href="struct.DebugMacinfoOffset.html" class="struct"
title="struct gimli::DebugMacinfoOffset">DebugMacinfoOffset</a>  
An offset into the `.debug_macinfo` section.

<a href="struct.DebugMacroOffset.html" class="struct"
title="struct gimli::DebugMacroOffset">DebugMacroOffset</a>  
An offset into the `.debug_macro` section.

<a href="struct.DebugRngListsBase.html" class="struct"
title="struct gimli::DebugRngListsBase">DebugRngListsBase</a>  
An offset to a set of range list offsets in the `.debug_rnglists`
section.

<a href="struct.DebugRngListsIndex.html" class="struct"
title="struct gimli::DebugRngListsIndex">DebugRngListsIndex</a>  
An index into a set of range list offsets in the `.debug_rnglists`
section.

<a href="struct.DebugStrOffset.html" class="struct"
title="struct gimli::DebugStrOffset">DebugStrOffset</a>  
An offset into the `.debug_str` section.

<a href="struct.DebugStrOffsetsBase.html" class="struct"
title="struct gimli::DebugStrOffsetsBase">DebugStrOffsetsBase</a>  
An offset to a set of entries in the `.debug_str_offsets` section.

<a href="struct.DebugStrOffsetsIndex.html" class="struct"
title="struct gimli::DebugStrOffsetsIndex">DebugStrOffsetsIndex</a>  
An index into a set of entries in the `.debug_str_offsets` section.

<a href="struct.DebugTypeSignature.html" class="struct"
title="struct gimli::DebugTypeSignature">DebugTypeSignature</a>  
A type signature as used in the `.debug_types` section.

<a href="struct.DebugTypesOffset.html" class="struct"
title="struct gimli::DebugTypesOffset">DebugTypesOffset</a>  
An offset into the `.debug_types` section.

<a href="struct.DwoId.html" class="struct"
title="struct gimli::DwoId">DwoId</a>  
An optionally-provided implementation-defined compilation unit ID to
enable split DWARF and linking a split compilation unit back together.

<a href="struct.EhFrameOffset.html" class="struct"
title="struct gimli::EhFrameOffset">EhFrameOffset</a>  
An offset into the `.eh_frame` section.

<a href="struct.Encoding.html" class="struct"
title="struct gimli::Encoding">Encoding</a>  
Encoding parameters that are commonly used for multiple DWARF sections.

<a href="struct.LineEncoding.html" class="struct"
title="struct gimli::LineEncoding">LineEncoding</a>  
Encoding parameters for a line number program.

<a href="struct.LittleEndian.html" class="struct"
title="struct gimli::LittleEndian">LittleEndian</a>  
Little endian byte order.

<a href="struct.LocationListsOffset.html" class="struct"
title="struct gimli::LocationListsOffset">LocationListsOffset</a>  
An offset into either the `.debug_loc` section or the `.debug_loclists`
section, depending on the version of the unit the offset was contained
in.

<a href="struct.LoongArch.html" class="struct"
title="struct gimli::LoongArch">LoongArch</a>  
LoongArch architecture specific definitions.

<a href="struct.MIPS.html" class="struct"
title="struct gimli::MIPS">MIPS</a>  
MIPS architecture specific definitions.

<a href="struct.PowerPc64.html" class="struct"
title="struct gimli::PowerPc64">PowerPc64</a>  
PowerPC 64bit

<a href="struct.RangeListsOffset.html" class="struct"
title="struct gimli::RangeListsOffset">RangeListsOffset</a>  
An offset into either the `.debug_ranges` section or the
`.debug_rnglists` section, depending on the version of the unit the
offset was contained in.

<a href="struct.RawRangeListsOffset.html" class="struct"
title="struct gimli::RawRangeListsOffset">RawRangeListsOffset</a>  
An offset into either the `.debug_ranges` section or the
`.debug_rnglists` section, depending on the version of the unit the
offset was contained in.

<a href="struct.Register.html" class="struct"
title="struct gimli::Register">Register</a>  
A DWARF register number.

<a href="struct.RiscV.html" class="struct"
title="struct gimli::RiscV">RiscV</a>  
RISC-V architecture specific definitions.

<a href="struct.X86.html" class="struct"
title="struct gimli::X86">X86</a>  
Intel i386 architecture specific definitions.

<a href="struct.X86_64.html" class="struct"
title="struct gimli::X86_64">X86_64</a>  
AMD64 architecture specific definitions.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DwarfFileType.html" class="enum"
title="enum gimli::DwarfFileType">DwarfFileType</a>  
The “type” of file with DWARF debugging information. This determines,
among other things, which files DWARF sections should be loaded from.

<a href="enum.Format.html" class="enum"
title="enum gimli::Format">Format</a>  
Whether the format of a compilation unit is 32- or 64-bit.

<a href="enum.RunTimeEndian.html" class="enum"
title="enum gimli::RunTimeEndian">RunTimeEndian</a>  
Byte order that is selectable at runtime.

<a href="enum.SectionId.html" class="enum"
title="enum gimli::SectionId">SectionId</a>  
An identifier for a DWARF section.

<a href="enum.UnitSectionOffset.html" class="enum"
title="enum gimli::UnitSectionOffset">UnitSectionOffset</a>  
An offset into the `.debug_info` or `.debug_types` sections.

<a href="enum.Vendor.html" class="enum"
title="enum gimli::Vendor">Vendor</a>  
Which vendor extensions to support.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Endianity.html" class="trait"
title="trait gimli::Endianity">Endianity</a>  
A trait describing the endianity of some buffer.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.NativeEndian.html" class="type"
title="type gimli::NativeEndian">NativeEndian</a>  
The native endianity for the target platform.
