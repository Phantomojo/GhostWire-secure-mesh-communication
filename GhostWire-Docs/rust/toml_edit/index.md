Crate toml\_edit Copy item path

<span class="sub-heading"><a href="../src/toml_edit/lib.rs.html#1-147" class="src">Source</a>
</span>

Expand description

# <a href="#toml_edit" class="doc-anchor">§</a>`toml_edit`

This crate allows you to parse and modify toml documents, while
preserving comments, spaces *and relative order* of items.

If you also need the ease of a more traditional API, see the
[`toml`](https://docs.rs/toml/latest/toml/) crate.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use toml_edit::{DocumentMut, value};

let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
"#;
let mut doc = toml.parse::<DocumentMut>().expect("invalid doc");
assert_eq!(doc.to_string(), toml);
// let's add a new key/value pair inside a.b: c = {d = "hello"}
doc["a"]["b"]["c"]["d"] = value("hello");
// autoformat inline table a.b.c: { d = "hello" }
doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
"#;
assert_eq!(doc.to_string(), expected);
```

## <a href="#controlling-formatting" class="doc-anchor">§</a>Controlling formatting

By default, values are created with default formatting

``` {.rust .rust-example-rendered}
let mut doc = toml_edit::DocumentMut::new();
doc["foo"] = toml_edit::value("bar");
let expected = r#"foo = "bar"
"#;
assert_eq!(doc.to_string(), expected);
```

You can choose a custom TOML representation by parsing the value.

``` {.rust .rust-example-rendered}
let mut doc = toml_edit::DocumentMut::new();
doc["foo"] = "'bar'".parse::<toml_edit::Item>().unwrap();
let expected = r#"foo = 'bar'
"#;
assert_eq!(doc.to_string(), expected);
```

## <a href="#limitations" class="doc-anchor">§</a>Limitations

Things it does not preserve:

-   Order of dotted keys, see
    [issue](https://github.com/toml-rs/toml/issues/163).

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod" title="mod toml_edit::de">de</a>  
Deserialization support for
[`Datetime`](struct.Datetime.html "struct toml_edit::Datetime")

<a href="ser/index.html" class="mod" title="mod toml_edit::ser">ser</a>  
Serialization support for
[`Datetime`](struct.Datetime.html "struct toml_edit::Datetime")

<a href="visit/index.html" class="mod"
title="mod toml_edit::visit">visit</a>  
Document tree traversal to walk a shared borrow of a document tree.

<a href="visit_mut/index.html" class="mod"
title="mod toml_edit::visit_mut">visit_mut</a>  
Document tree traversal to mutate an exclusive borrow of a document tree
in place.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Array.html" class="struct"
title="struct toml_edit::Array">Array</a>  
A TOML [`Value`](enum.Value.html "enum toml_edit::Value") that contains
a sequence of [`Value`](enum.Value.html "enum toml_edit::Value")s

<a href="struct.ArrayOfTables.html" class="struct"
title="struct toml_edit::ArrayOfTables">ArrayOfTables</a>  
A top-level sequence of
[`Table`](struct.Table.html "struct toml_edit::Table")s, each under
their own header

<a href="struct.Date.html" class="struct"
title="struct toml_edit::Date">Date</a>  
A parsed TOML date value

<a href="struct.Datetime.html" class="struct"
title="struct toml_edit::Datetime">Datetime</a>  
A parsed TOML datetime value

<a href="struct.DatetimeParseError.html" class="struct"
title="struct toml_edit::DatetimeParseError">DatetimeParseError</a>  
Error returned from parsing a `Datetime` in the `FromStr`
implementation.

<a href="struct.Decor.html" class="struct"
title="struct toml_edit::Decor">Decor</a>  
A prefix and suffix,

<a href="struct.Document.html" class="struct"
title="struct toml_edit::Document">Document</a>  
The root TOML [`Table`](struct.Table.html "struct toml_edit::Table"),
containing
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Value`](enum.Value.html "enum toml_edit::Value")
pairs and all other logic
[`Table`](struct.Table.html "struct toml_edit::Table")s

