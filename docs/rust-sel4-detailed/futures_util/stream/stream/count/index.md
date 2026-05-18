*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [count](index.md)*

---

# Module `count`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Count`](#count) | struct | Future for the [`count`](super::StreamExt::count) method. |

## Structs

### `Count<St>`

```rust
struct Count<St> {
    stream: St,
    count: usize,
}
```

Future for the [`count`](super::StreamExt::count) method.

#### Implementations

- <span id="count-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St> Debug for Count<St>`

- <span id="count-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: FusedStream> FusedFuture for Count<St>`

- <span id="count-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: Stream> Future for Count<St>`

- <span id="count-future-type-output"></span>`type Output = usize`

- <span id="count-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for Count<St>`

##### `impl IntoFuture for Count<St>`

- <span id="count-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="count-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="count-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St> Unpin for Count<St>`

