*[futures_util](../../index.md) / [stream](../index.md) / [select_all](index.md)*

---

# Module `select_all`

An unbounded set of streams

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SelectAll`](#selectall) | struct | An unbounded set of streams |
| [`Iter`](#iter) | struct | Immutable iterator over all streams in the unordered set. |
| [`IterMut`](#itermut) | struct | Mutable iterator over all streams in the unordered set. |
| [`IntoIter`](#intoiter) | struct | Owned iterator over all streams in the unordered set. |
| [`select_all`](#select-all) | fn | Convert a list of streams into a `Stream` of results from the streams. |

## Structs

### `SelectAll<St>`

```rust
struct SelectAll<St> {
    inner: crate::stream::FuturesUnordered<crate::stream::StreamFuture<St>>,
}
```

An unbounded set of streams

This "combinator" provides the ability to maintain a set of streams
and drive them all to completion.

Streams are pushed into this set and their realized values are
yielded as they become ready. Streams will only be polled when they
generate notifications. This allows to coordinate a large number of streams.

Note that you can create a ready-made `SelectAll` via the
`select_all` function in the `stream` module, or you can start with an
empty set with the `SelectAll::new` constructor.

#### Implementations

- <span id="selectall-new"></span>`fn new() -> Self`

  Constructs a new, empty `SelectAll`

  

  The returned `SelectAll` does not contain any streams and, in this

  state, `SelectAll::poll` will return `Poll::Ready(None)`.

- <span id="selectall-len"></span>`fn len(&self) -> usize`

  Returns the number of streams contained in the set.

  

  This represents the total number of in-flight streams.

- <span id="selectall-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the set contains no streams

- <span id="selectall-push"></span>`fn push(&mut self, stream: St)`

  Push a stream into the set.

  

  This function submits the given stream to the set for managing. This

  function will not call `poll` on the submitted stream. The caller must

  ensure that `SelectAll::poll` is called in order to receive task

  notifications.

- <span id="selectall-iter"></span>`fn iter(&self) -> Iter<'_, St>` — [`Iter`](#iter)

  Returns an iterator that allows inspecting each stream in the set.

- <span id="selectall-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, St>` — [`IterMut`](#itermut)

  Returns an iterator that allows modifying each stream in the set.

- <span id="selectall-clear"></span>`fn clear(&mut self)`

  Clears the set, removing all streams.

#### Trait Implementations

##### `impl<St: Debug> Debug for SelectAll<St>`

- <span id="selectall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream + Unpin> Default for SelectAll<St>`

- <span id="selectall-default"></span>`fn default() -> Self`

##### `impl<St: Stream + Unpin> Extend for SelectAll<St>`

- <span id="selectall-extend"></span>`fn extend<T: IntoIterator<Item = St>>(&mut self, iter: T)`

##### `impl<St: Stream + Unpin> FromIterator for SelectAll<St>`

- <span id="selectall-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = St>>(iter: T) -> Self`

##### `impl<St: Stream + Unpin> FusedStream for SelectAll<St>`

- <span id="selectall-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: Stream + Unpin> IntoIterator for SelectAll<St>`

- <span id="selectall-intoiterator-type-item"></span>`type Item = St`

- <span id="selectall-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<St>`

- <span id="selectall-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<St: Stream + Unpin> Stream for SelectAll<St>`

- <span id="selectall-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="selectall-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl StreamExt for SelectAll<St>`

##### `impl TryStream for SelectAll<St>`

- <span id="selectall-trystream-type-ok"></span>`type Ok = T`

- <span id="selectall-trystream-type-error"></span>`type Error = E`

- <span id="selectall-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for SelectAll<St>`

### `Iter<'a, St: Unpin>`

```rust
struct Iter<'a, St: Unpin>(futures_unordered::Iter<'a, crate::stream::StreamFuture<St>>);
```

Immutable iterator over all streams in the unordered set.

#### Trait Implementations

##### `impl<St: fmt::Debug + Unpin> Debug for Iter<'a, St>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream + Unpin> ExactSizeIterator for Iter<'_, St>`

##### `impl IntoIterator for Iter<'a, St>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<St: Stream + Unpin> Iterator for Iter<'a, St>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a St`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterMut<'a, St: Unpin>`

```rust
struct IterMut<'a, St: Unpin>(futures_unordered::IterMut<'a, crate::stream::StreamFuture<St>>);
```

Mutable iterator over all streams in the unordered set.

#### Trait Implementations

##### `impl<St: fmt::Debug + Unpin> Debug for IterMut<'a, St>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream + Unpin> ExactSizeIterator for IterMut<'_, St>`

##### `impl IntoIterator for IterMut<'a, St>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<St: Stream + Unpin> Iterator for IterMut<'a, St>`

- <span id="itermut-iterator-type-item"></span>`type Item = &'a mut St`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IntoIter<St: Unpin>`

```rust
struct IntoIter<St: Unpin>(futures_unordered::IntoIter<crate::stream::StreamFuture<St>>);
```

Owned iterator over all streams in the unordered set.

#### Trait Implementations

##### `impl<St: fmt::Debug + Unpin> Debug for IntoIter<St>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream + Unpin> ExactSizeIterator for IntoIter<St>`

##### `impl IntoIterator for IntoIter<St>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<St: Stream + Unpin> Iterator for IntoIter<St>`

- <span id="intoiter-iterator-type-item"></span>`type Item = St`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Functions

### `select_all`

```rust
fn select_all<I>(streams: I) -> SelectAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: Stream + Unpin
```

Convert a list of streams into a `Stream` of results from the streams.

This essentially takes a list of streams (e.g. a vector, an iterator, etc.)
and bundles them together into a single stream.
The stream will yield items as they become available on the underlying
streams internally, in the order they become available.

Note that the returned set can also be used to dynamically push more
streams into the set as they become available.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

