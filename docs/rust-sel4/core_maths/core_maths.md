**core_maths**

# Module: core_maths

## Contents

**Traits**

- [`CoreFloat`](#corefloat) - See [`crate`].

---

## core_maths::CoreFloat

*Trait*

See [`crate`].

**Methods:**

- `floor`: Returns the largest integer less than or equal to `self`.
- `ceil`: Returns the smallest integer greater than or equal to `self`.
- `round`: Returns the nearest integer to `self`. If a value is half-way between two
- `trunc`: Returns the integer part of `self`.
- `fract`: Returns the fractional part of `self`.
- `abs`: Computes the absolute value of `self`.
- `signum`: Returns a number that represents the sign of `self`.
- `copysign`: Returns a number composed of the magnitude of `self` and the sign of
- `mul_add`: Fused multiply-add. Computes `(self * a) + b` with only one rounding
- `div_euclid`: Calculates Euclidean division, the matching method for `rem_euclid`.
- `rem_euclid`: Calculates the least nonnegative remainder of `self (mod rhs)`.
- `powi`: Raises a number to an integer power.
- `powf`: Raises a number to a floating point power.
- `sqrt`: Returns the square root of a number.
- `exp`: Returns `e^(self)`, (the exponential function).
- `exp2`: Returns `2^(self)`.
- `ln`: Returns the natural logarithm of the number.
- `log`: Returns the logarithm of the number with respect to an arbitrary base.
- `log2`: Returns the base 2 logarithm of the number.
- `log10`: Returns the base 10 logarithm of the number.
- `cbrt`: Returns the cube root of a number.
- `hypot`: Compute the distance between the origin and a point (`x`, `y`) on the
- `sin`: Computes the sine of a number (in radians).
- `cos`: Computes the cosine of a number (in radians).
- `tan`: Computes the tangent of a number (in radians).
- `asin`: Computes the arcsine of a number. Return value is in radians in
- `acos`: Computes the arccosine of a number. Return value is in radians in
- `atan`: Computes the arctangent of a number. Return value is in radians in the
- `atan2`: Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
- `sin_cos`: Simultaneously computes the sine and cosine of the number, `x`. Returns
- `exp_m1`: Returns `e^(self) - 1` in a way that is accurate even if the
- `ln_1p`: Returns `ln(1+n)` (natural logarithm) more accurately than if
- `sinh`: Hyperbolic sine function.
- `cosh`: Hyperbolic cosine function.
- `tanh`: Hyperbolic tangent function.
- `asinh`: Inverse hyperbolic sine function.
- `acosh`: Inverse hyperbolic cosine function.
- `atanh`: Inverse hyperbolic tangent function.



