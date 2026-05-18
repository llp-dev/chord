*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [cycle](index.md)*

---

# Module `cycle`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Cycle`](#cycle) | struct | Stream for the [`cycle`](super::StreamExt::cycle) method. |

## Structs

### `Cycle<St>`

```rust
struct Cycle<St> {
    orig: St,
    stream: St,
}
```

Stream for the [`cycle`](super::StreamExt::cycle) method.

#### Implementations

- <span id="cycle-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Cycle<St>`

- <span id="cycle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for Cycle<St>`

- <span id="cycle-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Stream for Cycle<St>`

- <span id="cycle-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="cycle-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="cycle-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Cycle<St>`

##### `impl TryStream for Cycle<St>`

- <span id="cycle-trystream-type-ok"></span>`type Ok = T`

- <span id="cycle-trystream-type-error"></span>`type Error = E`

- <span id="cycle-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Cycle<St>`

##### `impl<St> Unpin for Cycle<St>`

