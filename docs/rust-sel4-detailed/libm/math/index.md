*[libm](../index.md) / [math](index.md)*

---

# Module `math`

## Contents

- [Modules](#modules)
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
- [Functions](#functions)
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
  - [`ceilf`](#ceilf)
  - [`copysign`](#copysign)
  - [`copysignf`](#copysignf)
  - [`cos`](#cos)
  - [`cosf`](#cosf)
  - [`cosh`](#cosh)
  - [`coshf`](#coshf)
  - [`erf`](#erf)
  - [`erfc`](#erfc)
  - [`erfcf`](#erfcf)
  - [`erff`](#erff)
  - [`exp`](#exp)
  - [`exp2`](#exp2)
  - [`exp2f`](#exp2f)
  - [`exp10`](#exp10)
  - [`exp10f`](#exp10f)
  - [`expf`](#expf)
  - [`expm1`](#expm1)
  - [`expm1f`](#expm1f)
  - [`fabs`](#fabs)
  - [`fabsf`](#fabsf)
  - [`fdim`](#fdim)
  - [`fdimf`](#fdimf)
  - [`floor`](#floor)
  - [`floorf`](#floorf)
  - [`fma`](#fma)
  - [`fmaf`](#fmaf)
  - [`fmax`](#fmax)
  - [`fmaxf`](#fmaxf)
  - [`fmin`](#fmin)
  - [`fminf`](#fminf)
  - [`fmaximum`](#fmaximum)
  - [`fmaximumf`](#fmaximumf)
  - [`fminimum`](#fminimum)
  - [`fminimumf`](#fminimumf)
  - [`fmaximum_num`](#fmaximum-num)
  - [`fmaximum_numf`](#fmaximum-numf)
  - [`fminimum_num`](#fminimum-num)
  - [`fminimum_numf`](#fminimum-numf)
  - [`fmod`](#fmod)
  - [`fmodf`](#fmodf)
  - [`frexp`](#frexp)
  - [`frexpf`](#frexpf)
  - [`hypot`](#hypot)
  - [`hypotf`](#hypotf)
  - [`ilogb`](#ilogb)
  - [`ilogbf`](#ilogbf)
  - [`j0`](#j0)
  - [`y0`](#y0)
  - [`j0f`](#j0f)
  - [`y0f`](#y0f)
  - [`j1`](#j1)
  - [`y1`](#y1)
  - [`j1f`](#j1f)
  - [`y1f`](#y1f)
  - [`jn`](#jn)
  - [`yn`](#yn)
  - [`jnf`](#jnf)
  - [`ynf`](#ynf)
  - [`ldexp`](#ldexp)
  - [`ldexpf`](#ldexpf)
  - [`lgamma`](#lgamma)
  - [`lgamma_r`](#lgamma-r)
  - [`lgammaf`](#lgammaf)
  - [`lgammaf_r`](#lgammaf-r)
  - [`log`](#log)
  - [`log1p`](#log1p)
  - [`log1pf`](#log1pf)
  - [`log2`](#log2)
  - [`log2f`](#log2f)
  - [`log10`](#log10)
  - [`log10f`](#log10f)
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
  - [`rintf`](#rintf)
  - [`round`](#round)
  - [`roundf`](#roundf)
  - [`roundeven`](#roundeven)
  - [`roundevenf`](#roundevenf)
  - [`scalbn`](#scalbn)
  - [`scalbnf`](#scalbnf)
  - [`sin`](#sin)
  - [`sincos`](#sincos)
  - [`sincosf`](#sincosf)
  - [`sinf`](#sinf)
  - [`sinh`](#sinh)
  - [`sinhf`](#sinhf)
  - [`sqrt`](#sqrt)
  - [`sqrtf`](#sqrtf)
  - [`tan`](#tan)
  - [`tanf`](#tanf)
  - [`tanh`](#tanh)
  - [`tanhf`](#tanhf)
  - [`tgamma`](#tgamma)
  - [`tgammaf`](#tgammaf)
  - [`trunc`](#trunc)
  - [`truncf`](#truncf)
  - [`get_high_word`](#get-high-word)
  - [`get_low_word`](#get-low-word)
  - [`with_set_high_word`](#with-set-high-word)
  - [`with_set_low_word`](#with-set-low-word)
  - [`combine_words`](#combine-words)
- [Macros](#macros)
  - [`force_eval!`](#force-eval)
  - [`i!`](#i)
  - [`div!`](#div)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
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
| [`acos`](#acos) | fn |  |
| [`acosf`](#acosf) | fn |  |
| [`acosh`](#acosh) | fn |  |
| [`acoshf`](#acoshf) | fn |  |
| [`asin`](#asin) | fn |  |
| [`asinf`](#asinf) | fn |  |
| [`asinh`](#asinh) | fn |  |
| [`asinhf`](#asinhf) | fn |  |
| [`atan`](#atan) | fn |  |
| [`atan2`](#atan2) | fn |  |
| [`atan2f`](#atan2f) | fn |  |
| [`atanf`](#atanf) | fn |  |
| [`atanh`](#atanh) | fn |  |
| [`atanhf`](#atanhf) | fn |  |
| [`cbrt`](#cbrt) | fn |  |
| [`cbrtf`](#cbrtf) | fn |  |
| [`ceil`](#ceil) | fn |  |
| [`ceilf`](#ceilf) | fn |  |
| [`copysign`](#copysign) | fn |  |
| [`copysignf`](#copysignf) | fn |  |
| [`cos`](#cos) | fn |  |
| [`cosf`](#cosf) | fn |  |
| [`cosh`](#cosh) | fn |  |
| [`coshf`](#coshf) | fn |  |
| [`erf`](#erf) | fn |  |
| [`erfc`](#erfc) | fn |  |
| [`erfcf`](#erfcf) | fn |  |
| [`erff`](#erff) | fn |  |
| [`exp`](#exp) | fn |  |
| [`exp2`](#exp2) | fn |  |
| [`exp2f`](#exp2f) | fn |  |
| [`exp10`](#exp10) | fn |  |
| [`exp10f`](#exp10f) | fn |  |
| [`expf`](#expf) | fn |  |
| [`expm1`](#expm1) | fn |  |
| [`expm1f`](#expm1f) | fn |  |
| [`fabs`](#fabs) | fn |  |
| [`fabsf`](#fabsf) | fn |  |
| [`fdim`](#fdim) | fn |  |
| [`fdimf`](#fdimf) | fn |  |
| [`floor`](#floor) | fn |  |
| [`floorf`](#floorf) | fn |  |
| [`fma`](#fma) | fn |  |
| [`fmaf`](#fmaf) | fn |  |
| [`fmax`](#fmax) | fn |  |
| [`fmaxf`](#fmaxf) | fn |  |
| [`fmin`](#fmin) | fn |  |
| [`fminf`](#fminf) | fn |  |
| [`fmaximum`](#fmaximum) | fn |  |
| [`fmaximumf`](#fmaximumf) | fn |  |
| [`fminimum`](#fminimum) | fn |  |
| [`fminimumf`](#fminimumf) | fn |  |
| [`fmaximum_num`](#fmaximum-num) | fn |  |
| [`fmaximum_numf`](#fmaximum-numf) | fn |  |
| [`fminimum_num`](#fminimum-num) | fn |  |
| [`fminimum_numf`](#fminimum-numf) | fn |  |
| [`fmod`](#fmod) | fn |  |
| [`fmodf`](#fmodf) | fn |  |
| [`frexp`](#frexp) | fn |  |
| [`frexpf`](#frexpf) | fn |  |
| [`hypot`](#hypot) | fn |  |
| [`hypotf`](#hypotf) | fn |  |
| [`ilogb`](#ilogb) | fn |  |
| [`ilogbf`](#ilogbf) | fn |  |
| [`j0`](#j0) | fn |  |
| [`y0`](#y0) | fn |  |
| [`j0f`](#j0f) | fn |  |
| [`y0f`](#y0f) | fn |  |
| [`j1`](#j1) | fn |  |
| [`y1`](#y1) | fn |  |
| [`j1f`](#j1f) | fn |  |
| [`y1f`](#y1f) | fn |  |
| [`jn`](#jn) | fn |  |
| [`yn`](#yn) | fn |  |
| [`jnf`](#jnf) | fn |  |
| [`ynf`](#ynf) | fn |  |
| [`ldexp`](#ldexp) | fn |  |
| [`ldexpf`](#ldexpf) | fn |  |
| [`lgamma`](#lgamma) | fn |  |
| [`lgamma_r`](#lgamma-r) | fn |  |
| [`lgammaf`](#lgammaf) | fn |  |
| [`lgammaf_r`](#lgammaf-r) | fn |  |
| [`log`](#log) | fn |  |
| [`log1p`](#log1p) | fn |  |
| [`log1pf`](#log1pf) | fn |  |
| [`log2`](#log2) | fn |  |
| [`log2f`](#log2f) | fn |  |
| [`log10`](#log10) | fn |  |
| [`log10f`](#log10f) | fn |  |
| [`logf`](#logf) | fn |  |
| [`modf`](#modf) | fn |  |
| [`modff`](#modff) | fn |  |
| [`nextafter`](#nextafter) | fn |  |
| [`nextafterf`](#nextafterf) | fn |  |
| [`pow`](#pow) | fn |  |
| [`powf`](#powf) | fn |  |
| [`remainder`](#remainder) | fn |  |
| [`remainderf`](#remainderf) | fn |  |
| [`remquo`](#remquo) | fn |  |
| [`remquof`](#remquof) | fn |  |
| [`rint`](#rint) | fn |  |
| [`rintf`](#rintf) | fn |  |
| [`round`](#round) | fn |  |
| [`roundf`](#roundf) | fn |  |
| [`roundeven`](#roundeven) | fn |  |
| [`roundevenf`](#roundevenf) | fn |  |
| [`scalbn`](#scalbn) | fn |  |
| [`scalbnf`](#scalbnf) | fn |  |
| [`sin`](#sin) | fn |  |
| [`sincos`](#sincos) | fn |  |
| [`sincosf`](#sincosf) | fn |  |
| [`sinf`](#sinf) | fn |  |
| [`sinh`](#sinh) | fn |  |
| [`sinhf`](#sinhf) | fn |  |
| [`sqrt`](#sqrt) | fn |  |
| [`sqrtf`](#sqrtf) | fn |  |
| [`tan`](#tan) | fn |  |
| [`tanf`](#tanf) | fn |  |
| [`tanh`](#tanh) | fn |  |
| [`tanhf`](#tanhf) | fn |  |
| [`tgamma`](#tgamma) | fn |  |
| [`tgammaf`](#tgammaf) | fn |  |
| [`trunc`](#trunc) | fn |  |
| [`truncf`](#truncf) | fn |  |
| [`get_high_word`](#get-high-word) | fn |  |
| [`get_low_word`](#get-low-word) | fn |  |
| [`with_set_high_word`](#with-set-high-word) | fn |  |
| [`with_set_low_word`](#with-set-low-word) | fn |  |
| [`combine_words`](#combine-words) | fn |  |
| [`force_eval!`](#force-eval) | macro |  |
| [`i!`](#i) | macro |  |
| [`div!`](#div) | macro |  |

## Modules

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

## Functions

### `acos`

```rust
fn acos(x: f64) -> f64
```

Arccosine (f64)

Computes the inverse cosine (arc cosine) of the input value.
Arguments must be in the range -1 to 1.
Returns values in radians, in the range of 0 to pi.

### `acosf`

```rust
fn acosf(x: f32) -> f32
```

Arccosine (f32)

Computes the inverse cosine (arc cosine) of the input value.
Arguments must be in the range -1 to 1.
Returns values in radians, in the range of 0 to pi.

### `acosh`

```rust
fn acosh(x: f64) -> f64
```

Inverse hyperbolic cosine (f64)

Calculates the inverse hyperbolic cosine of `x`.
Is defined as `log(x + sqrt(x*x-1))`.
`x` must be a number greater than or equal to 1.

### `acoshf`

```rust
fn acoshf(x: f32) -> f32
```

Inverse hyperbolic cosine (f32)

Calculates the inverse hyperbolic cosine of `x`.
Is defined as `log(x + sqrt(x*x-1))`.
`x` must be a number greater than or equal to 1.

### `asin`

```rust
fn asin(x: f64) -> f64
```

Arcsine (f64)

Computes the inverse sine (arc sine) of the argument `x`.
Arguments to asin must be in the range -1 to 1.
Returns values in radians, in the range of -pi/2 to pi/2.

### `asinf`

```rust
fn asinf(x: f32) -> f32
```

Arcsine (f32)

Computes the inverse sine (arc sine) of the argument `x`.
Arguments to asin must be in the range -1 to 1.
Returns values in radians, in the range of -pi/2 to pi/2.

### `asinh`

```rust
fn asinh(x: f64) -> f64
```

Inverse hyperbolic sine (f64)

Calculates the inverse hyperbolic sine of `x`.
Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`.

### `asinhf`

```rust
fn asinhf(x: f32) -> f32
```

Inverse hyperbolic sine (f32)

Calculates the inverse hyperbolic sine of `x`.
Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`.

### `atan`

```rust
fn atan(x: f64) -> f64
```

Arctangent (f64)

Computes the inverse tangent (arc tangent) of the input value.
Returns a value in radians, in the range of -pi/2 to pi/2.

### `atan2`

```rust
fn atan2(y: f64, x: f64) -> f64
```

Arctangent of y/x (f64)

Computes the inverse tangent (arc tangent) of `y/x`.
Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
Returns a value in radians, in the range of -pi to pi.

### `atan2f`

```rust
fn atan2f(y: f32, x: f32) -> f32
```

Arctangent of y/x (f32)

Computes the inverse tangent (arc tangent) of `y/x`.
Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
Returns a value in radians, in the range of -pi to pi.

### `atanf`

```rust
fn atanf(x: f32) -> f32
```

Arctangent (f32)

Computes the inverse tangent (arc tangent) of the input value.
Returns a value in radians, in the range of -pi/2 to pi/2.

### `atanh`

```rust
fn atanh(x: f64) -> f64
```

Inverse hyperbolic tangent (f64)

Calculates the inverse hyperbolic tangent of `x`.
Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.

### `atanhf`

```rust
fn atanhf(x: f32) -> f32
```

Inverse hyperbolic tangent (f32)

Calculates the inverse hyperbolic tangent of `x`.
Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.

### `cbrt`

```rust
fn cbrt(x: f64) -> f64
```

Compute the cube root of the argument.

### `cbrtf`

```rust
fn cbrtf(x: f32) -> f32
```

Cube root (f32)

Computes the cube root of the argument.

### `ceil`

```rust
fn ceil(x: f64) -> f64
```

Ceil (f64)

Finds the nearest integer greater than or equal to `x`.

### `ceilf`

```rust
fn ceilf(x: f32) -> f32
```

Ceil (f32)

Finds the nearest integer greater than or equal to `x`.

### `copysign`

```rust
fn copysign(x: f64, y: f64) -> f64
```

Sign of Y, magnitude of X (f64)

Constructs a number with the magnitude (absolute value) of its
first argument, `x`, and the sign of its second argument, `y`.

### `copysignf`

```rust
fn copysignf(x: f32, y: f32) -> f32
```

Sign of Y, magnitude of X (f32)

Constructs a number with the magnitude (absolute value) of its
first argument, `x`, and the sign of its second argument, `y`.

### `cos`

```rust
fn cos(x: f64) -> f64
```

The cosine of `x` (f64).

`x` is specified in radians.

### `cosf`

```rust
fn cosf(x: f32) -> f32
```

The cosine of `x` (f32).

`x` is specified in radians.

### `cosh`

```rust
fn cosh(x: f64) -> f64
```

Hyperbolic cosine (f64)

Computes the hyperbolic cosine of the argument x.
Is defined as `(exp(x) + exp(-x))/2`
Angles are specified in radians.

### `coshf`

```rust
fn coshf(x: f32) -> f32
```

Hyperbolic cosine (f64)

Computes the hyperbolic cosine of the argument x.
Is defined as `(exp(x) + exp(-x))/2`
Angles are specified in radians.

### `erf`

```rust
fn erf(x: f64) -> f64
```

Error function (f64)

Calculates an approximation to the “error function”, which estimates
the probability that an observation will fall within x standard
deviations of the mean (assuming a normal distribution).

### `erfc`

```rust
fn erfc(x: f64) -> f64
```

Complementary error function (f64)

Calculates the complementary probability.
Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid
the loss of precision that would result from subtracting
large probabilities (on large `x`) from 1.

### `erfcf`

```rust
fn erfcf(x: f32) -> f32
```

Complementary error function (f32)

Calculates the complementary probability.
Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid
the loss of precision that would result from subtracting
large probabilities (on large `x`) from 1.

### `erff`

```rust
fn erff(x: f32) -> f32
```

Error function (f32)

Calculates an approximation to the “error function”, which estimates
the probability that an observation will fall within x standard
deviations of the mean (assuming a normal distribution).

### `exp`

```rust
fn exp(x: f64) -> f64
```

Exponential, base *e* (f64)

Calculate the exponential of `x`, that is, *e* raised to the power `x`
(where *e* is the base of the natural system of logarithms, approximately 2.71828).

### `exp2`

```rust
fn exp2(x: f64) -> f64
```

Exponential, base 2 (f64)

Calculate `2^x`, that is, 2 raised to the power `x`.

### `exp2f`

```rust
fn exp2f(x: f32) -> f32
```

Exponential, base 2 (f32)

Calculate `2^x`, that is, 2 raised to the power `x`.

### `exp10`

```rust
fn exp10(x: f64) -> f64
```

Calculates 10 raised to the power of `x` (f64).

### `exp10f`

```rust
fn exp10f(x: f32) -> f32
```

Calculates 10 raised to the power of `x` (f32).

### `expf`

```rust
fn expf(x: f32) -> f32
```

Exponential, base *e* (f32)

Calculate the exponential of `x`, that is, *e* raised to the power `x`
(where *e* is the base of the natural system of logarithms, approximately 2.71828).

### `expm1`

```rust
fn expm1(x: f64) -> f64
```

Exponential, base *e*, of x-1 (f64)

Calculates the exponential of `x` and subtract 1, that is, *e* raised
to the power `x` minus 1 (where *e* is the base of the natural
system of logarithms, approximately 2.71828).
The result is accurate even for small values of `x`,
where using `exp(x)-1` would lose many significant digits.

### `expm1f`

```rust
fn expm1f(x: f32) -> f32
```

Exponential, base *e*, of x-1 (f32)

Calculates the exponential of `x` and subtract 1, that is, *e* raised
to the power `x` minus 1 (where *e* is the base of the natural
system of logarithms, approximately 2.71828).
The result is accurate even for small values of `x`,
where using `exp(x)-1` would lose many significant digits.

### `fabs`

```rust
fn fabs(x: f64) -> f64
```

Absolute value (magnitude) (f64)

Calculates the absolute value (magnitude) of the argument `x`,
by direct manipulation of the bit representation of `x`.

### `fabsf`

```rust
fn fabsf(x: f32) -> f32
```

Absolute value (magnitude) (f32)

Calculates the absolute value (magnitude) of the argument `x`,
by direct manipulation of the bit representation of `x`.

### `fdim`

```rust
fn fdim(x: f64, y: f64) -> f64
```

Positive difference (f64)

Determines the positive difference between arguments, returning:
* x - y if x > y, or
* +0    if x <= y, or
* NAN   if either argument is NAN.

A range error may occur.

### `fdimf`

```rust
fn fdimf(x: f32, y: f32) -> f32
```

Positive difference (f32)

Determines the positive difference between arguments, returning:
* x - y if x > y, or
* +0    if x <= y, or
* NAN   if either argument is NAN.

A range error may occur.

### `floor`

```rust
fn floor(x: f64) -> f64
```

Floor (f64)

Finds the nearest integer less than or equal to `x`.

### `floorf`

```rust
fn floorf(x: f32) -> f32
```

Floor (f32)

Finds the nearest integer less than or equal to `x`.

### `fma`

```rust
fn fma(x: f64, y: f64, z: f64) -> f64
```

Fused multiply add (f64)

Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision).

### `fmaf`

```rust
fn fmaf(x: f32, y: f32, z: f32) -> f32
```

Floating multiply add (f32)

Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision).

### `fmax`

```rust
fn fmax(x: f64, y: f64) -> f64
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

### `fmaxf`

```rust
fn fmaxf(x: f32, y: f32) -> f32
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

### `fmin`

```rust
fn fmin(x: f64, y: f64) -> f64
```

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

### `fminf`

```rust
fn fminf(x: f32, y: f32) -> f32
```

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

### `fmaximum`

```rust
fn fmaximum(x: f64, y: f64) -> f64
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0.

### `fmaximumf`

```rust
fn fmaximumf(x: f32, y: f32) -> f32
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0.

### `fminimum`

```rust
fn fminimum(x: f64, y: f64) -> f64
```

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0.

### `fminimumf`

```rust
fn fminimumf(x: f32, y: f32) -> f32
```

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0.

### `fmaximum_num`

```rust
fn fmaximum_num(x: f64, y: f64) -> f64
```

Return the greater of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0.

### `fmaximum_numf`

```rust
fn fmaximum_numf(x: f32, y: f32) -> f32
```

Return the greater of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0.

### `fminimum_num`

```rust
fn fminimum_num(x: f64, y: f64) -> f64
```

Return the lesser of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0.

### `fminimum_numf`

```rust
fn fminimum_numf(x: f32, y: f32) -> f32
```

Return the lesser of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0.

### `fmod`

```rust
fn fmod(x: f64, y: f64) -> f64
```

Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`.

### `fmodf`

```rust
fn fmodf(x: f32, y: f32) -> f32
```

Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`.

### `frexp`

```rust
fn frexp(x: f64) -> (f64, i32)
```

### `frexpf`

```rust
fn frexpf(x: f32) -> (f32, i32)
```

### `hypot`

```rust
fn hypot(x: f64, y: f64) -> f64
```

### `hypotf`

```rust
fn hypotf(x: f32, y: f32) -> f32
```

### `ilogb`

```rust
fn ilogb(x: f64) -> i32
```

### `ilogbf`

```rust
fn ilogbf(x: f32) -> i32
```

### `j0`

```rust
fn j0(x: f64) -> f64
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64).

### `y0`

```rust
fn y0(x: f64) -> f64
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64).

### `j0f`

```rust
fn j0f(x: f32) -> f32
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32).

### `y0f`

```rust
fn y0f(x: f32) -> f32
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32).

### `j1`

```rust
fn j1(x: f64) -> f64
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64).

### `y1`

```rust
fn y1(x: f64) -> f64
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64).

### `j1f`

```rust
fn j1f(x: f32) -> f32
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32).

### `y1f`

```rust
fn y1f(x: f32) -> f32
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32).

### `jn`

```rust
fn jn(n: i32, x: f64) -> f64
```

Integer order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64).

### `yn`

```rust
fn yn(n: i32, x: f64) -> f64
```

Integer order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64).

### `jnf`

```rust
fn jnf(n: i32, x: f32) -> f32
```

Integer order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32).

### `ynf`

```rust
fn ynf(n: i32, x: f32) -> f32
```

Integer order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32).

### `ldexp`

```rust
fn ldexp(x: f64, n: i32) -> f64
```

### `ldexpf`

```rust
fn ldexpf(x: f32, n: i32) -> f32
```

### `lgamma`

```rust
fn lgamma(x: f64) -> f64
```

The natural logarithm of the
[Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f64).

### `lgamma_r`

```rust
fn lgamma_r(x: f64) -> (f64, i32)
```

### `lgammaf`

```rust
fn lgammaf(x: f32) -> f32
```

The natural logarithm of the
[Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f32).

### `lgammaf_r`

```rust
fn lgammaf_r(x: f32) -> (f32, i32)
```

### `log`

```rust
fn log(x: f64) -> f64
```

The natural logarithm of `x` (f64).

### `log1p`

```rust
fn log1p(x: f64) -> f64
```

The natural logarithm of 1+`x` (f64).

### `log1pf`

```rust
fn log1pf(x: f32) -> f32
```

The natural logarithm of 1+`x` (f32).

### `log2`

```rust
fn log2(x: f64) -> f64
```

The base 2 logarithm of `x` (f64).

### `log2f`

```rust
fn log2f(x: f32) -> f32
```

The base 2 logarithm of `x` (f32).

### `log10`

```rust
fn log10(x: f64) -> f64
```

The base 10 logarithm of `x` (f64).

### `log10f`

```rust
fn log10f(x: f32) -> f32
```

The base 10 logarithm of `x` (f32).

### `logf`

```rust
fn logf(x: f32) -> f32
```

The natural logarithm of `x` (f32).

### `modf`

```rust
fn modf(x: f64) -> (f64, f64)
```

### `modff`

```rust
fn modff(x: f32) -> (f32, f32)
```

### `nextafter`

```rust
fn nextafter(x: f64, y: f64) -> f64
```

### `nextafterf`

```rust
fn nextafterf(x: f32, y: f32) -> f32
```

### `pow`

```rust
fn pow(x: f64, y: f64) -> f64
```

Returns `x` to the power of `y` (f64).

### `powf`

```rust
fn powf(x: f32, y: f32) -> f32
```

Returns `x` to the power of `y` (f32).

### `remainder`

```rust
fn remainder(x: f64, y: f64) -> f64
```

### `remainderf`

```rust
fn remainderf(x: f32, y: f32) -> f32
```

### `remquo`

```rust
fn remquo(x: f64, y: f64) -> (f64, i32)
```

### `remquof`

```rust
fn remquof(x: f32, y: f32) -> (f32, i32)
```

### `rint`

```rust
fn rint(x: f64) -> f64
```

Round `x` to the nearest integer, breaking ties toward even.

### `rintf`

```rust
fn rintf(x: f32) -> f32
```

Round `x` to the nearest integer, breaking ties toward even.

### `round`

```rust
fn round(x: f64) -> f64
```

Round `x` to the nearest integer, breaking ties away from zero.

### `roundf`

```rust
fn roundf(x: f32) -> f32
```

Round `x` to the nearest integer, breaking ties away from zero.

### `roundeven`

```rust
fn roundeven(x: f64) -> f64
```

Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754
`roundToIntegralTiesToEven`.

### `roundevenf`

```rust
fn roundevenf(x: f32) -> f32
```

Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754
`roundToIntegralTiesToEven`.

### `scalbn`

```rust
fn scalbn(x: f64, n: i32) -> f64
```

### `scalbnf`

```rust
fn scalbnf(x: f32, n: i32) -> f32
```

### `sin`

```rust
fn sin(x: f64) -> f64
```

The sine of `x` (f64).

`x` is specified in radians.

### `sincos`

```rust
fn sincos(x: f64) -> (f64, f64)
```

Both the sine and cosine of `x` (f64).

`x` is specified in radians and the return value is (sin(x), cos(x)).

### `sincosf`

```rust
fn sincosf(x: f32) -> (f32, f32)
```

Both the sine and cosine of `x` (f32).

`x` is specified in radians and the return value is (sin(x), cos(x)).

### `sinf`

```rust
fn sinf(x: f32) -> f32
```

The sine of `x` (f32).

`x` is specified in radians.

### `sinh`

```rust
fn sinh(x: f64) -> f64
```

The hyperbolic sine of `x` (f64).

### `sinhf`

```rust
fn sinhf(x: f32) -> f32
```

The hyperbolic sine of `x` (f32).

### `sqrt`

```rust
fn sqrt(x: f64) -> f64
```

The square root of `x` (f64).

### `sqrtf`

```rust
fn sqrtf(x: f32) -> f32
```

The square root of `x` (f32).

### `tan`

```rust
fn tan(x: f64) -> f64
```

The tangent of `x` (f64).

`x` is specified in radians.

### `tanf`

```rust
fn tanf(x: f32) -> f32
```

The tangent of `x` (f32).

`x` is specified in radians.

### `tanh`

```rust
fn tanh(x: f64) -> f64
```

The hyperbolic tangent of `x` (f64).

`x` is specified in radians.

### `tanhf`

```rust
fn tanhf(x: f32) -> f32
```

The hyperbolic tangent of `x` (f32).

`x` is specified in radians.

### `tgamma`

```rust
fn tgamma(x: f64) -> f64
```

The [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f64).

### `tgammaf`

```rust
fn tgammaf(x: f32) -> f32
```

The [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f32).

### `trunc`

```rust
fn trunc(x: f64) -> f64
```

Rounds the number toward 0 to the closest integral value (f64).

This effectively removes the decimal part of the number, leaving the integral part.

### `truncf`

```rust
fn truncf(x: f32) -> f32
```

Rounds the number toward 0 to the closest integral value (f32).

This effectively removes the decimal part of the number, leaving the integral part.

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

### `force_eval!`

### `i!`

### `div!`

