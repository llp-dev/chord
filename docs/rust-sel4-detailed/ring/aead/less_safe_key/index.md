*[ring](../../index.md) / [aead](../index.md) / [less_safe_key](index.md)*

---

# Module `less_safe_key`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LessSafeKey`](#lesssafekey) | struct | Immutable keys for use in situations where `OpeningKey`/`SealingKey` and `NonceSequence` cannot reasonably be used. |
| [`open_within_`](#open-within) | fn |  |
| [`seal_in_place_separate_tag_`](#seal-in-place-separate-tag) | fn |  |

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

- <span id="lesssafekey-new"></span>`fn new(key: UnboundKey) -> Self` — [`UnboundKey`](../unbound_key/index.md#unboundkey)

  Constructs a `LessSafeKey`.

- <span id="lesssafekey-new"></span>`fn new_(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>` — [`Algorithm`](../index.md#algorithm), [`Unspecified`](../../error/index.md#unspecified)

- <span id="lesssafekey-open-in-place-separate-tag"></span>`fn open_in_place_separate_tag<'in_out, A>(&self, nonce: Nonce, aad: Aad<A>, tag: Tag, in_out: &'in_out mut [u8], ciphertext: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Nonce`](../nonce/index.md#nonce), [`Aad`](../index.md#aad), [`Tag`](../index.md#tag), [`Unspecified`](../../error/index.md#unspecified)

  Like [open_in_place](Self::open_in_place), except the authentication tag is

  passed separately.

- <span id="lesssafekey-open-in-place"></span>`fn open_in_place<'in_out, A>(&self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out mut [u8]) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Nonce`](../nonce/index.md#nonce), [`Aad`](../index.md#aad), [`Unspecified`](../../error/index.md#unspecified)

  Like `super::OpeningKey::open_in_place()`, except it accepts an

  arbitrary nonce.

  

  `nonce` must be unique for every use of the key to open data.

- <span id="lesssafekey-open-within"></span>`fn open_within<'in_out, A>(&self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Nonce`](../nonce/index.md#nonce), [`Aad`](../index.md#aad), [`Unspecified`](../../error/index.md#unspecified)

  Like `super::OpeningKey::open_within()`, except it accepts an

  arbitrary nonce.

  

  `nonce` must be unique for every use of the key to open data.

- <span id="lesssafekey-seal-in-place-append-tag"></span>`fn seal_in_place_append_tag<A, InOut>(&self, nonce: Nonce, aad: Aad<A>, in_out: &mut InOut) -> Result<(), error::Unspecified>` — [`Nonce`](../nonce/index.md#nonce), [`Aad`](../index.md#aad), [`Unspecified`](../../error/index.md#unspecified)

  Like `super::SealingKey::seal_in_place_append_tag()`, except it

  accepts an arbitrary nonce.

  

  `nonce` must be unique for every use of the key to seal data.

- <span id="lesssafekey-seal-in-place-separate-tag"></span>`fn seal_in_place_separate_tag<A>(&self, nonce: Nonce, aad: Aad<A>, in_out: &mut [u8]) -> Result<Tag, error::Unspecified>` — [`Nonce`](../nonce/index.md#nonce), [`Aad`](../index.md#aad), [`Tag`](../index.md#tag), [`Unspecified`](../../error/index.md#unspecified)

  Like `super::SealingKey::seal_in_place_separate_tag()`, except it

  accepts an arbitrary nonce.

  

  `nonce` must be unique for every use of the key to seal data.

- <span id="lesssafekey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](../index.md#algorithm)

  The key's AEAD algorithm.

- <span id="lesssafekey-fmt-debug"></span>`fn fmt_debug(&self, type_name: &'static str, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

#### Trait Implementations

##### `impl Clone for LessSafeKey`

- <span id="lesssafekey-clone"></span>`fn clone(&self) -> LessSafeKey` — [`LessSafeKey`](#lesssafekey)

##### `impl Debug for LessSafeKey`

- <span id="lesssafekey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

## Functions

### `open_within_`

```rust
fn open_within_<'in_out>(key: &LessSafeKey, nonce: super::Nonce, aad: super::Aad<&[u8]>, received_tag: super::Tag, in_out: &'in_out mut [u8], src: core::ops::RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>
```

### `seal_in_place_separate_tag_`

```rust
fn seal_in_place_separate_tag_(key: &LessSafeKey, nonce: super::Nonce, aad: super::Aad<&[u8]>, in_out: &mut [u8]) -> Result<super::Tag, error::Unspecified>
```

