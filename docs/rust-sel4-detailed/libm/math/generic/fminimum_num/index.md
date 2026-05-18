*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [fminimum_num](index.md)*

---

# Module `fminimum_num`

IEEE 754-2019 `minimum`.

Per the spec, returns:
- `x` if `x < y`
- `y` if `y < x`
- -0.0 if x and y are zero with opposite signs
- Either `x` or `y` if `x == y` and the signs are the same
- Non-NaN if one operand is NaN
- qNaN if both operands are NaNx

Excluded from our implementation is sNaN handling.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fminimum_num`](#fminimum-num) | fn |  |

## Functions

### `fminimum_num`

```rust
fn fminimum_num<F: Float>(x: F, y: F) -> F
```

