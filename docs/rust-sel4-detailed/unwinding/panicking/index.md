*[unwinding](../index.md) / [panicking](index.md)*

---

# Module `panicking`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Exception`](#exception) | trait |  |
| [`begin_panic`](#begin-panic) | fn |  |
| [`catch_unwind`](#catch-unwind) | fn |  |

## Traits

### `Exception`

```rust
trait Exception { ... }
```

#### Associated Constants

- `const CLASS: [u8; 8]`

#### Required Methods

- `fn wrap(this: Self) -> *mut UnwindException`

- `fn unwrap(ex: *mut UnwindException) -> Self`

## Functions

### `begin_panic`

```rust
fn begin_panic<E: Exception>(exception: E) -> UnwindReasonCode
```

### `catch_unwind`

```rust
fn catch_unwind<E: Exception, R, F: FnOnce() -> R>(f: F) -> Result<R, Option<E>>
```

