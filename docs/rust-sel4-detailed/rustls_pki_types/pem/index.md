*[rustls_pki_types](../index.md) / [pem](index.md)*

---

# Module `pem`

Low-level PEM decoding APIs.

These APIs allow decoding PEM format in an iterator, which means you
can load multiple different types of PEM section from a file in a single
pass.

## Contents

- [Structs](#structs)
  - [`SliceIter`](#sliceiter)
- [Enums](#enums)
  - [`SectionLabel`](#sectionlabel)
  - [`SectionKind`](#sectionkind)
  - [`Error`](#error)
- [Traits](#traits)
  - [`PemObject`](#pemobject)
  - [`PemObjectFilter`](#pemobjectfilter)
- [Functions](#functions)
  - [`read`](#read)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SliceIter`](#sliceiter) | struct | Iterator over all PEM sections in a `&[u8]` slice. |
| [`SectionLabel`](#sectionlabel) | enum |  |
| [`SectionKind`](#sectionkind) | enum | A single recognised section in a PEM file. |
| [`Error`](#error) | enum | Errors that may arise when parsing the contents of a PEM file |
| [`PemObject`](#pemobject) | trait | Items that can be decoded from PEM data. |
| [`PemObjectFilter`](#pemobjectfilter) | trait |  |
| [`read`](#read) | fn |  |

## Structs

### `SliceIter<'a, T>`

```rust
struct SliceIter<'a, T> {
    current: &'a [u8],
    _ty: core::marker::PhantomData<T>,
    b64_buf: alloc::vec::Vec<u8>,
}
```

Iterator over all PEM sections in a `&[u8]` slice.

#### Implementations

- <span id="sliceiter-new"></span>`fn new(current: &'a [u8]) -> Self`

  Create a new iterator.

- <span id="sliceiter-read-section"></span>`fn read_section(&mut self) -> Result<Option<(SectionKind, Vec<u8>)>, Error>` — [`SectionKind`](#sectionkind), [`Error`](#error)

  Extract and decode the next supported PEM section from `input`

  

  - `Ok(None)` is returned if there is no PEM section to read from `input`

  - Syntax errors and decoding errors produce a `Err(...)`

  - Otherwise each decoded section is returned with a `Ok(Some((..., remainder)))` where

    `remainder` is the part of the `input` that follows the returned section

#### Trait Implementations

##### `impl IntoIterator for SliceIter<'a, T>`

- <span id="sliceiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sliceiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sliceiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: PemObject> Iterator for SliceIter<'_, T>`

- <span id="sliceiter-iterator-type-item"></span>`type Item = Result<T, Error>`

- <span id="sliceiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `SectionLabel`

```rust
enum SectionLabel {
    Known(SectionKind),
    Unknown(alloc::vec::Vec<u8>),
}
```

#### Implementations

- <span id="sectionlabel-is-end"></span>`fn is_end(&self, line: &[u8]) -> bool`

#### Trait Implementations

##### `impl AsRef for SectionLabel`

- <span id="sectionlabel-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

### `SectionKind`

```rust
enum SectionKind {
    Certificate,
    PublicKey,
    RsaPrivateKey,
    PrivateKey,
    EcPrivateKey,
    Crl,
    Csr,
    EchConfigList,
}
```

A single recognised section in a PEM file.

#### Variants

- **`Certificate`**

  A DER-encoded x509 certificate.
  
  Appears as "CERTIFICATE" in PEM files.

- **`PublicKey`**

  A DER-encoded Subject Public Key Info; as specified in RFC 7468.
  
  Appears as "PUBLIC KEY" in PEM files.

- **`RsaPrivateKey`**

  A DER-encoded plaintext RSA private key; as specified in PKCS #1/RFC 3447
  
  Appears as "RSA PRIVATE KEY" in PEM files.

- **`PrivateKey`**

  A DER-encoded plaintext private key; as specified in PKCS #8/RFC 5958
  
  Appears as "PRIVATE KEY" in PEM files.

- **`EcPrivateKey`**

  A Sec1-encoded plaintext private key; as specified in RFC 5915
  
  Appears as "EC PRIVATE KEY" in PEM files.

- **`Crl`**

  A Certificate Revocation List; as specified in RFC 5280
  
  Appears as "X509 CRL" in PEM files.

- **`Csr`**

  A Certificate Signing Request; as specified in RFC 2986
  
  Appears as "CERTIFICATE REQUEST" in PEM files.

- **`EchConfigList`**

  An EchConfigList structure, as specified in
  <https://www.ietf.org/archive/id/draft-farrell-tls-pemesni-05.html>.
  
  Appears as "ECHCONFIG" in PEM files.

#### Implementations

- <span id="sectionkind-secret"></span>`const fn secret(&self) -> bool`

- <span id="sectionkind-as-slice"></span>`fn as_slice(&self) -> &'static [u8]`

#### Trait Implementations

##### `impl Clone for SectionKind`

- <span id="sectionkind-clone"></span>`fn clone(&self) -> SectionKind` — [`SectionKind`](#sectionkind)

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- <span id="sectionkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for SectionKind`

- <span id="sectionkind-partialeq-eq"></span>`fn eq(&self, other: &SectionKind) -> bool` — [`SectionKind`](#sectionkind)

##### `impl StructuralPartialEq for SectionKind`

### `Error`

```rust
enum Error {
    MissingSectionEnd {
        end_marker: alloc::vec::Vec<u8>,
    },
    IllegalSectionStart {
        line: alloc::vec::Vec<u8>,
    },
    Base64Decode(alloc::string::String),
    NoItemsFound,
}
```

Errors that may arise when parsing the contents of a PEM file

#### Variants

- **`MissingSectionEnd`**

  a section is missing its "END marker" line

- **`IllegalSectionStart`**

  syntax error found in the line that starts a new section

- **`Base64Decode`**

  base64 decode error

- **`NoItemsFound`**

  No items found of desired type

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `PemObject`

```rust
trait PemObject: Sized { ... }
```

Items that can be decoded from PEM data.

#### Required Methods

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<Self>`

  Conversion from a PEM [`SectionKind`](#sectionkind) and body data.

#### Provided Methods

- `fn from_pem_slice(pem: &[u8]) -> Result<Self, Error>`

  Decode the first section of this type from PEM contained in

- `fn pem_slice_iter(pem: &[u8]) -> SliceIter<'_, Self>`

  Iterate over all sections of this type from PEM contained in

#### Implementors

- [`PrivateKeyDer`](../index.md#privatekeyder)
- `(SectionKind, alloc::vec::Vec<u8>)`
- `T`

### `PemObjectFilter`

```rust
trait PemObjectFilter: PemObject + From<alloc::vec::Vec<u8>> { ... }
```

#### Associated Constants

- `const KIND: SectionKind`

#### Implementors

- [`CertificateDer`](../index.md#certificateder)
- [`CertificateRevocationListDer`](../index.md#certificaterevocationlistder)
- [`CertificateSigningRequestDer`](../index.md#certificatesigningrequestder)
- [`EchConfigListBytes`](../index.md#echconfiglistbytes)
- [`PrivatePkcs1KeyDer`](../index.md#privatepkcs1keyder)
- [`PrivatePkcs8KeyDer`](../index.md#privatepkcs8keyder)
- [`PrivateSec1KeyDer`](../index.md#privatesec1keyder)
- [`SubjectPublicKeyInfoDer`](../index.md#subjectpublickeyinfoder)

## Functions

### `read`

```rust
fn read(next_line: Option<&[u8]>, section: &mut Option<SectionLabel>, b64buf: &mut alloc::vec::Vec<u8>) -> Result<core::ops::ControlFlow<Option<(SectionKind, alloc::vec::Vec<u8>)>, ()>, Error>
```

