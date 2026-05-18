**libm > math > fminimum_fmaximum_num**

# Module: math::fminimum_fmaximum_num

## Contents

**Functions**

- [`fmaximum_num`](#fmaximum_num) - Return the greater of two arguments or, if either argument is NaN, NaN.
- [`fmaximum_numf`](#fmaximum_numf) - Return the greater of two arguments or, if either argument is NaN, NaN.
- [`fminimum_num`](#fminimum_num) - Return the lesser of two arguments or, if either argument is NaN, NaN.
- [`fminimum_numf`](#fminimum_numf) - Return the lesser of two arguments or, if either argument is NaN, NaN.

---

## libm::math::fminimum_fmaximum_num::fmaximum_num

*Function*

Return the greater of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0.

```rust
fn fmaximum_num(x: f64, y: f64) -> f64
```



## libm::math::fminimum_fmaximum_num::fmaximum_numf

*Function*

Return the greater of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0.

```rust
fn fmaximum_numf(x: f32, y: f32) -> f32
```



## libm::math::fminimum_fmaximum_num::fminimum_num

*Function*

Return the lesser of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0.

```rust
fn fminimum_num(x: f64, y: f64) -> f64
```



## libm::math::fminimum_fmaximum_num::fminimum_numf

*Function*

Return the lesser of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0.

```rust
fn fminimum_numf(x: f32, y: f32) -> f32
```



