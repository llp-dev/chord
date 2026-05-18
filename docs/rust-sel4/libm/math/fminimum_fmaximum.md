**libm > math > fminimum_fmaximum**

# Module: math::fminimum_fmaximum

## Contents

**Functions**

- [`fmaximum`](#fmaximum) - Return the greater of two arguments or, if either argument is NaN, the other argument.
- [`fmaximumf`](#fmaximumf) - Return the greater of two arguments or, if either argument is NaN, the other argument.
- [`fminimum`](#fminimum) - Return the lesser of two arguments or, if either argument is NaN, the other argument.
- [`fminimumf`](#fminimumf) - Return the lesser of two arguments or, if either argument is NaN, the other argument.

---

## libm::math::fminimum_fmaximum::fmaximum

*Function*

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0.

```rust
fn fmaximum(x: f64, y: f64) -> f64
```



## libm::math::fminimum_fmaximum::fmaximumf

*Function*

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0.

```rust
fn fmaximumf(x: f32, y: f32) -> f32
```



## libm::math::fminimum_fmaximum::fminimum

*Function*

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0.

```rust
fn fminimum(x: f64, y: f64) -> f64
```



## libm::math::fminimum_fmaximum::fminimumf

*Function*

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0.

```rust
fn fminimumf(x: f32, y: f32) -> f32
```



