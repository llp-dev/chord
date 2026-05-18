*[gimli](../../index.md) / [leb128](../index.md) / [write](index.md)*

---

# Module `write`

A module for writing integers encoded as LEB128.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Leb128`](#leb128) | struct | An encoded LEB128 value. |
| [`uleb128_size`](#uleb128-size) | fn | Return the size of the LEB128 encoding of the given unsigned number. |
| [`sleb128_size`](#sleb128-size) | fn | Return the size of the LEB128 encoding of the given signed number. |

## Structs

### `Leb128`

```rust
struct Leb128 {
    bytes: [u8; 10],
    len: u8,
}
```

An encoded LEB128 value.

May be signed or unsigned.

#### Implementations

- <span id="leb128-bytes"></span>`fn bytes(&self) -> &[u8]`

  Return the bytes for the encoded value.

- <span id="leb128-len"></span>`fn len(&self) -> usize`

  Return the length of the encoded bytes.

- <span id="leb128-unsigned"></span>`fn unsigned(val: u64) -> Self`

  Generate the LEB128 encoding for the given unsigned number.

- <span id="leb128-signed"></span>`fn signed(val: i64) -> Self`

  Generate the LEB128 encoding for the given signed number.

#### Trait Implementations

##### `impl Clone for Leb128`

- <span id="leb128-clone"></span>`fn clone(&self) -> Leb128` — [`Leb128`](#leb128)

##### `impl Copy for Leb128`

##### `impl Debug for Leb128`

- <span id="leb128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `uleb128_size`

```rust
fn uleb128_size(val: u64) -> usize
```

Return the size of the LEB128 encoding of the given unsigned number.

### `sleb128_size`

```rust
fn sleb128_size(val: i64) -> usize
```

Return the size of the LEB128 encoding of the given signed number.

