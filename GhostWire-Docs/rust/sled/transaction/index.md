[sled](../index.html)

Module transaction Copy item path

<span class="sub-heading"><a href="../../src/sled/transaction.rs.html#1-664"
class="src">Source</a> </span>

Expand description

Fully serializable (ACID) multi-`Tree` transactions

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
let config = Config::new().temporary(true);
let db1 = config.open().unwrap();
let db = db1.open_tree(b"a").unwrap();

// Use write-only transactions as a writebatch:
db.transaction(|db| {
    db.insert(b"k1", b"cats")?;
    db.insert(b"k2", b"dogs")?;
    Ok(())
})?;

// Atomically swap two items:
db.transaction(|db| {
    let v1_option = db.remove(b"k1")?;
    let v1 = v1_option.unwrap();
    let v2_option = db.remove(b"k2")?;
    let v2 = v2_option.unwrap();

    db.insert(b"k1", v2)?;
    db.insert(b"k2", v1)?;

    Ok(())
})?;

assert_eq!(&db.get(b"k1")?.unwrap(), b"dogs");
assert_eq!(&db.get(b"k2")?.unwrap(), b"cats");
```

Transactions also work on tuples of `Tree`s, preserving serializable
ACID semantics! In this example, we treat two trees like a work queue,
atomically apply updates to data and move them from the unprocessed
`Tree` to the processed `Tree`.

``` {.rust .rust-example-rendered}
let config = Config::new().temporary(true);
let db = config.open().unwrap();

let unprocessed = db.open_tree(b"unprocessed items").unwrap();
let processed = db.open_tree(b"processed items").unwrap();

// An update somehow gets into the tree, which we
// later trigger the atomic processing of.
unprocessed.insert(b"k3", b"ligers").unwrap();

// Atomically process the new item and move it
// between `Tree`s.
(&unprocessed, &processed)
    .transaction(|(unprocessed, processed)| {
        let unprocessed_item = unprocessed.remove(b"k3")?.unwrap();
        let mut processed_item = b"yappin' ".to_vec();
        processed_item.extend_from_slice(&unprocessed_item);
        processed.insert(b"k3", processed_item)?;
        Ok(())
    })?;

assert_eq!(unprocessed.get(b"k3").unwrap(), None);
assert_eq!(&processed.get(b"k3").unwrap().unwrap(), b"yappin' ligers");
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.TransactionalTree.html" class="struct"
title="struct sled::transaction::TransactionalTree">TransactionalTree</a>  
A transaction that will be applied atomically to the Tree.

<a href="struct.TransactionalTrees.html" class="struct"
title="struct sled::transaction::TransactionalTrees">TransactionalTrees</a>  
A type which allows for pluggable transactional capabilities

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ConflictableTransactionError.html" class="enum"
title="enum sled::transaction::ConflictableTransactionError">ConflictableTransactionError</a>  
An error type that is returned from the closure passed to the
`transaction` method.

<a href="enum.TransactionError.html" class="enum"
title="enum sled::transaction::TransactionError">TransactionError</a>  
An error type that is returned from the closure passed to the
`transaction` method.

<a href="enum.UnabortableTransactionError.html" class="enum"
title="enum sled::transaction::UnabortableTransactionError">UnabortableTransactionError</a>  
An error type that is returned from the closure passed to the
`transaction` method.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Transactional.html" class="trait"
title="trait sled::transaction::Transactional">Transactional</a>  
A type that may be transacted on in sled transactions.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.abort.html" class="fn"
title="fn sled::transaction::abort">abort</a>  
A simple constructor for `Err(TransactionError::Abort(_))`

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ConflictableTransactionResult.html" class="type"
title="type sled::transaction::ConflictableTransactionResult">ConflictableTransactionResult</a>  
A transaction-related `Result` which is used for transparently handling
concurrency-related conflicts when running transaction closures.

<a href="type.TransactionResult.html" class="type"
title="type sled::transaction::TransactionResult">TransactionResult</a>  
A transaction-related `Result` which is used for returning the final
result of a transaction after potentially running the provided closure
several times due to underlying conflicts.
