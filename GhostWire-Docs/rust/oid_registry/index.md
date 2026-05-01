Crate oid\_registry Copy item path

<span class="sub-heading"><a href="../src/oid_registry/lib.rs.html#1-291" class="src">Source</a>
</span>

Expand description

[![License:
MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE-MIT)
[![Apache License
2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![docs.rs](https://docs.rs/oid-registry/badge.svg)](https://docs.rs/oid-registry)
[![crates.io](https://img.shields.io/crates/v/oid-registry.svg)](https://crates.io/crates/oid-registry)
[![Github
CI](https://github.com/rusticata/oid-registry/workflows/Continuous%20integration/badge.svg)](https://github.com/rusticata/oid-registry/actions)
[![Minimum rustc
version](https://img.shields.io/badge/rustc-1.63.0+-lightgray.svg)](#rust-version-requirements)

# <a href="#oid-registry" class="doc-anchor">§</a>OID Registry

This crate is a helper crate, containing a database of OID objects.
These objects are intended for use when manipulating ASN.1 grammars and
BER/DER encodings, for example.

This crate provides only a simple registry (similar to a `HashMap`) by
default. This object can be used to get names and descriptions from OID.

This crate provides default lists of known OIDs, that can be selected
using the build features. By default, the registry has no feature
enabled, to avoid embedding a huge database in crates.

It also declares constants for most of these OIDs.

``` {.rust .rust-example-rendered}
use oid_registry::OidRegistry;

let mut registry = OidRegistry::default()
    .with_crypto() // only if the 'crypto' feature is enabled
;

let e = registry.get(&oid_registry::OID_PKCS1_SHA256WITHRSA);
if let Some(entry) = e {
    // get sn: sha256WithRSAEncryption
    println!("sn: {}", entry.sn());
    // get description: SHA256 with RSA encryption
    println!("description: {}", entry.description());
}
```

## <a href="#extending-the-registry" class="doc-anchor">§</a>Extending the registry

These provided lists are often incomplete, or may lack some specific
OIDs. This is why the registry allows adding new entries after
construction:

``` {.rust .rust-example-rendered}
use asn1_rs::oid;
use oid_registry::{OidEntry, OidRegistry};

let mut registry = OidRegistry::default();

// entries can be added by creating an OidEntry object:
let entry = OidEntry::new("shortName", "description");
registry.insert(oid!(1.2.3.4), entry);

// when using static strings, a tuple can also be used directly for the entry:
registry.insert(oid!(1.2.3.5), ("shortName", "A description"));
```

## <a href="#versions-and-compatibility-with-asn1-rs"
class="doc-anchor">§</a>Versions and compatibility with `asn1-rs`

Versions of `oid-registry` must be chosen specifically, to depend on a
precise version of `asn1-rs`. The following table summarizes the
matching versions:

-   `oid-registry` 0.7.x depends on `asn1-rs` 0.6.0
-   `oid-registry` 0.6.x depends on `asn1-rs` 0.5.0
-   `oid-registry` 0.5.x depends on `asn1-rs` 0.4.0

## <a href="#contributing-oids" class="doc-anchor">§</a>Contributing OIDs

All OID values, constants, and features are derived from files in the
`assets` directory in the build script (see `build.rs`). See `load_file`
for documentation of the file format.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../asn1_rs/index.html" class="mod"
title="mod asn1_rs"><code>asn1_rs</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.LoadedEntry.html" class="struct"
title="struct oid_registry::LoadedEntry">LoadedEntry</a>  
Temporary structure, created when reading a file containing OID
declarations

<a href="struct.Oid.html" class="struct"
title="struct oid_registry::Oid">Oid</a>  
Object ID (OID) representation which can be relative or non-relative. An
example for an OID in string representation is `"1.2.840.113549.1.1.5"`.

<a href="struct.OidEntry.html" class="struct"
title="struct oid_registry::OidEntry">OidEntry</a>  
An entry stored in the OID registry

<a href="struct.OidRegistry.html" class="struct"
title="struct oid_registry::OidRegistry">OidRegistry</a>  
Registry of known OIDs

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MS_CTL.html" class="constant"
title="constant oid_registry::MS_CTL">MS_CTL</a>  
1.3.6.1.4.1.311.10.1

<a href="constant.MS_JURISDICTION_COUNTRY.html" class="constant"
title="constant oid_registry::MS_JURISDICTION_COUNTRY">MS_JURISDICTION_COUNTRY</a>  
1.3.6.1.4.1.311.60.2.1.3

<a href="constant.MS_JURISDICTION_LOCALITY.html" class="constant"
title="constant oid_registry::MS_JURISDICTION_LOCALITY">MS_JURISDICTION_LOCALITY</a>  
1.3.6.1.4.1.311.60.2.1.1

<a href="constant.MS_JURISDICTION_STATE_OR_PROVINCE.html"
class="constant"
title="constant oid_registry::MS_JURISDICTION_STATE_OR_PROVINCE">MS_JURISDICTION_STATE_OR_PROVINCE</a>  
1.3.6.1.4.1.311.60.2.1.2

<a href="constant.OID_CT_LIST_SCT.html" class="constant"
title="constant oid_registry::OID_CT_LIST_SCT">OID_CT_LIST_SCT</a>  
1.3.6.1.4.1.11129.2.4.2

<a href="constant.OID_DOMAIN_COMPONENT.html" class="constant"
title="constant oid_registry::OID_DOMAIN_COMPONENT">OID_DOMAIN_COMPONENT</a>  
0.9.2342.19200300.100.1.25

<a href="constant.OID_EC_P256.html" class="constant"
title="constant oid_registry::OID_EC_P256">OID_EC_P256</a>  
1.2.840.10045.3.1.7

<a href="constant.OID_GOST_R3410_2001.html" class="constant"
title="constant oid_registry::OID_GOST_R3410_2001">OID_GOST_R3410_2001</a>  
1.2.643.2.2.19

<a href="constant.OID_HASH_SHA1.html" class="constant"
title="constant oid_registry::OID_HASH_SHA1">OID_HASH_SHA1</a>  
1.3.14.3.2.26

<a href="constant.OID_KDF_SHA1_SINGLE.html" class="constant"
title="constant oid_registry::OID_KDF_SHA1_SINGLE">OID_KDF_SHA1_SINGLE</a>  
1.3.133.16.840.63.0.2

<a href="constant.OID_KEY_TYPE_DSA.html" class="constant"
title="constant oid_registry::OID_KEY_TYPE_DSA">OID_KEY_TYPE_DSA</a>  
1.2.840.10040.4.1

<a href="constant.OID_KEY_TYPE_EC_PUBLIC_KEY.html" class="constant"
title="constant oid_registry::OID_KEY_TYPE_EC_PUBLIC_KEY">OID_KEY_TYPE_EC_PUBLIC_KEY</a>  
1.2.840.10045.2.1

<a href="constant.OID_KEY_TYPE_GOST_R3410_2012_256.html"
class="constant"
title="constant oid_registry::OID_KEY_TYPE_GOST_R3410_2012_256">OID_KEY_TYPE_GOST_R3410_2012_256</a>  
1.2.643.7.1.1.1.1

<a href="constant.OID_KEY_TYPE_GOST_R3410_2012_512.html"
class="constant"
title="constant oid_registry::OID_KEY_TYPE_GOST_R3410_2012_512">OID_KEY_TYPE_GOST_R3410_2012_512</a>  
1.2.643.7.1.1.1.2

<a href="constant.OID_MD5_WITH_RSA.html" class="constant"
title="constant oid_registry::OID_MD5_WITH_RSA">OID_MD5_WITH_RSA</a>  
1.3.14.3.2.25

<a href="constant.OID_NIST_EC_P384.html" class="constant"
title="constant oid_registry::OID_NIST_EC_P384">OID_NIST_EC_P384</a>  
1.3.132.0.34

<a href="constant.OID_NIST_EC_P521.html" class="constant"
title="constant oid_registry::OID_NIST_EC_P521">OID_NIST_EC_P521</a>  
1.3.132.0.35

<a href="constant.OID_NIST_ENC_AES256_CBC.html" class="constant"
title="constant oid_registry::OID_NIST_ENC_AES256_CBC">OID_NIST_ENC_AES256_CBC</a>  
2.16.840.1.101.3.4.1.42

<a href="constant.OID_NIST_HASH_SHA256.html" class="constant"
title="constant oid_registry::OID_NIST_HASH_SHA256">OID_NIST_HASH_SHA256</a>  
2.16.840.1.101.3.4.2.1

<a href="constant.OID_NIST_HASH_SHA384.html" class="constant"
title="constant oid_registry::OID_NIST_HASH_SHA384">OID_NIST_HASH_SHA384</a>  
2.16.840.1.101.3.4.2.2

<a href="constant.OID_NIST_HASH_SHA512.html" class="constant"
title="constant oid_registry::OID_NIST_HASH_SHA512">OID_NIST_HASH_SHA512</a>  
2.16.840.1.101.3.4.2.3

<a href="constant.OID_PKCS1_MD2WITHRSAENC.html" class="constant"
title="constant oid_registry::OID_PKCS1_MD2WITHRSAENC">OID_PKCS1_MD2WITHRSAENC</a>  
1.2.840.113549.1.1.2

<a href="constant.OID_PKCS1_MD4WITHRSAENC.html" class="constant"
title="constant oid_registry::OID_PKCS1_MD4WITHRSAENC">OID_PKCS1_MD4WITHRSAENC</a>  
1.2.840.113549.1.1.3

<a href="constant.OID_PKCS1_MD5WITHRSAENC.html" class="constant"
title="constant oid_registry::OID_PKCS1_MD5WITHRSAENC">OID_PKCS1_MD5WITHRSAENC</a>  
1.2.840.113549.1.1.4

<a href="constant.OID_PKCS1_RSAENCRYPTION.html" class="constant"
title="constant oid_registry::OID_PKCS1_RSAENCRYPTION">OID_PKCS1_RSAENCRYPTION</a>  
1.2.840.113549.1.1.1

<a href="constant.OID_PKCS1_RSASSAPSS.html" class="constant"
title="constant oid_registry::OID_PKCS1_RSASSAPSS">OID_PKCS1_RSASSAPSS</a>  
1.2.840.113549.1.1.10

<a href="constant.OID_PKCS1_SHA1WITHRSA.html" class="constant"
title="constant oid_registry::OID_PKCS1_SHA1WITHRSA">OID_PKCS1_SHA1WITHRSA</a>  
1.2.840.113549.1.1.5

<a href="constant.OID_PKCS1_SHA224WITHRSA.html" class="constant"
title="constant oid_registry::OID_PKCS1_SHA224WITHRSA">OID_PKCS1_SHA224WITHRSA</a>  
1.2.840.113549.1.1.14

<a href="constant.OID_PKCS1_SHA256WITHRSA.html" class="constant"
title="constant oid_registry::OID_PKCS1_SHA256WITHRSA">OID_PKCS1_SHA256WITHRSA</a>  
1.2.840.113549.1.1.11

<a href="constant.OID_PKCS1_SHA384WITHRSA.html" class="constant"
title="constant oid_registry::OID_PKCS1_SHA384WITHRSA">OID_PKCS1_SHA384WITHRSA</a>  
1.2.840.113549.1.1.12

<a href="constant.OID_PKCS1_SHA512WITHRSA.html" class="constant"
title="constant oid_registry::OID_PKCS1_SHA512WITHRSA">OID_PKCS1_SHA512WITHRSA</a>  
1.2.840.113549.1.1.13

<a href="constant.OID_PKCS7_ID_DATA.html" class="constant"
title="constant oid_registry::OID_PKCS7_ID_DATA">OID_PKCS7_ID_DATA</a>  
1.2.840.113549.1.7.1

<a href="constant.OID_PKCS7_ID_DIGESTED_DATA.html" class="constant"
title="constant oid_registry::OID_PKCS7_ID_DIGESTED_DATA">OID_PKCS7_ID_DIGESTED_DATA</a>  
1.2.840.113549.1.7.5

<a href="constant.OID_PKCS7_ID_ENCRYPTED_DATA.html" class="constant"
title="constant oid_registry::OID_PKCS7_ID_ENCRYPTED_DATA">OID_PKCS7_ID_ENCRYPTED_DATA</a>  
1.2.840.113549.1.7.6

<a href="constant.OID_PKCS7_ID_ENVELOPED_DATA.html" class="constant"
title="constant oid_registry::OID_PKCS7_ID_ENVELOPED_DATA">OID_PKCS7_ID_ENVELOPED_DATA</a>  
1.2.840.113549.1.7.3

<a href="constant.OID_PKCS7_ID_SIGNED_DATA.html" class="constant"
title="constant oid_registry::OID_PKCS7_ID_SIGNED_DATA">OID_PKCS7_ID_SIGNED_DATA</a>  
1.2.840.113549.1.7.2

<a href="constant.OID_PKCS7_ID_SIGNED_ENVELOPED_DATA.html"
class="constant"
title="constant oid_registry::OID_PKCS7_ID_SIGNED_ENVELOPED_DATA">OID_PKCS7_ID_SIGNED_ENVELOPED_DATA</a>  
1.2.840.113549.1.7.4

<a href="constant.OID_PKCS9_CHALLENGE_PASSWORD.html" class="constant"
title="constant oid_registry::OID_PKCS9_CHALLENGE_PASSWORD">OID_PKCS9_CHALLENGE_PASSWORD</a>  
1.2.840.113549.1.9.7

<a href="constant.OID_PKCS9_CONTENT_TYPE.html" class="constant"
title="constant oid_registry::OID_PKCS9_CONTENT_TYPE">OID_PKCS9_CONTENT_TYPE</a>  
1.2.840.113549.1.9.3

<a href="constant.OID_PKCS9_EMAIL_ADDRESS.html" class="constant"
title="constant oid_registry::OID_PKCS9_EMAIL_ADDRESS">OID_PKCS9_EMAIL_ADDRESS</a>  
1.2.840.113549.1.9.1

<a href="constant.OID_PKCS9_EXTENSION_REQUEST.html" class="constant"
title="constant oid_registry::OID_PKCS9_EXTENSION_REQUEST">OID_PKCS9_EXTENSION_REQUEST</a>  
1.2.840.113549.1.9.14

<a href="constant.OID_PKCS9_FRIENDLY_NAME.html" class="constant"
title="constant oid_registry::OID_PKCS9_FRIENDLY_NAME">OID_PKCS9_FRIENDLY_NAME</a>  
1.2.840.113549.1.9.20

<a href="constant.OID_PKCS9_ID_MESSAGE_DIGEST.html" class="constant"
title="constant oid_registry::OID_PKCS9_ID_MESSAGE_DIGEST">OID_PKCS9_ID_MESSAGE_DIGEST</a>  
1.2.840.113549.1.9.4

<a href="constant.OID_PKCS9_SIGNING_TIME.html" class="constant"
title="constant oid_registry::OID_PKCS9_SIGNING_TIME">OID_PKCS9_SIGNING_TIME</a>  
1.2.840.113549.1.9.5

<a href="constant.OID_PKCS9_SMIME_CAPABILITIES.html" class="constant"
title="constant oid_registry::OID_PKCS9_SMIME_CAPABILITIES">OID_PKCS9_SMIME_CAPABILITIES</a>  
1.2.840.113549.1.9.15

<a href="constant.OID_PKCS9_UNSTRUCTURED_NAME.html" class="constant"
title="constant oid_registry::OID_PKCS9_UNSTRUCTURED_NAME">OID_PKCS9_UNSTRUCTURED_NAME</a>  
1.2.840.113549.1.9.2

<a href="constant.OID_PKCS12.html" class="constant"
title="constant oid_registry::OID_PKCS12">OID_PKCS12</a>  
1.2.840.113549.1.12

<a href="constant.OID_PKCS12_PBEIDS.html" class="constant"
title="constant oid_registry::OID_PKCS12_PBEIDS">OID_PKCS12_PBEIDS</a>  
1.2.840.113549.1.12.1

<a href="constant.OID_PKCS12_PBE_SHA1_2K_3DES_CBC.html" class="constant"
title="constant oid_registry::OID_PKCS12_PBE_SHA1_2K_3DES_CBC">OID_PKCS12_PBE_SHA1_2K_3DES_CBC</a>  
1.2.840.113549.1.12.1.4

<a href="constant.OID_PKCS12_PBE_SHA1_3K_3DES_CBC.html" class="constant"
title="constant oid_registry::OID_PKCS12_PBE_SHA1_3K_3DES_CBC">OID_PKCS12_PBE_SHA1_3K_3DES_CBC</a>  
1.2.840.113549.1.12.1.3

<a href="constant.OID_PKCS12_PBE_SHA1_40RC4.html" class="constant"
title="constant oid_registry::OID_PKCS12_PBE_SHA1_40RC4">OID_PKCS12_PBE_SHA1_40RC4</a>  
1.2.840.113549.1.12.1.2

<a href="constant.OID_PKCS12_PBE_SHA1_40RC2_CBC.html" class="constant"
title="constant oid_registry::OID_PKCS12_PBE_SHA1_40RC2_CBC">OID_PKCS12_PBE_SHA1_40RC2_CBC</a>  
1.2.840.113549.1.12.1.6

<a href="constant.OID_PKCS12_PBE_SHA1_128RC4.html" class="constant"
title="constant oid_registry::OID_PKCS12_PBE_SHA1_128RC4">OID_PKCS12_PBE_SHA1_128RC4</a>  
1.2.840.113549.1.12.1.1

<a href="constant.OID_PKCS12_PBE_SHA1_128RC2_CBC.html" class="constant"
title="constant oid_registry::OID_PKCS12_PBE_SHA1_128RC2_CBC">OID_PKCS12_PBE_SHA1_128RC2_CBC</a>  
1.2.840.113549.1.12.1.5

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_CA_ISSUERS.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_CA_ISSUERS">OID_PKIX_ACCESS_DESCRIPTOR_CA_ISSUERS</a>  
1.3.6.1.5.5.7.48.2

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_CA_REPOSITORY.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_CA_REPOSITORY">OID_PKIX_ACCESS_DESCRIPTOR_CA_REPOSITORY</a>  
1.3.6.1.5.5.7.48.5

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_CMC.html" class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_CMC">OID_PKIX_ACCESS_DESCRIPTOR_CMC</a>  
1.3.6.1.5.5.7.48.12

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_DVCS.html" class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_DVCS">OID_PKIX_ACCESS_DESCRIPTOR_DVCS</a>  
1.3.6.1.5.5.7.48.4

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CERTS.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CERTS">OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CERTS</a>  
1.3.6.1.5.5.7.48.6

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CRLS.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CRLS">OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CRLS</a>  
1.3.6.1.5.5.7.48.7

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_OCSP.html" class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_OCSP">OID_PKIX_ACCESS_DESCRIPTOR_OCSP</a>  
1.3.6.1.5.5.7.48.1

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_RPKI_MANIFEST.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_RPKI_MANIFEST">OID_PKIX_ACCESS_DESCRIPTOR_RPKI_MANIFEST</a>  
1.3.6.1.5.5.7.48.10

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_RPKI_NOTIFY.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_RPKI_NOTIFY">OID_PKIX_ACCESS_DESCRIPTOR_RPKI_NOTIFY</a>  
1.3.6.1.5.5.7.48.13

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_SIGNED_OBJECT.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_SIGNED_OBJECT">OID_PKIX_ACCESS_DESCRIPTOR_SIGNED_OBJECT</a>  
1.3.6.1.5.5.7.48.11

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_STIRTNLIST.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_STIRTNLIST">OID_PKIX_ACCESS_DESCRIPTOR_STIRTNLIST</a>  
1.3.6.1.5.5.7.48.14

<a href="constant.OID_PKIX_ACCESS_DESCRIPTOR_TIMESTAMPING.html"
class="constant"
title="constant oid_registry::OID_PKIX_ACCESS_DESCRIPTOR_TIMESTAMPING">OID_PKIX_ACCESS_DESCRIPTOR_TIMESTAMPING</a>  
1.3.6.1.5.5.7.48.3

<a href="constant.OID_PKIX_AUTHORITY_INFO_ACCESS.html" class="constant"
title="constant oid_registry::OID_PKIX_AUTHORITY_INFO_ACCESS">OID_PKIX_AUTHORITY_INFO_ACCESS</a>  
1.3.6.1.5.5.7.1.1

<a href="constant.OID_SHA1_WITH_RSA.html" class="constant"
title="constant oid_registry::OID_SHA1_WITH_RSA">OID_SHA1_WITH_RSA</a>  
1.3.14.3.2.29

<a href="constant.OID_SIG_DSA_WITH_SHA1.html" class="constant"
title="constant oid_registry::OID_SIG_DSA_WITH_SHA1">OID_SIG_DSA_WITH_SHA1</a>  
1.2.840.10040.4.3

<a href="constant.OID_SIG_ECDSA_WITH_SHA224.html" class="constant"
title="constant oid_registry::OID_SIG_ECDSA_WITH_SHA224">OID_SIG_ECDSA_WITH_SHA224</a>  
1.2.840.10045.4.3.1

<a href="constant.OID_SIG_ECDSA_WITH_SHA256.html" class="constant"
title="constant oid_registry::OID_SIG_ECDSA_WITH_SHA256">OID_SIG_ECDSA_WITH_SHA256</a>  
1.2.840.10045.4.3.2

<a href="constant.OID_SIG_ECDSA_WITH_SHA384.html" class="constant"
title="constant oid_registry::OID_SIG_ECDSA_WITH_SHA384">OID_SIG_ECDSA_WITH_SHA384</a>  
1.2.840.10045.4.3.3

<a href="constant.OID_SIG_ECDSA_WITH_SHA512.html" class="constant"
title="constant oid_registry::OID_SIG_ECDSA_WITH_SHA512">OID_SIG_ECDSA_WITH_SHA512</a>  
1.2.840.10045.4.3.4

<a href="constant.OID_SIG_ED448.html" class="constant"
title="constant oid_registry::OID_SIG_ED448">OID_SIG_ED448</a>  
1.3.101.113

<a href="constant.OID_SIG_ED25519.html" class="constant"
title="constant oid_registry::OID_SIG_ED25519">OID_SIG_ED25519</a>  
1.3.101.112

<a href="constant.OID_SIG_GOST_R3410_2012_256.html" class="constant"
title="constant oid_registry::OID_SIG_GOST_R3410_2012_256">OID_SIG_GOST_R3410_2012_256</a>  
1.2.643.7.1.1.3.2

<a href="constant.OID_SIG_GOST_R3410_2012_512.html" class="constant"
title="constant oid_registry::OID_SIG_GOST_R3410_2012_512">OID_SIG_GOST_R3410_2012_512</a>  
1.2.643.7.1.1.3.3

<a href="constant.OID_SIG_GOST_R3411_94_WITH_R3410_2001.html"
class="constant"
title="constant oid_registry::OID_SIG_GOST_R3411_94_WITH_R3410_2001">OID_SIG_GOST_R3411_94_WITH_R3410_2001</a>  
1.2.643.2.2.3

<a href="constant.OID_SIG_RSA_RIPE_MD160.html" class="constant"
title="constant oid_registry::OID_SIG_RSA_RIPE_MD160">OID_SIG_RSA_RIPE_MD160</a>  
1.3.36.3.3.1.2

<a href="constant.OID_USERID.html" class="constant"
title="constant oid_registry::OID_USERID">OID_USERID</a>  
0.9.2342.19200300.100.1.1

<a href="constant.OID_X500.html" class="constant"
title="constant oid_registry::OID_X500">OID_X500</a>  
2.5

<a href="constant.OID_X509.html" class="constant"
title="constant oid_registry::OID_X509">OID_X509</a>  
2.5.4

<a href="constant.OID_X509_ALIASED_ENTRY_NAME.html" class="constant"
title="constant oid_registry::OID_X509_ALIASED_ENTRY_NAME">OID_X509_ALIASED_ENTRY_NAME</a>  
2.5.4.1

<a href="constant.OID_X509_BUSINESS_CATEGORY.html" class="constant"
title="constant oid_registry::OID_X509_BUSINESS_CATEGORY">OID_X509_BUSINESS_CATEGORY</a>  
2.5.4.15

<a href="constant.OID_X509_COMMON_NAME.html" class="constant"
title="constant oid_registry::OID_X509_COMMON_NAME">OID_X509_COMMON_NAME</a>  
2.5.4.3

<a href="constant.OID_X509_COUNTRY_NAME.html" class="constant"
title="constant oid_registry::OID_X509_COUNTRY_NAME">OID_X509_COUNTRY_NAME</a>  
2.5.4.6

<a href="constant.OID_X509_DESCRIPTION.html" class="constant"
title="constant oid_registry::OID_X509_DESCRIPTION">OID_X509_DESCRIPTION</a>  
2.5.4.13

<a href="constant.OID_X509_DN_QUALIFIER.html" class="constant"
title="constant oid_registry::OID_X509_DN_QUALIFIER">OID_X509_DN_QUALIFIER</a>  
2.5.4.46

<a href="constant.OID_X509_EXT_AUTHORITY_KEY_IDENTIFIER.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_AUTHORITY_KEY_IDENTIFIER">OID_X509_EXT_AUTHORITY_KEY_IDENTIFIER</a>  
2.5.29.35

<a href="constant.OID_X509_EXT_BASE_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_BASE_URL">OID_X509_EXT_BASE_URL</a>  
2.16.840.1.113730.1.2

<a href="constant.OID_X509_EXT_BASIC_CONSTRAINTS.html" class="constant"
title="constant oid_registry::OID_X509_EXT_BASIC_CONSTRAINTS">OID_X509_EXT_BASIC_CONSTRAINTS</a>  
2.5.29.19

<a href="constant.OID_X509_EXT_CA_CERT_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_CA_CERT_URL">OID_X509_EXT_CA_CERT_URL</a>  
2.16.840.1.113730.1.6

<a href="constant.OID_X509_EXT_CA_CRL_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_CA_CRL_URL">OID_X509_EXT_CA_CRL_URL</a>  
2.16.840.1.113730.1.5

<a href="constant.OID_X509_EXT_CA_POLICY_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_CA_POLICY_URL">OID_X509_EXT_CA_POLICY_URL</a>  
2.16.840.1.113730.1.8

<a href="constant.OID_X509_EXT_CA_REVOCATION_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_CA_REVOCATION_URL">OID_X509_EXT_CA_REVOCATION_URL</a>  
2.16.840.1.113730.1.4

<a href="constant.OID_X509_EXT_CERTIFICATE_POLICIES.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_CERTIFICATE_POLICIES">OID_X509_EXT_CERTIFICATE_POLICIES</a>  
2.5.29.32

<a href="constant.OID_X509_EXT_CERT_COMMENT.html" class="constant"
title="constant oid_registry::OID_X509_EXT_CERT_COMMENT">OID_X509_EXT_CERT_COMMENT</a>  
2.16.840.1.113730.1.13

<a href="constant.OID_X509_EXT_CERT_TYPE.html" class="constant"
title="constant oid_registry::OID_X509_EXT_CERT_TYPE">OID_X509_EXT_CERT_TYPE</a>  
2.16.840.1.113730.1.1

<a href="constant.OID_X509_EXT_CRL_DISTRIBUTION_POINTS.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_CRL_DISTRIBUTION_POINTS">OID_X509_EXT_CRL_DISTRIBUTION_POINTS</a>  
2.5.29.31

<a href="constant.OID_X509_EXT_CRL_NUMBER.html" class="constant"
title="constant oid_registry::OID_X509_EXT_CRL_NUMBER">OID_X509_EXT_CRL_NUMBER</a>  
2.5.29.20

<a href="constant.OID_X509_EXT_DELTA_CRL_INDICATOR.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_DELTA_CRL_INDICATOR">OID_X509_EXT_DELTA_CRL_INDICATOR</a>  
2.5.29.27

<a href="constant.OID_X509_EXT_ENTITY_LOGO.html" class="constant"
title="constant oid_registry::OID_X509_EXT_ENTITY_LOGO">OID_X509_EXT_ENTITY_LOGO</a>  
2.16.840.1.113730.1.10

<a href="constant.OID_X509_EXT_EXTENDED_KEY_USAGE.html" class="constant"
title="constant oid_registry::OID_X509_EXT_EXTENDED_KEY_USAGE">OID_X509_EXT_EXTENDED_KEY_USAGE</a>  
2.5.29.37

<a href="constant.OID_X509_EXT_FRESHEST_CRL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_FRESHEST_CRL">OID_X509_EXT_FRESHEST_CRL</a>  
2.5.29.46

<a href="constant.OID_X509_EXT_HOLD_INSTRUCTION_CODE.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_HOLD_INSTRUCTION_CODE">OID_X509_EXT_HOLD_INSTRUCTION_CODE</a>  
2.5.29.23

<a href="constant.OID_X509_EXT_HOMEPAGE_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_HOMEPAGE_URL">OID_X509_EXT_HOMEPAGE_URL</a>  
2.16.840.1.113730.1.9

<a href="constant.OID_X509_EXT_INHIBITANT_ANY_POLICY.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_INHIBITANT_ANY_POLICY">OID_X509_EXT_INHIBITANT_ANY_POLICY</a>  
2.5.29.54

<a href="constant.OID_X509_EXT_INVALIDITY_DATE.html" class="constant"
title="constant oid_registry::OID_X509_EXT_INVALIDITY_DATE">OID_X509_EXT_INVALIDITY_DATE</a>  
2.5.29.24

<a href="constant.OID_X509_EXT_ISSUER.html" class="constant"
title="constant oid_registry::OID_X509_EXT_ISSUER">OID_X509_EXT_ISSUER</a>  
2.5.29.29

<a href="constant.OID_X509_EXT_ISSUER_ALT_NAME.html" class="constant"
title="constant oid_registry::OID_X509_EXT_ISSUER_ALT_NAME">OID_X509_EXT_ISSUER_ALT_NAME</a>  
2.5.29.18

<a href="constant.OID_X509_EXT_ISSUER_DISTRIBUTION_POINT.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_ISSUER_DISTRIBUTION_POINT">OID_X509_EXT_ISSUER_DISTRIBUTION_POINT</a>  
2.5.29.28

<a href="constant.OID_X509_EXT_KEY_USAGE.html" class="constant"
title="constant oid_registry::OID_X509_EXT_KEY_USAGE">OID_X509_EXT_KEY_USAGE</a>  
2.5.29.15

<a href="constant.OID_X509_EXT_NAME_CONSTRAINTS.html" class="constant"
title="constant oid_registry::OID_X509_EXT_NAME_CONSTRAINTS">OID_X509_EXT_NAME_CONSTRAINTS</a>  
2.5.29.30

<a href="constant.OID_X509_EXT_POLICY_CONSTRAINTS.html" class="constant"
title="constant oid_registry::OID_X509_EXT_POLICY_CONSTRAINTS">OID_X509_EXT_POLICY_CONSTRAINTS</a>  
2.5.29.36

<a href="constant.OID_X509_EXT_POLICY_MAPPINGS.html" class="constant"
title="constant oid_registry::OID_X509_EXT_POLICY_MAPPINGS">OID_X509_EXT_POLICY_MAPPINGS</a>  
2.5.29.33

<a href="constant.OID_X509_EXT_PRIVATE_KEY_USAGE_PERIOD.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_PRIVATE_KEY_USAGE_PERIOD">OID_X509_EXT_PRIVATE_KEY_USAGE_PERIOD</a>  
2.5.29.16

<a href="constant.OID_X509_EXT_REASON_CODE.html" class="constant"
title="constant oid_registry::OID_X509_EXT_REASON_CODE">OID_X509_EXT_REASON_CODE</a>  
2.5.29.21

<a href="constant.OID_X509_EXT_RENEWAL_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_RENEWAL_URL">OID_X509_EXT_RENEWAL_URL</a>  
2.16.840.1.113730.1.7

<a href="constant.OID_X509_EXT_REVOCATION_URL.html" class="constant"
title="constant oid_registry::OID_X509_EXT_REVOCATION_URL">OID_X509_EXT_REVOCATION_URL</a>  
2.16.840.1.113730.1.3

<a href="constant.OID_X509_EXT_SSL_SERVER_NAME.html" class="constant"
title="constant oid_registry::OID_X509_EXT_SSL_SERVER_NAME">OID_X509_EXT_SSL_SERVER_NAME</a>  
2.16.840.1.113730.1.12

<a href="constant.OID_X509_EXT_SUBJECT_ALT_NAME.html" class="constant"
title="constant oid_registry::OID_X509_EXT_SUBJECT_ALT_NAME">OID_X509_EXT_SUBJECT_ALT_NAME</a>  
2.5.29.17

<a href="constant.OID_X509_EXT_SUBJECT_KEY_IDENTIFIER.html"
class="constant"
title="constant oid_registry::OID_X509_EXT_SUBJECT_KEY_IDENTIFIER">OID_X509_EXT_SUBJECT_KEY_IDENTIFIER</a>  
2.5.29.14

<a href="constant.OID_X509_EXT_USER_PICTURE.html" class="constant"
title="constant oid_registry::OID_X509_EXT_USER_PICTURE">OID_X509_EXT_USER_PICTURE</a>  
2.16.840.1.113730.1.11

<a href="constant.OID_X509_GENERATION_QUALIFIER.html" class="constant"
title="constant oid_registry::OID_X509_GENERATION_QUALIFIER">OID_X509_GENERATION_QUALIFIER</a>  
2.5.4.44

<a href="constant.OID_X509_GIVEN_NAME.html" class="constant"
title="constant oid_registry::OID_X509_GIVEN_NAME">OID_X509_GIVEN_NAME</a>  
2.5.4.42

<a href="constant.OID_X509_INITIALS.html" class="constant"
title="constant oid_registry::OID_X509_INITIALS">OID_X509_INITIALS</a>  
2.5.4.43

<a href="constant.OID_X509_KNOWLEDGE_INFORMATION.html" class="constant"
title="constant oid_registry::OID_X509_KNOWLEDGE_INFORMATION">OID_X509_KNOWLEDGE_INFORMATION</a>  
2.5.4.2

<a href="constant.OID_X509_LOCALITY_NAME.html" class="constant"
title="constant oid_registry::OID_X509_LOCALITY_NAME">OID_X509_LOCALITY_NAME</a>  
2.5.4.7

<a href="constant.OID_X509_NAME.html" class="constant"
title="constant oid_registry::OID_X509_NAME">OID_X509_NAME</a>  
2.5.4.41

<a href="constant.OID_X509_OBJECT_CLASS.html" class="constant"
title="constant oid_registry::OID_X509_OBJECT_CLASS">OID_X509_OBJECT_CLASS</a>  
2.5.4.0

<a href="constant.OID_X509_OBSOLETE_AUTHORITY_KEY_IDENTIFIER.html"
class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_AUTHORITY_KEY_IDENTIFIER">OID_X509_OBSOLETE_AUTHORITY_KEY_IDENTIFIER</a>  
2.5.29.1

<a href="constant.OID_X509_OBSOLETE_CERTIFICATE_POLICIES.html"
class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_CERTIFICATE_POLICIES">OID_X509_OBSOLETE_CERTIFICATE_POLICIES</a>  
2.5.29.3

<a href="constant.OID_X509_OBSOLETE_ISSUER_ALT_NAME.html"
class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_ISSUER_ALT_NAME">OID_X509_OBSOLETE_ISSUER_ALT_NAME</a>  
2.5.29.8

<a href="constant.OID_X509_OBSOLETE_KEY_ATTRIBUTES.html"
class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_KEY_ATTRIBUTES">OID_X509_OBSOLETE_KEY_ATTRIBUTES</a>  
2.5.29.2

<a href="constant.OID_X509_OBSOLETE_KEY_USAGE.html" class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_KEY_USAGE">OID_X509_OBSOLETE_KEY_USAGE</a>  
2.5.29.4

<a href="constant.OID_X509_OBSOLETE_POLICY_MAPPING.html"
class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_POLICY_MAPPING">OID_X509_OBSOLETE_POLICY_MAPPING</a>  
2.5.29.5

<a href="constant.OID_X509_OBSOLETE_SUBJECT_ALT_NAME.html"
class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_SUBJECT_ALT_NAME">OID_X509_OBSOLETE_SUBJECT_ALT_NAME</a>  
2.5.29.7

<a href="constant.OID_X509_OBSOLETE_SUBTREES_CONSTRAINT.html"
class="constant"
title="constant oid_registry::OID_X509_OBSOLETE_SUBTREES_CONSTRAINT">OID_X509_OBSOLETE_SUBTREES_CONSTRAINT</a>  
2.5.29.6

<a href="constant.OID_X509_ORGANIZATIONAL_UNIT.html" class="constant"
title="constant oid_registry::OID_X509_ORGANIZATIONAL_UNIT">OID_X509_ORGANIZATIONAL_UNIT</a>  
2.5.4.11

<a href="constant.OID_X509_ORGANIZATION_NAME.html" class="constant"
title="constant oid_registry::OID_X509_ORGANIZATION_NAME">OID_X509_ORGANIZATION_NAME</a>  
2.5.4.10

<a href="constant.OID_X509_POSTAL_ADDRESS.html" class="constant"
title="constant oid_registry::OID_X509_POSTAL_ADDRESS">OID_X509_POSTAL_ADDRESS</a>  
2.5.4.16

<a href="constant.OID_X509_POSTAL_CODE.html" class="constant"
title="constant oid_registry::OID_X509_POSTAL_CODE">OID_X509_POSTAL_CODE</a>  
2.5.4.17

<a href="constant.OID_X509_SEARCH_GUIDE.html" class="constant"
title="constant oid_registry::OID_X509_SEARCH_GUIDE">OID_X509_SEARCH_GUIDE</a>  
2.5.4.14

<a href="constant.OID_X509_SERIALNUMBER.html" class="constant"
title="constant oid_registry::OID_X509_SERIALNUMBER">OID_X509_SERIALNUMBER</a>  
2.5.4.5

<a href="constant.OID_X509_STATE_OR_PROVINCE_NAME.html" class="constant"
title="constant oid_registry::OID_X509_STATE_OR_PROVINCE_NAME">OID_X509_STATE_OR_PROVINCE_NAME</a>  
2.5.4.8

<a href="constant.OID_X509_STREET_ADDRESS.html" class="constant"
title="constant oid_registry::OID_X509_STREET_ADDRESS">OID_X509_STREET_ADDRESS</a>  
2.5.4.9

<a href="constant.OID_X509_SURNAME.html" class="constant"
title="constant oid_registry::OID_X509_SURNAME">OID_X509_SURNAME</a>  
2.5.4.4

<a href="constant.OID_X509_TITLE.html" class="constant"
title="constant oid_registry::OID_X509_TITLE">OID_X509_TITLE</a>  
2.5.4.12

<a href="constant.OID_X509_UNIQUE_IDENTIFIER.html" class="constant"
title="constant oid_registry::OID_X509_UNIQUE_IDENTIFIER">OID_X509_UNIQUE_IDENTIFIER</a>  
2.5.4.45

<a href="constant.SPC_INDIRECT_DATA_OBJID.html" class="constant"
title="constant oid_registry::SPC_INDIRECT_DATA_OBJID">SPC_INDIRECT_DATA_OBJID</a>  
1.3.6.1.4.1.311.2.1.4

<a href="constant.SPC_INDIVIDUAL_SP_KEY_PURPOSE_OBJID.html"
class="constant"
title="constant oid_registry::SPC_INDIVIDUAL_SP_KEY_PURPOSE_OBJID">SPC_INDIVIDUAL_SP_KEY_PURPOSE_OBJID</a>  
1.3.6.1.4.1.311.2.1.21

<a href="constant.SPC_PE_IMAGE_DATA.html" class="constant"
title="constant oid_registry::SPC_PE_IMAGE_DATA">SPC_PE_IMAGE_DATA</a>  
1.3.6.1.4.1.311.2.1.15

<a href="constant.SPC_SP_OPUS_INFO_OBJID.html" class="constant"
title="constant oid_registry::SPC_SP_OPUS_INFO_OBJID">SPC_SP_OPUS_INFO_OBJID</a>  
1.3.6.1.4.1.311.2.1.12

<a href="constant.SPC_STATEMENT_TYPE_OBJID.html" class="constant"
title="constant oid_registry::SPC_STATEMENT_TYPE_OBJID">SPC_STATEMENT_TYPE_OBJID</a>  
1.3.6.1.4.1.311.2.1.11

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.format_oid.html" class="fn"
title="fn oid_registry::format_oid">format_oid</a>  
Format a OID to a `String`, using the provided registry to get the short
name if present.

<a href="fn.generate_file.html" class="fn"
title="fn oid_registry::generate_file">generate_file</a>  
Generate a file containing a `with_<feat>` method for OidRegistry

<a href="fn.load_file.html" class="fn"
title="fn oid_registry::load_file">load_file</a>  
Load a file to an OID description map

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.LoadedMap.html" class="type"
title="type oid_registry::LoadedMap">LoadedMap</a>  
Temporary structure, created when reading a file containing OID
declarations
