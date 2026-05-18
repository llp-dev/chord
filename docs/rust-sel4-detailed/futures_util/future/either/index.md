*[futures_util](../../index.md) / [future](../index.md) / [either](index.md)*

---

# Module `either`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Either`](#either) | enum | Combines two different futures, streams, or sinks having the same associated types into a single type. |

## Enums

### `Either<A, B>`

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

Combines two different futures, streams, or sinks having the same associated types into a single type.

This is useful when conditionally choosing between two distinct future types:

```rust
use futures::future::Either;

futures::executor::block_on(async {
let cond = true;

let fut = if cond {
    Either::Left(async move { 12 })
} else {
    Either::Right(async move { 44 })
};

assert_eq!(fut.await, 12);
})
```

#### Variants

- **`Left`**

  First branch of the type

- **`Right`**

  Second branch of the type

#### Implementations

- <span id="either-as-pin-ref"></span>`fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&A>, Pin<&B>>` — [`Either`](#either)

  Convert `Pin<&Either<A, B>>` to `Either<Pin<&A>, Pin<&B>>`,

  pinned projections of the inner variants.

- <span id="either-as-pin-mut"></span>`fn as_pin_mut(self: Pin<&mut Self>) -> Either<Pin<&mut A>, Pin<&mut B>>` — [`Either`](#either)

  Convert `Pin<&mut Either<A, B>>` to `Either<Pin<&mut A>, Pin<&mut B>>`,

  pinned projections of the inner variants.

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Either<A, B>`

- <span id="either-clone"></span>`fn clone(&self) -> Either<A, B>` — [`Either`](#either)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Either<A, B>`

- <span id="either-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> FusedFuture for Either<A, B>`

- <span id="either-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<A, B> FusedStream for Either<A, B>`

- <span id="either-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<A, B> Future for Either<A, B>`

- <span id="either-future-type-output"></span>`type Output = <A as Future>::Output`

- <span id="either-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Either<A, B>`

##### `impl IntoFuture for Either<A, B>`

- <span id="either-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="either-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="either-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<A, B, Item> Sink for Either<A, B>`

- <span id="either-sink-type-error"></span>`type Error = <A as Sink>::Error`

- <span id="either-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="either-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="either-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="either-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Either<A, B>`

##### `impl<A, B> Stream for Either<A, B>`

- <span id="either-stream-type-item"></span>`type Item = <A as Stream>::Item`

- <span id="either-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="either-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Either<A, B>`

##### `impl TryFuture for Either<A, B>`

- <span id="either-tryfuture-type-ok"></span>`type Ok = T`

- <span id="either-tryfuture-type-error"></span>`type Error = E`

- <span id="either-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for Either<A, B>`

##### `impl TryStream for Either<A, B>`

- <span id="either-trystream-type-ok"></span>`type Ok = T`

- <span id="either-trystream-type-error"></span>`type Error = E`

- <span id="either-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for Either<A, B>`

