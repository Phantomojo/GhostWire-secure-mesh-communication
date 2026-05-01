Crate webpki Copy item path

<span class="sub-heading"><a href="../src/webpki/lib.rs.html#15-221" class="src">Source</a>
</span>

Expand description

webpki: Web PKI X.509 Certificate Validation.

See `EndEntityCert`’s documentation for a description of the certificate
processing steps necessary for a TLS connection.

# <a href="#features" class="doc-anchor">§</a>Features

| Feature | Description |
|----|----|
| `alloc` | Enable features that require use of the heap. Currently all RSA signature algorithms require this feature. |
| `std` | Enable features that require libstd. Implies `alloc`. |
| `ring` | Enable use of the *ring* crate for cryptography. |
| `aws-lc-rs` | Enable use of the aws-lc-rs crate for cryptography. Previously this feature was named `aws_lc_rs`. |

# Modules<a href="#modules" class="anchor">§</a>

<a href="aws_lc_rs/index.html" class="mod"
title="mod webpki::aws_lc_rs">aws_lc_rs</a>  
Signature verification algorithm implementations using the aws-lc-rs
crypto library.

<a href="ring/index.html" class="mod" title="mod webpki::ring">ring</a>  
Signature verification algorithm implementations using the *ring* crypto
library.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BorrowedCertRevocationList.html" class="struct"
title="struct webpki::BorrowedCertRevocationList">BorrowedCertRevocationList</a>  
Borrowed representation of a RFC 5280 profile Certificate Revocation
List (CRL).

<a href="struct.BorrowedRevokedCert.html" class="struct"
title="struct webpki::BorrowedRevokedCert">BorrowedRevokedCert</a>  
Borrowed representation of a RFC 5280 profile Certificate Revocation
List (CRL) revoked certificate entry.

<a href="struct.Cert.html" class="struct"
title="struct webpki::Cert">Cert</a>  
A parsed X509 certificate.

<a href="struct.CrlsRequired.html" class="struct"
title="struct webpki::CrlsRequired">CrlsRequired</a>  
An opaque error indicating the caller must provide at least one CRL when
building a
[RevocationOptions](struct.RevocationOptions.html "struct webpki::RevocationOptions")
instance.

<a href="struct.DerIterator.html" class="struct"
title="struct webpki::DerIterator">DerIterator</a>  
Iterator to parse a sequence of DER-encoded values of type `T`.

<a href="struct.EndEntityCert.html" class="struct"
title="struct webpki::EndEntityCert">EndEntityCert</a>  
An end-entity certificate.

<a href="struct.IntermediateIterator.html" class="struct"
title="struct webpki::IntermediateIterator">IntermediateIterator</a>  
Iterator over a path’s intermediate certificates.

<a href="struct.InvalidNameContext.html" class="struct"
title="struct webpki::InvalidNameContext">InvalidNameContext</a>  
Additional context for the `CertNotValidForName` error variant.

<a href="struct.KeyPurposeId.html" class="struct"
title="struct webpki::KeyPurposeId">KeyPurposeId</a>  
An OID value indicating an Extended Key Usage (EKU) key purpose.

<a href="struct.KeyPurposeIdIter.html" class="struct"
title="struct webpki::KeyPurposeIdIter">KeyPurposeIdIter</a>  
Iterator over
[`KeyPurposeId`](struct.KeyPurposeId.html "struct webpki::KeyPurposeId")s,
for use in
[`ExtendedKeyUsageValidator`](trait.ExtendedKeyUsageValidator.html "trait webpki::ExtendedKeyUsageValidator").

<a href="struct.KeyUsage.html" class="struct"
title="struct webpki::KeyUsage">KeyUsage</a>  
The expected key usage of a certificate.

<a href="struct.OwnedCertRevocationList.html" class="struct"
title="struct webpki::OwnedCertRevocationList">OwnedCertRevocationList</a>  
Owned representation of a RFC 5280 profile Certificate Revocation List
(CRL).

<a href="struct.OwnedRevokedCert.html" class="struct"
title="struct webpki::OwnedRevokedCert">OwnedRevokedCert</a>  
Owned representation of a RFC 5280 profile Certificate Revocation List
(CRL) revoked certificate entry.

