[bluer](../index.html)

Module monitor Copy item path

<span class="sub-heading"><a href="../../src/bluer/monitor.rs.html#1-478" class="src">Source</a>
</span>

Expand description

Bluetooth advertisement monitor.

This API allows an client to specify a job of monitoring advertisements
by exposing advertisement monitors with filtering conditions, thresholds
of RSSI and timers of RSSI thresholds.

# Modules<a href="#modules" class="anchor">§</a>

<a href="data_type/index.html" class="mod"
title="mod bluer::monitor::data_type">data_type</a>  
Common advertising data types for
[`Pattern::data_type`](struct.Pattern.html#structfield.data_type "field bluer::monitor::Pattern::data_type").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DeviceId.html" class="struct"
title="struct bluer::monitor::DeviceId">DeviceId</a>  
Information identifying a found or lost device.

<a href="struct.Monitor.html" class="struct"
title="struct bluer::monitor::Monitor">Monitor</a>  
Advertisement monitor specification.

<a href="struct.MonitorHandle.html" class="struct"
title="struct bluer::monitor::MonitorHandle">MonitorHandle</a>  
A registered advertisement monitor target.

<a href="struct.MonitorManager.html" class="struct"
title="struct bluer::monitor::MonitorManager">MonitorManager</a>  
Advertisement monitor manager.

<a href="struct.Pattern.html" class="struct"
title="struct bluer::monitor::Pattern">Pattern</a>  
An advertisement data pattern, used to filter devices in the
advertisement monitor.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.MonitorEvent.html" class="enum"
title="enum bluer::monitor::MonitorEvent">MonitorEvent</a>  
An advertisement monitor event.

<a href="enum.RssiSamplingPeriod.html" class="enum"
title="enum bluer::monitor::RssiSamplingPeriod">RssiSamplingPeriod</a>  
Grouping rules on how to propagate the received advertisement packets to
the client.

<a href="enum.Type.html" class="enum"
title="enum bluer::monitor::Type">Type</a>  
Determines the type of advertisement monitor.
