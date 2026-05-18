*[num_bigint](../../index.md) / [biguint](../index.md) / [multiplication](index.md)*

---

# Module `multiplication`

## Contents

- [Functions](#functions)
  - [`mac_with_carry`](#mac-with-carry)
  - [`mul_with_carry`](#mul-with-carry)
  - [`mac_digit`](#mac-digit)
  - [`bigint_from_slice`](#bigint-from-slice)
  - [`mac3`](#mac3)
  - [`mul3`](#mul3)
  - [`scalar_mul`](#scalar-mul)
  - [`sub_sign`](#sub-sign)
- [Macros](#macros)
  - [`impl_mul!`](#impl-mul)
  - [`impl_mul_assign!`](#impl-mul-assign)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`mac_with_carry`](#mac-with-carry) | fn |  |
| [`mul_with_carry`](#mul-with-carry) | fn |  |
| [`mac_digit`](#mac-digit) | fn | Three argument multiply accumulate: acc += b * c |
| [`bigint_from_slice`](#bigint-from-slice) | fn |  |
| [`mac3`](#mac3) | fn | Three argument multiply accumulate: acc += b * c |
| [`mul3`](#mul3) | fn |  |
| [`scalar_mul`](#scalar-mul) | fn |  |
| [`sub_sign`](#sub-sign) | fn |  |
| [`impl_mul!`](#impl-mul) | macro |  |
| [`impl_mul_assign!`](#impl-mul-assign) | macro |  |

## Functions

### `mac_with_carry`

```rust
fn mac_with_carry(a: u64, b: u64, c: u64, acc: &mut u128) -> u64
```

### `mul_with_carry`

```rust
fn mul_with_carry(a: u64, b: u64, acc: &mut u128) -> u64
```

### `mac_digit`

```rust
fn mac_digit(acc: &mut [u64], b: &[u64], c: u64)
```

Three argument multiply accumulate:
acc += b * c

### `bigint_from_slice`

```rust
fn bigint_from_slice(slice: &[u64]) -> crate::BigInt
```

### `mac3`

```rust
fn mac3(acc: &mut [u64], b: &[u64], c: &[u64])
```

Three argument multiply accumulate:
acc += b * c

### `mul3`

```rust
fn mul3(x: &[u64], y: &[u64]) -> super::BigUint
```

### `scalar_mul`

```rust
fn scalar_mul(a: &mut super::BigUint, b: u64)
```

### `sub_sign`

```rust
fn sub_sign(a: &[u64], b: &[u64]) -> (crate::Sign, super::BigUint)
```

## Macros

### `impl_mul!`

### `impl_mul_assign!`

