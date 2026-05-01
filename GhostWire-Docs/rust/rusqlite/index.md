Crate rusqlite Copy item path

<span class="sub-heading"><a href="../src/rusqlite/lib.rs.html#1-2296" class="src">Source</a>
</span>

Expand description

Rusqlite is an ergonomic wrapper for using SQLite from Rust.

Historically, the API was based on the one from
[`rust-postgres`](https://github.com/sfackler/rust-postgres). However,
the two have diverged in many ways, and no compatibility between the two
is intended.

``` {.rust .rust-example-rendered}
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }
    Ok(())
}
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::types::`<a href="types/trait.ToSql.html" class="trait"
title="trait rusqlite::types::ToSql"><code>ToSql</code></a>`;`

`pub use `<a href="../fallible_iterator/index.html" class="mod"
title="mod fallible_iterator"><code>fallible_iterator</code></a>`;`

`pub use `<a href="../fallible_streaming_iterator/index.html" class="mod"
title="mod fallible_streaming_iterator"><code>fallible_streaming_iterator</code></a>`;`

`pub use `<a href="../libsqlite3_sys/index.html" class="mod"
title="mod libsqlite3_sys"><code>libsqlite3_sys</code></a>` as ffi;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="auto_extension/index.html" class="mod"
title="mod rusqlite::auto_extension">auto_extension</a>  
Automatic extension loading

<a href="config/index.html" class="mod"
title="mod rusqlite::config">config</a>  
Configure database connections

<a href="limits/index.html" class="mod"
title="mod rusqlite::limits">limits</a>  
Run-Time Limits

<a href="types/index.html" class="mod"
title="mod rusqlite::types">types</a>  
Traits dealing with SQLite data types.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.named_params.html" class="macro"
title="macro rusqlite::named_params">named_params</a>  
A macro making it more convenient to pass lists of named parameters as a
`&[(&str, &dyn ToSql)]`.

<a href="macro.params.html" class="macro"
title="macro rusqlite::params">params</a>  
A macro making it more convenient to pass longer lists of parameters as
a `&[&dyn ToSql]`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AndThenRows.html" class="struct"
title="struct rusqlite::AndThenRows">AndThenRows</a>  
An iterator over the mapped resulting rows of a query, with an Error
type unifying with Error.

<a href="struct.Batch.html" class="struct"
title="struct rusqlite::Batch">Batch</a>  
Batch fallible iterator

<a href="struct.CachedStatement.html" class="struct"
title="struct rusqlite::CachedStatement">CachedStatement</a>  
Cacheable statement.

<a href="struct.Connection.html" class="struct"
title="struct rusqlite::Connection">Connection</a>  
A connection to a SQLite database.

<a href="struct.InterruptHandle.html" class="struct"
title="struct rusqlite::InterruptHandle">InterruptHandle</a>  
Allows interrupting a long-running computation.

<a href="struct.Map.html" class="struct"
title="struct rusqlite::Map">Map</a>  
`F` is used to transform the *streaming* iterator into a *fallible*
iterator.

<a href="struct.MappedRows.html" class="struct"
title="struct rusqlite::MappedRows">MappedRows</a>  
An iterator over the mapped resulting rows of a query.

<a href="struct.OpenFlags.html" class="struct"
title="struct rusqlite::OpenFlags">OpenFlags</a>  
Flags for opening SQLite database connections. See
[sqlite3\_open\_v2](https://www.sqlite.org/c3ref/open.html) for details.

<a href="struct.ParamsFromIter.html" class="struct"
title="struct rusqlite::ParamsFromIter">ParamsFromIter</a>  
Adapter type which allows any iterator over
[`ToSql`](types/trait.ToSql.html "trait rusqlite::types::ToSql") values
to implement [`Params`](trait.Params.html "trait rusqlite::Params").

<a href="struct.PrepFlags.html" class="struct"
title="struct rusqlite::PrepFlags">PrepFlags</a>  
Prepare flags. See
[sqlite3\_prepare\_v3](https://sqlite.org/c3ref/c_prepare_normalize.html)
for details.

<a href="struct.Row.html" class="struct"
title="struct rusqlite::Row">Row</a>  
A single result row of a query.

<a href="struct.Rows.html" class="struct"
title="struct rusqlite::Rows">Rows</a>  
A handle (lazy fallible streaming iterator) for the resulting rows of a
query.

<a href="struct.Savepoint.html" class="struct"
title="struct rusqlite::Savepoint">Savepoint</a>  
Represents a savepoint on a database connection.

<a href="struct.Statement.html" class="struct"
title="struct rusqlite::Statement">Statement</a>  
A prepared statement.

<a href="struct.Transaction.html" class="struct"
title="struct rusqlite::Transaction">Transaction</a>  
Represents a transaction on a database connection.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DropBehavior.html" class="enum"
title="enum rusqlite::DropBehavior">DropBehavior</a>  
Options for how a Transaction or Savepoint should behave when it is
dropped.

<a href="enum.Error.html" class="enum"
title="enum rusqlite::Error">Error</a>  
Enum listing possible errors from rusqlite.

<a href="enum.ErrorCode.html" class="enum"
title="enum rusqlite::ErrorCode">ErrorCode</a>  
Error Codes

<a href="enum.StatementStatus.html" class="enum"
title="enum rusqlite::StatementStatus">StatementStatus</a>  
Prepared statement status counters.

<a href="enum.TransactionBehavior.html" class="enum"
title="enum rusqlite::TransactionBehavior">TransactionBehavior</a>  
Options for transaction behavior. See [BEGIN
TRANSACTION](http://www.sqlite.org/lang_transaction.html) for details.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MAIN_DB.html" class="constant"
title="constant rusqlite::MAIN_DB">MAIN_DB</a>  
Shorthand for `Main` database.

<a href="constant.TEMP_DB.html" class="constant"
title="constant rusqlite::TEMP_DB">TEMP_DB</a>  
Shorthand for `Temp` database.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BindIndex.html" class="trait"
title="trait rusqlite::BindIndex">BindIndex</a>  
A trait implemented by types that can index into parameters of a
statement.

<a href="trait.Name.html" class="trait"
title="trait rusqlite::Name">Name</a>  
Database, table, column, collation, function, module, vfs name

<a href="trait.OptionalExtension.html" class="trait"
title="trait rusqlite::OptionalExtension">OptionalExtension</a>  
See the [method documentation](#tymethod.optional).

<a href="trait.Params.html" class="trait"
title="trait rusqlite::Params">Params</a>  
Trait used for [sets of
parameter](https://www.sqlite.org/c3ref/bind_blob.html) passed into SQL
statements/queries.

<a href="trait.RowIndex.html" class="trait"
title="trait rusqlite::RowIndex">RowIndex</a>  
A trait implemented by types that can index into columns of a row.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.params_from_iter.html" class="fn"
title="fn rusqlite::params_from_iter">params_from_iter</a>  
Constructor function for a
[`ParamsFromIter`](struct.ParamsFromIter.html "struct rusqlite::ParamsFromIter").
See its documentation for more.

<a href="fn.to_sqlite_error.html" class="fn"
title="fn rusqlite::to_sqlite_error">to_sqlite_error</a><sup>⚠</sup>  
Transform Rust error to SQLite error (message and code).

<a href="fn.version.html" class="fn"
title="fn rusqlite::version">version</a>  
Returns the SQLite version as a string; e.g., `"3.16.2"` for version
3.16.2.

<a href="fn.version_number.html" class="fn"
title="fn rusqlite::version_number">version_number</a>  
Returns the SQLite version as an integer; e.g., `3016002` for version
3.16.2.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type rusqlite::Result">Result</a>  
A typedef of the result returned by many methods.
