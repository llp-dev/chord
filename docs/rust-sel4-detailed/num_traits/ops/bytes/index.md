*[num_traits](../../index.md) / [ops](../index.md) / [bytes](index.md)*

---

# Module `bytes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NumBytes`](#numbytes) | trait |  |
| [`ToBytes`](#tobytes) | trait |  |
| [`FromBytes`](#frombytes) | trait |  |
| [`float_to_from_bytes_impl!`](#float-to-from-bytes-impl) | macro |  |
| [`int_to_from_bytes_impl!`](#int-to-from-bytes-impl) | macro |  |

## Traits

### `NumBytes`

```rust
trait NumBytes: Debug + AsRef<[u8]> + AsMut<[u8]> + PartialEq + Eq + PartialOrd + Ord + Hash + Borrow<[u8]> + BorrowMut<[u8]> { ... }
```

#### Implementors

- `T`

### `ToBytes`

```rust
trait ToBytes { ... }
```

#### Associated Types

- `type Bytes: 1`

#### Required Methods

- `fn to_be_bytes(&self) -> <Self as >::Bytes`

  Return the memory representation of this number as a byte array in big-endian byte order.

- `fn to_le_bytes(&self) -> <Self as >::Bytes`

  Return the memory representation of this number as a byte array in little-endian byte order.

#### Provided Methods

- `fn to_ne_bytes(&self) -> <Self as >::Bytes`

  Return the memory representation of this number as a byte array in native byte order.

#### Implementors

- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `FromBytes`

```rust
trait FromBytes: Sized { ... }
```

#### Associated Types

- `type Bytes: 2`

#### Required Methods

- `fn from_be_bytes(bytes: &<Self as >::Bytes) -> Self`

  Create a number from its representation as a byte array in big endian.

- `fn from_le_bytes(bytes: &<Self as >::Bytes) -> Self`

  Create a number from its representation as a byte array in little endian.

#### Provided Methods

- `fn from_ne_bytes(bytes: &<Self as >::Bytes) -> Self`

  Create a number from its memory representation as a byte array in native endianness.

#### Implementors

- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Macros

### `float_to_from_bytes_impl!`

### `int_to_from_bytes_impl!`

