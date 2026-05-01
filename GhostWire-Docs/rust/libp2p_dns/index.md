Crate libp2p\_dns Copy item path

<span class="sub-heading"><a href="../src/libp2p_dns/lib.rs.html#21-848" class="src">Source</a>
</span>

Expand description

# <a href="#dns-name-resolution" class="doc-anchor">§</a>[DNS name resolution](https://github.com/libp2p/specs/blob/master/addressing/README.md#ip-and-name-resolution)

[`Transport`](struct.Transport.html "struct libp2p_dns::Transport") for
libp2p.

This crate provides the type
[`tokio::Transport`](tokio/type.Transport.html "type libp2p_dns::tokio::Transport")
based on
[`hickory_resolver::TokioResolver`](../hickory_resolver/resolver/type.TokioResolver.html "type hickory_resolver::resolver::TokioResolver").

A [`Transport`](struct.Transport.html "struct libp2p_dns::Transport") is
an address-rewriting
[`libp2p_core::Transport`](../libp2p_core/transport/trait.Transport.html "trait libp2p_core::transport::Transport")
wrapper around an inner `Transport`. The composed transport behaves like
the inner transport, except that
[`libp2p_core::Transport::dial`](../libp2p_core/transport/trait.Transport.html#tymethod.dial "method libp2p_core::transport::Transport::dial")
resolves `/dns/...`, `/dns4/...`, `/dns6/...` and `/dnsaddr/...`
components of the given `Multiaddr` through a DNS, replacing them with
the resolved protocols (typically TCP/IP).

The
[`tokio::Transport`](tokio/type.Transport.html "type libp2p_dns::tokio::Transport")
is enabled by default under the `tokio` feature. Tokio users can
furthermore opt-in to the `tokio-dns-over-rustls` and
`tokio-dns-over-https-rustls` features. For more information about these
features, please refer to the documentation of
[trust-dns-resolver](https://docs.rs/trust-dns-resolver/latest/trust_dns_resolver/#dns-over-tls-and-dns-over-https).
Alternative runtimes or resolvers can be used though a manual
implementation of \[`Resolver`\].

On Unix systems, if no custom configuration is given,
[trust-dns-resolver](https://docs.rs/trust-dns-resolver/latest/trust_dns_resolver/#dns-over-tls-and-dns-over-https)
will try to parse the `/etc/resolv.conf` file. This approach comes with
a few caveats to be aware of:

1.  This fails (panics even!) if `/etc/resolv.conf` does not exist. This
    is the case on all versions of Android.
2.  DNS configuration is only evaluated during startup. Runtime changes
    are thus ignored.
3.  DNS resolution is obviously done in process and consequently not
    using any system APIs (like libc’s `gethostbyname`). Again this is
    problematic on platforms like Android, where there’s a lot of
    complexity hidden behind the system APIs.

If the implementation requires different characteristics, one should
consider providing their own implementation of
[`Transport`](struct.Transport.html "struct libp2p_dns::Transport") or
use platform specific APIs to extract the host’s DNS configuration (if
possible) and provide a custom
[`ResolverConfig`](struct.ResolverConfig.html "struct libp2p_dns::ResolverConfig").

# Modules<a href="#modules" class="anchor">§</a>

<a href="tokio/index.html" class="mod"
title="mod libp2p_dns::tokio">tokio</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ResolveError.html" class="struct"
title="struct libp2p_dns::ResolveError">ResolveError</a>  
The error type for errors that get returned in the crate

<a href="struct.ResolverConfig.html" class="struct"
title="struct libp2p_dns::ResolverConfig">ResolverConfig</a>  
Configuration for the upstream nameservers to use for resolution

<a href="struct.ResolverOpts.html" class="struct"
title="struct libp2p_dns::ResolverOpts">ResolverOpts</a>  
Configuration for the Resolver

<a href="struct.Transport.html" class="struct"
title="struct libp2p_dns::Transport">Transport</a>  
A [`Transport`](struct.Transport.html "struct libp2p_dns::Transport")
for performing DNS lookups when dialing `Multiaddr`esses. You shouldn’t
need to use this type directly. Use
[`tokio::Transport`](tokio/type.Transport.html "type libp2p_dns::tokio::Transport")
instead.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum libp2p_dns::Error">Error</a>  
The possible errors of a
[`Transport`](struct.Transport.html "struct libp2p_dns::Transport")
wrapped transport.

<a href="enum.ResolveErrorKind.html" class="enum"
title="enum libp2p_dns::ResolveErrorKind">ResolveErrorKind</a>  
The error kind for errors that get returned in the crate
