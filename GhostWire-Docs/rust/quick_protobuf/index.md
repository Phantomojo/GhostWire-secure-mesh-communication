Crate quick\_protobuf Copy item path

<span class="sub-heading"><a href="../src/quick_protobuf/lib.rs.html#1-23" class="src">Source</a>
</span>

Expand description

A library to read binary protobuf files

This reader is developed similarly to a pull reader

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::errors::`<a href="errors/enum.Error.html" class="enum"
title="enum quick_protobuf::errors::Error"><code>Error</code></a>`;`

`pub use crate::errors::`<a href="errors/type.Result.html" class="type"
title="type quick_protobuf::errors::Result"><code>Result</code></a>`;`

`pub use crate::message::`<a href="message/trait.MessageInfo.html" class="trait"
title="trait quick_protobuf::message::MessageInfo"><code>MessageInfo</code></a>`;`

`pub use crate::message::`<a href="message/trait.MessageRead.html" class="trait"
title="trait quick_protobuf::message::MessageRead"><code>MessageRead</code></a>`;`

`pub use crate::message::`<a href="message/trait.MessageWrite.html" class="trait"
title="trait quick_protobuf::message::MessageWrite"><code>MessageWrite</code></a>`;`

`pub use crate::reader::`<a href="reader/fn.deserialize_from_slice.html" class="fn"
title="fn quick_protobuf::reader::deserialize_from_slice"><code>deserialize_from_slice</code></a>`;`

`pub use crate::reader::`<a href="reader/struct.BytesReader.html" class="struct"
title="struct quick_protobuf::reader::BytesReader"><code>BytesReader</code></a>`;`

`pub use crate::writer::`<a href="writer/fn.serialize_into_slice.html" class="fn"
title="fn quick_protobuf::writer::serialize_into_slice"><code>serialize_into_slice</code></a>`;`

`pub use crate::writer::`<a href="writer/struct.BytesWriter.html" class="struct"
title="struct quick_protobuf::writer::BytesWriter"><code>BytesWriter</code></a>`;`

`pub use crate::writer::`<a href="writer/struct.Writer.html" class="struct"
title="struct quick_protobuf::writer::Writer"><code>Writer</code></a>`;`

`pub use crate::writer::`<a href="writer/trait.WriterBackend.html" class="trait"
title="trait quick_protobuf::writer::WriterBackend"><code>WriterBackend</code></a>`;`

`pub use crate::reader::`<a href="reader/struct.Reader.html" class="struct"
title="struct quick_protobuf::reader::Reader"><code>Reader</code></a>`;`

`pub use crate::writer::`<a href="writer/fn.serialize_into_vec.html" class="fn"
title="fn quick_protobuf::writer::serialize_into_vec"><code>serialize_into_vec</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="errors/index.html" class="mod"
title="mod quick_protobuf::errors">errors</a>  
A module to handle all errors

<a href="message/index.html" class="mod"
title="mod quick_protobuf::message">message</a>  
A module to deserialize a `Message` as defined in a .proto file

<a href="reader/index.html" class="mod"
title="mod quick_protobuf::reader">reader</a>  
A module to manage protobuf deserialization

<a href="sizeofs/index.html" class="mod"
title="mod quick_protobuf::sizeofs">sizeofs</a>  
A module to compute the binary size of data once encoded

<a href="writer/index.html" class="mod"
title="mod quick_protobuf::writer">writer</a>  
A module to manage protobuf serialization
