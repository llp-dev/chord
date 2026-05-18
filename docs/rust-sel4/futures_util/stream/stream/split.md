**futures_util > stream > stream > split**

# Module: stream::stream::split

## Contents

**Structs**

- [`ReuniteError`](#reuniteerror) - Error indicating a `SplitSink<S>` and `SplitStream<S>` were not two halves
- [`SplitSink`](#splitsink) - A `Sink` part of the split pair
- [`SplitStream`](#splitstream) - A `Stream` part of the split pair

---

## futures_util::stream::stream::split::ReuniteError

*Struct*

Error indicating a `SplitSink<S>` and `SplitStream<S>` were not two halves
of a `Stream + Split`, and thus could not be `reunite`d.

**Generic Parameters:**
- T
- Item

**Tuple Struct**: `(SplitSink<T, Item>, SplitStream<T>)`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## futures_util::stream::stream::split::SplitSink

*Struct*

A `Sink` part of the split pair

**Generic Parameters:**
- S
- Item

**Methods:**

- `fn reunite(self: Self, other: SplitStream<S>) -> Result<S, ReuniteError<S, Item>>` - Attempts to put the two "halves" of a split `Stream + Sink` back
- `fn is_pair_of(self: &Self, other: &SplitStream<S>) -> bool` - Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`.

**Trait Implementations:**

- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <S as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, item: Item) -> Result<(), <S as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <S as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <S as >::Error>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::stream::split::SplitStream

*Struct*

A `Stream` part of the split pair

**Generic Parameters:**
- S

**Tuple Struct**: `()`

**Methods:**

- `fn is_pair_of<Item>(self: &Self, other: &SplitSink<S, Item>) -> bool` - Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`.
- `fn reunite<Item>(self: Self, other: SplitSink<S, Item>) -> Result<S, ReuniteError<S, Item>>` - Attempts to put the two "halves" of a split `Stream + Sink` back

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<S as >::Item>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



