**rustls > webpki > anchors**

# Module: webpki::anchors

## Contents

**Structs**

- [`RootCertStore`](#rootcertstore) - A container for root certificates able to provide a root-of-trust

---

## rustls::webpki::anchors::RootCertStore

*Struct*

A container for root certificates able to provide a root-of-trust
for connection authentication.

**Fields:**
- `roots: alloc::vec::Vec<pki_types::TrustAnchor<'static>>` - The list of roots.

**Methods:**

- `fn empty() -> Self` - Make a new, empty `RootCertStore`.
- `fn add_parsable_certificates<'a, impl IntoIterator<Item = CertificateDer<'a>>>(self: & mut Self, der_certs: impl Trait) -> (usize, usize)` - Parse the given DER-encoded certificates and add all that can be parsed
- `fn add(self: & mut Self, der: CertificateDer) -> Result<(), Error>` - Add a single DER-encoded certificate to the store.
- `fn subjects(self: &Self) -> Vec<DistinguishedName>` - Return the DER encoded [`DistinguishedName`] of each trust anchor subject in the root
- `fn is_empty(self: &Self) -> bool` - Return true if there are no certificates.
- `fn len(self: &Self) -> usize` - Say how many certificates are in the container.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RootCertStore`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



