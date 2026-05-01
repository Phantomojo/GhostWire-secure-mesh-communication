Crate rcgen Copy item path

<span class="sub-heading"><a href="../src/rcgen/lib.rs.html#1-842" class="src">Source</a>
</span>

Expand description

Rust X.509 certificate generation utility

This crate provides a way to generate self signed X.509 certificates.

The most simple way of using this crate is by calling the
[`generate_simple_self_signed`](fn.generate_simple_self_signed.html "fn rcgen::generate_simple_self_signed")
function. For more customization abilities, construct a
[`CertificateParams`](struct.CertificateParams.html "struct rcgen::CertificateParams")
and a key pair to call
[`CertificateParams::signed_by()`](struct.CertificateParams.html#method.signed_by "method rcgen::CertificateParams::signed_by")
or
[`CertificateParams::self_signed()`](struct.CertificateParams.html#method.self_signed "method rcgen::CertificateParams::self_signed").

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use rcgen::{generate_simple_self_signed, CertifiedKey};
// Generate a certificate that's valid for "localhost" and "hello.world.example"
let subject_alt_names = vec!["hello.world.example".to_string(),
    "localhost".to_string()];

let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();
println!("{}", cert.pem());
println!("{}", key_pair.serialize_pem());
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Attribute.html" class="struct"
title="struct rcgen::Attribute">Attribute</a>  
A PKCS \#10 CSR attribute, as defined in [RFC
5280](https://datatracker.ietf.org/doc/html/rfc5280#appendix-A.1) and
constrained by [RFC
2986](https://datatracker.ietf.org/doc/html/rfc2986#section-4).

<a href="struct.BmpString.html" class="struct"
title="struct rcgen::BmpString">BmpString</a>  
ASN.1 `BMPString` type.

<a href="struct.Certificate.html" class="struct"
title="struct rcgen::Certificate">Certificate</a>  
An issued certificate together with the parameters used to generate it.

<a href="struct.CertificateParams.html" class="struct"
title="struct rcgen::CertificateParams">CertificateParams</a>  
Parameters used for certificate generation

<a href="struct.CertificateRevocationList.html" class="struct"
title="struct rcgen::CertificateRevocationList">CertificateRevocationList</a>  
A certificate revocation list (CRL)

<a href="struct.CertificateRevocationListParams.html" class="struct"
title="struct rcgen::CertificateRevocationListParams">CertificateRevocationListParams</a>  
Parameters used for certificate revocation list (CRL) generation

<a href="struct.CertificateSigningRequest.html" class="struct"
title="struct rcgen::CertificateSigningRequest">CertificateSigningRequest</a>  
A certificate signing request (CSR) that can be encoded to PEM or DER.

<a href="struct.CertificateSigningRequestParams.html" class="struct"
title="struct rcgen::CertificateSigningRequestParams">CertificateSigningRequestParams</a>  
Parameters for a certificate signing request

<a href="struct.CertifiedKey.html" class="struct"
title="struct rcgen::CertifiedKey">CertifiedKey</a>  
An issued certificate, together with the subject keypair.

<a href="struct.CrlDistributionPoint.html" class="struct"
title="struct rcgen::CrlDistributionPoint">CrlDistributionPoint</a>  
A certificate revocation list (CRL) distribution point, to be included
in a certificate’s [distribution points
extension](https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.13) or a
CRL’s [issuing distribution point
extension](https://datatracker.ietf.org/doc/html/rfc5280#section-5.2.5)

<a href="struct.CrlIssuingDistributionPoint.html" class="struct"
title="struct rcgen::CrlIssuingDistributionPoint">CrlIssuingDistributionPoint</a>  
A certificate revocation list (CRL) issuing distribution point, to be
included in a CRL’s [issuing distribution point
extension](https://datatracker.ietf.org/doc/html/rfc5280#section-5.2.5).

<a href="struct.CustomExtension.html" class="struct"
title="struct rcgen::CustomExtension">CustomExtension</a>  
A custom extension of a certificate, as specified in [RFC
5280](https://tools.ietf.org/html/rfc5280#section-4.2)

<a href="struct.DistinguishedName.html" class="struct"
title="struct rcgen::DistinguishedName">DistinguishedName</a>  
Distinguished name used e.g. for the issuer and subject fields of a
certificate

<a href="struct.DistinguishedNameIterator.html" class="struct"
title="struct rcgen::DistinguishedNameIterator">DistinguishedNameIterator</a>  
Iterator over
[`DistinguishedName`](struct.DistinguishedName.html "struct rcgen::DistinguishedName")
entries

<a href="struct.Ia5String.html" class="struct"
title="struct rcgen::Ia5String">Ia5String</a>  
ASN.1 `IA5String` type.

<a href="struct.KeyPair.html" class="struct"
title="struct rcgen::KeyPair">KeyPair</a>  
A key pair used to sign certificates and CSRs

<a href="struct.NameConstraints.html" class="struct"
title="struct rcgen::NameConstraints">NameConstraints</a>  
The [NameConstraints
extension](https://tools.ietf.org/html/rfc5280#section-4.2.1.10) (only
relevant for CA certificates)

<a href="struct.PrintableString.html" class="struct"
title="struct rcgen::PrintableString">PrintableString</a>  
ASN.1 `PrintableString` type.

<a href="struct.PublicKey.html" class="struct"
title="struct rcgen::PublicKey">PublicKey</a>  
A public key, extracted from a CSR

<a href="struct.RevokedCertParams.html" class="struct"
title="struct rcgen::RevokedCertParams">RevokedCertParams</a>  
Parameters used for describing a revoked certificate included in a
[`CertificateRevocationList`](struct.CertificateRevocationList.html "struct rcgen::CertificateRevocationList").

<a href="struct.SerialNumber.html" class="struct"
title="struct rcgen::SerialNumber">SerialNumber</a>  
A certificate serial number.

<a href="struct.SignatureAlgorithm.html" class="struct"
title="struct rcgen::SignatureAlgorithm">SignatureAlgorithm</a>  
Signature algorithm type

<a href="struct.SubjectPublicKeyInfo.html" class="struct"
title="struct rcgen::SubjectPublicKeyInfo">SubjectPublicKeyInfo</a>  
A public key

<a href="struct.TeletexString.html" class="struct"
title="struct rcgen::TeletexString">TeletexString</a>  
ASN.1 `TeletexString` type.

<a href="struct.UniversalString.html" class="struct"
title="struct rcgen::UniversalString">UniversalString</a>  
ASN.1 `UniversalString` type.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.BasicConstraints.html" class="enum"
title="enum rcgen::BasicConstraints">BasicConstraints</a>  
The path length constraint (only relevant for CA certificates)

<a href="enum.CidrSubnet.html" class="enum"
title="enum rcgen::CidrSubnet">CidrSubnet</a>  
CIDR subnet, as per [RFC 4632](https://tools.ietf.org/html/rfc4632)

<a href="enum.CrlScope.html" class="enum"
title="enum rcgen::CrlScope">CrlScope</a>  
Describes the scope of a CRL for an issuing distribution point
extension.

<a href="enum.DnType.html" class="enum"
title="enum rcgen::DnType">DnType</a>  
The attribute type of a distinguished name entry

<a href="enum.DnValue.html" class="enum"
title="enum rcgen::DnValue">DnValue</a>  
A distinguished name entry

<a href="enum.Error.html" class="enum"
title="enum rcgen::Error">Error</a>  
The error type of the rcgen crate

<a href="enum.ExtendedKeyUsagePurpose.html" class="enum"
title="enum rcgen::ExtendedKeyUsagePurpose">ExtendedKeyUsagePurpose</a>  
One of the purposes contained in the [extended key usage
extension](https://tools.ietf.org/html/rfc5280#section-4.2.1.12)

<a href="enum.GeneralSubtree.html" class="enum"
title="enum rcgen::GeneralSubtree">GeneralSubtree</a>  
General Subtree type.

<a href="enum.InvalidAsn1String.html" class="enum"
title="enum rcgen::InvalidAsn1String">InvalidAsn1String</a>  
Invalid ASN.1 string type

<a href="enum.IsCa.html" class="enum" title="enum rcgen::IsCa">IsCa</a>  
Whether the certificate is allowed to sign other certificates

<a href="enum.KeyIdMethod.html" class="enum"
title="enum rcgen::KeyIdMethod">KeyIdMethod</a>  
Method to generate key identifiers from public keys.

<a href="enum.KeyUsagePurpose.html" class="enum"
title="enum rcgen::KeyUsagePurpose">KeyUsagePurpose</a>  
One of the purposes contained in the [key
usage](https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.3)
extension

<a href="enum.OtherNameValue.html" class="enum"
title="enum rcgen::OtherNameValue">OtherNameValue</a>  
An `OtherName` value, defined in [RFC
5280§4.1.2.4](https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.4).

<a href="enum.RevocationReason.html" class="enum"
title="enum rcgen::RevocationReason">RevocationReason</a>  
Identifies the reason a certificate was revoked. See [RFC 5280
§5.3.1](https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1)

<a href="enum.SanType.html" class="enum"
title="enum rcgen::SanType">SanType</a>  
The type of subject alt name

# Statics<a href="#statics" class="anchor">§</a>

<a href="static.PKCS_ECDSA_P256_SHA256.html" class="static"
title="static rcgen::PKCS_ECDSA_P256_SHA256">PKCS_ECDSA_P256_SHA256</a>  
ECDSA signing using the P-256 curves and SHA-256 hashing as per [RFC
5758](https://tools.ietf.org/html/rfc5758#section-3.2)

<a href="static.PKCS_ECDSA_P384_SHA384.html" class="static"
title="static rcgen::PKCS_ECDSA_P384_SHA384">PKCS_ECDSA_P384_SHA384</a>  
ECDSA signing using the P-384 curves and SHA-384 hashing as per [RFC
5758](https://tools.ietf.org/html/rfc5758#section-3.2)

<a href="static.PKCS_ED25519.html" class="static"
title="static rcgen::PKCS_ED25519">PKCS_ED25519</a>  
ED25519 curve signing as per [RFC
8410](https://tools.ietf.org/html/rfc8410)

<a href="static.PKCS_RSA_SHA256.html" class="static"
title="static rcgen::PKCS_RSA_SHA256">PKCS_RSA_SHA256</a>  
RSA signing with PKCS#1 1.5 padding and SHA-256 hashing as per [RFC
4055](https://tools.ietf.org/html/rfc4055)

<a href="static.PKCS_RSA_SHA384.html" class="static"
title="static rcgen::PKCS_RSA_SHA384">PKCS_RSA_SHA384</a>  
RSA signing with PKCS#1 1.5 padding and SHA-256 hashing as per [RFC
4055](https://tools.ietf.org/html/rfc4055)

<a href="static.PKCS_RSA_SHA512.html" class="static"
title="static rcgen::PKCS_RSA_SHA512">PKCS_RSA_SHA512</a>  
RSA signing with PKCS#1 1.5 padding and SHA-512 hashing as per [RFC
4055](https://tools.ietf.org/html/rfc4055)

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.PublicKeyData.html" class="trait"
title="trait rcgen::PublicKeyData">PublicKeyData</a>  
The public key data of a key pair

<a href="trait.RemoteKeyPair.html" class="trait"
title="trait rcgen::RemoteKeyPair">RemoteKeyPair</a>  
A private key that is not directly accessible, but can be used to sign
messages

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.date_time_ymd.html" class="fn"
title="fn rcgen::date_time_ymd">date_time_ymd</a>  
Helper to obtain an `OffsetDateTime` from year, month, day values

<a href="fn.generate_simple_self_signed.html" class="fn"
title="fn rcgen::generate_simple_self_signed">generate_simple_self_signed</a>  
KISS function to generate a self signed certificate

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.RcgenError.html" class="type"
title="type rcgen::RcgenError">RcgenError</a><span class="stab deprecated" title="">Deprecated</span>  
Type-alias for the old name of
[`Error`](enum.Error.html "enum rcgen::Error").
