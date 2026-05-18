*[libm](../../index.md) / [math](../index.md) / [atanhf](index.md)*

---

# Module `atanhf`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`atanhf`](#atanhf) | fn | Inverse hyperbolic tangent (f32) |

## Functions

### `atanhf`

```rust
fn atanhf(x: f32) -> f32
```

Inverse hyperbolic tangent (f32)

Calculates the inverse hyperbolic tangent of `x`.
Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.

