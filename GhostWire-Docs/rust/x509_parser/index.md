Crate x509\_parser Copy item path

<span class="sub-heading"><a href="../src/x509_parser/lib.rs.html#1-213" class="src">Source</a>
</span>

Expand description

[![License:
MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE-MIT)
[![Apache License
2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![docs.rs](https://docs.rs/x509-parser/badge.svg)](https://docs.rs/x509-parser)
[![crates.io](https://img.shields.io/crates/v/x509-parser.svg)](https://crates.io/crates/x509-parser)
[![Download
numbers](https://img.shields.io/crates/d/x509-parser.svg)](https://crates.io/crates/x509-parser)
[![Github
CI](https://github.com/rusticata/x509-parser/workflows/Continuous%20integration/badge.svg)](https://github.com/rusticata/x509-parser/actions)
[![Minimum rustc
version](https://img.shields.io/badge/rustc-1.63.0+-lightgray.svg)](#rust-version-requirements)

# <a href="#x509-parser" class="doc-anchor">§</a>X.509 Parser

A X.509 v3 ([RFC5280](https://tools.ietf.org/html/rfc5280)) parser,
implemented with the [nom](https://github.com/Geal/nom) parser
combinator framework.

It is written in pure Rust, fast, and makes extensive use of zero-copy.
A lot of care is taken to ensure security and safety of this crate,
including design (recursion limit, defensive programming), tests, and
fuzzing. It also aims to be panic-free.

The code is available on
[Github](https://github.com/rusticata/x509-parser) and is part of the
[Rusticata](https://github.com/rusticata) project.

Certificates are usually encoded in two main formats: PEM (usually the
most common format) or DER. A PEM-encoded certificate is a container,
storing a DER object. See the [`pem`](pem/index.html) module for more
documentation.

To decode a DER-encoded certificate, the main parsing method is
`X509Certificate::from_der` ( part of the
[`FromDer`](prelude/trait.FromDer.html) trait ), which builds a
[`X509Certificate`](certificate/struct.X509Certificate.html) object.

An alternative method is to use
[`X509CertificateParser`](certificate/struct.X509CertificateParser.html),
which allows specifying parsing options (for example, not automatically
parsing option contents).

The returned objects for parsers follow the definitions of the RFC. This
means that accessing fields is done by accessing struct members
recursively. Some helper functions are provided, for example
[`X509Certificate::issuer()`](certificate/struct.X509Certificate.html#method.issuer)
returns the same as accessing `<object>.tbs_certificate.issuer`.

For PEM-encoded certificates, use the [`pem`](pem/index.html) module.

# <a href="#examples" class="doc-anchor">§</a>Examples

Parsing a certificate in DER format:

``` {.rust .rust-example-rendered}
use x509_parser::prelude::*;

static IGCA_DER: &[u8] = include_bytes!("../assets/IGC_A.der");

let res = X509Certificate::from_der(IGCA_DER);
match res {
    Ok((rem, cert)) => {
        assert!(rem.is_empty());
        //
        assert_eq!(cert.version(), X509Version::V3);
    },
    _ => panic!("x509 parsing failed: {:?}", res),
}
```

To parse a CRL and print information about revoked certificates:

``` {.rust .rust-example-rendered}
let res = CertificateRevocationList::from_der(DER);
match res {
    Ok((_rem, crl)) => {
        for revoked in crl.iter_revoked_certificates() {
            println!("Revoked certificate serial: {}", revoked.raw_serial_as_string());
            println!("  Reason: {}", revoked.reason_code().unwrap_or_default().1);
        }
    },
    _ => panic!("CRL parsing failed: {:?}", res),
}
```

See also `examples/print-cert.rs`.

# <a href="#features" class="doc-anchor">§</a>Features

-   The `verify` feature adds support for (cryptographic) signature
    verification, based on `ring`. It adds the
    [`X509Certificate::verify_signature()`](certificate/struct.X509Certificate.html#method.verify_signature)
    to `X509Certificate`.

``` {.rust .rust-example-rendered}
/// Cryptographic signature verification: returns true if certificate was signed by issuer
#[cfg(feature = "verify")]
pub fn check_signature(cert: &X509Certificate<'_>, issuer: &X509Certificate<'_>) -> bool {
    let issuer_public_key = issuer.public_key();
    cert
        .verify_signature(Some(issuer_public_key))
        .is_ok()
}
```

-   The `validate` features add methods to run more validation functions
    on the certificate structure and values using the
    [`Validate`](validate/trait.Validate.html) trait. It does not
    validate any cryptographic parameter (see `verify` above).

## <a href="#rust-version-requirements" class="doc-anchor">§</a>Rust version requirements

`x509-parser` requires **Rustc version 1.63 or greater**, based on
der-parser dependencies and for proc-macro attributes support.

Note that due to breaking changes in the `time` crate, a specific
version of this crate must be specified for compiler versions &lt;=
1.63: `cargo update -p time --precise 0.3.20`

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../der_parser/index.html" class="mod"
title="mod der_parser"><code>der_parser</code></a>`;`

`pub use der_parser::`<a href="../num_bigint/index.html" class="mod"
title="mod num_bigint"><code>num_bigint</code></a>`;`

`pub use `<a href="../nom/index.html" class="mod"
title="mod nom"><code>nom</code></a>`;`

`pub use `<a href="../oid_registry/index.html" class="mod"
title="mod oid_registry"><code>oid_registry</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="certificate/index.html" class="mod"
title="mod x509_parser::certificate">certificate</a>  
X.509 Certificate object definitions and operations

<a href="certification_request/index.html" class="mod"
title="mod x509_parser::certification_request">certification_request</a>  
<a href="cri_attributes/index.html" class="mod"
title="mod x509_parser::cri_attributes">cri_attributes</a>  
<a href="error/index.html" class="mod"
title="mod x509_parser::error">error</a>  
X.509 errors

<a href="extensions/index.html" class="mod"
title="mod x509_parser::extensions">extensions</a>  
X.509 Extensions objects and types

<a href="objects/index.html" class="mod"
title="mod x509_parser::objects">objects</a>  
X.509 helper objects definitions and registry

<a href="pem/index.html" class="mod"
title="mod x509_parser::pem">pem</a>  
Decoding functions for PEM-encoded data

<a href="prelude/index.html" class="mod"
title="mod x509_parser::prelude">prelude</a>  
A “prelude” for users of the x509-parser crate.

<a href="public_key/index.html" class="mod"
title="mod x509_parser::public_key">public_key</a>  
<a href="revocation_list/index.html" class="mod"
title="mod x509_parser::revocation_list">revocation_list</a>  
<a href="signature_algorithm/index.html" class="mod"
title="mod x509_parser::signature_algorithm">signature_algorithm</a>  
<a href="signature_value/index.html" class="mod"
title="mod x509_parser::signature_value">signature_value</a>  
<a href="time/index.html" class="mod"
title="mod x509_parser::time">time</a>  
<a href="utils/index.html" class="mod"
title="mod x509_parser::utils">utils</a>  
<a href="verify/index.html" class="mod"
title="mod x509_parser::verify">verify</a>  
<a href="x509/index.html" class="mod"
title="mod x509_parser::x509">x509</a>  
X.509 objects and types

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.parse_crl_der.html" class="fn"
title="fn x509_parser::parse_crl_der">parse_crl_der</a><span class="stab deprecated" title="">Deprecated</span>  
Parse a DER-encoded X.509 v2 CRL, and return the remaining of the input
and the built object.

<a href="fn.parse_x509_certificate.html" class="fn"
title="fn x509_parser::parse_x509_certificate">parse_x509_certificate</a>  
Parse a **DER-encoded** X.509 Certificate, and return the remaining of
the input and the built object.

<a href="fn.parse_x509_crl.html" class="fn"
title="fn x509_parser::parse_x509_crl">parse_x509_crl</a>  
Parse a DER-encoded X.509 v2 CRL, and return the remaining of the input
and the built object.

<a href="fn.parse_x509_der.html" class="fn"
title="fn x509_parser::parse_x509_der">parse_x509_der</a><span class="stab deprecated" title="">Deprecated</span>  
Parse a DER-encoded X.509 Certificate, and return the remaining of the
input and the built
