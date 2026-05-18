**num_complex**

# Module: num_complex

## Contents

**Structs**

- [`Complex`](#complex) - A complex number in Cartesian form.
- [`ParseComplexError`](#parsecomplexerror)

**Functions**

- [`c32`](#c32) - Create a new [`Complex<f32>`] with arguments that can convert [`Into<f32>`].
- [`c64`](#c64) - Create a new [`Complex<f64>`] with arguments that can convert [`Into<f64>`].

**Type Aliases**

- [`Complex32`](#complex32) - Alias for a [`Complex<f32>`]
- [`Complex64`](#complex64) - Alias for a [`Complex<f64>`]

---

## num_complex::Complex

*Struct*

A complex number in Cartesian form.

## Representation and Foreign Function Interface Compatibility

`Complex<T>` is memory layout compatible with an array `[T; 2]`.

Note that `Complex<F>` where F is a floating point type is **only** memory
layout compatible with C's complex types, **not** necessarily calling
convention compatible.  This means that for FFI you can only pass
`Complex<F>` behind a pointer, not as a value.

## Examples

Example of extern function declaration.

```
use num_complex::Complex;
use std::os::raw::c_int;

extern "C" {
    fn zaxpy_(n: *const c_int, alpha: *const Complex<f64>,
              x: *const Complex<f64>, incx: *const c_int,
              y: *mut Complex<f64>, incy: *const c_int);
}
```

**Generic Parameters:**
- T

**Fields:**
- `re: T` - Real portion of the complex number
- `im: T` - Imaginary portion of the complex number

**Methods:**

- `fn conj(self: &Self) -> Self` - Returns the complex conjugate. i.e. `re - i im`
- `fn inv(self: &Self) -> Self` - Returns `1/self`
- `fn powi(self: &Self, exp: i32) -> Self` - Raises `self` to a signed integer power.
- `fn new(re: T, im: T) -> Self` - Create a new `Complex`
- `fn is_nan(self: Self) -> bool` - Checks if the given complex number is NaN
- `fn is_infinite(self: Self) -> bool` - Checks if the given complex number is infinite
- `fn is_finite(self: Self) -> bool` - Checks if the given complex number is finite
- `fn is_normal(self: Self) -> bool` - Checks if the given complex number is normal
- `fn i() -> Self` - Returns the imaginary unit.
- `fn norm_sqr(self: &Self) -> T` - Returns the square of the norm (since `T` doesn't necessarily
- `fn scale(self: &Self, t: T) -> Self` - Multiplies `self` by the scalar `t`.
- `fn unscale(self: &Self, t: T) -> Self` - Divides `self` by the scalar `t`.
- `fn powu(self: &Self, exp: u32) -> Self` - Raises `self` to an unsigned integer power.
- `fn exp2(self: Self) -> Self` - Computes `2^(self)`.
- `fn log2(self: Self) -> Self` - Computes the principal value of log base 2 of `self`.
- `fn log10(self: Self) -> Self` - Computes the principal value of log base 10 of `self`.
- `fn cis(phase: T) -> Self` - Create a new Complex with a given phase: `exp(i * phase)`.
- `fn norm(self: Self) -> T` - Calculate |self|
- `fn arg(self: Self) -> T` - Calculate the principal Arg of self.
- `fn to_polar(self: Self) -> (T, T)` - Convert to polar form (r, theta), such that
- `fn from_polar(r: T, theta: T) -> Self` - Convert a polar representation into a complex number.
- `fn exp(self: Self) -> Self` - Computes `e^(self)`, where `e` is the base of the natural logarithm.
- `fn ln(self: Self) -> Self` - Computes the principal value of natural logarithm of `self`.
- `fn sqrt(self: Self) -> Self` - Computes the principal value of the square root of `self`.
- `fn cbrt(self: Self) -> Self` - Computes the principal value of the cube root of `self`.
- `fn powf(self: Self, exp: T) -> Self` - Raises `self` to a floating point power.
- `fn log(self: Self, base: T) -> Self` - Returns the logarithm of `self` with respect to an arbitrary base.
- `fn powc(self: Self, exp: Self) -> Self` - Raises `self` to a complex power.
- `fn expf(self: Self, base: T) -> Self` - Raises a floating point number to the complex power `self`.
- `fn sin(self: Self) -> Self` - Computes the sine of `self`.
- `fn cos(self: Self) -> Self` - Computes the cosine of `self`.
- `fn tan(self: Self) -> Self` - Computes the tangent of `self`.
- `fn asin(self: Self) -> Self` - Computes the principal value of the inverse sine of `self`.
- `fn acos(self: Self) -> Self` - Computes the principal value of the inverse cosine of `self`.
- `fn atan(self: Self) -> Self` - Computes the principal value of the inverse tangent of `self`.
- `fn sinh(self: Self) -> Self` - Computes the hyperbolic sine of `self`.
- `fn cosh(self: Self) -> Self` - Computes the hyperbolic cosine of `self`.
- `fn tanh(self: Self) -> Self` - Computes the hyperbolic tangent of `self`.
- `fn asinh(self: Self) -> Self` - Computes the principal value of inverse hyperbolic sine of `self`.
- `fn acosh(self: Self) -> Self` - Computes the principal value of inverse hyperbolic cosine of `self`.
- `fn atanh(self: Self) -> Self` - Computes the principal value of inverse hyperbolic tangent of `self`.
- `fn finv(self: Self) -> Complex<T>` - Returns `1/self` using floating-point operations.
- `fn fdiv(self: Self, other: Complex<T>) -> Complex<T>` - Returns `self/other` using floating-point operations.
- `fn l1_norm(self: &Self) -> T` - Returns the L1 norm `|re| + |im|` -- the [Manhattan distance] from the origin.

**Traits:** Copy, ConstZero, Eq, ConstOne

**Trait Implementations:**

- **AsPrimitive**
  - `fn as_(self: Self) -> U`
- **ComplexFloat**
  - `fn re(self: Self) -> <Self as >::Real`
  - `fn im(self: Self) -> <Self as >::Real`
  - `fn abs(self: Self) -> <Self as >::Real`
  - `fn recip(self: Self) -> Self`
  - `fn l1_norm(self: &Self) -> <Self as >::Real`
  - `fn is_nan(self: Self) -> bool`
  - `fn is_infinite(self: Self) -> bool`
  - `fn is_finite(self: Self) -> bool`
  - `fn is_normal(self: Self) -> bool`
  - `fn arg(self: Self) -> <Self as >::Real`
  - `fn powc(self: Self, exp: Complex<<Self as >::Real>) -> Complex<<Self as >::Real>`
  - `fn exp2(self: Self) -> Self`
  - `fn log(self: Self, base: <Self as >::Real) -> Self`
  - `fn log2(self: Self) -> Self`
  - `fn log10(self: Self) -> Self`
  - `fn powf(self: Self, f: <Self as >::Real) -> Self`
  - `fn sqrt(self: Self) -> Self`
  - `fn cbrt(self: Self) -> Self`
  - `fn exp(self: Self) -> Self`
  - `fn expf(self: Self, base: <Self as >::Real) -> Self`
  - `fn ln(self: Self) -> Self`
  - `fn sin(self: Self) -> Self`
  - `fn cos(self: Self) -> Self`
  - `fn tan(self: Self) -> Self`
  - `fn asin(self: Self) -> Self`
  - `fn acos(self: Self) -> Self`
  - `fn atan(self: Self) -> Self`
  - `fn sinh(self: Self) -> Self`
  - `fn cosh(self: Self) -> Self`
  - `fn tanh(self: Self) -> Self`
  - `fn asinh(self: Self) -> Self`
  - `fn acosh(self: Self) -> Self`
  - `fn atanh(self: Self) -> Self`
  - `fn powi(self: Self, n: i32) -> Self`
  - `fn conj(self: Self) -> Self`
- **Rem**
  - `fn rem(self: Self, other: &Complex<T>) -> <Self as >::Output`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Div**
  - `fn div(self: Self, other: T) -> <Self as >::Output`
- **Num**
  - `fn from_str_radix(s: &str, radix: u32) -> Result<Self, <Self as >::FromStrRadixErr>` - Parses `a +/- bi`; `ai +/- b`; `a`; or `bi` where `a` and `b` are of type `T`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &Self)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &Complex<T>) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &Self)`
- **Rem**
  - `fn rem(self: Self, other: &T) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &Complex<T>) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &Self)`
- **MulAddAssign**
  - `fn mul_add_assign(self: & mut Self, other: Complex<T>, add: Complex<T>)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(re: T) -> Self`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &Self)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Pow**
  - `fn pow(self: Self, exp: Complex<T>) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &Complex<T>) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)`
- **Div**
  - `fn div(self: Self, other: &T) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &Self)`
- **Rem**
  - `fn rem(self: Self, modulus: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: T)`
- **Rem**
  - `fn rem(self: Self, other: T) -> <Self as >::Output`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: Self) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: T)`
