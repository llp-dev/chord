**rustls_pemfile > pemfile**

# Module: pemfile

## Contents

**Enums**

- [`Error`](#error) - Errors that may arise when parsing the contents of a PEM file
- [`Item`](#item) - The contents of a single recognised block in a PEM file.

**Functions**

- [`read_one_from_slice`](#read_one_from_slice) - Extract and decode the next PEM section from `input`

---

## rustls_pemfile::pemfile::Error

*Enum*

Errors that may arise when parsing the contents of a PEM file

This differs from [`rustls_pki_types::pem::Error`] because it is `PartialEq`;
it is retained for compatibility.

**Variants:**
- `MissingSectionEnd{ end_marker: alloc::vec::Vec<u8> }` - a section is missing its "END marker" line
- `IllegalSectionStart{ line: alloc::vec::Vec<u8> }` - syntax error found in the line that starts a new section
- `Base64Decode(alloc::string::String)` - base64 decode error

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(error: pem::Error) -> Self`



## rustls_pemfile::pemfile::Item

*Enum*

The contents of a single recognised block in a PEM file.

**Variants:**
- `X509Certificate(pki_types::CertificateDer<'static>)` - A DER-encoded x509 certificate.
- `SubjectPublicKeyInfo(pki_types::SubjectPublicKeyInfoDer<'static>)` - A DER-encoded Subject Public Key Info; as specified in RFC 7468.
- `Pkcs1Key(pki_types::PrivatePkcs1KeyDer<'static>)` - A DER-encoded plaintext RSA private key; as specified in PKCS #1/RFC 3447
- `Pkcs8Key(pki_types::PrivatePkcs8KeyDer<'static>)` - A DER-encoded plaintext private key; as specified in PKCS #8/RFC 5958
- `Sec1Key(pki_types::PrivateSec1KeyDer<'static>)` - A Sec1-encoded plaintext private key; as specified in RFC 5915
- `Crl(pki_types::CertificateRevocationListDer<'static>)` - A Certificate Revocation List; as specified in RFC 5280
- `Csr(pki_types::CertificateSigningRequestDer<'static>)` - A Certificate Signing Request; as specified in RFC 2986

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Item) -> bool`



## rustls_pemfile::pemfile::read_one_from_slice

*Function*

Extract and decode the next PEM section from `input`

- `Ok(None)` is returned if there is no PEM section to read from `input`
- Syntax errors and decoding errors produce a `Err(...)`
- Otherwise each decoded section is returned with a `Ok(Some((Item::..., remainder)))` where
  `remainder` is the part of the `input` that follows the returned section

```rust
fn read_one_from_slice(input: &[u8]) -> Result<Option<(Item, &[u8])>, Error>
```



