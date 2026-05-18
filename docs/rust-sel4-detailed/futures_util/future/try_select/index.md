*[futures_util](../../index.md) / [future](../index.md) / [try_select](index.md)*

---

# Module `try_select`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TrySelect`](#tryselect) | struct | Future for the [`try_select()`] function. |
| [`try_select`](#try-select) | fn | Waits for either one of two differently-typed futures to complete. |
| [`EitherOk`](#eitherok) | type |  |
| [`EitherErr`](#eithererr) | type |  |

## Structs

### `TrySelect<A, B>`

```rust
struct TrySelect<A, B> {
    inner: Option<(A, B)>,
}
```

Future for the [`try_select()`](#try-select) function.

#### Trait Implementations

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for TrySelect<A, B>`

- <span id="tryselect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> Future for TrySelect<A, B>`

- <span id="tryselect-future-type-output"></span>`type Output = Result<Either<(<A as TryFuture>::Ok, B), (<B as TryFuture>::Ok, A)>, Either<(<A as TryFuture>::Error, B), (<B as TryFuture>::Error, A)>>`

- <span id="tryselect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TrySelect<A, B>`

##### `impl IntoFuture for TrySelect<A, B>`

- <span id="tryselect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryselect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryselect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TrySelect<A, B>`

- <span id="tryselect-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryselect-tryfuture-type-error"></span>`type Error = E`

- <span id="tryselect-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TrySelect<A, B>`

##### `impl<A: Unpin, B: Unpin> Unpin for TrySelect<A, B>`

## Functions

### `try_select`

```rust
fn try_select<A, B>(future1: A, future2: B) -> TrySelect<A, B>
where
    A: TryFuture + Unpin,
    B: TryFuture + Unpin
```

Waits for either one of two differently-typed futures to complete.

This function will return a new future which awaits for either one of both
futures to complete. The returned future will finish with both the value
resolved and a future representing the completion of the other work.

Note that this function consumes the receiving futures and returns a
wrapped version of them.

Also note that if both this and the second future have the same
success/error type you can use the `Either::factor_first` method to
conveniently extract out the value at the end.

# Examples

```rust
use futures::future::{self, Either, Future, FutureExt, TryFuture, TryFutureExt};

// A poor-man's try_join implemented on top of select

fn try_join<A, B, E>(a: A, b: B) -> impl TryFuture<Ok=(A::Ok, B::Ok), Error=E>
     where A: TryFuture<Error = E> + Unpin + 'static,
           B: TryFuture<Error = E> + Unpin + 'static,
           E: 'static,
{
    future::try_select(a, b).then(|res| -> Box<dyn Future<Output = Result<_, _>> + Unpin> {
        match res {
            Ok(Either::Left((x, b))) => Box::new(b.map_ok(move |y| (x, y))),
            Ok(Either::Right((y, a))) => Box::new(a.map_ok(move |x| (x, y))),
            Err(Either::Left((e, _))) => Box::new(future::err(e)),
            Err(Either::Right((e, _))) => Box::new(future::err(e)),
        }
    })
}
```

## Type Aliases

### `EitherOk<A, B>`

```rust
type EitherOk<A, B> = crate::future::Either<(<A as TryFuture>::Ok, B), (<B as TryFuture>::Ok, A)>;
```

### `EitherErr<A, B>`

```rust
type EitherErr<A, B> = crate::future::Either<(<A as TryFuture>::Error, B), (<B as TryFuture>::Error, A)>;
```

