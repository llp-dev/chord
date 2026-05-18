*[libm](../../index.md) / [math](../index.md) / [atan2](index.md)*

---

# Module `atan2`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`atan2`](#atan2) | fn | Arctangent of y/x (f64) |
| [`PI`](#pi) | const |  |
| [`PI_LO`](#pi-lo) | const |  |

## Functions

### `atan2`

```rust
fn atan2(y: f64, x: f64) -> f64
```

Arctangent of y/x (f64)

Computes the inverse tangent (arc tangent) of `y/x`.
Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
Returns a value in radians, in the range of -pi to pi.

## Constants

### `PI`
```rust
const PI: f64 = 3.1415926535897931f64;
```

### `PI_LO`
```rust
const PI_LO: f64 = 1.2246467991473532E-16f64;
```

