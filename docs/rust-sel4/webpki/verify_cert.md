**webpki > verify_cert**

# Module: verify_cert

## Contents

**Structs**

- [`IntermediateIterator`](#intermediateiterator) - Iterator over a path's intermediate certificates.
- [`KeyPurposeId`](#keypurposeid) - An OID value indicating an Extended Key Usage (EKU) key purpose.
- [`KeyPurposeIdIter`](#keypurposeiditer) - Iterator over [`KeyPurposeId`]s, for use in [`ExtendedKeyUsageValidator`].
- [`KeyUsage`](#keyusage) - The expected key usage of a certificate.
- [`RequiredEkuNotFoundContext`](#requiredekunotfoundcontext) - Additional context for the `RequiredEkuNotFoundContext` error variant.
- [`VerifiedPath`](#verifiedpath) - Path from end-entity certificate to trust anchor that's been verified.

**Traits**

- [`ExtendedKeyUsageValidator`](#extendedkeyusagevalidator) - A trait for validating the Extended Key Usage (EKU) extensions of a certificate.

---

## webpki::verify_cert::ExtendedKeyUsageValidator

*Trait*

A trait for validating the Extended Key Usage (EKU) extensions of a certificate.

**Methods:**

- `validate`: Validate the EKU values in a certificate.



## webpki::verify_cert::IntermediateIterator

*Struct*

Iterator over a path's intermediate certificates.

Implements [`DoubleEndedIterator`] so it can be traversed in both directions.

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## webpki::verify_cert::KeyPurposeId

*Struct*

An OID value indicating an Extended Key Usage (EKU) key purpose.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(oid: &'a [u8]) -> Self` - Construct a new [`KeyPurposeId`].
- `fn to_decoded_oid(self: &Self) -> Vec<usize>` - Yield the OID value as a sequence of `usize` components.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> KeyPurposeId<'a>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## webpki::verify_cert::KeyPurposeIdIter

*Struct*

Iterator over [`KeyPurposeId`]s, for use in [`ExtendedKeyUsageValidator`].

**Generic Parameters:**
- 'a
- 'r

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## webpki::verify_cert::KeyUsage

*Struct*

The expected key usage of a certificate.

This type represents the expected key usage of an end entity certificate. Although for most
kinds of certificates the extended key usage extension is optional (and so certificates
not carrying a particular value in the EKU extension are acceptable). If the extension
is present, the certificate MUST only be used for one of the purposes indicated.

<https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.12>

**Methods:**

- `fn server_auth() -> Self` - Construct a new [`KeyUsage`] as appropriate for server certificate authentication.
- `fn client_auth() -> Self` - Construct a new [`KeyUsage`] as appropriate for client certificate authentication.
- `fn required(oid: &'static [u8]) -> Self` - Construct a new [`KeyUsage`] requiring a certificate to support the specified OID.
- `fn required_if_present(oid: &'static [u8]) -> Self` - Construct a new [`KeyUsage`] requiring a certificate to support the specified OID, if the certificate has EKUs.
- `fn oid_values(self: &Self) -> impl Trait` - Yield the OID values of the required extended key usage.

**Traits:** Eq, Copy

**Trait Implementations:**

- **ExtendedKeyUsageValidator**
  - `fn validate(self: &Self, iter: KeyPurposeIdIter) -> Result<(), Error>`
- **Clone**
  - `fn clone(self: &Self) -> KeyUsage`
- **PartialEq**
  - `fn eq(self: &Self, other: &KeyUsage) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## webpki::verify_cert::RequiredEkuNotFoundContext

*Struct*

Additional context for the `RequiredEkuNotFoundContext` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

**Fields:**
- `required: KeyUsage` - The required ExtendedKeyUsage.
- `present: alloc::vec::Vec<alloc::vec::Vec<usize>>` - The ExtendedKeyUsage OIDs present in the certificate.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RequiredEkuNotFoundContext) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RequiredEkuNotFoundContext`



## webpki::verify_cert::VerifiedPath

*Struct*

Path from end-entity certificate to trust anchor that's been verified.

See [`EndEntityCert::verify_for_usage()`] for more details on what verification entails.

**Generic Parameters:**
- 'p

**Methods:**

- `fn intermediate_certificates(self: &'p Self) -> IntermediateIterator<'p>` - Yields a (double-ended) iterator over the intermediate certificates in this path.
- `fn end_entity(self: &Self) -> &'p EndEntityCert<'p>` - Yields the end-entity certificate for this path.
- `fn anchor(self: &Self) -> &'p TrustAnchor<'p>` - Yields the trust anchor for this path.



