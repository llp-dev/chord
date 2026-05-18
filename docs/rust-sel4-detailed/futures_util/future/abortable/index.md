*[futures_util](../../index.md) / [future](../index.md) / [abortable](index.md)*

---

# Module `abortable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`abortable`](#abortable) | fn | Creates a new `Abortable` future and an `AbortHandle` which can be used to stop it. |

## Functions

### `abortable`

```rust
fn abortable<Fut>(future: Fut) -> (crate::future::Abortable<Fut>, crate::future::AbortHandle)
where
    Fut: Future
```

Creates a new `Abortable` future and an `AbortHandle` which can be used to stop it.

This function is a convenient (but less flexible) alternative to calling
`AbortHandle::new` and `Abortable::new` manually.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

