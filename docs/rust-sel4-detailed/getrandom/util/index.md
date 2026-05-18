*[getrandom](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`slice_assume_init_mut`](#slice-assume-init-mut) | fn | Polyfill for `maybe_uninit_slice` feature's `MaybeUninit::slice_assume_init_mut`. |
| [`uninit_slice_fill_zero`](#uninit-slice-fill-zero) | fn |  |
| [`slice_as_uninit`](#slice-as-uninit) | fn |  |
| [`slice_as_uninit_mut`](#slice-as-uninit-mut) | fn | View an mutable initialized array as potentially-uninitialized. |

## Functions

### `slice_assume_init_mut`

```rust
unsafe fn slice_assume_init_mut<T>(slice: &mut [core::mem::MaybeUninit<T>]) -> &mut [T]
```

Polyfill for `maybe_uninit_slice` feature's
`MaybeUninit::slice_assume_init_mut`. Every element of `slice` must have
been initialized.

### `uninit_slice_fill_zero`

```rust
fn uninit_slice_fill_zero(slice: &mut [core::mem::MaybeUninit<u8>]) -> &mut [u8]
```

### `slice_as_uninit`

```rust
fn slice_as_uninit<T>(slice: &[T]) -> &[core::mem::MaybeUninit<T>]
```

### `slice_as_uninit_mut`

```rust
unsafe fn slice_as_uninit_mut<T>(slice: &mut [T]) -> &mut [core::mem::MaybeUninit<T>]
```

View an mutable initialized array as potentially-uninitialized.

This is unsafe because it allows assigning uninitialized values into
`slice`, which would be undefined behavior.

