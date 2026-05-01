Crate dbus Copy item path

<span class="sub-heading"><a href="../src/dbus/lib.rs.html#1-72" class="src">Source</a>
</span>

Expand description

D-Bus bindings for Rust

[D-Bus](http://dbus.freedesktop.org/) is a message bus, and is mainly
used in Linux for communication between processes. It is present by
default on almost every Linux distribution out there, and runs in two
instances - one per session, and one system-wide.

In addition to the API documentation, which you’re currently reading,
you might want to look in the examples directory, which contains many
examples and some additional documents. README.md also contains a few
quick “getting started” examples (as well as information about the
`futures` and `no-string-validation` features).

In addition to this crate, there are some companion crates:

-   dbus-tokio for integrating D-Bus with [Tokio](http://tokio.rs)
-   dbus-codegen for generating code from D-Bus introspection data
-   libdbus-sys contains the raw bindings to the C libdbus library.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::message::`<a href="message/struct.Message.html" class="struct"
title="struct dbus::message::Message"><code>Message</code></a>`;`

`pub use crate::message::`<a href="message/enum.MessageType.html" class="enum"
title="enum dbus::message::MessageType"><code>MessageType</code></a>`;`

`pub use crate::strings::`<a href="strings/struct.Signature.html" class="struct"
title="struct dbus::strings::Signature"><code>Signature</code></a>`;`

`pub use crate::strings::`<a href="strings/struct.Path.html" class="struct"
title="struct dbus::strings::Path"><code>Path</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="arg/index.html" class="mod" title="mod dbus::arg">arg</a>  
Types and traits for easily getting a message’s arguments, or appending
a message with arguments.

<a href="blocking/index.html" class="mod"
title="mod dbus::blocking">blocking</a>  
Connections and proxies that make blocking method calls.

<a href="channel/index.html" class="mod"
title="mod dbus::channel">channel</a>  
Connection base / building block.

<a href="ffidisp/index.html" class="mod"
title="mod dbus::ffidisp">ffidisp</a>  
A connection that uses FFI callbacks to dispatch messages.

<a href="message/index.html" class="mod"
title="mod dbus::message">message</a>  
Contains structs and traits closely related to D-Bus messages.

<a href="nonblock/index.html" class="mod"
title="mod dbus::nonblock">nonblock</a>  
Async version of connection.

<a href="strings/index.html" class="mod"
title="mod dbus::strings">strings</a>  
This module contains strings with a specific format, such as a valid
Interface name, a valid Error name, etc.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct dbus::Error">Error</a>  
D-Bus Error wrapper.

<a href="struct.MethodErr.html" class="struct"
title="struct dbus::MethodErr">MethodErr</a>  
A D-Bus Method Error, containing an error name and a description.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type dbus::Result">Result</a>  
Alias for a
[`Result`](https://doc.rust-lang.org/1.93.1/core/result/enum.Result.html "enum core::result::Result")
containing [`dbus::Error`](struct.Error.html "struct dbus::Error") by
default.
