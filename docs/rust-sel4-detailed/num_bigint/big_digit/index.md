*[num_bigint](../index.md) / [big_digit](index.md)*

---

# Module `big_digit`

## Contents

- [Functions](#functions)
  - [`get_hi`](#get-hi)
  - [`get_lo`](#get-lo)
  - [`from_doublebigdigit`](#from-doublebigdigit)
  - [`to_doublebigdigit`](#to-doublebigdigit)
- [Type Aliases](#type-aliases)
  - [`BigDigit`](#bigdigit)
  - [`DoubleBigDigit`](#doublebigdigit)
- [Constants](#constants)
  - [`BITS`](#bits)
  - [`HALF_BITS`](#half-bits)
  - [`HALF`](#half)
  - [`MAX`](#max)
  - [`LO_MASK`](#lo-mask)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`get_hi`](#get-hi) | fn |  |
| [`get_lo`](#get-lo) | fn |  |
| [`from_doublebigdigit`](#from-doublebigdigit) | fn | Split one [`DoubleBigDigit`] into two [`BigDigit`]s. |
| [`to_doublebigdigit`](#to-doublebigdigit) | fn | Join two [`BigDigit`]s into one [`DoubleBigDigit`]. |
| [`BigDigit`](#bigdigit) | type |  |
| [`DoubleBigDigit`](#doublebigdigit) | type |  |
| [`BITS`](#bits) | const |  |
| [`HALF_BITS`](#half-bits) | const |  |
| [`HALF`](#half) | const |  |
| [`MAX`](#max) | const |  |
| [`LO_MASK`](#lo-mask) | const |  |

## Functions

### `get_hi`

```rust
fn get_hi(n: u128) -> u64
```

### `get_lo`

```rust
fn get_lo(n: u128) -> u64
```

### `from_doublebigdigit`

```rust
fn from_doublebigdigit(n: u128) -> (u64, u64)
```

Split one [`DoubleBigDigit`](#doublebigdigit) into two [`BigDigit`](#bigdigit)s.

### `to_doublebigdigit`

```rust
fn to_doublebigdigit(hi: u64, lo: u64) -> u128
```

Join two [`BigDigit`](#bigdigit)s into one [`DoubleBigDigit`](#doublebigdigit).

## Type Aliases

### `BigDigit`

```rust
type BigDigit = u64;
```

### `DoubleBigDigit`

```rust
type DoubleBigDigit = u128;
```

## Constants

### `BITS`
```rust
const BITS: u8 = 64u8;
```

### `HALF_BITS`
```rust
const HALF_BITS: u8 = 32u8;
```

### `HALF`
```rust
const HALF: u64 = 4_294_967_295u64;
```

### `MAX`
```rust
const MAX: u64 = 18_446_744_073_709_551_615u64;
```

### `LO_MASK`
```rust
const LO_MASK: u128 = 18_446_744_073_709_551_615u128;
```

