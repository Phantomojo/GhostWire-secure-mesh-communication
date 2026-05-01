Crate eyeball\_im Copy item path

<span class="sub-heading"><a href="../src/eyeball_im/lib.rs.html#1-20" class="src">Source</a>
</span>

Expand description

Observable collections based on the `im` crate.

Cargo features:

-   `tracing`: Emit [tracing](../tracing/index.html "mod tracing")
    events when updates are sent out
-   `serde`: Enable the `Serialize` implementation for
    [`VectorDiff`](enum.VectorDiff.html "enum eyeball_im::VectorDiff")

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use imbl::`<a href="../imbl/vector/type.Vector.html" class="type"
title="type imbl::vector::Vector"><code>Vector</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ObservableVector.html" class="struct"
title="struct eyeball_im::ObservableVector">ObservableVector</a>  
An ordered list of elements that broadcasts any changes made to it.

<a href="struct.ObservableVectorEntries.html" class="struct"
title="struct eyeball_im::ObservableVectorEntries">ObservableVectorEntries</a>  
An “iterator“¹ that yields entries into an
[`ObservableVector`](struct.ObservableVector.html "struct eyeball_im::ObservableVector").

<a href="struct.ObservableVectorEntry.html" class="struct"
title="struct eyeball_im::ObservableVectorEntry">ObservableVectorEntry</a>  
A handle to a single value in an
[`ObservableVector`](struct.ObservableVector.html "struct eyeball_im::ObservableVector").

<a href="struct.ObservableVectorTransaction.html" class="struct"
title="struct eyeball_im::ObservableVectorTransaction">ObservableVectorTransaction</a>  
A transaction that allows making multiple updates to an
`ObservableVector` as an atomic unit.

<a href="struct.ObservableVectorTransactionEntries.html" class="struct"
title="struct eyeball_im::ObservableVectorTransactionEntries">ObservableVectorTransactionEntries</a>  
An “iterator“¹ that yields entries into an
[`ObservableVector`](struct.ObservableVector.html "struct eyeball_im::ObservableVector"),
obtained from a transaction.

<a href="struct.ObservableVectorTransactionEntry.html" class="struct"
title="struct eyeball_im::ObservableVectorTransactionEntry">ObservableVectorTransactionEntry</a>  
A handle to a single value in an
[`ObservableVector`](struct.ObservableVector.html "struct eyeball_im::ObservableVector"),
obtained from a transaction.

<a href="struct.VectorSubscriber.html" class="struct"
title="struct eyeball_im::VectorSubscriber">VectorSubscriber</a>  
A subscriber for updates of a
[`Vector`](../imbl/vector/type.Vector.html "type imbl::vector::Vector").

<a href="struct.VectorSubscriberBatchedStream.html" class="struct"
title="struct eyeball_im::VectorSubscriberBatchedStream">VectorSubscriberBatchedStream</a>  
A batched stream of `VectorDiff`s created from a
[`VectorSubscriber`](struct.VectorSubscriber.html "struct eyeball_im::VectorSubscriber").

<a href="struct.VectorSubscriberStream.html" class="struct"
title="struct eyeball_im::VectorSubscriberStream">VectorSubscriberStream</a>  
A stream of `VectorDiff`s created from a
[`VectorSubscriber`](struct.VectorSubscriber.html "struct eyeball_im::VectorSubscriber").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.VectorDiff.html" class="enum"
title="enum eyeball_im::VectorDiff">VectorDiff</a>  
A change to an
[`ObservableVector`](struct.ObservableVector.html "struct eyeball_im::ObservableVector").
