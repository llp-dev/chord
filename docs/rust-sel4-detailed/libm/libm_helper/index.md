*[libm](../index.md) / [libm_helper](index.md)*

---

# Module `libm_helper`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Libm`](#libm) | struct | Generic helper for libm functions, abstracting over f32 and f64. |
| [`libm_helper!`](#libm-helper) | macro |  |

## Structs

### `Libm<T>`

```rust
struct Libm<T>(core::marker::PhantomData<T>);
```

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

#### Implementations

- <span id="libm-acos"></span>`fn acos(x: f32) -> f32`

- <span id="libm-acosh"></span>`fn acosh(x: f32) -> f32`

- <span id="libm-asin"></span>`fn asin(x: f32) -> f32`

- <span id="libm-asinh"></span>`fn asinh(x: f32) -> f32`

- <span id="libm-atan"></span>`fn atan(x: f32) -> f32`

- <span id="libm-atan2"></span>`fn atan2(y: f32, x: f32) -> f32`

- <span id="libm-atanh"></span>`fn atanh(x: f32) -> f32`

- <span id="libm-cbrt"></span>`fn cbrt(x: f32) -> f32`

- <span id="libm-ceil"></span>`fn ceil(x: f32) -> f32`

- <span id="libm-copysign"></span>`fn copysign(x: f32, y: f32) -> f32`

- <span id="libm-cos"></span>`fn cos(x: f32) -> f32`

- <span id="libm-cosh"></span>`fn cosh(x: f32) -> f32`

- <span id="libm-erf"></span>`fn erf(x: f32) -> f32`

- <span id="libm-erfc"></span>`fn erfc(x: f32) -> f32`

- <span id="libm-exp"></span>`fn exp(x: f32) -> f32`

- <span id="libm-exp10"></span>`fn exp10(x: f32) -> f32`

- <span id="libm-exp2"></span>`fn exp2(x: f32) -> f32`

- <span id="libm-expm1"></span>`fn expm1(x: f32) -> f32`

- <span id="libm-fabs"></span>`fn fabs(x: f32) -> f32`

- <span id="libm-fdim"></span>`fn fdim(x: f32, y: f32) -> f32`

- <span id="libm-floor"></span>`fn floor(x: f32) -> f32`

- <span id="libm-fma"></span>`fn fma(x: f32, y: f32, z: f32) -> f32`

- <span id="libm-fmax"></span>`fn fmax(x: f32, y: f32) -> f32`

- <span id="libm-fmin"></span>`fn fmin(x: f32, y: f32) -> f32`

- <span id="libm-fmod"></span>`fn fmod(x: f32, y: f32) -> f32`

- <span id="libm-frexp"></span>`fn frexp(x: f32) -> (f32, i32)`

- <span id="libm-hypot"></span>`fn hypot(x: f32, y: f32) -> f32`

- <span id="libm-ilogb"></span>`fn ilogb(x: f32) -> i32`

- <span id="libm-j0"></span>`fn j0(x: f32) -> f32`

- <span id="libm-j1"></span>`fn j1(x: f32) -> f32`

- <span id="libm-jn"></span>`fn jn(n: i32, x: f32) -> f32`

- <span id="libm-ldexp"></span>`fn ldexp(x: f32, n: i32) -> f32`

- <span id="libm-lgamma"></span>`fn lgamma(x: f32) -> f32`

- <span id="libm-lgamma-r"></span>`fn lgamma_r(x: f32) -> (f32, i32)`

- <span id="libm-log"></span>`fn log(x: f32) -> f32`

- <span id="libm-log10"></span>`fn log10(x: f32) -> f32`

- <span id="libm-log1p"></span>`fn log1p(x: f32) -> f32`

- <span id="libm-log2"></span>`fn log2(x: f32) -> f32`

- <span id="libm-modf"></span>`fn modf(x: f32) -> (f32, f32)`

- <span id="libm-nextafter"></span>`fn nextafter(x: f32, y: f32) -> f32`

- <span id="libm-pow"></span>`fn pow(x: f32, y: f32) -> f32`

- <span id="libm-remainder"></span>`fn remainder(x: f32, y: f32) -> f32`

- <span id="libm-remquo"></span>`fn remquo(x: f32, y: f32) -> (f32, i32)`

- <span id="libm-rint"></span>`fn rint(x: f32) -> f32`

- <span id="libm-round"></span>`fn round(x: f32) -> f32`

- <span id="libm-roundeven"></span>`fn roundeven(x: f32) -> f32`

- <span id="libm-scalbn"></span>`fn scalbn(x: f32, n: i32) -> f32`

- <span id="libm-sin"></span>`fn sin(x: f32) -> f32`

- <span id="libm-sincos"></span>`fn sincos(x: f32) -> (f32, f32)`

- <span id="libm-sinh"></span>`fn sinh(x: f32) -> f32`

- <span id="libm-sqrt"></span>`fn sqrt(x: f32) -> f32`

- <span id="libm-tan"></span>`fn tan(x: f32) -> f32`

- <span id="libm-tanh"></span>`fn tanh(x: f32) -> f32`

- <span id="libm-tgamma"></span>`fn tgamma(x: f32) -> f32`

- <span id="libm-trunc"></span>`fn trunc(x: f32) -> f32`

- <span id="libm-y0"></span>`fn y0(x: f32) -> f32`

- <span id="libm-y1"></span>`fn y1(x: f32) -> f32`

- <span id="libm-yn"></span>`fn yn(n: i32, x: f32) -> f32`

## Macros

### `libm_helper!`

