*[futures_util](../../index.md) / [future](../index.md) / [select_ok](index.md)*

---

# Module `select_ok`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SelectOk`](#selectok) | struct | Future for the [`select_ok`] function. |
| [`select_ok`](#select-ok) | fn | Creates a new future which will select the first successful future over a list of futures. |

## Structs

### `SelectOk<Fut>`

```rust
struct SelectOk<Fut> {
    inner: alloc::vec::Vec<Fut>,
}
```

Future for the [`select_ok`](#select-ok) function.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for SelectOk<Fut>`

- <span id="selectok-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: TryFuture + Unpin> FromIterator for SelectOk<Fut>`

- <span id="selectok-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Fut>>(iter: T) -> Self`

##### `impl<Fut: TryFuture + Unpin> Future for SelectOk<Fut>`

- <span id="selectok-future-type-output"></span>`type Output = Result<(<Fut as TryFuture>::Ok, Vec<Fut>), <Fut as TryFuture>::Error>`

- <span id="selectok-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for SelectOk<Fut>`

##### `impl IntoFuture for SelectOk<Fut>`

- <span id="selectok-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="selectok-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="selectok-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for SelectOk<Fut>`

- <span id="selectok-tryfuture-type-ok"></span>`type Ok = T`

- <span id="selectok-tryfuture-type-error"></span>`type Error = E`

- <span id="selectok-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for SelectOk<Fut>`

##### `impl<Fut: Unpin> Unpin for SelectOk<Fut>`

## Functions

### `select_ok`

```rust
fn select_ok<I>(iter: I) -> SelectOk<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: TryFuture + Unpin
```

Creates a new future which will select the first successful future over a list of futures.

The returned future will wait for any future within `iter` to be ready and Ok. Unlike
`select_all`, this will only return the first successful completion, or the last
failure. This is useful in contexts where any success is desired and failures
are ignored, unless all the futures fail.

 This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# Panics

This function will panic if the iterator specified contains no items.

