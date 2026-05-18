*[ring](../../index.md) / [aead](../index.md) / [chacha](index.md)*

---

# Module `chacha`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Key`](#key) | struct |  |
| [`Counter`](#counter) | struct | Counter \|\| Nonce, all native endian. |
| [`Iv`](#iv) | struct | The IV for a single block encryption. |
| [`KEY_LEN`](#key-len) | const |  |
| [`BLOCK_LEN`](#block-len) | const |  |

## Structs

### `Key`

```rust
struct Key {
    words: [u32; 8],
}
```

#### Implementations

- <span id="key-new"></span>`fn new(value: [u8; 32]) -> Self`

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

### `Counter`

```rust
struct Counter([u32; 4]);
```

Counter || Nonce, all native endian.

#### Implementations

- <span id="counter-zero"></span>`fn zero(nonce: Nonce) -> Self` — [`Nonce`](../nonce/index.md#nonce)

- <span id="counter-from-nonce-and-ctr"></span>`fn from_nonce_and_ctr(nonce: Nonce, ctr: u32) -> Self` — [`Nonce`](../nonce/index.md#nonce)

- <span id="counter-increment"></span>`fn increment(&mut self) -> Iv` — [`Iv`](#iv)

### `Iv`

```rust
struct Iv([u32; 4]);
```

The IV for a single block encryption.

Intentionally not `Clone` to ensure each is used only once.

#### Implementations

- <span id="iv-assume-unique-for-key"></span>`fn assume_unique_for_key(value: [u8; 16]) -> Self`

- <span id="iv-into-counter-for-single-block-less-safe"></span>`fn into_counter_for_single_block_less_safe(self) -> Counter` — [`Counter`](#counter)

## Constants

### `KEY_LEN`
```rust
const KEY_LEN: usize = 32usize;
```

### `BLOCK_LEN`
```rust
const BLOCK_LEN: usize = 64usize;
```

