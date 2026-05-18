**sel4_panicking**

# Module: sel4_panicking

## Contents

**Functions**

- [`abort_unwind`](#abort_unwind) - Like the unstable `core::panic::abort_unwind`
- [`catch_unwind`](#catch_unwind) - Like `std::panic::catch_unwind`.

---

## sel4_panicking::abort_unwind

*Function*

Like the unstable `core::panic::abort_unwind`

```rust
fn abort_unwind<F, R>(f: F) -> R
```



## sel4_panicking::catch_unwind

*Function*

Like `std::panic::catch_unwind`.

```rust
fn catch_unwind<R, F>(f: F) -> Result<R, ()>
```



