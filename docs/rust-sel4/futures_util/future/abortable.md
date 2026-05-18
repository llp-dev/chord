**futures_util > future > abortable**

# Module: future::abortable

## Contents

**Functions**

- [`abortable`](#abortable) - Creates a new `Abortable` future and an `AbortHandle` which can be used to stop it.

---

## futures_util::future::abortable::abortable

*Function*

Creates a new `Abortable` future and an `AbortHandle` which can be used to stop it.

This function is a convenient (but less flexible) alternative to calling
`AbortHandle::new` and `Abortable::new` manually.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

```rust
fn abortable<Fut>(future: Fut) -> (crate::future::Abortable<Fut>, crate::future::AbortHandle)
```



