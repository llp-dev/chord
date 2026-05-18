*[ring](../../index.md) / [aead](../index.md) / [sealing_key](index.md)*

---

# Module `sealing_key`

Authenticated Encryption with Associated Data (AEAD).

See [Authenticated encryption: relations among notions and analysis of the
generic composition paradigm][AEAD] for an introduction to the concept of
AEADs.



## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SealingKey`](#sealingkey) | struct | An AEAD key for encrypting and signing ("sealing"), bound to a nonce sequence. |

## Structs

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

- <span id="sealingkey-seal-in-place-append-tag"></span>`fn seal_in_place_append_tag<A, InOut>(&mut self, aad: Aad<A>, in_out: &mut InOut) -> Result<(), error::Unspecified>` — [`Aad`](../index.md#aad), [`Unspecified`](../../error/index.md#unspecified)

  Encrypts and signs (“seals”) data in place, appending the tag to the

  resulting ciphertext.

  

  `key.seal_in_place_append_tag(aad, in_out)` is equivalent to:

  

  ```skip

  key.seal_in_place_separate_tag(aad, in_out.as_mut())

      .map(|tag| in_out.extend(tag.as_ref()))

  ```

- <span id="sealingkey-seal-in-place-separate-tag"></span>`fn seal_in_place_separate_tag<A>(&mut self, aad: Aad<A>, in_out: &mut [u8]) -> Result<Tag, error::Unspecified>` — [`Aad`](../index.md#aad), [`Tag`](../index.md#tag), [`Unspecified`](../../error/index.md#unspecified)

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

- <span id="sealingkey-boundkey-new"></span>`fn new(key: UnboundKey, nonce_sequence: N) -> Self` — [`UnboundKey`](../unbound_key/index.md#unboundkey)

- <span id="sealingkey-boundkey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](../index.md#algorithm)

##### `impl<N: NonceSequence> Debug for SealingKey<N>`

- <span id="sealingkey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

