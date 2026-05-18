*[num_bigint](../../index.md) / [bigint](../index.md) / [convert](index.md)*

---

# Module `convert`

## Contents

- [Functions](#functions)
  - [`from_signed_bytes_be`](#from-signed-bytes-be)
  - [`from_signed_bytes_le`](#from-signed-bytes-le)
  - [`to_signed_bytes_be`](#to-signed-bytes-be)
  - [`to_signed_bytes_le`](#to-signed-bytes-le)
  - [`twos_complement_le`](#twos-complement-le)
  - [`twos_complement_be`](#twos-complement-be)
  - [`twos_complement`](#twos-complement)
- [Macros](#macros)
  - [`impl_try_from_bigint!`](#impl-try-from-bigint)
  - [`impl_bigint_from_int!`](#impl-bigint-from-int)
  - [`impl_bigint_from_uint!`](#impl-bigint-from-uint)
  - [`impl_to_bigint!`](#impl-to-bigint)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`from_signed_bytes_be`](#from-signed-bytes-be) | fn |  |
| [`from_signed_bytes_le`](#from-signed-bytes-le) | fn |  |
| [`to_signed_bytes_be`](#to-signed-bytes-be) | fn |  |
| [`to_signed_bytes_le`](#to-signed-bytes-le) | fn |  |
| [`twos_complement_le`](#twos-complement-le) | fn | Perform in-place two's complement of the given binary representation, in little-endian byte order. |
| [`twos_complement_be`](#twos-complement-be) | fn | Perform in-place two's complement of the given binary representation in big-endian byte order. |
| [`twos_complement`](#twos-complement) | fn | Perform in-place two's complement of the given digit iterator starting from the least significant byte. |
| [`impl_try_from_bigint!`](#impl-try-from-bigint) | macro |  |
| [`impl_bigint_from_int!`](#impl-bigint-from-int) | macro |  |
| [`impl_bigint_from_uint!`](#impl-bigint-from-uint) | macro |  |
| [`impl_to_bigint!`](#impl-to-bigint) | macro |  |

## Functions

### `from_signed_bytes_be`

```rust
fn from_signed_bytes_be(digits: &[u8]) -> super::BigInt
```

### `from_signed_bytes_le`

```rust
fn from_signed_bytes_le(digits: &[u8]) -> super::BigInt
```

### `to_signed_bytes_be`

```rust
fn to_signed_bytes_be(x: &super::BigInt) -> alloc::vec::Vec<u8>
```

### `to_signed_bytes_le`

```rust
fn to_signed_bytes_le(x: &super::BigInt) -> alloc::vec::Vec<u8>
```

### `twos_complement_le`

```rust
fn twos_complement_le(digits: &mut [u8])
```

Perform in-place two's complement of the given binary representation,
in little-endian byte order.

### `twos_complement_be`

```rust
fn twos_complement_be(digits: &mut [u8])
```

Perform in-place two's complement of the given binary representation
in big-endian byte order.

### `twos_complement`

```rust
fn twos_complement<'a, I>(digits: I)
where
    I: IntoIterator<Item = &'a mut u8>
```

Perform in-place two's complement of the given digit iterator
starting from the least significant byte.

## Macros

### `impl_try_from_bigint!`

### `impl_bigint_from_int!`

### `impl_bigint_from_uint!`

### `impl_to_bigint!`

