# Crate `zmij`

[![github]](https://github.com/dtolnay/zmij)&ensp;[![crates-io]](https://crates.io/crates/zmij)&ensp;[![docs-rs]](https://docs.rs/zmij)



<br>

A double-to-string conversion algorithm based on [Schubfach] and [yy].

This Rust implementation is a line-by-line port of Victor Zverovich's
implementation in C++, <https://github.com/vitaut/zmij>.


<br>

# Example

```rust
fn main() {
    let mut buffer = zmij::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

<br>

## Performance

The [dtoa-benchmark] compares this library and other Rust floating point
formatting implementations across a range of precisions. The vertical axis
in this chart shows nanoseconds taken by a single execution of
`zmij::Buffer::new().format_finite(value)` so a lower result indicates a
faster library.

![performance](https://raw.githubusercontent.com/dtolnay/zmij/master/dtoa-benchmark.png)

## Contents

- [Modules](#modules)
  - [`stdarch_x86`](#stdarch-x86)
  - [`traits`](#traits)
  - [`private`](#private)
- [Structs](#structs)
  - [`uint128`](#uint128)
  - [`Pow10SignificandsTable`](#pow10significandstable)
  - [`ExpShiftTable`](#expshifttable)
  - [`Digits2`](#digits2)
  - [`ToDecimalResult`](#todecimalresult)
  - [`Buffer`](#buffer)
- [Traits](#traits)
  - [`FloatTraits`](#floattraits)
  - [`Float`](#float)
- [Functions](#functions)
  - [`select_if_less`](#select-if-less)
  - [`umul128`](#umul128)
  - [`umul128_hi64`](#umul128-hi64)
  - [`umul192_hi128`](#umul192-hi128)
  - [`umulhi_inexact_to_odd`](#umulhi-inexact-to-odd)
  - [`compute_dec_exp`](#compute-dec-exp)
  - [`do_compute_exp_shift`](#do-compute-exp-shift)
  - [`compute_exp_shift`](#compute-exp-shift)
  - [`count_trailing_nonzeros`](#count-trailing-nonzeros)
  - [`digits2`](#digits2)
  - [`to_bcd8`](#to-bcd8)
  - [`write_if`](#write-if)
  - [`write8`](#write8)
  - [`write_significand`](#write-significand)
  - [`to_decimal_schubfach`](#to-decimal-schubfach)
  - [`to_decimal_fast`](#to-decimal-fast)
  - [`write`](#write)
- [Constants](#constants)
  - [`BUFFER_SIZE`](#buffer-size)
  - [`NAN`](#nan)
  - [`INFINITY`](#infinity)
  - [`NEG_INFINITY`](#neg-infinity)
  - [`USE_UMUL128_HI64`](#use-umul128-hi64)
  - [`POW10S`](#pow10s)
  - [`HIGH_PARTS`](#high-parts)
  - [`FIXUPS`](#fixups)
  - [`DIV10K_EXP`](#div10k-exp)
  - [`DIV10K_SIG`](#div10k-sig)
  - [`NEG10K`](#neg10k)
  - [`DIV100_EXP`](#div100-exp)
  - [`DIV100_SIG`](#div100-sig)
  - [`NEG100`](#neg100)
  - [`DIV10_EXP`](#div10-exp)
  - [`DIV10_SIG`](#div10-sig)
  - [`NEG10`](#neg10)
  - [`ZEROS`](#zeros)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`stdarch_x86`](#stdarch-x86) | mod |  |
| [`traits`](#traits) | mod |  |
| [`private`](#private) | mod |  |
| [`uint128`](#uint128) | struct |  |
| [`Pow10SignificandsTable`](#pow10significandstable) | struct |  |
| [`ExpShiftTable`](#expshifttable) | struct |  |
| [`Digits2`](#digits2) | struct |  |
| [`ToDecimalResult`](#todecimalresult) | struct |  |
| [`Buffer`](#buffer) | struct | Safe API for formatting floating point numbers to text. |
| [`FloatTraits`](#floattraits) | trait |  |
| [`Float`](#float) | trait | A floating point number, f32 or f64, that can be written into a [`zmij::Buffer`][Buffer]. |
| [`select_if_less`](#select-if-less) | fn |  |
| [`umul128`](#umul128) | fn |  |
| [`umul128_hi64`](#umul128-hi64) | fn |  |
| [`umul192_hi128`](#umul192-hi128) | fn |  |
| [`umulhi_inexact_to_odd`](#umulhi-inexact-to-odd) | fn |  |
| [`compute_dec_exp`](#compute-dec-exp) | fn |  |
| [`do_compute_exp_shift`](#do-compute-exp-shift) | fn |  |
| [`compute_exp_shift`](#compute-exp-shift) | fn |  |
| [`count_trailing_nonzeros`](#count-trailing-nonzeros) | fn |  |
| [`digits2`](#digits2) | fn |  |
| [`to_bcd8`](#to-bcd8) | fn |  |
| [`write_if`](#write-if) | fn |  |
| [`write8`](#write8) | fn |  |
| [`write_significand`](#write-significand) | fn |  |
| [`to_decimal_schubfach`](#to-decimal-schubfach) | fn |  |
| [`to_decimal_fast`](#to-decimal-fast) | fn |  |
| [`write`](#write) | fn | Writes the shortest correctly rounded decimal representation of `value` to `buffer`. |
| [`BUFFER_SIZE`](#buffer-size) | const |  |
| [`NAN`](#nan) | const |  |
| [`INFINITY`](#infinity) | const |  |
| [`NEG_INFINITY`](#neg-infinity) | const |  |
| [`USE_UMUL128_HI64`](#use-umul128-hi64) | const |  |
| [`POW10S`](#pow10s) | const |  |
| [`HIGH_PARTS`](#high-parts) | const |  |
| [`FIXUPS`](#fixups) | const |  |
| [`DIV10K_EXP`](#div10k-exp) | const |  |
| [`DIV10K_SIG`](#div10k-sig) | const |  |
| [`NEG10K`](#neg10k) | const |  |
| [`DIV100_EXP`](#div100-exp) | const |  |
| [`DIV100_SIG`](#div100-sig) | const |  |
| [`NEG100`](#neg100) | const |  |
| [`DIV10_EXP`](#div10-exp) | const |  |
| [`DIV10_SIG`](#div10-sig) | const |  |
| [`NEG10`](#neg10) | const |  |
| [`ZEROS`](#zeros) | const |  |

## Modules

- [`stdarch_x86`](stdarch_x86/index.md)
- [`traits`](traits/index.md)
- [`private`](private/index.md)

## Structs

### `uint128`

```rust
struct uint128 {
    hi: u64,
    lo: u64,
}
```

#### Trait Implementations

##### `impl Clone for uint128`

- <span id="uint128-clone"></span>`fn clone(&self) -> uint128` — [`uint128`](#uint128)

##### `impl Copy for uint128`

### `Pow10SignificandsTable`

```rust
struct Pow10SignificandsTable {
    data: [u64; 1234],
}
```

#### Implementations

- <span id="pow10significandstable-const-compress"></span>`const COMPRESS: bool`

- <span id="pow10significandstable-const-split-tables"></span>`const SPLIT_TABLES: bool`

- <span id="pow10significandstable-const-num-pow10"></span>`const NUM_POW10: usize`

- <span id="pow10significandstable-compute"></span>`const fn compute(i: u32) -> uint128` — [`uint128`](#uint128)

- <span id="pow10significandstable-new"></span>`const fn new() -> Self`

- <span id="pow10significandstable-get-unchecked"></span>`unsafe fn get_unchecked(&self, dec_exp: i32) -> uint128` — [`uint128`](#uint128)

### `ExpShiftTable`

```rust
struct ExpShiftTable {
    data: [u8; 2048],
}
```

#### Implementations

- <span id="expshifttable-const-enable"></span>`const ENABLE: bool`

### `Digits2`

```rust
struct Digits2([u8; 200]);
```

### `ToDecimalResult`

```rust
struct ToDecimalResult {
    sig: i64,
    exp: i32,
}
```

### `Buffer`

```rust
struct Buffer {
    bytes: [core::mem::MaybeUninit<u8>; 24],
}
```

Safe API for formatting floating point numbers to text.

## Example

```rust
let mut buffer = zmij::Buffer::new();
let printed = buffer.format_finite(1.234);
assert_eq!(printed, "1.234");
```

#### Implementations

- <span id="buffer-new"></span>`fn new() -> Self`

  This is a cheap operation; you don't need to worry about reusing buffers

  for efficiency.

- <span id="buffer-format"></span>`fn format<F: Float>(&mut self, f: F) -> &str`

  Print a floating point number into this buffer and return a reference to

  its string representation within the buffer.

  

  # Special cases

  

  This function formats NaN as the string "NaN", positive infinity as

  "inf", and negative infinity as "-inf" to match std::fmt.

  

  If your input is known to be finite, you may get better performance by

  calling the `format_finite` method instead of `format` to avoid the

  checks for special cases.

- <span id="buffer-format-finite"></span>`fn format_finite<F: Float>(&mut self, f: F) -> &str`

  Print a floating point number into this buffer and return a reference to

  its string representation within the buffer.

  

  # Special cases

  

  This function **does not** check for NaN or infinity. If the input

  number is not a finite float, the printed representation will be some

  correctly formatted but unspecified numerical value.

  

  Please check `is_finite` yourself before calling this function, or

  check `is_nan` and `is_infinite` and handle those cases yourself.

  

  

#### Trait Implementations

##### `impl Default for Buffer`

- <span id="buffer-default"></span>`fn default() -> Self`

## Traits

### `FloatTraits`

```rust
trait FloatTraits: traits::Float { ... }
```

#### Associated Types

- `type SigType: 1`

#### Associated Constants

- `const NUM_BITS: i32`

- `const NUM_SIG_BITS: i32`

- `const NUM_EXP_BITS: i32`

- `const EXP_MASK: i32`

- `const EXP_BIAS: i32`

- `const EXP_OFFSET: i32`

- `const IMPLICIT_BIT: <Self as >::SigType`

#### Required Methods

- `fn to_bits(self) -> <Self as >::SigType`

#### Provided Methods

- `fn is_negative(bits: <Self as >::SigType) -> bool`

- `fn get_sig(bits: <Self as >::SigType) -> <Self as >::SigType`

- `fn get_exp(bits: <Self as >::SigType) -> i64`

#### Implementors

- `f32`
- `f64`

### `Float`

```rust
trait Float: private::Sealed { ... }
```

A floating point number, f32 or f64, that can be written into a
[`zmij::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of the
`zmij` crate.

#### Implementors

- `f32`
- `f64`

## Functions

### `select_if_less`

```rust
fn select_if_less(lhs: u64, rhs: u64, true_value: i64, false_value: i64) -> i64
```

### `umul128`

```rust
const fn umul128(x: u64, y: u64) -> u128
```

### `umul128_hi64`

```rust
const fn umul128_hi64(x: u64, y: u64) -> u64
```

### `umul192_hi128`

```rust
fn umul192_hi128(x_hi: u64, x_lo: u64, y: u64) -> uint128
```

### `umulhi_inexact_to_odd`

```rust
fn umulhi_inexact_to_odd<UInt>(x_hi: u64, x_lo: u64, y: UInt) -> UInt
where
    UInt: traits::UInt
```

### `compute_dec_exp`

```rust
const fn compute_dec_exp(bin_exp: i32, regular: bool) -> i32
```

### `do_compute_exp_shift`

```rust
const fn do_compute_exp_shift(bin_exp: i32, dec_exp: i32) -> u8
```

### `compute_exp_shift`

```rust
unsafe fn compute_exp_shift<UInt, const ONLY_REGULAR: bool>(bin_exp: i32, dec_exp: i32) -> u8
where
    UInt: traits::UInt
```

### `count_trailing_nonzeros`

```rust
fn count_trailing_nonzeros(x: u64) -> usize
```

### `digits2`

```rust
unsafe fn digits2(value: usize) -> &'static u16
```

### `to_bcd8`

```rust
fn to_bcd8(abcdefgh: u64) -> u64
```

### `write_if`

```rust
unsafe fn write_if(buffer: *mut u8, digit: u32, condition: bool) -> *mut u8
```

### `write8`

```rust
unsafe fn write8(buffer: *mut u8, value: u64)
```

### `write_significand`

```rust
unsafe fn write_significand<Float>(buffer: *mut u8, value: u64, extra_digit: bool) -> *mut u8
where
    Float: FloatTraits
```

### `to_decimal_schubfach`

```rust
fn to_decimal_schubfach<UInt>(bin_sig: UInt, bin_exp: i64, regular: bool) -> ToDecimalResult
where
    UInt: traits::UInt
```

### `to_decimal_fast`

```rust
fn to_decimal_fast<Float, UInt>(bin_sig: UInt, raw_exp: i64, regular: bool) -> ToDecimalResult
where
    Float: FloatTraits,
    UInt: traits::UInt
```

### `write`

```rust
unsafe fn write<Float>(value: Float, buffer: *mut u8) -> *mut u8
where
    Float: FloatTraits
```

Writes the shortest correctly rounded decimal representation of `value` to
`buffer`. `buffer` should point to a buffer of size `buffer_size` or larger.

## Constants

### `BUFFER_SIZE`
```rust
const BUFFER_SIZE: usize = 24usize;
```

### `NAN`
```rust
const NAN: &str;
```

### `INFINITY`
```rust
const INFINITY: &str;
```

### `NEG_INFINITY`
```rust
const NEG_INFINITY: &str;
```

### `USE_UMUL128_HI64`
```rust
const USE_UMUL128_HI64: bool = false;
```

### `POW10S`
```rust
const POW10S: [u64; 28];
```

### `HIGH_PARTS`
```rust
const HIGH_PARTS: [uint128; 23];
```

### `FIXUPS`
```rust
const FIXUPS: [u32; 20];
```

### `DIV10K_EXP`
```rust
const DIV10K_EXP: i32 = 40i32;
```

### `DIV10K_SIG`
```rust
const DIV10K_SIG: u32 = 109_951_163u32;
```

### `NEG10K`
```rust
const NEG10K: u32 = 4_294_957_296u32;
```

### `DIV100_EXP`
```rust
const DIV100_EXP: i32 = 19i32;
```

### `DIV100_SIG`
```rust
const DIV100_SIG: u32 = 5_243u32;
```

### `NEG100`
```rust
const NEG100: u32 = 65_436u32;
```

### `DIV10_EXP`
```rust
const DIV10_EXP: i32 = 10i32;
```

### `DIV10_SIG`
```rust
const DIV10_SIG: u32 = 103u32;
```

### `NEG10`
```rust
const NEG10: u32 = 246u32;
```

### `ZEROS`
```rust
const ZEROS: u64 = 3_472_328_296_227_680_304u64;
```

