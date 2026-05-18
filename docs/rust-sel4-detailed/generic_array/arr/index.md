*[generic_array](../index.md) / [arr](index.md)*

---

# Module `arr`

Implementation for `arr!` macro.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`doctests_only`](#doctests-only) | mod |  |
| [`AddLength`](#addlength) | trait | Helper trait for `arr!` macro |
| [`Inc`](#inc) | type | Helper type for `arr!` macro |

## Modules

- [`doctests_only`](doctests_only/index.md)

## Traits

### `AddLength<T, N: ArrayLength<T>>`

```rust
trait AddLength<T, N: ArrayLength<T>>: ArrayLength<T> { ... }
```

Helper trait for `arr!` macro

#### Associated Types

- `type Output: 1`

#### Implementors

- `N1`

## Type Aliases

### `Inc<T, U>`

```rust
type Inc<T, U> = <U as AddLength>::Output;
```

Helper type for `arr!` macro

