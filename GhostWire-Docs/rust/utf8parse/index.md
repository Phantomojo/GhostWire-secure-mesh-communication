Crate utf8parse Copy item path

<span class="sub-heading"><a href="../src/utf8parse/lib.rs.html#1-132" class="src">Source</a>
</span>

Expand description

A table-driven UTF-8 Parser

This module implements a table-driven UTF-8 parser which should
theoretically contain the minimal number of branches (1). The only
branch is on the `Action` returned from unpacking a transition.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Parser.html" class="struct"
title="struct utf8parse::Parser">Parser</a>  
A parser for Utf8 Characters

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Receiver.html" class="trait"
title="trait utf8parse::Receiver">Receiver</a>  
Handles codepoint and invalid sequence events from the parser.
