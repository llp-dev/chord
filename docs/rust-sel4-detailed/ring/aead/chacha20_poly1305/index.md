*[ring](../../index.md) / [aead](../index.md) / [chacha20_poly1305](index.md)*

---

# Module `chacha20_poly1305`

## Contents

- [Structs](#structs)
  - [`Out`](#out)
- [Functions](#functions)
  - [`chacha20_poly1305_init`](#chacha20-poly1305-init)
  - [`chacha20_poly1305_seal`](#chacha20-poly1305-seal)
  - [`chacha20_poly1305_open`](#chacha20-poly1305-open)
  - [`has_integrated`](#has-integrated)
  - [`finish`](#finish)
  - [`poly1305_update_padded_16`](#poly1305-update-padded-16)
  - [`derive_poly1305_key`](#derive-poly1305-key)
- [Type Aliases](#type-aliases)
  - [`Key`](#key)
- [Constants](#constants)
  - [`MAX_IN_OUT_LEN`](#max-in-out-len)
  - [`_MAX_IN_OUT_LEN_BOUNDED_BY_RFC`](#max-in-out-len-bounded-by-rfc)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Out`](#out) | struct |  |
| [`chacha20_poly1305_init`](#chacha20-poly1305-init) | fn | Copies \|key\| into \|ctx_buf\|. |
| [`chacha20_poly1305_seal`](#chacha20-poly1305-seal) | fn |  |
| [`chacha20_poly1305_open`](#chacha20-poly1305-open) | fn |  |
| [`has_integrated`](#has-integrated) | fn |  |
| [`finish`](#finish) | fn |  |
| [`poly1305_update_padded_16`](#poly1305-update-padded-16) | fn |  |
| [`derive_poly1305_key`](#derive-poly1305-key) | fn |  |
| [`Key`](#key) | type |  |
| [`MAX_IN_OUT_LEN`](#max-in-out-len) | const |  |
| [`_MAX_IN_OUT_LEN_BOUNDED_BY_RFC`](#max-in-out-len-bounded-by-rfc) | const |  |

## Structs

### `Out`

```rust
struct Out {
    tag: [u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for Out`

- <span id="out-clone"></span>`fn clone(&self) -> Out` — [`Out`](#out)

##### `impl Copy for Out`

## Functions

### `chacha20_poly1305_init`

```rust
fn chacha20_poly1305_init(key: &[u8], _cpu_features: cpu::Features) -> Result<aead::KeyInner, error::Unspecified>
```

Copies |key| into |ctx_buf|.

### `chacha20_poly1305_seal`

```rust
fn chacha20_poly1305_seal(key: &aead::KeyInner, nonce: super::Nonce, aad: super::Aad<&[u8]>, in_out: &mut [u8], cpu_features: cpu::Features) -> Result<super::Tag, error::Unspecified>
```

### `chacha20_poly1305_open`

```rust
fn chacha20_poly1305_open(key: &aead::KeyInner, nonce: super::Nonce, aad: super::Aad<&[u8]>, in_out: &mut [u8], src: core::ops::RangeFrom<usize>, cpu_features: cpu::Features) -> Result<super::Tag, error::Unspecified>
```

### `has_integrated`

```rust
fn has_integrated(cpu_features: cpu::Features) -> bool
```

### `finish`

```rust
fn finish(auth: poly1305::Context, aad_len: usize, in_out_len: usize) -> super::Tag
```

### `poly1305_update_padded_16`

```rust
fn poly1305_update_padded_16(ctx: &mut poly1305::Context, input: &[u8])
```

### `derive_poly1305_key`

```rust
fn derive_poly1305_key(chacha_key: &chacha::Key, iv: super::chacha::Iv) -> poly1305::Key
```

## Type Aliases

### `Key`

```rust
type Key = chacha::Key;
```

## Constants

### `MAX_IN_OUT_LEN`
```rust
const MAX_IN_OUT_LEN: usize = 274_877_906_880usize;
```

### `_MAX_IN_OUT_LEN_BOUNDED_BY_RFC`
```rust
const _MAX_IN_OUT_LEN_BOUNDED_BY_RFC: ();
```

