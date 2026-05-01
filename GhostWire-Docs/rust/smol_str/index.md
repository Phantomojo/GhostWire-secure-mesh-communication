Crate smol\_str Copy item path

<span class="sub-heading"><a href="../src/smol_str/lib.rs.html#1-867" class="src">Source</a>
</span>

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.format_smolstr.html" class="macro"
title="macro smol_str::format_smolstr">format_smolstr</a>  
Formats arguments to a
[`SmolStr`](struct.SmolStr.html "struct smol_str::SmolStr"), potentially
without allocating.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SmolStr.html" class="struct"
title="struct smol_str::SmolStr">SmolStr</a>  
A `SmolStr` is a string type that has the following properties:

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.StrExt.html" class="trait"
title="trait smol_str::StrExt">StrExt</a>  
[`str`](https://doc.rust-lang.org/1.93.1/std/primitive.str.html "primitive str")
methods producing
[`SmolStr`](struct.SmolStr.html "struct smol_str::SmolStr")s.

<a href="trait.ToSmolStr.html" class="trait"
title="trait smol_str::ToSmolStr">ToSmolStr</a>  
Convert value to
[`SmolStr`](struct.SmolStr.html "struct smol_str::SmolStr") using
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display"),
potentially without allocating.
