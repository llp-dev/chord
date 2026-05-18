# Crate `rustls_pki_types`

This crate provides types for representing X.509 certificates, keys and other types as
commonly used in the rustls ecosystem. It is intended to be used by crates that need to work
with such X.509 types, such as [rustls](https://crates.io/crates/rustls),
[rustls-webpki](https://crates.io/crates/rustls-webpki),
[rustls-pemfile](https://crates.io/crates/rustls-pemfile), and others.

Some of these crates used to define their own trivial wrappers around DER-encoded bytes.
However, in order to avoid inconvenient dependency edges, these were all disconnected. By
using a common low-level crate of types with long-term stable API, we hope to avoid the
downsides of unnecessary dependency edges while providing good interoperability between crates.

## DER and PEM

Many of the types defined in this crate represent DER-encoded data. DER is a binary encoding of
the ASN.1 format commonly used in web PKI specifications. It is a binary encoding, so it is
relatively compact when stored in memory. However, as a binary format, it is not very easy to
work with for humans and in contexts where binary data is inconvenient. For this reason,
many tools and protocols use a ASCII-based encoding of DER, called PEM. In addition to the
base64-encoded DER, PEM objects are delimited by header and footer lines which indicate the type
of object contained in the PEM blob.

Types here can be created from:

- DER using (for example) `PrivatePkcs8KeyDer::from()`.
- PEM using (for example) `pem::PemObject::from_pem_slice()`.

The [`pem::PemObject`](pem/index.md) trait contains the full selection of ways to construct
these types from PEM encodings.  That includes ways to open and read from a file,
from a slice, or from an `std::io` stream.

There is also a lower-level API that allows a given PEM file to be fully consumed
in one pass, even if it contains different data types: see the implementation of
the [`pem::PemObject`](pem/index.md) trait on the `(pem::SectionKind, Vec<u8>)` tuple.

## Creating new certificates and keys

This crate does not provide any functionality for creating new certificates or keys. However,
the [rcgen](https://docs.rs/rcgen) crate can be used to create new certificates and keys.

## Cloning private keys

This crate intentionally **does not** implement `Clone` on private key types in
order to minimize the exposure of private key data in memory.

If you want to extend the lifetime of a `PrivateKeyDer<'_>`, consider `PrivateKeyDer::clone_key()`.
Alternatively  since these types are immutable, consider wrapping the `PrivateKeyDer<'_>` in a `Rc`
or an [`Arc`](#arc).



## Target `wasm32-unknown-unknown` with the `web` feature

[`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
is unavailable in `wasm32-unknown-unknown` targets, so calls to
[`UnixTime::now()`](https://docs.rs/rustls-pki-types/latest/rustls_pki_types/struct.UnixTime.html#method.now),
otherwise enabled by the [`std`](https://docs.rs/crate/rustls-pki-types/latest/features#std) feature,
require building instead with the [`web`](https://docs.rs/crate/rustls-pki-types/latest/features#web)
feature. It gets time by calling [`Date.now()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
in the browser.

## Contents

- [Modules](#modules)
  - [`alg_id`](#alg-id)
  - [`base64`](#base64)
  - [`server_name`](#server-name)
  - [`pem`](#pem)
- [Structs](#structs)
  - [`AlgorithmIdentifier`](#algorithmidentifier)
  - [`AddrParseError`](#addrparseerror)
  - [`DnsName`](#dnsname)
  - [`InvalidDnsNameError`](#invaliddnsnameerror)
  - [`Ipv4Addr`](#ipv4addr)
  - [`Ipv6Addr`](#ipv6addr)
  - [`PrivatePkcs1KeyDer`](#privatepkcs1keyder)
  - [`PrivateSec1KeyDer`](#privatesec1keyder)
  - [`PrivatePkcs8KeyDer`](#privatepkcs8keyder)
  - [`TrustAnchor`](#trustanchor)
  - [`CertificateRevocationListDer`](#certificaterevocationlistder)
  - [`CertificateSigningRequestDer`](#certificatesigningrequestder)
  - [`CertificateDer`](#certificateder)
  - [`SubjectPublicKeyInfoDer`](#subjectpublickeyinfoder)
  - [`EchConfigListBytes`](#echconfiglistbytes)
  - [`InvalidSignature`](#invalidsignature)
  - [`UnixTime`](#unixtime)
  - [`Der`](#der)
- [Enums](#enums)
  - [`IpAddr`](#ipaddr)
  - [`ServerName`](#servername)
  - [`PrivateKeyDer`](#privatekeyder)
  - [`BytesInner`](#bytesinner)
  - [`FipsStatus`](#fipsstatus)
- [Traits](#traits)
  - [`SignatureVerificationAlgorithm`](#signatureverificationalgorithm)
- [Functions](#functions)
  - [`hex`](#hex)
- [Type Aliases](#type-aliases)
  - [`SubjectPublicKeyInfo`](#subjectpublickeyinfo)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alg_id`](#alg-id) | mod | The PKIX [`AlgorithmIdentifier`] type, and common values. |
| [`base64`](#base64) | mod |  |
| [`server_name`](#server-name) | mod | DNS name validation according to RFC1035, but with underscores allowed. |
| [`pem`](#pem) | mod | Low-level PEM decoding APIs. |
| [`AlgorithmIdentifier`](#algorithmidentifier) | struct |  |
| [`AddrParseError`](#addrparseerror) | struct |  |
| [`DnsName`](#dnsname) | struct |  |
| [`InvalidDnsNameError`](#invaliddnsnameerror) | struct |  |
| [`Ipv4Addr`](#ipv4addr) | struct |  |
| [`Ipv6Addr`](#ipv6addr) | struct |  |
| [`PrivatePkcs1KeyDer`](#privatepkcs1keyder) | struct | A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447 |
| [`PrivateSec1KeyDer`](#privatesec1keyder) | struct | A Sec1-encoded plaintext private key; as specified in RFC 5915 |
| [`PrivatePkcs8KeyDer`](#privatepkcs8keyder) | struct | A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958 |
| [`TrustAnchor`](#trustanchor) | struct | A trust anchor (a.k.a. |
| [`CertificateRevocationListDer`](#certificaterevocationlistder) | struct | A Certificate Revocation List; as specified in RFC 5280 |
| [`CertificateSigningRequestDer`](#certificatesigningrequestder) | struct | A Certificate Signing Request; as specified in RFC 2986 |
| [`CertificateDer`](#certificateder) | struct | A DER-encoded X.509 certificate; as specified in RFC 5280 |
| [`SubjectPublicKeyInfoDer`](#subjectpublickeyinfoder) | struct | A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280. |
| [`EchConfigListBytes`](#echconfiglistbytes) | struct | A TLS-encoded Encrypted Client Hello (ECH) configuration list (`ECHConfigList`); as specified in [draft-ietf-tls-esni-18 §4](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-4) |
| [`InvalidSignature`](#invalidsignature) | struct | A detail-less error when a signature is not valid. |
| [`UnixTime`](#unixtime) | struct | A timestamp, tracking the number of non-leap seconds since the Unix epoch. |
| [`Der`](#der) | struct | DER-encoded data, either owned or borrowed |
| [`IpAddr`](#ipaddr) | enum |  |
| [`ServerName`](#servername) | enum |  |
| [`PrivateKeyDer`](#privatekeyder) | enum | A DER-encoded X.509 private key, in one of several formats |
| [`BytesInner`](#bytesinner) | enum |  |
| [`FipsStatus`](#fipsstatus) | enum | FIPS validation status of an algorithm or implementation. |
| [`SignatureVerificationAlgorithm`](#signatureverificationalgorithm) | trait | An abstract signature verification algorithm. |
| [`hex`](#hex) | fn |  |
| [`SubjectPublicKeyInfo`](#subjectpublickeyinfo) | type | A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280. |

## Modules

- [`alg_id`](alg_id/index.md) — The PKIX [`AlgorithmIdentifier`] type, and common values.
- [`base64`](base64/index.md)
- [`server_name`](server_name/index.md) — DNS name validation according to RFC1035, but with underscores allowed.
- [`pem`](pem/index.md) — Low-level PEM decoding APIs.

## Structs

### `AlgorithmIdentifier`

```rust
struct AlgorithmIdentifier(&'static [u8]);
```

A DER encoding of the PKIX AlgorithmIdentifier type:

```ASN.1
AlgorithmIdentifier  ::=  SEQUENCE  {
    algorithm               OBJECT IDENTIFIER,
    parameters              ANY DEFINED BY algorithm OPTIONAL  }
                               -- contains a value of the type
                               -- registered for use with the
                               -- algorithm object identifier value
```
(from <https://www.rfc-editor.org/rfc/rfc5280#section-4.1.1.2>)

The outer sequence encoding is *not included*, so this is the DER encoding
of an OID for `algorithm` plus the `parameters` value.

For example, this is the `rsaEncryption` algorithm (but prefer to use the constant
[`RSA_ENCRYPTION`](alg_id/index.md) instead):

```rust
let rsa_encryption = rustls_pki_types::AlgorithmIdentifier::from_slice(
    &[
        // algorithm: 1.2.840.113549.1.1.1
        0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01,
        // parameters: NULL
        0x05, 0x00
    ]
);
assert_eq!(rustls_pki_types::alg_id::RSA_ENCRYPTION, rsa_encryption);
```

Common values for this type are provided in this module.

#### Implementations

- <span id="algorithmidentifier-from-slice"></span>`const fn from_slice(bytes: &'static [u8]) -> Self`

  Makes a new `AlgorithmIdentifier` from a static octet slice.

  

  This does not validate the contents of the slice.

#### Trait Implementations

##### `impl AsRef for AlgorithmIdentifier`

- <span id="algorithmidentifier-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for AlgorithmIdentifier`

- <span id="algorithmidentifier-clone"></span>`fn clone(&self) -> AlgorithmIdentifier` — [`AlgorithmIdentifier`](alg_id/index.md#algorithmidentifier)

##### `impl Copy for AlgorithmIdentifier`

##### `impl Debug for AlgorithmIdentifier`

- <span id="algorithmidentifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for AlgorithmIdentifier`

- <span id="algorithmidentifier-deref-type-target"></span>`type Target = [u8]`

- <span id="algorithmidentifier-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for AlgorithmIdentifier`

##### `impl PartialEq for AlgorithmIdentifier`

- <span id="algorithmidentifier-partialeq-eq"></span>`fn eq(&self, other: &AlgorithmIdentifier) -> bool` — [`AlgorithmIdentifier`](alg_id/index.md#algorithmidentifier)

##### `impl Receiver for AlgorithmIdentifier`

- <span id="algorithmidentifier-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for AlgorithmIdentifier`

### `AddrParseError`

```rust
struct AddrParseError(parser::AddrKind);
```

Failure to parse an IP address

#### Trait Implementations

##### `impl Clone for AddrParseError`

- <span id="addrparseerror-clone"></span>`fn clone(&self) -> AddrParseError` — [`AddrParseError`](server_name/index.md#addrparseerror)

##### `impl Copy for AddrParseError`

##### `impl Debug for AddrParseError`

- <span id="addrparseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for AddrParseError`

- <span id="addrparseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddrParseError`

##### `impl PartialEq for AddrParseError`

- <span id="addrparseerror-partialeq-eq"></span>`fn eq(&self, other: &AddrParseError) -> bool` — [`AddrParseError`](server_name/index.md#addrparseerror)

##### `impl StructuralPartialEq for AddrParseError`

##### `impl ToString for AddrParseError`

- <span id="addrparseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DnsName<'a>`

```rust
struct DnsName<'a>(DnsNameInner<'a>);
```

A type which encapsulates a string (borrowed or owned) that is a syntactically valid DNS name.

#### Implementations

- <span id="dnsname-borrow"></span>`fn borrow(self: &'a Self) -> Self`

  Produce a borrowed `DnsName` from this owned `DnsName`.

- <span id="dnsname-to-lowercase-owned"></span>`fn to_lowercase_owned(&self) -> DnsName<'static>` — [`DnsName`](server_name/index.md#dnsname)

  Copy this object to produce an owned `DnsName`, smashing the case to lowercase

  in one operation.

- <span id="dnsname-to-owned"></span>`fn to_owned(&self) -> DnsName<'static>` — [`DnsName`](server_name/index.md#dnsname)

  Produce an owned `DnsName` from this (potentially borrowed) `DnsName`.

- <span id="dnsname-try-from-string"></span>`fn try_from_string(s: String) -> Result<Self, String>`

- <span id="dnsname-try-from-str"></span>`const fn try_from_str(s: &str) -> Result<DnsName<'_>, InvalidDnsNameError>` — [`DnsName`](server_name/index.md#dnsname), [`InvalidDnsNameError`](server_name/index.md#invaliddnsnameerror)

  Produces a borrowed [`DnsName`](server_name/index.md) from a borrowed [`str`](#str).

#### Trait Implementations

##### `impl AsRef for DnsName<'_>`

- <span id="dnsname-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for DnsName<'a>`

- <span id="dnsname-clone"></span>`fn clone(&self) -> DnsName<'a>` — [`DnsName`](server_name/index.md#dnsname)

##### `impl Debug for DnsName<'a>`

- <span id="dnsname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DnsName<'a>`

##### `impl Hash for DnsName<'a>`

- <span id="dnsname-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DnsName<'a>`

- <span id="dnsname-partialeq-eq"></span>`fn eq(&self, other: &DnsName<'a>) -> bool` — [`DnsName`](server_name/index.md#dnsname)

##### `impl StructuralPartialEq for DnsName<'a>`

### `InvalidDnsNameError`

```rust
struct InvalidDnsNameError;
```

The provided input could not be parsed because
it is not a syntactically-valid DNS Name.

#### Trait Implementations

##### `impl Debug for InvalidDnsNameError`

- <span id="invaliddnsnameerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for InvalidDnsNameError`

- <span id="invaliddnsnameerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for InvalidDnsNameError`

- <span id="invaliddnsnameerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Ipv4Addr`

```rust
struct Ipv4Addr([u8; 4]);
```

`no_std` implementation of `std::net::Ipv4Addr`.

Note: because we intend to replace this type with `core::net::Ipv4Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv4Addr`.

#### Trait Implementations

##### `impl AsRef for Ipv4Addr`

- <span id="ipv4addr-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl Clone for Ipv4Addr`

- <span id="ipv4addr-clone"></span>`fn clone(&self) -> Ipv4Addr` — [`Ipv4Addr`](server_name/index.md#ipv4addr)

##### `impl Copy for Ipv4Addr`

##### `impl Debug for Ipv4Addr`

- <span id="ipv4addr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ipv4Addr`

##### `impl Hash for Ipv4Addr`

- <span id="ipv4addr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ipv4Addr`

- <span id="ipv4addr-partialeq-eq"></span>`fn eq(&self, other: &Ipv4Addr) -> bool` — [`Ipv4Addr`](server_name/index.md#ipv4addr)

##### `impl StructuralPartialEq for Ipv4Addr`

### `Ipv6Addr`

```rust
struct Ipv6Addr([u8; 16]);
```

`no_std` implementation of `std::net::Ipv6Addr`.

Note: because we intend to replace this type with `core::net::Ipv6Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv6Addr`.

#### Trait Implementations

##### `impl AsRef for Ipv6Addr`

- <span id="ipv6addr-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 16]`

##### `impl Clone for Ipv6Addr`

- <span id="ipv6addr-clone"></span>`fn clone(&self) -> Ipv6Addr` — [`Ipv6Addr`](server_name/index.md#ipv6addr)

##### `impl Copy for Ipv6Addr`

##### `impl Debug for Ipv6Addr`

- <span id="ipv6addr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ipv6Addr`

##### `impl Hash for Ipv6Addr`

- <span id="ipv6addr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ipv6Addr`

- <span id="ipv6addr-partialeq-eq"></span>`fn eq(&self, other: &Ipv6Addr) -> bool` — [`Ipv6Addr`](server_name/index.md#ipv6addr)

##### `impl StructuralPartialEq for Ipv6Addr`

### `PrivatePkcs1KeyDer<'a>`

```rust
struct PrivatePkcs1KeyDer<'a>(Der<'a>);
```

A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447

RSA private keys are identified in PEM context as `RSA PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs1KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs1KeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/rsa1024.pkcs1.pem");
PrivatePkcs1KeyDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- <span id="privatepkcs1keyder-clone-key"></span>`fn clone_key(&self) -> PrivatePkcs1KeyDer<'static>` — [`PrivatePkcs1KeyDer`](#privatepkcs1keyder)

  Clone the private key to a `'static` value

- <span id="privatepkcs1keyder-secret-pkcs1-der"></span>`fn secret_pkcs1_der(&self) -> &[u8]`

  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl Debug for PrivatePkcs1KeyDer<'_>`

- <span id="privatepkcs1keyder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PrivatePkcs1KeyDer<'a>`

##### `impl PartialEq for PrivatePkcs1KeyDer<'a>`

- <span id="privatepkcs1keyder-partialeq-eq"></span>`fn eq(&self, other: &PrivatePkcs1KeyDer<'a>) -> bool` — [`PrivatePkcs1KeyDer`](#privatepkcs1keyder)

##### `impl PemObject for PrivatePkcs1KeyDer<'a>`

- <span id="privatepkcs1keyder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for PrivatePkcs1KeyDer<'static>`

- <span id="privatepkcs1keyder-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl StructuralPartialEq for PrivatePkcs1KeyDer<'a>`

##### `impl Zeroize for PrivatePkcs1KeyDer<'static>`

- <span id="privatepkcs1keyder-zeroize"></span>`fn zeroize(&mut self)`

### `PrivateSec1KeyDer<'a>`

```rust
struct PrivateSec1KeyDer<'a>(Der<'a>);
```

A Sec1-encoded plaintext private key; as specified in RFC 5915

Sec1 private keys are identified in PEM context as `EC PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateSec1KeyDer, pem::PemObject};

// load from a PEM file
PrivateSec1KeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/nistp256key.pem");
PrivateSec1KeyDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- <span id="privatesec1keyder-clone-key"></span>`fn clone_key(&self) -> PrivateSec1KeyDer<'static>` — [`PrivateSec1KeyDer`](#privatesec1keyder)

  Clone the private key to a `'static` value

- <span id="privatesec1keyder-secret-sec1-der"></span>`fn secret_sec1_der(&self) -> &[u8]`

  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl Debug for PrivateSec1KeyDer<'_>`

- <span id="privatesec1keyder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PrivateSec1KeyDer<'a>`

##### `impl PartialEq for PrivateSec1KeyDer<'a>`

- <span id="privatesec1keyder-partialeq-eq"></span>`fn eq(&self, other: &PrivateSec1KeyDer<'a>) -> bool` — [`PrivateSec1KeyDer`](#privatesec1keyder)

##### `impl PemObject for PrivateSec1KeyDer<'a>`

- <span id="privatesec1keyder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for PrivateSec1KeyDer<'static>`

- <span id="privatesec1keyder-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl StructuralPartialEq for PrivateSec1KeyDer<'a>`

##### `impl Zeroize for PrivateSec1KeyDer<'static>`

- <span id="privatesec1keyder-zeroize"></span>`fn zeroize(&mut self)`

### `PrivatePkcs8KeyDer<'a>`

```rust
struct PrivatePkcs8KeyDer<'a>(Der<'a>);
```

A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958

PKCS#8 private keys are identified in PEM context as `PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs8KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs8KeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
PrivatePkcs8KeyDer::from_pem_file("tests/data/rsa1024.pkcs8.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/nistp256key.pkcs8.pem");
PrivatePkcs8KeyDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- <span id="privatepkcs8keyder-clone-key"></span>`fn clone_key(&self) -> PrivatePkcs8KeyDer<'static>` — [`PrivatePkcs8KeyDer`](#privatepkcs8keyder)

  Clone the private key to a `'static` value

- <span id="privatepkcs8keyder-secret-pkcs8-der"></span>`fn secret_pkcs8_der(&self) -> &[u8]`

  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl Debug for PrivatePkcs8KeyDer<'_>`

- <span id="privatepkcs8keyder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PrivatePkcs8KeyDer<'a>`

##### `impl PartialEq for PrivatePkcs8KeyDer<'a>`

- <span id="privatepkcs8keyder-partialeq-eq"></span>`fn eq(&self, other: &PrivatePkcs8KeyDer<'a>) -> bool` — [`PrivatePkcs8KeyDer`](#privatepkcs8keyder)

##### `impl PemObject for PrivatePkcs8KeyDer<'a>`

- <span id="privatepkcs8keyder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for PrivatePkcs8KeyDer<'static>`

- <span id="privatepkcs8keyder-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl StructuralPartialEq for PrivatePkcs8KeyDer<'a>`

##### `impl Zeroize for PrivatePkcs8KeyDer<'static>`

- <span id="privatepkcs8keyder-zeroize"></span>`fn zeroize(&mut self)`

### `TrustAnchor<'a>`

```rust
struct TrustAnchor<'a> {
    pub subject: Der<'a>,
    pub subject_public_key_info: Der<'a>,
    pub name_constraints: Option<Der<'a>>,
}
```

A trust anchor (a.k.a. root CA)

Traditionally, certificate verification libraries have represented trust anchors as full X.509
root certificates. However, those certificates contain a lot more data than is needed for
verifying certificates. The [`TrustAnchor`](#trustanchor) representation allows an application to store
just the essential elements of trust anchors.

The most common way to get one of these is to call `rustls_webpki::anchor_from_trusted_cert()`.


#### Fields

- **`subject`**: `Der<'a>`

  Value of the `subject` field of the trust anchor

- **`subject_public_key_info`**: `Der<'a>`

  Value of the `subjectPublicKeyInfo` field of the trust anchor

- **`name_constraints`**: `Option<Der<'a>>`

  Value of DER-encoded `NameConstraints`, containing name constraints to the trust anchor, if any

#### Implementations

- <span id="trustanchor-to-owned"></span>`fn to_owned(&self) -> TrustAnchor<'static>` — [`TrustAnchor`](#trustanchor)

  Yield a `'static` lifetime of the `TrustAnchor` by allocating owned `Der` variants

#### Trait Implementations

##### `impl Clone for TrustAnchor<'a>`

- <span id="trustanchor-clone"></span>`fn clone(&self) -> TrustAnchor<'a>` — [`TrustAnchor`](#trustanchor)

##### `impl Debug for TrustAnchor<'a>`

- <span id="trustanchor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TrustAnchor<'a>`

##### `impl Hash for TrustAnchor<'a>`

- <span id="trustanchor-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TrustAnchor<'a>`

- <span id="trustanchor-partialeq-eq"></span>`fn eq(&self, other: &TrustAnchor<'a>) -> bool` — [`TrustAnchor`](#trustanchor)

##### `impl StructuralPartialEq for TrustAnchor<'a>`

### `CertificateRevocationListDer<'a>`

```rust
struct CertificateRevocationListDer<'a>(Der<'a>);
```

A Certificate Revocation List; as specified in RFC 5280

Certificate revocation lists are identified in PEM context as `X509 CRL` and when stored in a
file usually use a `.crl` extension. For more on PEM files, refer to the crate documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateRevocationListDer, pem::PemObject};

// load several from a PEM file
let crls: Vec<_> = CertificateRevocationListDer::pem_file_iter("tests/data/crl.pem")
    .unwrap()
    .collect();
assert!(crls.len() >= 1);

// or one from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/crl.pem");
CertificateRevocationListDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let crls: Vec<_> = CertificateRevocationListDer::pem_slice_iter(byte_slice)
    .collect();
assert!(crls.len() >= 1);
}
```

#### Trait Implementations

##### `impl AsRef for CertificateRevocationListDer<'_>`

- <span id="certificaterevocationlistder-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for CertificateRevocationListDer<'a>`

- <span id="certificaterevocationlistder-clone"></span>`fn clone(&self) -> CertificateRevocationListDer<'a>` — [`CertificateRevocationListDer`](#certificaterevocationlistder)

##### `impl Debug for CertificateRevocationListDer<'a>`

- <span id="certificaterevocationlistder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for CertificateRevocationListDer<'_>`

- <span id="certificaterevocationlistder-deref-type-target"></span>`type Target = [u8]`

- <span id="certificaterevocationlistder-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for CertificateRevocationListDer<'a>`

##### `impl Hash for CertificateRevocationListDer<'a>`

- <span id="certificaterevocationlistder-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CertificateRevocationListDer<'a>`

- <span id="certificaterevocationlistder-partialeq-eq"></span>`fn eq(&self, other: &CertificateRevocationListDer<'a>) -> bool` — [`CertificateRevocationListDer`](#certificaterevocationlistder)

##### `impl PemObject for CertificateRevocationListDer<'a>`

- <span id="certificaterevocationlistder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for CertificateRevocationListDer<'static>`

- <span id="certificaterevocationlistder-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl Receiver for CertificateRevocationListDer<'a>`

- <span id="certificaterevocationlistder-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for CertificateRevocationListDer<'a>`

### `CertificateSigningRequestDer<'a>`

```rust
struct CertificateSigningRequestDer<'a>(Der<'a>);
```

A Certificate Signing Request; as specified in RFC 2986

Certificate signing requests are identified in PEM context as `CERTIFICATE REQUEST` and when stored in a
file usually use a `.csr` extension. For more on PEM files, refer to the crate documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateSigningRequestDer, pem::PemObject};

// load from a PEM file
CertificateSigningRequestDer::from_pem_file("tests/data/csr.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/csr.pem");
CertificateSigningRequestDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Trait Implementations

##### `impl AsRef for CertificateSigningRequestDer<'_>`

- <span id="certificatesigningrequestder-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for CertificateSigningRequestDer<'a>`

- <span id="certificatesigningrequestder-clone"></span>`fn clone(&self) -> CertificateSigningRequestDer<'a>` — [`CertificateSigningRequestDer`](#certificatesigningrequestder)

##### `impl Debug for CertificateSigningRequestDer<'a>`

- <span id="certificatesigningrequestder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for CertificateSigningRequestDer<'_>`

- <span id="certificatesigningrequestder-deref-type-target"></span>`type Target = [u8]`

- <span id="certificatesigningrequestder-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for CertificateSigningRequestDer<'a>`

##### `impl Hash for CertificateSigningRequestDer<'a>`

- <span id="certificatesigningrequestder-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CertificateSigningRequestDer<'a>`

- <span id="certificatesigningrequestder-partialeq-eq"></span>`fn eq(&self, other: &CertificateSigningRequestDer<'a>) -> bool` — [`CertificateSigningRequestDer`](#certificatesigningrequestder)

##### `impl PemObject for CertificateSigningRequestDer<'a>`

- <span id="certificatesigningrequestder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for CertificateSigningRequestDer<'static>`

- <span id="certificatesigningrequestder-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl Receiver for CertificateSigningRequestDer<'a>`

- <span id="certificatesigningrequestder-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for CertificateSigningRequestDer<'a>`

### `CertificateDer<'a>`

```rust
struct CertificateDer<'a>(Der<'a>);
```

A DER-encoded X.509 certificate; as specified in RFC 5280

Certificates are identified in PEM context as `CERTIFICATE` and when stored in a
file usually use a `.pem`, `.cer` or `.crt` extension. For more on PEM files, refer to the
crate documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateDer, pem::PemObject};

// load several from a PEM file
let certs: Vec<_> = CertificateDer::pem_file_iter("tests/data/certificate.chain.pem")
    .unwrap()
    .collect();
assert_eq!(certs.len(), 3);

// or one from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/certificate.chain.pem");
CertificateDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let certs: Vec<_> = CertificateDer::pem_slice_iter(byte_slice)
    .collect();
assert_eq!(certs.len(), 3);
}
```

#### Implementations

- <span id="certificateder-from-slice"></span>`const fn from_slice(bytes: &'a [u8]) -> Self`

  A const constructor to create a `CertificateDer` from a slice of DER.

#### Trait Implementations

##### `impl AsRef for CertificateDer<'_>`

- <span id="certificateder-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for CertificateDer<'a>`

- <span id="certificateder-clone"></span>`fn clone(&self) -> CertificateDer<'a>` — [`CertificateDer`](#certificateder)

##### `impl Debug for CertificateDer<'a>`

- <span id="certificateder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for CertificateDer<'_>`

- <span id="certificateder-deref-type-target"></span>`type Target = [u8]`

- <span id="certificateder-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for CertificateDer<'a>`

##### `impl Hash for CertificateDer<'a>`

- <span id="certificateder-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CertificateDer<'a>`

- <span id="certificateder-partialeq-eq"></span>`fn eq(&self, other: &CertificateDer<'a>) -> bool` — [`CertificateDer`](#certificateder)

##### `impl PemObject for CertificateDer<'a>`

- <span id="certificateder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for CertificateDer<'static>`

- <span id="certificateder-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl Receiver for CertificateDer<'a>`

- <span id="certificateder-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for CertificateDer<'a>`

### `SubjectPublicKeyInfoDer<'a>`

```rust
struct SubjectPublicKeyInfoDer<'a>(Der<'a>);
```

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

Public keys are identified in PEM context as a `PUBLIC KEY`.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{SubjectPublicKeyInfoDer, pem::PemObject};

// load from a PEM file
SubjectPublicKeyInfoDer::from_pem_file("tests/data/spki.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/spki.pem");
SubjectPublicKeyInfoDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- <span id="subjectpublickeyinfoder-into-owned"></span>`fn into_owned(self) -> SubjectPublicKeyInfoDer<'static>` — [`SubjectPublicKeyInfoDer`](#subjectpublickeyinfoder)

  Converts this SubjectPublicKeyInfo into its owned variant, unfreezing borrowed content (if any)

#### Trait Implementations

##### `impl AsRef for SubjectPublicKeyInfoDer<'_>`

- <span id="subjectpublickeyinfoder-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for SubjectPublicKeyInfoDer<'a>`

- <span id="subjectpublickeyinfoder-clone"></span>`fn clone(&self) -> SubjectPublicKeyInfoDer<'a>` — [`SubjectPublicKeyInfoDer`](#subjectpublickeyinfoder)

##### `impl Debug for SubjectPublicKeyInfoDer<'a>`

- <span id="subjectpublickeyinfoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for SubjectPublicKeyInfoDer<'_>`

- <span id="subjectpublickeyinfoder-deref-type-target"></span>`type Target = [u8]`

- <span id="subjectpublickeyinfoder-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for SubjectPublicKeyInfoDer<'a>`

##### `impl Hash for SubjectPublicKeyInfoDer<'a>`

- <span id="subjectpublickeyinfoder-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SubjectPublicKeyInfoDer<'a>`

- <span id="subjectpublickeyinfoder-partialeq-eq"></span>`fn eq(&self, other: &SubjectPublicKeyInfoDer<'a>) -> bool` — [`SubjectPublicKeyInfoDer`](#subjectpublickeyinfoder)

##### `impl PemObject for SubjectPublicKeyInfoDer<'a>`

- <span id="subjectpublickeyinfoder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for SubjectPublicKeyInfoDer<'static>`

- <span id="subjectpublickeyinfoder-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl Receiver for SubjectPublicKeyInfoDer<'a>`

- <span id="subjectpublickeyinfoder-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for SubjectPublicKeyInfoDer<'a>`

### `EchConfigListBytes<'a>`

```rust
struct EchConfigListBytes<'a>(BytesInner<'a>);
```

A TLS-encoded Encrypted Client Hello (ECH) configuration list (`ECHConfigList`); as specified in
[draft-ietf-tls-esni-18 §4](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-4)

#### Implementations

- <span id="echconfiglistbytes-into-owned"></span>`fn into_owned(self) -> EchConfigListBytes<'static>` — [`EchConfigListBytes`](#echconfiglistbytes)

  Converts this config into its owned variant, unfreezing borrowed content (if any)

#### Trait Implementations

##### `impl AsRef for EchConfigListBytes<'_>`

- <span id="echconfiglistbytes-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for EchConfigListBytes<'a>`

- <span id="echconfiglistbytes-clone"></span>`fn clone(&self) -> EchConfigListBytes<'a>` — [`EchConfigListBytes`](#echconfiglistbytes)

##### `impl Debug for EchConfigListBytes<'_>`

- <span id="echconfiglistbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for EchConfigListBytes<'_>`

- <span id="echconfiglistbytes-deref-type-target"></span>`type Target = [u8]`

- <span id="echconfiglistbytes-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for EchConfigListBytes<'a>`

##### `impl Hash for EchConfigListBytes<'a>`

- <span id="echconfiglistbytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for EchConfigListBytes<'a>`

- <span id="echconfiglistbytes-partialeq-eq"></span>`fn eq(&self, other: &EchConfigListBytes<'a>) -> bool` — [`EchConfigListBytes`](#echconfiglistbytes)

##### `impl PemObject for EchConfigListBytes<'a>`

- <span id="echconfiglistbytes-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl PemObjectFilter for EchConfigListBytes<'static>`

- <span id="echconfiglistbytes-pemobjectfilter-const-kind"></span>`const KIND: SectionKind`

##### `impl Receiver for EchConfigListBytes<'a>`

- <span id="echconfiglistbytes-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for EchConfigListBytes<'a>`

### `InvalidSignature`

```rust
struct InvalidSignature;
```

A detail-less error when a signature is not valid.

#### Trait Implementations

##### `impl Clone for InvalidSignature`

- <span id="invalidsignature-clone"></span>`fn clone(&self) -> InvalidSignature` — [`InvalidSignature`](#invalidsignature)

##### `impl Copy for InvalidSignature`

##### `impl Debug for InvalidSignature`

- <span id="invalidsignature-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnixTime`

```rust
struct UnixTime(u64);
```

A timestamp, tracking the number of non-leap seconds since the Unix epoch.

The Unix epoch is defined January 1, 1970 00:00:00 UTC.

#### Implementations

- <span id="unixtime-since-unix-epoch"></span>`const fn since_unix_epoch(duration: Duration) -> Self`

  Convert a `Duration` since the start of 1970 to a `UnixTime`

  

  The `duration` must be relative to the Unix epoch.

- <span id="unixtime-as-secs"></span>`const fn as_secs(&self) -> u64`

  Number of seconds since the Unix epoch

#### Trait Implementations

##### `impl Clone for UnixTime`

- <span id="unixtime-clone"></span>`fn clone(&self) -> UnixTime` — [`UnixTime`](#unixtime)

##### `impl Copy for UnixTime`

##### `impl Debug for UnixTime`

- <span id="unixtime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnixTime`

##### `impl Hash for UnixTime`

- <span id="unixtime-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for UnixTime`

- <span id="unixtime-ord-cmp"></span>`fn cmp(&self, other: &UnixTime) -> cmp::Ordering` — [`UnixTime`](#unixtime)

##### `impl PartialEq for UnixTime`

- <span id="unixtime-partialeq-eq"></span>`fn eq(&self, other: &UnixTime) -> bool` — [`UnixTime`](#unixtime)

##### `impl PartialOrd for UnixTime`

- <span id="unixtime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnixTime) -> option::Option<cmp::Ordering>` — [`UnixTime`](#unixtime)

##### `impl StructuralPartialEq for UnixTime`

### `Der<'a>`

```rust
struct Der<'a>(BytesInner<'a>);
```

DER-encoded data, either owned or borrowed

This wrapper type is used to represent DER-encoded data in a way that is agnostic to whether
the data is owned (by a `Vec<u8>`) or borrowed (by a `&[u8]`). Support for the owned
variant is only available when the `alloc` feature is enabled.

#### Implementations

- <span id="der-from-slice"></span>`const fn from_slice(der: &'a [u8]) -> Self`

  A const constructor to create a `Der` from a borrowed slice

#### Trait Implementations

##### `impl AsRef for Der<'_>`

- <span id="der-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for Der<'a>`

- <span id="der-clone"></span>`fn clone(&self) -> Der<'a>` — [`Der`](#der)

##### `impl Debug for Der<'_>`

- <span id="der-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Der<'_>`

- <span id="der-deref-type-target"></span>`type Target = [u8]`

- <span id="der-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for Der<'a>`

##### `impl Hash for Der<'a>`

- <span id="der-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Der<'a>`

- <span id="der-partialeq-eq"></span>`fn eq(&self, other: &Der<'a>) -> bool` — [`Der`](#der)

##### `impl Receiver for Der<'a>`

- <span id="der-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Der<'a>`

## Enums

### `IpAddr`

```rust
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

`no_std` implementation of `std::net::IpAddr`.

Note: because we intend to replace this type with `core::net::IpAddr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::IpAddr`.

#### Variants

- **`V4`**

  An Ipv4 address.

- **`V6`**

  An Ipv6 address.

#### Trait Implementations

##### `impl Clone for IpAddr`

- <span id="ipaddr-clone"></span>`fn clone(&self) -> IpAddr` — [`IpAddr`](server_name/index.md#ipaddr)

##### `impl Copy for IpAddr`

##### `impl Debug for IpAddr`

- <span id="ipaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IpAddr`

##### `impl Hash for IpAddr`

- <span id="ipaddr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for IpAddr`

- <span id="ipaddr-partialeq-eq"></span>`fn eq(&self, other: &IpAddr) -> bool` — [`IpAddr`](server_name/index.md#ipaddr)

##### `impl StructuralPartialEq for IpAddr`

### `ServerName<'a>`

```rust
enum ServerName<'a> {
    DnsName(DnsName<'a>),
    IpAddress(IpAddr),
}
```

Encodes ways a client can know the expected name of the server.

This currently covers knowing the DNS name of the server, but
will be extended in the future to supporting privacy-preserving names
for the server ("ECH").  For this reason this enum is `non_exhaustive`.

# Making one

If you have a DNS name as a `&str`, this type implements `TryFrom<&str>`,
so you can do:

```rust
use rustls_pki_types::ServerName;
ServerName::try_from("example.com").expect("invalid DNS name");
```

If you have an owned `String`, you can use `TryFrom` directly:

```rust
use rustls_pki_types::ServerName;
let name = "example.com".to_string();
#[cfg(feature = "alloc")]
ServerName::try_from(name).expect("invalid DNS name");
```

which will yield a `ServerName<'static>` if successful.

or, alternatively...

```rust
use rustls_pki_types::ServerName;
let x: ServerName = "example.com".try_into().expect("invalid DNS name");
```

#### Variants

- **`DnsName`**

  The server is identified by a DNS name.  The name
  is sent in the TLS Server Name Indication (SNI)
  extension.

- **`IpAddress`**

  The server is identified by an IP address. SNI is not
  done.

#### Implementations

- <span id="servername-to-owned"></span>`fn to_owned(&self) -> ServerName<'static>` — [`ServerName`](server_name/index.md#servername)

  Produce an owned `ServerName` from this (potentially borrowed) `ServerName`.

#### Trait Implementations

##### `impl Clone for ServerName<'a>`

- <span id="servername-clone"></span>`fn clone(&self) -> ServerName<'a>` — [`ServerName`](server_name/index.md#servername)

##### `impl Debug for ServerName<'_>`

- <span id="servername-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ServerName<'a>`

##### `impl Hash for ServerName<'a>`

- <span id="servername-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ServerName<'a>`

- <span id="servername-partialeq-eq"></span>`fn eq(&self, other: &ServerName<'a>) -> bool` — [`ServerName`](server_name/index.md#servername)

##### `impl StructuralPartialEq for ServerName<'a>`

### `PrivateKeyDer<'a>`

```rust
enum PrivateKeyDer<'a> {
    Pkcs1(PrivatePkcs1KeyDer<'a>),
    Sec1(PrivateSec1KeyDer<'a>),
    Pkcs8(PrivatePkcs8KeyDer<'a>),
}
```

A DER-encoded X.509 private key, in one of several formats

See variant inner types for more detailed information.

This can load several types of PEM-encoded private key, and then reveal
which types were found:

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateKeyDer, pem::PemObject};

// load from a PEM file
let pkcs8 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
let pkcs1 = PrivateKeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();
let sec1 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();
assert!(matches!(pkcs8, PrivateKeyDer::Pkcs8(_)));
assert!(matches!(pkcs1, PrivateKeyDer::Pkcs1(_)));
assert!(matches!(sec1, PrivateKeyDer::Sec1(_)));
}
```

#### Variants

- **`Pkcs1`**

  An RSA private key

- **`Sec1`**

  A Sec1 private key

- **`Pkcs8`**

  A PKCS#8 private key

#### Implementations

- <span id="privatekeyder-clone-key"></span>`fn clone_key(&self) -> PrivateKeyDer<'static>` — [`PrivateKeyDer`](#privatekeyder)

  Clone the private key to a `'static` value

- <span id="privatekeyder-secret-der"></span>`fn secret_der(&self) -> &[u8]`

  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl Debug for PrivateKeyDer<'a>`

- <span id="privatekeyder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PrivateKeyDer<'a>`

##### `impl PartialEq for PrivateKeyDer<'a>`

- <span id="privatekeyder-partialeq-eq"></span>`fn eq(&self, other: &PrivateKeyDer<'a>) -> bool` — [`PrivateKeyDer`](#privatekeyder)

##### `impl PemObject for PrivateKeyDer<'static>`

- <span id="privatekeyder-pemobject-from-pem"></span>`fn from_pem(kind: SectionKind, value: Vec<u8>) -> Option<Self>` — [`SectionKind`](pem/index.md#sectionkind)

##### `impl StructuralPartialEq for PrivateKeyDer<'a>`

##### `impl Zeroize for PrivateKeyDer<'static>`

- <span id="privatekeyder-zeroize"></span>`fn zeroize(&mut self)`

### `BytesInner<'a>`

```rust
enum BytesInner<'a> {
    Owned(alloc::vec::Vec<u8>),
    Borrowed(&'a [u8]),
}
```

#### Implementations

- <span id="bytesinner-into-owned"></span>`fn into_owned(self) -> BytesInner<'static>` — [`BytesInner`](#bytesinner)

#### Trait Implementations

##### `impl AsRef for BytesInner<'_>`

- <span id="bytesinner-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for BytesInner<'a>`

- <span id="bytesinner-clone"></span>`fn clone(&self) -> BytesInner<'a>` — [`BytesInner`](#bytesinner)

##### `impl Debug for BytesInner<'a>`

- <span id="bytesinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BytesInner<'_>`

##### `impl Hash for BytesInner<'_>`

- <span id="bytesinner-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl PartialEq for BytesInner<'_>`

- <span id="bytesinner-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Zeroize for BytesInner<'static>`

- <span id="bytesinner-zeroize"></span>`fn zeroize(&mut self)`

### `FipsStatus`

```rust
enum FipsStatus {
    Unvalidated,
    Pending,
    Certified {
        certificate: &'static str,
    },
}
```

FIPS validation status of an algorithm or implementation.

#### Variants

- **`Unvalidated`**

  Not FIPS tested, or unapproved algorithm.

- **`Pending`**

  In queue for FIPS validation.

- **`Certified`**

  FIPS certified, with named certificate.

#### Trait Implementations

##### `impl Clone for FipsStatus`

- <span id="fipsstatus-clone"></span>`fn clone(&self) -> FipsStatus` — [`FipsStatus`](#fipsstatus)

##### `impl Copy for FipsStatus`

##### `impl Debug for FipsStatus`

- <span id="fipsstatus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FipsStatus`

##### `impl Ord for FipsStatus`

- <span id="fipsstatus-ord-cmp"></span>`fn cmp(&self, other: &FipsStatus) -> cmp::Ordering` — [`FipsStatus`](#fipsstatus)

##### `impl PartialEq for FipsStatus`

- <span id="fipsstatus-partialeq-eq"></span>`fn eq(&self, other: &FipsStatus) -> bool` — [`FipsStatus`](#fipsstatus)

##### `impl PartialOrd for FipsStatus`

- <span id="fipsstatus-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &FipsStatus) -> option::Option<cmp::Ordering>` — [`FipsStatus`](#fipsstatus)

##### `impl StructuralPartialEq for FipsStatus`

## Traits

### `SignatureVerificationAlgorithm`

```rust
trait SignatureVerificationAlgorithm: Send + Sync + fmt::Debug { ... }
```

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

#### Required Methods

- `fn verify_signature(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature>`

  Verify a signature.

- `fn public_key_alg_id(&self) -> AlgorithmIdentifier`

  Return the `AlgorithmIdentifier` that must equal a public key's

- `fn signature_alg_id(&self) -> AlgorithmIdentifier`

  Return the `AlgorithmIdentifier` that must equal the `signatureAlgorithm` value

#### Provided Methods

- `fn fips_status(&self) -> FipsStatus`

  Return the FIPS status of this algorithm or implementation.

- `fn fips(&self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

## Functions

### `hex`

```rust
fn hex<'a>(f: &mut fmt::Formatter<'_>, payload: impl IntoIterator<Item = &'a u8>) -> fmt::Result
```

## Type Aliases

### `SubjectPublicKeyInfo<'a>`

```rust
type SubjectPublicKeyInfo<'a> = SubjectPublicKeyInfoDer<'a>;
```

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

