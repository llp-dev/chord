*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [macros](index.md)*

---

# Module `macros`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfg_if!`](#cfg-if) | macro | `libm` cannot have dependencies, so this is vendored directly from the `cfg-if` crate (with some comments stripped for compactness). |
| [`select_implementation!`](#select-implementation) | macro | Choose between using an arch-specific implementation and the function body. |
| [`hf32!`](#hf32) | macro | Construct a 32-bit float from hex float representation (C-style), guaranteed to evaluate at compile time. |
| [`hf64!`](#hf64) | macro | Construct a 64-bit float from hex float representation (C-style), guaranteed to evaluate at compile time. |

## Macros

### `cfg_if!`

`libm` cannot have dependencies, so this is vendored directly from the `cfg-if` crate
(with some comments stripped for compactness).

### `select_implementation!`

Choose between using an arch-specific implementation and the function body. Returns directly
if the arch implementation is used, otherwise continue with the rest of the function.

Specify a `use_arch` meta field if an architecture-specific implementation is provided.
These live in the `math::arch::some_target_arch` module.

Specify a `use_arch_required` meta field if something architecture-specific must be used
regardless of feature configuration (`force-soft-floats`).

The passed meta options do not need to account for the `arch` target feature.

### `hf32!`

Construct a 32-bit float from hex float representation (C-style), guaranteed to
evaluate at compile time.

### `hf64!`

Construct a 64-bit float from hex float representation (C-style), guaranteed to
evaluate at compile time.

