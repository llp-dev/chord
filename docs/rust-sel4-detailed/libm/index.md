# Crate `libm`

libm in pure Rust

## Contents

- [Modules](#modules)
  - [`libm_helper`](#libm-helper)
  - [`math`](#math)
  - [`support`](#support)
  - [`generic`](#generic)
  - [`arch`](#arch)
  - [`expo2`](#expo2)
  - [`k_cos`](#k-cos)
  - [`k_cosf`](#k-cosf)
  - [`k_expo2`](#k-expo2)
  - [`k_expo2f`](#k-expo2f)
  - [`k_sin`](#k-sin)
  - [`k_sinf`](#k-sinf)
  - [`k_tan`](#k-tan)
  - [`k_tanf`](#k-tanf)
  - [`rem_pio2`](#rem-pio2)
  - [`rem_pio2_large`](#rem-pio2-large)
  - [`rem_pio2f`](#rem-pio2f)
  - [`acos`](#acos)
  - [`acosf`](#acosf)
  - [`acosh`](#acosh)
  - [`acoshf`](#acoshf)
  - [`asin`](#asin)
  - [`asinf`](#asinf)
  - [`asinh`](#asinh)
  - [`asinhf`](#asinhf)
  - [`atan`](#atan)
  - [`atan2`](#atan2)
  - [`atan2f`](#atan2f)
  - [`atanf`](#atanf)
  - [`atanh`](#atanh)
  - [`atanhf`](#atanhf)
  - [`cbrt`](#cbrt)
  - [`cbrtf`](#cbrtf)
  - [`ceil`](#ceil)
  - [`copysign`](#copysign)
  - [`cos`](#cos)
  - [`cosf`](#cosf)
  - [`cosh`](#cosh)
  - [`coshf`](#coshf)
  - [`erf`](#erf)
  - [`erff`](#erff)
  - [`exp`](#exp)
  - [`exp10`](#exp10)
  - [`exp10f`](#exp10f)
  - [`exp2`](#exp2)
  - [`exp2f`](#exp2f)
  - [`expf`](#expf)
  - [`expm1`](#expm1)
  - [`expm1f`](#expm1f)
  - [`fabs`](#fabs)
  - [`fdim`](#fdim)
  - [`floor`](#floor)
  - [`fma`](#fma)
  - [`fmin_fmax`](#fmin-fmax)
  - [`fminimum_fmaximum`](#fminimum-fmaximum)
  - [`fminimum_fmaximum_num`](#fminimum-fmaximum-num)
  - [`fmod`](#fmod)
  - [`frexp`](#frexp)
  - [`frexpf`](#frexpf)
  - [`hypot`](#hypot)
  - [`hypotf`](#hypotf)
  - [`ilogb`](#ilogb)
  - [`ilogbf`](#ilogbf)
  - [`j0`](#j0)
  - [`j0f`](#j0f)
  - [`j1`](#j1)
  - [`j1f`](#j1f)
  - [`jn`](#jn)
  - [`jnf`](#jnf)
  - [`ldexp`](#ldexp)
  - [`lgamma`](#lgamma)
  - [`lgamma_r`](#lgamma-r)
  - [`lgammaf`](#lgammaf)
  - [`lgammaf_r`](#lgammaf-r)
  - [`log`](#log)
  - [`log10`](#log10)
  - [`log10f`](#log10f)
  - [`log1p`](#log1p)
  - [`log1pf`](#log1pf)
  - [`log2`](#log2)
  - [`log2f`](#log2f)
  - [`logf`](#logf)
  - [`modf`](#modf)
  - [`modff`](#modff)
  - [`nextafter`](#nextafter)
  - [`nextafterf`](#nextafterf)
  - [`pow`](#pow)
  - [`powf`](#powf)
  - [`remainder`](#remainder)
  - [`remainderf`](#remainderf)
  - [`remquo`](#remquo)
  - [`remquof`](#remquof)
  - [`rint`](#rint)
  - [`round`](#round)
  - [`roundeven`](#roundeven)
  - [`scalbn`](#scalbn)
  - [`sin`](#sin)
  - [`sincos`](#sincos)
  - [`sincosf`](#sincosf)
  - [`sinf`](#sinf)
  - [`sinh`](#sinh)
  - [`sinhf`](#sinhf)
  - [`sqrt`](#sqrt)
  - [`tan`](#tan)
  - [`tanf`](#tanf)
  - [`tanh`](#tanh)
  - [`tanhf`](#tanhf)
  - [`tgamma`](#tgamma)
  - [`tgammaf`](#tgammaf)
  - [`trunc`](#trunc)
- [Structs](#structs)
  - [`Libm`](#libm)
- [Functions](#functions)
  - [`get_high_word`](#get-high-word)
  - [`get_low_word`](#get-low-word)
  - [`with_set_high_word`](#with-set-high-word)
  - [`with_set_low_word`](#with-set-low-word)
  - [`combine_words`](#combine-words)
- [Macros](#macros)
  - [`libm_helper!`](#libm-helper)
  - [`force_eval!`](#force-eval)
  - [`i!`](#i)
  - [`div!`](#div)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`libm_helper`](#libm-helper) | mod |  |
| [`math`](#math) | mod |  |
| [`support`](#support) | mod |  |
| [`generic`](#generic) | mod |  |
| [`arch`](#arch) | mod | Architecture-specific routines and operations. |
| [`expo2`](#expo2) | mod |  |
| [`k_cos`](#k-cos) | mod |  |
| [`k_cosf`](#k-cosf) | mod |  |
| [`k_expo2`](#k-expo2) | mod |  |
| [`k_expo2f`](#k-expo2f) | mod |  |
| [`k_sin`](#k-sin) | mod |  |
| [`k_sinf`](#k-sinf) | mod |  |
| [`k_tan`](#k-tan) | mod |  |
| [`k_tanf`](#k-tanf) | mod |  |
| [`rem_pio2`](#rem-pio2) | mod |  |
| [`rem_pio2_large`](#rem-pio2-large) | mod |  |
| [`rem_pio2f`](#rem-pio2f) | mod |  |
| [`acos`](#acos) | mod |  |
| [`acosf`](#acosf) | mod |  |
| [`acosh`](#acosh) | mod |  |
| [`acoshf`](#acoshf) | mod |  |
| [`asin`](#asin) | mod |  |
| [`asinf`](#asinf) | mod |  |
| [`asinh`](#asinh) | mod |  |
| [`asinhf`](#asinhf) | mod |  |
| [`atan`](#atan) | mod |  |
| [`atan2`](#atan2) | mod |  |
| [`atan2f`](#atan2f) | mod |  |
| [`atanf`](#atanf) | mod |  |
| [`atanh`](#atanh) | mod |  |
| [`atanhf`](#atanhf) | mod |  |
| [`cbrt`](#cbrt) | mod |  |
| [`cbrtf`](#cbrtf) | mod |  |
| [`ceil`](#ceil) | mod |  |
| [`copysign`](#copysign) | mod |  |
| [`cos`](#cos) | mod |  |
| [`cosf`](#cosf) | mod |  |
| [`cosh`](#cosh) | mod |  |
| [`coshf`](#coshf) | mod |  |
| [`erf`](#erf) | mod |  |
| [`erff`](#erff) | mod |  |
| [`exp`](#exp) | mod |  |
| [`exp10`](#exp10) | mod |  |
| [`exp10f`](#exp10f) | mod |  |
| [`exp2`](#exp2) | mod |  |
| [`exp2f`](#exp2f) | mod |  |
| [`expf`](#expf) | mod |  |
| [`expm1`](#expm1) | mod |  |
| [`expm1f`](#expm1f) | mod |  |
| [`fabs`](#fabs) | mod |  |
| [`fdim`](#fdim) | mod |  |
| [`floor`](#floor) | mod |  |
| [`fma`](#fma) | mod |  |
| [`fmin_fmax`](#fmin-fmax) | mod |  |
| [`fminimum_fmaximum`](#fminimum-fmaximum) | mod |  |
| [`fminimum_fmaximum_num`](#fminimum-fmaximum-num) | mod |  |
| [`fmod`](#fmod) | mod |  |
| [`frexp`](#frexp) | mod |  |
| [`frexpf`](#frexpf) | mod |  |
| [`hypot`](#hypot) | mod |  |
| [`hypotf`](#hypotf) | mod |  |
| [`ilogb`](#ilogb) | mod |  |
| [`ilogbf`](#ilogbf) | mod |  |
| [`j0`](#j0) | mod |  |
| [`j0f`](#j0f) | mod |  |
| [`j1`](#j1) | mod |  |
| [`j1f`](#j1f) | mod |  |
| [`jn`](#jn) | mod |  |
| [`jnf`](#jnf) | mod |  |
| [`ldexp`](#ldexp) | mod |  |
| [`lgamma`](#lgamma) | mod |  |
| [`lgamma_r`](#lgamma-r) | mod |  |
| [`lgammaf`](#lgammaf) | mod |  |
| [`lgammaf_r`](#lgammaf-r) | mod |  |
| [`log`](#log) | mod |  |
| [`log10`](#log10) | mod |  |
| [`log10f`](#log10f) | mod |  |
| [`log1p`](#log1p) | mod |  |
| [`log1pf`](#log1pf) | mod |  |
| [`log2`](#log2) | mod |  |
| [`log2f`](#log2f) | mod |  |
| [`logf`](#logf) | mod |  |
| [`modf`](#modf) | mod |  |
| [`modff`](#modff) | mod |  |
| [`nextafter`](#nextafter) | mod |  |
| [`nextafterf`](#nextafterf) | mod |  |
| [`pow`](#pow) | mod |  |
| [`powf`](#powf) | mod |  |
| [`remainder`](#remainder) | mod |  |
| [`remainderf`](#remainderf) | mod |  |
| [`remquo`](#remquo) | mod |  |
| [`remquof`](#remquof) | mod |  |
| [`rint`](#rint) | mod |  |
| [`round`](#round) | mod |  |
| [`roundeven`](#roundeven) | mod |  |
| [`scalbn`](#scalbn) | mod |  |
| [`sin`](#sin) | mod |  |
| [`sincos`](#sincos) | mod |  |
| [`sincosf`](#sincosf) | mod |  |
| [`sinf`](#sinf) | mod |  |
| [`sinh`](#sinh) | mod |  |
| [`sinhf`](#sinhf) | mod |  |
| [`sqrt`](#sqrt) | mod |  |
| [`tan`](#tan) | mod |  |
| [`tanf`](#tanf) | mod |  |
| [`tanh`](#tanh) | mod |  |
| [`tanhf`](#tanhf) | mod |  |
| [`tgamma`](#tgamma) | mod |  |
| [`tgammaf`](#tgammaf) | mod |  |
| [`trunc`](#trunc) | mod |  |
| [`Libm`](#libm) | struct | Generic helper for libm functions, abstracting over f32 and f64. |
| [`get_high_word`](#get-high-word) | fn |  |
| [`get_low_word`](#get-low-word) | fn |  |
| [`with_set_high_word`](#with-set-high-word) | fn |  |
| [`with_set_low_word`](#with-set-low-word) | fn |  |
| [`combine_words`](#combine-words) | fn |  |
| [`libm_helper!`](#libm-helper) | macro |  |
| [`force_eval!`](#force-eval) | macro |  |
| [`i!`](#i) | macro |  |
| [`div!`](#div) | macro |  |

## Modules

- [`libm_helper`](libm_helper/index.md)
- [`math`](math/index.md)
- [`support`](support/index.md)
- [`generic`](generic/index.md)
- [`arch`](arch/index.md) — Architecture-specific routines and operations.
- [`expo2`](expo2/index.md)
- [`k_cos`](k_cos/index.md)
- [`k_cosf`](k_cosf/index.md)
- [`k_expo2`](k_expo2/index.md)
- [`k_expo2f`](k_expo2f/index.md)
- [`k_sin`](k_sin/index.md)
- [`k_sinf`](k_sinf/index.md)
- [`k_tan`](k_tan/index.md)
- [`k_tanf`](k_tanf/index.md)
- [`rem_pio2`](rem_pio2/index.md)
- [`rem_pio2_large`](rem_pio2_large/index.md)
- [`rem_pio2f`](rem_pio2f/index.md)
- [`acos`](acos/index.md)
- [`acosf`](acosf/index.md)
- [`acosh`](acosh/index.md)
- [`acoshf`](acoshf/index.md)
- [`asin`](asin/index.md)
- [`asinf`](asinf/index.md)
- [`asinh`](asinh/index.md)
- [`asinhf`](asinhf/index.md)
- [`atan`](atan/index.md)
- [`atan2`](atan2/index.md)
- [`atan2f`](atan2f/index.md)
- [`atanf`](atanf/index.md)
- [`atanh`](atanh/index.md)
- [`atanhf`](atanhf/index.md)
- [`cbrt`](cbrt/index.md)
- [`cbrtf`](cbrtf/index.md)
- [`ceil`](ceil/index.md)
- [`copysign`](copysign/index.md)
- [`cos`](cos/index.md)
- [`cosf`](cosf/index.md)
- [`cosh`](cosh/index.md)
- [`coshf`](coshf/index.md)
- [`erf`](erf/index.md)
- [`erff`](erff/index.md)
- [`exp`](exp/index.md)
- [`exp10`](exp10/index.md)
- [`exp10f`](exp10f/index.md)
- [`exp2`](exp2/index.md)
- [`exp2f`](exp2f/index.md)
- [`expf`](expf/index.md)
- [`expm1`](expm1/index.md)
- [`expm1f`](expm1f/index.md)
- [`fabs`](fabs/index.md)
- [`fdim`](fdim/index.md)
- [`floor`](floor/index.md)
- [`fma`](fma/index.md)
- [`fmin_fmax`](fmin_fmax/index.md)
- [`fminimum_fmaximum`](fminimum_fmaximum/index.md)
- [`fminimum_fmaximum_num`](fminimum_fmaximum_num/index.md)
- [`fmod`](fmod/index.md)
- [`frexp`](frexp/index.md)
- [`frexpf`](frexpf/index.md)
- [`hypot`](hypot/index.md)
- [`hypotf`](hypotf/index.md)
- [`ilogb`](ilogb/index.md)
- [`ilogbf`](ilogbf/index.md)
- [`j0`](j0/index.md)
- [`j0f`](j0f/index.md)
- [`j1`](j1/index.md)
- [`j1f`](j1f/index.md)
- [`jn`](jn/index.md)
- [`jnf`](jnf/index.md)
- [`ldexp`](ldexp/index.md)
- [`lgamma`](lgamma/index.md)
- [`lgamma_r`](lgamma_r/index.md)
- [`lgammaf`](lgammaf/index.md)
- [`lgammaf_r`](lgammaf_r/index.md)
- [`log`](log/index.md)
- [`log10`](log10/index.md)
- [`log10f`](log10f/index.md)
- [`log1p`](log1p/index.md)
- [`log1pf`](log1pf/index.md)
- [`log2`](log2/index.md)
- [`log2f`](log2f/index.md)
- [`logf`](logf/index.md)
- [`modf`](modf/index.md)
- [`modff`](modff/index.md)
- [`nextafter`](nextafter/index.md)
- [`nextafterf`](nextafterf/index.md)
- [`pow`](pow/index.md)
- [`powf`](powf/index.md)
- [`remainder`](remainder/index.md)
- [`remainderf`](remainderf/index.md)
- [`remquo`](remquo/index.md)
- [`remquof`](remquof/index.md)
- [`rint`](rint/index.md)
- [`round`](round/index.md)
- [`roundeven`](roundeven/index.md)
- [`scalbn`](scalbn/index.md)
- [`sin`](sin/index.md)
- [`sincos`](sincos/index.md)
- [`sincosf`](sincosf/index.md)
- [`sinf`](sinf/index.md)
- [`sinh`](sinh/index.md)
- [`sinhf`](sinhf/index.md)
- [`sqrt`](sqrt/index.md)
- [`tan`](tan/index.md)
- [`tanf`](tanf/index.md)
- [`tanh`](tanh/index.md)
- [`tanhf`](tanhf/index.md)
- [`tgamma`](tgamma/index.md)
- [`tgammaf`](tgammaf/index.md)
- [`trunc`](trunc/index.md)

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

## Functions

### `get_high_word`

```rust
fn get_high_word(x: f64) -> u32
```

### `get_low_word`

```rust
fn get_low_word(x: f64) -> u32
```

### `with_set_high_word`

```rust
fn with_set_high_word(f: f64, hi: u32) -> f64
```

### `with_set_low_word`

```rust
fn with_set_low_word(f: f64, lo: u32) -> f64
```

### `combine_words`

```rust
fn combine_words(hi: u32, lo: u32) -> f64
```

## Macros

### `libm_helper!`

### `force_eval!`

### `i!`

### `div!`

