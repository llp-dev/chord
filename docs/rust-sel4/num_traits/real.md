**num_traits > real**

# Module: real

## Contents

**Traits**

- [`Real`](#real) - A trait for real number types that do not necessarily have

---

## num_traits::real::Real

*Trait*

A trait for real number types that do not necessarily have
floating-point-specific characteristics such as NaN and infinity.

See [this Wikipedia article](https://en.wikipedia.org/wiki/Real_data_type)
for a list of data types that could meaningfully implement this trait.

This trait is only available with the `std` feature, or with the `libm` feature otherwise.

**Methods:**

- `min_value`: Returns the smallest finite value that this type can represent.
- `min_positive_value`: Returns the smallest positive, normalized value that this type can represent.
- `epsilon`: Returns epsilon, a small positive value.
- `max_value`: Returns the largest finite value that this type can represent.
- `floor`: Returns the largest integer less than or equal to a number.
- `ceil`: Returns the smallest integer greater than or equal to a number.
- `round`: Returns the nearest integer to a number. Round half-way cases away from
- `trunc`: Return the integer part of a number.
- `fract`: Returns the fractional part of a number.
- `abs`: Computes the absolute value of `self`. Returns `Float::nan()` if the
- `signum`: Returns a number that represents the sign of `self`.
- `is_sign_positive`: Returns `true` if `self` is positive, including `+0.0`,
- `is_sign_negative`: Returns `true` if `self` is negative, including `-0.0`,
- `mul_add`: Fused multiply-add. Computes `(self * a) + b` with only one rounding
- `recip`: Take the reciprocal (inverse) of a number, `1/x`.
- `powi`: Raise a number to an integer power.
- `powf`: Raise a number to a real number power.
- `sqrt`: Take the square root of a number.
- `exp`: Returns `e^(self)`, (the exponential function).
- `exp2`: Returns `2^(self)`.
- `ln`: Returns the natural logarithm of the number.
- `log`: Returns the logarithm of the number with respect to an arbitrary base.
- `log2`: Returns the base 2 logarithm of the number.
- `log10`: Returns the base 10 logarithm of the number.
- `to_degrees`: Converts radians to degrees.
- `to_radians`: Converts degrees to radians.
- `max`: Returns the maximum of the two numbers.
- `min`: Returns the minimum of the two numbers.
- `abs_sub`: The positive difference of two numbers.
- `cbrt`: Take the cubic root of a number.
- `hypot`: Calculate the length of the hypotenuse of a right-angle triangle given
- `sin`: Computes the sine of a number (in radians).
- `cos`: Computes the cosine of a number (in radians).
- `tan`: Computes the tangent of a number (in radians).
- `asin`: Computes the arcsine of a number. Return value is in radians in
- `acos`: Computes the arccosine of a number. Return value is in radians in
- `atan`: Computes the arctangent of a number. Return value is in radians in the
- `atan2`: Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).
- `sin_cos`: Simultaneously computes the sine and cosine of the number, `x`. Returns
- `exp_m1`: Returns `e^(self) - 1` in a way that is accurate even if the
- `ln_1p`: Returns `ln(1+n)` (natural logarithm) more accurately than if
- `sinh`: Hyperbolic sine function.
- `cosh`: Hyperbolic cosine function.
- `tanh`: Hyperbolic tangent function.
- `asinh`: Inverse hyperbolic sine function.
- `acosh`: Inverse hyperbolic cosine function.
- `atanh`: Inverse hyperbolic tangent function.



