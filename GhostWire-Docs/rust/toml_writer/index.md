Crate toml\_writer Copy item path

<span class="sub-heading"><a href="../src/toml_writer/lib.rs.html#1-84" class="src">Source</a>
</span>

Expand description

A low-level interface for writing out TOML

Considerations when serializing arbitrary data:

-   Verify the implementation with
    [`toml-test-harness`](https://docs.rs/toml-test-harness)
-   Be sure to group keys under a table before writing another table
-   Watch for extra trailing newlines and leading newlines, both when
    starting with top-level keys or a table
-   When serializing an array-of-tables, be sure to verify that all
    elements of the array serialize as tables
-   Standard tables and inline tables may need separate implementations
    of corner cases, requiring verifying them both

When serializing Rust data structures

-   `Option`: Skip key-value pairs with a value of `None`, otherwise
    error when seeing `None`
    -   When skipping key-value pairs, be careful that a deeply nested
        `None` doesn’t get skipped
-   Scalars and arrays are unsupported as top-level data types
-   Tuples and tuple variants seriallize as arrays
-   Structs, struct variants, and maps serialize as tables
-   Newtype variants serialize as to the inner type
-   Unit variants serialize to a string
-   Unit and unit structs don’t have a clear meaning in TOML

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use toml_writer::TomlWrite as _;

let mut output = String::new();
output.newline()?;
output.open_table_header()?;
output.key("table")?;
output.close_table_header()?;
output.newline()?;

output.key("key")?;
output.space()?;
output.keyval_sep()?;
output.space()?;
output.value("value")?;
output.newline()?;

assert_eq!(output, r#"
[table]
key = "value"
"#);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.TomlInteger.html" class="struct"
title="struct toml_writer::TomlInteger">TomlInteger</a>  
Helper struct for formatting TOML integers.

<a href="struct.TomlIntegerFormat.html" class="struct"
title="struct toml_writer::TomlIntegerFormat">TomlIntegerFormat</a>  
Describes how a TOML integer should be formatted.

<a href="struct.TomlKey.html" class="struct"
title="struct toml_writer::TomlKey">TomlKey</a>  
<a href="struct.TomlKeyBuilder.html" class="struct"
title="struct toml_writer::TomlKeyBuilder">TomlKeyBuilder</a>  
<a href="struct.TomlString.html" class="struct"
title="struct toml_writer::TomlString">TomlString</a>  
<a href="struct.TomlStringBuilder.html" class="struct"
title="struct toml_writer::TomlStringBuilder">TomlStringBuilder</a>  
Describes how a TOML string (key or value) should be formatted.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ToTomlKey.html" class="trait"
title="trait toml_writer::ToTomlKey">ToTomlKey</a>

<a href="trait.ToTomlValue.html" class="trait"
title="trait toml_writer::ToTomlValue">ToTomlValue</a>

<a href="trait.TomlWrite.html" class="trait"
title="trait toml_writer::TomlWrite">TomlWrite</a>

<a href="trait.WriteTomlKey.html" class="trait"
title="trait toml_writer::WriteTomlKey">WriteTomlKey</a>

<a href="trait.WriteTomlValue.html" class="trait"
title="trait toml_writer::WriteTomlValue">WriteTomlValue</a>
