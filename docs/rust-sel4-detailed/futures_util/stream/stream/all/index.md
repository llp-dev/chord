*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [all](index.md)*

---

# Module `all`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`All`](#all) | struct | Future for the [`all`](super::StreamExt::all) method. |

## Structs

### `All<St, Fut, F>`

```rust
struct All<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`all`](super::StreamExt::all) method.

#### Implementations

- <span id="all-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for All<St, Fut, F>`

- <span id="all-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for All<St, Fut, F>`

- <span id="all-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for All<St, Fut, F>`

- <span id="all-future-type-output"></span>`type Output = bool`

- <span id="all-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl FutureExt for All<St, Fut, F>`

##### `impl<F> IntoFuture for All<St, Fut, F>`

- <span id="all-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="all-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="all-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for All<St, Fut, F>`

