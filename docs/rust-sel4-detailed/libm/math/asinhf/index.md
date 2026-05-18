*[libm](../../index.md) / [math](../index.md) / [asinhf](index.md)*

---

# Module `asinhf`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`asinhf`](#asinhf) | fn | Inverse hyperbolic sine (f32) |
| [`LN2`](#ln2) | const |  |

## Functions

### `asinhf`

```rust
fn asinhf(x: f32) -> f32
```

Inverse hyperbolic sine (f32)

Calculates the inverse hyperbolic sine of `x`.
Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`.

## Constants

### `LN2`
```rust
const LN2: f32 = 0.693147182f32;
```

