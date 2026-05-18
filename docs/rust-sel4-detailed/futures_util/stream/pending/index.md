*[futures_util](../../index.md) / [stream](../index.md) / [pending](index.md)*

---

# Module `pending`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pending`](#pending) | struct | Stream for the [`pending()`] function. |
| [`pending`](#pending) | fn | Creates a stream which never returns any elements. |

## Structs

### `Pending<T>`

```rust
struct Pending<T> {
    _data: marker::PhantomData<T>,
}
```

Stream for the [`pending()`](#pending) function.

#### Trait Implementations

##### `impl<T> Clone for Pending<T>`

- <span id="pending-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Pending<T>`

- <span id="pending-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedStream for Pending<T>`

- <span id="pending-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Stream for Pending<T>`

- <span id="pending-stream-type-item"></span>`type Item = T`

- <span id="pending-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="pending-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for Pending<T>`

##### `impl<T> TryStream for Pending<T>`

- <span id="pending-trystream-type-ok"></span>`type Ok = T`

- <span id="pending-trystream-type-error"></span>`type Error = E`

- <span id="pending-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Pending<T>`

##### `impl<T> Unpin for Pending<T>`

## Functions

### `pending`

```rust
fn pending<T>() -> Pending<T>
```

Creates a stream which never returns any elements.

The returned stream will always return `Pending` when polled.

