*[futures_util](../../index.md) / [stream](../index.md) / [empty](index.md)*

---

# Module `empty`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Empty`](#empty) | struct | Stream for the [`empty`] function. |
| [`empty`](#empty) | fn | Creates a stream which contains no elements. |

## Structs

### `Empty<T>`

```rust
struct Empty<T> {
    _phantom: core::marker::PhantomData<T>,
}
```

Stream for the [`empty`](#empty) function.

#### Trait Implementations

##### `impl<T> Clone for Empty<T>`

- <span id="empty-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Empty<T>`

- <span id="empty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedStream for Empty<T>`

- <span id="empty-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Stream for Empty<T>`

- <span id="empty-stream-type-item"></span>`type Item = T`

- <span id="empty-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="empty-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for Empty<T>`

##### `impl<T> TryStream for Empty<T>`

- <span id="empty-trystream-type-ok"></span>`type Ok = T`

- <span id="empty-trystream-type-error"></span>`type Error = E`

- <span id="empty-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Empty<T>`

##### `impl<T> Unpin for Empty<T>`

## Functions

### `empty`

```rust
fn empty<T>() -> Empty<T>
```

Creates a stream which contains no elements.

The returned stream will always return `Ready(None)` when polled.

