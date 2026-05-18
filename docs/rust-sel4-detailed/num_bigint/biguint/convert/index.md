*[num_bigint](../../index.md) / [biguint](../index.md) / [convert](index.md)*

---

# Module `convert`

## Contents

- [Functions](#functions)
  - [`fls`](#fls)
  - [`ilog2`](#ilog2)
  - [`from_bitwise_digits_le`](#from-bitwise-digits-le)
  - [`from_inexact_bitwise_digits_le`](#from-inexact-bitwise-digits-le)
  - [`from_radix_digits_be`](#from-radix-digits-be)
  - [`from_radix_be`](#from-radix-be)
  - [`from_radix_le`](#from-radix-le)
  - [`high_bits_to_u64`](#high-bits-to-u64)
  - [`to_bitwise_digits_le`](#to-bitwise-digits-le)
  - [`to_inexact_bitwise_digits_le`](#to-inexact-bitwise-digits-le)
  - [`to_radix_digits_le`](#to-radix-digits-le)
  - [`to_radix_le`](#to-radix-le)
  - [`to_str_radix_reversed`](#to-str-radix-reversed)
  - [`get_radix_base`](#get-radix-base)
  - [`get_half_radix_base`](#get-half-radix-base)
  - [`generate_radix_bases`](#generate-radix-bases)
- [Macros](#macros)
  - [`impl_try_from_biguint!`](#impl-try-from-biguint)
  - [`impl_biguint_from_uint!`](#impl-biguint-from-uint)
  - [`impl_biguint_try_from_int!`](#impl-biguint-try-from-int)
  - [`impl_to_biguint!`](#impl-to-biguint)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fls`](#fls) | fn | Find last set bit fls(0) == 0, fls(u32::MAX) == 32 |
| [`ilog2`](#ilog2) | fn |  |
| [`from_bitwise_digits_le`](#from-bitwise-digits-le) | fn |  |
| [`from_inexact_bitwise_digits_le`](#from-inexact-bitwise-digits-le) | fn |  |
| [`from_radix_digits_be`](#from-radix-digits-be) | fn |  |
| [`from_radix_be`](#from-radix-be) | fn |  |
| [`from_radix_le`](#from-radix-le) | fn |  |
| [`high_bits_to_u64`](#high-bits-to-u64) | fn |  |
| [`to_bitwise_digits_le`](#to-bitwise-digits-le) | fn |  |
| [`to_inexact_bitwise_digits_le`](#to-inexact-bitwise-digits-le) | fn |  |
| [`to_radix_digits_le`](#to-radix-digits-le) | fn |  |
| [`to_radix_le`](#to-radix-le) | fn |  |
| [`to_str_radix_reversed`](#to-str-radix-reversed) | fn |  |
| [`get_radix_base`](#get-radix-base) | fn | Returns the greatest power of the radix for the `BigDigit` bit size |
| [`get_half_radix_base`](#get-half-radix-base) | fn | Returns the greatest power of the radix for half the `BigDigit` bit size |
| [`generate_radix_bases`](#generate-radix-bases) | fn | Generate tables of the greatest power of each radix that is less that the given maximum. |
| [`impl_try_from_biguint!`](#impl-try-from-biguint) | macro |  |
| [`impl_biguint_from_uint!`](#impl-biguint-from-uint) | macro |  |
| [`impl_biguint_try_from_int!`](#impl-biguint-try-from-int) | macro |  |
| [`impl_to_biguint!`](#impl-to-biguint) | macro |  |

## Functions

### `fls`

```rust
fn fls<T: PrimInt>(v: T) -> u8
```

Find last set bit
fls(0) == 0, fls(u32::MAX) == 32

### `ilog2`

```rust
fn ilog2<T: PrimInt>(v: T) -> u8
```

### `from_bitwise_digits_le`

```rust
fn from_bitwise_digits_le(v: &[u8], bits: u8) -> super::BigUint
```

### `from_inexact_bitwise_digits_le`

```rust
fn from_inexact_bitwise_digits_le(v: &[u8], bits: u8) -> super::BigUint
```

### `from_radix_digits_be`

```rust
fn from_radix_digits_be(v: &[u8], radix: u32) -> super::BigUint
```

### `from_radix_be`

```rust
fn from_radix_be(buf: &[u8], radix: u32) -> Option<super::BigUint>
```

### `from_radix_le`

```rust
fn from_radix_le(buf: &[u8], radix: u32) -> Option<super::BigUint>
```

### `high_bits_to_u64`

```rust
fn high_bits_to_u64(v: &super::BigUint) -> u64
```

### `to_bitwise_digits_le`

```rust
fn to_bitwise_digits_le(u: &super::BigUint, bits: u8) -> alloc::vec::Vec<u8>
```

### `to_inexact_bitwise_digits_le`

```rust
fn to_inexact_bitwise_digits_le(u: &super::BigUint, bits: u8) -> alloc::vec::Vec<u8>
```

### `to_radix_digits_le`

```rust
fn to_radix_digits_le(u: &super::BigUint, radix: u32) -> alloc::vec::Vec<u8>
```

### `to_radix_le`

```rust
fn to_radix_le(u: &super::BigUint, radix: u32) -> alloc::vec::Vec<u8>
```

### `to_str_radix_reversed`

```rust
fn to_str_radix_reversed(u: &super::BigUint, radix: u32) -> alloc::vec::Vec<u8>
```

### `get_radix_base`

```rust
fn get_radix_base(radix: u32) -> (u64, usize)
```

Returns the greatest power of the radix for the `BigDigit` bit size

### `get_half_radix_base`

```rust
fn get_half_radix_base(radix: u32) -> (u64, usize)
```

Returns the greatest power of the radix for half the `BigDigit` bit size

### `generate_radix_bases`

```rust
const fn generate_radix_bases(max: u64) -> [(u64, usize); 257]
```

Generate tables of the greatest power of each radix that is less that the given maximum. These
are returned from `get_radix_base` to batch the multiplication/division of radix conversions on
full `BigUint` values, operating on primitive integers as much as possible.

e.g. BASES_16[3] = (59049, 10) // 3¹⁰ fits in u16, but 3¹¹ is too big
     BASES_32[3] = (3486784401, 20)
     BASES_64[3] = (12157665459056928801, 40)

Powers of two are not included, just zeroed, as they're implemented with shifts.

## Macros

### `impl_try_from_biguint!`

### `impl_biguint_from_uint!`

### `impl_biguint_try_from_int!`

### `impl_to_biguint!`

