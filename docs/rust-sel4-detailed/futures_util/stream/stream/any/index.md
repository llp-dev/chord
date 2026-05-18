*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [any](index.md)*

---

# Module `any`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Any`](#any) | struct | Future for the [`any`](super::StreamExt::any) method. |

## Structs

### `Any<St, Fut, F>`

```rust
struct Any<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`any`](super::StreamExt::any) method.

#### Implementations

- <span id="any-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for Any<St, Fut, F>`

- <span id="any-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for Any<St, Fut, F>`

- <span id="any-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for Any<St, Fut, F>`

- <span id="any-future-type-output"></span>`type Output = bool`

- <span id="any-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl FutureExt for Any<St, Fut, F>`

##### `impl<F> IntoFuture for Any<St, Fut, F>`

- <span id="any-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="any-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="any-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for Any<St, Fut, F>`

