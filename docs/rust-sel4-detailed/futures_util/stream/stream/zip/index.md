*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [zip](index.md)*

---

# Module `zip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Zip`](#zip) | struct | Stream for the [`zip`](super::StreamExt::zip) method. |

## Structs

### `Zip<St1: Stream, St2: Stream>`

```rust
struct Zip<St1: Stream, St2: Stream> {
    stream1: crate::stream::Fuse<St1>,
    stream2: crate::stream::Fuse<St2>,
    queued1: Option<<St1 as >::Item>,
    queued2: Option<<St2 as >::Item>,
}
```

Stream for the [`zip`](super::StreamExt::zip) method.

#### Implementations

- <span id="zip-new"></span>`fn new(stream1: St1, stream2: St2) -> Self`

- <span id="zip-get-ref"></span>`fn get_ref(&self) -> (&St1, &St2)`

  Acquires a reference to the underlying streams that this combinator is

  pulling from.

- <span id="zip-get-mut"></span>`fn get_mut(&mut self) -> (&mut St1, &mut St2)`

  Acquires a mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="zip-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut St1>, Pin<&mut St2>)`

  Acquires a pinned mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="zip-into-inner"></span>`fn into_inner(self) -> (St1, St2)`

  Consumes this combinator, returning the underlying streams.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St1: fmt::Debug + Stream, St2: fmt::Debug + Stream> Debug for Zip<St1, St2>`

- <span id="zip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St1, St2> FusedStream for Zip<St1, St2>`

- <span id="zip-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St1, St2> Stream for Zip<St1, St2>`

- <span id="zip-stream-type-item"></span>`type Item = (<St1 as Stream>::Item, <St2 as Stream>::Item)`

- <span id="zip-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="zip-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Zip<St1, St2>`

##### `impl<St1: Stream, St2: Stream> Unpin for Zip<St1, St2>`

