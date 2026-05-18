**rustls > crypto > signer**

# Module: crypto::signer

## Contents

**Structs**

- [`CertifiedKey`](#certifiedkey) - A packaged-together certificate chain, matching `SigningKey` and
- [`SingleCertAndKey`](#singlecertandkey) - Server certificate resolver which always resolves to the same certificate and key.

**Functions**

- [`public_key_to_spki`](#public_key_to_spki) - Convert a public key and algorithm identifier into [`SubjectPublicKeyInfoDer`].

**Traits**

- [`Signer`](#signer) - A thing that can sign a message.
- [`SigningKey`](#signingkey) - An abstract signing key.

---

## rustls::crypto::signer::CertifiedKey

*Struct*

A packaged-together certificate chain, matching `SigningKey` and
optional stapled OCSP response.

Note: this struct is also used to represent an [RFC 7250] raw public key,
when the client/server is configured to use raw public keys instead of
certificates.

[RFC 7250]: https://tools.ietf.org/html/rfc7250

**Fields:**
- `cert: alloc::vec::Vec<pki_types::CertificateDer<'static>>` - The certificate chain or raw public key.
- `key: alloc::sync::Arc<dyn SigningKey>` - The certified key.
- `ocsp: Option<alloc::vec::Vec<u8>>` - An optional OCSP response from the certificate issuer,

**Methods:**

- `fn from_der(cert_chain: Vec<CertificateDer<'static>>, key: PrivateKeyDer<'static>, provider: &CryptoProvider) -> Result<Self, Error>` - Create a new `CertifiedKey` from a certificate chain and DER-encoded private key.
- `fn new(cert: Vec<CertificateDer<'static>>, key: alloc::sync::Arc<dyn SigningKey>) -> Self` - Make a new CertifiedKey, with the given chain and key.
- `fn keys_match(self: &Self) -> Result<(), Error>` - Verify the consistency of this [`CertifiedKey`]'s public and private keys.
- `fn end_entity_cert(self: &Self) -> Result<&CertificateDer, Error>` - The end-entity certificate.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CertifiedKey`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::crypto::signer::Signer

*Trait*

A thing that can sign a message.

**Methods:**

- `sign`: Signs `message` using the selected scheme.
- `scheme`: Reveals which scheme will be used when you call [`Self::sign()`].



## rustls::crypto::signer::SigningKey

*Trait*

An abstract signing key.

This interface is used by rustls to use a private signing key
for authentication.  This includes server and client authentication.

Objects of this type are always used within Rustls as
`Arc<dyn SigningKey>`. There are no concrete public structs in Rustls
that implement this trait.

There are two main ways to get a signing key:

 - [`KeyProvider::load_private_key()`], or
 - some other method outside of the `KeyProvider` extension trait,
   for instance:
   - [`crypto::ring::sign::any_ecdsa_type()`]
   - [`crypto::ring::sign::any_eddsa_type()`]
   - [`crypto::ring::sign::any_supported_type()`]
   - [`crypto::aws_lc_rs::sign::any_ecdsa_type()`]
   - [`crypto::aws_lc_rs::sign::any_eddsa_type()`]
   - [`crypto::aws_lc_rs::sign::any_supported_type()`]

The `KeyProvider` method `load_private_key()` is called under the hood by
[`ConfigBuilder::with_single_cert()`],
[`ConfigBuilder::with_client_auth_cert()`], and
[`ConfigBuilder::with_single_cert_with_ocsp()`].

A signing key created outside of the `KeyProvider` extension trait can be used
to create a [`CertifiedKey`], which in turn can be used to create a
[`ResolvesServerCertUsingSni`]. Alternately, a `CertifiedKey` can be returned from a
custom implementation of the [`ResolvesServerCert`] or [`ResolvesClientCert`] traits.

[`KeyProvider::load_private_key()`]: crate::crypto::KeyProvider::load_private_key
[`ConfigBuilder::with_single_cert()`]: crate::ConfigBuilder::with_single_cert
[`ConfigBuilder::with_single_cert_with_ocsp()`]: crate::ConfigBuilder::with_single_cert_with_ocsp
[`ConfigBuilder::with_client_auth_cert()`]: crate::ConfigBuilder::with_client_auth_cert
[`crypto::ring::sign::any_ecdsa_type()`]: crate::crypto::ring::sign::any_ecdsa_type
[`crypto::ring::sign::any_eddsa_type()`]: crate::crypto::ring::sign::any_eddsa_type
[`crypto::ring::sign::any_supported_type()`]: crate::crypto::ring::sign::any_supported_type
[`crypto::aws_lc_rs::sign::any_ecdsa_type()`]: crate::crypto::aws_lc_rs::sign::any_ecdsa_type
[`crypto::aws_lc_rs::sign::any_eddsa_type()`]: crate::crypto::aws_lc_rs::sign::any_eddsa_type
[`crypto::aws_lc_rs::sign::any_supported_type()`]: crate::crypto::aws_lc_rs::sign::any_supported_type
[`ResolvesServerCertUsingSni`]: crate::server::ResolvesServerCertUsingSni
[`ResolvesServerCert`]: crate::server::ResolvesServerCert
[`ResolvesClientCert`]: crate::client::ResolvesClientCert

**Methods:**

- `choose_scheme`: Choose a `SignatureScheme` from those offered.
- `public_key`: Get the RFC 5280-compliant SubjectPublicKeyInfo (SPKI) of this [`SigningKey`] if available.
- `algorithm`: What kind of key we have.



## rustls::crypto::signer::SingleCertAndKey

*Struct*

Server certificate resolver which always resolves to the same certificate and key.

For use with [`ConfigBuilder::with_cert_resolver()`].

[`ConfigBuilder::with_cert_resolver()`]: crate::ConfigBuilder::with_cert_resolver

**Tuple Struct**: `()`

**Trait Implementations:**

- **From**
  - `fn from(certified_key: CertifiedKey) -> Self`
- **From**
  - `fn from(certified_key: alloc::sync::Arc<CertifiedKey>) -> Self`
- **ResolvesClientCert**
  - `fn resolve(self: &Self, _root_hint_subjects: &[&[u8]], _sigschemes: &[SignatureScheme]) -> Option<alloc::sync::Arc<CertifiedKey>>`
  - `fn has_certs(self: &Self) -> bool`
- **ResolvesServerCert**
  - `fn resolve(self: &Self, _client_hello: ClientHello) -> Option<alloc::sync::Arc<CertifiedKey>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::crypto::signer::public_key_to_spki

*Function*

Convert a public key and algorithm identifier into [`SubjectPublicKeyInfoDer`].

```rust
fn public_key_to_spki<impl AsRef<[u8]>>(alg_id: &pki_types::AlgorithmIdentifier, public_key: impl Trait) -> pki_types::SubjectPublicKeyInfoDer<'static>
```



