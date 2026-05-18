*[libm](../../../../index.md) / [math](../../../index.md) / [arch](../../index.md) / [x86](../index.md) / [fma](index.md)*

---

# Module `fma`

Use assembly fma if the `fma` or `fma4` feature is detected at runtime.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fma`](#fma) | fn |  |
| [`fmaf`](#fmaf) | fn |  |
| [`fma_with_fma`](#fma-with-fma) | fn | # Safety |
| [`fmaf_with_fma`](#fmaf-with-fma) | fn | # Safety |
| [`fma_with_fma4`](#fma-with-fma4) | fn | # Safety |
| [`fmaf_with_fma4`](#fmaf-with-fma4) | fn | # Safety |
| [`fma_fallback`](#fma-fallback) | fn |  |
| [`fmaf_fallback`](#fmaf-fallback) | fn |  |

## Functions

### `fma`

```rust
fn fma(x: f64, y: f64, z: f64) -> f64
```

### `fmaf`

```rust
fn fmaf(x: f32, y: f32, z: f32) -> f32
```

### `fma_with_fma`

```rust
unsafe fn fma_with_fma(x: f64, y: f64, z: f64) -> f64
```

# Safety

Must have +fma available.

### `fmaf_with_fma`

```rust
unsafe fn fmaf_with_fma(x: f32, y: f32, z: f32) -> f32
```

# Safety

Must have +fma available.

### `fma_with_fma4`

```rust
unsafe fn fma_with_fma4(x: f64, y: f64, z: f64) -> f64
```

# Safety

Must have +fma4 available.

### `fmaf_with_fma4`

```rust
unsafe fn fmaf_with_fma4(x: f32, y: f32, z: f32) -> f32
```

# Safety

Must have +fma4 available.

### `fma_fallback`

```rust
fn fma_fallback(x: f64, y: f64, z: f64) -> f64
```

### `fmaf_fallback`

```rust
fn fmaf_fallback(x: f32, y: f32, z: f32) -> f32
```

