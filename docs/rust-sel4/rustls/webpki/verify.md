**rustls > webpki > verify**

# Module: webpki::verify

## Contents

**Structs**

- [`ParsedCertificate`](#parsedcertificate) - Wrapper around internal representation of a parsed certificate.
- [`WebPkiSupportedAlgorithms`](#webpkisupportedalgorithms) - Describes which `webpki` signature verification algorithms are supported and

**Functions**

- [`verify_server_cert_signed_by_trust_anchor`](#verify_server_cert_signed_by_trust_anchor) - Verify that the end-entity certificate `end_entity` is a valid server cert
- [`verify_server_name`](#verify_server_name) - Verify that the `end_entity` has an alternative name matching the `server_name`.
- [`verify_tls12_signature`](#verify_tls12_signature) - Verify a message signature using the `cert` public key and any supported scheme.
- [`verify_tls13_signature`](#verify_tls13_signature) - Verify a message signature using the `cert` public key and the first TLS 1.3 compatible
- [`verify_tls13_signature_with_raw_key`](#verify_tls13_signature_with_raw_key) - Verify a message signature using a raw public key and the first TLS 1.3 compatible

---

## rustls::webpki::verify::ParsedCertificate

*Struct*

Wrapper around internal representation of a parsed certificate.

This is used in order to avoid parsing twice when specifying custom verification

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn subject_public_key_info(self: &Self) -> SubjectPublicKeyInfoDer<'static>` - Get the parsed certificate's SubjectPublicKeyInfo (SPKI)

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(value: &'a CertificateDer<'a>) -> Result<Self, <Self as >::Error>`



## rustls::webpki::verify::WebPkiSupportedAlgorithms

*Struct*

Describes which `webpki` signature verification algorithms are supported and
how they map to TLS [`SignatureScheme`]s.

**Fields:**
- `all: &'static [&'static dyn SignatureVerificationAlgorithm]` - A list of all supported signature verification algorithms.
- `mapping: &'static [(crate::enums::SignatureScheme, &'static [&'static dyn SignatureVerificationAlgorithm])]` - A mapping from TLS `SignatureScheme`s to matching webpki signature verification algorithms.

**Methods:**

- `fn supported_schemes(self: &Self) -> Vec<SignatureScheme>` - Return all the `scheme` items in `mapping`, maintaining order.
- `fn fips(self: &Self) -> bool` - Return `true` if all cryptography is FIPS-approved.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> WebPkiSupportedAlgorithms`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rustls::webpki::verify::verify_server_cert_signed_by_trust_anchor

*Function*

Verify that the end-entity certificate `end_entity` is a valid server cert
and chains to at least one of the trust anchors in the `roots` [RootCertStore].

This function is primarily useful when building a custom certificate verifier. It
performs **no revocation checking**. Implementers must handle this themselves,
along with checking that the server certificate is valid for the subject name
being used (see [`verify_server_name`]).

`intermediates` contains all certificates other than `end_entity` that
were sent as part of the server's `Certificate` message. It is in the
same order that the server sent them and may be empty.

```rust
fn verify_server_cert_signed_by_trust_anchor(cert: &ParsedCertificate, roots: &super::anchors::RootCertStore, intermediates: &[pki_types::CertificateDer], now: pki_types::UnixTime, supported_algs: &[&dyn SignatureVerificationAlgorithm]) -> Result<(), crate::error::Error>
```



## rustls::webpki::verify::verify_server_name

*Function*

Verify that the `end_entity` has an alternative name matching the `server_name`.

Note: this only verifies the name and should be used in conjunction with more verification
like [verify_server_cert_signed_by_trust_anchor]

```rust
fn verify_server_name(cert: &ParsedCertificate, server_name: &pki_types::ServerName) -> Result<(), crate::error::Error>
```



## rustls::webpki::verify::verify_tls12_signature

*Function*

Verify a message signature using the `cert` public key and any supported scheme.

This function verifies the `dss` signature over `message` using the subject public key from
`cert`. Since TLS 1.2 doesn't provide enough information to map the `dss.scheme` into a single
[`SignatureVerificationAlgorithm`], this function will map to several candidates and try each in
succession until one succeeds or we exhaust all candidates.

See [WebPkiSupportedAlgorithms::mapping] for more information.

```rust
fn verify_tls12_signature(message: &[u8], cert: &pki_types::CertificateDer, dss: &crate::verify::DigitallySignedStruct, supported_schemes: &WebPkiSupportedAlgorithms) -> Result<crate::verify::HandshakeSignatureValid, crate::error::Error>
```



## rustls::webpki::verify::verify_tls13_signature

*Function*

Verify a message signature using the `cert` public key and the first TLS 1.3 compatible
supported scheme.

This function verifies the `dss` signature over `message` using the subject public key from
`cert`. Unlike [verify_tls12_signature], this function only tries the first matching scheme. See
[WebPkiSupportedAlgorithms::mapping] for more information.

```rust
fn verify_tls13_signature(msg: &[u8], cert: &pki_types::CertificateDer, dss: &crate::verify::DigitallySignedStruct, supported_schemes: &WebPkiSupportedAlgorithms) -> Result<crate::verify::HandshakeSignatureValid, crate::error::Error>
```



## rustls::webpki::verify::verify_tls13_signature_with_raw_key

*Function*

Verify a message signature using a raw public key and the first TLS 1.3 compatible
supported scheme.

```rust
fn verify_tls13_signature_with_raw_key(msg: &[u8], spki: &pki_types::SubjectPublicKeyInfoDer, dss: &crate::verify::DigitallySignedStruct, supported_schemes: &WebPkiSupportedAlgorithms) -> Result<crate::verify::HandshakeSignatureValid, crate::error::Error>
```



