**libm > math > fmin_fmax**

# Module: math::fmin_fmax

## Contents

**Functions**

- [`fmax`](#fmax) - Return the greater of two arguments or, if either argument is NaN, the other argument.
- [`fmaxf`](#fmaxf) - Return the greater of two arguments or, if either argument is NaN, the other argument.
- [`fmin`](#fmin) - Return the lesser of two arguments or, if either argument is NaN, the other argument.
- [`fminf`](#fminf) - Return the lesser of two arguments or, if either argument is NaN, the other argument.

---

## libm::math::fmin_fmax::fmax

*Function*

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

```rust
fn fmax(x: f64, y: f64) -> f64
```



## libm::math::fmin_fmax::fmaxf

*Function*

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

```rust
fn fmaxf(x: f32, y: f32) -> f32
```



## libm::math::fmin_fmax::fmin

*Function*

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

```rust
fn fmin(x: f64, y: f64) -> f64
```



## libm::math::fmin_fmax::fminf

*Function*

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

```rust
fn fminf(x: f32, y: f32) -> f32
```



