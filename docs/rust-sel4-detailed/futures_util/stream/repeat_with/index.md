*[futures_util](../../index.md) / [stream](../index.md) / [repeat_with](index.md)*

---

# Module `repeat_with`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RepeatWith`](#repeatwith) | struct | An stream that repeats elements of type `A` endlessly by applying the provided closure `F: FnMut() -> A`. |
| [`repeat_with`](#repeat-with) | fn | Creates a new stream that repeats elements of type `A` endlessly by applying the provided closure, the repeater, `F: FnMut() -> A`. |

## Structs

### `RepeatWith<F>`

```rust
struct RepeatWith<F> {
    repeater: F,
}
```

An stream that repeats elements of type `A` endlessly by
applying the provided closure `F: FnMut() -> A`.

This `struct` is created by the [`repeat_with()`](#repeat-with) function.
See its documentation for more.

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for RepeatWith<F>`

- <span id="repeatwith-clone"></span>`fn clone(&self) -> RepeatWith<F>` — [`RepeatWith`](#repeatwith)

##### `impl<F: fmt::Debug> Debug for RepeatWith<F>`

- <span id="repeatwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: FnMut() -> A> FusedStream for RepeatWith<F>`

- <span id="repeatwith-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F: FnMut() -> A> Stream for RepeatWith<F>`

- <span id="repeatwith-stream-type-item"></span>`type Item = A`

- <span id="repeatwith-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="repeatwith-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for RepeatWith<F>`

##### `impl TryStream for RepeatWith<F>`

- <span id="repeatwith-trystream-type-ok"></span>`type Ok = T`

- <span id="repeatwith-trystream-type-error"></span>`type Error = E`

- <span id="repeatwith-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for RepeatWith<F>`

##### `impl<F: FnMut() -> A> Unpin for RepeatWith<F>`

## Functions

### `repeat_with`

```rust
fn repeat_with<A, F: FnMut() -> A>(repeater: F) -> RepeatWith<F>
```

Creates a new stream that repeats elements of type `A` endlessly by
applying the provided closure, the repeater, `F: FnMut() -> A`.

The `repeat_with()` function calls the repeater over and over again.

Infinite stream like `repeat_with()` are often used with adapters like
[`stream.take()`](crate::stream::StreamExt::take), in order to make them finite.

If the element type of the stream you need implements `Clone`, and
it is OK to keep the source element in memory, you should instead use
the [`stream::repeat()`](crate::stream::repeat) function.

# Examples

Basic usage:

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

// let's assume we have some value of a type that is not `Clone`
// or which don't want to have in memory just yet because it is expensive:
#[derive(PartialEq, Debug)]
struct Expensive;

// a particular value forever:
let mut things = stream::repeat_with(|| Expensive);

assert_eq!(Some(Expensive), things.next().await);
assert_eq!(Some(Expensive), things.next().await);
assert_eq!(Some(Expensive), things.next().await);
});
```

Using mutation and going finite:

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

// From the zeroth to the third power of two:
let mut curr = 1;
let mut pow2 = stream::repeat_with(|| { let tmp = curr; curr *= 2; tmp })
                    .take(4);

assert_eq!(Some(1), pow2.next().await);
assert_eq!(Some(2), pow2.next().await);
assert_eq!(Some(4), pow2.next().await);
assert_eq!(Some(8), pow2.next().await);

// ... and now we're done
assert_eq!(None, pow2.next().await);
});
```

