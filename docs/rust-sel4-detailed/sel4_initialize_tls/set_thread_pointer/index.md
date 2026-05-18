*[sel4_initialize_tls](../index.md) / [set_thread_pointer](index.md)*

---

# Module `set_thread_pointer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`default_set_thread_pointer`](#default-set-thread-pointer) | fn |  |
| [`SetThreadPointerFn`](#setthreadpointerfn) | type |  |
| [`DEFAULT_SET_THREAD_POINTER_FN`](#default-set-thread-pointer-fn) | const |  |

## Functions

### `default_set_thread_pointer`

```rust
unsafe fn default_set_thread_pointer(thread_pointer: usize)
```

## Type Aliases

### `SetThreadPointerFn`

```rust
type SetThreadPointerFn = fn(usize);
```

## Constants

### `DEFAULT_SET_THREAD_POINTER_FN`
```rust
const DEFAULT_SET_THREAD_POINTER_FN: SetThreadPointerFn = {set_thread_pointer::default_set_thread_pointer as unsafe extern "C" fn(usize)};
```

