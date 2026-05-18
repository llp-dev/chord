**futures_util > future > future**

# Module: future::future

## Contents

**Structs**

- [`Flatten`](#flatten) - Future for the [`flatten`](super::FutureExt::flatten) method.
- [`FlattenStream`](#flattenstream) - Stream for the [`flatten_stream`](FutureExt::flatten_stream) method.
- [`Inspect`](#inspect) - Future for the [`inspect`](FutureExt::inspect) method.
- [`IntoStream`](#intostream) - Stream for the [`into_stream`](FutureExt::into_stream) method.
- [`Map`](#map) - Future for the [`map`](super::FutureExt::map) method.
- [`MapInto`](#mapinto) - Future for the [`map_into`](FutureExt::map_into) combinator.
- [`NeverError`](#nevererror) - Future for the [`never_error`](super::FutureExt::never_error) combinator.
- [`Then`](#then) - Future for the [`then`](FutureExt::then) method.
- [`UnitError`](#uniterror) - Future for the [`unit_error`](super::FutureExt::unit_error) combinator.

**Traits**

- [`FutureExt`](#futureext) - An extension trait for `Future`s that provides a variety of convenient

---

## futures_util::future::future::Flatten

*Struct*

Future for the [`flatten`](super::FutureExt::flatten) method.

**Generic Parameters:**
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::future::FlattenStream

*Struct*

Stream for the [`flatten_stream`](FutureExt::flatten_stream) method.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::future::future::FutureExt

*Trait*

An extension trait for `Future`s that provides a variety of convenient
adapters.

**Methods:**

- `map`: Map this future's output to a different type, returning a new future of
- `map_into`: Map this future's output to a different type, returning a new future of
- `then`: Chain on a computation for when a future finished, passing the result of
- `left_future`: Wrap this future in an `Either` future, making it the left-hand variant
- `right_future`: Wrap this future in an `Either` future, making it the right-hand variant
- `into_stream`: Convert this future into a single element stream.
- `flatten`: Flatten the execution of this future when the output of this
- `flatten_stream`: Flatten the execution of this future when the successful result of this
- `fuse`: Fuse a future such that `poll` will never again be called once it has
- `inspect`: Do something with the output of a future before passing it on.
- `boxed`: Wrap the future in a Box, pinning it.
- `boxed_local`: Wrap the future in a Box, pinning it.
- `unit_error`: Turns a [`Future<Output = T>`](Future) into a
- `never_error`: Turns a [`Future<Output = T>`](Future) into a
- `poll_unpin`: A convenience for calling `Future::poll` on `Unpin` future types.
- `now_or_never`: Evaluates and consumes the future, returning the resulting output if



## futures_util::future::future::Inspect

*Struct*

Future for the [`inspect`](FutureExt::inspect) method.

**Generic Parameters:**
- Fut
- F

**Trait Implementations:**

- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::future::IntoStream

*Struct*

Stream for the [`into_stream`](FutureExt::into_stream) method.

**Generic Parameters:**
- F

**Trait Implementations:**

- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::future::Map

*Struct*

Future for the [`map`](super::FutureExt::map) method.

**Generic Parameters:**
- Fut
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::future::MapInto

*Struct*

Future for the [`map_into`](FutureExt::map_into) combinator.

**Generic Parameters:**
- Fut
- T

**Trait Implementations:**

- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::future::NeverError

*Struct*

Future for the [`never_error`](super::FutureExt::never_error) combinator.

**Generic Parameters:**
- Fut

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::future::Then

*Struct*

Future for the [`then`](FutureExt::then) method.

**Generic Parameters:**
- Fut1
- Fut2
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::future::UnitError

*Struct*

Future for the [`unit_error`](super::FutureExt::unit_error) combinator.

**Generic Parameters:**
- Fut

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



