*[ring](../../index.md) / [aead](../index.md) / [poly1305](index.md)*

---

# Module `poly1305`

## Contents

- [Structs](#structs)
  - [`Key`](#key)
  - [`Context`](#context)
  - [`poly1305_state`](#poly1305-state)
- [Functions](#functions)
  - [`sign`](#sign)
- [Constants](#constants)
  - [`BLOCK_LEN`](#block-len)
  - [`KEY_LEN`](#key-len)
  - [`OPAQUE_LEN`](#opaque-len)
- [Macros](#macros)
  - [`dispatch!`](#dispatch)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Key`](#key) | struct | A Poly1305 key. |
| [`Context`](#context) | struct |  |
| [`poly1305_state`](#poly1305-state) | struct |  |
| [`sign`](#sign) | fn | Implements the original, non-IETF padding semantics. |
| [`BLOCK_LEN`](#block-len) | const |  |
| [`KEY_LEN`](#key-len) | const |  |
| [`OPAQUE_LEN`](#opaque-len) | const |  |
| [`dispatch!`](#dispatch) | macro |  |

## Structs

### `Key`

```rust
struct Key {
    key_and_nonce: [u8; 32],
}
```

A Poly1305 key.

#### Implementations

- <span id="key-new"></span>`fn new(key_and_nonce: [u8; 32]) -> Self`

### `Context`

```rust
struct Context {
    state: poly1305_state,
    cpu_features: cpu::Features,
}
```

#### Implementations

- <span id="context-from-key"></span>`fn from_key(_: Key, cpu_features: cpu::Features) -> Self` — [`Key`](#key), [`Features`](../../cpu/index.md#features)

- <span id="context-update"></span>`fn update(&mut self, input: &[u8])`

- <span id="context-finish"></span>`fn finish(self) -> Tag` — [`Tag`](../index.md#tag)

### `poly1305_state`

```rust
struct poly1305_state([u8; 512]);
```

## Functions

### `sign`

```rust
fn sign(key: Key, input: &[u8], cpu_features: cpu::Features) -> super::Tag
```

Implements the original, non-IETF padding semantics.

This is used by chacha20_poly1305_openssh and the standalone
poly1305 test vectors.

## Constants

### `BLOCK_LEN`
```rust
const BLOCK_LEN: usize = 16usize;
```

### `KEY_LEN`
```rust
const KEY_LEN: usize = 32usize;
```

### `OPAQUE_LEN`
```rust
const OPAQUE_LEN: usize = 512usize;
```

## Macros

### `dispatch!`

