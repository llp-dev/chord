**webpki > error**

# Module: error

## Contents

**Structs**

- [`InvalidNameContext`](#invalidnamecontext) - Additional context for the `CertNotValidForName` error variant.
- [`UnsupportedSignatureAlgorithmContext`](#unsupportedsignaturealgorithmcontext) - Additional context for the `UnsupportedSignatureAlgorithm` error variant.
- [`UnsupportedSignatureAlgorithmForPublicKeyContext`](#unsupportedsignaturealgorithmforpublickeycontext) - Additional context for the `UnsupportedSignatureAlgorithmForPublicKey` error variant.

**Enums**

- [`DerTypeId`](#dertypeid) - Trailing data was found while parsing DER-encoded input for the named type.
- [`Error`](#error) - An error that occurs during certificate validation or name validation.

---

## webpki::error::DerTypeId

*Enum*

Trailing data was found while parsing DER-encoded input for the named type.

**Variants:**
- `BitString`
- `Bool`
- `Certificate`
- `CertificateExtensions`
- `CertificateTbsCertificate`
- `CertRevocationList`
- `CertRevocationListExtension`
- `CrlDistributionPoint`
- `CommonNameInner`
- `CommonNameOuter`
- `DistributionPointName`
- `Extension`
- `GeneralName`
- `RevocationReason`
- `Signature`
- `SignatureAlgorithm`
- `SignedData`
- `SubjectPublicKeyInfo`
- `Time`
- `TrustAnchorV1`
- `TrustAnchorV1TbsCertificate`
- `U8`
- `RevokedCertificate`
- `RevokedCertificateExtension`
- `RevokedCertEntry`
- `IssuingDistributionPoint`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DerTypeId) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DerTypeId`



## webpki::error::Error

*Enum*

An error that occurs during certificate validation or name validation.

**Variants:**
- `BadDer` - The encoding of some ASN.1 DER-encoded item is invalid.
- `BadDerTime` - The encoding of an ASN.1 DER-encoded time is invalid.
- `CaUsedAsEndEntity` - A CA certificate is being used as an end-entity certificate.
- `CertExpired{ time: pki_types::UnixTime, not_after: pki_types::UnixTime }` - The certificate is expired; i.e. the time it is being validated for is
- `CertNotValidForName(InvalidNameContext)` - The certificate is not valid for the name it is being validated for.
- `CertNotValidYet{ time: pki_types::UnixTime, not_before: pki_types::UnixTime }` - The certificate is not valid yet; i.e. the time it is being validated
- `CertRevoked` - The certificate, or one of its issuers, has been revoked.
- `CrlExpired{ time: pki_types::UnixTime, next_update: pki_types::UnixTime }` - The CRL is expired; i.e. the verification time is not before the time
- `EmptyEkuExtension` - The certificate has an Extended Key Usage extension without any EKU values.
- `EndEntityUsedAsCa` - An end-entity certificate is being used as a CA certificate.
- `ExtensionValueInvalid` - An X.509 extension is invalid.
- `InvalidCertValidity` - The certificate validity period (notBefore, notAfter) is invalid; e.g.
- `InvalidCrlNumber` - A CRL number extension was invalid:
- `InvalidNetworkMaskConstraint` - A iPAddress name constraint was invalid:
- `InvalidSerialNumber` - A serial number was invalid:
- `InvalidCrlSignatureForPublicKey` - The CRL signature is invalid for the issuer's public key.
- `InvalidSignatureForPublicKey` - The signature is invalid for the given public key.
- `IssuerNotCrlSigner` - A CRL was signed by an issuer that has a KeyUsage bitstring that does not include
- `MalformedDnsIdentifier` - A presented or reference DNS identifier was malformed, potentially
- `MalformedExtensions` - The certificate extensions are malformed.
- `MalformedNameConstraint` - A name constraint was malformed, potentially containing invalid characters or
- `MaximumNameConstraintComparisonsExceeded` - The maximum number of name constraint comparisons has been reached.
- `MaximumPathBuildCallsExceeded` - The maximum number of internal path building calls has been reached. Path complexity is too great.
- `MaximumPathDepthExceeded` - The path search was terminated because it became too deep.
- `MaximumSignatureChecksExceeded` - The maximum number of signature checks has been reached. Path complexity is too great.
- `NameConstraintViolation` - The certificate violates one or more name constraints.
- `PathLenConstraintViolated` - The certificate violates one or more path length constraints.
- `RequiredEkuNotFound` - The certificate is not valid for the Extended Key Usage for which it is
- `RequiredEkuNotFoundContext(crate::verify_cert::RequiredEkuNotFoundContext)` - The certificate is not valid for the Extended Key Usage for which it is
- `SignatureAlgorithmMismatch` - The algorithm in the TBSCertificate "signature" field of a certificate
- `TrailingData(DerTypeId)` - Trailing data was found while parsing DER-encoded input for the named type.
- `UnknownIssuer` - A valid issuer for the certificate could not be found.
- `UnknownRevocationStatus` - The certificate's revocation status could not be determined.
- `UnsupportedCertVersion` - The certificate is not a v3 X.509 certificate.
- `UnsupportedCriticalExtension` - The certificate contains an unsupported critical extension.
- `UnsupportedCrlIssuingDistributionPoint` - The CRL contains an issuing distribution point with no distribution point name,
- `UnsupportedCrlVersion` - The CRL is not a v2 X.509 CRL.
- `UnsupportedDeltaCrl` - The CRL is an unsupported "delta" CRL.
- `UnsupportedIndirectCrl` - The CRL contains unsupported "indirect" entries.
- `UnsupportedNameType` - The `ServerName` contained an unsupported type of value.
- `UnsupportedRevocationReason` - The revocation reason is not in the set of supported revocation reasons.
- `UnsupportedRevocationReasonsPartitioning` - The CRL is partitioned by revocation reasons.
- `UnsupportedCrlSignatureAlgorithm` - The signature algorithm for a signature over a CRL is not in the set of supported
- `UnsupportedCrlSignatureAlgorithmContext(UnsupportedSignatureAlgorithmContext)` - The signature algorithm for a signature is not in the set of supported
- `UnsupportedSignatureAlgorithm` - The signature algorithm for a signature is not in the set of supported
- `UnsupportedSignatureAlgorithmContext(UnsupportedSignatureAlgorithmContext)` - The signature algorithm for a signature is not in the set of supported
- `UnsupportedCrlSignatureAlgorithmForPublicKey` - The CRL signature's algorithm does not match the algorithm of the issuer
- `UnsupportedCrlSignatureAlgorithmForPublicKeyContext(UnsupportedSignatureAlgorithmForPublicKeyContext)` - The signature's algorithm does not match the algorithm of the public
- `UnsupportedSignatureAlgorithmForPublicKey` - The signature's algorithm does not match the algorithm of the public
- `UnsupportedSignatureAlgorithmForPublicKeyContext(UnsupportedSignatureAlgorithmForPublicKeyContext)` - The signature's algorithm does not match the algorithm of the public

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## webpki::error::InvalidNameContext

*Struct*

Additional context for the `CertNotValidForName` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

**Fields:**
- `expected: pki_types::ServerName<'static>` - Expected server name.
- `presented: alloc::vec::Vec<alloc::string::String>` - The names presented in the end entity certificate.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &InvalidNameContext) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> InvalidNameContext`



## webpki::error::UnsupportedSignatureAlgorithmContext

*Struct*

Additional context for the `UnsupportedSignatureAlgorithm` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

**Fields:**
- `signature_algorithm_id: alloc::vec::Vec<u8>` - The signature algorithm OID that was unsupported.
- `supported_algorithms: alloc::vec::Vec<pki_types::AlgorithmIdentifier>` - Supported algorithms that were available for signature verification.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> UnsupportedSignatureAlgorithmContext`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnsupportedSignatureAlgorithmContext) -> bool`



## webpki::error::UnsupportedSignatureAlgorithmForPublicKeyContext

*Struct*

Additional context for the `UnsupportedSignatureAlgorithmForPublicKey` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

**Fields:**
- `signature_algorithm_id: alloc::vec::Vec<u8>` - The signature algorithm OID.
- `public_key_algorithm_id: alloc::vec::Vec<u8>` - The public key algorithm OID.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> UnsupportedSignatureAlgorithmForPublicKeyContext`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnsupportedSignatureAlgorithmForPublicKeyContext) -> bool`



