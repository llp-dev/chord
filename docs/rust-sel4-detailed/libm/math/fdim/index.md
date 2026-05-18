*[libm](../../index.md) / [math](../index.md) / [fdim](index.md)*

---

# Module `fdim`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fdimf`](#fdimf) | fn | Positive difference (f32) |
| [`fdim`](#fdim) | fn | Positive difference (f64) |

## Functions

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

