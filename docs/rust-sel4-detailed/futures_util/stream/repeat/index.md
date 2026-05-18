*[futures_util](../../index.md) / [stream](../index.md) / [repeat](index.md)*

---

# Module `repeat`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Repeat`](#repeat) | struct | Stream for the [`repeat`] function. |
| [`repeat`](#repeat) | fn | Create a stream which produces the same item repeatedly. |

## Structs

### `Repeat<T>`

```rust
struct Repeat<T> {
    item: T,
}
```

Stream for the [`repeat`](#repeat) function.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Repeat<T>`

- <span id="repeat-clone"></span>`fn clone(&self) -> Repeat<T>` — [`Repeat`](#repeat)

##### `impl<T: fmt::Debug> Debug for Repeat<T>`

- <span id="repeat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedStream for Repeat<T>`

- <span id="repeat-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Stream for Repeat<T>`

- <span id="repeat-stream-type-item"></span>`type Item = T`

- <span id="repeat-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="repeat-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for Repeat<T>`

##### `impl<T> TryStream for Repeat<T>`

- <span id="repeat-trystream-type-ok"></span>`type Ok = T`

- <span id="repeat-trystream-type-error"></span>`type Error = E`

- <span id="repeat-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Repeat<T>`

##### `impl<T> Unpin for Repeat<T>`

## Functions

### `repeat`

```rust
fn repeat<T>(item: T) -> Repeat<T>
where
    T: Clone
```

Create a stream which produces the same item repeatedly.

The stream never terminates. Note that you likely want to avoid
usage of `collect` or such on the returned stream as it will exhaust
available memory as it tries to just fill up all RAM.

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::repeat(9);
assert_eq!(vec![9, 9, 9], stream.take(3).collect::<Vec<i32>>().await);
});
```

