**ring > aead**

# Module: aead

## Contents

**Modules**

- [`chacha20_poly1305_openssh`](#chacha20_poly1305_openssh) - The [chacha20-poly1305@openssh.com] AEAD-ish construct.
- [`quic`](#quic) - QUIC Header Protection.

**Structs**

- [`Aad`](#aad) - The additionally authenticated data (AAD) for an opening or sealing
- [`Algorithm`](#algorithm) - An AEAD Algorithm.
- [`Tag`](#tag) - A possibly valid authentication tag.

**Traits**

- [`BoundKey`](#boundkey) - An AEAD key bound to a nonce sequence.
- [`NonceSequence`](#noncesequence) - A sequences of unique nonces.

**Constants**

- [`MAX_TAG_LEN`](#max_tag_len) - The maximum length of a tag for the algorithms in this module.

---

## ring::aead::Aad

*Struct*

The additionally authenticated data (AAD) for an opening or sealing
operation. This data is authenticated but is **not** encrypted.

The type `A` could be a byte slice `&[u8]`, a byte array `[u8; N]`
for some constant `N`, `Vec<u8>`, etc.

**Generic Parameters:**
- A

**Tuple Struct**: `()`

**Methods:**

- `fn from(aad: A) -> Self` - Construct the `Aad` from the given bytes.
- `fn empty() -> Self` - Construct an empty `Aad`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Aad<A>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ring::aead::Algorithm

*Struct*

An AEAD Algorithm.

**Methods:**

- `fn key_len(self: &Self) -> usize` - The length of the key.
- `fn tag_len(self: &Self) -> usize` - The length of a tag.
- `fn nonce_len(self: &Self) -> usize` - The length of the nonces.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ring::aead::BoundKey

*Trait*

An AEAD key bound to a nonce sequence.

**Methods:**

- `new`: Constructs a new key from the given `UnboundKey` and `NonceSequence`.
- `algorithm`: The key's AEAD algorithm.



## ring::aead::MAX_TAG_LEN

*Constant*: `usize`

The maximum length of a tag for the algorithms in this module.



## ring::aead::NonceSequence

*Trait*

A sequences of unique nonces.

A given `NonceSequence` must never return the same `Nonce` twice from
`advance()`.

A simple counter is a reasonable (but probably not ideal) `NonceSequence`.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the sequence.

**Methods:**

- `advance`: Returns the next nonce in the sequence.



## ring::aead::Tag

*Struct*

A possibly valid authentication tag.

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **From**
  - `fn from(value: [u8; 16]) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Tag`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **TryFrom**
  - `fn try_from(value: &[u8]) -> Result<Self, <Self as >::Error>`



## Module: chacha20_poly1305_openssh

The [chacha20-poly1305@openssh.com] AEAD-ish construct.

This should only be used by SSH implementations. It has a similar, but
different API from `ring::aead` because the construct cannot use the same
API as `ring::aead` due to the way the construct handles the encrypted
packet length.

The concatenation of a and b is denoted `a||b`. `K_1` and `K_2` are defined
in the [chacha20-poly1305@openssh.com] specification. `packet_length`,
`padding_length`, `payload`, and `random padding` are defined in
[RFC 4253]. The term `plaintext` is used as a shorthand for
`padding_length||payload||random padding`.

[chacha20-poly1305@openssh.com]:
   http://cvsweb.openbsd.org/cgi-bin/cvsweb/src/usr.bin/ssh/PROTOCOL.chacha20poly1305?annotate=HEAD
[RFC 4253]: https://tools.ietf.org/html/rfc4253



## Module: quic

QUIC Header Protection.

See draft-ietf-quic-tls.



