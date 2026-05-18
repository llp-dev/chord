*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [fma_wide](index.md)*

---

# Module `fma_wide`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fma_wide_round`](#fma-wide-round) | fn | Fma implementation when a hardware-backed larger float type is available. |

## Functions

### `fma_wide_round`

```rust
fn fma_wide_round<F, B>(x: F, y: F, z: F, round: crate::support::Round) -> crate::support::FpResult<F>
where
    F: Float + HFloat<D = B>,
    B: Float + DFloat<H = F>,
    <B as >::Int: CastInto<i32>,
    i32: CastFrom<i32>
```

Fma implementation when a hardware-backed larger float type is available. For `f32` and `f64`,
`f64` has enough precision to represent the `f32` in its entirety, except for double rounding.

