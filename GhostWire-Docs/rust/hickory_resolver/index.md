Crate hickory\_resolver Copy item path

<span class="sub-heading"><a href="../src/hickory_resolver/lib.rs.html#8-241"
class="src">Source</a> </span>

Expand description

The Resolver is responsible for performing recursive queries to lookup
domain names.

This is a 100% in process DNS resolver. It *does not* use the Host OS’
resolver. If what is desired is to use the Host OS’ resolver, generally
in the system’s libc, then the `std::net::ToSocketAddrs` variant over
`&str` should be used.

Unlike the `hickory-client`, this tries to provide a simpler interface
to perform DNS queries. For update options, i.e. Dynamic DNS, the
`hickory-client` crate must be used instead. The Resolver library is
capable of searching multiple domains (this can be disabled by using an
FQDN during lookup), dual-stack IPv4/IPv6 lookups, performing chained
CNAME lookups, and features connection metric tracking for attempting to
pick the best upstream DNS resolver.

This as best as possible attempts to abide by the DNS RFCs, please file
issues at <https://github.com/hickory-dns/hickory-dns>.

# <a href="#usage" class="doc-anchor">§</a>Usage

## <a href="#declare-dependency" class="doc-anchor">§</a>Declare dependency

``` language-toml
[dependency]
hickory-resolver = "*"
```

## <a href="#using-the-host-system-config" class="doc-anchor">§</a>Using the host system config

On Unix systems, the `/etc/resolv.conf` can be used for configuration.
Not all options specified in the host systems `resolv.conf` are
applicable or compatible with this software. In addition there may be
additional options supported which the host system does not. Example:

``` {.rust .rust-example-rendered}
// Use the host OS'es `/etc/resolv.conf`
let resolver = Resolver::builder_tokio().unwrap().build();
let response = resolver.lookup_ip("www.example.com.").await.unwrap();
```

## <a href="#using-the-tokioasync-resolver" class="doc-anchor">§</a>Using the Tokio/Async Resolver

``` {.rust .rust-example-rendered}
use std::net::*;
use tokio::runtime::Runtime;
use hickory_resolver::Resolver;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::config::*;

// We need a Tokio Runtime to run the resolver
//  this is responsible for running all Future tasks and registering interest in IO channels
let mut io_loop = Runtime::new().unwrap();

// Construct a new Resolver with default configuration options
let resolver = Resolver::builder_with_config(
    ResolverConfig::default(),
    TokioConnectionProvider::default()
).build();

// Lookup the IP addresses associated with a name.
// This returns a future that will lookup the IP addresses, it must be run in the Core to
//  to get the actual result.
let lookup_future = resolver.lookup_ip("www.example.com.");

// Run the lookup until it resolves or errors
let mut response = io_loop.block_on(lookup_future).unwrap();

// There can be many addresses associated with the name,
//  this can return IPv4 and/or IPv6 addresses
let _address = response.iter().next().expect("no addresses returned!");
```

Generally after a lookup in an asynchronous context, there would
probably be a connection made to a server, for example:

``` {.rust .rust-example-rendered}
let ips = io_loop.block_on(resolver.lookup_ip("www.example.com.")).unwrap();

let result = io_loop.block_on(async {
    let ip = ips.iter().next().unwrap();
    TcpStream::connect((ip, 443))
})
.and_then(|conn| Ok(conn) /* do something with the connection... */)
.unwrap();
```

It’s beyond the scope of these examples to show how to deal with
connection failures and looping etc. But if you wanted to say try a
different address from the result set after a connection failure, it
will be necessary to create a type that implements the `Future` trait.
Inside the `Future::poll` method would be the place to implement a loop
over the different IP addresses.

## <a href="#optional-protocol-support" class="doc-anchor">§</a>Optional protocol support

The following DNS protocols are optionally supported:

