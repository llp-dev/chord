# Crate `num_traits`

Numeric traits for generic mathematics

## Compatibility

The `num-traits` crate is tested for rustc 1.60 and greater.

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`bounds`](#bounds)
  - [`cast`](#cast)
  - [`float`](#float)
  - [`identities`](#identities)
  - [`int`](#int)
  - [`ops`](#ops)
  - [`pow`](#pow)
  - [`real`](#real)
  - [`sign`](#sign)
- [Structs](#structs)
  - [`ParseFloatError`](#parsefloaterror)
- [Enums](#enums)
  - [`FloatErrorKind`](#floaterrorkind)
- [Traits](#traits)
  - [`Bounded`](#bounded)
  - [`Float`](#float)
  - [`FloatConst`](#floatconst)
  - [`AsPrimitive`](#asprimitive)
  - [`FromPrimitive`](#fromprimitive)
  - [`NumCast`](#numcast)
  - [`ToPrimitive`](#toprimitive)
  - [`ConstOne`](#constone)
  - [`ConstZero`](#constzero)
  - [`One`](#one)
  - [`Zero`](#zero)
  - [`PrimInt`](#primint)
  - [`FromBytes`](#frombytes)
  - [`ToBytes`](#tobytes)
  - [`CheckedAdd`](#checkedadd)
  - [`CheckedDiv`](#checkeddiv)
  - [`CheckedMul`](#checkedmul)
  - [`CheckedNeg`](#checkedneg)
  - [`CheckedRem`](#checkedrem)
  - [`CheckedShl`](#checkedshl)
  - [`CheckedShr`](#checkedshr)
  - [`CheckedSub`](#checkedsub)
  - [`CheckedEuclid`](#checkedeuclid)
  - [`Euclid`](#euclid)
  - [`Inv`](#inv)
  - [`MulAdd`](#muladd)
  - [`MulAddAssign`](#muladdassign)
  - [`Saturating`](#saturating)
  - [`SaturatingAdd`](#saturatingadd)
  - [`SaturatingMul`](#saturatingmul)
  - [`SaturatingSub`](#saturatingsub)
  - [`WrappingAdd`](#wrappingadd)
  - [`WrappingMul`](#wrappingmul)
  - [`WrappingNeg`](#wrappingneg)
  - [`WrappingShl`](#wrappingshl)
  - [`WrappingShr`](#wrappingshr)
  - [`WrappingSub`](#wrappingsub)
  - [`Pow`](#pow)
  - [`Signed`](#signed)
  - [`Unsigned`](#unsigned)
  - [`Num`](#num)
  - [`NumOps`](#numops)
  - [`NumRef`](#numref)
  - [`RefNum`](#refnum)
  - [`NumAssignOps`](#numassignops)
  - [`NumAssign`](#numassign)
  - [`NumAssignRef`](#numassignref)
- [Functions](#functions)
  - [`cast`](#cast)
  - [`one`](#one)
  - [`zero`](#zero)
  - [`checked_pow`](#checked-pow)
  - [`pow`](#pow)
  - [`abs`](#abs)
  - [`abs_sub`](#abs-sub)
  - [`signum`](#signum)
  - [`str_to_ascii_lower_eq_str`](#str-to-ascii-lower-eq-str)
  - [`clamp`](#clamp)
  - [`clamp_min`](#clamp-min)
  - [`clamp_max`](#clamp-max)
- [Macros](#macros)
  - [`int_trait_impl!`](#int-trait-impl)
  - [`float_trait_impl!`](#float-trait-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`bounds`](#bounds) | mod |  |
| [`cast`](#cast) | mod |  |
| [`float`](#float) | mod |  |
| [`identities`](#identities) | mod |  |
| [`int`](#int) | mod |  |
| [`ops`](#ops) | mod |  |
| [`pow`](#pow) | mod |  |
| [`real`](#real) | mod |  |
| [`sign`](#sign) | mod |  |
| [`ParseFloatError`](#parsefloaterror) | struct |  |
| [`FloatErrorKind`](#floaterrorkind) | enum |  |
| [`Bounded`](#bounded) | trait |  |
| [`Float`](#float) | trait |  |
| [`FloatConst`](#floatconst) | trait |  |
| [`AsPrimitive`](#asprimitive) | trait |  |
| [`FromPrimitive`](#fromprimitive) | trait |  |
| [`NumCast`](#numcast) | trait |  |
| [`ToPrimitive`](#toprimitive) | trait |  |
| [`ConstOne`](#constone) | trait |  |
| [`ConstZero`](#constzero) | trait |  |
| [`One`](#one) | trait |  |
| [`Zero`](#zero) | trait |  |
| [`PrimInt`](#primint) | trait |  |
| [`FromBytes`](#frombytes) | trait |  |
| [`ToBytes`](#tobytes) | trait |  |
| [`CheckedAdd`](#checkedadd) | trait |  |
| [`CheckedDiv`](#checkeddiv) | trait |  |
| [`CheckedMul`](#checkedmul) | trait |  |
| [`CheckedNeg`](#checkedneg) | trait |  |
| [`CheckedRem`](#checkedrem) | trait |  |
| [`CheckedShl`](#checkedshl) | trait |  |
| [`CheckedShr`](#checkedshr) | trait |  |
| [`CheckedSub`](#checkedsub) | trait |  |
| [`CheckedEuclid`](#checkedeuclid) | trait |  |
| [`Euclid`](#euclid) | trait |  |
| [`Inv`](#inv) | trait |  |
| [`MulAdd`](#muladd) | trait |  |
| [`MulAddAssign`](#muladdassign) | trait |  |
| [`Saturating`](#saturating) | trait |  |
| [`SaturatingAdd`](#saturatingadd) | trait |  |
| [`SaturatingMul`](#saturatingmul) | trait |  |
| [`SaturatingSub`](#saturatingsub) | trait |  |
| [`WrappingAdd`](#wrappingadd) | trait |  |
| [`WrappingMul`](#wrappingmul) | trait |  |
| [`WrappingNeg`](#wrappingneg) | trait |  |
| [`WrappingShl`](#wrappingshl) | trait |  |
| [`WrappingShr`](#wrappingshr) | trait |  |
| [`WrappingSub`](#wrappingsub) | trait |  |
| [`Pow`](#pow) | trait |  |
| [`Signed`](#signed) | trait |  |
| [`Unsigned`](#unsigned) | trait |  |
| [`Num`](#num) | trait | The base trait for numeric types, covering `0` and `1` values, comparisons, basic numeric operations, and string conversion. |
| [`NumOps`](#numops) | trait | Generic trait for types implementing basic numeric operations |
| [`NumRef`](#numref) | trait | The trait for `Num` types which also implement numeric operations taking the second operand by reference. |
| [`RefNum`](#refnum) | trait | The trait for `Num` references which implement numeric operations, taking the second operand either by value or by reference. |
| [`NumAssignOps`](#numassignops) | trait | Generic trait for types implementing numeric assignment operators (like `+=`). |
| [`NumAssign`](#numassign) | trait | The trait for `Num` types which also implement assignment operators. |
| [`NumAssignRef`](#numassignref) | trait | The trait for `NumAssign` types which also implement assignment operations taking the second operand by reference. |
| [`cast`](#cast) | fn |  |
| [`one`](#one) | fn |  |
| [`zero`](#zero) | fn |  |
| [`checked_pow`](#checked-pow) | fn |  |
| [`pow`](#pow) | fn |  |
| [`abs`](#abs) | fn |  |
| [`abs_sub`](#abs-sub) | fn |  |
| [`signum`](#signum) | fn |  |
| [`str_to_ascii_lower_eq_str`](#str-to-ascii-lower-eq-str) | fn |  |
| [`clamp`](#clamp) | fn | A value bounded by a minimum and a maximum |
| [`clamp_min`](#clamp-min) | fn | A value bounded by a minimum value |
| [`clamp_max`](#clamp-max) | fn | A value bounded by a maximum value |
| [`int_trait_impl!`](#int-trait-impl) | macro |  |
| [`float_trait_impl!`](#float-trait-impl) | macro |  |

## Modules

- [`macros`](macros/index.md)
- [`bounds`](bounds/index.md)
- [`cast`](cast/index.md)
- [`float`](float/index.md)
- [`identities`](identities/index.md)
- [`int`](int/index.md)
- [`ops`](ops/index.md)
- [`pow`](pow/index.md)
- [`real`](real/index.md)
- [`sign`](sign/index.md)

## Structs

### `ParseFloatError`

```rust
struct ParseFloatError {
    pub kind: FloatErrorKind,
}
```

#### Trait Implementations

##### `impl Debug for ParseFloatError`

- <span id="parsefloaterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseFloatError`

- <span id="parsefloaterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ParseFloatError`

- <span id="parsefloaterror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `FloatErrorKind`

```rust
enum FloatErrorKind {
    Empty,
    Invalid,
}
```

#### Trait Implementations

##### `impl Debug for FloatErrorKind`

- <span id="floaterrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Bounded`

```rust
trait Bounded { ... }
```

Numbers which have upper and lower bounds

#### Required Methods

- `fn min_value() -> Self`

  Returns the smallest finite number this type can represent

- `fn max_value() -> Self`

  Returns the largest finite number this type can represent

#### Implementors

- `()`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(J, K, L, M, N, O, P, Q, R, S, T)`
- `(K, L, M, N, O, P, Q, R, S, T)`
- `(L, M, N, O, P, Q, R, S, T)`
- `(M, N, O, P, Q, R, S, T)`
- `(N, O, P, Q, R, S, T)`
- `(O, P, Q, R, S, T)`
- `(P, Q, R, S, T)`
- `(Q, R, S, T)`
- `(R, S, T)`
- `(S, T)`
- `(T)`
- `core::num::Wrapping<T>`
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

### `Float`

```rust
trait Float: Num + Copy + NumCast + PartialOrd + Neg<Output = Self> { ... }
```

Generic trait for floating point numbers

This trait is only available with the `std` feature, or with the `libm` feature otherwise.

#### Required Methods

- `fn nan() -> Self`

  Returns the `NaN` value.

- `fn infinity() -> Self`

  Returns the infinite value.

- `fn neg_infinity() -> Self`

  Returns the negative infinite value.

- `fn neg_zero() -> Self`

  Returns `-0.0`.

- `fn min_value() -> Self`

  Returns the smallest finite value that this type can represent.

- `fn min_positive_value() -> Self`

  Returns the smallest positive, normalized value that this type can represent.

- `fn max_value() -> Self`

  Returns the largest finite value that this type can represent.

- `fn is_nan(self) -> bool`

  Returns `true` if this value is `NaN` and false otherwise.

- `fn is_infinite(self) -> bool`

  Returns `true` if this value is positive infinity or negative infinity and

- `fn is_finite(self) -> bool`

  Returns `true` if this number is neither infinite nor `NaN`.

- `fn is_normal(self) -> bool`

  Returns `true` if the number is neither zero, infinite,

- `fn classify(self) -> FpCategory`

  Returns the floating point category of the number. If only one property

- `fn floor(self) -> Self`

  Returns the largest integer less than or equal to a number.

- `fn ceil(self) -> Self`

  Returns the smallest integer greater than or equal to a number.

- `fn round(self) -> Self`

  Returns the nearest integer to a number. Round half-way cases away from

- `fn trunc(self) -> Self`

  Return the integer part of a number.

- `fn fract(self) -> Self`

  Returns the fractional part of a number.

- `fn abs(self) -> Self`

  Computes the absolute value of `self`. Returns `Float::nan()` if the

- `fn signum(self) -> Self`

  Returns a number that represents the sign of `self`.

- `fn is_sign_positive(self) -> bool`

  Returns `true` if `self` is positive, including `+0.0`,

- `fn is_sign_negative(self) -> bool`

  Returns `true` if `self` is negative, including `-0.0`,

- `fn mul_add(self, a: Self, b: Self) -> Self`

  Fused multiply-add. Computes `(self * a) + b` with only one rounding

- `fn recip(self) -> Self`

  Take the reciprocal (inverse) of a number, `1/x`.

- `fn powi(self, n: i32) -> Self`

  Raise a number to an integer power.

- `fn powf(self, n: Self) -> Self`

  Raise a number to a floating point power.

- `fn sqrt(self) -> Self`

  Take the square root of a number.

- `fn exp(self) -> Self`

  Returns `e^(self)`, (the exponential function).

- `fn exp2(self) -> Self`

  Returns `2^(self)`.

- `fn ln(self) -> Self`

  Returns the natural logarithm of the number.

- `fn log(self, base: Self) -> Self`

  Returns the logarithm of the number with respect to an arbitrary base.

- `fn log2(self) -> Self`

  Returns the base 2 logarithm of the number.

- `fn log10(self) -> Self`

  Returns the base 10 logarithm of the number.

- `fn max(self, other: Self) -> Self`

  Returns the maximum of the two numbers.

- `fn min(self, other: Self) -> Self`

  Returns the minimum of the two numbers.

- `fn abs_sub(self, other: Self) -> Self`

  The positive difference of two numbers.

- `fn cbrt(self) -> Self`

  Take the cubic root of a number.

- `fn hypot(self, other: Self) -> Self`

  Calculate the length of the hypotenuse of a right-angle triangle given

- `fn sin(self) -> Self`

  Computes the sine of a number (in radians).

- `fn cos(self) -> Self`

  Computes the cosine of a number (in radians).

- `fn tan(self) -> Self`

  Computes the tangent of a number (in radians).

- `fn asin(self) -> Self`

  Computes the arcsine of a number. Return value is in radians in

- `fn acos(self) -> Self`

  Computes the arccosine of a number. Return value is in radians in

- `fn atan(self) -> Self`

  Computes the arctangent of a number. Return value is in radians in the

- `fn atan2(self, other: Self) -> Self`

  Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).

- `fn sin_cos(self) -> (Self, Self)`

  Simultaneously computes the sine and cosine of the number, `x`. Returns

- `fn exp_m1(self) -> Self`

  Returns `e^(self) - 1` in a way that is accurate even if the

- `fn ln_1p(self) -> Self`

  Returns `ln(1+n)` (natural logarithm) more accurately than if

- `fn sinh(self) -> Self`

  Hyperbolic sine function.

- `fn cosh(self) -> Self`

  Hyperbolic cosine function.

- `fn tanh(self) -> Self`

  Hyperbolic tangent function.

- `fn asinh(self) -> Self`

  Inverse hyperbolic sine function.

- `fn acosh(self) -> Self`

  Inverse hyperbolic cosine function.

- `fn atanh(self) -> Self`

  Inverse hyperbolic tangent function.

- `fn integer_decode(self) -> (u64, i16, i8)`

  Returns the mantissa, base 2 exponent, and sign as integers, respectively.

#### Provided Methods

- `fn epsilon() -> Self`

  Returns epsilon, a small positive value.

- `fn is_subnormal(self) -> bool`

  Returns `true` if the number is [subnormal].

- `fn to_degrees(self) -> Self`

  Converts radians to degrees.

- `fn to_radians(self) -> Self`

  Converts degrees to radians.

- `fn clamp(self, min: Self, max: Self) -> Self`

  Clamps a value between a min and max.

- `fn copysign(self, sign: Self) -> Self`

  Returns a number composed of the magnitude of `self` and the sign of

#### Implementors

- `f32`
- `f64`

### `FloatConst`

```rust
trait FloatConst { ... }
```

#### Required Methods

- `fn E() -> Self`

  Return Euler’s number.

- `fn FRAC_1_PI() -> Self`

  Return `1.0 / π`.

- `fn FRAC_1_SQRT_2() -> Self`

  Return `1.0 / sqrt(2.0)`.

- `fn FRAC_2_PI() -> Self`

  Return `2.0 / π`.

- `fn FRAC_2_SQRT_PI() -> Self`

  Return `2.0 / sqrt(π)`.

- `fn FRAC_PI_2() -> Self`

  Return `π / 2.0`.

- `fn FRAC_PI_3() -> Self`

  Return `π / 3.0`.

- `fn FRAC_PI_4() -> Self`

  Return `π / 4.0`.

- `fn FRAC_PI_6() -> Self`

  Return `π / 6.0`.

- `fn FRAC_PI_8() -> Self`

  Return `π / 8.0`.

- `fn LN_10() -> Self`

  Return `ln(10.0)`.

- `fn LN_2() -> Self`

  Return `ln(2.0)`.

- `fn LOG10_E() -> Self`

  Return `log10(e)`.

- `fn LOG2_E() -> Self`

  Return `log2(e)`.

- `fn PI() -> Self`

  Return Archimedes’ constant `π`.

- `fn SQRT_2() -> Self`

  Return `sqrt(2.0)`.

#### Provided Methods

- `fn TAU() -> Self`

  Return the full circle constant `τ`.

- `fn LOG10_2() -> Self`

  Return `log10(2.0)`.

- `fn LOG2_10() -> Self`

  Return `log2(10.0)`.

#### Implementors

- `f32`
- `f64`

### `AsPrimitive<T>`

```rust
trait AsPrimitive<T>: 'static + Copy
where
    T: 'static + Copy { ... }
```

A generic interface for casting between machine scalars with the
`as` operator, which admits narrowing and precision loss.
Implementers of this trait `AsPrimitive` should behave like a primitive
numeric type (e.g. a newtype around another primitive), and the
intended conversion must never fail.

# Examples

```rust
use num_traits::AsPrimitive;
let three: i32 = (3.14159265f32).as_();
assert_eq!(three, 3);
```

# Safety

**In Rust versions before 1.45.0**, some uses of the `as` operator were not entirely safe.
In particular, it was undefined behavior if
a truncated floating point value could not fit in the target integer
type ([#10184](https://github.com/rust-lang/rust/issues/10184)).

```ignore
use num_traits::AsPrimitive;
let x: u8 = (1.04E+17).as_(); // UB
```


#### Required Methods

- `fn as_(self) -> T`

  Convert a value to another, using the `as` operator.

#### Implementors

- `bool`
- `char`
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

### `FromPrimitive`

```rust
trait FromPrimitive: Sized { ... }
```

A generic trait for converting a number to a value.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

#### Required Methods

- `fn from_i64(n: i64) -> Option<Self>`

  Converts an `i64` to return an optional value of this type. If the

- `fn from_u64(n: u64) -> Option<Self>`

  Converts an `u64` to return an optional value of this type. If the

#### Provided Methods

- `fn from_isize(n: isize) -> Option<Self>`

  Converts an `isize` to return an optional value of this type. If the

- `fn from_i8(n: i8) -> Option<Self>`

  Converts an `i8` to return an optional value of this type. If the

- `fn from_i16(n: i16) -> Option<Self>`

  Converts an `i16` to return an optional value of this type. If the

- `fn from_i32(n: i32) -> Option<Self>`

  Converts an `i32` to return an optional value of this type. If the

- `fn from_i128(n: i128) -> Option<Self>`

  Converts an `i128` to return an optional value of this type. If the

- `fn from_usize(n: usize) -> Option<Self>`

  Converts a `usize` to return an optional value of this type. If the

- `fn from_u8(n: u8) -> Option<Self>`

  Converts an `u8` to return an optional value of this type. If the

- `fn from_u16(n: u16) -> Option<Self>`

  Converts an `u16` to return an optional value of this type. If the

- `fn from_u32(n: u32) -> Option<Self>`

  Converts an `u32` to return an optional value of this type. If the

- `fn from_u128(n: u128) -> Option<Self>`

  Converts an `u128` to return an optional value of this type. If the

- `fn from_f32(n: f32) -> Option<Self>`

  Converts a `f32` to return an optional value of this type. If the

- `fn from_f64(n: f64) -> Option<Self>`

  Converts a `f64` to return an optional value of this type. If the

#### Implementors

- `core::num::Wrapping<T>`
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

### `NumCast`

```rust
trait NumCast: Sized + ToPrimitive { ... }
```

An interface for casting between machine scalars.

#### Required Methods

- `fn from<T: ToPrimitive>(n: T) -> Option<Self>`

  Creates a number from another value that can be converted into

#### Implementors

- `core::num::Wrapping<T>`
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

### `ToPrimitive`

```rust
trait ToPrimitive { ... }
```

A generic trait for converting a value to a number.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

#### Required Methods

- `fn to_i64(&self) -> Option<i64>`

  Converts the value of `self` to an `i64`. If the value cannot be

- `fn to_u64(&self) -> Option<u64>`

  Converts the value of `self` to a `u64`. If the value cannot be

#### Provided Methods

- `fn to_isize(&self) -> Option<isize>`

  Converts the value of `self` to an `isize`. If the value cannot be

- `fn to_i8(&self) -> Option<i8>`

  Converts the value of `self` to an `i8`. If the value cannot be

- `fn to_i16(&self) -> Option<i16>`

  Converts the value of `self` to an `i16`. If the value cannot be

- `fn to_i32(&self) -> Option<i32>`

  Converts the value of `self` to an `i32`. If the value cannot be

- `fn to_i128(&self) -> Option<i128>`

  Converts the value of `self` to an `i128`. If the value cannot be

- `fn to_usize(&self) -> Option<usize>`

  Converts the value of `self` to a `usize`. If the value cannot be

- `fn to_u8(&self) -> Option<u8>`

  Converts the value of `self` to a `u8`. If the value cannot be

- `fn to_u16(&self) -> Option<u16>`

  Converts the value of `self` to a `u16`. If the value cannot be

- `fn to_u32(&self) -> Option<u32>`

  Converts the value of `self` to a `u32`. If the value cannot be

- `fn to_u128(&self) -> Option<u128>`

  Converts the value of `self` to a `u128`. If the value cannot be

- `fn to_f32(&self) -> Option<f32>`

  Converts the value of `self` to an `f32`. Overflows may map to positive

- `fn to_f64(&self) -> Option<f64>`

  Converts the value of `self` to an `f64`. Overflows may map to positive

#### Implementors

- `core::num::Wrapping<T>`
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

### `ConstOne`

```rust
trait ConstOne: One { ... }
```

Defines an associated constant representing the multiplicative identity
element for `Self`.

#### Associated Constants

- `const ONE: Self`

#### Implementors

- `core::num::Wrapping<T>`
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

### `ConstZero`

```rust
trait ConstZero: Zero { ... }
```

Defines an associated constant representing the additive identity element
for `Self`.

#### Associated Constants

- `const ZERO: Self`

#### Implementors

- `core::num::Wrapping<T>`
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

### `One`

```rust
trait One: Sized + Mul<Self, Output = Self> { ... }
```

Defines a multiplicative identity element for `Self`.

# Laws

```text
a * 1 = a       ∀ a ∈ Self
1 * a = a       ∀ a ∈ Self
```

#### Required Methods

- `fn one() -> Self`

  Returns the multiplicative identity element of `Self`, `1`.

#### Provided Methods

- `fn set_one(&mut self)`

  Sets `self` to the multiplicative identity element of `Self`, `1`.

- `fn is_one(&self) -> bool`

  Returns `true` if `self` is equal to the multiplicative identity.

#### Implementors

- `core::num::Wrapping<T>`
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

### `Zero`

```rust
trait Zero: Sized + Add<Self, Output = Self> { ... }
```

Defines an additive identity element for `Self`.

# Laws

```text
a + 0 = a       ∀ a ∈ Self
0 + a = a       ∀ a ∈ Self
```

#### Required Methods

- `fn zero() -> Self`

  Returns the additive identity element of `Self`, `0`.

- `fn is_zero(&self) -> bool`

  Returns `true` if `self` is equal to the additive identity.

#### Provided Methods

- `fn set_zero(&mut self)`

  Sets `self` to the additive identity element of `Self`, `0`.

#### Implementors

- `core::num::Wrapping<T>`
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

### `CheckedAdd`

```rust
trait CheckedAdd: Sized + Add<Self, Output = Self> { ... }
```

Performs addition that returns `None` instead of wrapping around on
overflow.

#### Required Methods

- `fn checked_add(&self, v: &Self) -> Option<Self>`

  Adds two numbers, checking for overflow. If overflow happens, `None` is

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

### `CheckedDiv`

```rust
trait CheckedDiv: Sized + Div<Self, Output = Self> { ... }
```

Performs division that returns `None` instead of panicking on division by zero and instead of
wrapping around on underflow and overflow.

#### Required Methods

- `fn checked_div(&self, v: &Self) -> Option<Self>`

  Divides two numbers, checking for underflow, overflow and division by

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

### `CheckedMul`

```rust
trait CheckedMul: Sized + Mul<Self, Output = Self> { ... }
```

Performs multiplication that returns `None` instead of wrapping around on underflow or
overflow.

#### Required Methods

- `fn checked_mul(&self, v: &Self) -> Option<Self>`

  Multiplies two numbers, checking for underflow or overflow. If underflow

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

### `CheckedNeg`

```rust
trait CheckedNeg: Sized { ... }
```

Performs negation that returns `None` if the result can't be represented.

#### Required Methods

- `fn checked_neg(&self) -> Option<Self>`

  Negates a number, returning `None` for results that can't be represented, like signed `MIN`

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

### `CheckedRem`

```rust
trait CheckedRem: Sized + Rem<Self, Output = Self> { ... }
```

Performs an integral remainder that returns `None` instead of panicking on division by zero and
instead of wrapping around on underflow and overflow.

#### Required Methods

- `fn checked_rem(&self, v: &Self) -> Option<Self>`

  Finds the remainder of dividing two numbers, checking for underflow, overflow and division

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

### `CheckedShl`

```rust
trait CheckedShl: Sized + Shl<u32, Output = Self> { ... }
```

Performs a left shift that returns `None` on shifts larger than
or equal to the type width.

#### Required Methods

- `fn checked_shl(&self, rhs: u32) -> Option<Self>`

  Checked shift left. Computes `self << rhs`, returning `None`

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

### `CheckedShr`

```rust
trait CheckedShr: Sized + Shr<u32, Output = Self> { ... }
```

Performs a right shift that returns `None` on shifts larger than
or equal to the type width.

#### Required Methods

- `fn checked_shr(&self, rhs: u32) -> Option<Self>`

  Checked shift right. Computes `self >> rhs`, returning `None`

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

### `CheckedSub`

```rust
trait CheckedSub: Sized + Sub<Self, Output = Self> { ... }
```

Performs subtraction that returns `None` instead of wrapping around on underflow.

#### Required Methods

- `fn checked_sub(&self, v: &Self) -> Option<Self>`

  Subtracts two numbers, checking for underflow. If underflow happens,

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

### `CheckedEuclid`

```rust
trait CheckedEuclid: Euclid { ... }
```

#### Required Methods

- `fn checked_div_euclid(&self, v: &Self) -> Option<Self>`

  Performs euclid division that returns `None` instead of panicking on division by zero

- `fn checked_rem_euclid(&self, v: &Self) -> Option<Self>`

  Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and

#### Provided Methods

- `fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)>`

  Returns both the quotient and remainder from checked Euclidean division.

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

### `Euclid`

```rust
trait Euclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> { ... }
```

#### Required Methods

- `fn div_euclid(&self, v: &Self) -> Self`

  Calculates Euclidean division, the matching method for `rem_euclid`.

- `fn rem_euclid(&self, v: &Self) -> Self`

  Calculates the least nonnegative remainder of `self (mod v)`.

#### Provided Methods

- `fn div_rem_euclid(&self, v: &Self) -> (Self, Self)`

  Returns both the quotient and remainder from Euclidean division.

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

### `Inv`

```rust
trait Inv { ... }
```

Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.

#### Associated Types

- `type Output`

#### Required Methods

- `fn inv(self) -> <Self as >::Output`

  Returns the multiplicative inverse of `self`.

#### Implementors

- `&'a f32`
- `&'a f64`
- `f32`
- `f64`

### `MulAdd<A, B>`

```rust
trait MulAdd<A, B> { ... }
```

Fused multiply-add. Computes `(self * a) + b` with only one rounding
error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` can be more performant than an unfused multiply-add if
the target architecture has a dedicated `fma` CPU instruction.

Note that `A` and `B` are `Self` by default, but this is not mandatory.

# Example

```rust
use std::f32;

let m = 10.0_f32;
let x = 4.0_f32;
let b = 60.0_f32;

// 100.0
let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();

assert!(abs_difference <= 100.0 * f32::EPSILON);
```

#### Associated Types

- `type Output`

#### Required Methods

- `fn mul_add(self, a: A, b: B) -> <Self as >::Output`

  Performs the fused multiply-add operation `(self * a) + b`

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

### `MulAddAssign<A, B>`

```rust
trait MulAddAssign<A, B> { ... }
```

The fused multiply-add assignment operation `*self = (*self * a) + b`

#### Required Methods

- `fn mul_add_assign(&mut self, a: A, b: B)`

  Performs the fused multiply-add assignment operation `*self = (*self * a) + b`

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

### `Saturating`

```rust
trait Saturating { ... }
```

Saturating math operations. Deprecated, use `SaturatingAdd`, `SaturatingSub` and
`SaturatingMul` instead.

#### Required Methods

- `fn saturating_add(self, v: Self) -> Self`

  Saturating addition operator.

- `fn saturating_sub(self, v: Self) -> Self`

  Saturating subtraction operator.

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

### `SaturatingAdd`

```rust
trait SaturatingAdd: Sized + Add<Self, Output = Self> { ... }
```

Performs addition that saturates at the numeric bounds instead of overflowing.

#### Required Methods

- `fn saturating_add(&self, v: &Self) -> Self`

  Saturating addition. Computes `self + other`, saturating at the relevant high or low boundary of

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

### `SaturatingMul`

```rust
trait SaturatingMul: Sized + Mul<Self, Output = Self> { ... }
```

Performs multiplication that saturates at the numeric bounds instead of overflowing.

#### Required Methods

- `fn saturating_mul(&self, v: &Self) -> Self`

  Saturating multiplication. Computes `self * other`, saturating at the relevant high or low boundary of

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

### `SaturatingSub`

```rust
trait SaturatingSub: Sized + Sub<Self, Output = Self> { ... }
```

Performs subtraction that saturates at the numeric bounds instead of overflowing.

#### Required Methods

- `fn saturating_sub(&self, v: &Self) -> Self`

  Saturating subtraction. Computes `self - other`, saturating at the relevant high or low boundary of

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

### `WrappingAdd`

```rust
trait WrappingAdd: Sized + Add<Self, Output = Self> { ... }
```

Performs addition that wraps around on overflow.

#### Required Methods

- `fn wrapping_add(&self, v: &Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingMul`

```rust
trait WrappingMul: Sized + Mul<Self, Output = Self> { ... }
```

Performs multiplication that wraps around on overflow.

#### Required Methods

- `fn wrapping_mul(&self, v: &Self) -> Self`

  Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingNeg`

```rust
trait WrappingNeg: Sized { ... }
```

Performs a negation that does not panic.

#### Required Methods

- `fn wrapping_neg(&self) -> Self`

  Wrapping (modular) negation. Computes `-self`,

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingShl`

```rust
trait WrappingShl: Sized + Shl<usize, Output = Self> { ... }
```

Performs a left shift that does not panic.

#### Required Methods

- `fn wrapping_shl(&self, rhs: u32) -> Self`

  Panic-free bitwise shift-left; yields `self << mask(rhs)`,

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingShr`

```rust
trait WrappingShr: Sized + Shr<usize, Output = Self> { ... }
```

Performs a right shift that does not panic.

#### Required Methods

- `fn wrapping_shr(&self, rhs: u32) -> Self`

  Panic-free bitwise shift-right; yields `self >> mask(rhs)`,

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingSub`

```rust
trait WrappingSub: Sized + Sub<Self, Output = Self> { ... }
```

Performs subtraction that wraps around on overflow.

#### Required Methods

- `fn wrapping_sub(&self, v: &Self) -> Self`

  Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary

#### Implementors

- `core::num::Wrapping<T>`
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

### `Pow<RHS>`

```rust
trait Pow<RHS> { ... }
```

Binary operator for raising a value to a power.

#### Associated Types

- `type Output`

#### Required Methods

- `fn pow(self, rhs: RHS) -> <Self as >::Output`

  Returns `self` to the power `rhs`.

#### Implementors

- `&'a core::num::Wrapping<i128>`
- `&'a core::num::Wrapping<i16>`
- `&'a core::num::Wrapping<i32>`
- `&'a core::num::Wrapping<i64>`
- `&'a core::num::Wrapping<i8>`
- `&'a core::num::Wrapping<isize>`
- `&'a core::num::Wrapping<u128>`
- `&'a core::num::Wrapping<u16>`
- `&'a core::num::Wrapping<u32>`
- `&'a core::num::Wrapping<u64>`
- `&'a core::num::Wrapping<u8>`
- `&'a core::num::Wrapping<usize>`
- `&'a f32`
- `&'a f64`
- `&'a i128`
- `&'a i16`
- `&'a i32`
- `&'a i64`
- `&'a i8`
- `&'a isize`
- `&'a u128`
- `&'a u16`
- `&'a u32`
- `&'a u64`
- `&'a u8`
- `&'a usize`
- `&'b core::num::Wrapping<i128>`
- `&'b core::num::Wrapping<i16>`
- `&'b core::num::Wrapping<i32>`
- `&'b core::num::Wrapping<i64>`
- `&'b core::num::Wrapping<i8>`
- `&'b core::num::Wrapping<isize>`
- `&'b core::num::Wrapping<u128>`
- `&'b core::num::Wrapping<u16>`
- `&'b core::num::Wrapping<u32>`
- `&'b core::num::Wrapping<u64>`
- `&'b core::num::Wrapping<u8>`
- `&'b core::num::Wrapping<usize>`
- `&'b f32`
- `&'b f64`
- `&'b i128`
- `&'b i16`
- `&'b i32`
- `&'b i64`
- `&'b i8`
- `&'b isize`
- `&'b u128`
- `&'b u16`
- `&'b u32`
- `&'b u64`
- `&'b u8`
- `&'b usize`
- `core::num::Wrapping<i128>`
- `core::num::Wrapping<i16>`
- `core::num::Wrapping<i32>`
- `core::num::Wrapping<i64>`
- `core::num::Wrapping<i8>`
- `core::num::Wrapping<isize>`
- `core::num::Wrapping<u128>`
- `core::num::Wrapping<u16>`
- `core::num::Wrapping<u32>`
- `core::num::Wrapping<u64>`
- `core::num::Wrapping<u8>`
- `core::num::Wrapping<usize>`
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

### `Signed`

```rust
trait Signed: Sized + Num + Neg<Output = Self> { ... }
```

Useful functions for signed numbers (i.e. numbers that can be negative).

#### Required Methods

- `fn abs(&self) -> Self`

  Computes the absolute value.

- `fn abs_sub(&self, other: &Self) -> Self`

  The positive difference of two numbers.

- `fn signum(&self) -> Self`

  Returns the sign of the number.

- `fn is_positive(&self) -> bool`

  Returns true if the number is positive and false if the number is zero or negative.

- `fn is_negative(&self) -> bool`

  Returns true if the number is negative and false if the number is zero or positive.

#### Implementors

- `core::num::Wrapping<T>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`

### `Unsigned`

```rust
trait Unsigned: Num { ... }
```

A trait for values which cannot be negative

#### Implementors

- `core::num::Wrapping<T>`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `Num`

```rust
trait Num: PartialEq + Zero + One + NumOps { ... }
```

The base trait for numeric types, covering `0` and `1` values,
comparisons, basic numeric operations, and string conversion.

#### Associated Types

- `type FromStrRadixErr`

#### Required Methods

- `fn from_str_radix(str: &str, radix: u32) -> Result<Self, <Self as >::FromStrRadixErr>`

  Convert from a string and radix (typically `2..=36`).

#### Implementors

- `core::num::Wrapping<T>`
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

### `NumOps<Rhs, Output>`

```rust
trait NumOps<Rhs, Output>: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> { ... }
```

Generic trait for types implementing basic numeric operations

This is automatically implemented for types which implement the operators.

#### Implementors

- `T`

### `NumRef`

```rust
trait NumRef: Num + NumOps<&'r Self> { ... }
```

The trait for `Num` types which also implement numeric operations taking
the second operand by reference.

This is automatically implemented for types which implement the operators.

#### Implementors

- `T`

### `RefNum<Base>`

```rust
trait RefNum<Base>: NumOps<Base, Base> + NumOps<&'r Base, Base> { ... }
```

The trait for `Num` references which implement numeric operations, taking the
second operand either by value or by reference.

This is automatically implemented for all types which implement the operators. It covers
every type implementing the operations though, regardless of it being a reference or
related to `Num`.

#### Implementors

- `T`

### `NumAssignOps<Rhs>`

```rust
trait NumAssignOps<Rhs>: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs> { ... }
```

Generic trait for types implementing numeric assignment operators (like `+=`).

This is automatically implemented for types which implement the operators.

#### Implementors

- `T`

### `NumAssign`

```rust
trait NumAssign: Num + NumAssignOps { ... }
```

The trait for `Num` types which also implement assignment operators.

This is automatically implemented for types which implement the operators.

#### Implementors

- `T`

### `NumAssignRef`

```rust
trait NumAssignRef: NumAssign + NumAssignOps<&'r Self> { ... }
```

The trait for `NumAssign` types which also implement assignment operations
taking the second operand by reference.

This is automatically implemented for types which implement the operators.

#### Implementors

- `T`

## Functions

### `cast`

```rust
fn cast<T: NumCast, U: NumCast>(n: T) -> Option<U>
```

Cast from one machine scalar to another.

# Examples

```rust
use num_traits as num;
let twenty: f32 = num::cast(0x14).unwrap();
assert_eq!(twenty, 20f32);
```


### `one`

```rust
fn one<T: One>() -> T
```

Returns the multiplicative identity, `1`.

### `zero`

```rust
fn zero<T: Zero>() -> T
```

Returns the additive identity, `0`.

### `checked_pow`

```rust
fn checked_pow<T: Clone + One + CheckedMul>(base: T, exp: usize) -> Option<T>
```

Raises a value to the power of exp, returning `None` if an overflow occurred.

Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined.

Otherwise same as the `pow` function.

# Example

```rust
use num_traits::checked_pow;

assert_eq!(checked_pow(2i8, 4), Some(16));
assert_eq!(checked_pow(7i8, 8), None);
assert_eq!(checked_pow(7u32, 8), Some(5_764_801));
assert_eq!(checked_pow(0u32, 0), Some(1)); // Be aware if this case affect you
```

### `pow`

```rust
fn pow<T: Clone + One + Mul<T, Output = T>>(base: T, exp: usize) -> T
```

Raises a value to the power of exp, using exponentiation by squaring.

Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.

# Example

```rust
use num_traits::pow;

assert_eq!(pow(2i8, 4), 16);
assert_eq!(pow(6u8, 3), 216);
assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
```

### `abs`

```rust
fn abs<T: Signed>(value: T) -> T
```

Computes the absolute value.

For `f32` and `f64`, `NaN` will be returned if the number is `NaN`

For signed integers, `::MIN` will be returned if the number is `::MIN`.

### `abs_sub`

```rust
fn abs_sub<T: Signed>(x: T, y: T) -> T
```

The positive difference of two numbers.

Returns zero if `x` is less than or equal to `y`, otherwise the difference
between `x` and `y` is returned.

### `signum`

```rust
fn signum<T: Signed>(value: T) -> T
```

Returns the sign of the number.

For `f32` and `f64`:

* `1.0` if the number is positive, `+0.0` or `INFINITY`
* `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
* `NaN` if the number is `NaN`

For signed integers:

* `0` if the number is zero
* `1` if the number is positive
* `-1` if the number is negative

### `str_to_ascii_lower_eq_str`

```rust
fn str_to_ascii_lower_eq_str(a: &str, b: &str) -> bool
```

### `clamp`

```rust
fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T
```

A value bounded by a minimum and a maximum

 If input is less than min then this returns min.
 If input is greater than max then this returns max.
 Otherwise this returns input.

**Panics** in debug mode if `!(min <= max)`.

### `clamp_min`

```rust
fn clamp_min<T: PartialOrd>(input: T, min: T) -> T
```

A value bounded by a minimum value

 If input is less than min then this returns min.
 Otherwise this returns input.
 `clamp_min(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::min(std::f32::NAN, 1.0)`.

**Panics** in debug mode if `!(min == min)`. (This occurs if `min` is `NAN`.)

### `clamp_max`

```rust
fn clamp_max<T: PartialOrd>(input: T, max: T) -> T
```

A value bounded by a maximum value

 If input is greater than max then this returns max.
 Otherwise this returns input.
 `clamp_max(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::max(std::f32::NAN, 1.0)`.

**Panics** in debug mode if `!(max == max)`. (This occurs if `max` is `NAN`.)

## Macros

### `int_trait_impl!`

### `float_trait_impl!`

