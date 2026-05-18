*[num_traits](../index.md) / [float](index.md)*

---

# Module `float`

## Contents

- [Traits](#traits)
  - [`FloatCore`](#floatcore)
  - [`Float`](#float)
  - [`FloatConst`](#floatconst)
  - [`TotalOrder`](#totalorder)
- [Functions](#functions)
  - [`integer_decode_f32`](#integer-decode-f32)
  - [`integer_decode_f64`](#integer-decode-f64)
- [Macros](#macros)
  - [`float_impl_std!`](#float-impl-std)
  - [`float_const_impl!`](#float-const-impl)
  - [`totalorder_impl!`](#totalorder-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FloatCore`](#floatcore) | trait | Generic trait for floating point numbers that works with `no_std`. |
| [`Float`](#float) | trait | Generic trait for floating point numbers |
| [`FloatConst`](#floatconst) | trait |  |
| [`TotalOrder`](#totalorder) | trait | Trait for floating point numbers that provide an implementation of the `totalOrder` predicate as defined in the IEEE 754 (2008 revision) floating point standard. |
| [`integer_decode_f32`](#integer-decode-f32) | fn |  |
| [`integer_decode_f64`](#integer-decode-f64) | fn |  |
| [`float_impl_std!`](#float-impl-std) | macro |  |
| [`float_const_impl!`](#float-const-impl) | macro |  |
| [`totalorder_impl!`](#totalorder-impl) | macro |  |

## Traits

### `FloatCore`

```rust
trait FloatCore: Num + NumCast + Neg<Output = Self> + PartialOrd + Copy { ... }
```

Generic trait for floating point numbers that works with `no_std`.

This trait implements a subset of the `Float` trait.

#### Required Methods

- `fn infinity() -> Self`

  Returns positive infinity.

- `fn neg_infinity() -> Self`

  Returns negative infinity.

- `fn nan() -> Self`

  Returns NaN.

- `fn neg_zero() -> Self`

  Returns `-0.0`.

- `fn min_value() -> Self`

  Returns the smallest finite value that this type can represent.

- `fn min_positive_value() -> Self`

  Returns the smallest positive, normalized value that this type can represent.

- `fn epsilon() -> Self`

  Returns epsilon, a small positive value.

- `fn max_value() -> Self`

  Returns the largest finite value that this type can represent.

- `fn classify(self) -> FpCategory`

  Returns the floating point category of the number. If only one property

- `fn to_degrees(self) -> Self`

  Converts to degrees, assuming the number is in radians.

- `fn to_radians(self) -> Self`

  Converts to radians, assuming the number is in degrees.

- `fn integer_decode(self) -> (u64, i16, i8)`

  Returns the mantissa, base 2 exponent, and sign as integers, respectively.

#### Provided Methods

- `fn is_nan(self) -> bool`

  Returns `true` if the number is NaN.

- `fn is_infinite(self) -> bool`

  Returns `true` if the number is infinite.

- `fn is_finite(self) -> bool`

  Returns `true` if the number is neither infinite or NaN.

- `fn is_normal(self) -> bool`

  Returns `true` if the number is neither zero, infinite, subnormal or NaN.

- `fn is_subnormal(self) -> bool`

  Returns `true` if the number is [subnormal].

- `fn floor(self) -> Self`

  Returns the largest integer less than or equal to a number.

- `fn ceil(self) -> Self`

  Returns the smallest integer greater than or equal to a number.

- `fn round(self) -> Self`

  Returns the nearest integer to a number. Round half-way cases away from `0.0`.

- `fn trunc(self) -> Self`

  Return the integer part of a number.

- `fn fract(self) -> Self`

  Returns the fractional part of a number.

- `fn abs(self) -> Self`

  Computes the absolute value of `self`. Returns `FloatCore::nan()` if the

- `fn signum(self) -> Self`

  Returns a number that represents the sign of `self`.

- `fn is_sign_positive(self) -> bool`

  Returns `true` if `self` is positive, including `+0.0` and

- `fn is_sign_negative(self) -> bool`

  Returns `true` if `self` is negative, including `-0.0` and

- `fn min(self, other: Self) -> Self`

  Returns the minimum of the two numbers.

- `fn max(self, other: Self) -> Self`

  Returns the maximum of the two numbers.

- `fn clamp(self, min: Self, max: Self) -> Self`

  A value bounded by a minimum and a maximum

- `fn recip(self) -> Self`

  Returns the reciprocal (multiplicative inverse) of the number.

- `fn powi(self, exp: i32) -> Self`

  Raise a number to an integer power.

#### Implementors

- `f32`
- `f64`

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

### `TotalOrder`

```rust
trait TotalOrder { ... }
```

Trait for floating point numbers that provide an implementation
of the `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
floating point standard.

#### Required Methods

- `fn total_cmp(&self, other: &Self) -> Ordering`

  Return the ordering between `self` and `other`.

#### Implementors

- `f32`
- `f64`

## Functions

### `integer_decode_f32`

```rust
fn integer_decode_f32(f: f32) -> (u64, i16, i8)
```

### `integer_decode_f64`

```rust
fn integer_decode_f64(f: f64) -> (u64, i16, i8)
```

## Macros

### `float_impl_std!`

### `float_const_impl!`

### `totalorder_impl!`

