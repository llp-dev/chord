*[ring](../../index.md) / [aead](../index.md) / [unbound_key](index.md)*

---

# Module `unbound_key`

Authenticated Encryption with Associated Data (AEAD).

See [Authenticated encryption: relations among notions and analysis of the
generic composition paradigm][AEAD] for an introduction to the concept of
AEADs.



## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UnboundKey`](#unboundkey) | struct | An AEAD key without a designated role or nonce sequence. |

## Structs

### `UnboundKey`

```rust
struct UnboundKey {
    inner: super::LessSafeKey,
}
```

An AEAD key without a designated role or nonce sequence.

#### Implementations

- <span id="unboundkey-new"></span>`fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>` — [`Algorithm`](../index.md#algorithm), [`Unspecified`](../../error/index.md#unspecified)

  Constructs a `UnboundKey`.

  

  Fails if `key_bytes.len() != algorithm.key_len()`.

- <span id="unboundkey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](../index.md#algorithm)

  The key's AEAD algorithm.

- <span id="unboundkey-into-inner"></span>`fn into_inner(self) -> LessSafeKey` — [`LessSafeKey`](../less_safe_key/index.md#lesssafekey)

#### Trait Implementations

##### `impl Debug for UnboundKey`

- <span id="unboundkey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

