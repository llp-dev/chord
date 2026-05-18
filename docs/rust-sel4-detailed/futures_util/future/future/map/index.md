*[futures_util](../../../index.md) / [future](../../index.md) / [future](../index.md) / [map](index.md)*

---

# Module `map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Map`](#map) | enum | Internal Map future |

## Enums

### `Map<Fut, F>`

```rust
enum Map<Fut, F> {
    Incomplete {
        future: Fut,
        f: F,
    },
    Complete,
}
```

Internal Map future

#### Implementations

- <span id="map-new"></span>`fn new(future: Fut, f: F) -> Self`

  Creates a new Map.

#### Trait Implementations

##### `impl<Fut: fmt::Debug, F: fmt::Debug> Debug for Map<Fut, F>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut, F> FusedFuture for Map<Fut, F>`

- <span id="map-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for Map<Fut, F>`

- <span id="map-future-type-output"></span>`type Output = T`

- <span id="map-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl FutureExt for Map<Fut, F>`

##### `impl<F> IntoFuture for Map<Fut, F>`

- <span id="map-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="map-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="map-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Map<Fut, F>`

- <span id="map-tryfuture-type-ok"></span>`type Ok = T`

- <span id="map-tryfuture-type-error"></span>`type Error = E`

- <span id="map-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl<Fut> TryFutureExt for Map<Fut, F>`

##### `impl<Fut, F> Unpin for Map<Fut, F>`

