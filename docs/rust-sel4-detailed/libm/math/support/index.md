*[libm](../../index.md) / [math](../index.md) / [support](index.md)*

---

# Module `support`

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`big`](#big)
  - [`env`](#env)
  - [`feature_detect`](#feature-detect)
  - [`float_traits`](#float-traits)
  - [`hex_float`](#hex-float)
  - [`int_traits`](#int-traits)
  - [`modular`](#modular)
- [Structs](#structs)
  - [`i256`](#i256)
  - [`u256`](#u256)
  - [`FpResult`](#fpresult)
  - [`Status`](#status)
- [Enums](#enums)
  - [`Round`](#round)
- [Traits](#traits)
  - [`DFloat`](#dfloat)
  - [`Float`](#float)
  - [`HFloat`](#hfloat)
  - [`CastFrom`](#castfrom)
  - [`CastInto`](#castinto)
  - [`DInt`](#dint)
  - [`HInt`](#hint)
  - [`Int`](#int)
  - [`MinInt`](#minint)
  - [`NarrowingDiv`](#narrowingdiv)
- [Functions](#functions)
  - [`hf32`](#hf32)
  - [`hf64`](#hf64)
  - [`linear_mul_reduction`](#linear-mul-reduction)
  - [`cold_path`](#cold-path)
- [Type Aliases](#type-aliases)
  - [`IntTy`](#intty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`big`](#big) | mod | Integers used for wide operations, larger than `u128`. |
| [`env`](#env) | mod | Support for rounding directions and status flags as specified by IEEE 754. |
| [`feature_detect`](#feature-detect) | mod | Helpers for runtime target feature detection that are shared across architectures. |
| [`float_traits`](#float-traits) | mod |  |
| [`hex_float`](#hex-float) | mod | Utilities for working with hex float formats. |
| [`int_traits`](#int-traits) | mod |  |
| [`modular`](#modular) | mod | This module provides accelerated modular multiplication by large powers of two, which is needed for computing floating point remainders in `fmod` and similar functions. |
| [`i256`](#i256) | struct |  |
| [`u256`](#u256) | struct |  |
| [`FpResult`](#fpresult) | struct |  |
| [`Status`](#status) | struct |  |
| [`Round`](#round) | enum |  |
| [`DFloat`](#dfloat) | trait |  |
| [`Float`](#float) | trait |  |
| [`HFloat`](#hfloat) | trait |  |
| [`CastFrom`](#castfrom) | trait |  |
| [`CastInto`](#castinto) | trait |  |
| [`DInt`](#dint) | trait |  |
| [`HInt`](#hint) | trait |  |
| [`Int`](#int) | trait |  |
| [`MinInt`](#minint) | trait |  |
| [`NarrowingDiv`](#narrowingdiv) | trait |  |
| [`hf32`](#hf32) | fn |  |
| [`hf64`](#hf64) | fn |  |
| [`linear_mul_reduction`](#linear-mul-reduction) | fn |  |
| [`cold_path`](#cold-path) | fn | Hint to the compiler that the current path is cold. |
| [`IntTy`](#intty) | type |  |

## Modules

- [`macros`](macros/index.md)
- [`big`](big/index.md) — Integers used for wide operations, larger than `u128`.
- [`env`](env/index.md) — Support for rounding directions and status flags as specified by IEEE 754.
- [`feature_detect`](feature_detect/index.md) — Helpers for runtime target feature detection that are shared across architectures.
- [`float_traits`](float_traits/index.md)
- [`hex_float`](hex_float/index.md) — Utilities for working with hex float formats.
- [`int_traits`](int_traits/index.md)
- [`modular`](modular/index.md) — This module provides accelerated modular multiplication by large powers

## Structs

### `i256`

```rust
struct i256 {
    pub hi: i128,
    pub lo: u128,
}
```

A 256-bit signed integer represented as two 128-bit native-endian limbs.

#### Trait Implementations

##### `impl Add for i256`

- <span id="i256-add-type-output"></span>`type Output = i256`

- <span id="i256-add"></span>`fn add(self, rhs: Self) -> <Self as >::Output`

##### `impl BitOr for i256`

- <span id="i256-bitor-type-output"></span>`type Output = i256`

- <span id="i256-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for i256`

- <span id="i256-clone"></span>`fn clone(&self) -> i256` — [`i256`](big/index.md#i256)

##### `impl Copy for i256`

##### `impl DInt for i256`

- <span id="i256-dint-type-h"></span>`type H = i128`

- <span id="i256-dint-lo"></span>`fn lo(self) -> <Self as >::H` — [`DInt`](int_traits/index.md#dint)

- <span id="i256-dint-hi"></span>`fn hi(self) -> <Self as >::H` — [`DInt`](int_traits/index.md#dint)

##### `impl Debug for i256`

- <span id="i256-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for i256`

##### `impl MinInt for i256`

- <span id="i256-minint-type-othersign"></span>`type OtherSign = u256`

- <span id="i256-minint-type-unsigned"></span>`type Unsigned = u256`

- <span id="i256-minint-const-signed"></span>`const SIGNED: bool`

- <span id="i256-minint-const-bits"></span>`const BITS: u32`

- <span id="i256-minint-const-zero"></span>`const ZERO: Self`

- <span id="i256-minint-const-one"></span>`const ONE: Self`

- <span id="i256-minint-const-min"></span>`const MIN: Self`

- <span id="i256-minint-const-max"></span>`const MAX: Self`

##### `impl Not for i256`

- <span id="i256-not-type-output"></span>`type Output = i256`

- <span id="i256-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Ord for i256`

- <span id="i256-ord-cmp"></span>`fn cmp(&self, other: &i256) -> cmp::Ordering` — [`i256`](big/index.md#i256)

##### `impl PartialEq for i256`

- <span id="i256-partialeq-eq"></span>`fn eq(&self, other: &i256) -> bool` — [`i256`](big/index.md#i256)

##### `impl PartialOrd for i256`

- <span id="i256-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &i256) -> option::Option<cmp::Ordering>` — [`i256`](big/index.md#i256)

##### `impl Shl for i256`

- <span id="i256-shl-type-output"></span>`type Output = i256`

- <span id="i256-shl"></span>`fn shl(self, rhs: u32) -> <Self as >::Output`

##### `impl Shr for i256`

- <span id="i256-shr-type-output"></span>`type Output = i256`

- <span id="i256-shr"></span>`fn shr(self, rhs: u32) -> <Self as >::Output`

##### `impl StructuralPartialEq for i256`

##### `impl Sub for i256`

- <span id="i256-sub-type-output"></span>`type Output = i256`

- <span id="i256-sub"></span>`fn sub(self, rhs: Self) -> <Self as >::Output`

### `u256`

```rust
struct u256 {
    pub hi: u128,
    pub lo: u128,
}
```

A 256-bit unsigned integer represented as two 128-bit native-endian limbs.

#### Implementations

- <span id="u256-signed"></span>`fn signed(self) -> i256` — [`i256`](big/index.md#i256)

  Reinterpret as a signed integer

#### Trait Implementations

##### `impl Add for u256`

- <span id="u256-add-type-output"></span>`type Output = u256`

- <span id="u256-add"></span>`fn add(self, rhs: Self) -> <Self as >::Output`

##### `impl BitOr for u256`

- <span id="u256-bitor-type-output"></span>`type Output = u256`

- <span id="u256-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for u256`

- <span id="u256-clone"></span>`fn clone(&self) -> u256` — [`u256`](big/index.md#u256)

##### `impl Copy for u256`

##### `impl DInt for u256`

- <span id="u256-dint-type-h"></span>`type H = u128`

- <span id="u256-dint-lo"></span>`fn lo(self) -> <Self as >::H` — [`DInt`](int_traits/index.md#dint)

- <span id="u256-dint-hi"></span>`fn hi(self) -> <Self as >::H` — [`DInt`](int_traits/index.md#dint)

##### `impl Debug for u256`

- <span id="u256-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for u256`

##### `impl MinInt for u256`

- <span id="u256-minint-type-othersign"></span>`type OtherSign = i256`

- <span id="u256-minint-type-unsigned"></span>`type Unsigned = u256`

- <span id="u256-minint-const-signed"></span>`const SIGNED: bool`

- <span id="u256-minint-const-bits"></span>`const BITS: u32`

- <span id="u256-minint-const-zero"></span>`const ZERO: Self`

- <span id="u256-minint-const-one"></span>`const ONE: Self`

- <span id="u256-minint-const-min"></span>`const MIN: Self`

- <span id="u256-minint-const-max"></span>`const MAX: Self`

##### `impl NarrowingDiv for crate::support::u256`

- <span id="cratesupportu256-narrowingdiv-unchecked-narrowing-div-rem"></span>`unsafe fn unchecked_narrowing_div_rem(self, n: <Self as >::H) -> (<Self as >::H, <Self as >::H)` — [`DInt`](int_traits/index.md#dint)

##### `impl Not for u256`

- <span id="u256-not-type-output"></span>`type Output = u256`

- <span id="u256-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Ord for u256`

- <span id="u256-ord-cmp"></span>`fn cmp(&self, other: &u256) -> cmp::Ordering` — [`u256`](big/index.md#u256)

##### `impl PartialEq for u256`

- <span id="u256-partialeq-eq"></span>`fn eq(&self, other: &u256) -> bool` — [`u256`](big/index.md#u256)

##### `impl PartialOrd for u256`

- <span id="u256-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &u256) -> option::Option<cmp::Ordering>` — [`u256`](big/index.md#u256)

##### `impl Shl for u256`

- <span id="u256-shl-type-output"></span>`type Output = u256`

- <span id="u256-shl"></span>`fn shl(self, rhs: u32) -> <Self as >::Output`

##### `impl Shr for u256`

- <span id="u256-shr-type-output"></span>`type Output = u256`

- <span id="u256-shr"></span>`fn shr(self, rhs: u32) -> <Self as >::Output`

##### `impl StructuralPartialEq for u256`

##### `impl Sub for u256`

- <span id="u256-sub-type-output"></span>`type Output = u256`

- <span id="u256-sub"></span>`fn sub(self, rhs: Self) -> <Self as >::Output`

### `FpResult<T>`

```rust
struct FpResult<T> {
    pub val: T,
    pub status: Status,
}
```

A value combined with a floating point status.

#### Implementations

- <span id="fpresult-new"></span>`fn new(val: T, status: Status) -> Self` — [`Status`](env/index.md#status)

- <span id="fpresult-ok"></span>`fn ok(val: T) -> Self`

  Return `val` with `Status::OK`.

### `Status`

```rust
struct Status(u8);
```

IEEE 754 exception status flags.

#### Implementations

- <span id="status-const-ok"></span>`const OK: Self`

- <span id="status-const-invalid"></span>`const INVALID: Self`

- <span id="status-const-divide-by-zero"></span>`const DIVIDE_BY_ZERO: Self`

- <span id="status-const-overflow"></span>`const OVERFLOW: Self`

- <span id="status-const-underflow"></span>`const UNDERFLOW: Self`

- <span id="status-const-inexact"></span>`const INEXACT: Self`

- <span id="status-underflow"></span>`const fn underflow(self) -> bool`

  True if `UNDERFLOW` is set.

- <span id="status-overflow"></span>`const fn overflow(self) -> bool`

  True if `OVERFLOW` is set.

- <span id="status-set-underflow"></span>`fn set_underflow(&mut self, val: bool)`

- <span id="status-inexact"></span>`const fn inexact(self) -> bool`

  True if `INEXACT` is set.

- <span id="status-set-inexact"></span>`fn set_inexact(&mut self, val: bool)`

- <span id="status-set-flag"></span>`fn set_flag(&mut self, val: bool, mask: Self)`

- <span id="status-with"></span>`const fn with(self, rhs: Self) -> Self`

#### Trait Implementations

##### `impl Clone for Status`

- <span id="status-clone"></span>`fn clone(&self) -> Status` — [`Status`](env/index.md#status)

##### `impl Copy for Status`

##### `impl Debug for Status`

- <span id="status-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Status`

##### `impl PartialEq for Status`

- <span id="status-partialeq-eq"></span>`fn eq(&self, other: &Status) -> bool` — [`Status`](env/index.md#status)

##### `impl StructuralPartialEq for Status`

## Enums

### `Round`

```rust
enum Round {
    Nearest,
    Negative,
    Positive,
    Zero,
}
```

IEEE 754 rounding mode, excluding the optional `roundTiesToAway` version of nearest.

Integer representation comes from what CORE-MATH uses for indexing.

#### Variants

- **`Nearest`**

  IEEE 754 nearest, `roundTiesToEven`.

- **`Negative`**

  IEEE 754 `roundTowardNegative`.

- **`Positive`**

  IEEE 754 `roundTowardPositive`.

- **`Zero`**

  IEEE 754 `roundTowardZero`.

#### Trait Implementations

##### `impl Clone for Round`

- <span id="round-clone"></span>`fn clone(&self) -> Round` — [`Round`](env/index.md#round)

##### `impl Copy for Round`

##### `impl Debug for Round`

- <span id="round-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Round`

- <span id="round-partialeq-eq"></span>`fn eq(&self, other: &Round) -> bool` — [`Round`](env/index.md#round)

##### `impl StructuralPartialEq for Round`

## Traits

### `DFloat`

```rust
trait DFloat: Float { ... }
```

Trait for floats twice the bit width of another integer.

#### Associated Types

- `type H: 1`

#### Required Methods

- `fn narrow(self) -> <Self as >::H`

  Narrow the float type.

#### Implementors

- `f64`

### `Float`

```rust
trait Float: Copy + fmt::Debug + PartialEq + PartialOrd + ops::AddAssign + ops::MulAssign + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> + ops::Rem<Output = Self> + ops::Neg<Output = Self> + 'static { ... }
```

Trait for some basic operations on floats

#### Associated Types

- `type Int: 1`

- `type SignedInt: 3`

#### Associated Constants

- `const ZERO: Self`

- `const NEG_ZERO: Self`

- `const ONE: Self`

- `const NEG_ONE: Self`

- `const INFINITY: Self`

- `const NEG_INFINITY: Self`

- `const NAN: Self`

- `const NEG_NAN: Self`

- `const MAX: Self`

- `const MIN: Self`

- `const EPSILON: Self`

- `const PI: Self`

- `const NEG_PI: Self`

- `const FRAC_PI_2: Self`

- `const MIN_POSITIVE_NORMAL: Self`

- `const BITS: u32`

- `const SIG_BITS: u32`

- `const EXP_BITS: u32`

- `const EXP_SAT: u32`

- `const EXP_BIAS: u32`

- `const EXP_MAX: i32`

- `const EXP_MIN: i32`

- `const EXP_MIN_SUBNORM: i32`

- `const SIGN_MASK: <Self as >::Int`

- `const SIG_MASK: <Self as >::Int`

- `const EXP_MASK: <Self as >::Int`

- `const IMPLICIT_BIT: <Self as >::Int`

#### Required Methods

- `fn to_bits(self) -> <Self as >::Int`

  Returns `self` transmuted to `Self::Int`

- `fn is_nan(self) -> bool`

  Returns true if the value is NaN.

- `fn is_infinite(self) -> bool`

  Returns true if the value is +inf or -inf.

- `fn is_sign_negative(self) -> bool`

  Returns true if the sign is negative. Extracts the sign bit regardless of zero or NaN.

- `fn from_bits(a: <Self as >::Int) -> Self`

  Returns a `Self::Int` transmuted back to `Self`

- `fn abs(self) -> Self`

- `fn copysign(self, other: Self) -> Self`

  Returns a number composed of the magnitude of self and the sign of sign.

- `fn fma(self, y: Self, z: Self) -> Self`

  Fused multiply add, rounding once.

- `fn normalize(significand: <Self as >::Int) -> (i32, <Self as >::Int)`

  Returns (normalized exponent, normalized significand)

#### Provided Methods

- `fn to_bits_signed(self) -> <Self as >::SignedInt`

  Returns `self` transmuted to `Self::SignedInt`

- `fn biteq(self, rhs: Self) -> bool`

  Check bitwise equality.

- `fn eq_repr(self, rhs: Self) -> bool`

  Checks if two floats have the same bit representation. *Except* for NaNs! NaN can be

- `fn is_sign_positive(self) -> bool`

  Returns true if the sign is positive. Extracts the sign bit regardless of zero or NaN.

- `fn is_subnormal(self) -> bool`

  Returns if `self` is subnormal.

- `fn ex(self) -> u32`

  Returns the exponent, not adjusting for bias, not accounting for subnormals or zero.

- `fn exp_unbiased(self) -> i32`

  Extract the exponent and adjust it for bias, not accounting for subnormals or zero.

- `fn frac(self) -> <Self as >::Int`

  Returns the significand with no implicit bit (or the "fractional" part)

- `fn from_parts(negative: bool, exponent: u32, significand: <Self as >::Int) -> Self`

  Constructs a `Self` from its parts. Inputs are treated as bits and shifted into position.

- `fn signum(self) -> Self`

  Returns a number that represents the sign of self.

- `fn canonicalize(self) -> Self`

  Make a best-effort attempt to canonicalize the number. Note that this is allowed

#### Implementors

- `f32`
- `f64`

### `HFloat`

```rust
trait HFloat: Float { ... }
```

Trait for floats half the bit width of another float.

#### Associated Types

- `type D: 1`

#### Required Methods

- `fn widen(self) -> <Self as >::D`

  Widen the float type.

#### Implementors

- `f32`

### `CastFrom<T: Copy>`

```rust
trait CastFrom<T: Copy>: Copy { ... }
```

#### Required Methods

- `fn cast_from(value: T) -> Self`

  By default, casts should be exact.

- `fn cast_from_lossy(value: T) -> Self`

  Call for casts that are expected to truncate.

#### Implementors

- `T`

### `CastInto<T: Copy>`

```rust
trait CastInto<T: Copy>: Copy { ... }
```

Trait to express (possibly lossy) casting of integers

#### Required Methods

- `fn cast(self) -> T`

  By default, casts should be exact.

- `fn cast_lossy(self) -> T`

  Call for casts that are expected to truncate.

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

### `DInt`

```rust
trait DInt: MinInt + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Shl<u32, Output = Self> + ops::Shr<u32, Output = Self> + Ord { ... }
```

Trait for integers twice the bit width of another integer. This is implemented for all
primitives except for `u8`, because there is not a smaller primitive.

#### Associated Types

- `type H: 1`

#### Required Methods

- `fn lo(self) -> <Self as >::H`

  Returns the low half of `self`

- `fn hi(self) -> <Self as >::H`

  Returns the high half of `self`

#### Provided Methods

- `fn lo_hi(self) -> (<Self as >::H, <Self as >::H)`

  Returns the low and high halves of `self` as a tuple

- `fn from_lo_hi(lo: <Self as >::H, hi: <Self as >::H) -> Self`

  Constructs an integer using lower and higher half parts

#### Implementors

- [`i256`](big/index.md#i256)
- [`u256`](big/index.md#u256)
- `i128`
- `i16`
- `i32`
- `i64`
- `u128`
- `u16`
- `u32`
- `u64`

### `HInt`

```rust
trait HInt: Int { ... }
```

Trait for integers half the bit width of another integer. This is implemented for all
primitives except for `u128`, because it there is not a larger primitive.

#### Associated Types

- `type D: 2`

#### Required Methods

- `fn widen(self) -> <Self as >::D`

  Widens (using default extension) the integer to have double bit width

- `fn zero_widen(self) -> <Self as >::D`

  Widens (zero extension only) the integer to have double bit width. This is needed to get

- `fn widen_hi(self) -> <Self as >::D`

  Widens the integer to have double bit width and shifts the integer into the higher bits

- `fn zero_widen_mul(self, rhs: Self) -> <Self as >::D`

  Widening multiplication with zero widening. This cannot overflow.

- `fn widen_mul(self, rhs: Self) -> <Self as >::D`

  Widening multiplication. This cannot overflow.

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`

### `Int`

```rust
trait Int: MinInt + fmt::Display + fmt::Binary + fmt::LowerHex + ops::AddAssign + ops::SubAssign + ops::MulAssign + ops::DivAssign + ops::RemAssign + ops::BitAndAssign + ops::BitOrAssign + ops::BitXorAssign + ops::ShlAssign<i32> + ops::ShlAssign<u32> + ops::ShrAssign<u32> + ops::ShrAssign<i32> + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> + ops::Rem<Output = Self> + ops::Shl<i32, Output = Self> + ops::Shl<u32, Output = Self> + ops::Shr<i32, Output = Self> + ops::Shr<u32, Output = Self> + ops::BitXor<Output = Self> + ops::BitAnd<Output = Self> + cmp::Ord + From<bool> + CastFrom<i32> + CastFrom<u16> + CastFrom<u32> + CastFrom<u8> + CastFrom<usize> + CastInto<i32> + CastInto<u16> + CastInto<u32> + CastInto<u8> + CastInto<usize> { ... }
```

Trait for some basic operations on integers

#### Required Methods

- `fn signed(self) -> <<Self as >::Unsigned as MinInt>::OtherSign`

- `fn unsigned(self) -> <Self as >::Unsigned`

- `fn from_unsigned(unsigned: <Self as >::Unsigned) -> Self`

- `fn abs(self) -> Self`

- `fn unsigned_abs(self) -> <Self as >::Unsigned`

- `fn from_bool(b: bool) -> Self`

- `fn logical_shr(self, other: u32) -> Self`

  Prevents the need for excessive conversions between signed and unsigned

- `fn abs_diff(self, other: Self) -> <Self as >::Unsigned`

  Absolute difference between two integers.

- `fn is_zero(self) -> bool`

- `fn checked_add(self, other: Self) -> Option<Self>`

- `fn checked_sub(self, other: Self) -> Option<Self>`

- `fn wrapping_neg(self) -> Self`

- `fn wrapping_add(self, other: Self) -> Self`

- `fn wrapping_mul(self, other: Self) -> Self`

- `fn wrapping_sub(self, other: Self) -> Self`

- `fn wrapping_shl(self, other: u32) -> Self`

- `fn wrapping_shr(self, other: u32) -> Self`

- `fn rotate_left(self, other: u32) -> Self`

- `fn overflowing_add(self, other: Self) -> (Self, bool)`

- `fn overflowing_sub(self, other: Self) -> (Self, bool)`

- `fn carrying_add(self, other: Self, carry: bool) -> (Self, bool)`

- `fn borrowing_sub(self, other: Self, borrow: bool) -> (Self, bool)`

- `fn leading_zeros(self) -> u32`

- `fn trailing_zeros(self) -> u32`

- `fn ilog2(self) -> u32`

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

### `MinInt`

```rust
trait MinInt: Copy + fmt::Debug + ops::BitOr<Output = Self> + ops::Not<Output = Self> + ops::Shl<u32, Output = Self> { ... }
```

Minimal integer implementations needed on all integer types, including wide integers.

#### Associated Types

- `type OtherSign: 1`

- `type Unsigned: 1`

#### Associated Constants

- `const SIGNED: bool`

- `const BITS: u32`

- `const ZERO: Self`

- `const ONE: Self`

- `const MIN: Self`

- `const MAX: Self`

#### Implementors

- [`i256`](big/index.md#i256)
- [`u256`](big/index.md#u256)
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

### `NarrowingDiv`

```rust
trait NarrowingDiv: DInt + MinInt<Unsigned = Self> { ... }
```

Trait for unsigned division of a double-wide integer
when the quotient doesn't overflow.

This is the inverse of widening multiplication:
 - for any `x` and nonzero `y`: `x.widen_mul(y).checked_narrowing_div_rem(y) == Some((x, 0))`,
 - and for any `r in 0..y`: `x.carrying_mul(y, r).checked_narrowing_div_rem(y) == Some((x, r))`,

#### Required Methods

- `fn unchecked_narrowing_div_rem(self, n: <Self as >::H) -> (<Self as >::H, <Self as >::H)`

  Computes `(self / n, self % n))`

#### Provided Methods

- `fn checked_narrowing_div_rem(self, n: <Self as >::H) -> Option<(<Self as >::H, <Self as >::H)>`

  Returns `Some((self / n, self % n))` when `self.hi() < n`.

#### Implementors

- [`u256`](big/index.md#u256)
- `u128`
- `u16`
- `u32`
- `u64`

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

### `linear_mul_reduction`

```rust
fn linear_mul_reduction<U>(x: U, e: u32, y: U) -> U
where
    U: HInt + Int<Unsigned = U>,
    <U as >::D: NarrowingDiv
```

Compute the remainder `(x << e) % y` with unbounded integers.
Requires `x < 2y` and `y.leading_zeros() >= 2`

### `cold_path`

```rust
fn cold_path()
```

Hint to the compiler that the current path is cold.

## Type Aliases

### `IntTy<F>`

```rust
type IntTy<F> = <F as Float>::Int;
```

Access the associated `Int` type from a float (helper to avoid ambiguous associated types).

