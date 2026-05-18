*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [ceil](index.md)*

---

# Module `ceil`

Generic `ceil` algorithm.

Note that this uses the algorithm from musl's `ceilf` rather than `ceil` or `ceill` because
performance seems to be better (based on icount) and it does not seem to experience rounding
errors on i386.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ceil`](#ceil) | fn |  |
| [`ceil_status`](#ceil-status) | fn |  |

## Functions

### `ceil`

```rust
fn ceil<F: Float>(x: F) -> F
```

### `ceil_status`

```rust
fn ceil_status<F: Float>(x: F) -> crate::support::FpResult<F>
```

