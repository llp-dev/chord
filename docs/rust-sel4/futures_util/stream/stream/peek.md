**futures_util > stream > stream > peek**

# Module: stream::stream::peek

## Contents

**Structs**

- [`NextIf`](#nextif) - Future for the [`Peekable::next_if`](self::Peekable::next_if) method.
- [`NextIfEq`](#nextifeq) - Future for the [`Peekable::next_if_eq`](self::Peekable::next_if_eq) method.
- [`Peek`](#peek) - Future for the [`Peekable::peek`](self::Peekable::peek) method.
- [`PeekMut`](#peekmut) - Future for the [`Peekable::peek_mut`](self::Peekable::peek_mut) method.
- [`Peekable`](#peekable) - A `Stream` that implements a `peek` method.

---

## futures_util::stream::stream::peek::NextIf

*Struct*

Future for the [`Peekable::next_if`](self::Peekable::next_if) method.

**Generic Parameters:**
- 'a
- St
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::stream::stream::peek::NextIfEq

*Struct*

Future for the [`Peekable::next_if_eq`](self::Peekable::next_if_eq) method.

**Generic Parameters:**
- 'a
- St
- T

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::stream::stream::peek::Peek

*Struct*

Future for the [`Peekable::peek`](self::Peekable::peek) method.

**Generic Parameters:**
- 'a
- St

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::stream::stream::peek::PeekMut

*Struct*

Future for the [`Peekable::peek_mut`](self::Peekable::peek_mut) method.

**Generic Parameters:**
- 'a
- St

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::stream::stream::peek::Peekable

*Struct*

A `Stream` that implements a `peek` method.

The `peek` method can be used to retrieve a reference
to the next `Stream::Item` if available. A subsequent
call to `poll` will return the owned item.

**Generic Parameters:**
- St

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.
- `fn peek(self: Pin<& mut Self>) -> Peek<St>` - Produces a future which retrieves a reference to the next item
- `fn poll_peek(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<&<St as >::Item>>` - Peek retrieves a reference to the next item in the stream.
- `fn peek_mut(self: Pin<& mut Self>) -> PeekMut<St>` - Produces a future which retrieves a mutable reference to the next item
- `fn poll_peek_mut(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<& mut <St as >::Item>>` - Peek retrieves a mutable reference to the next item in the stream.
- `fn next_if<F>(self: Pin<& mut Self>, func: F) -> NextIf<St, F>` - Creates a future which will consume and return the next value of this
- `fn next_if_eq<'a, T>(self: Pin<&'a  mut Self>, expected: &'a T) -> NextIfEq<'a, St, T>` - Creates a future which will consume and return the next item if it is

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



