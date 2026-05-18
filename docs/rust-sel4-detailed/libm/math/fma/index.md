*[libm](../../index.md) / [math](../index.md) / [fma](index.md)*

---

# Module `fma`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fmaf`](#fmaf) | fn | Floating multiply add (f32) |
| [`fma`](#fma) | fn | Fused multiply add (f64) |

## Functions

### `fmaf`

```rust
fn fmaf(x: f32, y: f32, z: f32) -> f32
```

Floating multiply add (f32)

Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision).

### `fma`

```rust
fn fma(x: f64, y: f64, z: f64) -> f64
```

Fused multiply add (f64)

Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision).

