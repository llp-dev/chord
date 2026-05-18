*[num_traits](../index.md) / [int](index.md)*

---

# Module `int`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PrimInt`](#primint) | trait | Generic trait for primitive integers. |
| [`one_per_byte`](#one-per-byte) | fn |  |
| [`reverse_bits_fallback`](#reverse-bits-fallback) | fn |  |
| [`prim_int_impl!`](#prim-int-impl) | macro |  |

## Traits

### `PrimInt`

```rust
trait PrimInt: Sized + Copy + Num + NumCast + Bounded + PartialOrd + Ord + Eq + Not<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Shl<usize, Output = Self> + Shr<usize, Output = Self> + CheckedAdd<Output = Self> + CheckedSub<Output = Self> + CheckedMul<Output = Self> + CheckedDiv<Output = Self> + Saturating { ... }
```

Generic trait for primitive integers.

The `PrimInt` trait is an abstraction over the builtin primitive integer types (e.g., `u8`,
`u32`, `isize`, `i128`, ...). It inherits the basic numeric traits and extends them with
bitwise operators and non-wrapping arithmetic.

The trait explicitly inherits `Copy`, `Eq`, `Ord`, and `Sized`. The intention is that all
types implementing this trait behave like primitive types that are passed by value by default
and behave like builtin integers. Furthermore, the types are expected to expose the integer
value in binary representation and support bitwise operators. The standard bitwise operations
(e.g., bitwise-and, bitwise-or, right-shift, left-shift) are inherited and the trait extends
these with introspective queries (e.g., `PrimInt::count_ones()`, `PrimInt::leading_zeros()`),
bitwise combinators (e.g., `PrimInt::rotate_left()`), and endianness converters (e.g.,
`PrimInt::to_be()`).

All `PrimInt` types are expected to be fixed-width binary integers. The width can be queried
via `T::zero().count_zeros()`. The trait currently lacks a way to query the width at
compile-time.

While a default implementation for all builtin primitive integers is provided, the trait is in
no way restricted to these. Other integer types that fulfil the requirements are free to
implement the trait was well.

This trait and many of the method names originate in the unstable `core::num::Int` trait from
the rust standard library. The original trait was never stabilized and thus removed from the
standard library.

#### Required Methods

- `fn count_ones(self) -> u32`

  Returns the number of ones in the binary representation of `self`.

- `fn count_zeros(self) -> u32`

  Returns the number of zeros in the binary representation of `self`.

- `fn leading_zeros(self) -> u32`

  Returns the number of leading zeros in the binary representation

- `fn trailing_zeros(self) -> u32`

  Returns the number of trailing zeros in the binary representation

- `fn rotate_left(self, n: u32) -> Self`

  Shifts the bits to the left by a specified amount, `n`, wrapping

- `fn rotate_right(self, n: u32) -> Self`

  Shifts the bits to the right by a specified amount, `n`, wrapping

- `fn signed_shl(self, n: u32) -> Self`

  Shifts the bits to the left by a specified amount, `n`, filling

- `fn signed_shr(self, n: u32) -> Self`

  Shifts the bits to the right by a specified amount, `n`, copying

- `fn unsigned_shl(self, n: u32) -> Self`

  Shifts the bits to the left by a specified amount, `n`, filling

- `fn unsigned_shr(self, n: u32) -> Self`

  Shifts the bits to the right by a specified amount, `n`, filling

- `fn swap_bytes(self) -> Self`

  Reverses the byte order of the integer.

- `fn from_be(x: Self) -> Self`

  Convert an integer from big endian to the target's endianness.

- `fn from_le(x: Self) -> Self`

  Convert an integer from little endian to the target's endianness.

- `fn to_be(self) -> Self`

  Convert `self` to big endian from the target's endianness.

- `fn to_le(self) -> Self`

  Convert `self` to little endian from the target's endianness.

- `fn pow(self, exp: u32) -> Self`

  Raises self to the power of `exp`, using exponentiation by squaring.

#### Provided Methods

- `fn leading_ones(self) -> u32`

  Returns the number of leading ones in the binary representation

- `fn trailing_ones(self) -> u32`

  Returns the number of trailing ones in the binary representation

- `fn reverse_bits(self) -> Self`

  Reverses the order of bits in the integer.

#### Implementors

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

## Functions

### `one_per_byte`

```rust
fn one_per_byte<P: PrimInt>() -> P
```

### `reverse_bits_fallback`

```rust
fn reverse_bits_fallback<P: PrimInt>(i: P) -> P
```

## Macros

### `prim_int_impl!`

