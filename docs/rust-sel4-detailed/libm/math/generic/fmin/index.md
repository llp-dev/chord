*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [fmin](index.md)*

---

# Module `fmin`

IEEE 754-2008 `minNum`. This has been superseded by IEEE 754-2019 `minimumNumber`.

Per the spec, returns the canonicalized result of:
- `x` if `x < y`
- `y` if `y < x`
- The other number if one is NaN
- Otherwise, either `x` or `y`, canonicalized
- -0.0 and +0.0 may be disregarded (unlike newer operations)

Excluded from our implementation is sNaN handling.

More on the differences: [`link`](../../../../libc/index.md).


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fmin`](#fmin) | fn |  |

## Functions

### `fmin`

```rust
fn fmin<F: Float>(x: F, y: F) -> F
```

