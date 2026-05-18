*[libm](../../index.md) / [math](../index.md) / [fminimum_fmaximum_num](index.md)*

---

# Module `fminimum_fmaximum_num`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fminimum_numf`](#fminimum-numf) | fn | Return the lesser of two arguments or, if either argument is NaN, NaN. |
| [`fminimum_num`](#fminimum-num) | fn | Return the lesser of two arguments or, if either argument is NaN, NaN. |
| [`fmaximum_numf`](#fmaximum-numf) | fn | Return the greater of two arguments or, if either argument is NaN, NaN. |
| [`fmaximum_num`](#fmaximum-num) | fn | Return the greater of two arguments or, if either argument is NaN, NaN. |

## Functions

### `fminimum_numf`

```rust
fn fminimum_numf(x: f32, y: f32) -> f32
```

Return the lesser of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0.

### `fminimum_num`

```rust
fn fminimum_num(x: f64, y: f64) -> f64
```

Return the lesser of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0.

### `fmaximum_numf`

```rust
fn fmaximum_numf(x: f32, y: f32) -> f32
```

Return the greater of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0.

### `fmaximum_num`

```rust
fn fmaximum_num(x: f64, y: f64) -> f64
```

Return the greater of two arguments or, if either argument is NaN, NaN.

This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0.

