Crate toml\_datetime Copy item path

<span class="sub-heading"><a href="../src/toml_datetime/lib.rs.html#1-39" class="src">Source</a>
</span>

Expand description

A [TOML](https://github.com/toml-lang/toml)-compatible datetime type

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod" title="mod toml_datetime::de">de</a>  
Deserialization support for
[`Datetime`](struct.Datetime.html "struct toml_datetime::Datetime")

<a href="ser/index.html" class="mod"
title="mod toml_datetime::ser">ser</a>  
Serialization support for
[`Datetime`](struct.Datetime.html "struct toml_datetime::Datetime")

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Date.html" class="struct"
title="struct toml_datetime::Date">Date</a>  
A parsed TOML date value

<a href="struct.Datetime.html" class="struct"
title="struct toml_datetime::Datetime">Datetime</a>  
A parsed TOML datetime value

<a href="struct.DatetimeParseError.html" class="struct"
title="struct toml_datetime::DatetimeParseError">DatetimeParseError</a>  
Error returned from parsing a `Datetime` in the `FromStr`
implementation.

<a href="struct.Time.html" class="struct"
title="struct toml_datetime::Time">Time</a>  
A parsed TOML time value

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Offset.html" class="enum"
title="enum toml_datetime::Offset">Offset</a>  
A parsed TOML time offset
