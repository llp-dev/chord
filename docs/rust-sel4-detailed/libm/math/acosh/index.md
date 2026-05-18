*[libm](../../index.md) / [math](../index.md) / [acosh](index.md)*

---

# Module `acosh`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`acosh`](#acosh) | fn | Inverse hyperbolic cosine (f64) |
| [`LN2`](#ln2) | const |  |

## Functions

### `acosh`

```rust
fn acosh(x: f64) -> f64
```

Inverse hyperbolic cosine (f64)

Calculates the inverse hyperbolic cosine of `x`.
Is defined as `log(x + sqrt(x*x-1))`.
`x` must be a number greater than or equal to 1.

## Constants

### `LN2`
```rust
const LN2: f64 = 0.69314718055994529f64;
```

