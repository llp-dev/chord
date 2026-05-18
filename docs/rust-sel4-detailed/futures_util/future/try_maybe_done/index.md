*[futures_util](../../index.md) / [future](../index.md) / [try_maybe_done](index.md)*

---

# Module `try_maybe_done`

Definition of the TryMaybeDone combinator

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryMaybeDone`](#trymaybedone) | enum | A future that may have completed with an error. |
| [`try_maybe_done`](#try-maybe-done) | fn | Wraps a future into a `TryMaybeDone` |

## Enums

### `TryMaybeDone<Fut: TryFuture>`

```rust
enum TryMaybeDone<Fut: TryFuture> {
    Future(Fut),
    Done(<Fut as >::Ok),
    Gone,
}
```

A future that may have completed with an error.

This is created by the [`try_maybe_done()`](#try-maybe-done) function.

#### Variants

- **`Future`**

  A not-yet-completed future

- **`Done`**

  The output of the completed future

- **`Gone`**

  The empty variant after the result of a [`TryMaybeDone`](#trymaybedone) has been
  taken using the [`take_output`](TryMaybeDone::take_output) method,
  or if the future returned an error.

#### Implementations

- <span id="trymaybedone-output-mut"></span>`fn output_mut(self: Pin<&mut Self>) -> Option<&mut <Fut as >::Ok>` — [`TryFuture`](../index.md#tryfuture)

  Returns an [`Option`](../../../serde_core/index.md) containing a mutable reference to the output of the future.

  The output of this method will be [`Some`](../../../managed/index.md) if and only if the inner

  future has completed successfully and [`take_output`](TryMaybeDone::take_output)

  has not yet been called.

- <span id="trymaybedone-take-output"></span>`fn take_output(self: Pin<&mut Self>) -> Option<<Fut as >::Ok>` — [`TryFuture`](../index.md#tryfuture)

  Attempt to take the output of a `TryMaybeDone` without driving it

  towards completion.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + TryFuture> Debug for TryMaybeDone<Fut>`

- <span id="trymaybedone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: TryFuture> FusedFuture for TryMaybeDone<Fut>`

- <span id="trymaybedone-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: TryFuture> Future for TryMaybeDone<Fut>`

- <span id="trymaybedone-future-type-output"></span>`type Output = Result<(), <Fut as TryFuture>::Error>`

- <span id="trymaybedone-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryMaybeDone<Fut>`

##### `impl IntoFuture for TryMaybeDone<Fut>`

- <span id="trymaybedone-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trymaybedone-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trymaybedone-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryMaybeDone<Fut>`

- <span id="trymaybedone-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trymaybedone-tryfuture-type-error"></span>`type Error = E`

- <span id="trymaybedone-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for TryMaybeDone<Fut>`

##### `impl<Fut: TryFuture + Unpin> Unpin for TryMaybeDone<Fut>`

## Functions

### `try_maybe_done`

```rust
fn try_maybe_done<Fut: TryFuture>(future: Fut) -> TryMaybeDone<Fut>
```

Wraps a future into a `TryMaybeDone`

