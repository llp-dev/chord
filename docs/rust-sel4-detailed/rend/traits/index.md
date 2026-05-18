*[rend](../index.md) / [traits](index.md)*

---

# Module `traits`

## Contents

- [Macros](#macros)
  - [`impl_unop!`](#impl-unop)
  - [`impl_binop_nonzero!`](#impl-binop-nonzero)
  - [`impl_binop_one!`](#impl-binop-one)
  - [`impl_binop_both!`](#impl-binop-both)
  - [`impl_binop!`](#impl-binop)
  - [`impl_binassign_nonzero!`](#impl-binassign-nonzero)
  - [`impl_binassign!`](#impl-binassign)
  - [`impl_clone_and_copy!`](#impl-clone-and-copy)
  - [`impl_fmt!`](#impl-fmt)
  - [`impl_default!`](#impl-default)
  - [`impl_from!`](#impl-from)
  - [`impl_try_from_ptr_size!`](#impl-try-from-ptr-size)
  - [`impl_try_into_ptr_size!`](#impl-try-into-ptr-size)
  - [`impl_into_ptr_size!`](#impl-into-ptr-size)
  - [`impl_hash!`](#impl-hash)
  - [`impl_partial_ord_and_ord!`](#impl-partial-ord-and-ord)
  - [`impl_partial_eq_and_eq!`](#impl-partial-eq-and-eq)
  - [`impl_partial_ord!`](#impl-partial-ord)
  - [`impl_product_and_sum!`](#impl-product-and-sum)
  - [`unsafe_impl_check_bytes_noop!`](#unsafe-impl-check-bytes-noop)
  - [`unsafe_impl_zeroable!`](#unsafe-impl-zeroable)
  - [`unsafe_impl_pod!`](#unsafe-impl-pod)
  - [`unsafe_impl_no_uninit!`](#unsafe-impl-no-uninit)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impl_unop!`](#impl-unop) | macro |  |
| [`impl_binop_nonzero!`](#impl-binop-nonzero) | macro |  |
| [`impl_binop_one!`](#impl-binop-one) | macro |  |
| [`impl_binop_both!`](#impl-binop-both) | macro |  |
| [`impl_binop!`](#impl-binop) | macro |  |
| [`impl_binassign_nonzero!`](#impl-binassign-nonzero) | macro |  |
| [`impl_binassign!`](#impl-binassign) | macro |  |
| [`impl_clone_and_copy!`](#impl-clone-and-copy) | macro |  |
| [`impl_fmt!`](#impl-fmt) | macro |  |
| [`impl_default!`](#impl-default) | macro |  |
| [`impl_from!`](#impl-from) | macro |  |
| [`impl_try_from_ptr_size!`](#impl-try-from-ptr-size) | macro |  |
| [`impl_try_into_ptr_size!`](#impl-try-into-ptr-size) | macro |  |
| [`impl_into_ptr_size!`](#impl-into-ptr-size) | macro |  |
| [`impl_hash!`](#impl-hash) | macro |  |
| [`impl_partial_ord_and_ord!`](#impl-partial-ord-and-ord) | macro |  |
| [`impl_partial_eq_and_eq!`](#impl-partial-eq-and-eq) | macro |  |
| [`impl_partial_ord!`](#impl-partial-ord) | macro |  |
| [`impl_product_and_sum!`](#impl-product-and-sum) | macro |  |
| [`unsafe_impl_check_bytes_noop!`](#unsafe-impl-check-bytes-noop) | macro | # Safety |
| [`unsafe_impl_zeroable!`](#unsafe-impl-zeroable) | macro | # Safety |
| [`unsafe_impl_pod!`](#unsafe-impl-pod) | macro | # Safety |
| [`unsafe_impl_no_uninit!`](#unsafe-impl-no-uninit) | macro | # Safety |

## Macros

### `impl_unop!`

### `impl_binop_nonzero!`

### `impl_binop_one!`

### `impl_binop_both!`

### `impl_binop!`

### `impl_binassign_nonzero!`

### `impl_binassign!`

### `impl_clone_and_copy!`

### `impl_fmt!`

### `impl_default!`

### `impl_from!`

### `impl_try_from_ptr_size!`

### `impl_try_into_ptr_size!`

### `impl_into_ptr_size!`

### `impl_hash!`

### `impl_partial_ord_and_ord!`

### `impl_partial_eq_and_eq!`

### `impl_partial_ord!`

### `impl_product_and_sum!`

### `unsafe_impl_check_bytes_noop!`

# Safety

An impl of `CheckBytes` with a `check_bytes` function that is a no-op must
be sound for `$name`.

### `unsafe_impl_zeroable!`

# Safety

An all-zero bit pattern for `$name` must be a valid value.
As a rule, any derivative type (e.g. `u64_le` or `i32_be`) of a native type
(e.g. `u64`, `i32`) that is `Zeroable` will also be zeroable.

### `unsafe_impl_pod!`

# Safety

Read the safety requirements of `bytemuck::Pod`.
In general, any type that is natively `Pod` (e.g. `u64`, `AtomicU32`) will
be `Pod` even if wrapped (`u64_le`).

It is required that `$name` has an impl for `Zeroable` for this macro to
work. See [`unsafe_impl_zeroable!()`](#unsafe-impl-zeroable).

### `unsafe_impl_no_uninit!`

# Safety

Read the safety requirements of `bytemuck::NoUninit`.
All primitive types are `NoUninit` and as a result, all of this crates's
wrapped primitive types are also `NoUninit`.

