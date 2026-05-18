*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [for_each](index.md)*

---

# Module `for_each`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ForEach`](#foreach) | struct | Future for the [`for_each`](super::StreamExt::for_each) method. |

## Structs

### `ForEach<St, Fut, F>`

```rust
struct ForEach<St, Fut, F> {
    stream: St,
    f: F,
    future: Option<Fut>,
}
```

Future for the [`for_each`](super::StreamExt::for_each) method.

#### Implementations

- <span id="foreach-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for ForEach<St, Fut, F>`

- <span id="foreach-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for ForEach<St, Fut, F>`

- <span id="foreach-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for ForEach<St, Fut, F>`

- <span id="foreach-future-type-output"></span>`type Output = ()`

- <span id="foreach-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl FutureExt for ForEach<St, Fut, F>`

##### `impl<F> IntoFuture for ForEach<St, Fut, F>`

- <span id="foreach-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="foreach-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="foreach-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for ForEach<St, Fut, F>`

