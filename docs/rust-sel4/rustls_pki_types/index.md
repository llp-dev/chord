# rustls_pki_types

This crate provides types for representing X.509 certificates, keys and other types as
commonly used in the rustls ecosystem. It is intended to be used by crates that need to work
with such X.509 types, such as [rustls](https://crates.io/crates/rustls),
[rustls-webpki](https://crates.io/crates/rustls-webpki),
[rustls-pemfile](https://crates.io/crates/rustls-pemfile), and others.

Some of these crates used to define their own trivial wrappers around DER-encoded bytes.
However, in order to avoid inconvenient dependency edges, these were all disconnected. By
using a common low-level crate of types with long-term stable API, we hope to avoid the
downsides of unnecessary dependency edges while providing good interoperability between crates.

## DER and PEM

Many of the types defined in this crate represent DER-encoded data. DER is a binary encoding of
the ASN.1 format commonly used in web PKI specifications. It is a binary encoding, so it is
relatively compact when stored in memory. However, as a binary format, it is not very easy to
work with for humans and in contexts where binary data is inconvenient. For this reason,
many tools and protocols use a ASCII-based encoding of DER, called PEM. In addition to the
base64-encoded DER, PEM objects are delimited by header and footer lines which indicate the type
of object contained in the PEM blob.

Types here can be created from:

- DER using (for example) [`PrivatePkcs8KeyDer::from()`].
- PEM using (for example) [`pem::PemObject::from_pem_slice()`].

The [`pem::PemObject`] trait contains the full selection of ways to construct
these types from PEM encodings.  That includes ways to open and read from a file,
from a slice, or from an `std::io` stream.

There is also a lower-level API that allows a given PEM file to be fully consumed
in one pass, even if it contains different data types: see the implementation of
the [`pem::PemObject`] trait on the `(pem::SectionKind, Vec<u8>)` tuple.

## Creating new certificates and keys

This crate does not provide any functionality for creating new certificates or keys. However,
the [rcgen](https://docs.rs/rcgen) crate can be used to create new certificates and keys.

## Cloning private keys

This crate intentionally **does not** implement `Clone` on private key types in
order to minimize the exposure of private key data in memory.

If you want to extend the lifetime of a `PrivateKeyDer<'_>`, consider [`PrivateKeyDer::clone_key()`].
Alternatively  since these types are immutable, consider wrapping the `PrivateKeyDer<'_>` in a [`Rc`]
or an [`Arc`].

[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`PrivateKeyDer::clone_key()`]: https://docs.rs/rustls-pki-types/latest/rustls_pki_types/enum.PrivateKeyDer.html#method.clone_key

## Target `wasm32-unknown-unknown` with the `web` feature

[`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
is unavailable in `wasm32-unknown-unknown` targets, so calls to
[`UnixTime::now()`](https://docs.rs/rustls-pki-types/latest/rustls_pki_types/struct.UnixTime.html#method.now),
otherwise enabled by the [`std`](https://docs.rs/crate/rustls-pki-types/latest/features#std) feature,
require building instead with the [`web`](https://docs.rs/crate/rustls-pki-types/latest/features#web)
feature. It gets time by calling [`Date.now()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
in the browser.

## Modules

### [`rustls_pki_types`](rustls_pki_types.md)

*1 trait, 1 type alias, 12 structs, 2 enums, 2 modules*

### [`alg_id`](alg_id.md)

*1 struct, 19 constants*

### [`pem`](pem.md)

*1 struct, 1 trait, 2 enums*

### [`server_name`](server_name.md)

*2 enums, 5 structs*

