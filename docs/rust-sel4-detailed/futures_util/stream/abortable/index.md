*[futures_util](../../index.md) / [stream](../index.md) / [abortable](index.md)*

---

# Module `abortable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`abortable`](#abortable) | fn | Creates a new `Abortable` stream and an `AbortHandle` which can be used to stop it. |

## Functions

### `abortable`

```rust
fn abortable<St>(stream: St) -> (crate::stream::Abortable<St>, crate::stream::AbortHandle)
where
    St: Stream
```

Creates a new `Abortable` stream and an `AbortHandle` which can be used to stop it.

This function is a convenient (but less flexible) alternative to calling
`AbortHandle::new` and `Abortable::new` manually.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

