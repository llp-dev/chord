*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [rint](index.md)*

---

# Module `rint`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`rint_round`](#rint-round) | fn | IEEE 754-2019 `roundToIntegralExact`, which respects rounding mode and raises inexact if applicable. |

## Functions

### `rint_round`

```rust
fn rint_round<F: Float>(x: F, _round: crate::support::Round) -> crate::support::FpResult<F>
```

IEEE 754-2019 `roundToIntegralExact`, which respects rounding mode and raises inexact if
applicable.

