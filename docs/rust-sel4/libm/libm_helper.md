**libm > libm_helper**

# Module: libm_helper

## Contents

**Structs**

- [`Libm`](#libm) - Generic helper for libm functions, abstracting over f32 and f64. <br/>

---

## libm::libm_helper::Libm

*Struct*

Generic helper for libm functions, abstracting over f32 and f64. <br/>
# Type Parameter:
- `T`: Either `f32` or `f64`

# Examples
```rust
use libm::{self, Libm};

const PI_F32: f32 = 3.1415927410e+00;
const PI_F64: f64 = 3.1415926535897931160e+00;

assert!(Libm::<f32>::cos(0.0f32) == libm::cosf(0.0));
assert!(Libm::<f32>::sin(PI_F32) == libm::sinf(PI_F32));

assert!(Libm::<f64>::cos(0.0f64) == libm::cos(0.0));
assert!(Libm::<f64>::sin(PI_F64) == libm::sin(PI_F64));
```

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn acos(x: f32) -> f32`
- `fn acosh(x: f32) -> f32`
- `fn asin(x: f32) -> f32`
- `fn asinh(x: f32) -> f32`
- `fn atan(x: f32) -> f32`
- `fn atan2(y: f32, x: f32) -> f32`
- `fn atanh(x: f32) -> f32`
- `fn cbrt(x: f32) -> f32`
- `fn ceil(x: f32) -> f32`
- `fn copysign(x: f32, y: f32) -> f32`
- `fn cos(x: f32) -> f32`
- `fn cosh(x: f32) -> f32`
- `fn erf(x: f32) -> f32`
- `fn erfc(x: f32) -> f32`
- `fn exp(x: f32) -> f32`
- `fn exp10(x: f32) -> f32`
- `fn exp2(x: f32) -> f32`
- `fn expm1(x: f32) -> f32`
- `fn fabs(x: f32) -> f32`
- `fn fdim(x: f32, y: f32) -> f32`
- `fn floor(x: f32) -> f32`
- `fn fma(x: f32, y: f32, z: f32) -> f32`
- `fn fmax(x: f32, y: f32) -> f32`
- `fn fmin(x: f32, y: f32) -> f32`
- `fn fmod(x: f32, y: f32) -> f32`
- `fn frexp(x: f32) -> (f32, i32)`
- `fn hypot(x: f32, y: f32) -> f32`
- `fn ilogb(x: f32) -> i32`
- `fn j0(x: f32) -> f32`
- `fn j1(x: f32) -> f32`
- `fn jn(n: i32, x: f32) -> f32`
- `fn ldexp(x: f32, n: i32) -> f32`
- `fn lgamma(x: f32) -> f32`
- `fn lgamma_r(x: f32) -> (f32, i32)`
- `fn log(x: f32) -> f32`
- `fn log10(x: f32) -> f32`
- `fn log1p(x: f32) -> f32`
- `fn log2(x: f32) -> f32`
- `fn modf(x: f32) -> (f32, f32)`
- `fn nextafter(x: f32, y: f32) -> f32`
- `fn pow(x: f32, y: f32) -> f32`
- `fn remainder(x: f32, y: f32) -> f32`
- `fn remquo(x: f32, y: f32) -> (f32, i32)`
- `fn rint(x: f32) -> f32`
- `fn round(x: f32) -> f32`
- `fn roundeven(x: f32) -> f32`
- `fn scalbn(x: f32, n: i32) -> f32`
- `fn sin(x: f32) -> f32`
- `fn sincos(x: f32) -> (f32, f32)`
- `fn sinh(x: f32) -> f32`
- `fn sqrt(x: f32) -> f32`
- `fn tan(x: f32) -> f32`
- `fn tanh(x: f32) -> f32`
- `fn tgamma(x: f32) -> f32`
- `fn trunc(x: f32) -> f32`
- `fn y0(x: f32) -> f32`
- `fn y1(x: f32) -> f32`
- `fn yn(n: i32, x: f32) -> f32`
- `fn acos(x: f64) -> f64`
- `fn acosh(x: f64) -> f64`
- `fn asin(x: f64) -> f64`
- `fn asinh(x: f64) -> f64`
- `fn atan(x: f64) -> f64`
- `fn atan2(y: f64, x: f64) -> f64`
- `fn atanh(x: f64) -> f64`
- `fn cbrt(x: f64) -> f64`
- `fn ceil(x: f64) -> f64`
- `fn copysign(x: f64, y: f64) -> f64`
- `fn cos(x: f64) -> f64`
- `fn cosh(x: f64) -> f64`
- `fn erf(x: f64) -> f64`
- `fn erfc(x: f64) -> f64`
- `fn exp(x: f64) -> f64`
- `fn exp10(x: f64) -> f64`
- `fn exp2(x: f64) -> f64`
- `fn expm1(x: f64) -> f64`
- `fn fabs(x: f64) -> f64`
- `fn fdim(x: f64, y: f64) -> f64`
- `fn floor(x: f64) -> f64`
- `fn fma(x: f64, y: f64, z: f64) -> f64`
- `fn fmax(x: f64, y: f64) -> f64`
- `fn fmaximum(x: f64, y: f64) -> f64`
- `fn fmaximum_num(x: f64, y: f64) -> f64`
- `fn fmaximum_numf(x: f32, y: f32) -> f32`
- `fn fmaximumf(x: f32, y: f32) -> f32`
- `fn fmin(x: f64, y: f64) -> f64`
- `fn fminimum(x: f64, y: f64) -> f64`
- `fn fminimum_num(x: f64, y: f64) -> f64`
- `fn fminimum_numf(x: f32, y: f32) -> f32`
- `fn fminimumf(x: f32, y: f32) -> f32`
- `fn fmod(x: f64, y: f64) -> f64`
- `fn frexp(x: f64) -> (f64, i32)`
- `fn hypot(x: f64, y: f64) -> f64`
- `fn ilogb(x: f64) -> i32`
- `fn j0(x: f64) -> f64`
- `fn j1(x: f64) -> f64`
- `fn jn(n: i32, x: f64) -> f64`
- `fn ldexp(x: f64, n: i32) -> f64`
- `fn lgamma(x: f64) -> f64`
- `fn lgamma_r(x: f64) -> (f64, i32)`
- `fn log(x: f64) -> f64`
- `fn log10(x: f64) -> f64`
- `fn log1p(x: f64) -> f64`
- `fn log2(x: f64) -> f64`
- `fn modf(x: f64) -> (f64, f64)`
- `fn nextafter(x: f64, y: f64) -> f64`
- `fn pow(x: f64, y: f64) -> f64`
- `fn remainder(x: f64, y: f64) -> f64`
- `fn remquo(x: f64, y: f64) -> (f64, i32)`
- `fn rint(x: f64) -> f64`
- `fn round(x: f64) -> f64`
- `fn roundeven(x: f64) -> f64`
- `fn scalbn(x: f64, n: i32) -> f64`
- `fn sin(x: f64) -> f64`
- `fn sincos(x: f64) -> (f64, f64)`
- `fn sinh(x: f64) -> f64`
- `fn sqrt(x: f64) -> f64`
- `fn tan(x: f64) -> f64`
- `fn tanh(x: f64) -> f64`
- `fn tgamma(x: f64) -> f64`
- `fn trunc(x: f64) -> f64`
- `fn y0(x: f64) -> f64`
- `fn y1(x: f64) -> f64`
- `fn yn(n: i32, x: f64) -> f64`



