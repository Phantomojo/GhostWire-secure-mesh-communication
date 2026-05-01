Crate rustls\_native\_certs Copy item path

<span class="sub-heading"><a href="../src/rustls_native_certs/lib.rs.html#1-515"
class="src">Source</a> </span>

Expand description

rustls-native-certs allows rustls to use the platform’s native
certificate store when operating as a TLS client.

It provides a single function
[`load_native_certs()`](fn.load_native_certs.html "fn rustls_native_certs::load_native_certs"),
which returns a collection of certificates found by reading the
platform-native certificate store.

If the SSL\_CERT\_FILE environment variable is set, certificates (in PEM
format) are read from that file instead.

If you want to load these certificates into a `rustls::RootCertStore`,
you’ll likely want to do something like this:

``` {.rust .rust-example-rendered}
let mut roots = rustls::RootCertStore::empty();
for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
    roots.add(cert).unwrap();
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CertificateResult.html" class="struct"
title="struct rustls_native_certs::CertificateResult">CertificateResult</a>

Results from trying to load certificates from the platform’s native
store.

<a href="struct.Error.html" class="struct"
title="struct rustls_native_certs::Error">Error</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ErrorKind.html" class="enum"
title="enum rustls_native_certs::ErrorKind">ErrorKind</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.load_certs_from_paths.html" class="fn"
title="fn rustls_native_certs::load_certs_from_paths">load_certs_from_paths</a>  
Load certificates from the given paths.

<a href="fn.load_native_certs.html" class="fn"
title="fn rustls_native_certs::load_native_certs">load_native_certs</a>  
Load root certificates found in the platform’s native certificate store.
