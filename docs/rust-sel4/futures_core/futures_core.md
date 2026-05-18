**futures_core**

# Module: futures_core

## Contents

**Modules**

- [`future`](#future) - Futures.
- [`stream`](#stream) - Asynchronous streams.
- [`task`](#task) - Task notification.

**Macros**

- [`ready`](#ready) - Extracts the successful type of `Poll<T>`.

---

## Module: future

Futures.



## futures_core::ready

*Declarative Macro*

Extracts the successful type of `Poll<T>`.

This macro bakes in propagation of `Pending` signals by returning early.

**Note:** Since Rust 1.64, this macro is soft-deprecated in favor of
[`ready!`](core::task::ready) macro in the standard library.

```rust
macro_rules! ready {
    ($e:expr $(,)?) => { ... };
}
```



## Module: stream

Asynchronous streams.



## Module: task

Task notification.



