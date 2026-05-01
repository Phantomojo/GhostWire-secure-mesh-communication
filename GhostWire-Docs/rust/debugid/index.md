Crate debugid Copy item path

<span class="sub-heading"><a href="../src/debugid/lib.rs.html#1-543" class="src">Source</a>
</span>

Expand description

This crate provides types for identifiers of object files, such as
executables, dynamic libraries or debug companion files. The concept
originates in Google Breakpad and defines two types:

-   [`CodeId`](struct.CodeId.html): Identifies the file containing
    source code, i.e. the actual library or executable. The identifier
    is platform dependent and implementation defined. Thus, there is no
    canonical representation.
-   [`DebugId`](struct.DebugId.html): Identifies a debug information
    file, which may or may not use information from the Code ID. The
    contents are also implementation defined, but as opposed to
    `CodeId`, the structure is streamlined across platforms. It is also
    guaranteed to be 32 bytes in size.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BreakpadFormat.html" class="struct"
title="struct debugid::BreakpadFormat">BreakpadFormat</a>  
Wrapper around [`DebugId`](struct.DebugId.html) for Breakpad formatting.

<a href="struct.CodeId.html" class="struct"
title="struct debugid::CodeId">CodeId</a>  
Unique platform-dependent identifier of code files.

<a href="struct.DebugId.html" class="struct"
title="struct debugid::DebugId">DebugId</a>  
Unique identifier for debug information files and their debug
information.

<a href="struct.ParseCodeIdError.html" class="struct"
title="struct debugid::ParseCodeIdError">ParseCodeIdError</a>  
Indicates an error parsing a [`CodeId`](struct.CodeId.html).

<a href="struct.ParseDebugIdError.html" class="struct"
title="struct debugid::ParseDebugIdError">ParseDebugIdError</a>  
Indicates an error parsing a [`DebugId`](struct.DebugId.html).