- **Mul**
  - `fn mul(self: Self, other: &T) -> <Self as >::Output`
- **FromPrimitive**
  - `fn from_usize(n: usize) -> Option<Self>`
  - `fn from_isize(n: isize) -> Option<Self>`
  - `fn from_u8(n: u8) -> Option<Self>`
  - `fn from_u16(n: u16) -> Option<Self>`
  - `fn from_u32(n: u32) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_i8(n: i8) -> Option<Self>`
  - `fn from_i16(n: i16) -> Option<Self>`
  - `fn from_i32(n: i32) -> Option<Self>`
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **Pow**
  - `fn pow(self: Self, exp: f64) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &Complex<T>) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, modulus: Self)`
- **Div**
  - `fn div(self: Self, other: Self) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Complex<T>) -> bool`
- **MulAddAssign**
  - `fn mul_add_assign(self: & mut Self, other: &Complex<T>, add: &Complex<T>)`
- **Add**
  - `fn add(self: Self, other: T) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Sub**
  - `fn sub(self: Self, other: &T) -> <Self as >::Output`
- **Pow**
  - `fn pow(self: Self, exp: &'b Complex<T>) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: Self) -> <Self as >::Output`
- **Pow**
  - `fn pow(self: Self, exp: f32) -> <Self as >::Output`
- **ToPrimitive**
  - `fn to_usize(self: &Self) -> Option<usize>`
  - `fn to_isize(self: &Self) -> Option<isize>`
  - `fn to_u8(self: &Self) -> Option<u8>`
  - `fn to_u16(self: &Self) -> Option<u16>`
  - `fn to_u32(self: &Self) -> Option<u32>`
  - `fn to_u64(self: &Self) -> Option<u64>`
  - `fn to_i8(self: &Self) -> Option<i8>`
  - `fn to_i16(self: &Self) -> Option<i16>`
  - `fn to_i32(self: &Self) -> Option<i32>`
  - `fn to_i64(self: &Self) -> Option<i64>`
  - `fn to_u128(self: &Self) -> Option<u128>`
  - `fn to_i128(self: &Self) -> Option<i128>`
  - `fn to_f32(self: &Self) -> Option<f32>`
  - `fn to_f64(self: &Self) -> Option<f64>`
- **Add**
  - `fn add(self: Self, other: &T) -> <Self as >::Output`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **MulAdd**
  - `fn mul_add(self: Self, other: Complex<T>, add: Complex<T>) -> Complex<T>`
- **NumCast**
  - `fn from<U>(n: U) -> Option<Self>`
- **Pow**
  - `fn pow(self: Self, exp: &f64) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: Self) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &T)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &T)`
- **Sub**
  - `fn sub(self: Self, other: T) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &T)`
- **Pow**
  - `fn pow(self: Self, exp: &f32) -> <Self as >::Output`
- **From**
  - `fn from(re: &T) -> Self`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &T)`
- **Zero**
  - `fn zero() -> Self`
  - `fn is_zero(self: &Self) -> bool`
  - `fn set_zero(self: & mut Self)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: Self)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &T)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: Self)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: T)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: T)`
- **Clone**
  - `fn clone(self: &Self) -> Complex<T>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: T)`
- **Mul**
  - `fn mul(self: Self, other: T) -> <Self as >::Output`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>` - Parses `a +/- bi`; `ai +/- b`; `a`; or `bi` where `a` and `b` are of type `T`