<a href="struct.DocumentMut.html" class="struct"
title="struct toml_edit::DocumentMut">DocumentMut</a>  
The editable root TOML
[`Table`](struct.Table.html "struct toml_edit::Table"), containing
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Value`](enum.Value.html "enum toml_edit::Value")
pairs and all other logic
[`Table`](struct.Table.html "struct toml_edit::Table")s

<a href="struct.Formatted.html" class="struct"
title="struct toml_edit::Formatted">Formatted</a>  
A scalar TOML [`Value`](enum.Value.html "enum toml_edit::Value")’s
logical value and its representation in a `&str`

<a href="struct.InlineOccupiedEntry.html" class="struct"
title="struct toml_edit::InlineOccupiedEntry">InlineOccupiedEntry</a>  
A view into a single occupied location in an
[`InlineTable`](struct.InlineTable.html "struct toml_edit::InlineTable").

<a href="struct.InlineTable.html" class="struct"
title="struct toml_edit::InlineTable">InlineTable</a>  
A TOML [`Value`](enum.Value.html "enum toml_edit::Value") that contains
a collection of
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Value`](enum.Value.html "enum toml_edit::Value")
pairs

<a href="struct.InlineVacantEntry.html" class="struct"
title="struct toml_edit::InlineVacantEntry">InlineVacantEntry</a>  
A view into a single empty location in an
[`InlineTable`](struct.InlineTable.html "struct toml_edit::InlineTable").

<a href="struct.Key.html" class="struct"
title="struct toml_edit::Key">Key</a>  
For Key/[`Value`](enum.Value.html "enum toml_edit::Value") pairs under a
[`Table`](struct.Table.html "struct toml_edit::Table") header or inside
an
[`InlineTable`](struct.InlineTable.html "struct toml_edit::InlineTable")

<a href="struct.KeyMut.html" class="struct"
title="struct toml_edit::KeyMut">KeyMut</a>  
A mutable reference to a
[`Key`](struct.Key.html "struct toml_edit::Key")’s formatting

<a href="struct.OccupiedEntry.html" class="struct"
title="struct toml_edit::OccupiedEntry">OccupiedEntry</a>  
A view into a single occupied location in a
[`Table`](struct.Table.html "struct toml_edit::Table").

<a href="struct.RawString.html" class="struct"
title="struct toml_edit::RawString">RawString</a>  
Opaque string storage for raw TOML; internal to `toml_edit`

<a href="struct.Repr.html" class="struct"
title="struct toml_edit::Repr">Repr</a>  
A TOML [`Value`](enum.Value.html "enum toml_edit::Value") encoded as a
`&str`

<a href="struct.Table.html" class="struct"
title="struct toml_edit::Table">Table</a>  
A TOML table, a top-level collection of
key/[`Value`](enum.Value.html "enum toml_edit::Value") pairs under a
header and logical sub-tables

<a href="struct.Time.html" class="struct"
title="struct toml_edit::Time">Time</a>  
A parsed TOML time value

<a href="struct.TomlError.html" class="struct"
title="struct toml_edit::TomlError">TomlError</a>  
A TOML parse error

<a href="struct.VacantEntry.html" class="struct"
title="struct toml_edit::VacantEntry">VacantEntry</a>  
A view into a single empty location in a
[`Table`](struct.Table.html "struct toml_edit::Table").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Entry.html" class="enum"
title="enum toml_edit::Entry">Entry</a>  
A view into a single location in a
[`Table`](struct.Table.html "struct toml_edit::Table"), which may be
vacant or occupied.

<a href="enum.InlineEntry.html" class="enum"
title="enum toml_edit::InlineEntry">InlineEntry</a>  
A view into a single location in an
[`InlineTable`](struct.InlineTable.html "struct toml_edit::InlineTable"),
which may be vacant or occupied.

<a href="enum.Item.html" class="enum"
title="enum toml_edit::Item">Item</a>  
Type representing either a value, a table, an array of tables, or none.

<a href="enum.Offset.html" class="enum"
title="enum toml_edit::Offset">Offset</a>  
A parsed TOML time offset

<a href="enum.Value.html" class="enum"
title="enum toml_edit::Value">Value</a>  
For [`Key`](struct.Key.html "struct toml_edit::Key")/Value pairs under a
[`Table`](struct.Table.html "struct toml_edit::Table") header or inside
another Value

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.TableLike.html" class="trait"
title="trait toml_edit::TableLike">TableLike</a>  
This trait represents either a `Table`, or an `InlineTable`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.array.html" class="fn" title="fn toml_edit::array">array</a>  
Returns an empty array of tables.

