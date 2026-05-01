Crate openssl Copy item path

<span class="sub-heading"><a href="../src/openssl/lib.rs.html#1-259" class="src">Source</a>
</span>

Expand description

Bindings to OpenSSL

This crate provides a safe interface to the popular OpenSSL cryptography
library. OpenSSL versions 1.0.2 through 3.x.x and LibreSSL versions 3.5
through 4.2.x are supported.

# <a href="#building" class="doc-anchor">§</a>Building

Both OpenSSL libraries and headers are required to build this crate.
There are multiple options available to locate OpenSSL.

## <a href="#vendored" class="doc-anchor">§</a>Vendored

If the `vendored` Cargo feature is enabled, the `openssl-src` crate will
be used to compile and statically link to a copy of OpenSSL. The build
process requires a C compiler, perl (and perl-core), and make. The
OpenSSL version will generally track the newest OpenSSL release, and
changes to the version are *not* considered breaking changes.

``` language-toml
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```

The vendored copy will be configured to automatically find a
configuration and root certificates at `/usr/local/ssl`. This path can
be overridden with an environment variable (see the manual section
below). Alternatively, the `openssl-probe` crate can be used to find
root certificates at runtime.

## <a href="#automatic" class="doc-anchor">§</a>Automatic

The `openssl-sys` crate will automatically detect OpenSSL installations
via Homebrew on macOS and vcpkg on Windows. Additionally, it will use
`pkg-config` on Unix-like systems to find the system installation.

``` language-not_rust
# macOS (Homebrew)
$ brew install openssl@3

# macOS (MacPorts)
$ sudo port install openssl

# macOS (pkgsrc)
$ sudo pkgin install openssl

# Arch Linux
$ sudo pacman -S pkgconf openssl

# Debian and Ubuntu
$ sudo apt-get install pkg-config libssl-dev

# Fedora
$ sudo dnf install pkgconf perl-FindBin perl-IPC-Cmd openssl-devel

# Alpine Linux
$ apk add pkgconf openssl-dev

# openSUSE
$ sudo zypper in libopenssl-devel
```

## <a href="#manual" class="doc-anchor">§</a>Manual

A set of environment variables can be used to point `openssl-sys`
towards an OpenSSL installation. They will override the automatic
detection logic.

-   `OPENSSL_DIR` - If specified, the directory of an OpenSSL
    installation. The directory should contain `lib` and `include`
    subdirectories containing the libraries and headers respectively.
-   `OPENSSL_LIB_DIR` and `OPENSSL_INCLUDE_DIR` - If specified, the
    directories containing the OpenSSL libraries and headers
    respectively. This can be used if the OpenSSL installation is split
    in a nonstandard directory layout.
-   `OPENSSL_STATIC` - If set, the crate will statically link to OpenSSL
    rather than dynamically link.
-   `OPENSSL_LIBS` - If set, a `:`-separated list of library names to
    link to (e.g. `ssl:crypto`). This can be used if nonstandard library
    names were used for whatever reason.
-   `OPENSSL_NO_VENDOR` - If set, always find OpenSSL in the system,
    even if the `vendored` feature is enabled.

If the `vendored` Cargo feature is enabled, the following environment
variable can also be used to further configure the OpenSSL build.

-   `OPENSSL_CONFIG_DIR` - If set, the copy of OpenSSL built by the
    `openssl-src` crate will be configured to look for configuration
    files and root certificates in this directory.

Additionally, these variables can be prefixed with the upper-cased
target architecture (e.g. `X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR`), which
can be useful when cross compiling.

# <a href="#feature-detection" class="doc-anchor">§</a>Feature Detection

APIs have been added to and removed from the various supported OpenSSL
versions, and this library exposes the functionality available in the
version being linked against. This means that methods, constants, and
even modules will be present when building against one version of
OpenSSL but not when building against another! APIs will document any
version-specific availability restrictions.

A build script can be used to detect the OpenSSL or LibreSSL version at
compile time if needed. The `openssl-sys` crate propagates the version
via the `DEP_OPENSSL_VERSION_NUMBER` and
`DEP_OPENSSL_LIBRESSL_VERSION_NUMBER` environment variables to build
scripts. The version format is a hex-encoding of the OpenSSL release
version: `0xMNNFFPPS`. For example, version 1.0.2g’s encoding is
`0x1_00_02_07_0`.

For example, let’s say we want to adjust the TLSv1.3 cipher suites used
by a client, but also want to compile against OpenSSL versions that
don’t support TLSv1.3:

Cargo.toml:

``` language-toml
[dependencies]
openssl-sys = "0.9"
openssl = "0.10"
```

build.rs:

``` {.rust .rust-example-rendered}
use std::env;

fn main() {
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();

        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }
}
```

lib.rs:

