*[libm](../../index.md) / [math](../index.md) / [fmin_fmax](index.md)*

---

# Module `fmin_fmax`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fminf`](#fminf) | fn | Return the lesser of two arguments or, if either argument is NaN, the other argument. |
| [`fmin`](#fmin) | fn | Return the lesser of two arguments or, if either argument is NaN, the other argument. |
| [`fmaxf`](#fmaxf) | fn | Return the greater of two arguments or, if either argument is NaN, the other argument. |
| [`fmax`](#fmax) | fn | Return the greater of two arguments or, if either argument is NaN, the other argument. |

## Functions

### `fminf`

```rust
fn fminf(x: f32, y: f32) -> f32
```

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

### `fmin`

```rust
fn fmin(x: f64, y: f64) -> f64
```

Return the lesser of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

### `fmaxf`

```rust
fn fmaxf(x: f32, y: f32) -> f32
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

### `fmax`

```rust
fn fmax(x: f64, y: f64) -> f64
```

Return the greater of two arguments or, if either argument is NaN, the other argument.

This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if
the inputs are -0.0 and +0.0, either may be returned).

