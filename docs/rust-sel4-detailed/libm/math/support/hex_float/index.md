*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [hex_float](index.md)*

---

# Module `hex_float`

Utilities for working with hex float formats.

## Contents

- [Structs](#structs)
  - [`HexFloatParseError`](#hexfloatparseerror)
  - [`Parsed`](#parsed)
- [Functions](#functions)
  - [`hf32`](#hf32)
  - [`hf64`](#hf64)
  - [`parse_hex_exact`](#parse-hex-exact)
  - [`parse_any`](#parse-any)
  - [`parse_finite`](#parse-finite)
  - [`shr_odd_rounding`](#shr-odd-rounding)
  - [`shr2_round`](#shr2-round)
  - [`parse_hex`](#parse-hex)
  - [`dec_digit`](#dec-digit)
  - [`hex_digit`](#hex-digit)
  - [`u128_ilog2`](#u128-ilog2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HexFloatParseError`](#hexfloatparseerror) | struct |  |
| [`Parsed`](#parsed) | struct | A parsed finite and unsigned floating point number. |
| [`hf32`](#hf32) | fn | Construct a 32-bit float from hex float representation (C-style) |
| [`hf64`](#hf64) | fn | Construct a 64-bit float from hex float representation (C-style) |
| [`parse_hex_exact`](#parse-hex-exact) | fn | Parses any float to its bitwise representation, returning an error if it cannot be represented exactly |
| [`parse_any`](#parse-any) | fn | Parse any float from hex to its bitwise representation. |
| [`parse_finite`](#parse-finite) | fn |  |
| [`shr_odd_rounding`](#shr-odd-rounding) | fn | Shift right, rounding all inexact divisions to the nearest odd number E.g. |
| [`shr2_round`](#shr2-round) | fn | Divide by 4, rounding with the given mode |
| [`parse_hex`](#parse-hex) | fn | Parse a hexadecimal float x |
| [`dec_digit`](#dec-digit) | fn |  |
| [`hex_digit`](#hex-digit) | fn |  |
| [`u128_ilog2`](#u128-ilog2) | fn | `u128::ilog2` |

## Structs

### `HexFloatParseError`

```rust
struct HexFloatParseError(&'static str);
```

#### Trait Implementations

##### `impl Clone for HexFloatParseError`

- <span id="hexfloatparseerror-clone"></span>`fn clone(&self) -> HexFloatParseError` — [`HexFloatParseError`](#hexfloatparseerror)

##### `impl Copy for HexFloatParseError`

##### `impl Debug for HexFloatParseError`

- <span id="hexfloatparseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Parsed`

```rust
struct Parsed {
    sig: u128,
    exp: i32,
}
```

A parsed finite and unsigned floating point number.

#### Fields

- **`sig`**: `u128`

  Absolute value sig * 2^exp

## Functions

### `hf32`

```rust
const fn hf32(s: &str) -> f32
```

Construct a 32-bit float from hex float representation (C-style)

### `hf64`

```rust
const fn hf64(s: &str) -> f64
```

Construct a 64-bit float from hex float representation (C-style)

### `parse_hex_exact`

```rust
const fn parse_hex_exact(s: &str, bits: u32, sig_bits: u32) -> Result<u128, HexFloatParseError>
```

Parses any float to its bitwise representation, returning an error if it cannot be represented exactly

### `parse_any`

```rust
const fn parse_any(s: &str, bits: u32, sig_bits: u32, round: super::Round) -> Result<(u128, super::Status), HexFloatParseError>
```

Parse any float from hex to its bitwise representation.

### `parse_finite`

```rust
const fn parse_finite(b: &[u8], bits: u32, sig_bits: u32, rounding_mode: super::Round) -> Result<(u128, super::Status), HexFloatParseError>
```

### `shr_odd_rounding`

```rust
const fn shr_odd_rounding(x: u128, k: u32) -> u128
```

Shift right, rounding all inexact divisions to the nearest odd number
E.g. (0 >> 4) -> 0, (1..=31 >> 4) -> 1, (32 >> 4) -> 2, ...

Useful for reducing a number before rounding the last two bits, since
the result of the final rounding is preserved for all rounding modes.

### `shr2_round`

```rust
const fn shr2_round(x: u128, round: super::Round) -> u128
```

Divide by 4, rounding with the given mode

### `parse_hex`

```rust
const fn parse_hex(b: &[u8]) -> Result<Parsed, HexFloatParseError>
```

Parse a hexadecimal float x

### `dec_digit`

```rust
const fn dec_digit(c: u8) -> Option<u8>
```

### `hex_digit`

```rust
const fn hex_digit(c: u8) -> Option<u8>
```

### `u128_ilog2`

```rust
const fn u128_ilog2(v: u128) -> u32
```

`u128::ilog2`

