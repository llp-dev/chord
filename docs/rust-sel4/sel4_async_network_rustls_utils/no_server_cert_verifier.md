**sel4_async_network_rustls_utils > no_server_cert_verifier**

# Module: no_server_cert_verifier

## Contents

**Structs**

- [`NoServerCertVerifier`](#noservercertverifier)

---

## sel4_async_network_rustls_utils::no_server_cert_verifier::NoServerCertVerifier

*Struct*

**Unit Struct**

**Trait Implementations:**

- **ServerCertVerifier**
  - `fn verify_server_cert(self: &Self, _end_entity: &CertificateDer, _intermediates: &[CertificateDer], _server_name: &ServerName, _ocsp_response: &[u8], _now: UnixTime) -> Result<ServerCertVerified, Error>`
  - `fn verify_tls12_signature(self: &Self, _message: &[u8], _cert: &CertificateDer, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn verify_tls13_signature(self: &Self, _message: &[u8], _cert: &CertificateDer, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`
  - `fn supported_verify_schemes(self: &Self) -> Vec<SignatureScheme>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



