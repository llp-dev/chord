*[futures_util](../index.md) / [stream](index.md)*

---

# Module `stream`

Asynchronous streams.

This module contains:

- The [`Stream`](#stream) trait, for objects that can asynchronously produce a
  sequence of values.
- The [`StreamExt`](stream/index.md) and [`TryStreamExt`](try_stream/index.md) trait, which provides adapters for
  chaining and composing streams.
- Top-level stream constructors like [`iter`](iter()) which creates a
  stream from an iterator.

## Contents

- [Modules](#modules)
  - [`stream`](#stream)
  - [`try_stream`](#try-stream)
  - [`iter`](#iter)
  - [`repeat`](#repeat)
  - [`repeat_with`](#repeat-with)
  - [`empty`](#empty)
  - [`once`](#once)
  - [`pending`](#pending)
  - [`poll_fn`](#poll-fn)
  - [`poll_immediate`](#poll-immediate)
  - [`select`](#select)
  - [`select_with_strategy`](#select-with-strategy)
  - [`unfold`](#unfold)
  - [`futures_ordered`](#futures-ordered)
  - [`futures_unordered`](#futures-unordered)
  - [`select_all`](#select-all)
  - [`abortable`](#abortable)
- [Structs](#structs)
  - [`All`](#all)
  - [`Any`](#any)
  - [`Chain`](#chain)
  - [`Collect`](#collect)
  - [`Concat`](#concat)
  - [`Count`](#count)
  - [`Cycle`](#cycle)
  - [`Enumerate`](#enumerate)
  - [`Filter`](#filter)
  - [`FilterMap`](#filtermap)
  - [`FlatMap`](#flatmap)
  - [`Flatten`](#flatten)
  - [`Fold`](#fold)
  - [`ForEach`](#foreach)
  - [`Fuse`](#fuse)
  - [`Inspect`](#inspect)
  - [`Map`](#map)
  - [`Next`](#next)
  - [`NextIf`](#nextif)
  - [`NextIfEq`](#nextifeq)
  - [`Peek`](#peek)
  - [`PeekMut`](#peekmut)
  - [`Peekable`](#peekable)
  - [`Scan`](#scan)
  - [`SelectNextSome`](#selectnextsome)
  - [`Skip`](#skip)
  - [`SkipWhile`](#skipwhile)
  - [`StreamFuture`](#streamfuture)
  - [`Take`](#take)
  - [`TakeUntil`](#takeuntil)
  - [`TakeWhile`](#takewhile)
  - [`Then`](#then)
  - [`Unzip`](#unzip)
  - [`Zip`](#zip)
  - [`Chunks`](#chunks)
  - [`ReadyChunks`](#readychunks)
  - [`Forward`](#forward)
  - [`BufferUnordered`](#bufferunordered)
  - [`Buffered`](#buffered)
  - [`FlatMapUnordered`](#flatmapunordered)
  - [`ForEachConcurrent`](#foreachconcurrent)
  - [`ReuniteError`](#reuniteerror)
  - [`SplitSink`](#splitsink)
  - [`SplitStream`](#splitstream)
  - [`AndThen`](#andthen)
  - [`ErrInto`](#errinto)
  - [`InspectErr`](#inspecterr)
  - [`InspectOk`](#inspectok)
  - [`IntoStream`](#intostream)
  - [`MapErr`](#maperr)
  - [`MapOk`](#mapok)
  - [`OrElse`](#orelse)
  - [`TryAll`](#tryall)
  - [`TryAny`](#tryany)
  - [`TryCollect`](#trycollect)
  - [`TryConcat`](#tryconcat)
  - [`TryFilter`](#tryfilter)
  - [`TryFilterMap`](#tryfiltermap)
  - [`TryFlatten`](#tryflatten)
  - [`TryFold`](#tryfold)
  - [`TryForEach`](#tryforeach)
  - [`TryNext`](#trynext)
  - [`TrySkipWhile`](#tryskipwhile)
  - [`TryTakeWhile`](#trytakewhile)
  - [`TryUnfold`](#tryunfold)
  - [`TryBufferUnordered`](#trybufferunordered)
  - [`TryBuffered`](#trybuffered)
  - [`TryFlattenUnordered`](#tryflattenunordered)
  - [`TryForEachConcurrent`](#tryforeachconcurrent)
  - [`TryChunks`](#trychunks)
  - [`TryChunksError`](#trychunkserror)
  - [`TryReadyChunks`](#tryreadychunks)
  - [`TryReadyChunksError`](#tryreadychunkserror)
  - [`Iter`](#iter)
  - [`Repeat`](#repeat)
  - [`RepeatWith`](#repeatwith)
  - [`Empty`](#empty)
  - [`Once`](#once)
  - [`Pending`](#pending)
  - [`PollFn`](#pollfn)
  - [`PollImmediate`](#pollimmediate)
  - [`Select`](#select)
  - [`SelectWithStrategy`](#selectwithstrategy)
  - [`Unfold`](#unfold)
  - [`FuturesOrdered`](#futuresordered)
  - [`FuturesUnordered`](#futuresunordered)
  - [`SelectAll`](#selectall)
  - [`AbortHandle`](#aborthandle)
  - [`AbortRegistration`](#abortregistration)
  - [`Abortable`](#abortable)
  - [`Aborted`](#aborted)
- [Enums](#enums)
  - [`PollNext`](#pollnext)
- [Traits](#traits)
  - [`StreamExt`](#streamext)
  - [`TryStreamExt`](#trystreamext)
- [Functions](#functions)
  - [`BoxStream`](#boxstream)
  - [`TryStream`](#trystream)
  - [`try_unfold`](#try-unfold)
  - [`iter`](#iter)
  - [`repeat`](#repeat)
  - [`repeat_with`](#repeat-with)
  - [`empty`](#empty)
  - [`once`](#once)
  - [`pending`](#pending)
  - [`poll_fn`](#poll-fn)
  - [`poll_immediate`](#poll-immediate)
  - [`select`](#select)
  - [`select_with_strategy`](#select-with-strategy)
  - [`unfold`](#unfold)
  - [`select_all`](#select-all)
  - [`abortable`](#abortable)
  - [`assert_stream`](#assert-stream)
- [Type Aliases](#type-aliases)
  - [`FlattenUnordered`](#flattenunordered)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`stream`](#stream) | mod | Streams |
| [`try_stream`](#try-stream) | mod | Streams |
| [`iter`](#iter) | mod |  |
| [`repeat`](#repeat) | mod |  |
| [`repeat_with`](#repeat-with) | mod |  |
| [`empty`](#empty) | mod |  |
| [`once`](#once) | mod |  |
| [`pending`](#pending) | mod |  |
| [`poll_fn`](#poll-fn) | mod | Definition of the `PollFn` combinator |
| [`poll_immediate`](#poll-immediate) | mod |  |
| [`select`](#select) | mod |  |
| [`select_with_strategy`](#select-with-strategy) | mod |  |
| [`unfold`](#unfold) | mod |  |
| [`futures_ordered`](#futures-ordered) | mod |  |
| [`futures_unordered`](#futures-unordered) | mod | An unbounded set of futures. |
| [`select_all`](#select-all) | mod | An unbounded set of streams |
| [`abortable`](#abortable) | mod |  |
| [`All`](#all) | struct |  |
| [`Any`](#any) | struct |  |
| [`Chain`](#chain) | struct |  |
| [`Collect`](#collect) | struct |  |
| [`Concat`](#concat) | struct |  |
| [`Count`](#count) | struct |  |
| [`Cycle`](#cycle) | struct |  |
| [`Enumerate`](#enumerate) | struct |  |
| [`Filter`](#filter) | struct |  |
| [`FilterMap`](#filtermap) | struct |  |
| [`FlatMap`](#flatmap) | struct |  |
| [`Flatten`](#flatten) | struct |  |
| [`Fold`](#fold) | struct |  |
| [`ForEach`](#foreach) | struct |  |
| [`Fuse`](#fuse) | struct |  |
| [`Inspect`](#inspect) | struct |  |
| [`Map`](#map) | struct |  |
| [`Next`](#next) | struct |  |
| [`NextIf`](#nextif) | struct |  |
| [`NextIfEq`](#nextifeq) | struct |  |
| [`Peek`](#peek) | struct |  |
| [`PeekMut`](#peekmut) | struct |  |
| [`Peekable`](#peekable) | struct |  |
| [`Scan`](#scan) | struct |  |
| [`SelectNextSome`](#selectnextsome) | struct |  |
| [`Skip`](#skip) | struct |  |
| [`SkipWhile`](#skipwhile) | struct |  |
| [`StreamFuture`](#streamfuture) | struct |  |
| [`Take`](#take) | struct |  |
| [`TakeUntil`](#takeuntil) | struct |  |
| [`TakeWhile`](#takewhile) | struct |  |
| [`Then`](#then) | struct |  |
| [`Unzip`](#unzip) | struct |  |
| [`Zip`](#zip) | struct |  |
| [`Chunks`](#chunks) | struct |  |
| [`ReadyChunks`](#readychunks) | struct |  |
| [`Forward`](#forward) | struct |  |
| [`BufferUnordered`](#bufferunordered) | struct |  |
| [`Buffered`](#buffered) | struct |  |
| [`FlatMapUnordered`](#flatmapunordered) | struct |  |
| [`ForEachConcurrent`](#foreachconcurrent) | struct |  |
| [`ReuniteError`](#reuniteerror) | struct |  |
| [`SplitSink`](#splitsink) | struct |  |
| [`SplitStream`](#splitstream) | struct |  |
| [`AndThen`](#andthen) | struct |  |
| [`ErrInto`](#errinto) | struct |  |
| [`InspectErr`](#inspecterr) | struct |  |
| [`InspectOk`](#inspectok) | struct |  |
| [`IntoStream`](#intostream) | struct |  |
| [`MapErr`](#maperr) | struct |  |
| [`MapOk`](#mapok) | struct |  |
| [`OrElse`](#orelse) | struct |  |
| [`TryAll`](#tryall) | struct |  |
| [`TryAny`](#tryany) | struct |  |
| [`TryCollect`](#trycollect) | struct |  |
| [`TryConcat`](#tryconcat) | struct |  |
| [`TryFilter`](#tryfilter) | struct |  |
| [`TryFilterMap`](#tryfiltermap) | struct |  |
| [`TryFlatten`](#tryflatten) | struct |  |
| [`TryFold`](#tryfold) | struct |  |
| [`TryForEach`](#tryforeach) | struct |  |
| [`TryNext`](#trynext) | struct |  |
| [`TrySkipWhile`](#tryskipwhile) | struct |  |
| [`TryTakeWhile`](#trytakewhile) | struct |  |
| [`TryUnfold`](#tryunfold) | struct |  |
| [`TryBufferUnordered`](#trybufferunordered) | struct |  |
| [`TryBuffered`](#trybuffered) | struct |  |
| [`TryFlattenUnordered`](#tryflattenunordered) | struct |  |
| [`TryForEachConcurrent`](#tryforeachconcurrent) | struct |  |
| [`TryChunks`](#trychunks) | struct |  |
| [`TryChunksError`](#trychunkserror) | struct |  |
| [`TryReadyChunks`](#tryreadychunks) | struct |  |
| [`TryReadyChunksError`](#tryreadychunkserror) | struct |  |
| [`Iter`](#iter) | struct |  |
| [`Repeat`](#repeat) | struct |  |
| [`RepeatWith`](#repeatwith) | struct |  |
| [`Empty`](#empty) | struct |  |
| [`Once`](#once) | struct |  |
| [`Pending`](#pending) | struct |  |
| [`PollFn`](#pollfn) | struct |  |
| [`PollImmediate`](#pollimmediate) | struct |  |
| [`Select`](#select) | struct |  |
| [`SelectWithStrategy`](#selectwithstrategy) | struct |  |
| [`Unfold`](#unfold) | struct |  |
| [`FuturesOrdered`](#futuresordered) | struct |  |
| [`FuturesUnordered`](#futuresunordered) | struct |  |
| [`SelectAll`](#selectall) | struct |  |
| [`AbortHandle`](#aborthandle) | struct |  |
| [`AbortRegistration`](#abortregistration) | struct |  |
| [`Abortable`](#abortable) | struct |  |
| [`Aborted`](#aborted) | struct |  |
| [`PollNext`](#pollnext) | enum |  |
| [`StreamExt`](#streamext) | trait |  |
| [`TryStreamExt`](#trystreamext) | trait |  |
| [`BoxStream`](#boxstream) | fn |  |
| [`TryStream`](#trystream) | fn |  |
| [`try_unfold`](#try-unfold) | fn |  |
| [`iter`](#iter) | fn |  |
| [`repeat`](#repeat) | fn |  |
| [`repeat_with`](#repeat-with) | fn |  |
| [`empty`](#empty) | fn |  |
| [`once`](#once) | fn |  |
| [`pending`](#pending) | fn |  |
| [`poll_fn`](#poll-fn) | fn |  |
| [`poll_immediate`](#poll-immediate) | fn |  |
| [`select`](#select) | fn |  |
| [`select_with_strategy`](#select-with-strategy) | fn |  |
| [`unfold`](#unfold) | fn |  |
| [`select_all`](#select-all) | fn |  |
| [`abortable`](#abortable) | fn |  |
| [`assert_stream`](#assert-stream) | fn |  |
| [`FlattenUnordered`](#flattenunordered) | type |  |

## Modules

- [`stream`](stream/index.md) — Streams
- [`try_stream`](try_stream/index.md) — Streams
- [`iter`](iter/index.md)
- [`repeat`](repeat/index.md)
- [`repeat_with`](repeat_with/index.md)
- [`empty`](empty/index.md)
- [`once`](once/index.md)
- [`pending`](pending/index.md)
- [`poll_fn`](poll_fn/index.md) — Definition of the `PollFn` combinator
- [`poll_immediate`](poll_immediate/index.md)
- [`select`](select/index.md)
- [`select_with_strategy`](select_with_strategy/index.md)
- [`unfold`](unfold/index.md)
- [`futures_ordered`](futures_ordered/index.md)
- [`futures_unordered`](futures_unordered/index.md) — An unbounded set of futures.
- [`select_all`](select_all/index.md) — An unbounded set of streams
- [`abortable`](abortable/index.md)

## Structs

### `All<St, Fut, F>`

```rust
struct All<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`all`](super::StreamExt::all) method.

#### Implementations

- <span id="all-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for All<St, Fut, F>`

- <span id="all-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for All<St, Fut, F>`

- <span id="all-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for All<St, Fut, F>`

- <span id="all-future-type-output"></span>`type Output = bool`

- <span id="all-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for All<St, Fut, F>`

##### `impl<F> IntoFuture for All<St, Fut, F>`

- <span id="all-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="all-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="all-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for All<St, Fut, F>`

### `Any<St, Fut, F>`

```rust
struct Any<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`any`](super::StreamExt::any) method.

#### Implementations

- <span id="any-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for Any<St, Fut, F>`

- <span id="any-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for Any<St, Fut, F>`

- <span id="any-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for Any<St, Fut, F>`

- <span id="any-future-type-output"></span>`type Output = bool`

- <span id="any-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for Any<St, Fut, F>`

##### `impl<F> IntoFuture for Any<St, Fut, F>`

- <span id="any-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="any-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="any-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for Any<St, Fut, F>`

### `Chain<St1, St2>`

```rust
struct Chain<St1, St2> {
    first: Option<St1>,
    second: St2,
}
```

Stream for the [`chain`](super::StreamExt::chain) method.

#### Implementations

- <span id="chain-new"></span>`fn new(stream1: St1, stream2: St2) -> Self`

#### Trait Implementations

##### `impl<St1: fmt::Debug, St2: fmt::Debug> Debug for Chain<St1, St2>`

- <span id="chain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St1, St2> FusedStream for Chain<St1, St2>`

- <span id="chain-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St1, St2> Stream for Chain<St1, St2>`

- <span id="chain-stream-type-item"></span>`type Item = <St1 as Stream>::Item`

- <span id="chain-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="chain-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Chain<St1, St2>`

##### `impl TryStream for Chain<St1, St2>`

- <span id="chain-trystream-type-ok"></span>`type Ok = T`

- <span id="chain-trystream-type-error"></span>`type Error = E`

- <span id="chain-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Chain<St1, St2>`

##### `impl<St1, St2> Unpin for Chain<St1, St2>`

### `Collect<St, C>`

```rust
struct Collect<St, C> {
    stream: St,
    collection: C,
}
```

Future for the [`collect`](super::StreamExt::collect) method.

#### Implementations

- <span id="collect-finish"></span>`fn finish(self: Pin<&mut Self>) -> C`

- <span id="collect-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, C: fmt::Debug> Debug for Collect<St, C>`

- <span id="collect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, C> FusedFuture for Collect<St, C>`

- <span id="collect-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, C> Future for Collect<St, C>`

- <span id="collect-future-type-output"></span>`type Output = C`

- <span id="collect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<C>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for Collect<St, C>`

##### `impl IntoFuture for Collect<St, C>`

- <span id="collect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="collect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="collect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, C> Unpin for Collect<St, C>`

### `Concat<St: Stream>`

```rust
struct Concat<St: Stream> {
    stream: St,
    accum: Option<<St as >::Item>,
}
```

Future for the [`concat`](super::StreamExt::concat) method.

#### Implementations

- <span id="concat-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + Stream> Debug for Concat<St>`

- <span id="concat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedFuture for Concat<St>`

- <span id="concat-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Future for Concat<St>`

- <span id="concat-future-type-output"></span>`type Output = <St as Stream>::Item`

- <span id="concat-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Concat<St>`

##### `impl IntoFuture for Concat<St>`

- <span id="concat-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="concat-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="concat-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Concat<St>`

- <span id="concat-tryfuture-type-ok"></span>`type Ok = T`

- <span id="concat-tryfuture-type-error"></span>`type Error = E`

- <span id="concat-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Concat<St>`

##### `impl<St: Stream> Unpin for Concat<St>`

### `Count<St>`

```rust
struct Count<St> {
    stream: St,
    count: usize,
}
```

Future for the [`count`](super::StreamExt::count) method.

#### Implementations

- <span id="count-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St> Debug for Count<St>`

- <span id="count-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: FusedStream> FusedFuture for Count<St>`

- <span id="count-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: Stream> Future for Count<St>`

- <span id="count-future-type-output"></span>`type Output = usize`

- <span id="count-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Count<St>`

##### `impl IntoFuture for Count<St>`

- <span id="count-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="count-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="count-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St> Unpin for Count<St>`

### `Cycle<St>`

```rust
struct Cycle<St> {
    orig: St,
    stream: St,
}
```

Stream for the [`cycle`](super::StreamExt::cycle) method.

#### Implementations

- <span id="cycle-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Cycle<St>`

- <span id="cycle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for Cycle<St>`

- <span id="cycle-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Stream for Cycle<St>`

- <span id="cycle-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="cycle-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="cycle-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Cycle<St>`

##### `impl TryStream for Cycle<St>`

- <span id="cycle-trystream-type-ok"></span>`type Ok = T`

- <span id="cycle-trystream-type-error"></span>`type Error = E`

- <span id="cycle-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Cycle<St>`

##### `impl<St> Unpin for Cycle<St>`

### `Enumerate<St>`

```rust
struct Enumerate<St> {
    stream: St,
    count: usize,
}
```

Stream for the [`enumerate`](super::StreamExt::enumerate) method.

#### Implementations

- <span id="enumerate-new"></span>`fn new(stream: St) -> Self`

- <span id="enumerate-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="enumerate-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="enumerate-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="enumerate-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Enumerate<St>`

- <span id="enumerate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream + FusedStream> FusedStream for Enumerate<St>`

- <span id="enumerate-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Enumerate<S>`

- <span id="enumerate-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="enumerate-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="enumerate-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="enumerate-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="enumerate-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Enumerate<St>`

##### `impl<St: Stream> Stream for Enumerate<St>`

- <span id="enumerate-stream-type-item"></span>`type Item = (usize, <St as Stream>::Item)`

- <span id="enumerate-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="enumerate-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Enumerate<St>`

##### `impl<St> Unpin for Enumerate<St>`

### `Filter<St, Fut, F>`

```rust
struct Filter<St, Fut, F>
where
    St: Stream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Item>,
}
```

Stream for the [`filter`](super::StreamExt::filter) method.

#### Implementations

- <span id="filter-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="filter-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="filter-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filter-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filter-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for Filter<St, Fut, F>`

- <span id="filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for Filter<St, Fut, F>`

- <span id="filter-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for Filter<S, Fut, F>`

- <span id="filter-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="filter-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="filter-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="filter-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="filter-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Filter<St, Fut, F>`

##### `impl<St, Fut, F> Stream for Filter<St, Fut, F>`

- <span id="filter-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="filter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="filter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Filter<St, Fut, F>`

##### `impl TryStream for Filter<St, Fut, F>`

- <span id="filter-trystream-type-ok"></span>`type Ok = T`

- <span id="filter-trystream-type-error"></span>`type Error = E`

- <span id="filter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Filter<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for Filter<St, Fut, F>`

### `FilterMap<St, Fut, F>`

```rust
struct FilterMap<St, Fut, F> {
    stream: St,
    f: F,
    pending: Option<Fut>,
}
```

Stream for the [`filter_map`](super::StreamExt::filter_map) method.

#### Implementations

- <span id="filtermap-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="filtermap-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="filtermap-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filtermap-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filtermap-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for FilterMap<St, Fut, F>`

- <span id="filtermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for FilterMap<St, Fut, F>`

- <span id="filtermap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for FilterMap<S, Fut, F>`

- <span id="filtermap-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="filtermap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="filtermap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="filtermap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="filtermap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for FilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Stream for FilterMap<St, Fut, F>`

- <span id="filtermap-stream-type-item"></span>`type Item = T`

- <span id="filtermap-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

- <span id="filtermap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FilterMap<St, Fut, F>`

##### `impl TryStream for FilterMap<St, Fut, F>`

- <span id="filtermap-trystream-type-ok"></span>`type Ok = T`

- <span id="filtermap-trystream-type-error"></span>`type Error = E`

- <span id="filtermap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for FilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for FilterMap<St, Fut, F>`

### `FlatMap<St, U, F>`

```rust
struct FlatMap<St, U, F> {
    inner: flatten::Flatten<Map<St, F>, U>,
}
```

Stream for the [`flat_map`](StreamExt::flat_map) method.

#### Implementations

- <span id="flatmap-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, U, F> Debug for FlatMap<St, U, F>`

- <span id="flatmap-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, U, F> FusedStream for FlatMap<St, U, F>`

- <span id="flatmap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, U, F> Sink for FlatMap<St, U, F>`

- <span id="flatmap-sink-type-error"></span>`type Error = <Flatten<Map<St, F>, U> as Sink>::Error`

- <span id="flatmap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flatmap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="flatmap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flatmap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for FlatMap<St, U, F>`

##### `impl<St, U, F> Stream for FlatMap<St, U, F>`

- <span id="flatmap-stream-type-item"></span>`type Item = <Flatten<Map<St, F>, U> as Stream>::Item`

- <span id="flatmap-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="flatmap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlatMap<St, U, F>`

##### `impl TryStream for FlatMap<St, U, F>`

- <span id="flatmap-trystream-type-ok"></span>`type Ok = T`

- <span id="flatmap-trystream-type-error"></span>`type Error = E`

- <span id="flatmap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for FlatMap<St, U, F>`

##### `impl<St, U, F> Unpin for FlatMap<St, U, F>`

### `Flatten<St>`

```rust
struct Flatten<St>
where
    St: Stream {
    inner: flatten::Flatten<St, <St as >::Item>,
}
```

Stream for the [`flatten`](StreamExt::flatten) method.

#### Implementations

- <span id="flatten-new"></span>`fn new(x: St) -> Self`

#### Trait Implementations

##### `impl<St> Debug for Flatten<St>`

- <span id="flatten-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St> FusedStream for Flatten<St>`

- <span id="flatten-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St> Sink for Flatten<St>`

- <span id="flatten-sink-type-error"></span>`type Error = <Flatten<St, <St as Stream>::Item> as Sink>::Error`

- <span id="flatten-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flatten-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="flatten-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flatten-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Flatten<St>`

##### `impl<St> Stream for Flatten<St>`

- <span id="flatten-stream-type-item"></span>`type Item = <Flatten<St, <St as Stream>::Item> as Stream>::Item`

- <span id="flatten-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="flatten-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Flatten<St>`

##### `impl TryStream for Flatten<St>`

- <span id="flatten-trystream-type-ok"></span>`type Ok = T`

- <span id="flatten-trystream-type-error"></span>`type Error = E`

- <span id="flatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Flatten<St>`

##### `impl<St> Unpin for Flatten<St>`

### `Fold<St, Fut, T, F>`

```rust
struct Fold<St, Fut, T, F> {
    stream: St,
    f: F,
    accum: Option<T>,
    future: Option<Fut>,
}
```

Future for the [`fold`](super::StreamExt::fold) method.

#### Implementations

- <span id="fold-new"></span>`fn new(stream: St, f: F, t: T) -> Self`

#### Trait Implementations

##### `impl<St, Fut, T, F> Debug for Fold<St, Fut, T, F>`

- <span id="fold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, T, F> FusedFuture for Fold<St, Fut, T, F>`

- <span id="fold-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, T, F> Future for Fold<St, Fut, T, F>`

- <span id="fold-future-type-output"></span>`type Output = T`

- <span id="fold-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl<T> FutureExt for Fold<St, Fut, T, F>`

##### `impl<F> IntoFuture for Fold<St, Fut, T, F>`

- <span id="fold-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="fold-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="fold-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for Fold<St, Fut, T, F>`

- <span id="fold-tryfuture-type-ok"></span>`type Ok = T`

- <span id="fold-tryfuture-type-error"></span>`type Error = E`

- <span id="fold-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<Fut> TryFutureExt for Fold<St, Fut, T, F>`

##### `impl<St, Fut, T, F> Unpin for Fold<St, Fut, T, F>`

### `ForEach<St, Fut, F>`

```rust
struct ForEach<St, Fut, F> {
    stream: St,
    f: F,
    future: Option<Fut>,
}
```

Future for the [`for_each`](super::StreamExt::for_each) method.

#### Implementations

- <span id="foreach-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for ForEach<St, Fut, F>`

- <span id="foreach-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for ForEach<St, Fut, F>`

- <span id="foreach-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for ForEach<St, Fut, F>`

- <span id="foreach-future-type-output"></span>`type Output = ()`

- <span id="foreach-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for ForEach<St, Fut, F>`

##### `impl<F> IntoFuture for ForEach<St, Fut, F>`

- <span id="foreach-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="foreach-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="foreach-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for ForEach<St, Fut, F>`

### `Fuse<St>`

```rust
struct Fuse<St> {
    stream: St,
    done: bool,
}
```

Stream for the [`fuse`](super::StreamExt::fuse) method.

#### Implementations

- <span id="fuse-new"></span>`fn new(stream: St) -> Self`

- <span id="fuse-is-done"></span>`fn is_done(&self) -> bool`

  Returns whether the underlying stream has finished or not.

  

  If this method returns `true`, then all future calls to poll are

  guaranteed to return `None`. If this returns `false`, then the

  underlying stream is still in use.

- <span id="fuse-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="fuse-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="fuse-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="fuse-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Fuse<St>`

- <span id="fuse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Stream> FusedStream for Fuse<S>`

- <span id="fuse-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S: Stream + Sink<Item>, Item> Sink for Fuse<S>`

- <span id="fuse-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="fuse-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="fuse-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="fuse-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="fuse-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Fuse<St>`

##### `impl<S: Stream> Stream for Fuse<S>`

- <span id="fuse-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="fuse-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="fuse-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Fuse<St>`

##### `impl TryStream for Fuse<St>`

- <span id="fuse-trystream-type-ok"></span>`type Ok = T`

- <span id="fuse-trystream-type-error"></span>`type Error = E`

- <span id="fuse-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Fuse<St>`

##### `impl<St> Unpin for Fuse<St>`

### `Inspect<St, F>`

```rust
struct Inspect<St, F> {
    inner: map::Map<St, crate::fns::InspectFn<F>>,
}
```

Stream for the [`inspect`](StreamExt::inspect) method.

#### Implementations

- <span id="inspect-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for Inspect<St, F>`

- <span id="inspect-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for Inspect<St, F>`

- <span id="inspect-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for Inspect<St, F>`

- <span id="inspect-sink-type-error"></span>`type Error = <Map<St, InspectFn<F>> as Sink>::Error`

- <span id="inspect-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="inspect-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="inspect-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="inspect-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Inspect<St, F>`

##### `impl<St, F> Stream for Inspect<St, F>`

- <span id="inspect-stream-type-item"></span>`type Item = <Map<St, InspectFn<F>> as Stream>::Item`

- <span id="inspect-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="inspect-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Inspect<St, F>`

##### `impl TryStream for Inspect<St, F>`

- <span id="inspect-trystream-type-ok"></span>`type Ok = T`

- <span id="inspect-trystream-type-error"></span>`type Error = E`

- <span id="inspect-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Inspect<St, F>`

##### `impl<St, F> Unpin for Inspect<St, F>`

### `Map<St, F>`

```rust
struct Map<St, F> {
    stream: St,
    f: F,
}
```

Stream for the [`map`](super::StreamExt::map) method.

#### Implementations

- <span id="map-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="map-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="map-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="map-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="map-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, F> Debug for Map<St, F>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, F> FusedStream for Map<St, F>`

- <span id="map-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, F, Item> Sink for Map<St, F>`

- <span id="map-sink-type-error"></span>`type Error = <St as Sink>::Error`

- <span id="map-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="map-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="map-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="map-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Map<St, F>`

##### `impl<St, F> Stream for Map<St, F>`

- <span id="map-stream-type-item"></span>`type Item = <F as FnOnce1>::Output`

- <span id="map-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="map-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Map<St, F>`

##### `impl TryStream for Map<St, F>`

- <span id="map-trystream-type-ok"></span>`type Ok = T`

- <span id="map-trystream-type-error"></span>`type Error = E`

- <span id="map-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Map<St, F>`

##### `impl<St, F> Unpin for Map<St, F>`

### `Next<'a, St: ?Sized>`

```rust
struct Next<'a, St: ?Sized> {
    stream: &'a mut St,
}
```

Future for the [`next`](super::StreamExt::next) method.

#### Implementations

- <span id="next-new"></span>`fn new(stream: &'a mut St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + ?Sized> Debug for Next<'a, St>`

- <span id="next-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: ?Sized + FusedStream + Unpin> FusedFuture for Next<'_, St>`

- <span id="next-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: ?Sized + Stream + Unpin> Future for Next<'_, St>`

- <span id="next-future-type-output"></span>`type Output = Option<<St as Stream>::Item>`

- <span id="next-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Next<'a, St>`

##### `impl IntoFuture for Next<'a, St>`

- <span id="next-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="next-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="next-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: ?Sized + Unpin> Unpin for Next<'_, St>`

### `NextIf<'a, St: Stream, F>`

```rust
struct NextIf<'a, St: Stream, F> {
    inner: Option<(core::pin::Pin<&'a mut Peekable<St>>, F)>,
}
```

Future for the [`Peekable::next_if`](self::Peekable::next_if) method.

#### Trait Implementations

##### `impl<St, F> Debug for NextIf<'_, St, F>`

- <span id="nextif-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, F> FusedFuture for NextIf<'_, St, F>`

- <span id="nextif-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, F> Future for NextIf<'_, St, F>`

- <span id="nextif-future-type-output"></span>`type Output = Option<<St as Stream>::Item>`

- <span id="nextif-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for NextIf<'a, St, F>`

##### `impl<F> IntoFuture for NextIf<'a, St, F>`

- <span id="nextif-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="nextif-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="nextif-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream, F> Unpin for NextIf<'a, St, F>`

### `NextIfEq<'a, St: Stream, T: ?Sized>`

```rust
struct NextIfEq<'a, St: Stream, T: ?Sized> {
    inner: NextIf<'a, St, NextIfEqFn<'a, T, <St as >::Item>>,
}
```

Future for the [`Peekable::next_if_eq`](self::Peekable::next_if_eq) method.

#### Trait Implementations

##### `impl<St, T> Debug for NextIfEq<'_, St, T>`

- <span id="nextifeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, T> FusedFuture for NextIfEq<'_, St, T>`

- <span id="nextifeq-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, T> Future for NextIfEq<'_, St, T>`

- <span id="nextifeq-future-type-output"></span>`type Output = Option<<St as Stream>::Item>`

- <span id="nextifeq-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<T> FutureExt for NextIfEq<'a, St, T>`

##### `impl IntoFuture for NextIfEq<'a, St, T>`

- <span id="nextifeq-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="nextifeq-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="nextifeq-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream, T: ?Sized> Unpin for NextIfEq<'a, St, T>`

### `Peek<'a, St: Stream>`

```rust
struct Peek<'a, St: Stream> {
    inner: Option<core::pin::Pin<&'a mut Peekable<St>>>,
}
```

Future for the [`Peekable::peek`](self::Peekable::peek) method.

#### Trait Implementations

##### `impl<St> Debug for Peek<'_, St>`

- <span id="peek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream> FusedFuture for Peek<'_, St>`

- <span id="peek-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Future for Peek<'a, St>`

- <span id="peek-future-type-output"></span>`type Output = Option<&'a <St as Stream>::Item>`

- <span id="peek-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Peek<'a, St>`

##### `impl IntoFuture for Peek<'a, St>`

- <span id="peek-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="peek-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="peek-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream> Unpin for Peek<'a, St>`

### `PeekMut<'a, St: Stream>`

```rust
struct PeekMut<'a, St: Stream> {
    inner: Option<core::pin::Pin<&'a mut Peekable<St>>>,
}
```

Future for the [`Peekable::peek_mut`](self::Peekable::peek_mut) method.

#### Trait Implementations

##### `impl<St> Debug for PeekMut<'_, St>`

- <span id="peekmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream> FusedFuture for PeekMut<'_, St>`

- <span id="peekmut-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Future for PeekMut<'a, St>`

- <span id="peekmut-future-type-output"></span>`type Output = Option<&'a mut <St as Stream>::Item>`

- <span id="peekmut-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for PeekMut<'a, St>`

##### `impl IntoFuture for PeekMut<'a, St>`

- <span id="peekmut-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="peekmut-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="peekmut-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream> Unpin for PeekMut<'a, St>`

### `Peekable<St: Stream>`

```rust
struct Peekable<St: Stream> {
    stream: crate::stream::Fuse<St>,
    peeked: Option<<St as >::Item>,
}
```

A `Stream` that implements a `peek` method.

The `peek` method can be used to retrieve a reference
to the next `Stream::Item` if available. A subsequent
call to `poll` will return the owned item.

#### Implementations

- <span id="peekable-new"></span>`fn new(stream: St) -> Self`

- <span id="peekable-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="peekable-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="peekable-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="peekable-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="peekable-peek"></span>`fn peek(self: Pin<&mut Self>) -> Peek<'_, St>` — [`Peek`](stream/peek/index.md#peek)

  Produces a future which retrieves a reference to the next item

  in the stream, or `None` if the underlying stream terminates.

- <span id="peekable-poll-peek"></span>`fn poll_peek(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&<St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

  Peek retrieves a reference to the next item in the stream.

  

  This method polls the underlying stream and return either a reference

  to the next item if the stream is ready or passes through any errors.

- <span id="peekable-peek-mut"></span>`fn peek_mut(self: Pin<&mut Self>) -> PeekMut<'_, St>` — [`PeekMut`](stream/peek/index.md#peekmut)

  Produces a future which retrieves a mutable reference to the next item

  in the stream, or `None` if the underlying stream terminates.

  

  # Examples

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(vec![1, 2, 3]).peekable();

  let mut stream = pin!(stream);

  

  assert_eq!(stream.as_mut().peek_mut().await, Some(&mut 1));

  assert_eq!(stream.as_mut().next().await, Some(1));

  

  // Peek into the stream and modify the value which will be returned next

  if let Some(p) = stream.as_mut().peek_mut().await {

      if *p == 2 {

          *p = 5;

      }

  }

  

  assert_eq!(stream.collect::<Vec<_>>().await, vec![5, 3]);

  });

  ```

- <span id="peekable-poll-peek-mut"></span>`fn poll_peek_mut(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&mut <St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

  Peek retrieves a mutable reference to the next item in the stream.

- <span id="peekable-next-if"></span>`fn next_if<F>(self: Pin<&mut Self>, func: F) -> NextIf<'_, St, F>` — [`NextIf`](stream/peek/index.md#nextif)

  Creates a future which will consume and return the next value of this

  stream if a condition is true.

  

  If `func` returns `true` for the next value of this stream, consume and

  return it. Otherwise, return `None`.

  

  # Examples

  

  Consume a number if it's equal to 0.

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(0..5).peekable();

  let mut stream = pin!(stream);

  // The first item of the stream is 0; consume it.

  assert_eq!(stream.as_mut().next_if(|&x| x == 0).await, Some(0));

  // The next item returned is now 1, so `consume` will return `false`.

  assert_eq!(stream.as_mut().next_if(|&x| x == 0).await, None);

  // `next_if` saves the value of the next item if it was not equal to `expected`.

  assert_eq!(stream.next().await, Some(1));

  });

  ```

  

  Consume any number less than 10.

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(1..20).peekable();

  let mut stream = pin!(stream);

  // Consume all numbers less than 10

  while stream.as_mut().next_if(|&x| x < 10).await.is_some() {}

  // The next value returned will be 10

  assert_eq!(stream.next().await, Some(10));

  });

  ```

- <span id="peekable-next-if-eq"></span>`fn next_if_eq<'a, T>(self: Pin<&'a mut Self>, expected: &'a T) -> NextIfEq<'a, St, T>` — [`NextIfEq`](stream/peek/index.md#nextifeq)

  Creates a future which will consume and return the next item if it is

  equal to `expected`.

  

  # Example

  

  Consume a number if it's equal to 0.

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(0..5).peekable();

  let mut stream = pin!(stream);

  // The first item of the stream is 0; consume it.

  assert_eq!(stream.as_mut().next_if_eq(&0).await, Some(0));

  // The next item returned is now 1, so `consume` will return `false`.

  assert_eq!(stream.as_mut().next_if_eq(&0).await, None);

  // `next_if_eq` saves the value of the next item if it was not equal to `expected`.

  assert_eq!(stream.next().await, Some(1));

  });

  ```

#### Trait Implementations

##### `impl<St: fmt::Debug + Stream> Debug for Peekable<St>`

- <span id="peekable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream> FusedStream for Peekable<St>`

- <span id="peekable-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Peekable<S>`

- <span id="peekable-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="peekable-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="peekable-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="peekable-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="peekable-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Peekable<St>`

##### `impl<S: Stream> Stream for Peekable<S>`

- <span id="peekable-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="peekable-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="peekable-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Peekable<St>`

##### `impl TryStream for Peekable<St>`

- <span id="peekable-trystream-type-ok"></span>`type Ok = T`

- <span id="peekable-trystream-type-error"></span>`type Error = E`

- <span id="peekable-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Peekable<St>`

##### `impl<St: Stream> Unpin for Peekable<St>`

### `Scan<St: Stream, S, Fut, F>`

```rust
struct Scan<St: Stream, S, Fut, F> {
    stream: St,
    state_f: Option<StateFn<S, F>>,
    future: Option<Fut>,
}
```

Stream for the [`scan`](super::StreamExt::scan) method.

#### Implementations

- <span id="scan-is-done-taking"></span>`fn is_done_taking(&self) -> bool`

  Checks if internal state is `None`.

#### Trait Implementations

##### `impl<St, S, Fut, F> Debug for Scan<St, S, Fut, F>`

- <span id="scan-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, S, Fut, F> FusedStream for Scan<St, S, Fut, F>`

- <span id="scan-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, S, Fut, F, Item> Sink for Scan<St, S, Fut, F>`

- <span id="scan-sink-type-error"></span>`type Error = <St as Sink>::Error`

- <span id="scan-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="scan-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="scan-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="scan-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Scan<St, S, Fut, F>`

##### `impl<St, S, Fut, F> Stream for Scan<St, S, Fut, F>`

- <span id="scan-stream-type-item"></span>`type Item = B`

- <span id="scan-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<B>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

- <span id="scan-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Scan<St, S, Fut, F>`

##### `impl<S> TryStream for Scan<St, S, Fut, F>`

- <span id="scan-trystream-type-ok"></span>`type Ok = T`

- <span id="scan-trystream-type-error"></span>`type Error = E`

- <span id="scan-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl<S> TryStreamExt for Scan<St, S, Fut, F>`

##### `impl<St: Stream, S, Fut, F> Unpin for Scan<St, S, Fut, F>`

### `SelectNextSome<'a, St: ?Sized>`

```rust
struct SelectNextSome<'a, St: ?Sized> {
    stream: &'a mut St,
}
```

Future for the [`select_next_some`](super::StreamExt::select_next_some)
method.

#### Implementations

- <span id="selectnextsome-new"></span>`fn new(stream: &'a mut St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + ?Sized> Debug for SelectNextSome<'a, St>`

- <span id="selectnextsome-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: ?Sized + FusedStream + Unpin> FusedFuture for SelectNextSome<'_, St>`

- <span id="selectnextsome-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: ?Sized + FusedStream + Unpin> Future for SelectNextSome<'_, St>`

- <span id="selectnextsome-future-type-output"></span>`type Output = <St as Stream>::Item`

- <span id="selectnextsome-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for SelectNextSome<'a, St>`

##### `impl IntoFuture for SelectNextSome<'a, St>`

- <span id="selectnextsome-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="selectnextsome-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="selectnextsome-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for SelectNextSome<'a, St>`

- <span id="selectnextsome-tryfuture-type-ok"></span>`type Ok = T`

- <span id="selectnextsome-tryfuture-type-error"></span>`type Error = E`

- <span id="selectnextsome-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for SelectNextSome<'a, St>`

### `Skip<St>`

```rust
struct Skip<St> {
    stream: St,
    remaining: usize,
}
```

Stream for the [`skip`](super::StreamExt::skip) method.

#### Implementations

- <span id="skip-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="skip-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="skip-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skip-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skip-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Skip<St>`

- <span id="skip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: FusedStream> FusedStream for Skip<St>`

- <span id="skip-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Skip<S>`

- <span id="skip-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="skip-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="skip-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="skip-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="skip-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Skip<St>`

##### `impl<St: Stream> Stream for Skip<St>`

- <span id="skip-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="skip-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="skip-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Skip<St>`

##### `impl TryStream for Skip<St>`

- <span id="skip-trystream-type-ok"></span>`type Ok = T`

- <span id="skip-trystream-type-error"></span>`type Error = E`

- <span id="skip-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Skip<St>`

##### `impl<St> Unpin for Skip<St>`

### `SkipWhile<St, Fut, F>`

```rust
struct SkipWhile<St, Fut, F>
where
    St: Stream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Item>,
    done_skipping: bool,
}
```

Stream for the [`skip_while`](super::StreamExt::skip_while) method.

#### Implementations

- <span id="skipwhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="skipwhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="skipwhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skipwhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skipwhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for SkipWhile<St, Fut, F>`

- <span id="skipwhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for SkipWhile<S, Fut, F>`

- <span id="skipwhile-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="skipwhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="skipwhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="skipwhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="skipwhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for SkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="skipwhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="skipwhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for SkipWhile<St, Fut, F>`

##### `impl TryStream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-trystream-type-ok"></span>`type Ok = T`

- <span id="skipwhile-trystream-type-error"></span>`type Error = E`

- <span id="skipwhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for SkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for SkipWhile<St, Fut, F>`

### `StreamFuture<St>`

```rust
struct StreamFuture<St> {
    stream: Option<St>,
}
```

Future for the [`into_future`](super::StreamExt::into_future) method.

#### Implementations

- <span id="streamfuture-new"></span>`fn new(stream: St) -> Self`

- <span id="streamfuture-get-ref"></span>`fn get_ref(&self) -> Option<&St>`

  Acquires a reference to the underlying stream that this combinator is

  pulling from.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

- <span id="streamfuture-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut St>`

  Acquires a mutable reference to the underlying stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

- <span id="streamfuture-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut St>>`

  Acquires a pinned mutable reference to the underlying stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

- <span id="streamfuture-into-inner"></span>`fn into_inner(self) -> Option<St>`

  Consumes this combinator, returning the underlying stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for StreamFuture<St>`

- <span id="streamfuture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream + Unpin> FusedFuture for StreamFuture<St>`

- <span id="streamfuture-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: Stream + Unpin> Future for StreamFuture<St>`

- <span id="streamfuture-future-type-output"></span>`type Output = (Option<<St as Stream>::Item>, St)`

- <span id="streamfuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for StreamFuture<St>`

##### `impl IntoFuture for StreamFuture<St>`

- <span id="streamfuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="streamfuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="streamfuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

### `Take<St>`

```rust
struct Take<St> {
    stream: St,
    remaining: usize,
}
```

Stream for the [`take`](super::StreamExt::take) method.

#### Implementations

- <span id="take-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="take-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="take-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="take-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="take-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Take<St>`

- <span id="take-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for Take<St>`

- <span id="take-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Take<S>`

- <span id="take-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="take-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="take-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="take-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="take-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Take<St>`

##### `impl<St> Stream for Take<St>`

- <span id="take-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="take-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="take-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Take<St>`

##### `impl TryStream for Take<St>`

- <span id="take-trystream-type-ok"></span>`type Ok = T`

- <span id="take-trystream-type-error"></span>`type Error = E`

- <span id="take-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Take<St>`

##### `impl<St> Unpin for Take<St>`

### `TakeUntil<St: Stream, Fut: Future>`

```rust
struct TakeUntil<St: Stream, Fut: Future> {
    stream: St,
    fut: Option<Fut>,
    fut_result: Option<<Fut as >::Output>,
    free: bool,
}
```

Stream for the [`take_until`](super::StreamExt::take_until) method.

#### Implementations

- <span id="takeuntil-new"></span>`fn new(stream: St, fut: Fut) -> Self`

- <span id="takeuntil-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="takeuntil-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takeuntil-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takeuntil-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="takeuntil-take-future"></span>`fn take_future(&mut self) -> Option<Fut>`

  Extract the stopping future out of the combinator.

  The future is returned only if it isn't resolved yet, ie. if the stream isn't stopped yet.

  Taking out the future means the combinator will be yielding

  elements from the wrapped stream without ever stopping it.

- <span id="takeuntil-take-result"></span>`fn take_result(&mut self) -> Option<<Fut as >::Output>` — [`Future`](../future/index.md#future)

  Once the stopping future is resolved, this method can be used

  to extract the value returned by the stopping future.

  

  This may be used to retrieve arbitrary data from the stopping

  future, for example a reason why the stream was stopped.

  

  This method will return `None` if the future isn't resolved yet,

  or if the result was already taken out.

  

  # Examples

  

  ```rust

  futures::executor::block_on(async {

  use futures::future;

  use futures::stream::{self, StreamExt};

  use futures::task::Poll;

  

  let stream = stream::iter(1..=10);

  

  let mut i = 0;

  let stop_fut = future::poll_fn(|_cx| {

      i += 1;

      if i <= 5 {

          Poll::Pending

      } else {

          Poll::Ready("reason")

      }

  });

  

  let mut stream = stream.take_until(stop_fut);

  let _ = stream.by_ref().collect::<Vec<_>>().await;

  

  let result = stream.take_result().unwrap();

  assert_eq!(result, "reason");

  });

  ```

- <span id="takeuntil-is-stopped"></span>`fn is_stopped(&self) -> bool`

  Whether the stream was stopped yet by the stopping future

  being resolved.

#### Trait Implementations

##### `impl<St, Fut> Debug for TakeUntil<St, Fut>`

- <span id="takeuntil-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut> FusedStream for TakeUntil<St, Fut>`

- <span id="takeuntil-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, Item> Sink for TakeUntil<S, Fut>`

- <span id="takeuntil-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="takeuntil-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="takeuntil-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="takeuntil-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="takeuntil-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TakeUntil<St, Fut>`

##### `impl<St, Fut> Stream for TakeUntil<St, Fut>`

- <span id="takeuntil-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="takeuntil-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="takeuntil-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TakeUntil<St, Fut>`

##### `impl TryStream for TakeUntil<St, Fut>`

- <span id="takeuntil-trystream-type-ok"></span>`type Ok = T`

- <span id="takeuntil-trystream-type-error"></span>`type Error = E`

- <span id="takeuntil-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TakeUntil<St, Fut>`

##### `impl<St: Stream, Fut: Future> Unpin for TakeUntil<St, Fut>`

### `TakeWhile<St: Stream, Fut, F>`

```rust
struct TakeWhile<St: Stream, Fut, F> {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Item>,
    done_taking: bool,
}
```

Stream for the [`take_while`](super::StreamExt::take_while) method.

#### Implementations

- <span id="takewhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="takewhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="takewhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takewhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takewhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TakeWhile<St, Fut, F>`

- <span id="takewhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TakeWhile<St, Fut, F>`

- <span id="takewhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TakeWhile<S, Fut, F>`

- <span id="takewhile-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="takewhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="takewhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="takewhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="takewhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TakeWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TakeWhile<St, Fut, F>`

- <span id="takewhile-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="takewhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="takewhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TakeWhile<St, Fut, F>`

##### `impl TryStream for TakeWhile<St, Fut, F>`

- <span id="takewhile-trystream-type-ok"></span>`type Ok = T`

- <span id="takewhile-trystream-type-error"></span>`type Error = E`

- <span id="takewhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TakeWhile<St, Fut, F>`

##### `impl<St: Stream, Fut, F> Unpin for TakeWhile<St, Fut, F>`

### `Then<St, Fut, F>`

```rust
struct Then<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`then`](super::StreamExt::then) method.

#### Implementations

- <span id="then-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="then-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="then-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="then-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="then-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for Then<St, Fut, F>`

- <span id="then-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for Then<St, Fut, F>`

- <span id="then-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for Then<S, Fut, F>`

- <span id="then-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="then-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="then-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="then-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="then-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Then<St, Fut, F>`

##### `impl<St, Fut, F> Stream for Then<St, Fut, F>`

- <span id="then-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="then-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="then-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Then<St, Fut, F>`

##### `impl TryStream for Then<St, Fut, F>`

- <span id="then-trystream-type-ok"></span>`type Ok = T`

- <span id="then-trystream-type-error"></span>`type Error = E`

- <span id="then-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Then<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for Then<St, Fut, F>`

### `Unzip<St, FromA, FromB>`

```rust
struct Unzip<St, FromA, FromB> {
    stream: St,
    left: FromA,
    right: FromB,
}
```

Future for the [`unzip`](super::StreamExt::unzip) method.

#### Implementations

- <span id="unzip-finish"></span>`fn finish(self: Pin<&mut Self>) -> (FromA, FromB)`

- <span id="unzip-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, FromA: fmt::Debug, FromB: fmt::Debug> Debug for Unzip<St, FromA, FromB>`

- <span id="unzip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, FromA, FromB> FusedFuture for Unzip<St, FromA, FromB>`

- <span id="unzip-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, FromA, FromB> Future for Unzip<St, FromA, FromB>`

- <span id="unzip-future-type-output"></span>`type Output = (FromA, FromB)`

- <span id="unzip-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<(FromA, FromB)>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for Unzip<St, FromA, FromB>`

##### `impl IntoFuture for Unzip<St, FromA, FromB>`

- <span id="unzip-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="unzip-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="unzip-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, FromA, FromB> Unpin for Unzip<St, FromA, FromB>`

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

- <span id="zip-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="zip-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Zip<St1, St2>`

##### `impl<St1: Stream, St2: Stream> Unpin for Zip<St1, St2>`

### `Chunks<St: Stream>`

```rust
struct Chunks<St: Stream> {
    stream: crate::stream::Fuse<St>,
    items: alloc::vec::Vec<<St as >::Item>,
    cap: usize,
}
```

Stream for the [`chunks`](super::StreamExt::chunks) method.

#### Implementations

- <span id="chunks-new"></span>`fn new(stream: St, capacity: usize) -> Self`

- <span id="chunks-take"></span>`fn take(self: Pin<&mut Self>) -> Vec<<St as >::Item>` — [`Stream`](#stream)

- <span id="chunks-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="chunks-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="chunks-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="chunks-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug + Stream> Debug for Chunks<St>`

- <span id="chunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: FusedStream> FusedStream for Chunks<St>`

- <span id="chunks-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Chunks<S>`

- <span id="chunks-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="chunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="chunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="chunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="chunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Chunks<St>`

##### `impl<St: Stream> Stream for Chunks<St>`

- <span id="chunks-stream-type-item"></span>`type Item = Vec<<St as Stream>::Item>`

- <span id="chunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="chunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Chunks<St>`

##### `impl<St: Stream> Unpin for Chunks<St>`

### `ReadyChunks<St: Stream>`

```rust
struct ReadyChunks<St: Stream> {
    stream: crate::stream::Fuse<St>,
    cap: usize,
}
```

Stream for the [`ready_chunks`](super::StreamExt::ready_chunks) method.

#### Implementations

- <span id="readychunks-new"></span>`fn new(stream: St, capacity: usize) -> Self`

- <span id="readychunks-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="readychunks-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="readychunks-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="readychunks-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug + Stream> Debug for ReadyChunks<St>`

- <span id="readychunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream> FusedStream for ReadyChunks<St>`

- <span id="readychunks-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for ReadyChunks<S>`

- <span id="readychunks-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="readychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="readychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="readychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="readychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for ReadyChunks<St>`

##### `impl<St: Stream> Stream for ReadyChunks<St>`

- <span id="readychunks-stream-type-item"></span>`type Item = Vec<<St as Stream>::Item>`

- <span id="readychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="readychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for ReadyChunks<St>`

##### `impl<St: Stream> Unpin for ReadyChunks<St>`

### `Forward<St, Si>`

```rust
struct Forward<St, Si>
where
    St: TryStream {
    inner: forward::Forward<St, Si, <St as >::Ok>,
}
```

Future for the [`forward`](super::StreamExt::forward) method.

#### Implementations

- <span id="forward-new"></span>`fn new(x: St, y: Si) -> Self`

#### Trait Implementations

##### `impl<St, Si> Debug for Forward<St, Si>`

- <span id="forward-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, Si> FusedFuture for Forward<St, Si>`

- <span id="forward-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Si> Future for Forward<St, Si>`

- <span id="forward-future-type-output"></span>`type Output = <Forward<St, Si, <St as TryStream>::Ok> as Future>::Output`

- <span id="forward-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Forward<St, Si>`

##### `impl IntoFuture for Forward<St, Si>`

- <span id="forward-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="forward-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="forward-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Forward<St, Si>`

- <span id="forward-tryfuture-type-ok"></span>`type Ok = T`

- <span id="forward-tryfuture-type-error"></span>`type Error = E`

- <span id="forward-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Forward<St, Si>`

##### `impl<St, Si> Unpin for Forward<St, Si>`

### `BufferUnordered<St>`

```rust
struct BufferUnordered<St>
where
    St: Stream {
    stream: crate::stream::Fuse<St>,
    in_progress_queue: crate::stream::FuturesUnordered<<St as >::Item>,
    max: usize,
}
```

Stream for the [`buffer_unordered`](super::StreamExt::buffer_unordered)
method.

#### Implementations

- <span id="bufferunordered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="bufferunordered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="bufferunordered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="bufferunordered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="bufferunordered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for BufferUnordered<St>`

- <span id="bufferunordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for BufferUnordered<St>`

- <span id="bufferunordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for BufferUnordered<S>`

- <span id="bufferunordered-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="bufferunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="bufferunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="bufferunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="bufferunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for BufferUnordered<St>`

##### `impl<St> Stream for BufferUnordered<St>`

- <span id="bufferunordered-stream-type-item"></span>`type Item = <<St as Stream>::Item as Future>::Output`

- <span id="bufferunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="bufferunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for BufferUnordered<St>`

##### `impl TryStream for BufferUnordered<St>`

- <span id="bufferunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="bufferunordered-trystream-type-error"></span>`type Error = E`

- <span id="bufferunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for BufferUnordered<St>`

##### `impl<St> Unpin for BufferUnordered<St>`

### `Buffered<St>`

```rust
struct Buffered<St>
where
    St: Stream,
    <St as >::Item: Future {
    stream: crate::stream::Fuse<St>,
    in_progress_queue: crate::stream::FuturesOrdered<<St as >::Item>,
    max: usize,
}
```

Stream for the [`buffered`](super::StreamExt::buffered) method.

#### Implementations

- <span id="buffered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="buffered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="buffered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for Buffered<St>`

- <span id="buffered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for Buffered<St>`

- <span id="buffered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Buffered<S>`

- <span id="buffered-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="buffered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="buffered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="buffered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="buffered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Buffered<St>`

##### `impl<St> Stream for Buffered<St>`

- <span id="buffered-stream-type-item"></span>`type Item = <<St as Stream>::Item as Future>::Output`

- <span id="buffered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="buffered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Buffered<St>`

##### `impl TryStream for Buffered<St>`

- <span id="buffered-trystream-type-ok"></span>`type Ok = T`

- <span id="buffered-trystream-type-error"></span>`type Error = E`

- <span id="buffered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Buffered<St>`

##### `impl<St> Unpin for Buffered<St>`

### `FlatMapUnordered<St, U, F>`

```rust
struct FlatMapUnordered<St, U, F>
where
    St: Stream,
    U: Stream + Unpin,
    F: FnMut(<St as >::Item) -> U {
    inner: FlattenUnordered<Map<St, F>>,
}
```

Stream for the [`flat_map_unordered`](StreamExt::flat_map_unordered) method.

#### Implementations

- <span id="flatmapunordered-new"></span>`fn new(x: St, limit: Option<usize>, f: F) -> Self`

#### Trait Implementations

##### `impl<St, U, F> Debug for FlatMapUnordered<St, U, F>`

- <span id="flatmapunordered-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, U, F> FusedStream for FlatMapUnordered<St, U, F>`

- <span id="flatmapunordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, U, F> Sink for FlatMapUnordered<St, U, F>`

- <span id="flatmapunordered-sink-type-error"></span>`type Error = <FlattenUnorderedWithFlowController<Map<St, F>, ()> as Sink>::Error`

- <span id="flatmapunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flatmapunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="flatmapunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flatmapunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for FlatMapUnordered<St, U, F>`

##### `impl<St, U, F> Stream for FlatMapUnordered<St, U, F>`

- <span id="flatmapunordered-stream-type-item"></span>`type Item = <FlattenUnorderedWithFlowController<Map<St, F>, ()> as Stream>::Item`

- <span id="flatmapunordered-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="flatmapunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlatMapUnordered<St, U, F>`

##### `impl TryStream for FlatMapUnordered<St, U, F>`

- <span id="flatmapunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="flatmapunordered-trystream-type-error"></span>`type Error = E`

- <span id="flatmapunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for FlatMapUnordered<St, U, F>`

##### `impl<St, U, F> Unpin for FlatMapUnordered<St, U, F>`

### `ForEachConcurrent<St, Fut, F>`

```rust
struct ForEachConcurrent<St, Fut, F> {
    stream: Option<St>,
    f: F,
    futures: crate::stream::FuturesUnordered<Fut>,
    limit: Option<core::num::NonZeroUsize>,
}
```

Future for the [`for_each_concurrent`](super::StreamExt::for_each_concurrent)
method.

#### Implementations

- <span id="foreachconcurrent-new"></span>`fn new(stream: St, limit: Option<usize>, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-future-type-output"></span>`type Output = ()`

- <span id="foreachconcurrent-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for ForEachConcurrent<St, Fut, F>`

##### `impl<F> IntoFuture for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="foreachconcurrent-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="foreachconcurrent-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for ForEachConcurrent<St, Fut, F>`

### `ReuniteError<T, Item>`

```rust
struct ReuniteError<T, Item>(SplitSink<T, Item>, SplitStream<T>);
```

Error indicating a `SplitSink<S>` and `SplitStream<S>` were not two halves
of a `Stream + Split`, and thus could not be `reunite`d.

#### Trait Implementations

##### `impl<T, Item> Debug for ReuniteError<T, Item>`

- <span id="reuniteerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, Item> Display for ReuniteError<T, Item>`

- <span id="reuniteerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for ReuniteError<T, Item>`

- <span id="reuniteerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `SplitSink<S, Item>`

```rust
struct SplitSink<S, Item> {
    lock: self::bilock::BiLock<S>,
    slot: Option<Item>,
}
```

A `Sink` part of the split pair

#### Implementations

- <span id="splitsink-reunite"></span>`fn reunite(self, other: SplitStream<S>) -> Result<S, ReuniteError<S, Item>>` — [`SplitStream`](stream/split/index.md#splitstream), [`ReuniteError`](stream/split/index.md#reuniteerror)

  Attempts to put the two "halves" of a split `Stream + Sink` back

  together. Succeeds only if the `SplitStream<S>` and `SplitSink<S>` are

  a matching pair originating from the same call to `StreamExt::split`.

#### Trait Implementations

##### `impl<S: fmt::Debug, Item: fmt::Debug> Debug for SplitSink<S, Item>`

- <span id="splitsink-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Sink<Item>, Item> Sink for SplitSink<S, Item>`

- <span id="splitsink-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="splitsink-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="splitsink-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <S as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="splitsink-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="splitsink-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for SplitSink<S, Item>`

##### `impl<S, Item> Unpin for SplitSink<S, Item>`

### `SplitStream<S>`

```rust
struct SplitStream<S>(self::bilock::BiLock<S>);
```

A `Stream` part of the split pair

#### Implementations

- <span id="splitstream-is-pair-of"></span>`fn is_pair_of<Item>(&self, other: &SplitSink<S, Item>) -> bool` — [`SplitSink`](stream/split/index.md#splitsink)

  Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`.

#### Trait Implementations

##### `impl<S: fmt::Debug> Debug for SplitStream<S>`

- <span id="splitstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Stream> Stream for SplitStream<S>`

- <span id="splitstream-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="splitstream-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl StreamExt for SplitStream<S>`

##### `impl<S> TryStream for SplitStream<S>`

- <span id="splitstream-trystream-type-ok"></span>`type Ok = T`

- <span id="splitstream-trystream-type-error"></span>`type Error = E`

- <span id="splitstream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl<S> TryStreamExt for SplitStream<S>`

##### `impl<S> Unpin for SplitStream<S>`

### `AndThen<St, Fut, F>`

```rust
struct AndThen<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`and_then`](super::TryStreamExt::and_then) method.

#### Implementations

- <span id="andthen-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="andthen-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="andthen-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="andthen-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="andthen-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for AndThen<St, Fut, F>`

- <span id="andthen-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for AndThen<St, Fut, F>`

- <span id="andthen-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for AndThen<S, Fut, F>`

- <span id="andthen-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="andthen-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="andthen-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="andthen-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="andthen-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for AndThen<St, Fut, F>`

##### `impl<St, Fut, F> Stream for AndThen<St, Fut, F>`

- <span id="andthen-stream-type-item"></span>`type Item = Result<<Fut as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="andthen-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="andthen-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for AndThen<St, Fut, F>`

##### `impl TryStream for AndThen<St, Fut, F>`

- <span id="andthen-trystream-type-ok"></span>`type Ok = T`

- <span id="andthen-trystream-type-error"></span>`type Error = E`

- <span id="andthen-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for AndThen<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for AndThen<St, Fut, F>`

### `ErrInto<St, E>`

```rust
struct ErrInto<St, E> {
    inner: MapErr<St, crate::fns::IntoFn<E>>,
}
```

Stream for the [`err_into`](super::TryStreamExt::err_into) method.

#### Implementations

- <span id="errinto-new"></span>`fn new(x: St) -> Self`

#### Trait Implementations

##### `impl<St, E> Debug for ErrInto<St, E>`

- <span id="errinto-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, E> FusedStream for ErrInto<St, E>`

- <span id="errinto-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, E> Sink for ErrInto<St, E>`

- <span id="errinto-sink-type-error"></span>`type Error = <MapErr<St, IntoFn<E>> as Sink>::Error`

- <span id="errinto-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="errinto-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="errinto-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="errinto-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for ErrInto<St, E>`

##### `impl<St, E> Stream for ErrInto<St, E>`

- <span id="errinto-stream-type-item"></span>`type Item = <MapErr<St, IntoFn<E>> as Stream>::Item`

- <span id="errinto-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="errinto-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for ErrInto<St, E>`

##### `impl<E> TryStream for ErrInto<St, E>`

- <span id="errinto-trystream-type-ok"></span>`type Ok = T`

- <span id="errinto-trystream-type-error"></span>`type Error = E`

- <span id="errinto-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for ErrInto<St, E>`

##### `impl<St, E> Unpin for ErrInto<St, E>`

### `InspectErr<St, F>`

```rust
struct InspectErr<St, F> {
    inner: crate::stream::Inspect<IntoStream<St>, crate::fns::InspectErrFn<F>>,
}
```

Stream for the [`inspect_err`](super::TryStreamExt::inspect_err) method.

#### Implementations

- <span id="inspecterr-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for InspectErr<St, F>`

- <span id="inspecterr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for InspectErr<St, F>`

- <span id="inspecterr-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for InspectErr<St, F>`

- <span id="inspecterr-sink-type-error"></span>`type Error = <Inspect<IntoStream<St>, InspectErrFn<F>> as Sink>::Error`

- <span id="inspecterr-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="inspecterr-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="inspecterr-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="inspecterr-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for InspectErr<St, F>`

##### `impl<St, F> Stream for InspectErr<St, F>`

- <span id="inspecterr-stream-type-item"></span>`type Item = <Inspect<IntoStream<St>, InspectErrFn<F>> as Stream>::Item`

- <span id="inspecterr-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="inspecterr-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for InspectErr<St, F>`

##### `impl TryStream for InspectErr<St, F>`

- <span id="inspecterr-trystream-type-ok"></span>`type Ok = T`

- <span id="inspecterr-trystream-type-error"></span>`type Error = E`

- <span id="inspecterr-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for InspectErr<St, F>`

##### `impl<St, F> Unpin for InspectErr<St, F>`

### `InspectOk<St, F>`

```rust
struct InspectOk<St, F> {
    inner: crate::stream::Inspect<IntoStream<St>, crate::fns::InspectOkFn<F>>,
}
```

Stream for the [`inspect_ok`](super::TryStreamExt::inspect_ok) method.

#### Implementations

- <span id="inspectok-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for InspectOk<St, F>`

- <span id="inspectok-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for InspectOk<St, F>`

- <span id="inspectok-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for InspectOk<St, F>`

- <span id="inspectok-sink-type-error"></span>`type Error = <Inspect<IntoStream<St>, InspectOkFn<F>> as Sink>::Error`

- <span id="inspectok-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="inspectok-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="inspectok-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="inspectok-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for InspectOk<St, F>`

##### `impl<St, F> Stream for InspectOk<St, F>`

- <span id="inspectok-stream-type-item"></span>`type Item = <Inspect<IntoStream<St>, InspectOkFn<F>> as Stream>::Item`

- <span id="inspectok-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="inspectok-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for InspectOk<St, F>`

##### `impl TryStream for InspectOk<St, F>`

- <span id="inspectok-trystream-type-ok"></span>`type Ok = T`

- <span id="inspectok-trystream-type-error"></span>`type Error = E`

- <span id="inspectok-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for InspectOk<St, F>`

##### `impl<St, F> Unpin for InspectOk<St, F>`

### `IntoStream<St>`

```rust
struct IntoStream<St> {
    stream: St,
}
```

Stream for the [`into_stream`](super::TryStreamExt::into_stream) method.

#### Implementations

- <span id="intostream-new"></span>`fn new(stream: St) -> Self`

- <span id="intostream-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="intostream-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="intostream-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="intostream-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for IntoStream<St>`

- <span id="intostream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: TryStream + FusedStream> FusedStream for IntoStream<St>`

- <span id="intostream-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S: Sink<Item>, Item> Sink for IntoStream<S>`

- <span id="intostream-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="intostream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="intostream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="intostream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="intostream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for IntoStream<St>`

##### `impl<St: TryStream> Stream for IntoStream<St>`

- <span id="intostream-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="intostream-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="intostream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for IntoStream<St>`

##### `impl TryStream for IntoStream<St>`

- <span id="intostream-trystream-type-ok"></span>`type Ok = T`

- <span id="intostream-trystream-type-error"></span>`type Error = E`

- <span id="intostream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for IntoStream<St>`

##### `impl<St> Unpin for IntoStream<St>`

### `MapErr<St, F>`

```rust
struct MapErr<St, F> {
    inner: crate::stream::Map<IntoStream<St>, crate::fns::MapErrFn<F>>,
}
```

Stream for the [`map_err`](super::TryStreamExt::map_err) method.

#### Implementations

- <span id="maperr-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for MapErr<St, F>`

- <span id="maperr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for MapErr<St, F>`

- <span id="maperr-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for MapErr<St, F>`

- <span id="maperr-sink-type-error"></span>`type Error = <Map<IntoStream<St>, MapErrFn<F>> as Sink>::Error`

- <span id="maperr-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="maperr-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="maperr-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="maperr-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for MapErr<St, F>`

##### `impl<St, F> Stream for MapErr<St, F>`

- <span id="maperr-stream-type-item"></span>`type Item = <Map<IntoStream<St>, MapErrFn<F>> as Stream>::Item`

- <span id="maperr-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="maperr-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for MapErr<St, F>`

##### `impl TryStream for MapErr<St, F>`

- <span id="maperr-trystream-type-ok"></span>`type Ok = T`

- <span id="maperr-trystream-type-error"></span>`type Error = E`

- <span id="maperr-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for MapErr<St, F>`

##### `impl<St, F> Unpin for MapErr<St, F>`

### `MapOk<St, F>`

```rust
struct MapOk<St, F> {
    inner: crate::stream::Map<IntoStream<St>, crate::fns::MapOkFn<F>>,
}
```

Stream for the [`map_ok`](super::TryStreamExt::map_ok) method.

#### Implementations

- <span id="mapok-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for MapOk<St, F>`

- <span id="mapok-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for MapOk<St, F>`

- <span id="mapok-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for MapOk<St, F>`

- <span id="mapok-sink-type-error"></span>`type Error = <Map<IntoStream<St>, MapOkFn<F>> as Sink>::Error`

- <span id="mapok-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="mapok-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="mapok-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="mapok-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for MapOk<St, F>`

##### `impl<St, F> Stream for MapOk<St, F>`

- <span id="mapok-stream-type-item"></span>`type Item = <Map<IntoStream<St>, MapOkFn<F>> as Stream>::Item`

- <span id="mapok-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="mapok-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for MapOk<St, F>`

##### `impl TryStream for MapOk<St, F>`

- <span id="mapok-trystream-type-ok"></span>`type Ok = T`

- <span id="mapok-trystream-type-error"></span>`type Error = E`

- <span id="mapok-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for MapOk<St, F>`

##### `impl<St, F> Unpin for MapOk<St, F>`

### `OrElse<St, Fut, F>`

```rust
struct OrElse<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`or_else`](super::TryStreamExt::or_else) method.

#### Implementations

- <span id="orelse-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="orelse-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="orelse-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="orelse-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="orelse-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for OrElse<St, Fut, F>`

- <span id="orelse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for OrElse<St, Fut, F>`

- <span id="orelse-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for OrElse<S, Fut, F>`

- <span id="orelse-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="orelse-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="orelse-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="orelse-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="orelse-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for OrElse<St, Fut, F>`

##### `impl<St, Fut, F> Stream for OrElse<St, Fut, F>`

- <span id="orelse-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <Fut as TryFuture>::Error>`

- <span id="orelse-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="orelse-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for OrElse<St, Fut, F>`

##### `impl TryStream for OrElse<St, Fut, F>`

- <span id="orelse-trystream-type-ok"></span>`type Ok = T`

- <span id="orelse-trystream-type-error"></span>`type Error = E`

- <span id="orelse-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for OrElse<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for OrElse<St, Fut, F>`

### `TryAll<St, Fut, F>`

```rust
struct TryAll<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`try_all`](super::TryStreamExt::try_all) method.

#### Implementations

- <span id="tryall-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryAll<St, Fut, F>`

- <span id="tryall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryAll<St, Fut, F>`

- <span id="tryall-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryAll<St, Fut, F>`

- <span id="tryall-future-type-output"></span>`type Output = Result<bool, <St as TryStream>::Error>`

- <span id="tryall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<bool, <St as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl FutureExt for TryAll<St, Fut, F>`

##### `impl<F> IntoFuture for TryAll<St, Fut, F>`

- <span id="tryall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryAll<St, Fut, F>`

- <span id="tryall-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryall-tryfuture-type-error"></span>`type Error = E`

- <span id="tryall-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryAll<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryAll<St, Fut, F>`

### `TryAny<St, Fut, F>`

```rust
struct TryAny<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`try_any`](super::TryStreamExt::try_any) method.

#### Implementations

- <span id="tryany-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryAny<St, Fut, F>`

- <span id="tryany-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryAny<St, Fut, F>`

- <span id="tryany-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryAny<St, Fut, F>`

- <span id="tryany-future-type-output"></span>`type Output = Result<bool, <St as TryStream>::Error>`

- <span id="tryany-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<bool, <St as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl FutureExt for TryAny<St, Fut, F>`

##### `impl<F> IntoFuture for TryAny<St, Fut, F>`

- <span id="tryany-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryany-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryany-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryAny<St, Fut, F>`

- <span id="tryany-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryany-tryfuture-type-error"></span>`type Error = E`

- <span id="tryany-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryAny<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryAny<St, Fut, F>`

### `TryCollect<St, C>`

```rust
struct TryCollect<St, C> {
    stream: St,
    items: C,
}
```

Future for the [`try_collect`](super::TryStreamExt::try_collect) method.

#### Implementations

- <span id="trycollect-new"></span>`fn new(s: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, C: fmt::Debug> Debug for TryCollect<St, C>`

- <span id="trycollect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, C> FusedFuture for TryCollect<St, C>`

- <span id="trycollect-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, C> Future for TryCollect<St, C>`

- <span id="trycollect-future-type-output"></span>`type Output = Result<C, <St as TryStream>::Error>`

- <span id="trycollect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for TryCollect<St, C>`

##### `impl IntoFuture for TryCollect<St, C>`

- <span id="trycollect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trycollect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trycollect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryCollect<St, C>`

- <span id="trycollect-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trycollect-tryfuture-type-error"></span>`type Error = E`

- <span id="trycollect-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for TryCollect<St, C>`

##### `impl<St, C> Unpin for TryCollect<St, C>`

### `TryConcat<St: TryStream>`

```rust
struct TryConcat<St: TryStream> {
    stream: St,
    accum: Option<<St as >::Ok>,
}
```

Future for the [`try_concat`](super::TryStreamExt::try_concat) method.

#### Implementations

- <span id="tryconcat-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + TryStream> Debug for TryConcat<St>`

- <span id="tryconcat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> Future for TryConcat<St>`

- <span id="tryconcat-future-type-output"></span>`type Output = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryconcat-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for TryConcat<St>`

##### `impl IntoFuture for TryConcat<St>`

- <span id="tryconcat-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryconcat-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryconcat-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryConcat<St>`

- <span id="tryconcat-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryconcat-tryfuture-type-error"></span>`type Error = E`

- <span id="tryconcat-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for TryConcat<St>`

##### `impl<St: TryStream> Unpin for TryConcat<St>`

### `TryFilter<St, Fut, F>`

```rust
struct TryFilter<St, Fut, F>
where
    St: TryStream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Ok>,
}
```

Stream for the [`try_filter`](super::TryStreamExt::try_filter)
method.

#### Implementations

- <span id="tryfilter-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryfilter-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryfilter-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfilter-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfilter-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryFilter<St, Fut, F>`

- <span id="tryfilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryFilter<St, Fut, F>`

- <span id="tryfilter-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryFilter<S, Fut, F>`

- <span id="tryfilter-sink-type-error"></span>`type Error = E`

- <span id="tryfilter-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryfilter-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="tryfilter-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryfilter-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFilter<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryFilter<St, Fut, F>`

- <span id="tryfilter-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryfilter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="tryfilter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFilter<St, Fut, F>`

##### `impl TryStream for TryFilter<St, Fut, F>`

- <span id="tryfilter-trystream-type-ok"></span>`type Ok = T`

- <span id="tryfilter-trystream-type-error"></span>`type Error = E`

- <span id="tryfilter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryFilter<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryFilter<St, Fut, F>`

### `TryFilterMap<St, Fut, F>`

```rust
struct TryFilterMap<St, Fut, F> {
    stream: St,
    f: F,
    pending: Option<Fut>,
}
```

Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map)
method.

#### Implementations

- <span id="tryfiltermap-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryfiltermap-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryfiltermap-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfiltermap-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfiltermap-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryFilterMap<S, Fut, F>`

- <span id="tryfiltermap-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryfiltermap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryfiltermap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="tryfiltermap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryfiltermap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-stream-type-item"></span>`type Item = Result<T, <St as TryStream>::Error>`

- <span id="tryfiltermap-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="tryfiltermap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFilterMap<St, Fut, F>`

##### `impl TryStream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-trystream-type-ok"></span>`type Ok = T`

- <span id="tryfiltermap-trystream-type-error"></span>`type Error = E`

- <span id="tryfiltermap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryFilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryFilterMap<St, Fut, F>`

### `TryFlatten<St>`

```rust
struct TryFlatten<St>
where
    St: TryStream {
    stream: St,
    next: Option<<St as >::Ok>,
}
```

Stream for the [`try_flatten`](super::TryStreamExt::try_flatten) method.

#### Implementations

- <span id="tryflatten-new"></span>`fn new(stream: St) -> Self`

- <span id="tryflatten-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryflatten-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryflatten-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryflatten-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for TryFlatten<St>`

- <span id="tryflatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for TryFlatten<St>`

- <span id="tryflatten-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for TryFlatten<S>`

- <span id="tryflatten-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryflatten-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryflatten-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="tryflatten-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryflatten-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlatten<St>`

##### `impl<St> Stream for TryFlatten<St>`

- <span id="tryflatten-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryStream>::Ok, <<St as TryStream>::Ok as TryStream>::Error>`

- <span id="tryflatten-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl StreamExt for TryFlatten<St>`

##### `impl TryStream for TryFlatten<St>`

- <span id="tryflatten-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflatten-trystream-type-error"></span>`type Error = E`

- <span id="tryflatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryFlatten<St>`

##### `impl<St> Unpin for TryFlatten<St>`

### `TryFold<St, Fut, T, F>`

```rust
struct TryFold<St, Fut, T, F> {
    stream: St,
    f: F,
    accum: Option<T>,
    future: Option<Fut>,
}
```

Future for the [`try_fold`](super::TryStreamExt::try_fold) method.

#### Implementations

- <span id="tryfold-new"></span>`fn new(stream: St, f: F, t: T) -> Self`

#### Trait Implementations

##### `impl<St, Fut, T, F> Debug for TryFold<St, Fut, T, F>`

- <span id="tryfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, T, F> FusedFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, T, F> Future for TryFold<St, Fut, T, F>`

- <span id="tryfold-future-type-output"></span>`type Output = Result<T, <St as TryStream>::Error>`

- <span id="tryfold-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<T> FutureExt for TryFold<St, Fut, T, F>`

##### `impl<F> IntoFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryfold-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryfold-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryfold-tryfuture-type-error"></span>`type Error = E`

- <span id="tryfold-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryFold<St, Fut, T, F>`

##### `impl<St, Fut, T, F> Unpin for TryFold<St, Fut, T, F>`

### `TryForEach<St, Fut, F>`

```rust
struct TryForEach<St, Fut, F> {
    stream: St,
    f: F,
    future: Option<Fut>,
}
```

Future for the [`try_for_each`](super::TryStreamExt::try_for_each) method.

#### Implementations

- <span id="tryforeach-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryForEach<St, Fut, F>`

- <span id="tryforeach-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> Future for TryForEach<St, Fut, F>`

- <span id="tryforeach-future-type-output"></span>`type Output = Result<(), <St as TryStream>::Error>`

- <span id="tryforeach-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for TryForEach<St, Fut, F>`

##### `impl<F> IntoFuture for TryForEach<St, Fut, F>`

- <span id="tryforeach-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryforeach-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryforeach-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryForEach<St, Fut, F>`

- <span id="tryforeach-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryforeach-tryfuture-type-error"></span>`type Error = E`

- <span id="tryforeach-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryForEach<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryForEach<St, Fut, F>`

### `TryNext<'a, St: ?Sized>`

```rust
struct TryNext<'a, St: ?Sized> {
    stream: &'a mut St,
}
```

Future for the [`try_next`](super::TryStreamExt::try_next) method.

#### Implementations

- <span id="trynext-new"></span>`fn new(stream: &'a mut St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + ?Sized> Debug for TryNext<'a, St>`

- <span id="trynext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: ?Sized + TryStream + Unpin + FusedStream> FusedFuture for TryNext<'_, St>`

- <span id="trynext-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: ?Sized + TryStream + Unpin> Future for TryNext<'_, St>`

- <span id="trynext-future-type-output"></span>`type Output = Result<Option<<St as TryStream>::Ok>, <St as TryStream>::Error>`

- <span id="trynext-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for TryNext<'a, St>`

##### `impl IntoFuture for TryNext<'a, St>`

- <span id="trynext-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trynext-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trynext-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryNext<'a, St>`

- <span id="trynext-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trynext-tryfuture-type-error"></span>`type Error = E`

- <span id="trynext-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for TryNext<'a, St>`

##### `impl<St: ?Sized + Unpin> Unpin for TryNext<'_, St>`

### `TrySkipWhile<St, Fut, F>`

```rust
struct TrySkipWhile<St, Fut, F>
where
    St: TryStream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Ok>,
    done_skipping: bool,
}
```

Stream for the [`try_skip_while`](super::TryStreamExt::try_skip_while)
method.

#### Implementations

- <span id="tryskipwhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryskipwhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryskipwhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryskipwhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryskipwhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TrySkipWhile<S, Fut, F>`

- <span id="tryskipwhile-sink-type-error"></span>`type Error = E`

- <span id="tryskipwhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryskipwhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="tryskipwhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryskipwhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TrySkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryskipwhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="tryskipwhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TrySkipWhile<St, Fut, F>`

##### `impl TryStream for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-trystream-type-ok"></span>`type Ok = T`

- <span id="tryskipwhile-trystream-type-error"></span>`type Error = E`

- <span id="tryskipwhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TrySkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TrySkipWhile<St, Fut, F>`

### `TryTakeWhile<St, Fut, F>`

```rust
struct TryTakeWhile<St, Fut, F>
where
    St: TryStream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Ok>,
    done_taking: bool,
}
```

Stream for the [`try_take_while`](super::TryStreamExt::try_take_while)
method.

#### Implementations

- <span id="trytakewhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="trytakewhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trytakewhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trytakewhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trytakewhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryTakeWhile<S, Fut, F>`

- <span id="trytakewhile-sink-type-error"></span>`type Error = E`

- <span id="trytakewhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trytakewhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="trytakewhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trytakewhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryTakeWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="trytakewhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="trytakewhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryTakeWhile<St, Fut, F>`

##### `impl TryStream for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-trystream-type-ok"></span>`type Ok = T`

- <span id="trytakewhile-trystream-type-error"></span>`type Error = E`

- <span id="trytakewhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryTakeWhile<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryTakeWhile<St, Fut, F>`

### `TryUnfold<T, F, Fut>`

```rust
struct TryUnfold<T, F, Fut> {
    f: F,
    state: Option<T>,
    fut: Option<Fut>,
}
```

Stream for the [`try_unfold`](try_stream/try_unfold/index.md) function.

#### Trait Implementations

##### `impl<T, F, Fut> Debug for TryUnfold<T, F, Fut>`

- <span id="tryunfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F, Fut> Stream for TryUnfold<T, F, Fut>`

- <span id="tryunfold-stream-type-item"></span>`type Item = Result<Item, <Fut as TryFuture>::Error>`

- <span id="tryunfold-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl<T> StreamExt for TryUnfold<T, F, Fut>`

##### `impl<T> TryStream for TryUnfold<T, F, Fut>`

- <span id="tryunfold-trystream-type-ok"></span>`type Ok = T`

- <span id="tryunfold-trystream-type-error"></span>`type Error = E`

- <span id="tryunfold-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryUnfold<T, F, Fut>`

##### `impl<T, F, Fut> Unpin for TryUnfold<T, F, Fut>`

### `TryBufferUnordered<St>`

```rust
struct TryBufferUnordered<St>
where
    St: TryStream {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    in_progress_queue: crate::stream::FuturesUnordered<crate::future::IntoFuture<<St as >::Ok>>,
    max: usize,
}
```

Stream for the
[`try_buffer_unordered`](super::TryStreamExt::try_buffer_unordered) method.

#### Implementations

- <span id="trybufferunordered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="trybufferunordered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trybufferunordered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybufferunordered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybufferunordered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for TryBufferUnordered<St>`

- <span id="trybufferunordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item> Sink for TryBufferUnordered<S>`

- <span id="trybufferunordered-sink-type-error"></span>`type Error = E`

- <span id="trybufferunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trybufferunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="trybufferunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trybufferunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryBufferUnordered<St>`

##### `impl<St> Stream for TryBufferUnordered<St>`

- <span id="trybufferunordered-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="trybufferunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl StreamExt for TryBufferUnordered<St>`

##### `impl TryStream for TryBufferUnordered<St>`

- <span id="trybufferunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="trybufferunordered-trystream-type-error"></span>`type Error = E`

- <span id="trybufferunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryBufferUnordered<St>`

##### `impl<St> Unpin for TryBufferUnordered<St>`

### `TryBuffered<St>`

```rust
struct TryBuffered<St>
where
    St: TryStream,
    <St as >::Ok: TryFuture {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    in_progress_queue: crate::stream::FuturesOrdered<crate::future::IntoFuture<<St as >::Ok>>,
    max: usize,
}
```

Stream for the [`try_buffered`](super::TryStreamExt::try_buffered) method.

#### Implementations

- <span id="trybuffered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="trybuffered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trybuffered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybuffered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybuffered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for TryBuffered<St>`

- <span id="trybuffered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item> Sink for TryBuffered<S>`

- <span id="trybuffered-sink-type-error"></span>`type Error = E`

- <span id="trybuffered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trybuffered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="trybuffered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trybuffered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryBuffered<St>`

##### `impl<St> Stream for TryBuffered<St>`

- <span id="trybuffered-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="trybuffered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl StreamExt for TryBuffered<St>`

##### `impl TryStream for TryBuffered<St>`

- <span id="trybuffered-trystream-type-ok"></span>`type Ok = T`

- <span id="trybuffered-trystream-type-error"></span>`type Error = E`

- <span id="trybuffered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryBuffered<St>`

##### `impl<St> Unpin for TryBuffered<St>`

### `TryFlattenUnordered<St>`

```rust
struct TryFlattenUnordered<St>
where
    St: TryStream,
    <St as >::Ok: TryStream + Unpin,
    <<St as >::Ok as TryStream>::Error: From<<St as >::Error> {
    inner: crate::stream::stream::flatten_unordered::FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>>,
}
```

Stream for the [`try_flatten_unordered`](super::TryStreamExt::try_flatten_unordered) method.

#### Implementations

- <span id="tryflattenunordered-new"></span>`fn new(stream: St, limit: impl Into<Option<usize>>) -> Self`

#### Trait Implementations

##### `impl<St> Debug for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St> FusedStream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St> Sink for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-sink-type-error"></span>`type Error = <FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>> as Sink>::Error`

- <span id="tryflattenunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryflattenunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="tryflattenunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryflattenunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlattenUnordered<St>`

##### `impl<St> Stream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-stream-type-item"></span>`type Item = <FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>> as Stream>::Item`

- <span id="tryflattenunordered-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="tryflattenunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFlattenUnordered<St>`

##### `impl TryStream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflattenunordered-trystream-type-error"></span>`type Error = E`

- <span id="tryflattenunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryFlattenUnordered<St>`

##### `impl<St> Unpin for TryFlattenUnordered<St>`

### `TryForEachConcurrent<St, Fut, F>`

```rust
struct TryForEachConcurrent<St, Fut, F> {
    stream: Option<St>,
    f: F,
    futures: crate::stream::FuturesUnordered<Fut>,
    limit: Option<core::num::NonZeroUsize>,
}
```

Future for the
[`try_for_each_concurrent`](super::TryStreamExt::try_for_each_concurrent)
method.

#### Implementations

- <span id="tryforeachconcurrent-new"></span>`fn new(stream: St, limit: Option<usize>, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-future-type-output"></span>`type Output = Result<(), <St as TryStream>::Error>`

- <span id="tryforeachconcurrent-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for TryForEachConcurrent<St, Fut, F>`

##### `impl<F> IntoFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryforeachconcurrent-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryforeachconcurrent-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryforeachconcurrent-tryfuture-type-error"></span>`type Error = E`

- <span id="tryforeachconcurrent-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryForEachConcurrent<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryForEachConcurrent<St, Fut, F>`

### `TryChunks<St: TryStream>`

```rust
struct TryChunks<St: TryStream> {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    items: alloc::vec::Vec<<St as >::Ok>,
    cap: usize,
}
```

Stream for the [`try_chunks`](super::TryStreamExt::try_chunks) method.

#### Implementations

- <span id="trychunks-new"></span>`fn new(stream: St, capacity: usize) -> Self`

- <span id="trychunks-take"></span>`fn take(self: Pin<&mut Self>) -> Vec<<St as >::Ok>` — [`TryStream`](#trystream)

- <span id="trychunks-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trychunks-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trychunks-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trychunks-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug + TryStream> Debug for TryChunks<St>`

- <span id="trychunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: TryStream + FusedStream> FusedStream for TryChunks<St>`

- <span id="trychunks-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for TryChunks<S>`

- <span id="trychunks-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="trychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="trychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="trychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryChunks<St>`

##### `impl<St: TryStream> Stream for TryChunks<St>`

- <span id="trychunks-stream-type-item"></span>`type Item = Result<Vec<<St as TryStream>::Ok>, TryChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>>`

- <span id="trychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="trychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryChunks<St>`

##### `impl TryStream for TryChunks<St>`

- <span id="trychunks-trystream-type-ok"></span>`type Ok = T`

- <span id="trychunks-trystream-type-error"></span>`type Error = E`

- <span id="trychunks-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryChunks<St>`

##### `impl<St: TryStream> Unpin for TryChunks<St>`

### `TryChunksError<T, E>`

```rust
struct TryChunksError<T, E>(alloc::vec::Vec<T>, E);
```

Error indicating, that while chunk was collected inner stream produced an error.

Contains all items that were collected before an error occurred, and the stream error itself.

#### Trait Implementations

##### `impl<T, E: fmt::Debug> Debug for TryChunksError<T, E>`

- <span id="trychunkserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E: fmt::Display> Display for TryChunksError<T, E>`

- <span id="trychunkserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq, E: cmp::Eq> Eq for TryChunksError<T, E>`

##### `impl<T: cmp::PartialEq, E: cmp::PartialEq> PartialEq for TryChunksError<T, E>`

- <span id="trychunkserror-partialeq-eq"></span>`fn eq(&self, other: &TryChunksError<T, E>) -> bool` — [`TryChunksError`](try_stream/try_chunks/index.md#trychunkserror)

##### `impl<T, E> StructuralPartialEq for TryChunksError<T, E>`

##### `impl<T> ToString for TryChunksError<T, E>`

- <span id="trychunkserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `TryReadyChunks<St: TryStream>`

```rust
struct TryReadyChunks<St: TryStream> {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    cap: usize,
}
```

Stream for the [`try_ready_chunks`](super::TryStreamExt::try_ready_chunks) method.

#### Implementations

- <span id="tryreadychunks-new"></span>`fn new(stream: St, capacity: usize) -> Self`

- <span id="tryreadychunks-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryreadychunks-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryreadychunks-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryreadychunks-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug + TryStream> Debug for TryReadyChunks<St>`

- <span id="tryreadychunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: TryStream + FusedStream> FusedStream for TryReadyChunks<St>`

- <span id="tryreadychunks-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for TryReadyChunks<S>`

- <span id="tryreadychunks-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryreadychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryreadychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="tryreadychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryreadychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryReadyChunks<St>`

##### `impl<St: TryStream> Stream for TryReadyChunks<St>`

- <span id="tryreadychunks-stream-type-item"></span>`type Item = Result<Vec<<St as TryStream>::Ok>, TryReadyChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>>`

- <span id="tryreadychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="tryreadychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryReadyChunks<St>`

##### `impl TryStream for TryReadyChunks<St>`

- <span id="tryreadychunks-trystream-type-ok"></span>`type Ok = T`

- <span id="tryreadychunks-trystream-type-error"></span>`type Error = E`

- <span id="tryreadychunks-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for TryReadyChunks<St>`

##### `impl<St: TryStream> Unpin for TryReadyChunks<St>`

### `TryReadyChunksError<T, E>`

```rust
struct TryReadyChunksError<T, E>(alloc::vec::Vec<T>, E);
```

Error indicating, that while chunk was collected inner stream produced an error.

Contains all items that were collected before an error occurred, and the stream error itself.

#### Trait Implementations

##### `impl<T, E: fmt::Debug> Debug for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E: fmt::Display> Display for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq, E: cmp::Eq> Eq for TryReadyChunksError<T, E>`

##### `impl<T: cmp::PartialEq, E: cmp::PartialEq> PartialEq for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-partialeq-eq"></span>`fn eq(&self, other: &TryReadyChunksError<T, E>) -> bool` — [`TryReadyChunksError`](try_stream/try_ready_chunks/index.md#tryreadychunkserror)

##### `impl<T, E> StructuralPartialEq for TryReadyChunksError<T, E>`

##### `impl<T> ToString for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Iter<I>`

```rust
struct Iter<I> {
    iter: I,
}
```

Stream for the [`iter`](iter/index.md) function.

#### Implementations

- <span id="iter-get-ref"></span>`fn get_ref(&self) -> &I`

  Acquires a reference to the underlying iterator that this stream is pulling from.

- <span id="iter-get-mut"></span>`fn get_mut(&mut self) -> &mut I`

  Acquires a mutable reference to the underlying iterator that this stream is pulling from.

- <span id="iter-into-inner"></span>`fn into_inner(self) -> I`

  Consumes this stream, returning the underlying iterator.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Iter<I>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<I>` — [`Iter`](iter/index.md#iter)

##### `impl<I: fmt::Debug> Debug for Iter<I>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Stream for Iter<I>`

- <span id="iter-stream-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<I as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

- <span id="iter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Iter<I>`

##### `impl TryStream for Iter<I>`

- <span id="iter-trystream-type-ok"></span>`type Ok = T`

- <span id="iter-trystream-type-error"></span>`type Error = E`

- <span id="iter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Iter<I>`

##### `impl<I> Unpin for Iter<I>`

### `Repeat<T>`

```rust
struct Repeat<T> {
    item: T,
}
```

Stream for the [`repeat`](repeat/index.md) function.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Repeat<T>`

- <span id="repeat-clone"></span>`fn clone(&self) -> Repeat<T>` — [`Repeat`](repeat/index.md#repeat)

##### `impl<T: fmt::Debug> Debug for Repeat<T>`

- <span id="repeat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedStream for Repeat<T>`

- <span id="repeat-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Stream for Repeat<T>`

- <span id="repeat-stream-type-item"></span>`type Item = T`

- <span id="repeat-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="repeat-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for Repeat<T>`

##### `impl<T> TryStream for Repeat<T>`

- <span id="repeat-trystream-type-ok"></span>`type Ok = T`

- <span id="repeat-trystream-type-error"></span>`type Error = E`

- <span id="repeat-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Repeat<T>`

##### `impl<T> Unpin for Repeat<T>`

### `RepeatWith<F>`

```rust
struct RepeatWith<F> {
    repeater: F,
}
```

An stream that repeats elements of type `A` endlessly by
applying the provided closure `F: FnMut() -> A`.

This `struct` is created by the [`repeat_with()`](repeat_with/index.md) function.
See its documentation for more.

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for RepeatWith<F>`

- <span id="repeatwith-clone"></span>`fn clone(&self) -> RepeatWith<F>` — [`RepeatWith`](repeat_with/index.md#repeatwith)

##### `impl<F: fmt::Debug> Debug for RepeatWith<F>`

- <span id="repeatwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: FnMut() -> A> FusedStream for RepeatWith<F>`

- <span id="repeatwith-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F: FnMut() -> A> Stream for RepeatWith<F>`

- <span id="repeatwith-stream-type-item"></span>`type Item = A`

- <span id="repeatwith-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="repeatwith-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for RepeatWith<F>`

##### `impl TryStream for RepeatWith<F>`

- <span id="repeatwith-trystream-type-ok"></span>`type Ok = T`

- <span id="repeatwith-trystream-type-error"></span>`type Error = E`

- <span id="repeatwith-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for RepeatWith<F>`

##### `impl<F: FnMut() -> A> Unpin for RepeatWith<F>`

### `Empty<T>`

```rust
struct Empty<T> {
    _phantom: core::marker::PhantomData<T>,
}
```

Stream for the [`empty`](empty/index.md) function.

#### Trait Implementations

##### `impl<T> Clone for Empty<T>`

- <span id="empty-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Empty<T>`

- <span id="empty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedStream for Empty<T>`

- <span id="empty-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Stream for Empty<T>`

- <span id="empty-stream-type-item"></span>`type Item = T`

- <span id="empty-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="empty-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for Empty<T>`

##### `impl<T> TryStream for Empty<T>`

- <span id="empty-trystream-type-ok"></span>`type Ok = T`

- <span id="empty-trystream-type-error"></span>`type Error = E`

- <span id="empty-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Empty<T>`

##### `impl<T> Unpin for Empty<T>`

### `Once<Fut>`

```rust
struct Once<Fut> {
    future: Option<Fut>,
}
```

A stream which emits single element and then EOF.

#### Implementations

- <span id="once-new"></span>`fn new(future: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for Once<Fut>`

- <span id="once-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> FusedStream for Once<Fut>`

- <span id="once-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Stream for Once<Fut>`

- <span id="once-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="once-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="once-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Once<Fut>`

##### `impl TryStream for Once<Fut>`

- <span id="once-trystream-type-ok"></span>`type Ok = T`

- <span id="once-trystream-type-error"></span>`type Error = E`

- <span id="once-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Once<Fut>`

##### `impl<Fut> Unpin for Once<Fut>`

### `Pending<T>`

```rust
struct Pending<T> {
    _data: marker::PhantomData<T>,
}
```

Stream for the [`pending()`](pending/index.md) function.

#### Trait Implementations

##### `impl<T> Clone for Pending<T>`

- <span id="pending-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Pending<T>`

- <span id="pending-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedStream for Pending<T>`

- <span id="pending-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Stream for Pending<T>`

- <span id="pending-stream-type-item"></span>`type Item = T`

- <span id="pending-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="pending-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for Pending<T>`

##### `impl<T> TryStream for Pending<T>`

- <span id="pending-trystream-type-ok"></span>`type Ok = T`

- <span id="pending-trystream-type-error"></span>`type Error = E`

- <span id="pending-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Pending<T>`

##### `impl<T> Unpin for Pending<T>`

### `PollFn<F>`

```rust
struct PollFn<F> {
    f: F,
}
```

Stream for the [`poll_fn`](poll_fn/index.md) function.

#### Trait Implementations

##### `impl<F> Debug for PollFn<F>`

- <span id="pollfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> Stream for PollFn<F>`

- <span id="pollfn-stream-type-item"></span>`type Item = T`

- <span id="pollfn-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl StreamExt for PollFn<F>`

##### `impl TryStream for PollFn<F>`

- <span id="pollfn-trystream-type-ok"></span>`type Ok = T`

- <span id="pollfn-trystream-type-error"></span>`type Error = E`

- <span id="pollfn-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for PollFn<F>`

##### `impl<F> Unpin for PollFn<F>`

### `PollImmediate<S>`

```rust
struct PollImmediate<S> {
    stream: Option<S>,
}
```

Stream for the [poll_immediate](poll_immediate()) function.

It will never return [Poll::Pending](core::task::Poll::Pending)

#### Trait Implementations

##### `impl<S: clone::Clone> Clone for PollImmediate<S>`

- <span id="pollimmediate-clone"></span>`fn clone(&self) -> PollImmediate<S>` — [`PollImmediate`](poll_immediate/index.md#pollimmediate)

##### `impl<S: fmt::Debug> Debug for PollImmediate<S>`

- <span id="pollimmediate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Stream> FusedStream for PollImmediate<S>`

- <span id="pollimmediate-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S> Stream for PollImmediate<S>`

- <span id="pollimmediate-stream-type-item"></span>`type Item = Poll<T>`

- <span id="pollimmediate-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="pollimmediate-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for PollImmediate<S>`

##### `impl<S> Unpin for PollImmediate<S>`

### `Select<St1, St2>`

```rust
struct Select<St1, St2> {
    inner: crate::stream::SelectWithStrategy<St1, St2, fn(&mut crate::stream::PollNext) -> crate::stream::PollNext, crate::stream::PollNext>,
}
```

Stream for the [`select()`](select/index.md) function.

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

- <span id="select-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St1 as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl StreamExt for Select<St1, St2>`

##### `impl TryStream for Select<St1, St2>`

- <span id="select-trystream-type-ok"></span>`type Ok = T`

- <span id="select-trystream-type-error"></span>`type Error = E`

- <span id="select-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Select<St1, St2>`

##### `impl<St1, St2> Unpin for Select<St1, St2>`

### `SelectWithStrategy<St1, St2, Clos, State>`

```rust
struct SelectWithStrategy<St1, St2, Clos, State> {
    stream1: St1,
    stream2: St2,
    internal_state: InternalState,
    state: State,
    clos: Clos,
}
```

Stream for the [`select_with_strategy()`](select_with_strategy/index.md) function. See function docs for details.

#### Implementations

- <span id="selectwithstrategy-get-ref"></span>`fn get_ref(&self) -> (&St1, &St2)`

  Acquires a reference to the underlying streams that this combinator is

  pulling from.

- <span id="selectwithstrategy-get-mut"></span>`fn get_mut(&mut self) -> (&mut St1, &mut St2)`

  Acquires a mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="selectwithstrategy-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut St1>, Pin<&mut St2>)`

  Acquires a pinned mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="selectwithstrategy-into-inner"></span>`fn into_inner(self) -> (St1, St2)`

  Consumes this combinator, returning the underlying streams.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St1, St2, Clos, State> Debug for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St1, St2, Clos, State> FusedStream for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St1, St2, Clos, State> Stream for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-stream-type-item"></span>`type Item = <St1 as Stream>::Item`

- <span id="selectwithstrategy-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St1 as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl StreamExt for SelectWithStrategy<St1, St2, Clos, State>`

##### `impl TryStream for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-trystream-type-ok"></span>`type Ok = T`

- <span id="selectwithstrategy-trystream-type-error"></span>`type Error = E`

- <span id="selectwithstrategy-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for SelectWithStrategy<St1, St2, Clos, State>`

##### `impl<St1, St2, Clos, State> Unpin for SelectWithStrategy<St1, St2, Clos, State>`

### `Unfold<T, F, Fut>`

```rust
struct Unfold<T, F, Fut> {
    f: F,
    state: crate::unfold_state::UnfoldState<T, Fut>,
}
```

Stream for the [`unfold`](unfold/index.md) function.

#### Trait Implementations

##### `impl<T, F, Fut> Debug for Unfold<T, F, Fut>`

- <span id="unfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F, Fut> FusedStream for Unfold<T, F, Fut>`

- <span id="unfold-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T, F, Fut> Stream for Unfold<T, F, Fut>`

- <span id="unfold-stream-type-item"></span>`type Item = Item`

- <span id="unfold-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl<T> StreamExt for Unfold<T, F, Fut>`

##### `impl<T> TryStream for Unfold<T, F, Fut>`

- <span id="unfold-trystream-type-ok"></span>`type Ok = T`

- <span id="unfold-trystream-type-error"></span>`type Error = E`

- <span id="unfold-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Unfold<T, F, Fut>`

##### `impl<T, F, Fut> Unpin for Unfold<T, F, Fut>`

### `FuturesOrdered<T: Future>`

```rust
struct FuturesOrdered<T: Future> {
    in_progress_queue: crate::stream::FuturesUnordered<OrderWrapper<T>>,
    queued_outputs: alloc::collections::binary_heap::BinaryHeap<OrderWrapper<<T as >::Output>>,
    next_incoming_index: i64,
    next_outgoing_index: i64,
}
```

An unbounded queue of futures.

This "combinator" is similar to [`FuturesUnordered`](futures_unordered/index.md), but it imposes a FIFO
order on top of the set of futures. While futures in the set will race to
completion in parallel, results will only be returned in the order their
originating futures were added to the queue.

Futures are pushed into this queue and their realized values are yielded in
order. This structure is optimized to manage a large number of futures.
Futures managed by [`FuturesOrdered`](futures_ordered/index.md) will only be polled when they generate
notifications. This reduces the required amount of work needed to coordinate
large numbers of futures.

When a [`FuturesOrdered`](futures_ordered/index.md) is first created, it does not contain any futures.
Calling [`poll_next`](FuturesOrdered::poll_next) in this state will result
in [`Poll::Ready(None)`](Poll::Ready) to be returned. Futures are submitted
to the queue using [`push_back`](FuturesOrdered::push_back) (or
[`push_front`](FuturesOrdered::push_front)); however, the future will
**not** be polled at this point. [`FuturesOrdered`](futures_ordered/index.md) will only poll managed
futures when `FuturesOrdered::poll_next` is called. As such, it
is important to call [`poll_next`](FuturesOrdered::poll_next) after pushing
new futures.

If `FuturesOrdered::poll_next` returns [`Poll::Ready(None)`](Poll::Ready)
this means that the queue is currently not managing any futures. A future
may be submitted to the queue at a later time. At that point, a call to
`FuturesOrdered::poll_next` will either return the future's resolved value
**or** `Poll::Pending` if the future has not yet completed. When
multiple futures are submitted to the queue, `FuturesOrdered::poll_next`
will return `Poll::Pending` until the first future completes, even if
some of the later futures have already completed.

Note that you can create a ready-made [`FuturesOrdered`](futures_ordered/index.md) via the
[`collect`](Iterator::collect) method, or you can start with an empty queue
with the `FuturesOrdered::new` constructor.

This type is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

#### Implementations

- <span id="futuresordered-new"></span>`fn new() -> Self`

  Constructs a new, empty `FuturesOrdered`

  

  The returned [`FuturesOrdered`](futures_ordered/index.md) does not contain any futures and, in

  this state, `FuturesOrdered::poll_next` will return

  [`Poll::Ready(None)`](Poll::Ready).

- <span id="futuresordered-len"></span>`fn len(&self) -> usize`

  Returns the number of futures contained in the queue.

  

  This represents the total number of in-flight futures, both

  those currently processing and those that have completed but

  which are waiting for earlier futures to complete.

- <span id="futuresordered-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the queue contains no futures

- <span id="futuresordered-push"></span>`fn push(&mut self, future: Fut)`

  Push a future into the queue.

  

  This function submits the given future to the internal set for managing.

  This function will not call [`poll`](Future::poll) on the submitted

  future. The caller must ensure that `FuturesOrdered::poll_next` is

  called in order to receive task notifications.

- <span id="futuresordered-push-back"></span>`fn push_back(&mut self, future: Fut)`

  Pushes a future to the back of the queue.

  

  This function submits the given future to the internal set for managing.

  This function will not call [`poll`](Future::poll) on the submitted

  future. The caller must ensure that `FuturesOrdered::poll_next` is

  called in order to receive task notifications.

- <span id="futuresordered-push-front"></span>`fn push_front(&mut self, future: Fut)`

  Pushes a future to the front of the queue.

  

  This function submits the given future to the internal set for managing.

  This function will not call [`poll`](Future::poll) on the submitted

  future. The caller must ensure that `FuturesOrdered::poll_next` is

  called in order to receive task notifications. This future will be

  the next future to be returned complete.

- <span id="futuresordered-clear"></span>`fn clear(&mut self)`

  Clear the whole `FuturesOrdered` queue.

  

  This function clears the pending futures and the queued outputs

  to make it fully empty.

#### Trait Implementations

##### `impl<Fut: Future> Debug for FuturesOrdered<Fut>`

- <span id="futuresordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> Default for FuturesOrdered<Fut>`

- <span id="futuresordered-default"></span>`fn default() -> Self`

##### `impl<Fut: Future> Extend for FuturesOrdered<Fut>`

- <span id="futuresordered-extend"></span>`fn extend<I>(&mut self, iter: I)`

##### `impl<Fut: Future> FromIterator for FuturesOrdered<Fut>`

- <span id="futuresordered-fromiterator-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl<Fut: Future> FusedStream for FuturesOrdered<Fut>`

- <span id="futuresordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Stream for FuturesOrdered<Fut>`

- <span id="futuresordered-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="futuresordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="futuresordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for FuturesOrdered<T>`

##### `impl<T> TryStream for FuturesOrdered<T>`

- <span id="futuresordered-trystream-type-ok"></span>`type Ok = T`

- <span id="futuresordered-trystream-type-error"></span>`type Error = E`

- <span id="futuresordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for FuturesOrdered<T>`

##### `impl<T: Future> Unpin for FuturesOrdered<T>`

### `FuturesUnordered<Fut>`

```rust
struct FuturesUnordered<Fut> {
    ready_to_run_queue: alloc::sync::Arc<self::ready_to_run_queue::ReadyToRunQueue<Fut>>,
    head_all: core::sync::atomic::AtomicPtr<self::task::Task<Fut>>,
    is_terminated: core::sync::atomic::AtomicBool,
}
```

A set of futures which may complete in any order.

See [`FuturesOrdered`](crate::stream::FuturesOrdered) for a version of this
type that preserves a FIFO order.

This structure is optimized to manage a large number of futures.
Futures managed by [`FuturesUnordered`](futures_unordered/index.md) will only be polled when they
generate wake-up notifications. This reduces the required amount of work
needed to poll large numbers of futures.

[`FuturesUnordered`](futures_unordered/index.md) can be filled by [`collect`](Iterator::collect)ing an
iterator of futures into a [`FuturesUnordered`](futures_unordered/index.md), or by
[`push`](FuturesUnordered::push)ing futures onto an existing
[`FuturesUnordered`](futures_unordered/index.md). When new futures are added,
[`poll_next`](Stream::poll_next) must be called in order to begin receiving
wake-ups for new futures.

Note that you can create a ready-made [`FuturesUnordered`](futures_unordered/index.md) via the
[`collect`](Iterator::collect) method, or you can start with an empty set
with the `FuturesUnordered::new` constructor.

This type is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

#### Implementations

- <span id="futuresunordered-new"></span>`fn new() -> Self`

  Constructs a new, empty [`FuturesUnordered`](futures_unordered/index.md).

  

  The returned [`FuturesUnordered`](futures_unordered/index.md) does not contain any futures.

  In this state, [`FuturesUnordered::poll_next`](Stream::poll_next) will

  return [`Poll::Ready(None)`](Poll::Ready).

- <span id="futuresunordered-len"></span>`fn len(&self) -> usize`

  Returns the number of futures contained in the set.

  

  This represents the total number of in-flight futures.

- <span id="futuresunordered-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the set contains no futures.

- <span id="futuresunordered-push"></span>`fn push(&self, future: Fut)`

  Push a future into the set.

  

  This method adds the given future to the set. This method will not

  call [`poll`](core::future::Future::poll) on the submitted future. The caller must

  ensure that [`FuturesUnordered::poll_next`](Stream::poll_next) is called

  in order to receive wake-up notifications for the given future.

- <span id="futuresunordered-iter"></span>`fn iter(&self) -> Iter<'_, Fut>` — [`Iter`](futures_unordered/iter/index.md#iter)

  Returns an iterator that allows inspecting each future in the set.

- <span id="futuresunordered-iter-pin-ref"></span>`fn iter_pin_ref(self: Pin<&Self>) -> IterPinRef<'_, Fut>` — [`IterPinRef`](futures_unordered/iter/index.md#iterpinref)

  Returns an iterator that allows inspecting each future in the set.

- <span id="futuresunordered-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, Fut>` — [`IterMut`](futures_unordered/iter/index.md#itermut)

  Returns an iterator that allows modifying each future in the set.

- <span id="futuresunordered-iter-pin-mut"></span>`fn iter_pin_mut(self: Pin<&mut Self>) -> IterPinMut<'_, Fut>` — [`IterPinMut`](futures_unordered/iter/index.md#iterpinmut)

  Returns an iterator that allows modifying each future in the set.

- <span id="futuresunordered-atomic-load-head-and-len-all"></span>`fn atomic_load_head_and_len_all(&self) -> (*const Task<Fut>, usize)` — [`Task`](futures_unordered/task/index.md#task)

  Returns the current head node and number of futures in the list of all

  futures within a context where access is shared with other threads

  (mostly for use with the `len` and `iter_pin_ref` methods).

- <span id="futuresunordered-release-task"></span>`fn release_task(&mut self, task: Arc<Task<Fut>>)` — [`Task`](futures_unordered/task/index.md#task)

  Releases the task. It destroys the future inside and either drops

  the `Arc<Task>` or transfers ownership to the ready to run queue.

  The task this method is called on must have been unlinked before.

- <span id="futuresunordered-link"></span>`fn link(&self, task: Arc<Task<Fut>>) -> *const Task<Fut>` — [`Task`](futures_unordered/task/index.md#task)

  Insert a new task into the internal linked list.

- <span id="futuresunordered-unlink"></span>`unsafe fn unlink(&mut self, task: *const Task<Fut>) -> Arc<Task<Fut>>` — [`Task`](futures_unordered/task/index.md#task)

  Remove the task from the linked list tracking all tasks currently

  managed by `FuturesUnordered`.

  This method is unsafe because it has be guaranteed that `task` is a

  valid pointer.

- <span id="futuresunordered-pending-next-all"></span>`fn pending_next_all(&self) -> *mut Task<Fut>` — [`Task`](futures_unordered/task/index.md#task)

  Returns the reserved value for `Task::next_all` to indicate a pending

  assignment from the thread that inserted the task.

  

  `FuturesUnordered::link` needs to update `Task` pointers in an order

  that ensures any iterators created on other threads can correctly

  traverse the entire `Task` list using the chain of `next_all` pointers.

  This could be solved with a compare-exchange loop that stores the

  current `head_all` in `next_all` and swaps out `head_all` with the new

  `Task` pointer if the head hasn't already changed. Under heavy thread

  contention, this compare-exchange loop could become costly.

  

  An alternative is to initialize `next_all` to a reserved pending state

  first, perform an atomic swap on `head_all`, and finally update

  `next_all` with the old head node. Iterators will then either see the

  pending state value or the correct next node pointer, and can reload

  `next_all` as needed until the correct value is loaded. The number of

  retries needed (if any) would be small and will always be finite, so

  this should generally perform better than the compare-exchange loop.

  

  A valid `Task` pointer in the `head_all` list is guaranteed to never be

  this value, so it is safe to use as a reserved value until the correct

  value can be written.

#### Trait Implementations

##### `impl<Fut> Debug for FuturesUnordered<Fut>`

- <span id="futuresunordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> Default for FuturesUnordered<Fut>`

- <span id="futuresunordered-default"></span>`fn default() -> Self`

##### `impl<Fut> Drop for FuturesUnordered<Fut>`

- <span id="futuresunordered-drop"></span>`fn drop(&mut self)`

##### `impl<Fut> Extend for FuturesUnordered<Fut>`

- <span id="futuresunordered-extend"></span>`fn extend<I>(&mut self, iter: I)`

##### `impl<Fut> FromIterator for FuturesUnordered<Fut>`

- <span id="futuresunordered-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Self`

##### `impl<Fut: Future> FusedStream for FuturesUnordered<Fut>`

- <span id="futuresunordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Unpin> IntoIterator for &'a FuturesUnordered<Fut>`

- <span id="a-futuresunordered-intoiterator-type-item"></span>`type Item = &'a Fut`

- <span id="a-futuresunordered-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, Fut>`

- <span id="a-futuresunordered-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LocalSpawn for FuturesUnordered<futures_task::LocalFutureObj<'_, ()>>`

- <span id="futuresunordered-localspawn-spawn-local-obj"></span>`fn spawn_local_obj(&self, future_obj: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>` — [`LocalFutureObj`](../future/index.md#localfutureobj), [`SpawnError`](../task/index.md#spawnerror)

##### `impl LocalSpawnExt for FuturesUnordered<Fut>`

##### `impl<Fut: Send> Send for FuturesUnordered<Fut>`

##### `impl Spawn for FuturesUnordered<futures_task::FutureObj<'_, ()>>`

- <span id="futuresunordered-spawn-spawn-obj"></span>`fn spawn_obj(&self, future_obj: FutureObj<'static, ()>) -> Result<(), SpawnError>` — [`FutureObj`](../future/index.md#futureobj), [`SpawnError`](../task/index.md#spawnerror)

##### `impl SpawnExt for FuturesUnordered<Fut>`

##### `impl<Fut: Future> Stream for FuturesUnordered<Fut>`

- <span id="futuresunordered-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="futuresunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

- <span id="futuresunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FuturesUnordered<Fut>`

##### `impl<Fut: Send + Sync> Sync for FuturesUnordered<Fut>`

##### `impl TryStream for FuturesUnordered<Fut>`

- <span id="futuresunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="futuresunordered-trystream-type-error"></span>`type Error = E`

- <span id="futuresunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for FuturesUnordered<Fut>`

##### `impl<Fut> Unpin for FuturesUnordered<Fut>`

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

- <span id="selectall-iter"></span>`fn iter(&self) -> Iter<'_, St>` — [`Iter`](select_all/index.md#iter)

  Returns an iterator that allows inspecting each stream in the set.

- <span id="selectall-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, St>` — [`IterMut`](select_all/index.md#itermut)

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

- <span id="selectall-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl StreamExt for SelectAll<St>`

##### `impl TryStream for SelectAll<St>`

- <span id="selectall-trystream-type-ok"></span>`type Ok = T`

- <span id="selectall-trystream-type-error"></span>`type Error = E`

- <span id="selectall-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for SelectAll<St>`

### `AbortHandle`

```rust
struct AbortHandle {
    inner: alloc::sync::Arc<AbortInner>,
}
```

A handle to an `Abortable` task.

#### Implementations

- <span id="aborthandle-new-pair"></span>`fn new_pair() -> (Self, AbortRegistration)` — [`AbortRegistration`](../abortable/index.md#abortregistration)

  Creates an (`AbortHandle`, `AbortRegistration`) pair which can be used

  to abort a running future or stream.

  

  This function is usually paired with a call to `Abortable::new`.

#### Trait Implementations

##### `impl Clone for AbortHandle`

- <span id="aborthandle-clone"></span>`fn clone(&self) -> AbortHandle` — [`AbortHandle`](../abortable/index.md#aborthandle)

##### `impl Debug for AbortHandle`

- <span id="aborthandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AbortRegistration`

```rust
struct AbortRegistration {
    inner: alloc::sync::Arc<AbortInner>,
}
```

A registration handle for an `Abortable` task.
Values of this type can be acquired from `AbortHandle::new` and are used
in calls to `Abortable::new`.

#### Implementations

- <span id="abortregistration-handle"></span>`fn handle(&self) -> AbortHandle` — [`AbortHandle`](../abortable/index.md#aborthandle)

  Create an [`AbortHandle`](../abortable/index.md) from the given [`AbortRegistration`](../abortable/index.md).

  

  The created [`AbortHandle`](../abortable/index.md) is functionally the same as any other

  [`AbortHandle`](../abortable/index.md)s that are associated with the same [`AbortRegistration`](../abortable/index.md),

  such as the one created by `AbortHandle::new_pair`.

#### Trait Implementations

##### `impl Debug for AbortRegistration`

- <span id="abortregistration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Abortable<T>`

```rust
struct Abortable<T> {
    task: T,
    inner: alloc::sync::Arc<AbortInner>,
}
```

A future/stream which can be remotely short-circuited using an `AbortHandle`.

#### Implementations

- <span id="abortable-new"></span>`fn new(task: T, reg: AbortRegistration) -> Self` — [`AbortRegistration`](../abortable/index.md#abortregistration)

  Creates a new `Abortable` future/stream using an existing `AbortRegistration`.

  `AbortRegistration`s can be acquired through `AbortHandle::new`.

  

  When `abort` is called on the handle tied to `reg` or if `abort` has

  already been called, the future/stream will complete immediately without making

  any further progress.

  

  # Examples:

  

  Usage with futures:

  

  ```rust

  futures::executor::block_on(async {

  use futures::future::{Abortable, AbortHandle, Aborted};

  

  let (abort_handle, abort_registration) = AbortHandle::new_pair();

  let future = Abortable::new(async { 2 }, abort_registration);

  abort_handle.abort();

  assert_eq!(future.await, Err(Aborted));

  });

  ```

  

  Usage with streams:

  

  ```rust

  futures::executor::block_on(async {

  use futures::future::{Abortable, AbortHandle};

  use futures::stream::{self, StreamExt};

  

  let (abort_handle, abort_registration) = AbortHandle::new_pair();

  let mut stream = Abortable::new(stream::iter(vec![1, 2, 3]), abort_registration);

  abort_handle.abort();

  assert_eq!(stream.next().await, None);

  });

  ```

- <span id="abortable-is-aborted"></span>`fn is_aborted(&self) -> bool`

  Checks whether the task has been aborted. Note that all this

  method indicates is whether `AbortHandle::abort` was *called*.

  This means that it will return `true` even if:

  * `abort` was called after the task had completed.

  * `abort` was called while the task was being polled - the task may still be running and

    will not be stopped until `poll` returns.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Abortable<T>`

- <span id="abortable-clone"></span>`fn clone(&self) -> Abortable<T>` — [`Abortable`](../abortable/index.md#abortable)

##### `impl<T: fmt::Debug> Debug for Abortable<T>`

- <span id="abortable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> Future for Abortable<Fut>`

- <span id="abortable-future-type-output"></span>`type Output = Result<<Fut as Future>::Output, Aborted>`

- <span id="abortable-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<T> FutureExt for Abortable<T>`

##### `impl IntoFuture for Abortable<T>`

- <span id="abortable-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="abortable-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="abortable-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St> Stream for Abortable<St>`

- <span id="abortable-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="abortable-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](#stream)

##### `impl<T> StreamExt for Abortable<T>`

##### `impl<T> TryFuture for Abortable<T>`

- <span id="abortable-tryfuture-type-ok"></span>`type Ok = T`

- <span id="abortable-tryfuture-type-error"></span>`type Error = E`

- <span id="abortable-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Abortable<T>`

##### `impl<T> TryStream for Abortable<T>`

- <span id="abortable-trystream-type-ok"></span>`type Ok = T`

- <span id="abortable-trystream-type-error"></span>`type Error = E`

- <span id="abortable-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](#trystream)

##### `impl TryStreamExt for Abortable<T>`

##### `impl<T> Unpin for Abortable<T>`

### `Aborted`

```rust
struct Aborted;
```

Indicator that the `Abortable` task was aborted.

#### Trait Implementations

##### `impl Clone for Aborted`

- <span id="aborted-clone"></span>`fn clone(&self) -> Aborted` — [`Aborted`](../abortable/index.md#aborted)

##### `impl Copy for Aborted`

##### `impl Debug for Aborted`

- <span id="aborted-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Aborted`

- <span id="aborted-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Aborted`

##### `impl PartialEq for Aborted`

- <span id="aborted-partialeq-eq"></span>`fn eq(&self, other: &Aborted) -> bool` — [`Aborted`](../abortable/index.md#aborted)

##### `impl StructuralPartialEq for Aborted`

##### `impl ToString for Aborted`

- <span id="aborted-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `PollNext`

```rust
enum PollNext {
    Left,
    Right,
}
```

Type to tell [`SelectWithStrategy`](select_with_strategy/index.md) which stream to poll next.

#### Variants

- **`Left`**

  Poll the first stream.

- **`Right`**

  Poll the second stream.

#### Implementations

- <span id="pollnext-toggle"></span>`fn toggle(&mut self) -> Self`

  Toggle the value and return the old one.

- <span id="pollnext-other"></span>`fn other(&self) -> Self`

#### Trait Implementations

##### `impl Clone for PollNext`

- <span id="pollnext-clone"></span>`fn clone(&self) -> PollNext` — [`PollNext`](select_with_strategy/index.md#pollnext)

##### `impl Copy for PollNext`

##### `impl Debug for PollNext`

- <span id="pollnext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PollNext`

- <span id="pollnext-default"></span>`fn default() -> Self`

##### `impl Eq for PollNext`

##### `impl Hash for PollNext`

- <span id="pollnext-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for PollNext`

- <span id="pollnext-partialeq-eq"></span>`fn eq(&self, other: &PollNext) -> bool` — [`PollNext`](select_with_strategy/index.md#pollnext)

##### `impl StructuralPartialEq for PollNext`

## Traits

### `StreamExt`

```rust
trait StreamExt: Stream { ... }
```

An extension trait for `Stream`s that provides a variety of convenient
combinator functions.

#### Provided Methods

- `fn next(&mut self) -> Next<'_, Self>`

  Creates a future that resolves to the next item in the stream.

- `fn into_future(self) -> StreamFuture<Self>`

  Converts this stream into a future of `(next_item, tail_of_stream)`.

- `fn map<T, F>(self, f: F) -> Map<Self, F>`

  Maps this stream's items to a different type, returning a new stream of

- `fn enumerate(self) -> Enumerate<Self>`

  Creates a stream which gives the current iteration count as well as

- `fn filter<Fut, F>(self, f: F) -> Filter<Self, Fut, F>`

  Filters the values produced by this stream according to the provided

- `fn filter_map<Fut, T, F>(self, f: F) -> FilterMap<Self, Fut, F>`

  Filters the values produced by this stream while simultaneously mapping

- `fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>`

  Computes from this stream's items new items of a different type using

- `fn collect<C: Default + Extend<<Self as >::Item>>(self) -> Collect<Self, C>`

  Transforms a stream into a collection, returning a

- `fn unzip<A, B, FromA, FromB>(self) -> Unzip<Self, FromA, FromB>`

  Converts a stream of pairs into a future, which

- `fn concat(self) -> Concat<Self>`

  Concatenate all items of a stream into a single extendable

- `fn count(self) -> Count<Self>`

  Drives the stream to completion, counting the number of items.

- `fn cycle(self) -> Cycle<Self>`

  Repeats a stream endlessly.

- `fn fold<T, Fut, F>(self, init: T, f: F) -> Fold<Self, Fut, T, F>`

  Execute an accumulating asynchronous computation over a stream,

- `fn any<Fut, F>(self, f: F) -> Any<Self, Fut, F>`

  Execute predicate over asynchronous stream, and return `true` if any element in stream satisfied a predicate.

- `fn all<Fut, F>(self, f: F) -> All<Self, Fut, F>`

  Execute predicate over asynchronous stream, and return `true` if all element in stream satisfied a predicate.

- `fn flatten(self) -> Flatten<Self>`

  Flattens a stream of streams into just one continuous stream.

- `fn flatten_unordered(self, limit: impl Into<Option<usize>>) -> FlattenUnordered<Self>`

  Flattens a stream of streams into just one continuous stream. Polls

- `fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>`

  Maps a stream like `StreamExt::map` but flattens nested `Stream`s.

- `fn flat_map_unordered<U, F>(self, limit: impl Into<Option<usize>>, f: F) -> FlatMapUnordered<Self, U, F>`

  Maps a stream like `StreamExt::map` but flattens nested `Stream`s

- `fn scan<S, B, Fut, F>(self, initial_state: S, f: F) -> Scan<Self, S, Fut, F>`

  Combinator similar to `StreamExt::fold` that holds internal state

- `fn skip_while<Fut, F>(self, f: F) -> SkipWhile<Self, Fut, F>`

  Skip elements on this stream while the provided asynchronous predicate

- `fn take_while<Fut, F>(self, f: F) -> TakeWhile<Self, Fut, F>`

  Take elements from this stream while the provided asynchronous predicate

- `fn take_until<Fut>(self, fut: Fut) -> TakeUntil<Self, Fut>`

  Take elements from this stream until the provided future resolves.

- `fn for_each<Fut, F>(self, f: F) -> ForEach<Self, Fut, F>`

  Runs this stream to completion, executing the provided asynchronous

- `fn for_each_concurrent<Fut, F>(self, limit: impl Into<Option<usize>>, f: F) -> ForEachConcurrent<Self, Fut, F>`

  Runs this stream to completion, executing the provided asynchronous

- `fn take(self, n: usize) -> Take<Self>`

  Creates a new stream of at most `n` items of the underlying stream.

- `fn skip(self, n: usize) -> Skip<Self>`

  Creates a new stream which skips `n` items of the underlying stream.

- `fn fuse(self) -> Fuse<Self>`

  Fuse a stream such that [`poll_next`](Stream::poll_next) will never

- `fn by_ref(&mut self) -> &mut Self`

  Borrows a stream, rather than consuming it.

- `fn boxed<'a>(self) -> BoxStream<'a, <Self as >::Item>`

  Wrap the stream in a Box, pinning it.

- `fn boxed_local<'a>(self) -> LocalBoxStream<'a, <Self as >::Item>`

  Wrap the stream in a Box, pinning it.

- `fn buffered(self, n: usize) -> Buffered<Self>`

  An adaptor for creating a buffered list of pending futures.

- `fn buffer_unordered(self, n: usize) -> BufferUnordered<Self>`

  An adaptor for creating a buffered list of pending futures (unordered).

- `fn zip<St>(self, other: St) -> Zip<Self, St>`

  An adapter for zipping two streams together.

- `fn chain<St>(self, other: St) -> Chain<Self, St>`

  Adapter for chaining two streams.

- `fn peekable(self) -> Peekable<Self>`

  Creates a new stream which exposes a `peek` method.

- `fn chunks(self, capacity: usize) -> Chunks<Self>`

  An adaptor for chunking up items of the stream inside a vector.

- `fn ready_chunks(self, capacity: usize) -> ReadyChunks<Self>`

  An adaptor for chunking up ready items of the stream inside a vector.

- `fn forward<S>(self, sink: S) -> Forward<Self, S>`

  A future that completes after the given stream has been fully processed

- `fn split<Item>(self) -> (SplitSink<Self, Item>, SplitStream<Self>)`

  Splits this `Stream + Sink` object into separate `Sink` and `Stream`

- `fn inspect<F>(self, f: F) -> Inspect<Self, F>`

  Do something with each item of this stream, afterwards passing it on.

- `fn left_stream<B>(self) -> Either<Self, B>`

  Wrap this stream in an `Either` stream, making it the left-hand variant

- `fn right_stream<B>(self) -> Either<B, Self>`

  Wrap this stream in an `Either` stream, making it the right-hand variant

- `fn poll_next_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>`

  A convenience method for calling `Stream::poll_next` on `Unpin`

- `fn select_next_some(&mut self) -> SelectNextSome<'_, Self>`

  Returns a [`Future`](../future/index.md) that resolves when the next item in this stream is

#### Implementors

- `T`

### `TryStreamExt`

```rust
trait TryStreamExt: TryStream { ... }
```

Adapters specific to `Result`-returning streams

#### Provided Methods

- `fn err_into<E>(self) -> ErrInto<Self, E>`

  Wraps the current stream in a new stream which converts the error type

- `fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>`

  Wraps the current stream in a new stream which maps the success value

- `fn map_err<E, F>(self, f: F) -> MapErr<Self, F>`

  Wraps the current stream in a new stream which maps the error value

- `fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>`

  Chain on a computation for when a value is ready, passing the successful

- `fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>`

  Chain on a computation for when an error happens, passing the

- `fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>`

  Do something with the success value of this stream, afterwards passing

- `fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>`

  Do something with the error value of this stream, afterwards passing it on.

- `fn into_stream(self) -> IntoStream<Self>`

  Wraps a [`TryStream`](#trystream) into a type that implements

- `fn try_next(&mut self) -> TryNext<'_, Self>`

  Creates a future that attempts to resolve the next item in the stream.

- `fn try_for_each<Fut, F>(self, f: F) -> TryForEach<Self, Fut, F>`

  Attempts to run this stream to completion, executing the provided

- `fn try_skip_while<Fut, F>(self, f: F) -> TrySkipWhile<Self, Fut, F>`

  Skip elements on this stream while the provided asynchronous predicate

- `fn try_take_while<Fut, F>(self, f: F) -> TryTakeWhile<Self, Fut, F>`

  Take elements on this stream while the provided asynchronous predicate

- `fn try_for_each_concurrent<Fut, F>(self, limit: impl Into<Option<usize>>, f: F) -> TryForEachConcurrent<Self, Fut, F>`

  Attempts to run this stream to completion, executing the provided asynchronous

- `fn try_collect<C: Default + Extend<<Self as >::Ok>>(self) -> TryCollect<Self, C>`

  Attempt to transform a stream into a collection,

- `fn try_chunks(self, capacity: usize) -> TryChunks<Self>`

  An adaptor for chunking up successful items of the stream inside a vector.

- `fn try_ready_chunks(self, capacity: usize) -> TryReadyChunks<Self>`

  An adaptor for chunking up successful, ready items of the stream inside a vector.

- `fn try_filter<Fut, F>(self, f: F) -> TryFilter<Self, Fut, F>`

  Attempt to filter the values produced by this stream according to the

- `fn try_filter_map<Fut, F, T>(self, f: F) -> TryFilterMap<Self, Fut, F>`

  Attempt to filter the values produced by this stream while

- `fn try_flatten_unordered(self, limit: impl Into<Option<usize>>) -> TryFlattenUnordered<Self>`

  Flattens a stream of streams into just one continuous stream. Produced streams

- `fn try_flatten(self) -> TryFlatten<Self>`

  Flattens a stream of streams into just one continuous stream.

- `fn try_fold<T, Fut, F>(self, init: T, f: F) -> TryFold<Self, Fut, T, F>`

  Attempt to execute an accumulating asynchronous computation over a

- `fn try_concat(self) -> TryConcat<Self>`

  Attempt to concatenate all items of a stream into a single

- `fn try_buffer_unordered(self, n: usize) -> TryBufferUnordered<Self>`

  Attempt to execute several futures from a stream concurrently (unordered).

- `fn try_buffered(self, n: usize) -> TryBuffered<Self>`

  Attempt to execute several futures from a stream concurrently.

- `fn try_poll_next_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<<Self as >::Ok, <Self as >::Error>>>`

  A convenience method for calling `TryStream::try_poll_next` on `Unpin`

- `fn try_all<Fut, F>(self, f: F) -> TryAll<Self, Fut, F>`

  Attempt to execute a predicate over an asynchronous stream and evaluate if all items

- `fn try_any<Fut, F>(self, f: F) -> TryAny<Self, Fut, F>`

  Attempt to execute a predicate over an asynchronous stream and evaluate if any items

#### Implementors

- `S`

## Functions

### `BoxStream`

```rust
fn BoxStream() -> Self
```

### `TryStream`

```rust
fn TryStream(&self) -> U32X4
```

### `try_unfold`

```rust
fn try_unfold<T, F, Fut, Item>(init: T, f: F) -> TryUnfold<T, F, Fut>
where
    F: FnMut(T) -> Fut,
    Fut: TryFuture<Ok = Option<(Item, T)>>
```

Creates a `TryStream` from a seed and a closure returning a `TryFuture`.

This function is the dual for the `TryStream::try_fold()` adapter: while
`TryStream::try_fold()` reduces a `TryStream` to one single value,
`try_unfold()` creates a `TryStream` from a seed value.

`try_unfold()` will call the provided closure with the provided seed, then
wait for the returned `TryFuture` to complete with `(a, b)`. It will then
yield the value `a`, and use `b` as the next internal state.

If the closure returns `None` instead of `Some(TryFuture)`, then the
`try_unfold()` will stop producing items and return `Poll::Ready(None)` in
future calls to `poll()`.

In case of error generated by the returned `TryFuture`, the error will be
returned by the `TryStream`. The `TryStream` will then yield
`Poll::Ready(None)` in future calls to `poll()`.

This function can typically be used when wanting to go from the "world of
futures" to the "world of streams": the provided closure can build a
`TryFuture` using other library functions working on futures, and
`try_unfold()` will turn it into a `TryStream` by repeating the operation.

# Example

```rust
#[derive(Debug, PartialEq)]
struct SomeError;
futures::executor::block_on(async {
use futures::stream::{self, TryStreamExt};

let stream = stream::try_unfold(0, |state| async move {
    if state < 0 {
        return Err(SomeError);
    }

    if state <= 2 {
        let next_state = state + 1;
        let yielded = state * 2;
        Ok(Some((yielded, next_state)))
    } else {
        Ok(None)
    }
});

let result: Result<Vec<i32>, _> = stream.try_collect().await;
assert_eq!(result, Ok(vec![0, 2, 4]));
});
```

### `iter`

```rust
fn iter<I>(i: I) -> Iter<<I as >::IntoIter>
where
    I: IntoIterator
```

Converts an `Iterator` into a `Stream` which is always ready
to yield the next value.

Iterators in Rust don't express the ability to block, so this adapter
simply always calls `iter.next()` and returns that.

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::iter(vec![17, 19]);
assert_eq!(vec![17, 19], stream.collect::<Vec<i32>>().await);
});
```

### `repeat`

```rust
fn repeat<T>(item: T) -> Repeat<T>
where
    T: Clone
```

Create a stream which produces the same item repeatedly.

The stream never terminates. Note that you likely want to avoid
usage of `collect` or such on the returned stream as it will exhaust
available memory as it tries to just fill up all RAM.

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::repeat(9);
assert_eq!(vec![9, 9, 9], stream.take(3).collect::<Vec<i32>>().await);
});
```

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

### `empty`

```rust
fn empty<T>() -> Empty<T>
```

Creates a stream which contains no elements.

The returned stream will always return `Ready(None)` when polled.

### `once`

```rust
fn once<Fut: Future>(future: Fut) -> Once<Fut>
```

Creates a stream of a single element.

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::once(async { 17 });
let collected = stream.collect::<Vec<i32>>().await;
assert_eq!(collected, vec![17]);
});
```

### `pending`

```rust
fn pending<T>() -> Pending<T>
```

Creates a stream which never returns any elements.

The returned stream will always return `Pending` when polled.

### `poll_fn`

```rust
fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut futures_core::task::Context<'_>) -> futures_core::task::Poll<Option<T>>
```

Creates a new stream wrapping a function returning `Poll<Option<T>>`.

Polling the returned stream calls the wrapped function.

# Examples

```rust
use futures::stream::poll_fn;
use futures::task::Poll;

let mut counter = 1usize;

let read_stream = poll_fn(move |_| -> Poll<Option<String>> {
    if counter == 0 { return Poll::Ready(None); }
    counter -= 1;
    Poll::Ready(Some("Hello, World!".to_owned()))
});
```

### `poll_immediate`

```rust
fn poll_immediate<S: Stream>(s: S) -> PollImmediate<S>
```

Creates a new stream that always immediately returns [Poll::Ready](core::task::Poll::Ready) when awaiting it.

This is useful when immediacy is more important than waiting for the next item to be ready.

# Examples

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};
use futures::task::Poll;

let mut r = stream::poll_immediate(Box::pin(stream::iter(1_u32..3)));
assert_eq!(r.next().await, Some(Poll::Ready(1)));
assert_eq!(r.next().await, Some(Poll::Ready(2)));
assert_eq!(r.next().await, None);

let mut p = stream::poll_immediate(Box::pin(stream::once(async {
    futures::pending!();
    42_u8
})));
assert_eq!(p.next().await, Some(Poll::Pending));
assert_eq!(p.next().await, Some(Poll::Ready(42)));
assert_eq!(p.next().await, None);
});
```

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

### `select_with_strategy`

```rust
fn select_with_strategy<St1, St2, Clos, State>(stream1: St1, stream2: St2, which: Clos) -> SelectWithStrategy<St1, St2, Clos, State>
where
    St1: Stream,
    St2: Stream<Item = <St1 as >::Item>,
    Clos: FnMut(&mut State) -> PollNext,
    State: Default
```

This function will attempt to pull items from both streams. You provide a
closure to tell [`SelectWithStrategy`](select_with_strategy/index.md) which stream to poll. The closure can
store state on `SelectWithStrategy` to which it will receive a `&mut` on every
invocation. This allows basing the strategy on prior choices.

After one of the two input streams completes, the remaining one will be
polled exclusively. The returned stream completes when both input
streams have completed.

Note that this function consumes both streams and returns a wrapped
version of them.

## Examples

### Priority
This example shows how to always prioritize the left stream.

```rust
futures::executor::block_on(async {
use futures::stream::{ repeat, select_with_strategy, PollNext, StreamExt };

let left = repeat(1);
let right = repeat(2);

// We don't need any state, so let's make it an empty tuple.
// We must provide some type here, as there is no way for the compiler
// to infer it. As we don't need to capture variables, we can just
// use a function pointer instead of a closure.
fn prio_left(_: &mut ()) -> PollNext { PollNext::Left }

let mut out = select_with_strategy(left, right, prio_left);

for _ in 0..100 {
    // Whenever we poll out, we will alwas get `1`.
    assert_eq!(1, out.select_next_some().await);
}
});
```

### Round Robin
This example shows how to select from both streams round robin.
Note: this special case is provided by [`stream::select`](crate::stream::select).

```rust
futures::executor::block_on(async {
use futures::stream::{ repeat, select_with_strategy, PollNext, StreamExt };

let left = repeat(1);
let right = repeat(2);

let rrobin = |last: &mut PollNext| last.toggle();

let mut out = select_with_strategy(left, right, rrobin);

for _ in 0..100 {
    // We should be alternating now.
    assert_eq!(1, out.select_next_some().await);
    assert_eq!(2, out.select_next_some().await);
}
});
```

### `unfold`

```rust
fn unfold<T, F, Fut, Item>(init: T, f: F) -> Unfold<T, F, Fut>
where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = Option<(Item, T)>>
```

Creates a `Stream` from a seed and a closure returning a `Future`.

This function is the dual for the `Stream::fold()` adapter: while
`Stream::fold()` reduces a `Stream` to one single value, `unfold()` creates a
`Stream` from a seed value.

`unfold()` will call the provided closure with the provided seed, then wait
for the returned `Future` to complete with `(a, b)`. It will then yield the
value `a`, and use `b` as the next internal state.

If the closure returns `None` instead of `Some(Future)`, then the `unfold()`
will stop producing items and return `Poll::Ready(None)` in future
calls to `poll()`.

This function can typically be used when wanting to go from the "world of
futures" to the "world of streams": the provided closure can build a
`Future` using other library functions working on futures, and `unfold()`
will turn it into a `Stream` by repeating the operation.

# Example

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::unfold(0, |state| async move {
    if state <= 2 {
        let next_state = state + 1;
        let yielded = state * 2;
        Some((yielded, next_state))
    } else {
        None
    }
});

let result = stream.collect::<Vec<i32>>().await;
assert_eq!(result, vec![0, 2, 4]);
});
```

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

### `abortable`

```rust
fn abortable<St>(stream: St) -> (crate::stream::Abortable<St>, crate::stream::AbortHandle)
where
    St: Stream
```

Creates a new `Abortable` stream and an `AbortHandle` which can be used to stop it.

This function is a convenient (but less flexible) alternative to calling
`AbortHandle::new` and `Abortable::new` manually.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

### `assert_stream`

```rust
fn assert_stream<T, S>(stream: S) -> S
where
    S: Stream<Item = T>
```

## Type Aliases

### `FlattenUnordered<St>`

```rust
type FlattenUnordered<St> = FlattenUnorderedWithFlowController<St, ()>;
```

Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)
method.

