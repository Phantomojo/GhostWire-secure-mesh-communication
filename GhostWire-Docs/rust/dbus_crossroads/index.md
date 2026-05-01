Crate dbus\_crossroads Copy item path

<span class="sub-heading"><a href="../src/dbus_crossroads/lib.rs.html#1-33" class="src">Source</a>
</span>

Expand description

dbus-crossroads is a framework for quickly implementing interfaces on
object paths, i e server side.

To get started, you can jump into the commented examples, one for
[sync](https://github.com/diwic/dbus-rs/blob/master/dbus-crossroads/examples/server_cr.rs)
one for [async
(dbus-tokio)](https://github.com/diwic/dbus-rs/blob/master/dbus-tokio/examples/tokio_server_cr.rs),
and one [slightly more
advanced](https://github.com/diwic/dbus-rs/blob/master/dbus-tokio/examples/tokio_adv_server_cr.rs),
or familiarize yourself using this API reference.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Context.html" class="struct"
title="struct dbus_crossroads::Context">Context</a>  
Context is the struct that accompanies you through your method call
handler, providing helpful information about the message sent from the
client, as well as some methods to send extra messages (typically
signals) in return.

<a href="struct.Crossroads.html" class="struct"
title="struct dbus_crossroads::Crossroads">Crossroads</a>  
Crossroads is the “main” object, containing object paths, a registry of
interfaces, and a crossreference of which object paths implement which
interfaces.

<a href="struct.IfaceBuilder.html" class="struct"
title="struct dbus_crossroads::IfaceBuilder">IfaceBuilder</a>  
Struct used to build an interface.

<a href="struct.IfaceToken.html" class="struct"
title="struct dbus_crossroads::IfaceToken">IfaceToken</a>  
Contains a reference to a registered interface.

<a href="struct.MethodDesc.html" class="struct"
title="struct dbus_crossroads::MethodDesc">MethodDesc</a>  
Struct used to describe a method when building an interface.

<a href="struct.MethodErr.html" class="struct"
title="struct dbus_crossroads::MethodErr">MethodErr</a>  
A D-Bus Method Error, containing an error name and a description.

<a href="struct.PropBuilder.html" class="struct"
title="struct dbus_crossroads::PropBuilder">PropBuilder</a>  
Struct used to describe a property when building an interface.

<a href="struct.PropContext.html" class="struct"
title="struct dbus_crossroads::PropContext">PropContext</a>  
PropContext is a struct that provides helpful information inside a
get/set property handler.

<a href="struct.SignalDesc.html" class="struct"
title="struct dbus_crossroads::SignalDesc">SignalDesc</a>  
Struct used to describe a signal when building an interface.
