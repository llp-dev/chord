*[futures_util](../../index.md) / [stream](../index.md) / [iter](index.md)*

---

# Module `iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Iter`](#iter) | struct | Stream for the [`iter`] function. |
| [`iter`](#iter) | fn | Converts an `Iterator` into a `Stream` which is always ready to yield the next value. |

## Structs

### `Iter<I>`

```rust
struct Iter<I> {
    iter: I,
}
```

Stream for the [`iter`](#iter) function.

#### Implementations

- <span id="iter-get-ref"></span>`fn get_ref(&self) -> &I`

  Acquires a reference to the underlying iterator that this stream is pulling from.

- <span id="iter-get-mut"></span>`fn get_mut(&mut self) -> &mut I`

  Acquires a mutable reference to the underlying iterator that this stream is pulling from.

- <span id="iter-into-inner"></span>`fn into_inner(self) -> I`

  Consumes this stream, returning the underlying iterator.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Iter<I>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<I>` — [`Iter`](#iter)

##### `impl<I: fmt::Debug> Debug for Iter<I>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Stream for Iter<I>`

- <span id="iter-stream-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<I as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

- <span id="iter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Iter<I>`

##### `impl TryStream for Iter<I>`

- <span id="iter-trystream-type-ok"></span>`type Ok = T`

- <span id="iter-trystream-type-error"></span>`type Error = E`

- <span id="iter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Iter<I>`

##### `impl<I> Unpin for Iter<I>`

## Functions

### `iter`

```rust
fn iter<I>(i: I) -> Iter<<I as >::IntoIter>
where
    I: IntoIterator
```

Converts an `Iterator` into a `Stream` which is always ready
to yield the next value.

Iterators in Rust don't express the ability to block, so this adapter
simply always calls `iter.next()` and returns that.

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::iter(vec![17, 19]);
assert_eq!(vec![17, 19], stream.collect::<Vec<i32>>().await);
});
```

