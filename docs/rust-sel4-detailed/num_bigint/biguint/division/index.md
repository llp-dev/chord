*[num_bigint](../../index.md) / [biguint](../index.md) / [division](index.md)*

---

# Module `division`

## Contents

- [Functions](#functions)
  - [`div_wide`](#div-wide)
  - [`div_half`](#div-half)
  - [`div_rem_digit`](#div-rem-digit)
  - [`rem_digit`](#rem-digit)
  - [`sub_mul_digit_same_len`](#sub-mul-digit-same-len)
  - [`div_rem`](#div-rem)
  - [`div_rem_ref`](#div-rem-ref)
  - [`div_rem_core`](#div-rem-core)
- [Constants](#constants)
  - [`FAST_DIV_WIDE`](#fast-div-wide)
- [Macros](#macros)
  - [`impl_rem_assign_scalar!`](#impl-rem-assign-scalar)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`div_wide`](#div-wide) | fn | x86 and x86_64 can use a real `div` instruction. |
| [`div_half`](#div-half) | fn | For small divisors, we can divide without promoting to `DoubleBigDigit` by using half-size pieces of digit, like long-division. |
| [`div_rem_digit`](#div-rem-digit) | fn |  |
| [`rem_digit`](#rem-digit) | fn |  |
| [`sub_mul_digit_same_len`](#sub-mul-digit-same-len) | fn | Subtract a multiple. |
| [`div_rem`](#div-rem) | fn |  |
| [`div_rem_ref`](#div-rem-ref) | fn |  |
| [`div_rem_core`](#div-rem-core) | fn | An implementation of the base division algorithm. |
| [`FAST_DIV_WIDE`](#fast-div-wide) | const |  |
| [`impl_rem_assign_scalar!`](#impl-rem-assign-scalar) | macro |  |

## Functions

### `div_wide`

```rust
fn div_wide(hi: u64, lo: u64, divisor: u64) -> (u64, u64)
```

x86 and x86_64 can use a real `div` instruction.

### `div_half`

```rust
fn div_half(rem: u64, digit: u64, divisor: u64) -> (u64, u64)
```

For small divisors, we can divide without promoting to `DoubleBigDigit` by
using half-size pieces of digit, like long-division.

### `div_rem_digit`

```rust
fn div_rem_digit(a: super::BigUint, b: u64) -> (super::BigUint, u64)
```

### `rem_digit`

```rust
fn rem_digit(a: &super::BigUint, b: u64) -> u64
```

### `sub_mul_digit_same_len`

```rust
fn sub_mul_digit_same_len(a: &mut [u64], b: &[u64], c: u64) -> u64
```

Subtract a multiple.
a -= b * c
Returns a borrow (if a < b then borrow > 0).

### `div_rem`

```rust
fn div_rem(u: super::BigUint, d: super::BigUint) -> (super::BigUint, super::BigUint)
```

### `div_rem_ref`

```rust
fn div_rem_ref(u: &super::BigUint, d: &super::BigUint) -> (super::BigUint, super::BigUint)
```

### `div_rem_core`

```rust
fn div_rem_core(a: super::BigUint, b: &[u64]) -> (super::BigUint, super::BigUint)
```

An implementation of the base division algorithm.
Knuth, TAOCP vol 2 section 4.3.1, algorithm D, with an improvement from exercises 19-21.

## Constants

### `FAST_DIV_WIDE`
```rust
const FAST_DIV_WIDE: bool = true;
```

## Macros

### `impl_rem_assign_scalar!`

