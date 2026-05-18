*[zerocopy](../../index.md) / [impls](../index.md) / [simd](index.md)*

---

# Module `simd`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`simd_arch_mod!`](#simd-arch-mod) | macro | Defines a module which implements `TryFromBytes`, `FromZeros`, `FromBytes`, and `IntoBytes` for a set of types from a module in `core::arch`. |

## Macros

### `simd_arch_mod!`

Defines a module which implements `TryFromBytes`, `FromZeros`,
`FromBytes`, and `IntoBytes` for a set of types from a module in
`core::arch`.

`$arch` is both the name of the defined module and the name of the
module in `core::arch`, and `$typ` is the list of items from that module
to implement `FromZeros`, `FromBytes`, and `IntoBytes` for.

