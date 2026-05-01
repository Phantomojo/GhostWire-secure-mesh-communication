Crate libdd\_dogstatsd\_client Copy item path

<span class="sub-heading"><a href="../src/libdd_dogstatsd_client/lib.rs.html#3-415"
class="src">Source</a> </span>

Expand description

dogstatsd-client implements a client to emit metrics to a dogstatsd
server. This is made use of in at least the data-pipeline and sidecar
crates.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Client.html" class="struct"
title="struct libdd_dogstatsd_client::Client">Client</a>  
A dogstatsd-client that flushes stats to a given endpoint.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DogStatsDAction.html" class="enum"
title="enum libdd_dogstatsd_client::DogStatsDAction">DogStatsDAction</a>  
The `DogStatsDAction` enum gathers the metric types that can be sent to
the DogStatsD server.

<a href="enum.DogStatsDActionOwned.html" class="enum"
title="enum libdd_dogstatsd_client::DogStatsDActionOwned">DogStatsDActionOwned</a>  
The `DogStatsDActionOwned` enum gathers the metric types that can be
sent to the DogStatsD server. This type takes ownership of the relevant
data to support the sidecar better. For documentation on the dogstatsd
metric types:
https://docs.datadoghq.com/metrics/types/?tab=count#metric-types

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.new.html" class="fn"
title="fn libdd_dogstatsd_client::new">new</a>  
Build a new flusher instance pointed at the provided endpoint. Returns
error if the provided endpoint is not valid.
