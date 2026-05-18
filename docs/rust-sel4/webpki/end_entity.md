**webpki > end_entity**

# Module: end_entity

## Contents

**Structs**

- [`EndEntityCert`](#endentitycert) - An end-entity certificate.

---

## webpki::end_entity::EndEntityCert

*Struct*

An end-entity certificate.

Server certificate processing in a TLS connection consists of several
steps. All of these steps are necessary:

* [`EndEntityCert::verify_for_usage()`]: Verify that the peer's certificate
  is valid for the current usage scenario. For server authentication, use
  [`crate::KeyUsage::server_auth()`].
* [`EndEntityCert::verify_is_valid_for_subject_name()`]: Verify that the server's
  certificate is valid for the host or IP address that is being connected to.
* [`EndEntityCert::verify_signature()`]: Verify that the signature of server's
  `ServerKeyExchange` message is valid for the server's certificate.

Client certificate processing in a TLS connection consists of analogous
steps. All of these steps are necessary:

* [`EndEntityCert::verify_for_usage()`]: Verify that the peer's certificate
  is valid for the current usage scenario. For client authentication, use
  [`crate::KeyUsage::client_auth()`].
* [`EndEntityCert::verify_signature()`]: Verify that the signature of client's
  `CertificateVerify` message is valid using the public key from the
  client's certificate.

Although it would be less error-prone to combine all these steps into a
single function call, some significant optimizations are possible if the
three steps are processed separately (in parallel). It does not matter much
which order the steps are done in, but **all of these steps must completed
before application data is sent and before received application data is
processed**. The [`TryFrom`] conversion from `&CertificateDer<'_>` is an
inexpensive operation and is deterministic, so if these tasks are done in
multiple threads, it is probably best to just create multiple [`EndEntityCert`]
instances for the same DER-encoded ASN.1 certificate bytes.

**Generic Parameters:**
- 'a

**Methods:**

- `fn verify_for_usage<'p, impl ExtendedKeyUsageValidator>(self: &'p Self, supported_sig_algs: &[&dyn SignatureVerificationAlgorithm], trust_anchors: &'p [TrustAnchor], intermediate_certs: &'p [CertificateDer<'p>], time: UnixTime, usage: impl Trait, revocation: Option<RevocationOptions>, verify_path: Option<&dyn Fn>) -> Result<VerifiedPath<'p>, Error>` - Verifies that the end-entity certificate is valid for use against the
- `fn verify_is_valid_for_subject_name(self: &Self, server_name: &ServerName) -> Result<(), Error>` - Verifies that the certificate is valid for the given Subject Name.
- `fn verify_signature(self: &Self, signature_alg: &dyn SignatureVerificationAlgorithm, msg: &[u8], signature: &[u8]) -> Result<(), Error>` - Verifies the signature `signature` of message `msg` using the

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(cert: &'a CertificateDer<'a>) -> Result<Self, <Self as >::Error>` - Parse the ASN.1 DER-encoded X.509 encoding of the certificate
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