- **One**
  - `fn one() -> Self`
  - `fn is_one(self: &Self) -> bool`
  - `fn set_one(self: & mut Self)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: Self)`
- **Inv**
  - `fn inv(self: Self) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Complex<T>`
- **Product**
  - `fn product<I>(iter: I) -> Self`



## num_complex::Complex32

*Type Alias*: `Complex<f32>`

Alias for a [`Complex<f32>`]



## num_complex::Complex64

*Type Alias*: `Complex<f64>`

Alias for a [`Complex<f64>`]



## num_complex::ParseComplexError

*Struct*

**Generic Parameters:**
- E

**Trait Implementations:**

- **Error**
  - `fn description(self: &Self) -> &str`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseComplexError<E>) -> bool`



## num_complex::c32

*Function*

Create a new [`Complex<f32>`] with arguments that can convert [`Into<f32>`].

```
use num_complex::{c32, Complex32};
assert_eq!(c32(1u8, 2), Complex32::new(1.0, 2.0));
```

Note: ambiguous integer literals in Rust will [default] to `i32`, which does **not** implement
`Into<f32>`, so a call like `c32(1, 2)` will result in a type error. The example above uses a
suffixed `1u8` to set its type, and then the `2` can be inferred as the same type.

[default]: https://doc.rust-lang.org/reference/expressions/literal-expr.html#integer-literal-expressions

```rust
fn c32<T>(re: T, im: T) -> Complex32
```



## num_complex::c64

*Function*

Create a new [`Complex<f64>`] with arguments that can convert [`Into<f64>`].

```
use num_complex::{c64, Complex64};
assert_eq!(c64(1, 2), Complex64::new(1.0, 2.0));
```

```rust
fn c64<T>(re: T, im: T) -> Complex64
```



