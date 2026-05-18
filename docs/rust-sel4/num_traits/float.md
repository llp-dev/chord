**num_traits > float**

# Module: float

## Contents

**Traits**

- [`Float`](#float) - Generic trait for floating point numbers
- [`FloatConst`](#floatconst)
- [`FloatCore`](#floatcore) - Generic trait for floating point numbers that works with `no_std`.
- [`TotalOrder`](#totalorder) - Trait for floating point numbers that provide an implementation

---

## num_traits::float::Float

*Trait*

Generic trait for floating point numbers

This trait is only available with the `std` feature, or with the `libm` feature otherwise.

**Methods:**

- `nan`: Returns the `NaN` value.
- `infinity`: Returns the infinite value.
- `neg_infinity`: Returns the negative infinite value.
- `neg_zero`: Returns `-0.0`.
- `min_value`: Returns the smallest finite value that this type can represent.
- `min_positive_value`: Returns the smallest positive, normalized value that this type can represent.
- `epsilon`: Returns epsilon, a small positive value.
- `max_value`: Returns the largest finite value that this type can represent.
- `is_nan`: Returns `true` if this value is `NaN` and false otherwise.
- `is_infinite`: Returns `true` if this value is positive infinity or negative infinity and
- `is_finite`: Returns `true` if this number is neither infinite nor `NaN`.
- `is_normal`: Returns `true` if the number is neither zero, infinite,
- `is_subnormal`: Returns `true` if the number is [subnormal].
- `classify`: Returns the floating point category of the number. If only one property
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
- `powf`: Raise a number to a floating point power.
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
- `clamp`: Clamps a value between a min and max.
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
- `integer_decode`: Returns the mantissa, base 2 exponent, and sign as integers, respectively.
- `copysign`: Returns a number composed of the magnitude of `self` and the sign of



## num_traits::float::FloatConst

*Trait*

**Methods:**

- `E`: Return Euler’s number.
- `FRAC_1_PI`: Return `1.0 / π`.
- `FRAC_1_SQRT_2`: Return `1.0 / sqrt(2.0)`.
- `FRAC_2_PI`: Return `2.0 / π`.
- `FRAC_2_SQRT_PI`: Return `2.0 / sqrt(π)`.
- `FRAC_PI_2`: Return `π / 2.0`.
- `FRAC_PI_3`: Return `π / 3.0`.
- `FRAC_PI_4`: Return `π / 4.0`.
- `FRAC_PI_6`: Return `π / 6.0`.
- `FRAC_PI_8`: Return `π / 8.0`.
- `LN_10`: Return `ln(10.0)`.
- `LN_2`: Return `ln(2.0)`.
- `LOG10_E`: Return `log10(e)`.
- `LOG2_E`: Return `log2(e)`.
- `PI`: Return Archimedes’ constant `π`.
- `SQRT_2`: Return `sqrt(2.0)`.
- `TAU`: Return the full circle constant `τ`.
- `LOG10_2`: Return `log10(2.0)`.
- `LOG2_10`: Return `log2(10.0)`.



## num_traits::float::FloatCore

*Trait*

Generic trait for floating point numbers that works with `no_std`.

This trait implements a subset of the `Float` trait.

**Methods:**

- `infinity`: Returns positive infinity.
- `neg_infinity`: Returns negative infinity.
- `nan`: Returns NaN.
- `neg_zero`: Returns `-0.0`.
- `min_value`: Returns the smallest finite value that this type can represent.
- `min_positive_value`: Returns the smallest positive, normalized value that this type can represent.
- `epsilon`: Returns epsilon, a small positive value.
- `max_value`: Returns the largest finite value that this type can represent.
- `is_nan`: Returns `true` if the number is NaN.
- `is_infinite`: Returns `true` if the number is infinite.
- `is_finite`: Returns `true` if the number is neither infinite or NaN.
- `is_normal`: Returns `true` if the number is neither zero, infinite, subnormal or NaN.
- `is_subnormal`: Returns `true` if the number is [subnormal].
- `classify`: Returns the floating point category of the number. If only one property
- `floor`: Returns the largest integer less than or equal to a number.
- `ceil`: Returns the smallest integer greater than or equal to a number.
- `round`: Returns the nearest integer to a number. Round half-way cases away from `0.0`.
- `trunc`: Return the integer part of a number.
- `fract`: Returns the fractional part of a number.
- `abs`: Computes the absolute value of `self`. Returns `FloatCore::nan()` if the
- `signum`: Returns a number that represents the sign of `self`.
- `is_sign_positive`: Returns `true` if `self` is positive, including `+0.0` and
- `is_sign_negative`: Returns `true` if `self` is negative, including `-0.0` and
- `min`: Returns the minimum of the two numbers.
- `max`: Returns the maximum of the two numbers.
- `clamp`: A value bounded by a minimum and a maximum
- `recip`: Returns the reciprocal (multiplicative inverse) of the number.
- `powi`: Raise a number to an integer power.
- `to_degrees`: Converts to degrees, assuming the number is in radians.
- `to_radians`: Converts to radians, assuming the number is in degrees.
- `integer_decode`: Returns the mantissa, base 2 exponent, and sign as integers, respectively.



## num_traits::float::TotalOrder

*Trait*

Trait for floating point numbers that provide an implementation
of the `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
floating point standard.

**Methods:**

- `total_cmp`: Return the ordering between `self` and `other`.



