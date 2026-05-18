*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [unzip](index.md)*

---

# Module `unzip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Unzip`](#unzip) | struct | Future for the [`unzip`](super::StreamExt::unzip) method. |

## Structs

### `Unzip<St, FromA, FromB>`

```rust
struct Unzip<St, FromA, FromB> {
    stream: St,
    left: FromA,
    right: FromB,
}
```

Future for the [`unzip`](super::StreamExt::unzip) method.

#### Implementations

- <span id="unzip-finish"></span>`fn finish(self: Pin<&mut Self>) -> (FromA, FromB)`

- <span id="unzip-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, FromA: fmt::Debug, FromB: fmt::Debug> Debug for Unzip<St, FromA, FromB>`

- <span id="unzip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, FromA, FromB> FusedFuture for Unzip<St, FromA, FromB>`

- <span id="unzip-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, FromA, FromB> Future for Unzip<St, FromA, FromB>`

- <span id="unzip-future-type-output"></span>`type Output = (FromA, FromB)`

- <span id="unzip-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<(FromA, FromB)>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl FutureExt for Unzip<St, FromA, FromB>`

##### `impl IntoFuture for Unzip<St, FromA, FromB>`

- <span id="unzip-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="unzip-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="unzip-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, FromA, FromB> Unpin for Unzip<St, FromA, FromB>`

