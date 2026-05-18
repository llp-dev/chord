**rustls > verify**

# Module: verify

## Contents

**Structs**

- [`ClientCertVerified`](#clientcertverified) - Zero-sized marker type representing verification of a client cert chain.
- [`DigitallySignedStruct`](#digitallysignedstruct) - This type combines a [`SignatureScheme`] and a signature payload produced with that scheme.
- [`HandshakeSignatureValid`](#handshakesignaturevalid) - Zero-sized marker type representing verification of a signature.
- [`NoClientAuth`](#noclientauth) - Turns off client authentication.
- [`ServerCertVerified`](#servercertverified) - Zero-sized marker type representing verification of a server cert chain.

**Traits**

- [`ClientCertVerifier`](#clientcertverifier) - Something that can verify a client certificate chain
- [`ServerCertVerifier`](#servercertverifier) - Something that can verify a server certificate chain, and verify

---

## rustls::verify::ClientCertVerified

*Struct*

Zero-sized marker type representing verification of a client cert chain.

**Tuple Struct**: `()`

**Methods:**

- `fn assertion() -> Self` - Make a `ClientCertVerified`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::verify::ClientCertVerifier

*Trait*

Something that can verify a client certificate chain

**Methods:**

- `offer_client_auth`: Returns `true` to enable the server to request a client certificate and
- `client_auth_mandatory`: Return `true` to require a client certificate and `false` to make
- `root_hint_subjects`: Returns the [`DistinguishedName`] [subjects] that the server will hint to clients to
- `verify_client_cert`: Verify the end-entity certificate `end_entity` is valid, acceptable,
- `verify_tls12_signature`: Verify a signature allegedly by the given client certificate.
- `verify_tls13_signature`: Verify a signature allegedly by the given client certificate.
- `supported_verify_schemes`: Return the list of SignatureSchemes that this verifier will handle,
- `requires_raw_public_keys`: Returns whether this verifier requires raw public keys as defined



## rustls::verify::DigitallySignedStruct

*Struct*

This type combines a [`SignatureScheme`] and a signature payload produced with that scheme.

**Fields:**
- `scheme: crate::enums::SignatureScheme` - The [`SignatureScheme`] used to produce the signature.

**Methods:**

- `fn signature(self: &Self) -> &[u8]` - Get the signature.

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DigitallySignedStruct`



## rustls::verify::HandshakeSignatureValid

*Struct*

Zero-sized marker type representing verification of a signature.

**Tuple Struct**: `()`

**Methods:**

- `fn assertion() -> Self` - Make a `HandshakeSignatureValid`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::verify::NoClientAuth

*Struct*

Turns off client authentication.

In contrast to using
`WebPkiClientVerifier::builder(roots).allow_unauthenticated().build()`, the `NoClientAuth`
`ClientCertVerifier` will not offer client authentication at all, vs offering but not
requiring it.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ClientCertVerifier**
  - `fn offer_client_auth(self: &Self) -> bool`
  - `fn root_hint_subjects(self: &Self) -> &[DistinguishedName]`
  - `fn verify_client_cert(self: &Self, _end_entity: &CertificateDer, _intermediates: &[CertificateDer], _now: UnixTime) -> Result<ClientCertVerified, Error>`
  - `fn verify_tls12_signature(self: &Self, _message: &[u8], _cert: &CertificateDer, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn verify_tls13_signature(self: &Self, _message: &[u8], _cert: &CertificateDer, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn supported_verify_schemes(self: &Self) -> Vec<SignatureScheme>`



## rustls::verify::ServerCertVerified

*Struct*

Zero-sized marker type representing verification of a server cert chain.

**Tuple Struct**: `()`

**Methods:**

- `fn assertion() -> Self` - Make a `ServerCertVerified`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::verify::ServerCertVerifier

*Trait*

Something that can verify a server certificate chain, and verify
signatures made by certificates.

**Methods:**

- `verify_server_cert`: Verify the end-entity certificate `end_entity` is valid for the
- `verify_tls12_signature`: Verify a signature allegedly by the given server certificate.
- `verify_tls13_signature`: Verify a signature allegedly by the given server certificate.
- `supported_verify_schemes`: Return the list of SignatureSchemes that this verifier will handle,
- `requires_raw_public_keys`: Returns whether this verifier requires raw public keys as defined
- `root_hint_subjects`: Return the [`DistinguishedName`]s of certificate authorities that this verifier trusts.



