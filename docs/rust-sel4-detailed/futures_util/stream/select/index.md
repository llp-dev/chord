*[futures_util](../../index.md) / [stream](../index.md) / [select](index.md)*

---

# Module `select`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Select`](#select) | struct | Stream for the [`select()`] function. |
| [`select`](#select) | fn | This function will attempt to pull items from both streams. |

## Structs

### `Select<St1, St2>`

```rust
struct Select<St1, St2> {
    inner: crate::stream::SelectWithStrategy<St1, St2, fn(&mut crate::stream::PollNext) -> crate::stream::PollNext, crate::stream::PollNext>,
}
```

Stream for the [`select()`](#select) function.

#### Implementations

- <span id="select-get-ref"></span>`fn get_ref(&self) -> (&St1, &St2)`

  Acquires a reference to the underlying streams that this combinator is

  pulling from.

- <span id="select-get-mut"></span>`fn get_mut(&mut self) -> (&mut St1, &mut St2)`

  Acquires a mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="select-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut St1>, Pin<&mut St2>)`

  Acquires a pinned mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="select-into-inner"></span>`fn into_inner(self) -> (St1, St2)`

  Consumes this combinator, returning the underlying streams.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St1: fmt::Debug, St2: fmt::Debug> Debug for Select<St1, St2>`

- <span id="select-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St1, St2> FusedStream for Select<St1, St2>`

- <span id="select-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St1, St2> Stream for Select<St1, St2>`

- <span id="select-stream-type-item"></span>`type Item = <St1 as Stream>::Item`

- <span id="select-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St1 as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl StreamExt for Select<St1, St2>`

##### `impl TryStream for Select<St1, St2>`

- <span id="select-trystream-type-ok"></span>`type Ok = T`

- <span id="select-trystream-type-error"></span>`type Error = E`

- <span id="select-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Select<St1, St2>`

##### `impl<St1, St2> Unpin for Select<St1, St2>`

## Functions

### `select`

```rust
fn select<St1, St2>(stream1: St1, stream2: St2) -> Select<St1, St2>
where
    St1: Stream,
    St2: Stream<Item = <St1 as >::Item>
```

This function will attempt to pull items from both streams. Each
stream will be polled in a round-robin fashion, and whenever a stream is
ready to yield an item that item is yielded.

After one of the two input streams completes, the remaining one will be
polled exclusively. The returned stream completes when both input
streams have completed.

Note that this function consumes both streams and returns a wrapped
version of them.

## Examples

```rust
futures::executor::block_on(async {
use futures::stream::{ repeat, select, StreamExt };

let left = repeat(1);
let right = repeat(2);

let mut out = select(left, right);

for _ in 0..100 {
    // We should be alternating.
    assert_eq!(1, out.select_next_some().await);
    assert_eq!(2, out.select_next_some().await);
}
});
```

