**rustls > server > handy**

# Module: server::handy

## Contents

**Structs**

- [`AlwaysResolvesServerRawPublicKeys`](#alwaysresolvesserverrawpublickeys) - An exemplar `ResolvesServerCert` implementation that always resolves to a single
- [`NoServerSessionStorage`](#noserversessionstorage) - Something which never stores sessions.

---

## rustls::server::handy::AlwaysResolvesServerRawPublicKeys

*Struct*

An exemplar `ResolvesServerCert` implementation that always resolves to a single
[RFC 7250] raw public key.

[RFC 7250]: https://tools.ietf.org/html/rfc7250

**Tuple Struct**: `()`

**Methods:**

- `fn new(certified_key: alloc::sync::Arc<sign::CertifiedKey>) -> Self` - Create a new `AlwaysResolvesServerRawPublicKeys` instance.

**Trait Implementations:**

- **ResolvesServerCert**
  - `fn resolve(self: &Self, _client_hello: ClientHello) -> Option<alloc::sync::Arc<sign::CertifiedKey>>`
  - `fn only_raw_public_keys(self: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AlwaysResolvesServerRawPublicKeys`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::server::handy::NoServerSessionStorage

*Struct*

Something which never stores sessions.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **StoresServerSessions**
  - `fn put(self: &Self, _id: Vec<u8>, _sec: Vec<u8>) -> bool`
  - `fn get(self: &Self, _id: &[u8]) -> Option<Vec<u8>>`
  - `fn take(self: &Self, _id: &[u8]) -> Option<Vec<u8>>`
  - `fn can_cache(self: &Self) -> bool`



