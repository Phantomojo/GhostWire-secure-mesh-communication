Crate xml Copy item path

<span class="sub-heading"><a href="../src/xml/lib.rs.html#1-27" class="src">Source</a>
</span>

Expand description

This crate currently provides an almost XML 1.0/1.1-compliant pull
parser.

Please note that functions of this parser may panic. If a panic could
cause a Denial Of Service in your codebase, *you’re* responsible for
wrapping access to this library in `catch_unwind`.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::reader::`<a href="reader/struct.EventReader.html" class="struct"
title="struct xml::reader::EventReader"><code>EventReader</code></a>`;`

`pub use crate::reader::`<a href="reader/struct.ParserConfig.html" class="struct"
title="struct xml::reader::ParserConfig"><code>ParserConfig</code></a>`;`

`pub use crate::writer::`<a href="writer/struct.EmitterConfig.html" class="struct"
title="struct xml::writer::EmitterConfig"><code>EmitterConfig</code></a>`;`

`pub use crate::writer::`<a href="writer/struct.EventWriter.html" class="struct"
title="struct xml::writer::EventWriter"><code>EventWriter</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="attribute/index.html" class="mod"
title="mod xml::attribute">attribute</a>  
Contains XML attributes manipulation types and functions.

<a href="common/index.html" class="mod"
title="mod xml::common">common</a>  
Contains common types and functions used throughout the library.

<a href="escape/index.html" class="mod"
title="mod xml::escape">escape</a>  
Contains functions for performing XML special characters escaping.

<a href="name/index.html" class="mod" title="mod xml::name">name</a>  
Contains XML qualified names manipulation types and functions.

<a href="namespace/index.html" class="mod"
title="mod xml::namespace">namespace</a>  
Contains namespace manipulation types and functions.

<a href="reader/index.html" class="mod"
title="mod xml::reader">reader</a>  
Contains high-level interface for a pull-based XML parser.

<a href="writer/index.html" class="mod"
title="mod xml::writer">writer</a>  
Contains high-level interface for an events-based XML emitter.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Encoding.html" class="enum"
title="enum xml::Encoding">Encoding</a>  
Character encoding used for parsing
