*[futures_util](../../index.md) / [future](../index.md) / [option](index.md)*

---

# Module `option`

Definition of the `Option` (optional step) combinator

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OptionFuture`](#optionfuture) | struct | A future representing a value which may or may not be present. |

## Structs

### `OptionFuture<F>`

```rust
struct OptionFuture<F> {
    inner: Option<F>,
}
```

A future representing a value which may or may not be present.

Created by the [`From`](../../../thiserror_impl/attr/index.md) implementation for [`Option`](std::option::Option).

# Examples

```rust
futures::executor::block_on(async {
use futures::future::OptionFuture;

let mut a: OptionFuture<_> = Some(async { 123 }).into();
assert_eq!(a.await, Some(123));

a = None.into();
assert_eq!(a.await, None);
});
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for OptionFuture<F>`

- <span id="optionfuture-clone"></span>`fn clone(&self) -> OptionFuture<F>` — [`OptionFuture`](#optionfuture)

##### `impl<F: fmt::Debug> Debug for OptionFuture<F>`

- <span id="optionfuture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> Default for OptionFuture<F>`

- <span id="optionfuture-default"></span>`fn default() -> Self`

##### `impl<F: FusedFuture> FusedFuture for OptionFuture<F>`

- <span id="optionfuture-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F: Future> Future for OptionFuture<F>`

- <span id="optionfuture-future-type-output"></span>`type Output = Option<<F as Future>::Output>`

- <span id="optionfuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for OptionFuture<F>`

##### `impl<F> IntoFuture for OptionFuture<F>`

- <span id="optionfuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="optionfuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="optionfuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> Unpin for OptionFuture<F>`

