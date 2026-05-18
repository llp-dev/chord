**webpki > cert**

# Module: cert

## Contents

**Structs**

- [`Cert`](#cert) - A parsed X509 certificate.

---

## webpki::cert::Cert

*Struct*

A parsed X509 certificate.

**Generic Parameters:**
- 'a

**Methods:**

- `fn valid_dns_names(self: &Self) -> impl Trait` - Returns a list of valid DNS names provided in the subject alternative names extension
- `fn valid_uri_names(self: &Self) -> impl Trait` - Returns a list of valid URI names provided in the subject alternative names extension
- `fn serial(self: &Self) -> &[u8]` - Raw certificate serial number.
- `fn issuer(self: &Self) -> &[u8]` - Raw DER-encoded certificate issuer.
- `fn subject(self: &Self) -> &[u8]` - Raw DER encoded certificate subject.
- `fn subject_public_key_info(self: &Self) -> SubjectPublicKeyInfoDer<'static>` - Get the RFC 5280-compliant [`SubjectPublicKeyInfoDer`] (SPKI) of this [`Cert`].
- `fn der(self: &Self) -> CertificateDer<'a>` - Raw DER-encoded representation of the certificate.



