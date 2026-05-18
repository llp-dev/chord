*[ring](../../index.md) / [aead](../index.md) / [nonce](index.md)*

---

# Module `nonce`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Nonce`](#nonce) | struct | A nonce for a single AEAD opening or sealing operation. |
| [`NONCE_LEN`](#nonce-len) | const | All the AEADs we support use 96-bit nonces. |

## Structs

### `Nonce`

```rust
struct Nonce([u8; 12]);
```

A nonce for a single AEAD opening or sealing operation.

The user must ensure, for a particular key, that each nonce is unique.

`Nonce` intentionally doesn't implement `Clone` to ensure that each one is
consumed at most once.

#### Implementations

- <span id="nonce-try-assume-unique-for-key"></span>`fn try_assume_unique_for_key(value: &[u8]) -> Result<Self, error::Unspecified>` — [`Unspecified`](../../error/index.md#unspecified)

  Constructs a `Nonce` with the given value, assuming that the value is

  unique for the lifetime of the key it is being used with.

  

  Fails if `value` isn't `NONCE_LEN` bytes long.

- <span id="nonce-assume-unique-for-key"></span>`fn assume_unique_for_key(value: [u8; 12]) -> Self`

  Constructs a `Nonce` with the given value, assuming that the value is

  unique for the lifetime of the key it is being used with.

#### Trait Implementations

##### `impl AsRef for Nonce`

- <span id="nonce-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 12]`

## Constants

### `NONCE_LEN`
```rust
const NONCE_LEN: usize = 12usize;
```

All the AEADs we support use 96-bit nonces.

