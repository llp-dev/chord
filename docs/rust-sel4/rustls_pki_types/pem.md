**rustls_pki_types > pem**

# Module: pem

## Contents

**Structs**

- [`SliceIter`](#sliceiter) - Iterator over all PEM sections in a `&[u8]` slice.

**Enums**

- [`Error`](#error) - Errors that may arise when parsing the contents of a PEM file
- [`SectionKind`](#sectionkind) - A single recognised section in a PEM file.

**Traits**

- [`PemObject`](#pemobject) - Items that can be decoded from PEM data.

---

## rustls_pki_types::pem::Error

*Enum*

Errors that may arise when parsing the contents of a PEM file

**Variants:**
- `MissingSectionEnd{ end_marker: alloc::vec::Vec<u8> }` - a section is missing its "END marker" line
- `IllegalSectionStart{ line: alloc::vec::Vec<u8> }` - syntax error found in the line that starts a new section
- `Base64Decode(alloc::string::String)` - base64 decode error
- `NoItemsFound` - No items found of desired type

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls_pki_types::pem::PemObject

*Trait*

Items that can be decoded from PEM data.

**Methods:**

- `from_pem_slice`: Decode the first section of this type from PEM contained in
- `pem_slice_iter`: Iterate over all sections of this type from PEM contained in
- `from_pem`: Conversion from a PEM [`SectionKind`] and body data.



## rustls_pki_types::pem::SectionKind

*Enum*

A single recognised section in a PEM file.

**Variants:**
- `Certificate` - A DER-encoded x509 certificate.
- `PublicKey` - A DER-encoded Subject Public Key Info; as specified in RFC 7468.
- `RsaPrivateKey` - A DER-encoded plaintext RSA private key; as specified in PKCS #1/RFC 3447
- `PrivateKey` - A DER-encoded plaintext private key; as specified in PKCS #8/RFC 5958
- `EcPrivateKey` - A Sec1-encoded plaintext private key; as specified in RFC 5915
- `Crl` - A Certificate Revocation List; as specified in RFC 5280
- `Csr` - A Certificate Signing Request; as specified in RFC 2986
- `EchConfigList` - An EchConfigList structure, as specified in

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionKind) -> bool`
- **TryFrom**
  - `fn try_from(value: &[u8]) -> Result<Self, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> SectionKind`



## rustls_pki_types::pem::SliceIter

*Struct*

Iterator over all PEM sections in a `&[u8]` slice.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new(current: &'a [u8]) -> Self` - Create a new iterator.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



