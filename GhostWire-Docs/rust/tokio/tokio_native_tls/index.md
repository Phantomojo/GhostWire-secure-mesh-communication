Crate tokio\_native\_tls Copy item path

<span class="sub-heading"><a href="../src/tokio_native_tls/lib.rs.html#1-384"
class="src">Source</a> </span>

Expand description

Async TLS streams

This library is an implementation of TLS streams using the most
appropriate system library by default for negotiating the connection.
That is, on Windows this library uses SChannel, on OSX it uses
SecureTransport, and on other platforms it uses OpenSSL.

Each TLS stream implements the `Read` and `Write` traits to interact and
interoperate with the rest of the futures I/O ecosystem. Client
connections initiated from this crate verify hostnames automatically and
by default.

This crate primarily exports this ability through two newtypes,
`TlsConnector` and `TlsAcceptor`. These newtypes augment the
functionality provided by the `native-tls` crate, on which this crate is
built. Configuration of TLS parameters is still primarily done through
the `native-tls` crate.

# Modules<a href="#modules" class="anchor">§</a>

<a href="native_tls/index.html" class="mod"
title="mod tokio_native_tls::native_tls">native_tls</a>  
re-export native\_tls

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AllowStd.html" class="struct"
title="struct tokio_native_tls::AllowStd">AllowStd</a>  
An intermediate wrapper for the inner stream `S`.

<a href="struct.TlsAcceptor.html" class="struct"
title="struct tokio_native_tls::TlsAcceptor">TlsAcceptor</a>  
A wrapper around a `native_tls::TlsAcceptor`, providing an async
`accept` method.

<a href="struct.TlsConnector.html" class="struct"
title="struct tokio_native_tls::TlsConnector">TlsConnector</a>  
A wrapper around a `native_tls::TlsConnector`, providing an async
`connect` method.

<a href="struct.TlsStream.html" class="struct"
title="struct tokio_native_tls::TlsStream">TlsStream</a>  
A wrapper around an underlying raw stream which implements the TLS or
SSL protocol.
