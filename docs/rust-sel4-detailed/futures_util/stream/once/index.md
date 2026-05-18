*[futures_util](../../index.md) / [stream](../index.md) / [once](index.md)*

---

# Module `once`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Once`](#once) | struct | A stream which emits single element and then EOF. |
| [`once`](#once) | fn | Creates a stream of a single element. |

## Structs

### `Once<Fut>`

```rust
struct Once<Fut> {
    future: Option<Fut>,
}
```

A stream which emits single element and then EOF.

#### Implementations

- <span id="once-new"></span>`fn new(future: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for Once<Fut>`

- <span id="once-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> FusedStream for Once<Fut>`

- <span id="once-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Stream for Once<Fut>`

- <span id="once-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="once-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="once-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Once<Fut>`

##### `impl TryStream for Once<Fut>`

- <span id="once-trystream-type-ok"></span>`type Ok = T`

- <span id="once-trystream-type-error"></span>`type Error = E`

- <span id="once-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Once<Fut>`

##### `impl<Fut> Unpin for Once<Fut>`

## Functions

### `once`

```rust
fn once<Fut: Future>(future: Fut) -> Once<Fut>
```

Creates a stream of a single element.

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::once(async { 17 });
let collected = stream.collect::<Vec<i32>>().await;
assert_eq!(collected, vec![17]);
});
```

