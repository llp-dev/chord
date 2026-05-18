*[ring](../../index.md) / [aead](../index.md) / [opening_key](index.md)*

---

# Module `opening_key`

Authenticated Encryption with Associated Data (AEAD).

See [Authenticated encryption: relations among notions and analysis of the
generic composition paradigm][AEAD] for an introduction to the concept of
AEADs.



## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OpeningKey`](#openingkey) | struct | An AEAD key for authenticating and decrypting ("opening"), bound to a nonce sequence. |

## Structs

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

- <span id="openingkey-open-in-place"></span>`fn open_in_place<'in_out, A>(&mut self, aad: Aad<A>, in_out: &'in_out mut [u8]) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Aad`](../index.md#aad), [`Unspecified`](../../error/index.md#unspecified)

  Authenticates and decrypts (“opens”) data in place.

  

  `aad` is the additional authenticated data (AAD), if any.

  

  On input, `in_out` must be the ciphertext followed by the tag. When

  `open_in_place()` returns `Ok(plaintext)`, the input ciphertext

  has been overwritten by the plaintext; `plaintext` will refer to the

  plaintext without the tag.

  

  When `open_in_place()` returns `Err(..)`, `in_out` may have been

  overwritten in an unspecified way.

- <span id="openingkey-open-within"></span>`fn open_within<'in_out, A>(&mut self, aad: Aad<A>, in_out: &'in_out mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>` — [`Aad`](../index.md#aad), [`Unspecified`](../../error/index.md#unspecified)

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

- <span id="openingkey-boundkey-new"></span>`fn new(key: UnboundKey, nonce_sequence: N) -> Self` — [`UnboundKey`](../unbound_key/index.md#unboundkey)

- <span id="openingkey-boundkey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](../index.md#algorithm)

##### `impl<N: NonceSequence> Debug for OpeningKey<N>`

- <span id="openingkey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

