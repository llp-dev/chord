*[num_traits](../index.md) / [real](index.md)*

---

# Module `real`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Real`](#real) | trait | A trait for real number types that do not necessarily have floating-point-specific characteristics such as NaN and infinity. |

## Traits

### `Real`

```rust
trait Real: Num + Copy + NumCast + PartialOrd + Neg<Output = Self> { ... }
```

A trait for real number types that do not necessarily have
floating-point-specific characteristics such as NaN and infinity.

See [this Wikipedia article](https://en.wikipedia.org/wiki/Real_data_type)
for a list of data types that could meaningfully implement this trait.

This trait is only available with the `std` feature, or with the `libm` feature otherwise.

#### Required Methods

- `fn min_value() -> Self`

  Returns the smallest finite value that this type can represent.

- `fn min_positive_value() -> Self`

  Returns the smallest positive, normalized value that this type can represent.

- `fn epsilon() -> Self`

  Returns epsilon, a small positive value.

- `fn max_value() -> Self`

  Returns the largest finite value that this type can represent.

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

  Raise a number to a real number power.

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

- `fn to_degrees(self) -> Self`

  Converts radians to degrees.

- `fn to_radians(self) -> Self`

  Converts degrees to radians.

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

#### Implementors

- `T`

