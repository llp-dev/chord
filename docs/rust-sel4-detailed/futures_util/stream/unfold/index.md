*[futures_util](../../index.md) / [stream](../index.md) / [unfold](index.md)*

---

# Module `unfold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Unfold`](#unfold) | struct | Stream for the [`unfold`] function. |
| [`unfold`](#unfold) | fn | Creates a `Stream` from a seed and a closure returning a `Future`. |

## Structs

### `Unfold<T, F, Fut>`

```rust
struct Unfold<T, F, Fut> {
    f: F,
    state: crate::unfold_state::UnfoldState<T, Fut>,
}
```

Stream for the [`unfold`](#unfold) function.

#### Trait Implementations

##### `impl<T, F, Fut> Debug for Unfold<T, F, Fut>`

- <span id="unfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F, Fut> FusedStream for Unfold<T, F, Fut>`

- <span id="unfold-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T, F, Fut> Stream for Unfold<T, F, Fut>`

- <span id="unfold-stream-type-item"></span>`type Item = Item`

- <span id="unfold-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl<T> StreamExt for Unfold<T, F, Fut>`

##### `impl<T> TryStream for Unfold<T, F, Fut>`

- <span id="unfold-trystream-type-ok"></span>`type Ok = T`

- <span id="unfold-trystream-type-error"></span>`type Error = E`

- <span id="unfold-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Unfold<T, F, Fut>`

##### `impl<T, F, Fut> Unpin for Unfold<T, F, Fut>`

## Functions

### `unfold`

```rust
fn unfold<T, F, Fut, Item>(init: T, f: F) -> Unfold<T, F, Fut>
where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = Option<(Item, T)>>
```

Creates a `Stream` from a seed and a closure returning a `Future`.

This function is the dual for the `Stream::fold()` adapter: while
`Stream::fold()` reduces a `Stream` to one single value, `unfold()` creates a
`Stream` from a seed value.

`unfold()` will call the provided closure with the provided seed, then wait
for the returned `Future` to complete with `(a, b)`. It will then yield the
value `a`, and use `b` as the next internal state.

If the closure returns `None` instead of `Some(Future)`, then the `unfold()`
will stop producing items and return `Poll::Ready(None)` in future
calls to `poll()`.

This function can typically be used when wanting to go from the "world of
futures" to the "world of streams": the provided closure can build a
`Future` using other library functions working on futures, and `unfold()`
will turn it into a `Stream` by repeating the operation.

# Example

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::unfold(0, |state| async move {
    if state <= 2 {
        let next_state = state + 1;
        let yielded = state * 2;
        Some((yielded, next_state))
    } else {
        None
    }
});

let result = stream.collect::<Vec<i32>>().await;
assert_eq!(result, vec![0, 2, 4]);
});
```

