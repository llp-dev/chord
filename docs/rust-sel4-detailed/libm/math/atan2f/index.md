*[libm](../../index.md) / [math](../index.md) / [atan2f](index.md)*

---

# Module `atan2f`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`atan2f`](#atan2f) | fn | Arctangent of y/x (f32) |
| [`PI`](#pi) | const |  |
| [`PI_LO`](#pi-lo) | const |  |

## Functions

### `atan2f`

```rust
fn atan2f(y: f32, x: f32) -> f32
```

Arctangent of y/x (f32)

Computes the inverse tangent (arc tangent) of `y/x`.
Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
Returns a value in radians, in the range of -pi to pi.

## Constants

### `PI`
```rust
const PI: f32 = 3.14159274f32;
```

### `PI_LO`
```rust
const PI_LO: f32 = -8.74227765E-8f32;
```

