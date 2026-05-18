**rustls_pki_types**

# Module: rustls_pki_types

## Contents

**Modules**

- [`alg_id`](#alg_id) - The PKIX [`AlgorithmIdentifier`] type, and common values.
- [`pem`](#pem) - Low-level PEM decoding APIs.

**Structs**

- [`CertificateDer`](#certificateder) - A DER-encoded X.509 certificate; as specified in RFC 5280
- [`CertificateRevocationListDer`](#certificaterevocationlistder) - A Certificate Revocation List; as specified in RFC 5280
- [`CertificateSigningRequestDer`](#certificatesigningrequestder) - A Certificate Signing Request; as specified in RFC 2986
- [`Der`](#der) - DER-encoded data, either owned or borrowed
- [`EchConfigListBytes`](#echconfiglistbytes) - A TLS-encoded Encrypted Client Hello (ECH) configuration list (`ECHConfigList`); as specified in
- [`InvalidSignature`](#invalidsignature) - A detail-less error when a signature is not valid.
- [`PrivatePkcs1KeyDer`](#privatepkcs1keyder) - A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447
- [`PrivatePkcs8KeyDer`](#privatepkcs8keyder) - A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958
- [`PrivateSec1KeyDer`](#privatesec1keyder) - A Sec1-encoded plaintext private key; as specified in RFC 5915
- [`SubjectPublicKeyInfoDer`](#subjectpublickeyinfoder) - A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.
- [`TrustAnchor`](#trustanchor) - A trust anchor (a.k.a. root CA)
- [`UnixTime`](#unixtime) - A timestamp, tracking the number of non-leap seconds since the Unix epoch.

**Enums**

- [`FipsStatus`](#fipsstatus) - FIPS validation status of an algorithm or implementation.
- [`PrivateKeyDer`](#privatekeyder) - A DER-encoded X.509 private key, in one of several formats

**Traits**

- [`SignatureVerificationAlgorithm`](#signatureverificationalgorithm) - An abstract signature verification algorithm.

**Type Aliases**

- [`SubjectPublicKeyInfo`](#subjectpublickeyinfo) - A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

---

## rustls_pki_types::CertificateDer

*Struct*

A DER-encoded X.509 certificate; as specified in RFC 5280

Certificates are identified in PEM context as `CERTIFICATE` and when stored in a
file usually use a `.pem`, `.cer` or `.crt` extension. For more on PEM files, refer to the
crate documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateDer, pem::PemObject};

// load several from a PEM file
let certs: Vec<_> = CertificateDer::pem_file_iter("tests/data/certificate.chain.pem")
    .unwrap()
    .collect();
assert_eq!(certs.len(), 3);

// or one from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/certificate.chain.pem");
CertificateDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let certs: Vec<_> = CertificateDer::pem_slice_iter(byte_slice)
    .collect();
assert_eq!(certs.len(), 3);
# }
```

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn from_slice(bytes: &'a [u8]) -> Self` - A const constructor to create a `CertificateDer` from a slice of DER.
- `fn into_owned(self: Self) -> CertificateDer<'static>` - Converts this certificate into its owned variant, unfreezing borrowed content (if any)

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> CertificateDer<'a>`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &CertificateDer<'a>) -> bool`



## rustls_pki_types::CertificateRevocationListDer

*Struct*

A Certificate Revocation List; as specified in RFC 5280

Certificate revocation lists are identified in PEM context as `X509 CRL` and when stored in a
file usually use a `.crl` extension. For more on PEM files, refer to the crate documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateRevocationListDer, pem::PemObject};

// load several from a PEM file
let crls: Vec<_> = CertificateRevocationListDer::pem_file_iter("tests/data/crl.pem")
    .unwrap()
    .collect();
assert!(crls.len() >= 1);

// or one from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/crl.pem");
CertificateRevocationListDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let crls: Vec<_> = CertificateRevocationListDer::pem_slice_iter(byte_slice)
    .collect();
assert!(crls.len() >= 1);
# }
```

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> CertificateRevocationListDer<'a>`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &CertificateRevocationListDer<'a>) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



## rustls_pki_types::CertificateSigningRequestDer

*Struct*

A Certificate Signing Request; as specified in RFC 2986

Certificate signing requests are identified in PEM context as `CERTIFICATE REQUEST` and when stored in a
file usually use a `.csr` extension. For more on PEM files, refer to the crate documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateSigningRequestDer, pem::PemObject};

// load from a PEM file
CertificateSigningRequestDer::from_pem_file("tests/data/csr.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/csr.pem");
CertificateSigningRequestDer::from_pem_slice(byte_slice).unwrap();
# }
```

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &CertificateSigningRequestDer<'a>) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> CertificateSigningRequestDer<'a>`



## rustls_pki_types::Der

*Struct*

DER-encoded data, either owned or borrowed

This wrapper type is used to represent DER-encoded data in a way that is agnostic to whether
the data is owned (by a `Vec<u8>`) or borrowed (by a `&[u8]`). Support for the owned
variant is only available when the `alloc` feature is enabled.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn from_slice(der: &'a [u8]) -> Self` - A const constructor to create a `Der` from a borrowed slice

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Der<'a>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Clone**
  - `fn clone(self: &Self) -> Der<'a>`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## rustls_pki_types::EchConfigListBytes

*Struct*

A TLS-encoded Encrypted Client Hello (ECH) configuration list (`ECHConfigList`); as specified in
[draft-ietf-tls-esni-18 §4](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-4)

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn into_owned(self: Self) -> EchConfigListBytes<'static>` - Converts this config into its owned variant, unfreezing borrowed content (if any)
- `fn config_and_key_from_iter<impl Iterator<Item = Result<(SectionKind, Vec<u8>), pem::Error>>>(iter: impl Trait) -> Result<(Self, PrivatePkcs8KeyDer<'static>), pem::Error>` - Convert an iterator over PEM items into an `EchConfigListBytes` and private key.

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **PartialEq**
  - `fn eq(self: &Self, other: &EchConfigListBytes<'a>) -> bool`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EchConfigListBytes<'a>`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## rustls_pki_types::FipsStatus

*Enum*

FIPS validation status of an algorithm or implementation.

**Variants:**
- `Unvalidated` - Not FIPS tested, or unapproved algorithm.
- `Pending` - In queue for FIPS validation.
- `Certified{ certificate: &'static str }` - FIPS certified, with named certificate.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &FipsStatus) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FipsStatus`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &FipsStatus) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &FipsStatus) -> $crate::cmp::Ordering`



## rustls_pki_types::InvalidSignature

*Struct*

A detail-less error when a signature is not valid.

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> InvalidSignature`



## rustls_pki_types::PrivateKeyDer

*Enum*

A DER-encoded X.509 private key, in one of several formats

See variant inner types for more detailed information.

This can load several types of PEM-encoded private key, and then reveal
which types were found:

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateKeyDer, pem::PemObject};

// load from a PEM file
let pkcs8 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
let pkcs1 = PrivateKeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();
let sec1 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();
assert!(matches!(pkcs8, PrivateKeyDer::Pkcs8(_)));
assert!(matches!(pkcs1, PrivateKeyDer::Pkcs1(_)));
assert!(matches!(sec1, PrivateKeyDer::Sec1(_)));
# }
```

**Generic Parameters:**
- 'a

**Variants:**
- `Pkcs1(PrivatePkcs1KeyDer<'a>)` - An RSA private key
- `Sec1(PrivateSec1KeyDer<'a>)` - A Sec1 private key
- `Pkcs8(PrivatePkcs8KeyDer<'a>)` - A PKCS#8 private key

**Methods:**

- `fn clone_key(self: &Self) -> PrivateKeyDer<'static>` - Clone the private key to a `'static` value
- `fn secret_der(self: &Self) -> &[u8]` - Yield the DER-encoded bytes of the private key

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(key: PrivatePkcs8KeyDer<'a>) -> Self`
- **From**
  - `fn from(key: PrivatePkcs1KeyDer<'a>) -> Self`
- **PemObject**
  - `fn from_pem(kind: SectionKind, value: Vec<u8>) -> Option<Self>`
- **TryFrom**
  - `fn try_from(key: &'a [u8]) -> Result<Self, <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryFrom**
  - `fn try_from(key: Vec<u8>) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(key: PrivateSec1KeyDer<'a>) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &PrivateKeyDer<'a>) -> bool`
- **Zeroize**
  - `fn zeroize(self: & mut Self)`



## rustls_pki_types::PrivatePkcs1KeyDer

*Struct*

A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447

RSA private keys are identified in PEM context as `RSA PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs1KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs1KeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/rsa1024.pkcs1.pem");
PrivatePkcs1KeyDer::from_pem_slice(byte_slice).unwrap();
# }
```

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn clone_key(self: &Self) -> PrivatePkcs1KeyDer<'static>` - Clone the private key to a `'static` value
- `fn secret_pkcs1_der(self: &Self) -> &[u8]` - Yield the DER-encoded bytes of the private key

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Zeroize**
  - `fn zeroize(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &PrivatePkcs1KeyDer<'a>) -> bool`



## rustls_pki_types::PrivatePkcs8KeyDer

*Struct*

A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958

PKCS#8 private keys are identified in PEM context as `PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs8KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs8KeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
PrivatePkcs8KeyDer::from_pem_file("tests/data/rsa1024.pkcs8.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/nistp256key.pkcs8.pem");
PrivatePkcs8KeyDer::from_pem_slice(byte_slice).unwrap();
# }
```

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn clone_key(self: &Self) -> PrivatePkcs8KeyDer<'static>` - Clone the private key to a `'static` value
- `fn secret_pkcs8_der(self: &Self) -> &[u8]` - Yield the DER-encoded bytes of the private key

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &PrivatePkcs8KeyDer<'a>) -> bool`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Zeroize**
  - `fn zeroize(self: & mut Self)`



## rustls_pki_types::PrivateSec1KeyDer

*Struct*

A Sec1-encoded plaintext private key; as specified in RFC 5915

Sec1 private keys are identified in PEM context as `EC PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateSec1KeyDer, pem::PemObject};

// load from a PEM file
PrivateSec1KeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/nistp256key.pem");
PrivateSec1KeyDer::from_pem_slice(byte_slice).unwrap();
# }
```

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn clone_key(self: &Self) -> PrivateSec1KeyDer<'static>` - Clone the private key to a `'static` value
- `fn secret_sec1_der(self: &Self) -> &[u8]` - Yield the DER-encoded bytes of the private key

**Traits:** Eq

**Trait Implementations:**

- **Zeroize**
  - `fn zeroize(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &PrivateSec1KeyDer<'a>) -> bool`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`



## rustls_pki_types::SignatureVerificationAlgorithm

*Trait*

An abstract signature verification algorithm.

One of these is needed per supported pair of public key type (identified
with `public_key_alg_id()`) and `signatureAlgorithm` (identified with
`signature_alg_id()`).  Note that both of these `AlgorithmIdentifier`s include
the parameters encoding, so separate `SignatureVerificationAlgorithm`s are needed
for each possible public key or signature parameters.

Debug implementations should list the public key algorithm identifier and
signature algorithm identifier in human friendly form (i.e. not encoded bytes),
along with the name of the implementing library (to distinguish different
implementations of the same algorithms).

**Methods:**

- `verify_signature`: Verify a signature.
- `public_key_alg_id`: Return the `AlgorithmIdentifier` that must equal a public key's
- `signature_alg_id`: Return the `AlgorithmIdentifier` that must equal the `signatureAlgorithm` value
- `fips_status`: Return the FIPS status of this algorithm or implementation.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls_pki_types::SubjectPublicKeyInfo

*Type Alias*: `SubjectPublicKeyInfoDer<'a>`

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.



## rustls_pki_types::SubjectPublicKeyInfoDer

*Struct*

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

Public keys are identified in PEM context as a `PUBLIC KEY`.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{SubjectPublicKeyInfoDer, pem::PemObject};

// load from a PEM file
SubjectPublicKeyInfoDer::from_pem_file("tests/data/spki.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/spki.pem");
SubjectPublicKeyInfoDer::from_pem_slice(byte_slice).unwrap();
# }
```

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn into_owned(self: Self) -> SubjectPublicKeyInfoDer<'static>` - Converts this SubjectPublicKeyInfo into its owned variant, unfreezing borrowed content (if any)

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SubjectPublicKeyInfoDer<'a>) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(vec: Vec<u8>) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SubjectPublicKeyInfoDer<'a>`
- **From**
  - `fn from(slice: &'a [u8]) -> Self`



## rustls_pki_types::TrustAnchor

*Struct*

A trust anchor (a.k.a. root CA)

Traditionally, certificate verification libraries have represented trust anchors as full X.509
root certificates. However, those certificates contain a lot more data than is needed for
verifying certificates. The [`TrustAnchor`] representation allows an application to store
just the essential elements of trust anchors.

The most common way to get one of these is to call [`rustls_webpki::anchor_from_trusted_cert()`].

[`rustls_webpki::anchor_from_trusted_cert()`]: https://docs.rs/rustls-webpki/latest/webpki/fn.anchor_from_trusted_cert.html

**Generic Parameters:**
- 'a

**Fields:**
- `subject: Der<'a>` - Value of the `subject` field of the trust anchor
- `subject_public_key_info: Der<'a>` - Value of the `subjectPublicKeyInfo` field of the trust anchor
- `name_constraints: Option<Der<'a>>` - Value of DER-encoded `NameConstraints`, containing name constraints to the trust anchor, if any

**Methods:**

- `fn to_owned(self: &Self) -> TrustAnchor<'static>` - Yield a `'static` lifetime of the `TrustAnchor` by allocating owned `Der` variants

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &TrustAnchor<'a>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> TrustAnchor<'a>`



## rustls_pki_types::UnixTime

*Struct*

A timestamp, tracking the number of non-leap seconds since the Unix epoch.

The Unix epoch is defined January 1, 1970 00:00:00 UTC.

**Tuple Struct**: `()`

**Methods:**

- `fn since_unix_epoch(duration: Duration) -> Self` - Convert a `Duration` since the start of 1970 to a `UnixTime`
- `fn as_secs(self: &Self) -> u64` - Number of seconds since the Unix epoch

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &UnixTime) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnixTime) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &UnixTime) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> UnixTime`



## Module: alg_id

The PKIX [`AlgorithmIdentifier`] type, and common values.

If you need to use an [`AlgorithmIdentifier`] not defined here,
you can define it locally.



## Module: pem

Low-level PEM decoding APIs.

These APIs allow decoding PEM format in an iterator, which means you
can load multiple different types of PEM section from a file in a single
pass.



