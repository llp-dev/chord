*[libm](../../index.md) / [math](../index.md) / [fminimum_fmaximum](index.md)*

---

# Module `fminimum_fmaximum`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fminimum`](#fminimum) | fn | Return the lesser of two arguments or, if either argument is NaN, the other argument. |
| [`fminimumf`](#fminimumf) | fn | Return the lesser of two arguments or, if either argument is NaN, the other argument. |
| [`fmaximumf`](#fmaximumf) | fn | Return the greater of two arguments or, if either argument is NaN, the other argument. |
| [`fmaximum`](#fmaximum) | fn | Return the greater of two arguments or, if either argument is NaN, the other argument. |

## Functions

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

### `fmaximumf`

```rust
fn fmaximumf(x: f32, y: f32) -> f32
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0.

### `fmaximum`

```rust
fn fmaximum(x: f64, y: f64) -> f64
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0.

