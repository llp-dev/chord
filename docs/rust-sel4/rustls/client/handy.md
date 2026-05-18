**rustls > client > handy**

# Module: client::handy

## Contents

**Structs**

- [`AlwaysResolvesClientRawPublicKeys`](#alwaysresolvesclientrawpublickeys) - An exemplar `ResolvesClientCert` implementation that always resolves to a single

---

## rustls::client::handy::AlwaysResolvesClientRawPublicKeys

*Struct*

An exemplar `ResolvesClientCert` implementation that always resolves to a single
[RFC 7250] raw public key.

[RFC 7250]: https://tools.ietf.org/html/rfc7250

**Tuple Struct**: `()`

**Methods:**

- `fn new(certified_key: alloc::sync::Arc<sign::CertifiedKey>) -> Self` - Create a new `AlwaysResolvesClientRawPublicKeys` instance.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AlwaysResolvesClientRawPublicKeys`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ResolvesClientCert**
  - `fn resolve(self: &Self, _root_hint_subjects: &[&[u8]], _sigschemes: &[SignatureScheme]) -> Option<alloc::sync::Arc<sign::CertifiedKey>>`
  - `fn only_raw_public_keys(self: &Self) -> bool`
  - `fn has_certs(self: &Self) -> bool` - Returns true if the resolver is ready to present an identity.



