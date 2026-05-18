*[libm](../../../index.md) / [math](../../index.md) / [arch](../index.md) / [x86](index.md)*

---

# Module `x86`

Architecture-specific support for x86-32 and x86-64 with SSE2

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`detect`](#detect) | mod |  |
| [`fma`](#fma) | mod | Use assembly fma if the `fma` or `fma4` feature is detected at runtime. |
| [`fma`](#fma) | fn |  |
| [`fmaf`](#fmaf) | fn |  |
| [`sqrtf`](#sqrtf) | fn |  |
| [`sqrt`](#sqrt) | fn |  |

## Modules

- [`detect`](detect/index.md)
- [`fma`](fma/index.md) — Use assembly fma if the `fma` or `fma4` feature is detected at runtime.

## Functions

### `fma`

```rust
fn fma(x: f64, y: f64, z: f64) -> f64
```

### `fmaf`

```rust
fn fmaf(x: f32, y: f32, z: f32) -> f32
```

### `sqrtf`

```rust
fn sqrtf(x: f32) -> f32
```

### `sqrt`

```rust
fn sqrt(x: f64) -> f64
```

