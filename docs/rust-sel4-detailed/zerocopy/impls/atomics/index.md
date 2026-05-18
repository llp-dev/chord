*[zerocopy](../../index.md) / [impls](../index.md) / [atomics](index.md)*

---

# Module `atomics`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`atomic_8`](#atomic-8) | mod |  |
| [`atomic_16`](#atomic-16) | mod |  |
| [`atomic_32`](#atomic-32) | mod |  |
| [`atomic_64`](#atomic-64) | mod |  |
| [`atomic_ptr`](#atomic-ptr) | mod |  |
| [`impl_traits_for_atomics!`](#impl-traits-for-atomics) | macro |  |
| [`unsafe_impl_transmute_from_for_atomic!`](#unsafe-impl-transmute-from-for-atomic) | macro | Implements `TransmuteFrom` for `$atomic`, `$prim`, and `UnsafeCell<$prim>`. |

## Modules

- [`atomic_8`](atomic_8/index.md)
- [`atomic_16`](atomic_16/index.md)
- [`atomic_32`](atomic_32/index.md)
- [`atomic_64`](atomic_64/index.md)
- [`atomic_ptr`](atomic_ptr/index.md)

## Macros

### `impl_traits_for_atomics!`

### `unsafe_impl_transmute_from_for_atomic!`

Implements `TransmuteFrom` for `$atomic`, `$prim`, and
`UnsafeCell<$prim>`.

# Safety

`$atomic` must have the same size and bit validity as `$prim`.

