Crate labyrinth\_macros Copy item path

<span class="sub-heading"><a href="../src/labyrinth_macros/lib.rs.html#1-136"
class="src">Source</a> </span>

Expand description

`labyrinth_macros` crate provides procedural macros for compile-time
obfuscation. NOT MEANT TO BE USED STANDALONE.

This crate includes macros like `encrypt_string` and `flow_stmt` which
are used to enhance the security of Rust code by obfuscating strings and
control flows.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.encrypt_string.html" class="macro"
title="macro labyrinth_macros::encrypt_string">encrypt_string</a>  
A procedural macro that encrypts a string literal at compile time.

<a href="macro.flow_stmt.html" class="macro"
title="macro labyrinth_macros::flow_stmt">flow_stmt</a>  
A procedural macro that adds a compile-time randomly generated loop and
variables.
