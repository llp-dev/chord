*[ring](../index.md) / [c](index.md)*

---

# Module `c`

C types.

Avoid using the `libc` crate to get C types since `libc` doesn't support
all the targets we need to support. It turns out that the few types we need
are all uniformly defined on the platforms we care about. This will
probably change if/when we support 16-bit platforms or platforms where
`usize` and `uintptr_t` are different sizes.

TODO(MSRV-1.64): Use `core::ffi::{c_int, c_uint}`, remove the libc
compatibility testing, and remove the libc dev-dependency.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`int`](#int) | type |  |
| [`uint`](#uint) | type |  |
| [`size_t`](#size-t) | type |  |

## Type Aliases

### `int`

```rust
type int = i32;
```

### `uint`

```rust
type uint = u32;
```

### `size_t`

```rust
type size_t = usize;
```