-   Enable `tls` for DNS over TLS (DoT)
-   Enable `https-rustls` for DNS over HTTP/2 (DoH)
-   Enable `quic` for DNS over QUIC (DoQ)
-   Enable `h3` for DNS over HTTP/3 (DoH3)

### <a href="#example" class="doc-anchor">§</a>Example

Enable the TLS library through the dependency on `hickory-resolver`:

``` language-toml
hickory-resolver = { version = "*", features = ["tls"] }
```

A default TLS configuration is available for Cloudflare’s `1.1.1.1` DNS
service (Quad9 as well):

``` {.rust .rust-example-rendered}
use hickory_resolver::Resolver;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::config::*;

// Construct a new Resolver with default configuration options
let mut resolver = Resolver::builder_with_config(
    ResolverConfig::cloudflare_tls(),
    TokioConnectionProvider::default(),
).build();

// see example above...
```

## <a href="#mdns-multicast-dns" class="doc-anchor">§</a>mDNS (multicast DNS)

Multicast DNS is an experimental feature in Hickory DNS at the moment.
Its support on different platforms is not yet ideal. Initial support is
only for IPv4 mDNS, as there are some complexities to figure out with
IPv6. Once enabled, an mDNS `NameServer` will automatically be added to
the `Resolver` and used for any lookups performed in the `.local.` zone.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../hickory_proto/index.html" class="mod"
title="mod hickory_proto"><code>hickory_proto</code></a>` as proto;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="caching_client/index.html" class="mod"
title="mod hickory_resolver::caching_client">caching_client</a>  
Caching related functionality for the Resolver.

<a href="config/index.html" class="mod"
title="mod hickory_resolver::config">config</a>  
Configuration for a resolver

<a href="dns_lru/index.html" class="mod"
title="mod hickory_resolver::dns_lru">dns_lru</a>  
An LRU cache designed for work with DNS lookups

<a href="lookup/index.html" class="mod"
title="mod hickory_resolver::lookup">lookup</a>  
Lookup result from a resolution of ipv4 and ipv6 records with a
Resolver.

<a href="lookup_ip/index.html" class="mod"
title="mod hickory_resolver::lookup_ip">lookup_ip</a>  
LookupIp result from a resolution of ipv4 and ipv6 records with a
Resolver.

<a href="name_server/index.html" class="mod"
title="mod hickory_resolver::name_server">name_server</a>  
A module with associated items for working with nameservers

<a href="system_conf/index.html" class="mod"
title="mod hickory_resolver::system_conf">system_conf</a>  
System configuration loading

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Hosts.html" class="struct"
title="struct hickory_resolver::Hosts">Hosts</a>  
Configuration for the local hosts file

<a href="struct.Name.html" class="struct"
title="struct hickory_resolver::Name">Name</a>  
A domain name

<a href="struct.ResolveError.html" class="struct"
title="struct hickory_resolver::ResolveError">ResolveError</a>  
The error type for errors that get returned in the crate

<a href="struct.Resolver.html" class="struct"
title="struct hickory_resolver::Resolver">Resolver</a>  
An asynchronous resolver for DNS generic over async Runtimes.

<a href="struct.ResolverBuilder.html" class="struct"
title="struct hickory_resolver::ResolverBuilder">ResolverBuilder</a>  
A builder to construct a
[`Resolver`](struct.Resolver.html "struct hickory_resolver::Resolver").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ResolveErrorKind.html" class="enum"
title="enum hickory_resolver::ResolveErrorKind">ResolveErrorKind</a>  
The error kind for errors that get returned in the crate

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IntoName.html" class="trait"
title="trait hickory_resolver::IntoName">IntoName</a>  
Conversion into a Name

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.version.html" class="fn"
title="fn hickory_resolver::version">version</a>  
returns a version as specified in Cargo.toml

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.TokioResolver.html" class="type"
title="type hickory_resolver::TokioResolver">TokioResolver</a>  
A Resolver used with Tokio
