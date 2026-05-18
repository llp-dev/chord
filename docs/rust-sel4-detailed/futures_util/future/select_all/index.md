*[futures_util](../../index.md) / [future](../index.md) / [select_all](index.md)*

---

# Module `select_all`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SelectAll`](#selectall) | struct | Future for the [`select_all`] function. |
| [`select_all`](#select-all) | fn | Creates a new future which will select over a list of futures. |

## Structs

### `SelectAll<Fut>`

```rust
struct SelectAll<Fut> {
    inner: alloc::vec::Vec<Fut>,
}
```

Future for the [`select_all`](#select-all) function.

#### Implementations

- <span id="selectall-into-inner"></span>`fn into_inner(self) -> Vec<Fut>`

  Consumes this combinator, returning the underlying futures.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for SelectAll<Fut>`

- <span id="selectall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future + Unpin> FromIterator for SelectAll<Fut>`

- <span id="selectall-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Fut>>(iter: T) -> Self`

##### `impl<Fut: Future + Unpin> Future for SelectAll<Fut>`

- <span id="selectall-future-type-output"></span>`type Output = (<Fut as Future>::Output, usize, Vec<Fut>)`

- <span id="selectall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for SelectAll<Fut>`

##### `impl IntoFuture for SelectAll<Fut>`

- <span id="selectall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="selectall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="selectall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut: Unpin> Unpin for SelectAll<Fut>`

## Functions

### `select_all`

```rust
fn select_all<I>(iter: I) -> SelectAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: Future + Unpin
```

Creates a new future which will select over a list of futures.

The returned future will wait for any future within `iter` to be ready. Upon
completion the item resolved will be returned, along with the index of the
future that was ready and the list of all the remaining futures.

There are no guarantees provided on the order of the list with the remaining
futures. They might be swapped around, reversed, or completely random.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# Panics

This function will panic if the iterator specified contains no items.

