Crate libudev Copy item path

<span class="sub-heading"><a href="../src/libudev/lib.rs.html#1-29" class="src">Source</a>
</span>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Attribute.html" class="struct"
title="struct libudev::Attribute">Attribute</a>  
A device attribute.

<a href="struct.Attributes.html" class="struct"
title="struct libudev::Attributes">Attributes</a>  
Iterator over a device’s attributes.

<a href="struct.Context.html" class="struct"
title="struct libudev::Context">Context</a>  
A libudev context. Contexts may not be sent or shared between threads.
The `libudev(3)` manpage says:

<a href="struct.Device.html" class="struct"
title="struct libudev::Device">Device</a>  
A structure that provides access to sysfs/kernel devices.

<a href="struct.Devices.html" class="struct"
title="struct libudev::Devices">Devices</a>  
Iterator over devices.

<a href="struct.Enumerator.html" class="struct"
title="struct libudev::Enumerator">Enumerator</a>  
An enumeration context.

<a href="struct.Error.html" class="struct"
title="struct libudev::Error">Error</a>  
The error type for libudev operations.

<a href="struct.Event.html" class="struct"
title="struct libudev::Event">Event</a>  
An event that indicates a change in device state.

<a href="struct.Monitor.html" class="struct"
title="struct libudev::Monitor">Monitor</a>  
Monitors for device events.

<a href="struct.MonitorSocket.html" class="struct"
title="struct libudev::MonitorSocket">MonitorSocket</a>  
An active monitor that can receive events.

<a href="struct.Properties.html" class="struct"
title="struct libudev::Properties">Properties</a>  
Iterator over a device’s properties.

<a href="struct.Property.html" class="struct"
title="struct libudev::Property">Property</a>  
A device property.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ErrorKind.html" class="enum"
title="enum libudev::ErrorKind">ErrorKind</a>  
Types of errors that occur in libudev.

<a href="enum.EventType.html" class="enum"
title="enum libudev::EventType">EventType</a>  
Types of events that can be received from udev.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type libudev::Result">Result</a>  
A `Result` type for libudev operations.
