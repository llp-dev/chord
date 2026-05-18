**rustls > webpki > server_verifier**

# Module: webpki::server_verifier

## Contents

**Structs**

- [`ServerCertVerifierBuilder`](#servercertverifierbuilder) - A builder for configuring a `webpki` server certificate verifier.
- [`WebPkiServerVerifier`](#webpkiserververifier) - Default `ServerCertVerifier`, see the trait impl for more information.

---

## rustls::webpki::server_verifier::ServerCertVerifierBuilder

*Struct*

A builder for configuring a `webpki` server certificate verifier.

For more information, see the [`WebPkiServerVerifier`] documentation.

**Methods:**

- `fn with_crls<impl IntoIterator<Item = CertificateRevocationListDer<'static>>>(self: Self, crls: impl Trait) -> Self` - Verify the revocation state of presented client certificates against the provided
- `fn only_check_end_entity_revocation(self: Self) -> Self` - Only check the end entity certificate revocation status when using CRLs.
- `fn allow_unknown_revocation_status(self: Self) -> Self` - Allow unknown certificate revocation status when using CRLs.
- `fn enforce_revocation_expiration(self: Self) -> Self` - Enforce the CRL nextUpdate field (i.e. expiration)
- `fn build(self: Self) -> Result<alloc::sync::Arc<WebPkiServerVerifier>, VerifierBuilderError>` - Build a server certificate verifier, allowing control over the root certificates to use as

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ServerCertVerifierBuilder`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::webpki::server_verifier::WebPkiServerVerifier

*Struct*

Default `ServerCertVerifier`, see the trait impl for more information.

**Methods:**

- `fn builder(roots: alloc::sync::Arc<RootCertStore>) -> ServerCertVerifierBuilder` - Create a builder for the `webpki` server certificate verifier configuration using
- `fn builder_with_provider(roots: alloc::sync::Arc<RootCertStore>, provider: alloc::sync::Arc<CryptoProvider>) -> ServerCertVerifierBuilder` - Create a builder for the `webpki` server certificate verifier configuration using

**Trait Implementations:**

- **ServerCertVerifier**
  - `fn verify_server_cert(self: &Self, end_entity: &CertificateDer, intermediates: &[CertificateDer], server_name: &ServerName, ocsp_response: &[u8], now: UnixTime) -> Result<ServerCertVerified, Error>` - Will verify the certificate is valid in the following ways:
  - `fn verify_tls12_signature(self: &Self, message: &[u8], cert: &CertificateDer, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn verify_tls13_signature(self: &Self, message: &[u8], cert: &CertificateDer, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn supported_verify_schemes(self: &Self) -> Vec<SignatureScheme>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



