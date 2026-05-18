**rustls > webpki > client_verifier**

# Module: webpki::client_verifier

## Contents

**Structs**

- [`ClientCertVerifierBuilder`](#clientcertverifierbuilder) - A builder for configuring a `webpki` client certificate verifier.
- [`WebPkiClientVerifier`](#webpkiclientverifier) - A client certificate verifier that uses the `webpki` crate[^1] to perform client certificate

---

## rustls::webpki::client_verifier::ClientCertVerifierBuilder

*Struct*

A builder for configuring a `webpki` client certificate verifier.

For more information, see the [`WebPkiClientVerifier`] documentation.

**Methods:**

- `fn clear_root_hint_subjects(self: Self) -> Self` - Clear the list of trust anchor hint subjects.
- `fn add_root_hint_subjects<impl IntoIterator<Item = DistinguishedName>>(self: Self, subjects: impl Trait) -> Self` - Add additional [`DistinguishedName`]s to the list of trust anchor hint subjects.
- `fn with_crls<impl IntoIterator<Item = CertificateRevocationListDer<'static>>>(self: Self, crls: impl Trait) -> Self` - Verify the revocation state of presented client certificates against the provided
- `fn only_check_end_entity_revocation(self: Self) -> Self` - Only check the end entity certificate revocation status when using CRLs.
- `fn allow_unauthenticated(self: Self) -> Self` - Allow unauthenticated clients to connect.
- `fn allow_unknown_revocation_status(self: Self) -> Self` - Allow unknown certificate revocation status when using CRLs.
- `fn enforce_revocation_expiration(self: Self) -> Self` - Enforce the CRL nextUpdate field (i.e. expiration)
- `fn build(self: Self) -> Result<alloc::sync::Arc<dyn ClientCertVerifier>, VerifierBuilderError>` - Build a client certificate verifier. The built verifier will be used for the server to offer

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ClientCertVerifierBuilder`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::webpki::client_verifier::WebPkiClientVerifier

*Struct*

A client certificate verifier that uses the `webpki` crate[^1] to perform client certificate
validation.

It must be created via the [`WebPkiClientVerifier::builder()`] or
[`WebPkiClientVerifier::builder_with_provider()`] functions.

Once built, the provided `Arc<dyn ClientCertVerifier>` can be used with a Rustls [`ServerConfig`]
to configure client certificate validation using [`with_client_cert_verifier`][ConfigBuilder<ClientConfig, WantsVerifier>::with_client_cert_verifier].

Example:

To require all clients present a client certificate issued by a trusted CA:
```no_run
# #[cfg(any(feature = "ring", feature = "aws_lc_rs"))] {
# use rustls::RootCertStore;
# use rustls::server::WebPkiClientVerifier;
# let roots = RootCertStore::empty();
let client_verifier = WebPkiClientVerifier::builder(roots.into())
  .build()
  .unwrap();
# }
```

Or, to allow clients presenting a client certificate authenticated by a trusted CA, or
anonymous clients that present no client certificate:
```no_run
# #[cfg(any(feature = "ring", feature = "aws_lc_rs"))] {
# use rustls::RootCertStore;
# use rustls::server::WebPkiClientVerifier;
# let roots = RootCertStore::empty();
let client_verifier = WebPkiClientVerifier::builder(roots.into())
  .allow_unauthenticated()
  .build()
  .unwrap();
# }
```

If you wish to disable advertising client authentication:
```no_run
# use rustls::RootCertStore;
# use rustls::server::WebPkiClientVerifier;
# let roots = RootCertStore::empty();
let client_verifier = WebPkiClientVerifier::no_client_auth();
```

You can also configure the client verifier to check for certificate revocation with
client certificate revocation lists (CRLs):
```no_run
# #[cfg(any(feature = "ring", feature = "aws_lc_rs"))] {
# use rustls::RootCertStore;
# use rustls::server::{WebPkiClientVerifier};
# let roots = RootCertStore::empty();
# let crls = Vec::new();
let client_verifier = WebPkiClientVerifier::builder(roots.into())
  .with_crls(crls)
  .build()
  .unwrap();
# }
```

[^1]: <https://github.com/rustls/webpki>

**Methods:**

- `fn builder(roots: alloc::sync::Arc<RootCertStore>) -> ClientCertVerifierBuilder` - Create a builder for the `webpki` client certificate verifier configuration using
- `fn builder_with_provider(roots: alloc::sync::Arc<RootCertStore>, provider: alloc::sync::Arc<CryptoProvider>) -> ClientCertVerifierBuilder` - Create a builder for the `webpki` client certificate verifier configuration using
- `fn no_client_auth() -> alloc::sync::Arc<dyn ClientCertVerifier>` - Create a new `WebPkiClientVerifier` that disables client authentication. The server will

**Trait Implementations:**

- **ClientCertVerifier**
  - `fn offer_client_auth(self: &Self) -> bool`
  - `fn client_auth_mandatory(self: &Self) -> bool`
  - `fn root_hint_subjects(self: &Self) -> &[DistinguishedName]`
  - `fn verify_client_cert(self: &Self, end_entity: &CertificateDer, intermediates: &[CertificateDer], now: UnixTime) -> Result<ClientCertVerified, Error>`
  - `fn verify_tls12_signature(self: &Self, message: &[u8], cert: &CertificateDer, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn verify_tls13_signature(self: &Self, message: &[u8], cert: &CertificateDer, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn supported_verify_schemes(self: &Self) -> Vec<SignatureScheme>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



