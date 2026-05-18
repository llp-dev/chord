*[libm](../../index.md) / [math](../index.md) / [asinh](index.md)*

---

# Module `asinh`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`asinh`](#asinh) | fn | Inverse hyperbolic sine (f64) |
| [`LN2`](#ln2) | const |  |

## Functions

### `asinh`

```rust
fn asinh(x: f64) -> f64
```

Inverse hyperbolic sine (f64)

Calculates the inverse hyperbolic sine of `x`.
Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`.

## Constants

### `LN2`
```rust
const LN2: f64 = 0.69314718055994529f64;
```

