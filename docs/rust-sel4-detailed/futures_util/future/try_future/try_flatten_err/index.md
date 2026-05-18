*[futures_util](../../../index.md) / [future](../../index.md) / [try_future](../index.md) / [try_flatten_err](index.md)*

---

# Module `try_flatten_err`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFlattenErr`](#tryflattenerr) | enum |  |

## Enums

### `TryFlattenErr<Fut1, Fut2>`

```rust
enum TryFlattenErr<Fut1, Fut2> {
    First {
        f: Fut1,
    },
    Second {
        f: Fut2,
    },
    Empty,
}
```

#### Implementations

- <span id="tryflattenerr-new"></span>`fn new(future: Fut1) -> Self`

#### Trait Implementations

##### `impl<Fut1: fmt::Debug, Fut2: fmt::Debug> Debug for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> FusedFuture for TryFlattenErr<Fut, <Fut as >::Error>`

- <span id="tryflattenerr-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut> Future for TryFlattenErr<Fut, <Fut as >::Error>`

- <span id="tryflattenerr-future-type-output"></span>`type Output = Result<<Fut as TryFuture>::Ok, <<Fut as TryFuture>::Error as TryFuture>::Error>`

- <span id="tryflattenerr-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl FutureExt for TryFlattenErr<Fut1, Fut2>`

##### `impl IntoFuture for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryflattenerr-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryflattenerr-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryflattenerr-tryfuture-type-error"></span>`type Error = E`

- <span id="tryflattenerr-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl TryFutureExt for TryFlattenErr<Fut1, Fut2>`

##### `impl<Fut1, Fut2> Unpin for TryFlattenErr<Fut1, Fut2>`

