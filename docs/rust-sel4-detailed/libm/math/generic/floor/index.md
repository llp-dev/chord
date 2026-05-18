*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [floor](index.md)*

---

# Module `floor`

Generic `floor` algorithm.

Note that this uses the algorithm from musl's `floorf` rather than `floor` or `floorl` because
performance seems to be better (based on icount) and it does not seem to experience rounding
errors on i386.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`floor`](#floor) | fn |  |
| [`floor_status`](#floor-status) | fn |  |

## Functions

### `floor`

```rust
fn floor<F: Float>(x: F) -> F
```

### `floor_status`

```rust
fn floor_status<F: Float>(x: F) -> crate::support::FpResult<F>
```

