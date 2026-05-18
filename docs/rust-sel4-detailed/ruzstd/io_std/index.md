*[ruzstd](../index.md) / [io_std](index.md)*

---

# Module `io_std`

Re-exports of std traits or local reimplementations if std is not available

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Read`](#read) | fn |  |
| [`Write`](#write) | fn |  |

## Functions

### `Read`

```rust
fn Read(&self, n: usize) -> Vec<T, Global>
where
    T: Copy
```

Creates a vector by copying a slice `n` times.

# Panics

This function will panic if the capacity would overflow.

# Examples

Basic usage:

```rust
assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
```

A panic upon overflow:

```should_panic
// this will panic at runtime
b"0123456789abcdef".repeat(usize::MAX);
```

### `Write`

```rust
unsafe fn Write(&mut self) -> &mut T
```

Mutably dereferences the content.

The resulting lifetime is bound to self so this behaves "as if"
it were actually an instance of T that is getting borrowed. If a longer
(unbound) lifetime is needed, use `&mut *my_ptr.as_ptr()`.

