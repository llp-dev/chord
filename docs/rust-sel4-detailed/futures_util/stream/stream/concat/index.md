*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [concat](index.md)*

---

# Module `concat`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Concat`](#concat) | struct | Future for the [`concat`](super::StreamExt::concat) method. |

## Structs

### `Concat<St: Stream>`

```rust
struct Concat<St: Stream> {
    stream: St,
    accum: Option<<St as >::Item>,
}
```

Future for the [`concat`](super::StreamExt::concat) method.

#### Implementations

- <span id="concat-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + Stream> Debug for Concat<St>`

- <span id="concat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedFuture for Concat<St>`

- <span id="concat-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Future for Concat<St>`

- <span id="concat-future-type-output"></span>`type Output = <St as Stream>::Item`

- <span id="concat-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for Concat<St>`

##### `impl IntoFuture for Concat<St>`

- <span id="concat-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="concat-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="concat-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Concat<St>`

- <span id="concat-tryfuture-type-ok"></span>`type Ok = T`

- <span id="concat-tryfuture-type-error"></span>`type Error = E`

- <span id="concat-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl TryFutureExt for Concat<St>`

##### `impl<St: Stream> Unpin for Concat<St>`

