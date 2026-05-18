*[postcard](../index.md) / [varint](index.md)*

---

# Module `varint`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`varint_max`](#varint-max) | fn | Returns the maximum number of bytes required to encode T. |
| [`max_of_last_byte`](#max-of-last-byte) | fn | Returns the maximum value stored in the last encoded byte. |
| [`varint_usize`](#varint-usize) | fn |  |
| [`varint_u16`](#varint-u16) | fn |  |
| [`varint_u32`](#varint-u32) | fn |  |
| [`varint_u64`](#varint-u64) | fn |  |
| [`varint_u128`](#varint-u128) | fn |  |

## Functions

### `varint_max`

```rust
const fn varint_max<T: Sized>() -> usize
```

Returns the maximum number of bytes required to encode T.

### `max_of_last_byte`

```rust
const fn max_of_last_byte<T: Sized>() -> u8
```

Returns the maximum value stored in the last encoded byte.

### `varint_usize`

```rust
fn varint_usize(n: usize, out: &mut [u8; 10]) -> &mut [u8]
```

### `varint_u16`

```rust
fn varint_u16(n: u16, out: &mut [u8; 3]) -> &mut [u8]
```

### `varint_u32`

```rust
fn varint_u32(n: u32, out: &mut [u8; 5]) -> &mut [u8]
```

### `varint_u64`

```rust
fn varint_u64(n: u64, out: &mut [u8; 10]) -> &mut [u8]
```

### `varint_u128`

```rust
fn varint_u128(n: u128, out: &mut [u8; 19]) -> &mut [u8]
```

