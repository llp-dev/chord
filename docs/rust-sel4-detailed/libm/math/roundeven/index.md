*[libm](../../index.md) / [math](../index.md) / [roundeven](index.md)*

---

# Module `roundeven`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`roundevenf`](#roundevenf) | fn | Round `x` to the nearest integer, breaking ties toward even. |
| [`roundeven`](#roundeven) | fn | Round `x` to the nearest integer, breaking ties toward even. |
| [`roundeven_impl`](#roundeven-impl) | fn |  |

## Functions

### `roundevenf`

```rust
fn roundevenf(x: f32) -> f32
```

Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754
`roundToIntegralTiesToEven`.

### `roundeven`

```rust
fn roundeven(x: f64) -> f64
```

Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754
`roundToIntegralTiesToEven`.

### `roundeven_impl`

```rust
fn roundeven_impl<F: Float>(x: F) -> F
```

