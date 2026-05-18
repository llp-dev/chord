*[ring](../index.md) / [aead](index.md)*

---

# Module `aead`

Authenticated Encryption with Associated Data (AEAD).

See [Authenticated encryption: relations among notions and analysis of the
generic composition paradigm][AEAD] for an introduction to the concept of
AEADs.



## Contents

- [Modules](#modules)
  - [`aes`](#aes)
  - [`aes_gcm`](#aes-gcm)
  - [`block`](#block)
  - [`chacha`](#chacha)
  - [`chacha20_poly1305`](#chacha20-poly1305)
  - [`chacha20_poly1305_openssh`](#chacha20-poly1305-openssh)
  - [`gcm`](#gcm)
  - [`less_safe_key`](#less-safe-key)
  - [`nonce`](#nonce)
  - [`opening_key`](#opening-key)
  - [`poly1305`](#poly1305)
  - [`quic`](#quic)
  - [`sealing_key`](#sealing-key)
  - [`shift`](#shift)
  - [`unbound_key`](#unbound-key)
- [Structs](#structs)
  - [`LessSafeKey`](#lesssafekey)
  - [`Nonce`](#nonce)
  - [`OpeningKey`](#openingkey)
  - [`SealingKey`](#sealingkey)
  - [`UnboundKey`](#unboundkey)
  - [`Aad`](#aad)
  - [`Algorithm`](#algorithm)
  - [`Tag`](#tag)
- [Enums](#enums)
  - [`KeyInner`](#keyinner)
  - [`AlgorithmID`](#algorithmid)
- [Traits](#traits)
  - [`NonceSequence`](#noncesequence)
  - [`BoundKey`](#boundkey)
- [Functions](#functions)
  - [`max_input_len`](#max-input-len)
- [Constants](#constants)
  - [`NONCE_LEN`](#nonce-len)
  - [`MAX_KEY_LEN`](#max-key-len)
  - [`TAG_LEN`](#tag-len)
  - [`MAX_TAG_LEN`](#max-tag-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`aes`](#aes) | mod |  |
| [`aes_gcm`](#aes-gcm) | mod |  |
| [`block`](#block) | mod |  |
| [`chacha`](#chacha) | mod |  |
| [`chacha20_poly1305`](#chacha20-poly1305) | mod |  |
| [`chacha20_poly1305_openssh`](#chacha20-poly1305-openssh) | mod | The [chacha20-poly1305@openssh.com] AEAD-ish construct. |
| [`gcm`](#gcm) | mod |  |
| [`less_safe_key`](#less-safe-key) | mod |  |
| [`nonce`](#nonce) | mod |  |
| [`opening_key`](#opening-key) | mod | Authenticated Encryption with Associated Data (AEAD). |
| [`poly1305`](#poly1305) | mod |  |
| [`quic`](#quic) | mod | QUIC Header Protection. |
| [`sealing_key`](#sealing-key) | mod | Authenticated Encryption with Associated Data (AEAD). |
| [`shift`](#shift) | mod |  |
| [`unbound_key`](#unbound-key) | mod | Authenticated Encryption with Associated Data (AEAD). |
| [`LessSafeKey`](#lesssafekey) | struct |  |
| [`Nonce`](#nonce) | struct |  |
| [`OpeningKey`](#openingkey) | struct |  |
| [`SealingKey`](#sealingkey) | struct |  |
| [`UnboundKey`](#unboundkey) | struct |  |
| [`Aad`](#aad) | struct | The additionally authenticated data (AAD) for an opening or sealing operation. |
| [`Algorithm`](#algorithm) | struct | An AEAD Algorithm. |
| [`Tag`](#tag) | struct | A possibly valid authentication tag. |
| [`KeyInner`](#keyinner) | enum |  |
| [`AlgorithmID`](#algorithmid) | enum |  |
| [`NonceSequence`](#noncesequence) | trait | A sequences of unique nonces. |
| [`BoundKey`](#boundkey) | trait | An AEAD key bound to a nonce sequence. |
| [`max_input_len`](#max-input-len) | fn |  |
| [`NONCE_LEN`](#nonce-len) | const |  |
| [`MAX_KEY_LEN`](#max-key-len) | const |  |
| [`TAG_LEN`](#tag-len) | const |  |
| [`MAX_TAG_LEN`](#max-tag-len) | const | The maximum length of a tag for the algorithms in this module. |

## Modules

- [`aes`](aes/index.md)
- [`aes_gcm`](aes_gcm/index.md)
- [`block`](block/index.md)
- [`chacha`](chacha/index.md)
- [`chacha20_poly1305`](chacha20_poly1305/index.md)
- [`chacha20_poly1305_openssh`](chacha20_poly1305_openssh/index.md) — The [chacha20-poly1305@openssh.com] AEAD-ish construct.
- [`gcm`](gcm/index.md)
- [`less_safe_key`](less_safe_key/index.md)
- [`nonce`](nonce/index.md)
- [`opening_key`](opening_key/index.md) — Authenticated Encryption with Associated Data (AEAD).
- [`poly1305`](poly1305/index.md)
- [`quic`](quic/index.md) — QUIC Header Protection.
- [`sealing_key`](sealing_key/index.md) — Authenticated Encryption with Associated Data (AEAD).
- [`shift`](shift/index.md)
- [`unbound_key`](unbound_key/index.md) — Authenticated Encryption with Associated Data (AEAD).

## Structs

### `LessSafeKey`

```rust
struct LessSafeKey {
    inner: super::KeyInner,
    algorithm: &'static super::Algorithm,
}
```

Immutable keys for use in situations where `OpeningKey`/`SealingKey` and
`NonceSequence` cannot reasonably be used.

Prefer to use `OpeningKey`/`SealingKey` and `NonceSequence` when practical.

#### Implementations

- <span id="lesssafekey-new"></span>`fn new(key: UnboundKey) -> Self` — [`UnboundKey`](unbound_key/index.md#unboundkey)

  Constructs a `LessSafeKey`.

- <span id="lesssafekey-new"></span>`fn new_(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>` — [`Algorithm`](#algorithm), [`Unspecified`](../error/index.md#unspecified)

- <span id="lesssafekey-open-in-place-separate-tag"></span>`fn open_in_place_separate_tag<'in_out, A>(&self, nonce: Nonce, aad: Aad<A>, tag: Tag, in_out: &'in_out mut [u8], ciphertext: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Nonce`](nonce/index.md#nonce), [`Aad`](#aad), [`Tag`](#tag), [`Unspecified`](../error/index.md#unspecified)

  Like [open_in_place](Self::open_in_place), except the authentication tag is

  passed separately.

- <span id="lesssafekey-open-in-place"></span>`fn open_in_place<'in_out, A>(&self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out mut [u8]) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Nonce`](nonce/index.md#nonce), [`Aad`](#aad), [`Unspecified`](../error/index.md#unspecified)

  Like `super::OpeningKey::open_in_place()`, except it accepts an

  arbitrary nonce.

  

  `nonce` must be unique for every use of the key to open data.

- <span id="lesssafekey-open-within"></span>`fn open_within<'in_out, A>(&self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Nonce`](nonce/index.md#nonce), [`Aad`](#aad), [`Unspecified`](../error/index.md#unspecified)

  Like `super::OpeningKey::open_within()`, except it accepts an

  arbitrary nonce.

  

  `nonce` must be unique for every use of the key to open data.

- <span id="lesssafekey-seal-in-place-append-tag"></span>`fn seal_in_place_append_tag<A, InOut>(&self, nonce: Nonce, aad: Aad<A>, in_out: &mut InOut) -> Result<(), error::Unspecified>` — [`Nonce`](nonce/index.md#nonce), [`Aad`](#aad), [`Unspecified`](../error/index.md#unspecified)

  Like `super::SealingKey::seal_in_place_append_tag()`, except it

  accepts an arbitrary nonce.

  

  `nonce` must be unique for every use of the key to seal data.

- <span id="lesssafekey-seal-in-place-separate-tag"></span>`fn seal_in_place_separate_tag<A>(&self, nonce: Nonce, aad: Aad<A>, in_out: &mut [u8]) -> Result<Tag, error::Unspecified>` — [`Nonce`](nonce/index.md#nonce), [`Aad`](#aad), [`Tag`](#tag), [`Unspecified`](../error/index.md#unspecified)

  Like `super::SealingKey::seal_in_place_separate_tag()`, except it

  accepts an arbitrary nonce.

  

  `nonce` must be unique for every use of the key to seal data.

- <span id="lesssafekey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The key's AEAD algorithm.

- <span id="lesssafekey-fmt-debug"></span>`fn fmt_debug(&self, type_name: &'static str, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

#### Trait Implementations

##### `impl Clone for LessSafeKey`

- <span id="lesssafekey-clone"></span>`fn clone(&self) -> LessSafeKey` — [`LessSafeKey`](less_safe_key/index.md#lesssafekey)

##### `impl Debug for LessSafeKey`

- <span id="lesssafekey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `Nonce`

```rust
struct Nonce([u8; 12]);
```

A nonce for a single AEAD opening or sealing operation.

The user must ensure, for a particular key, that each nonce is unique.

`Nonce` intentionally doesn't implement `Clone` to ensure that each one is
consumed at most once.

#### Implementations

- <span id="nonce-try-assume-unique-for-key"></span>`fn try_assume_unique_for_key(value: &[u8]) -> Result<Self, error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

  Constructs a `Nonce` with the given value, assuming that the value is

  unique for the lifetime of the key it is being used with.

  

  Fails if `value` isn't `NONCE_LEN` bytes long.

- <span id="nonce-assume-unique-for-key"></span>`fn assume_unique_for_key(value: [u8; 12]) -> Self`

  Constructs a `Nonce` with the given value, assuming that the value is

  unique for the lifetime of the key it is being used with.

#### Trait Implementations

##### `impl AsRef for Nonce`

- <span id="nonce-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 12]`

### `OpeningKey<N: NonceSequence>`

```rust
struct OpeningKey<N: NonceSequence> {
    key: super::LessSafeKey,
    nonce_sequence: N,
}
```

An AEAD key for authenticating and decrypting ("opening"), bound to a nonce
sequence.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the nonce sequence.

#### Implementations

- <span id="openingkey-open-in-place"></span>`fn open_in_place<'in_out, A>(&mut self, aad: Aad<A>, in_out: &'in_out mut [u8]) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Aad`](#aad), [`Unspecified`](../error/index.md#unspecified)

  Authenticates and decrypts (“opens”) data in place.

  

  `aad` is the additional authenticated data (AAD), if any.

  

  On input, `in_out` must be the ciphertext followed by the tag. When

  `open_in_place()` returns `Ok(plaintext)`, the input ciphertext

  has been overwritten by the plaintext; `plaintext` will refer to the

  plaintext without the tag.

  

  When `open_in_place()` returns `Err(..)`, `in_out` may have been

  overwritten in an unspecified way.

- <span id="openingkey-open-within"></span>`fn open_within<'in_out, A>(&mut self, aad: Aad<A>, in_out: &'in_out mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Aad`](#aad), [`Unspecified`](../error/index.md#unspecified)

  Authenticates and decrypts (“opens”) data in place, with a shift.

  

  `aad` is the additional authenticated data (AAD), if any.

  

  On input, `in_out[ciphertext_and_tag]` must be the ciphertext followed

  by the tag. When `open_within()` returns `Ok(plaintext)`, the plaintext

  will be at `in_out[0..plaintext.len()]`. In other words, the following

  two code fragments are equivalent for valid values of

  `ciphertext_and_tag`, except `open_within` will often be more efficient:

  

  

  ```skip

  let plaintext = key.open_within(aad, in_out, cipertext_and_tag)?;

  ```

  

  ```skip

  let ciphertext_and_tag_len = in_out[ciphertext_and_tag].len();

  in_out.copy_within(ciphertext_and_tag, 0);

  let plaintext = key.open_in_place(aad, &mut in_out[..ciphertext_and_tag_len])?;

  ```

  

  Similarly, `key.open_within(aad, in_out, 0..)` is equivalent to

  `key.open_in_place(aad, in_out)`.

  

   When `open_in_place()` returns `Err(..)`, `in_out` may have been

  overwritten in an unspecified way.

  

  The shifting feature is useful in the case where multiple packets are

  being reassembled in place. Consider this example where the peer has

  sent the message “Split stream reassembled in place” split into

  three sealed packets:

  

  ```ascii-art

                  Packet 1                  Packet 2                 Packet 3

  Input:  [Header][Ciphertext][Tag][Header][Ciphertext][Tag][Header][Ciphertext][Tag]

                       |         +--------------+                        |

                +------+   +-----+    +----------------------------------+

                v          v          v

  Output: [Plaintext][Plaintext][Plaintext]

         “Split stream reassembled in place”

  ```

  

  This reassembly can be accomplished with three calls to `open_within()`.

#### Trait Implementations

##### `impl<N: NonceSequence> BoundKey for OpeningKey<N>`

- <span id="openingkey-boundkey-new"></span>`fn new(key: UnboundKey, nonce_sequence: N) -> Self` — [`UnboundKey`](unbound_key/index.md#unboundkey)

- <span id="openingkey-boundkey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

##### `impl<N: NonceSequence> Debug for OpeningKey<N>`

- <span id="openingkey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `SealingKey<N: NonceSequence>`

```rust
struct SealingKey<N: NonceSequence> {
    key: super::LessSafeKey,
    nonce_sequence: N,
}
```

An AEAD key for encrypting and signing ("sealing"), bound to a nonce
sequence.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the nonce sequence.

#### Implementations

- <span id="sealingkey-seal-in-place-append-tag"></span>`fn seal_in_place_append_tag<A, InOut>(&mut self, aad: Aad<A>, in_out: &mut InOut) -> Result<(), error::Unspecified>` — [`Aad`](#aad), [`Unspecified`](../error/index.md#unspecified)

  Encrypts and signs (“seals”) data in place, appending the tag to the

  resulting ciphertext.

  

  `key.seal_in_place_append_tag(aad, in_out)` is equivalent to:

  

  ```skip

  key.seal_in_place_separate_tag(aad, in_out.as_mut())

      .map(|tag| in_out.extend(tag.as_ref()))

  ```

- <span id="sealingkey-seal-in-place-separate-tag"></span>`fn seal_in_place_separate_tag<A>(&mut self, aad: Aad<A>, in_out: &mut [u8]) -> Result<Tag, error::Unspecified>` — [`Aad`](#aad), [`Tag`](#tag), [`Unspecified`](../error/index.md#unspecified)

  Encrypts and signs (“seals”) data in place.

  

  `aad` is the additional authenticated data (AAD), if any. This is

  authenticated but not encrypted. The type `A` could be a byte slice

  `&[u8]`, a byte array `[u8; N]` for some constant `N`, `Vec<u8>`, etc.

  If there is no AAD then use `Aad::empty()`.

  

  The plaintext is given as the input value of `in_out`. `seal_in_place()`

  will overwrite the plaintext with the ciphertext and return the tag.

  For most protocols, the caller must append the tag to the ciphertext.

  The tag will be `self.algorithm.tag_len()` bytes long.

#### Trait Implementations

##### `impl<N: NonceSequence> BoundKey for SealingKey<N>`

- <span id="sealingkey-boundkey-new"></span>`fn new(key: UnboundKey, nonce_sequence: N) -> Self` — [`UnboundKey`](unbound_key/index.md#unboundkey)

- <span id="sealingkey-boundkey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

##### `impl<N: NonceSequence> Debug for SealingKey<N>`

- <span id="sealingkey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `UnboundKey`

```rust
struct UnboundKey {
    inner: super::LessSafeKey,
}
```

An AEAD key without a designated role or nonce sequence.

#### Implementations

- <span id="unboundkey-new"></span>`fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>` — [`Algorithm`](#algorithm), [`Unspecified`](../error/index.md#unspecified)

  Constructs a `UnboundKey`.

  

  Fails if `key_bytes.len() != algorithm.key_len()`.

- <span id="unboundkey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The key's AEAD algorithm.

- <span id="unboundkey-into-inner"></span>`fn into_inner(self) -> LessSafeKey` — [`LessSafeKey`](less_safe_key/index.md#lesssafekey)

#### Trait Implementations

##### `impl Debug for UnboundKey`

- <span id="unboundkey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `Aad<A>`

```rust
struct Aad<A>(A);
```

The additionally authenticated data (AAD) for an opening or sealing
operation. This data is authenticated but is **not** encrypted.

The type `A` could be a byte slice `&[u8]`, a byte array `[u8; N]`
for some constant `N`, `Vec<u8>`, etc.

#### Implementations

- <span id="aad-from"></span>`fn from(aad: A) -> Self`

  Construct the `Aad` from the given bytes.

#### Trait Implementations

##### `impl<A> AsRef for Aad<A>`

- <span id="aad-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl<A: clone::Clone> Clone for Aad<A>`

- <span id="aad-clone"></span>`fn clone(&self) -> Aad<A>` — [`Aad`](#aad)

##### `impl<A: marker::Copy> Copy for Aad<A>`

##### `impl<A> Debug for Aad<A>`

- <span id="aad-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<A> Eq for Aad<A>`

##### `impl<A> PartialEq for Aad<A>`

- <span id="aad-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `Algorithm`

```rust
struct Algorithm {
    init: fn(&[u8], cpu::Features) -> Result<KeyInner, error::Unspecified>,
    seal: fn(&KeyInner, Nonce, Aad<&[u8]>, &mut [u8], cpu::Features) -> Result<Tag, error::Unspecified>,
    open: fn(&KeyInner, Nonce, Aad<&[u8]>, &mut [u8], core::ops::RangeFrom<usize>, cpu::Features) -> Result<Tag, error::Unspecified>,
    key_len: usize,
    id: AlgorithmID,
}
```

An AEAD Algorithm.

#### Implementations

- <span id="algorithm-key-len"></span>`fn key_len(&self) -> usize`

  The length of the key.

- <span id="algorithm-tag-len"></span>`fn tag_len(&self) -> usize`

  The length of a tag.

  

  See also `MAX_TAG_LEN`.

- <span id="algorithm-nonce-len"></span>`fn nonce_len(&self) -> usize`

  The length of the nonces.

#### Trait Implementations

##### `impl Debug for Algorithm`

- <span id="algorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Eq for Algorithm`

##### `impl KeyType for &'static Algorithm`

- <span id="static-algorithm-keytype-len"></span>`fn len(&self) -> usize`

##### `impl PartialEq for Algorithm`

- <span id="algorithm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `Tag`

```rust
struct Tag([u8; 16]);
```

A possibly valid authentication tag.

#### Trait Implementations

##### `impl AsRef for Tag`

- <span id="tag-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for Tag`

- <span id="tag-clone"></span>`fn clone(&self) -> Tag` — [`Tag`](#tag)

##### `impl Copy for Tag`

## Enums

### `KeyInner`

```rust
enum KeyInner {
    AesGcm(aes_gcm::Key),
    ChaCha20Poly1305(chacha::Key),
}
```

#### Trait Implementations

##### `impl Clone for KeyInner`

- <span id="keyinner-clone"></span>`fn clone(&self) -> KeyInner` — [`KeyInner`](#keyinner)

### `AlgorithmID`

```rust
enum AlgorithmID {
    AES_128_GCM,
    AES_256_GCM,
    CHACHA20_POLY1305,
}
```

#### Trait Implementations

##### `impl Debug for AlgorithmID`

- <span id="algorithmid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AlgorithmID`

##### `impl PartialEq for AlgorithmID`

- <span id="algorithmid-partialeq-eq"></span>`fn eq(&self, other: &AlgorithmID) -> bool` — [`AlgorithmID`](#algorithmid)

##### `impl StructuralPartialEq for AlgorithmID`

## Traits

### `NonceSequence`

```rust
trait NonceSequence { ... }
```

A sequences of unique nonces.

A given `NonceSequence` must never return the same `Nonce` twice from
`advance()`.

A simple counter is a reasonable (but probably not ideal) `NonceSequence`.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the sequence.

#### Required Methods

- `fn advance(&mut self) -> Result<Nonce, error::Unspecified>`

  Returns the next nonce in the sequence.

### `BoundKey<N: NonceSequence>`

```rust
trait BoundKey<N: NonceSequence>: core::fmt::Debug { ... }
```

An AEAD key bound to a nonce sequence.

#### Required Methods

- `fn new(key: UnboundKey, nonce_sequence: N) -> Self`

  Constructs a new key from the given `UnboundKey` and `NonceSequence`.

- `fn algorithm(&self) -> &'static Algorithm`

  The key's AEAD algorithm.

#### Implementors

- [`OpeningKey`](opening_key/index.md#openingkey)
- [`SealingKey`](sealing_key/index.md#sealingkey)

## Functions

### `max_input_len`

```rust
const fn max_input_len(block_len: usize, overhead_blocks_per_nonce: usize) -> usize
```

## Constants

### `NONCE_LEN`
```rust
const NONCE_LEN: usize = 12usize;
```

All the AEADs we support use 96-bit nonces.

### `MAX_KEY_LEN`
```rust
const MAX_KEY_LEN: usize = 32usize;
```

### `TAG_LEN`
```rust
const TAG_LEN: usize = 16usize;
```

### `MAX_TAG_LEN`
```rust
const MAX_TAG_LEN: usize = 16usize;
```

The maximum length of a tag for the algorithms in this module.

