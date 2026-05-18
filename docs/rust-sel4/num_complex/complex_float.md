**num_complex > complex_float**

# Module: complex_float

## Contents

**Traits**

- [`ComplexFloat`](#complexfloat) - Generic trait for floating point complex numbers.

---

## num_complex::complex_float::ComplexFloat

*Trait*

Generic trait for floating point complex numbers.

This trait defines methods which are common to complex floating point
numbers and regular floating point numbers.

This trait is sealed to prevent it from being implemented by anything other
than floating point scalars and [Complex] floats.

**Methods:**

- `Real`: The type used to represent the real coefficients of this complex number.
- `is_nan`: Returns `true` if this value is `NaN` and false otherwise.
- `is_infinite`: Returns `true` if this value is positive infinity or negative infinity and
- `is_finite`: Returns `true` if this number is neither infinite nor `NaN`.
- `is_normal`: Returns `true` if the number is neither zero, infinite,
- `recip`: Take the reciprocal (inverse) of a number, `1/x`. See also [Complex::finv].
- `powi`: Raises `self` to a signed integer power.
- `powf`: Raises `self` to a real power.
- `powc`: Raises `self` to a complex power.
- `sqrt`: Take the square root of a number.
- `exp`: Returns `e^(self)`, (the exponential function).
- `exp2`: Returns `2^(self)`.
- `expf`: Returns `base^(self)`.
- `ln`: Returns the natural logarithm of the number.
- `log`: Returns the logarithm of the number with respect to an arbitrary base.
- `log2`: Returns the base 2 logarithm of the number.
- `log10`: Returns the base 10 logarithm of the number.
- `cbrt`: Take the cubic root of a number.
- `sin`: Computes the sine of a number (in radians).
- `cos`: Computes the cosine of a number (in radians).
- `tan`: Computes the tangent of a number (in radians).
- `asin`: Computes the arcsine of a number. Return value is in radians in
- `acos`: Computes the arccosine of a number. Return value is in radians in
- `atan`: Computes the arctangent of a number. Return value is in radians in the
- `sinh`: Hyperbolic sine function.
- `cosh`: Hyperbolic cosine function.
- `tanh`: Hyperbolic tangent function.
- `asinh`: Inverse hyperbolic sine function.
- `acosh`: Inverse hyperbolic cosine function.
- `atanh`: Inverse hyperbolic tangent function.
- `re`: Returns the real part of the number.
- `im`: Returns the imaginary part of the number.
- `abs`: Returns the absolute value of the number. See also [Complex::norm]
- `l1_norm`: Returns the L1 norm `|re| + |im|` -- the [Manhattan distance] from the origin.
- `arg`: Computes the argument of the number.
- `conj`: Computes the complex conjugate of the number.



