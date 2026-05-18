*[libm](../../index.md) / [math](../index.md) / [acoshf](index.md)*

---

# Module `acoshf`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`acoshf`](#acoshf) | fn | Inverse hyperbolic cosine (f32) |
| [`LN2`](#ln2) | const |  |

## Functions

### `acoshf`

```rust
fn acoshf(x: f32) -> f32
```

Inverse hyperbolic cosine (f32)

Calculates the inverse hyperbolic cosine of `x`.
Is defined as `log(x + sqrt(x*x-1))`.
`x` must be a number greater than or equal to 1.

## Constants

### `LN2`
```rust
const LN2: f32 = 0.693147182f32;
```

