*[libm](../../index.md) / [math](../index.md) / [atanh](index.md)*

---

# Module `atanh`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`atanh`](#atanh) | fn | Inverse hyperbolic tangent (f64) |

## Functions

### `atanh`

```rust
fn atanh(x: f64) -> f64
```

Inverse hyperbolic tangent (f64)

Calculates the inverse hyperbolic tangent of `x`.
Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.

