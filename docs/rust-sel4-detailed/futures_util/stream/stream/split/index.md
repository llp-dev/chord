*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [split](index.md)*

---

# Module `split`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SplitStream`](#splitstream) | struct | A `Stream` part of the split pair |
| [`SplitSink`](#splitsink) | struct | A `Sink` part of the split pair |
| [`ReuniteError`](#reuniteerror) | struct | Error indicating a `SplitSink<S>` and `SplitStream<S>` were not two halves of a `Stream + Split`, and thus could not be `reunite`d. |
| [`SplitSink`](#splitsink) | fn |  |
| [`split`](#split) | fn |  |

## Structs

### `SplitStream<S>`

```rust
struct SplitStream<S>(self::bilock::BiLock<S>);
```

A `Stream` part of the split pair

#### Implementations

- <span id="splitstream-is-pair-of"></span>`fn is_pair_of<Item>(&self, other: &SplitSink<S, Item>) -> bool` — [`SplitSink`](#splitsink)

  Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`.

#### Trait Implementations

##### `impl<S: fmt::Debug> Debug for SplitStream<S>`

- <span id="splitstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Stream> Stream for SplitStream<S>`

- <span id="splitstream-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="splitstream-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

##### `impl StreamExt for SplitStream<S>`

##### `impl<S> TryStream for SplitStream<S>`

- <span id="splitstream-trystream-type-ok"></span>`type Ok = T`

- <span id="splitstream-trystream-type-error"></span>`type Error = E`

- <span id="splitstream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl<S> TryStreamExt for SplitStream<S>`

##### `impl<S> Unpin for SplitStream<S>`

### `SplitSink<S, Item>`

```rust
struct SplitSink<S, Item> {
    lock: self::bilock::BiLock<S>,
    slot: Option<Item>,
}
```

A `Sink` part of the split pair

#### Implementations

- <span id="splitsink-reunite"></span>`fn reunite(self, other: SplitStream<S>) -> Result<S, ReuniteError<S, Item>>` — [`SplitStream`](#splitstream), [`ReuniteError`](#reuniteerror)

  Attempts to put the two "halves" of a split `Stream + Sink` back

  together. Succeeds only if the `SplitStream<S>` and `SplitSink<S>` are

  a matching pair originating from the same call to `StreamExt::split`.

#### Trait Implementations

##### `impl<S: fmt::Debug, Item: fmt::Debug> Debug for SplitSink<S, Item>`

- <span id="splitsink-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Sink<Item>, Item> Sink for SplitSink<S, Item>`

- <span id="splitsink-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="splitsink-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="splitsink-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <S as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="splitsink-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="splitsink-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <S as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for SplitSink<S, Item>`

##### `impl<S, Item> Unpin for SplitSink<S, Item>`

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

## Functions

### `SplitSink`

```rust
fn SplitSink<S: Sink<Item>, Item>(lock: self::bilock::BiLock<S>) -> SplitSink<S, Item>
```

### `split`

```rust
fn split<S: Stream + Sink<Item>, Item>(s: S) -> (SplitSink<S, Item>, SplitStream<S>)
```

