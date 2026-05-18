*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [chain](index.md)*

---

# Module `chain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Chain`](#chain) | struct | Stream for the [`chain`](super::StreamExt::chain) method. |

## Structs

### `Chain<St1, St2>`

```rust
struct Chain<St1, St2> {
    first: Option<St1>,
    second: St2,
}
```

Stream for the [`chain`](super::StreamExt::chain) method.

#### Implementations

- <span id="chain-new"></span>`fn new(stream1: St1, stream2: St2) -> Self`

#### Trait Implementations

##### `impl<St1: fmt::Debug, St2: fmt::Debug> Debug for Chain<St1, St2>`

- <span id="chain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St1, St2> FusedStream for Chain<St1, St2>`

- <span id="chain-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St1, St2> Stream for Chain<St1, St2>`

- <span id="chain-stream-type-item"></span>`type Item = <St1 as Stream>::Item`

- <span id="chain-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="chain-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Chain<St1, St2>`

##### `impl TryStream for Chain<St1, St2>`

- <span id="chain-trystream-type-ok"></span>`type Ok = T`

- <span id="chain-trystream-type-error"></span>`type Error = E`

- <span id="chain-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Chain<St1, St2>`

##### `impl<St1, St2> Unpin for Chain<St1, St2>`

