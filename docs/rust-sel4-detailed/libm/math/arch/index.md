*[libm](../../index.md) / [math](../index.md) / [arch](index.md)*

---

# Module `arch`

Architecture-specific routines and operations.

LLVM will already optimize calls to some of these in cases that there are hardware
instructions. Providing an implementation here just ensures that the faster implementation
is used when calling the function directly. This helps anyone who uses `libm` directly, as
well as improving things when these routines are called as part of other implementations.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86`](#x86) | mod | Architecture-specific support for x86-32 and x86-64 with SSE2 |
| [`sqrt`](#sqrt) | fn |  |
| [`sqrtf`](#sqrtf) | fn |  |
| [`fma`](#fma) | fn |  |
| [`fmaf`](#fmaf) | fn |  |

## Modules

- [`x86`](x86/index.md) — Architecture-specific support for x86-32 and x86-64 with SSE2

## Functions

### `sqrt`

```rust
fn sqrt(x: f64) -> f64
```

### `sqrtf`

```rust
fn sqrtf(x: f32) -> f32
```

### `fma`

```rust
fn fma(x: f64, y: f64, z: f64) -> f64
```

### `fmaf`

```rust
fn fmaf(x: f32, y: f32, z: f32) -> f32
```

