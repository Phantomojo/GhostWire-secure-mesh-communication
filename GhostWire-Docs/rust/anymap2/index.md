Crate anymap2 Copy item path

<span class="sub-heading"><a href="../src/anymap2/lib.rs.html#1-511" class="src">Source</a>
</span>

Expand description

This crate provides the `AnyMap` type, a safe and convenient store for
one value of each type.

# Modules<a href="#modules" class="anchor">§</a>

<a href="any/index.html" class="mod" title="mod anymap2::any">any</a>  
The different types of `Any` for use in a map.

<a href="raw/index.html" class="mod" title="mod anymap2::raw">raw</a>  
The raw form of a `Map`, allowing untyped access.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Map.html" class="struct"
title="struct anymap2::Map">Map</a>  
A collection containing zero or one values for any given type and
allowing convenient, type-safe access to those values.

<a href="struct.OccupiedEntry.html" class="struct"
title="struct anymap2::OccupiedEntry">OccupiedEntry</a>  
A view into a single occupied location in an `Map`.

<a href="struct.VacantEntry.html" class="struct"
title="struct anymap2::VacantEntry">VacantEntry</a>  
A view into a single empty location in an `Map`.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Entry.html" class="enum"
title="enum anymap2::Entry">Entry</a>  
A view into a single location in an `Map`, which may be vacant or
occupied.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.AnyMap.html" class="type"
title="type anymap2::AnyMap">AnyMap</a>  
The most common type of `Map`: just using `Any`.

<a href="type.SendSyncAnyMap.html" class="type"
title="type anymap2::SendSyncAnyMap">SendSyncAnyMap</a>  
Sync version