<a href="fn.table.html" class="fn" title="fn toml_edit::table">table</a>  
Returns an empty table.

<a href="fn.value.html" class="fn" title="fn toml_edit::value">value</a>  
Returns a formatted value.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ArrayIntoIter.html" class="type"
title="type toml_edit::ArrayIntoIter">ArrayIntoIter</a>  
An owned iterator type over
[`Array`](struct.Array.html "struct toml_edit::Array")’s
[`Value`](enum.Value.html "enum toml_edit::Value")s

<a href="type.ArrayIter.html" class="type"
title="type toml_edit::ArrayIter">ArrayIter</a>  
An iterator type over
[`Array`](struct.Array.html "struct toml_edit::Array")’s
[`Value`](enum.Value.html "enum toml_edit::Value")s

<a href="type.ArrayIterMut.html" class="type"
title="type toml_edit::ArrayIterMut">ArrayIterMut</a>  
An iterator type over
[`Array`](struct.Array.html "struct toml_edit::Array")’s
[`Value`](enum.Value.html "enum toml_edit::Value")s

<a href="type.ArrayOfTablesIntoIter.html" class="type"
title="type toml_edit::ArrayOfTablesIntoIter">ArrayOfTablesIntoIter</a>  
An iterator type over
[`ArrayOfTables`](struct.ArrayOfTables.html "struct toml_edit::ArrayOfTables")’s
[`Table`](struct.Table.html "struct toml_edit::Table")s

<a href="type.ArrayOfTablesIter.html" class="type"
title="type toml_edit::ArrayOfTablesIter">ArrayOfTablesIter</a>  
An iterator type over
[`ArrayOfTables`](struct.ArrayOfTables.html "struct toml_edit::ArrayOfTables")’s
[`Table`](struct.Table.html "struct toml_edit::Table")s

<a href="type.ArrayOfTablesIterMut.html" class="type"
title="type toml_edit::ArrayOfTablesIterMut">ArrayOfTablesIterMut</a>  
An iterator type over
[`ArrayOfTables`](struct.ArrayOfTables.html "struct toml_edit::ArrayOfTables")’s
[`Table`](struct.Table.html "struct toml_edit::Table")s

<a href="type.ImDocument.html" class="type"
title="type toml_edit::ImDocument">ImDocument</a><span class="stab deprecated" title="">Deprecated</span>  
Type representing a parsed TOML document

<a href="type.InlineTableIntoIter.html" class="type"
title="type toml_edit::InlineTableIntoIter">InlineTableIntoIter</a>  
An owned iterator type over an
[`InlineTable`](struct.InlineTable.html "struct toml_edit::InlineTable")’s
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Value`](enum.Value.html "enum toml_edit::Value")
pairs

<a href="type.InlineTableIter.html" class="type"
title="type toml_edit::InlineTableIter">InlineTableIter</a>  
An iterator type over
[`InlineTable`](struct.InlineTable.html "struct toml_edit::InlineTable")’s
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Value`](enum.Value.html "enum toml_edit::Value")
pairs

<a href="type.InlineTableIterMut.html" class="type"
title="type toml_edit::InlineTableIterMut">InlineTableIterMut</a>  
A mutable iterator type over
[`InlineTable`](struct.InlineTable.html "struct toml_edit::InlineTable")’s
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Value`](enum.Value.html "enum toml_edit::Value")
pairs

<a href="type.IntoIter.html" class="type"
title="type toml_edit::IntoIter">IntoIter</a>  
An owned iterator type over
[`Table`](struct.Table.html "struct toml_edit::Table")’s
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Item`](enum.Item.html "enum toml_edit::Item")
pairs

<a href="type.Iter.html" class="type"
title="type toml_edit::Iter">Iter</a>  
An iterator type over
[`Table`](struct.Table.html "struct toml_edit::Table")’s
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Item`](enum.Item.html "enum toml_edit::Item")
pairs

<a href="type.IterMut.html" class="type"
title="type toml_edit::IterMut">IterMut</a>  
A mutable iterator type over
[`Table`](struct.Table.html "struct toml_edit::Table")’s
[`Key`](struct.Key.html "struct toml_edit::Key")/[`Item`](enum.Item.html "enum toml_edit::Item")
pairs
