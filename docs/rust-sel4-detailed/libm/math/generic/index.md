*[libm](../../index.md) / [math](../index.md) / [generic](index.md)*

---

# Module `generic`

## Contents

- [Modules](#modules)
  - [`ceil`](#ceil)
  - [`copysign`](#copysign)
  - [`fabs`](#fabs)
  - [`fdim`](#fdim)
  - [`floor`](#floor)
  - [`fma`](#fma)
  - [`fma_wide`](#fma-wide)
  - [`fmax`](#fmax)
  - [`fmaximum`](#fmaximum)
  - [`fmaximum_num`](#fmaximum-num)
  - [`fmin`](#fmin)
  - [`fminimum`](#fminimum)
  - [`fminimum_num`](#fminimum-num)
  - [`fmod`](#fmod)
  - [`rint`](#rint)
  - [`round`](#round)
  - [`scalbn`](#scalbn)
  - [`sqrt`](#sqrt)
  - [`trunc`](#trunc)
- [Functions](#functions)
  - [`ceil`](#ceil)
  - [`copysign`](#copysign)
  - [`fabs`](#fabs)
  - [`fdim`](#fdim)
  - [`floor`](#floor)
  - [`fma_round`](#fma-round)
  - [`fma_wide_round`](#fma-wide-round)
  - [`fmax`](#fmax)
  - [`fmaximum`](#fmaximum)
  - [`fmaximum_num`](#fmaximum-num)
  - [`fmin`](#fmin)
  - [`fminimum`](#fminimum)
  - [`fminimum_num`](#fminimum-num)
  - [`fmod`](#fmod)
  - [`rint_round`](#rint-round)
  - [`round`](#round)
  - [`scalbn`](#scalbn)
  - [`sqrt`](#sqrt)
  - [`trunc`](#trunc)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ceil`](#ceil) | mod | Generic `ceil` algorithm. |
| [`copysign`](#copysign) | mod |  |
| [`fabs`](#fabs) | mod |  |
| [`fdim`](#fdim) | mod |  |
| [`floor`](#floor) | mod | Generic `floor` algorithm. |
| [`fma`](#fma) | mod |  |
| [`fma_wide`](#fma-wide) | mod |  |
| [`fmax`](#fmax) | mod | IEEE 754-2011 `maxNum`. |
| [`fmaximum`](#fmaximum) | mod | IEEE 754-2019 `maximum`. |
| [`fmaximum_num`](#fmaximum-num) | mod | IEEE 754-2019 `maximumNumber`. |
| [`fmin`](#fmin) | mod | IEEE 754-2008 `minNum`. |
| [`fminimum`](#fminimum) | mod | IEEE 754-2019 `minimum`. |
| [`fminimum_num`](#fminimum-num) | mod | IEEE 754-2019 `minimum`. |
| [`fmod`](#fmod) | mod |  |
| [`rint`](#rint) | mod |  |
| [`round`](#round) | mod |  |
| [`scalbn`](#scalbn) | mod |  |
| [`sqrt`](#sqrt) | mod | Generic square root algorithm. |
| [`trunc`](#trunc) | mod |  |
| [`ceil`](#ceil) | fn |  |
| [`copysign`](#copysign) | fn |  |
| [`fabs`](#fabs) | fn |  |
| [`fdim`](#fdim) | fn |  |
| [`floor`](#floor) | fn |  |
| [`fma_round`](#fma-round) | fn |  |
| [`fma_wide_round`](#fma-wide-round) | fn |  |
| [`fmax`](#fmax) | fn |  |
| [`fmaximum`](#fmaximum) | fn |  |
| [`fmaximum_num`](#fmaximum-num) | fn |  |
| [`fmin`](#fmin) | fn |  |
| [`fminimum`](#fminimum) | fn |  |
| [`fminimum_num`](#fminimum-num) | fn |  |
| [`fmod`](#fmod) | fn |  |
| [`rint_round`](#rint-round) | fn |  |
| [`round`](#round) | fn |  |
| [`scalbn`](#scalbn) | fn |  |
| [`sqrt`](#sqrt) | fn |  |
| [`trunc`](#trunc) | fn |  |

## Modules

- [`ceil`](ceil/index.md) — Generic `ceil` algorithm.
- [`copysign`](copysign/index.md)
- [`fabs`](fabs/index.md)
- [`fdim`](fdim/index.md)
- [`floor`](floor/index.md) — Generic `floor` algorithm.
- [`fma`](fma/index.md)
- [`fma_wide`](fma_wide/index.md)
- [`fmax`](fmax/index.md) — IEEE 754-2011 `maxNum`. This has been superseded by IEEE 754-2019 `maximumNumber`.
- [`fmaximum`](fmaximum/index.md) — IEEE 754-2019 `maximum`.
- [`fmaximum_num`](fmaximum_num/index.md) — IEEE 754-2019 `maximumNumber`.
- [`fmin`](fmin/index.md) — IEEE 754-2008 `minNum`. This has been superseded by IEEE 754-2019 `minimumNumber`.
- [`fminimum`](fminimum/index.md) — IEEE 754-2019 `minimum`.
- [`fminimum_num`](fminimum_num/index.md) — IEEE 754-2019 `minimum`.
- [`fmod`](fmod/index.md)
- [`rint`](rint/index.md)
- [`round`](round/index.md)
- [`scalbn`](scalbn/index.md)
- [`sqrt`](sqrt/index.md) — Generic square root algorithm.
- [`trunc`](trunc/index.md)

## Functions

### `ceil`

```rust
fn ceil<F: Float>(x: F) -> F
```

### `copysign`

```rust
fn copysign<F: Float>(x: F, y: F) -> F
```

Copy the sign of `y` to `x`.

### `fabs`

```rust
fn fabs<F: Float>(x: F) -> F
```

Absolute value.

### `fdim`

```rust
fn fdim<F: Float>(x: F, y: F) -> F
```

### `floor`

```rust
fn floor<F: Float>(x: F) -> F
```

### `fma_round`

```rust
fn fma_round<F>(x: F, y: F, z: F, _round: crate::support::Round) -> crate::support::FpResult<F>
where
    F: Float + CastFrom<<F as >::SignedInt> + CastFrom<i8>,
    <F as >::Int: HInt,
    u32: CastInto<<F as >::Int>
```

Fused multiply-add that works when there is not a larger float size available. Computes
`(x * y) + z`.

### `fma_wide_round`

```rust
fn fma_wide_round<F, B>(x: F, y: F, z: F, round: crate::support::Round) -> crate::support::FpResult<F>
where
    F: Float + HFloat<D = B>,
    B: Float + DFloat<H = F>,
    <B as >::Int: CastInto<i32>,
    i32: CastFrom<i32>
```

Fma implementation when a hardware-backed larger float type is available. For `f32` and `f64`,
`f64` has enough precision to represent the `f32` in its entirety, except for double rounding.

### `fmax`

```rust
fn fmax<F: Float>(x: F, y: F) -> F
```

### `fmaximum`

```rust
fn fmaximum<F: Float>(x: F, y: F) -> F
```

### `fmaximum_num`

```rust
fn fmaximum_num<F: Float>(x: F, y: F) -> F
```

### `fmin`

```rust
fn fmin<F: Float>(x: F, y: F) -> F
```

### `fminimum`

```rust
fn fminimum<F: Float>(x: F, y: F) -> F
```

### `fminimum_num`

```rust
fn fminimum_num<F: Float>(x: F, y: F) -> F
```

### `fmod`

```rust
fn fmod<F: Float>(x: F, y: F) -> F
where
    <F as >::Int: HInt,
    <<F as >::Int as HInt>::D: NarrowingDiv
```

### `rint_round`

```rust
fn rint_round<F: Float>(x: F, _round: crate::support::Round) -> crate::support::FpResult<F>
```

IEEE 754-2019 `roundToIntegralExact`, which respects rounding mode and raises inexact if
applicable.

### `round`

```rust
fn round<F: Float>(x: F) -> F
```

### `scalbn`

```rust
fn scalbn<F: Float>(x: F, n: i32) -> F
where
    u32: CastInto<<F as >::Int>,
    <F as >::Int: CastFrom<i32> + CastFrom<u32>
```

Scale the exponent.

From N3220:

> The scalbn and scalbln functions compute `x * b^n`, where `b = FLT_RADIX` if the return type
> of the function is a standard floating type, or `b = 10` if the return type of the function
> is a decimal floating type. A range error occurs for some finite x, depending on n.
>
> [...]
>
> * `scalbn(±0, n)` returns `±0`.
> * `scalbn(x, 0)` returns `x`.
> * `scalbn(±∞, n)` returns `±∞`.
>
> If the calculation does not overflow or underflow, the returned value is exact and
> independent of the current rounding direction mode.

### `sqrt`

```rust
fn sqrt<F>(x: F) -> F
where
    F: Float + SqrtHelper,
    <F as >::Int: HInt + From<u8> + From<<F as >::ISet2> + CastInto<<F as >::ISet1> + CastInto<<F as >::ISet2>,
    u32: CastInto<<F as >::Int>
```

### `trunc`

```rust
fn trunc<F: Float>(x: F) -> F
```

