*[ring](../../index.md) / [aead](../index.md) / [aes](index.md)*

---

# Module `aes`

## Contents

- [Structs](#structs)
  - [`Key`](#key)
  - [`AES_KEY`](#aes-key)
  - [`Counter`](#counter)
  - [`Iv`](#iv)
- [Enums](#enums)
  - [`Variant`](#variant)
  - [`Implementation`](#implementation)
- [Functions](#functions)
  - [`set_encrypt_key`](#set-encrypt-key)
  - [`encrypt_block_`](#encrypt-block)
  - [`ctr32_encrypt_blocks_`](#ctr32-encrypt-blocks)
  - [`detect_implementation`](#detect-implementation)
- [Constants](#constants)
  - [`MAX_ROUNDS`](#max-rounds)
- [Macros](#macros)
  - [`set_encrypt_key!`](#set-encrypt-key)
  - [`encrypt_block!`](#encrypt-block)
  - [`ctr32_encrypt_blocks!`](#ctr32-encrypt-blocks)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Key`](#key) | struct |  |
| [`AES_KEY`](#aes-key) | struct |  |
| [`Counter`](#counter) | struct | Nonce \|\| Counter, all big-endian. |
| [`Iv`](#iv) | struct | The IV for a single block encryption. |
| [`Variant`](#variant) | enum |  |
| [`Implementation`](#implementation) | enum |  |
| [`set_encrypt_key`](#set-encrypt-key) | fn |  |
| [`encrypt_block_`](#encrypt-block) | fn |  |
| [`ctr32_encrypt_blocks_`](#ctr32-encrypt-blocks) | fn |  |
| [`detect_implementation`](#detect-implementation) | fn |  |
| [`MAX_ROUNDS`](#max-rounds) | const |  |
| [`set_encrypt_key!`](#set-encrypt-key) | macro |  |
| [`encrypt_block!`](#encrypt-block) | macro |  |
| [`ctr32_encrypt_blocks!`](#ctr32-encrypt-blocks) | macro |  |

## Structs

### `Key`

```rust
struct Key {
    inner: AES_KEY,
}
```

#### Implementations

- <span id="key-new"></span>`fn new(bytes: &[u8], variant: Variant, cpu_features: cpu::Features) -> Result<Self, error::Unspecified>` — [`Variant`](#variant), [`Features`](../../cpu/index.md#features), [`Unspecified`](../../error/index.md#unspecified)

- <span id="key-encrypt-block"></span>`fn encrypt_block(&self, a: Block, cpu_features: cpu::Features) -> Block` — [`Block`](../block/index.md#block), [`Features`](../../cpu/index.md#features)

- <span id="key-encrypt-iv-xor-block"></span>`fn encrypt_iv_xor_block(&self, iv: Iv, input: Block, cpu_features: cpu::Features) -> Block` — [`Iv`](#iv), [`Block`](../block/index.md#block), [`Features`](../../cpu/index.md#features)

- <span id="key-ctr32-encrypt-within"></span>`fn ctr32_encrypt_within(&self, in_out: &mut [u8], src: RangeFrom<usize>, ctr: &mut Counter, cpu_features: cpu::Features)` — [`Counter`](#counter), [`Features`](../../cpu/index.md#features)

- <span id="key-new-mask"></span>`fn new_mask(&self, sample: Sample) -> [u8; 5]` — [`Sample`](../quic/index.md#sample)

- <span id="key-is-aes-hw"></span>`fn is_aes_hw(&self, cpu_features: cpu::Features) -> bool` — [`Features`](../../cpu/index.md#features)

- <span id="key-inner-less-safe"></span>`fn inner_less_safe(&self) -> &AES_KEY` — [`AES_KEY`](#aes-key)

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

### `AES_KEY`

```rust
struct AES_KEY {
    pub rd_key: [u32; 60],
    pub rounds: u32,
}
```

#### Trait Implementations

##### `impl Clone for AES_KEY`

- <span id="aes-key-clone"></span>`fn clone(&self) -> AES_KEY` — [`AES_KEY`](#aes-key)

### `Counter`

```rust
struct Counter([crate::endian::BigEndian<u32>; 4]);
```

Nonce || Counter, all big-endian.

#### Implementations

- <span id="counter-one"></span>`fn one(nonce: Nonce) -> Self` — [`Nonce`](../nonce/index.md#nonce)

- <span id="counter-increment"></span>`fn increment(&mut self) -> Iv` — [`Iv`](#iv)

- <span id="counter-increment-by-less-safe"></span>`fn increment_by_less_safe(&mut self, increment_by: u32)`

### `Iv`

```rust
struct Iv(super::block::Block);
```

The IV for a single block encryption.

Intentionally not `Clone` to ensure each is used only once.

#### Implementations

- <span id="iv-into-block-less-safe"></span>`fn into_block_less_safe(self) -> Block` — [`Block`](../block/index.md#block)

  "Less safe" because it defeats attempts to use the type system to prevent reuse of the IV.

## Enums

### `Variant`

```rust
enum Variant {
    AES_128,
    AES_256,
}
```

### `Implementation`

```rust
enum Implementation {
    HWAES,
    VPAES_BSAES,
    NOHW,
}
```

#### Trait Implementations

##### `impl Clone for Implementation`

- <span id="implementation-clone"></span>`fn clone(&self) -> Implementation` — [`Implementation`](#implementation)

##### `impl Copy for Implementation`

## Functions

### `set_encrypt_key`

```rust
fn set_encrypt_key(f: fn(*const u8, u32, &mut AES_KEY) -> i32, bytes: &[u8], key_bits: crate::bits::BitLength, key: &mut AES_KEY) -> Result<(), error::Unspecified>
```

### `encrypt_block_`

```rust
fn encrypt_block_(f: fn(&super::block::Block, *mut super::block::Block, &AES_KEY), a: super::block::Block, key: &Key) -> super::block::Block
```

### `ctr32_encrypt_blocks_`

```rust
fn ctr32_encrypt_blocks_(f: fn(*const [u8; 16], *mut [u8; 16], usize, &AES_KEY, &Counter), in_out: &mut [u8], src: core::ops::RangeFrom<usize>, key: &AES_KEY, ctr: &mut Counter)
```

### `detect_implementation`

```rust
fn detect_implementation(cpu_features: cpu::Features) -> Implementation
```

## Constants

### `MAX_ROUNDS`
```rust
const MAX_ROUNDS: usize = 14usize;
```

## Macros

### `set_encrypt_key!`

### `encrypt_block!`

### `ctr32_encrypt_blocks!`

