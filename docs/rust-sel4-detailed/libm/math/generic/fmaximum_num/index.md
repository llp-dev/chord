*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [fmaximum_num](index.md)*

---

# Module `fmaximum_num`

IEEE 754-2019 `maximumNumber`.

Per the spec, returns:
- `x` if `x > y`
- `y` if `y > x`
- +0.0 if x and y are zero with opposite signs
- Either `x` or `y` if `x == y` and the signs are the same
- Non-NaN if one operand is NaN
- qNaN if both operands are NaNx

Excluded from our implementation is sNaN handling.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fmaximum_num`](#fmaximum-num) | fn |  |

## Functions

### `fmaximum_num`

```rust
fn fmaximum_num<F: Float>(x: F, y: F) -> F
```