``` {.rust .rust-example-rendered}
use openssl::ssl::{SslConnector, SslMethod};

let mut ctx = SslConnector::builder(SslMethod::tls()).unwrap();

// set_ciphersuites was added in OpenSSL 1.1.1, so we can only call it when linking against that version
#[cfg(openssl111)]
ctx.set_ciphersuites("TLS_AES_256_GCM_SHA384:TLS_AES_128_GCM_SHA256").unwrap();
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="aes/index.html" class="mod" title="mod openssl::aes">aes</a>  
Low level AES IGE and key wrapping functionality

<a href="asn1/index.html" class="mod" title="mod openssl::asn1">asn1</a>  
Defines the format of certificates

<a href="base64/index.html" class="mod"
title="mod openssl::base64">base64</a>  
Base64 encoding support.

<a href="bn/index.html" class="mod" title="mod openssl::bn">bn</a>  
BigNum implementation

<a href="cipher/index.html" class="mod"
title="mod openssl::cipher">cipher</a>  
Symmetric ciphers.

<a href="cipher_ctx/index.html" class="mod"
title="mod openssl::cipher_ctx">cipher_ctx</a>  
The symmetric encryption context.

<a href="cms/index.html" class="mod" title="mod openssl::cms">cms</a>  
SMIME implementation using CMS

<a href="conf/index.html" class="mod" title="mod openssl::conf">conf</a>  
Interface for processing OpenSSL configuration files.

<a href="derive/index.html" class="mod"
title="mod openssl::derive">derive</a>  
Shared secret derivation.

<a href="dh/index.html" class="mod" title="mod openssl::dh">dh</a>  
Diffie-Hellman key agreement.

<a href="dsa/index.html" class="mod" title="mod openssl::dsa">dsa</a>  
Digital Signatures

<a href="ec/index.html" class="mod" title="mod openssl::ec">ec</a>  
Elliptic Curve

<a href="ecdsa/index.html" class="mod"
title="mod openssl::ecdsa">ecdsa</a>  
Low level Elliptic Curve Digital Signature Algorithm (ECDSA) functions.

<a href="encrypt/index.html" class="mod"
title="mod openssl::encrypt">encrypt</a>  
Message encryption.

<a href="envelope/index.html" class="mod"
title="mod openssl::envelope">envelope</a>  
Envelope encryption.

<a href="error/index.html" class="mod"
title="mod openssl::error">error</a>  
Errors returned by OpenSSL library.

<a href="ex_data/index.html" class="mod"
title="mod openssl::ex_data">ex_data</a>  
<a href="hash/index.html" class="mod" title="mod openssl::hash">hash</a>  
Message digest (hash) computation support.

<a href="kdf/index.html" class="mod" title="mod openssl::kdf">kdf</a>  
<a href="lib_ctx/index.html" class="mod"
title="mod openssl::lib_ctx">lib_ctx</a>  
<a href="md/index.html" class="mod" title="mod openssl::md">md</a>  
Message digest algorithms.

<a href="md_ctx/index.html" class="mod"
title="mod openssl::md_ctx">md_ctx</a>  
The message digest context.

<a href="memcmp/index.html" class="mod"
title="mod openssl::memcmp">memcmp</a>  
Utilities to safely compare cryptographic values.

<a href="nid/index.html" class="mod" title="mod openssl::nid">nid</a>  
A collection of numerical identifiers for OpenSSL objects.

<a href="ocsp/index.html" class="mod" title="mod openssl::ocsp">ocsp</a>  
<a href="pkcs5/index.html" class="mod"
title="mod openssl::pkcs5">pkcs5</a>  
<a href="pkcs7/index.html" class="mod"
title="mod openssl::pkcs7">pkcs7</a>  
<a href="pkcs12/index.html" class="mod"
title="mod openssl::pkcs12">pkcs12</a>  
PKCS \#12 archives.

<a href="pkey/index.html" class="mod" title="mod openssl::pkey">pkey</a>  
Public/private key processing.

<a href="pkey_ctx/index.html" class="mod"
title="mod openssl::pkey_ctx">pkey_ctx</a>  
The asymmetric encryption context.

<a href="provider/index.html" class="mod"
title="mod openssl::provider">provider</a>  
<a href="rand/index.html" class="mod" title="mod openssl::rand">rand</a>  
Utilities for secure random number generation.

<a href="rsa/index.html" class="mod" title="mod openssl::rsa">rsa</a>  
Rivest–Shamir–Adleman cryptosystem

<a href="sha/index.html" class="mod" title="mod openssl::sha">sha</a>  
The SHA family of hashes.

<a href="sign/index.html" class="mod" title="mod openssl::sign">sign</a>  
Message signatures.

<a href="srtp/index.html" class="mod" title="mod openssl::srtp">srtp</a>  
<a href="ssl/index.html" class="mod" title="mod openssl::ssl">ssl</a>  
SSL/TLS support.

<a href="stack/index.html" class="mod"
title="mod openssl::stack">stack</a>  
<a href="string/index.html" class="mod"
title="mod openssl::string">string</a>  
<a href="symm/index.html" class="mod" title="mod openssl::symm">symm</a>  
High level interface to certain symmetric ciphers.

<a href="version/index.html" class="mod"
title="mod openssl::version">version</a>  
Build and version information.

<a href="x509/index.html" class="mod" title="mod openssl::x509">x509</a>  
The standard defining the format of public key certificates.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.init.html" class="fn" title="fn openssl::init">init</a>
