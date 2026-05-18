*[ring](../../index.md) / [aead](../index.md) / [aes_gcm](index.md)*

---

# Module `aes_gcm`

## Contents

- [Structs](#structs)
  - [`Key`](#key)
- [Functions](#functions)
  - [`init_128`](#init-128)
  - [`init_256`](#init-256)
  - [`init`](#init)
  - [`aes_gcm_seal`](#aes-gcm-seal)
  - [`aes_gcm_open`](#aes-gcm-open)
  - [`finish`](#finish)
- [Constants](#constants)
  - [`CHUNK_BLOCKS`](#chunk-blocks)
  - [`MAX_IN_OUT_LEN`](#max-in-out-len)
  - [`_MAX_INPUT_LEN_BOUNDED_BY_NIST`](#max-input-len-bounded-by-nist)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Key`](#key) | struct |  |
| [`init_128`](#init-128) | fn |  |
| [`init_256`](#init-256) | fn |  |
| [`init`](#init) | fn |  |
| [`aes_gcm_seal`](#aes-gcm-seal) | fn |  |
| [`aes_gcm_open`](#aes-gcm-open) | fn |  |
| [`finish`](#finish) | fn |  |
| [`CHUNK_BLOCKS`](#chunk-blocks) | const |  |
| [`MAX_IN_OUT_LEN`](#max-in-out-len) | const |  |
| [`_MAX_INPUT_LEN_BOUNDED_BY_NIST`](#max-input-len-bounded-by-nist) | const |  |

## Structs

### `Key`

```rust
struct Key {
    gcm_key: gcm::Key,
    aes_key: aes::Key,
}
```

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

## Functions

### `init_128`

```rust
fn init_128(key: &[u8], cpu_features: cpu::Features) -> Result<aead::KeyInner, error::Unspecified>
```

### `init_256`

```rust
fn init_256(key: &[u8], cpu_features: cpu::Features) -> Result<aead::KeyInner, error::Unspecified>
```

### `init`

```rust
fn init(key: &[u8], variant: aes::Variant, cpu_features: cpu::Features) -> Result<aead::KeyInner, error::Unspecified>
```

### `aes_gcm_seal`

```rust
fn aes_gcm_seal(key: &aead::KeyInner, nonce: super::Nonce, aad: super::Aad<&[u8]>, in_out: &mut [u8], cpu_features: cpu::Features) -> Result<super::Tag, error::Unspecified>
```

### `aes_gcm_open`

```rust
fn aes_gcm_open(key: &aead::KeyInner, nonce: super::Nonce, aad: super::Aad<&[u8]>, in_out: &mut [u8], src: core::ops::RangeFrom<usize>, cpu_features: cpu::Features) -> Result<super::Tag, error::Unspecified>
```

### `finish`

```rust
fn finish(aes_key: &aes::Key, gcm_ctx: gcm::Context, tag_iv: aes::Iv) -> super::Tag
```

## Constants

### `CHUNK_BLOCKS`
```rust
const CHUNK_BLOCKS: usize = 192usize;
```

### `MAX_IN_OUT_LEN`
```rust
const MAX_IN_OUT_LEN: usize = 68_719_476_704usize;
```

### `_MAX_INPUT_LEN_BOUNDED_BY_NIST`
```rust
const _MAX_INPUT_LEN_BOUNDED_BY_NIST: ();
```