<a href="struct.RawPublicKeyEntity.html" class="struct"
title="struct webpki::RawPublicKeyEntity">RawPublicKeyEntity</a>  
A Raw Public Key, used for connections using raw public keys as
specified in [RFC 7250](https://www.rfc-editor.org/rfc/rfc7250).

<a href="struct.RequiredEkuNotFoundContext.html" class="struct"
title="struct webpki::RequiredEkuNotFoundContext">RequiredEkuNotFoundContext</a>  
Additional context for the `RequiredEkuNotFoundContext` error variant.

<a href="struct.RevocationOptions.html" class="struct"
title="struct webpki::RevocationOptions">RevocationOptions</a>  
Describes how revocation checking is performed, if at all. Can be
constructed with a
[RevocationOptionsBuilder](struct.RevocationOptionsBuilder.html "struct webpki::RevocationOptionsBuilder")
instance.

<a href="struct.RevocationOptionsBuilder.html" class="struct"
title="struct webpki::RevocationOptionsBuilder">RevocationOptionsBuilder</a>  
Builds a RevocationOptions instance to control how revocation checking
is performed.

<a href="struct.UnsupportedSignatureAlgorithmContext.html"
class="struct"
title="struct webpki::UnsupportedSignatureAlgorithmContext">UnsupportedSignatureAlgorithmContext</a>  
Additional context for the `UnsupportedSignatureAlgorithm` error
variant.

<a href="struct.UnsupportedSignatureAlgorithmForPublicKeyContext.html"
class="struct"
title="struct webpki::UnsupportedSignatureAlgorithmForPublicKeyContext">UnsupportedSignatureAlgorithmForPublicKeyContext</a>  
Additional context for the `UnsupportedSignatureAlgorithmForPublicKey`
error variant.

<a href="struct.VerifiedPath.html" class="struct"
title="struct webpki::VerifiedPath">VerifiedPath</a>  
Path from end-entity certificate to trust anchor that’s been verified.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CertRevocationList.html" class="enum"
title="enum webpki::CertRevocationList">CertRevocationList</a>  
A RFC 5280 profile Certificate Revocation List (CRL).

<a href="enum.DerTypeId.html" class="enum"
title="enum webpki::DerTypeId">DerTypeId</a>  
Trailing data was found while parsing DER-encoded input for the named
type.

<a href="enum.Error.html" class="enum"
title="enum webpki::Error">Error</a>  
An error that occurs during certificate validation or name validation.

<a href="enum.ExpirationPolicy.html" class="enum"
title="enum webpki::ExpirationPolicy">ExpirationPolicy</a>  
Describes how to handle the nextUpdate field of the CRL (i.e.
expiration).

<a href="enum.RevocationCheckDepth.html" class="enum"
title="enum webpki::RevocationCheckDepth">RevocationCheckDepth</a>  
Describes how much of a certificate chain is checked for revocation
status.

<a href="enum.RevocationReason.html" class="enum"
title="enum webpki::RevocationReason">RevocationReason</a>  
Identifies the reason a certificate was revoked. See [RFC 5280
§5.3.1](https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1)

<a href="enum.UnknownStatusPolicy.html" class="enum"
title="enum webpki::UnknownStatusPolicy">UnknownStatusPolicy</a>  
Describes how to handle the case where a certificate’s revocation status
is unknown.

# Statics<a href="#statics" class="anchor">§</a>

<a href="static.ALL_VERIFICATION_ALGS.html" class="static"
title="static webpki::ALL_VERIFICATION_ALGS">ALL_VERIFICATION_ALGS</a>  
An array of all the verification algorithms exported by this crate.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ExtendedKeyUsageValidator.html" class="trait"
title="trait webpki::ExtendedKeyUsageValidator">ExtendedKeyUsageValidator</a>  
A trait for validating the Extended Key Usage (EKU) extensions of a
certificate.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.anchor_from_trusted_cert.html" class="fn"
title="fn webpki::anchor_from_trusted_cert">anchor_from_trusted_cert</a>  
Interprets the given pre-validated DER-encoded certificate as a
`TrustAnchor`.
