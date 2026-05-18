*[futures_util](../../index.md) / [stream](../index.md) / [stream](index.md)*

---

# Module `stream`

Streams

This module contains a number of functions for working with `Stream`s,
including the `StreamExt` trait which adds methods to `Stream` types.

## Contents

- [Modules](#modules)
  - [`chain`](#chain)
  - [`collect`](#collect)
  - [`unzip`](#unzip)
  - [`concat`](#concat)
  - [`count`](#count)
  - [`cycle`](#cycle)
  - [`enumerate`](#enumerate)
  - [`filter`](#filter)
  - [`filter_map`](#filter-map)
  - [`flatten`](#flatten)
  - [`fold`](#fold)
  - [`any`](#any)
  - [`all`](#all)
  - [`forward`](#forward)
  - [`for_each`](#for-each)
  - [`fuse`](#fuse)
  - [`into_future`](#into-future)
  - [`map`](#map)
  - [`next`](#next)
  - [`select_next_some`](#select-next-some)
  - [`peek`](#peek)
  - [`skip`](#skip)
  - [`skip_while`](#skip-while)
  - [`take`](#take)
  - [`take_while`](#take-while)
  - [`take_until`](#take-until)
  - [`then`](#then)
  - [`zip`](#zip)
  - [`chunks`](#chunks)
  - [`ready_chunks`](#ready-chunks)
  - [`scan`](#scan)
  - [`buffer_unordered`](#buffer-unordered)
  - [`buffered`](#buffered)
  - [`flatten_unordered`](#flatten-unordered)
  - [`for_each_concurrent`](#for-each-concurrent)
  - [`split`](#split)
- [Structs](#structs)
  - [`Chain`](#chain)
  - [`Collect`](#collect)
  - [`Unzip`](#unzip)
  - [`Concat`](#concat)
  - [`Count`](#count)
  - [`Cycle`](#cycle)
  - [`Enumerate`](#enumerate)
  - [`Filter`](#filter)
  - [`FilterMap`](#filtermap)
  - [`Flatten`](#flatten)
  - [`Fold`](#fold)
  - [`Any`](#any)
  - [`All`](#all)
  - [`Forward`](#forward)
  - [`ForEach`](#foreach)
  - [`Fuse`](#fuse)
  - [`StreamFuture`](#streamfuture)
  - [`Inspect`](#inspect)
  - [`Map`](#map)
  - [`FlatMap`](#flatmap)
  - [`Next`](#next)
  - [`SelectNextSome`](#selectnextsome)
  - [`NextIf`](#nextif)
  - [`NextIfEq`](#nextifeq)
  - [`Peek`](#peek)
  - [`PeekMut`](#peekmut)
  - [`Peekable`](#peekable)
  - [`Skip`](#skip)
  - [`SkipWhile`](#skipwhile)
  - [`Take`](#take)
  - [`TakeWhile`](#takewhile)
  - [`TakeUntil`](#takeuntil)
  - [`Then`](#then)
  - [`Zip`](#zip)
  - [`Chunks`](#chunks)
  - [`ReadyChunks`](#readychunks)
  - [`Scan`](#scan)
  - [`BufferUnordered`](#bufferunordered)
  - [`Buffered`](#buffered)
  - [`FlatMapUnordered`](#flatmapunordered)
  - [`ForEachConcurrent`](#foreachconcurrent)
  - [`ReuniteError`](#reuniteerror)
  - [`SplitSink`](#splitsink)
  - [`SplitStream`](#splitstream)
- [Traits](#traits)
  - [`StreamExt`](#streamext)
- [Type Aliases](#type-aliases)
  - [`FlattenUnordered`](#flattenunordered)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`chain`](#chain) | mod |  |
| [`collect`](#collect) | mod |  |
| [`unzip`](#unzip) | mod |  |
| [`concat`](#concat) | mod |  |
| [`count`](#count) | mod |  |
| [`cycle`](#cycle) | mod |  |
| [`enumerate`](#enumerate) | mod |  |
| [`filter`](#filter) | mod |  |
| [`filter_map`](#filter-map) | mod |  |
| [`flatten`](#flatten) | mod |  |
| [`fold`](#fold) | mod |  |
| [`any`](#any) | mod |  |
| [`all`](#all) | mod |  |
| [`forward`](#forward) | mod |  |
| [`for_each`](#for-each) | mod |  |
| [`fuse`](#fuse) | mod |  |
| [`into_future`](#into-future) | mod |  |
| [`map`](#map) | mod |  |
| [`next`](#next) | mod |  |
| [`select_next_some`](#select-next-some) | mod |  |
| [`peek`](#peek) | mod |  |
| [`skip`](#skip) | mod |  |
| [`skip_while`](#skip-while) | mod |  |
| [`take`](#take) | mod |  |
| [`take_while`](#take-while) | mod |  |
| [`take_until`](#take-until) | mod |  |
| [`then`](#then) | mod |  |
| [`zip`](#zip) | mod |  |
| [`chunks`](#chunks) | mod |  |
| [`ready_chunks`](#ready-chunks) | mod |  |
| [`scan`](#scan) | mod |  |
| [`buffer_unordered`](#buffer-unordered) | mod |  |
| [`buffered`](#buffered) | mod |  |
| [`flatten_unordered`](#flatten-unordered) | mod |  |
| [`for_each_concurrent`](#for-each-concurrent) | mod |  |
| [`split`](#split) | mod |  |
| [`Chain`](#chain) | struct |  |
| [`Collect`](#collect) | struct |  |
| [`Unzip`](#unzip) | struct |  |
| [`Concat`](#concat) | struct |  |
| [`Count`](#count) | struct |  |
| [`Cycle`](#cycle) | struct |  |
| [`Enumerate`](#enumerate) | struct |  |
| [`Filter`](#filter) | struct |  |
| [`FilterMap`](#filtermap) | struct |  |
| [`Flatten`](#flatten) | struct | Stream for the [`flatten`](StreamExt::flatten) method. |
| [`Fold`](#fold) | struct |  |
| [`Any`](#any) | struct |  |
| [`All`](#all) | struct |  |
| [`Forward`](#forward) | struct | Future for the [`forward`](super::StreamExt::forward) method. |
| [`ForEach`](#foreach) | struct |  |
| [`Fuse`](#fuse) | struct |  |
| [`StreamFuture`](#streamfuture) | struct |  |
| [`Inspect`](#inspect) | struct | Stream for the [`inspect`](StreamExt::inspect) method. |
| [`Map`](#map) | struct |  |
| [`FlatMap`](#flatmap) | struct | Stream for the [`flat_map`](StreamExt::flat_map) method. |
| [`Next`](#next) | struct |  |
| [`SelectNextSome`](#selectnextsome) | struct |  |
| [`NextIf`](#nextif) | struct |  |
| [`NextIfEq`](#nextifeq) | struct |  |
| [`Peek`](#peek) | struct |  |
| [`PeekMut`](#peekmut) | struct |  |
| [`Peekable`](#peekable) | struct |  |
| [`Skip`](#skip) | struct |  |
| [`SkipWhile`](#skipwhile) | struct |  |
| [`Take`](#take) | struct |  |
| [`TakeWhile`](#takewhile) | struct |  |
| [`TakeUntil`](#takeuntil) | struct |  |
| [`Then`](#then) | struct |  |
| [`Zip`](#zip) | struct |  |
| [`Chunks`](#chunks) | struct |  |
| [`ReadyChunks`](#readychunks) | struct |  |
| [`Scan`](#scan) | struct |  |
| [`BufferUnordered`](#bufferunordered) | struct |  |
| [`Buffered`](#buffered) | struct |  |
| [`FlatMapUnordered`](#flatmapunordered) | struct | Stream for the [`flat_map_unordered`](StreamExt::flat_map_unordered) method. |
| [`ForEachConcurrent`](#foreachconcurrent) | struct |  |
| [`ReuniteError`](#reuniteerror) | struct |  |
| [`SplitSink`](#splitsink) | struct |  |
| [`SplitStream`](#splitstream) | struct |  |
| [`StreamExt`](#streamext) | trait | An extension trait for `Stream`s that provides a variety of convenient combinator functions. |
| [`FlattenUnordered`](#flattenunordered) | type |  |

## Modules

- [`chain`](chain/index.md)
- [`collect`](collect/index.md)
- [`unzip`](unzip/index.md)
- [`concat`](concat/index.md)
- [`count`](count/index.md)
- [`cycle`](cycle/index.md)
- [`enumerate`](enumerate/index.md)
- [`filter`](filter/index.md)
- [`filter_map`](filter_map/index.md)
- [`flatten`](flatten/index.md)
- [`fold`](fold/index.md)
- [`any`](any/index.md)
- [`all`](all/index.md)
- [`forward`](forward/index.md)
- [`for_each`](for_each/index.md)
- [`fuse`](fuse/index.md)
- [`into_future`](into_future/index.md)
- [`map`](map/index.md)
- [`next`](next/index.md)
- [`select_next_some`](select_next_some/index.md)
- [`peek`](peek/index.md)
- [`skip`](skip/index.md)
- [`skip_while`](skip_while/index.md)
- [`take`](take/index.md)
- [`take_while`](take_while/index.md)
- [`take_until`](take_until/index.md)
- [`then`](then/index.md)
- [`zip`](zip/index.md)
- [`chunks`](chunks/index.md)
- [`ready_chunks`](ready_chunks/index.md)
- [`scan`](scan/index.md)
- [`buffer_unordered`](buffer_unordered/index.md)
- [`buffered`](buffered/index.md)
- [`flatten_unordered`](flatten_unordered/index.md)
- [`for_each_concurrent`](for_each_concurrent/index.md)
- [`split`](split/index.md)

## Structs

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

- <span id="chain-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="chain-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Chain<St1, St2>`

##### `impl TryStream for Chain<St1, St2>`

- <span id="chain-trystream-type-ok"></span>`type Ok = T`

- <span id="chain-trystream-type-error"></span>`type Error = E`

- <span id="chain-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="collect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<C>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl FutureExt for Collect<St, C>`

##### `impl IntoFuture for Collect<St, C>`

- <span id="collect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="collect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="collect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, C> Unpin for Collect<St, C>`

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

- <span id="unzip-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<(FromA, FromB)>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl FutureExt for Unzip<St, FromA, FromB>`

##### `impl IntoFuture for Unzip<St, FromA, FromB>`

- <span id="unzip-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="unzip-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="unzip-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, FromA, FromB> Unpin for Unzip<St, FromA, FromB>`

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

- <span id="concat-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for Concat<St>`

##### `impl IntoFuture for Concat<St>`

- <span id="concat-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="concat-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="concat-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Concat<St>`

- <span id="concat-tryfuture-type-ok"></span>`type Ok = T`

- <span id="concat-tryfuture-type-error"></span>`type Error = E`

- <span id="concat-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

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

- <span id="count-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

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

- <span id="cycle-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="cycle-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Cycle<St>`

##### `impl TryStream for Cycle<St>`

- <span id="cycle-trystream-type-ok"></span>`type Ok = T`

- <span id="cycle-trystream-type-error"></span>`type Error = E`

- <span id="cycle-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="enumerate-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="enumerate-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="enumerate-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="enumerate-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Enumerate<St>`

##### `impl<St: Stream> Stream for Enumerate<St>`

- <span id="enumerate-stream-type-item"></span>`type Item = (usize, <St as Stream>::Item)`

- <span id="enumerate-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

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

- <span id="filter-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="filter-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="filter-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="filter-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Filter<St, Fut, F>`

##### `impl<St, Fut, F> Stream for Filter<St, Fut, F>`

- <span id="filter-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="filter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="filter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Filter<St, Fut, F>`

##### `impl TryStream for Filter<St, Fut, F>`

- <span id="filter-trystream-type-ok"></span>`type Ok = T`

- <span id="filter-trystream-type-error"></span>`type Error = E`

- <span id="filter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="filtermap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="filtermap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="filtermap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="filtermap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for FilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Stream for FilterMap<St, Fut, F>`

- <span id="filtermap-stream-type-item"></span>`type Item = T`

- <span id="filtermap-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

- <span id="filtermap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FilterMap<St, Fut, F>`

##### `impl TryStream for FilterMap<St, Fut, F>`

- <span id="filtermap-trystream-type-ok"></span>`type Ok = T`

- <span id="filtermap-trystream-type-error"></span>`type Error = E`

- <span id="filtermap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for FilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for FilterMap<St, Fut, F>`

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

- <span id="flatten-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flatten-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="flatten-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flatten-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Flatten<St>`

##### `impl<St> Stream for Flatten<St>`

- <span id="flatten-stream-type-item"></span>`type Item = <Flatten<St, <St as Stream>::Item> as Stream>::Item`

- <span id="flatten-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="flatten-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Flatten<St>`

##### `impl TryStream for Flatten<St>`

- <span id="flatten-trystream-type-ok"></span>`type Ok = T`

- <span id="flatten-trystream-type-error"></span>`type Error = E`

- <span id="flatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="fold-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl<T> FutureExt for Fold<St, Fut, T, F>`

##### `impl<F> IntoFuture for Fold<St, Fut, T, F>`

- <span id="fold-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="fold-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="fold-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for Fold<St, Fut, T, F>`

- <span id="fold-tryfuture-type-ok"></span>`type Ok = T`

- <span id="fold-tryfuture-type-error"></span>`type Error = E`

- <span id="fold-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<Fut> TryFutureExt for Fold<St, Fut, T, F>`

##### `impl<St, Fut, T, F> Unpin for Fold<St, Fut, T, F>`

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

- <span id="any-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl FutureExt for Any<St, Fut, F>`

##### `impl<F> IntoFuture for Any<St, Fut, F>`

- <span id="any-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="any-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="any-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for Any<St, Fut, F>`

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

- <span id="all-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl FutureExt for All<St, Fut, F>`

##### `impl<F> IntoFuture for All<St, Fut, F>`

- <span id="all-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="all-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="all-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for All<St, Fut, F>`

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

- <span id="forward-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for Forward<St, Si>`

##### `impl IntoFuture for Forward<St, Si>`

- <span id="forward-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="forward-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="forward-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Forward<St, Si>`

- <span id="forward-tryfuture-type-ok"></span>`type Ok = T`

- <span id="forward-tryfuture-type-error"></span>`type Error = E`

- <span id="forward-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for Forward<St, Si>`

##### `impl<St, Si> Unpin for Forward<St, Si>`

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

- <span id="foreach-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

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

- <span id="fuse-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="fuse-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="fuse-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="fuse-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Fuse<St>`

##### `impl<S: Stream> Stream for Fuse<S>`

- <span id="fuse-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="fuse-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="fuse-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Fuse<St>`

##### `impl TryStream for Fuse<St>`

- <span id="fuse-trystream-type-ok"></span>`type Ok = T`

- <span id="fuse-trystream-type-error"></span>`type Error = E`

- <span id="fuse-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Fuse<St>`

##### `impl<St> Unpin for Fuse<St>`

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

- <span id="streamfuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for StreamFuture<St>`

##### `impl IntoFuture for StreamFuture<St>`

- <span id="streamfuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="streamfuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="streamfuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

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

- <span id="inspect-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="inspect-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="inspect-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="inspect-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Inspect<St, F>`

##### `impl<St, F> Stream for Inspect<St, F>`

- <span id="inspect-stream-type-item"></span>`type Item = <Map<St, InspectFn<F>> as Stream>::Item`

- <span id="inspect-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="inspect-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Inspect<St, F>`

##### `impl TryStream for Inspect<St, F>`

- <span id="inspect-trystream-type-ok"></span>`type Ok = T`

- <span id="inspect-trystream-type-error"></span>`type Error = E`

- <span id="inspect-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="map-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="map-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="map-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="map-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Map<St, F>`

##### `impl<St, F> Stream for Map<St, F>`

- <span id="map-stream-type-item"></span>`type Item = <F as FnOnce1>::Output`

- <span id="map-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="map-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Map<St, F>`

##### `impl TryStream for Map<St, F>`

- <span id="map-trystream-type-ok"></span>`type Ok = T`

- <span id="map-trystream-type-error"></span>`type Error = E`

- <span id="map-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Map<St, F>`

##### `impl<St, F> Unpin for Map<St, F>`

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

- <span id="flatmap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flatmap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="flatmap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flatmap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for FlatMap<St, U, F>`

##### `impl<St, U, F> Stream for FlatMap<St, U, F>`

- <span id="flatmap-stream-type-item"></span>`type Item = <Flatten<Map<St, F>, U> as Stream>::Item`

- <span id="flatmap-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="flatmap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlatMap<St, U, F>`

##### `impl TryStream for FlatMap<St, U, F>`

- <span id="flatmap-trystream-type-ok"></span>`type Ok = T`

- <span id="flatmap-trystream-type-error"></span>`type Error = E`

- <span id="flatmap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for FlatMap<St, U, F>`

##### `impl<St, U, F> Unpin for FlatMap<St, U, F>`

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

- <span id="next-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for Next<'a, St>`

##### `impl IntoFuture for Next<'a, St>`

- <span id="next-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="next-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="next-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: ?Sized + Unpin> Unpin for Next<'_, St>`

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

- <span id="selectnextsome-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for SelectNextSome<'a, St>`

##### `impl IntoFuture for SelectNextSome<'a, St>`

- <span id="selectnextsome-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="selectnextsome-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="selectnextsome-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for SelectNextSome<'a, St>`

- <span id="selectnextsome-tryfuture-type-ok"></span>`type Ok = T`

- <span id="selectnextsome-tryfuture-type-error"></span>`type Error = E`

- <span id="selectnextsome-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for SelectNextSome<'a, St>`

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

- <span id="nextif-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

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

- <span id="nextifeq-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

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

- <span id="peek-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

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

- <span id="peekmut-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

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

- <span id="peekable-peek"></span>`fn peek(self: Pin<&mut Self>) -> Peek<'_, St>` — [`Peek`](peek/index.md#peek)

  Produces a future which retrieves a reference to the next item

  in the stream, or `None` if the underlying stream terminates.

- <span id="peekable-poll-peek"></span>`fn poll_peek(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&<St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

  Peek retrieves a reference to the next item in the stream.

  

  This method polls the underlying stream and return either a reference

  to the next item if the stream is ready or passes through any errors.

- <span id="peekable-peek-mut"></span>`fn peek_mut(self: Pin<&mut Self>) -> PeekMut<'_, St>` — [`PeekMut`](peek/index.md#peekmut)

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

- <span id="peekable-poll-peek-mut"></span>`fn poll_peek_mut(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&mut <St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

  Peek retrieves a mutable reference to the next item in the stream.

- <span id="peekable-next-if"></span>`fn next_if<F>(self: Pin<&mut Self>, func: F) -> NextIf<'_, St, F>` — [`NextIf`](peek/index.md#nextif)

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

- <span id="peekable-next-if-eq"></span>`fn next_if_eq<'a, T>(self: Pin<&'a mut Self>, expected: &'a T) -> NextIfEq<'a, St, T>` — [`NextIfEq`](peek/index.md#nextifeq)

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

- <span id="peekable-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="peekable-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="peekable-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="peekable-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Peekable<St>`

##### `impl<S: Stream> Stream for Peekable<S>`

- <span id="peekable-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="peekable-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="peekable-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Peekable<St>`

##### `impl TryStream for Peekable<St>`

- <span id="peekable-trystream-type-ok"></span>`type Ok = T`

- <span id="peekable-trystream-type-error"></span>`type Error = E`

- <span id="peekable-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Peekable<St>`

##### `impl<St: Stream> Unpin for Peekable<St>`

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

- <span id="skip-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="skip-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="skip-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="skip-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Skip<St>`

##### `impl<St: Stream> Stream for Skip<St>`

- <span id="skip-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="skip-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="skip-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Skip<St>`

##### `impl TryStream for Skip<St>`

- <span id="skip-trystream-type-ok"></span>`type Ok = T`

- <span id="skip-trystream-type-error"></span>`type Error = E`

- <span id="skip-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="skipwhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="skipwhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="skipwhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="skipwhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for SkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="skipwhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="skipwhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for SkipWhile<St, Fut, F>`

##### `impl TryStream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-trystream-type-ok"></span>`type Ok = T`

- <span id="skipwhile-trystream-type-error"></span>`type Error = E`

- <span id="skipwhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for SkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for SkipWhile<St, Fut, F>`

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

- <span id="take-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="take-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="take-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="take-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Take<St>`

##### `impl<St> Stream for Take<St>`

- <span id="take-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="take-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="take-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Take<St>`

##### `impl TryStream for Take<St>`

- <span id="take-trystream-type-ok"></span>`type Ok = T`

- <span id="take-trystream-type-error"></span>`type Error = E`

- <span id="take-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Take<St>`

##### `impl<St> Unpin for Take<St>`

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

- <span id="takewhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="takewhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="takewhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="takewhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TakeWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TakeWhile<St, Fut, F>`

- <span id="takewhile-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="takewhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="takewhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TakeWhile<St, Fut, F>`

##### `impl TryStream for TakeWhile<St, Fut, F>`

- <span id="takewhile-trystream-type-ok"></span>`type Ok = T`

- <span id="takewhile-trystream-type-error"></span>`type Error = E`

- <span id="takewhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TakeWhile<St, Fut, F>`

##### `impl<St: Stream, Fut, F> Unpin for TakeWhile<St, Fut, F>`

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

- <span id="takeuntil-take-result"></span>`fn take_result(&mut self) -> Option<<Fut as >::Output>` — [`Future`](../../future/index.md#future)

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

- <span id="takeuntil-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="takeuntil-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="takeuntil-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="takeuntil-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TakeUntil<St, Fut>`

##### `impl<St, Fut> Stream for TakeUntil<St, Fut>`

- <span id="takeuntil-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="takeuntil-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="takeuntil-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TakeUntil<St, Fut>`

##### `impl TryStream for TakeUntil<St, Fut>`

- <span id="takeuntil-trystream-type-ok"></span>`type Ok = T`

- <span id="takeuntil-trystream-type-error"></span>`type Error = E`

- <span id="takeuntil-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TakeUntil<St, Fut>`

##### `impl<St: Stream, Fut: Future> Unpin for TakeUntil<St, Fut>`

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

- <span id="then-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="then-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="then-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="then-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Then<St, Fut, F>`

##### `impl<St, Fut, F> Stream for Then<St, Fut, F>`

- <span id="then-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="then-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="then-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Then<St, Fut, F>`

##### `impl TryStream for Then<St, Fut, F>`

- <span id="then-trystream-type-ok"></span>`type Ok = T`

- <span id="then-trystream-type-error"></span>`type Error = E`

- <span id="then-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for Then<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for Then<St, Fut, F>`

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

- <span id="zip-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

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

- <span id="chunks-take"></span>`fn take(self: Pin<&mut Self>) -> Vec<<St as >::Item>` — [`Stream`](../index.md#stream)

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

- <span id="chunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="chunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="chunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="chunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Chunks<St>`

##### `impl<St: Stream> Stream for Chunks<St>`

- <span id="chunks-stream-type-item"></span>`type Item = Vec<<St as Stream>::Item>`

- <span id="chunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

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

- <span id="readychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="readychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="readychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="readychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for ReadyChunks<St>`

##### `impl<St: Stream> Stream for ReadyChunks<St>`

- <span id="readychunks-stream-type-item"></span>`type Item = Vec<<St as Stream>::Item>`

- <span id="readychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="readychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for ReadyChunks<St>`

##### `impl<St: Stream> Unpin for ReadyChunks<St>`

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

- <span id="scan-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="scan-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="scan-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="scan-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Scan<St, S, Fut, F>`

##### `impl<St, S, Fut, F> Stream for Scan<St, S, Fut, F>`

- <span id="scan-stream-type-item"></span>`type Item = B`

- <span id="scan-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<B>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

- <span id="scan-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Scan<St, S, Fut, F>`

##### `impl<S> TryStream for Scan<St, S, Fut, F>`

- <span id="scan-trystream-type-ok"></span>`type Ok = T`

- <span id="scan-trystream-type-error"></span>`type Error = E`

- <span id="scan-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl<S> TryStreamExt for Scan<St, S, Fut, F>`

##### `impl<St: Stream, S, Fut, F> Unpin for Scan<St, S, Fut, F>`

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

- <span id="bufferunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="bufferunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="bufferunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="bufferunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for BufferUnordered<St>`

##### `impl<St> Stream for BufferUnordered<St>`

- <span id="bufferunordered-stream-type-item"></span>`type Item = <<St as Stream>::Item as Future>::Output`

- <span id="bufferunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="bufferunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for BufferUnordered<St>`

##### `impl TryStream for BufferUnordered<St>`

- <span id="bufferunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="bufferunordered-trystream-type-error"></span>`type Error = E`

- <span id="bufferunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="buffered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="buffered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="buffered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="buffered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for Buffered<St>`

##### `impl<St> Stream for Buffered<St>`

- <span id="buffered-stream-type-item"></span>`type Item = <<St as Stream>::Item as Future>::Output`

- <span id="buffered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="buffered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Buffered<St>`

##### `impl TryStream for Buffered<St>`

- <span id="buffered-trystream-type-ok"></span>`type Ok = T`

- <span id="buffered-trystream-type-error"></span>`type Error = E`

- <span id="buffered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="flatmapunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flatmapunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="flatmapunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flatmapunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for FlatMapUnordered<St, U, F>`

##### `impl<St, U, F> Stream for FlatMapUnordered<St, U, F>`

- <span id="flatmapunordered-stream-type-item"></span>`type Item = <FlattenUnorderedWithFlowController<Map<St, F>, ()> as Stream>::Item`

- <span id="flatmapunordered-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="flatmapunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlatMapUnordered<St, U, F>`

##### `impl TryStream for FlatMapUnordered<St, U, F>`

- <span id="flatmapunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="flatmapunordered-trystream-type-error"></span>`type Error = E`

- <span id="flatmapunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

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

- <span id="foreachconcurrent-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

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

- <span id="splitsink-reunite"></span>`fn reunite(self, other: SplitStream<S>) -> Result<S, ReuniteError<S, Item>>` — [`SplitStream`](split/index.md#splitstream), [`ReuniteError`](split/index.md#reuniteerror)

  Attempts to put the two "halves" of a split `Stream + Sink` back

  together. Succeeds only if the `SplitStream<S>` and `SplitSink<S>` are

  a matching pair originating from the same call to `StreamExt::split`.

#### Trait Implementations

##### `impl<S: fmt::Debug, Item: fmt::Debug> Debug for SplitSink<S, Item>`

- <span id="splitsink-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Sink<Item>, Item> Sink for SplitSink<S, Item>`

- <span id="splitsink-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="splitsink-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="splitsink-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <S as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="splitsink-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="splitsink-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for SplitSink<S, Item>`

##### `impl<S, Item> Unpin for SplitSink<S, Item>`

### `SplitStream<S>`

```rust
struct SplitStream<S>(self::bilock::BiLock<S>);
```

A `Stream` part of the split pair

#### Implementations

- <span id="splitstream-is-pair-of"></span>`fn is_pair_of<Item>(&self, other: &SplitSink<S, Item>) -> bool` — [`SplitSink`](split/index.md#splitsink)

  Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`.

#### Trait Implementations

##### `impl<S: fmt::Debug> Debug for SplitStream<S>`

- <span id="splitstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Stream> Stream for SplitStream<S>`

- <span id="splitstream-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="splitstream-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl StreamExt for SplitStream<S>`

##### `impl<S> TryStream for SplitStream<S>`

- <span id="splitstream-trystream-type-ok"></span>`type Ok = T`

- <span id="splitstream-trystream-type-error"></span>`type Error = E`

- <span id="splitstream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl<S> TryStreamExt for SplitStream<S>`

##### `impl<S> Unpin for SplitStream<S>`

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

  Returns a [`Future`](../../future/index.md) that resolves when the next item in this stream is

#### Implementors

- `T`

## Type Aliases

### `FlattenUnordered<St>`

```rust
type FlattenUnordered<St> = FlattenUnorderedWithFlowController<St, ()>;
```

Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)
method.

