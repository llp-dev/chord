**webpki > trust_anchor**

# Module: trust_anchor

## Contents

**Functions**

- [`anchor_from_trusted_cert`](#anchor_from_trusted_cert) - Interprets the given pre-validated DER-encoded certificate as a `TrustAnchor`.

---

## webpki::trust_anchor::anchor_from_trusted_cert

*Function*

Interprets the given pre-validated DER-encoded certificate as a `TrustAnchor`.

This function extracts the components of a trust anchor (see [RFC 5280 6.1.1]) from
an X.509 certificate obtained from a source trusted to have appropriately validated
the subject name, public key, and name constraints in the certificate, for example your
operating system's trust store.

No additional checks on the content of the certificate, including whether it is self-signed,
or has a basic constraints extension indicating the `cA` boolean is true, will be performed.
[RFC 5280 6.2] notes:
> Implementations that use self-signed certificates to specify trust
> anchor information are free to process or ignore such information.

This function is intended for users constructing `TrustAnchor`'s from existing trust stores
that express trust anchors as X.509 certificates. It should **not** be used to treat an
end-entity certificate as a `TrustAnchor` in an effort to validate the same end-entity
certificate during path building. Webpki has no support for self-signed certificates.

[RFC 5280 6.1.1]: <https://datatracker.ietf.org/doc/html/rfc5280#section-6.1.1>
[RFC 5280 6.2]: <https://www.rfc-editor.org/rfc/rfc5280#section-6.2>

```rust
fn anchor_from_trusted_cert<'a>(cert: &'a pki_types::CertificateDer<'a>) -> Result<pki_types::TrustAnchor<'a>, crate::error::Error>
```



