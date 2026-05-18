*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [fmaximum](index.md)*

---

# Module `fmaximum`

IEEE 754-2019 `maximum`.

Per the spec, returns the canonicalized result of:
- `x` if `x > y`
- `y` if `y > x`
- +0.0 if x and y are zero with opposite signs
- qNaN if either operation is NaN

Excluded from our implementation is sNaN handling.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fmaximum`](#fmaximum) | fn |  |

## Functions

### `fmaximum`

```rust
fn fmaximum<F: Float>(x: F, y: F) -> F
```

