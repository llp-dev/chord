**futures_util > future > try_future**

# Module: future::try_future

## Contents

**Structs**

- [`AndThen`](#andthen) - Future for the [`and_then`](TryFutureExt::and_then) method.
- [`ErrInto`](#errinto) - Future for the [`err_into`](TryFutureExt::err_into) method.
- [`FlattenSink`](#flattensink) - Sink for the [`flatten_sink`](TryFutureExt::flatten_sink) method.
- [`InspectErr`](#inspecterr) - Future for the [`inspect_err`](super::TryFutureExt::inspect_err) method.
- [`InspectOk`](#inspectok) - Future for the [`inspect_ok`](super::TryFutureExt::inspect_ok) method.
- [`MapErr`](#maperr) - Future for the [`map_err`](TryFutureExt::map_err) method.
- [`MapOk`](#mapok) - Future for the [`map_ok`](TryFutureExt::map_ok) method.
- [`MapOkOrElse`](#mapokorelse) - Future for the [`map_ok_or_else`](TryFutureExt::map_ok_or_else) method.
- [`OkInto`](#okinto) - Future for the [`ok_into`](TryFutureExt::ok_into) method.
- [`OrElse`](#orelse) - Future for the [`or_else`](TryFutureExt::or_else) method.
- [`TryFlatten`](#tryflatten) - Future for the [`try_flatten`](TryFutureExt::try_flatten) method.
- [`TryFlattenErr`](#tryflattenerr) - Future for the [`try_flatten_err`](TryFutureExt::try_flatten_err) method.
- [`TryFlattenStream`](#tryflattenstream) - Future for the [`try_flatten_stream`](TryFutureExt::try_flatten_stream) method.
- [`UnwrapOrElse`](#unwraporelse) - Future for the [`unwrap_or_else`](TryFutureExt::unwrap_or_else) method.

**Traits**

- [`TryFutureExt`](#tryfutureext) - Adapters specific to [`Result`]-returning futures

---

## futures_util::future::try_future::AndThen

*Struct*

Future for the [`and_then`](TryFutureExt::and_then) method.

**Generic Parameters:**
- Fut1
- Fut2
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::try_future::ErrInto

*Struct*

Future for the [`err_into`](TryFutureExt::err_into) method.

**Generic Parameters:**
- Fut
- E

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::try_future::FlattenSink

*Struct*

Sink for the [`flatten_sink`](TryFutureExt::flatten_sink) method.

**Generic Parameters:**
- Fut
- Si

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::try_future::InspectErr

*Struct*

Future for the [`inspect_err`](super::TryFutureExt::inspect_err) method.

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



## futures_util::future::try_future::InspectOk

*Struct*

Future for the [`inspect_ok`](super::TryFutureExt::inspect_ok) method.

**Generic Parameters:**
- Fut
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::try_future::MapErr

*Struct*

Future for the [`map_err`](TryFutureExt::map_err) method.

**Generic Parameters:**
- Fut
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::try_future::MapOk

*Struct*

Future for the [`map_ok`](TryFutureExt::map_ok) method.

**Generic Parameters:**
- Fut
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::try_future::MapOkOrElse

*Struct*

Future for the [`map_ok_or_else`](TryFutureExt::map_ok_or_else) method.

**Generic Parameters:**
- Fut
- F
- G

**Trait Implementations:**

- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::future::try_future::OkInto

*Struct*

Future for the [`ok_into`](TryFutureExt::ok_into) method.

**Generic Parameters:**
- Fut
- E

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::try_future::OrElse

*Struct*

Future for the [`or_else`](TryFutureExt::or_else) method.

**Generic Parameters:**
- Fut1
- Fut2
- F

**Trait Implementations:**

- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::future::try_future::TryFlatten

*Struct*

Future for the [`try_flatten`](TryFutureExt::try_flatten) method.

**Generic Parameters:**
- Fut1
- Fut2

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::future::try_future::TryFlattenErr

*Struct*

Future for the [`try_flatten_err`](TryFutureExt::try_flatten_err) method.

**Generic Parameters:**
- Fut1
- Fut2



## futures_util::future::try_future::TryFlattenStream

*Struct*

Future for the [`try_flatten_stream`](TryFutureExt::try_flatten_stream) method.

**Generic Parameters:**
- Fut

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::future::try_future::TryFutureExt

*Trait*

Adapters specific to [`Result`]-returning futures

**Methods:**

- `flatten_sink`: Flattens the execution of this future when the successful result of this
- `map_ok`: Maps this future's success value to a different value.
- `map_ok_or_else`: Maps this future's success value to a different value, and permits for error handling resulting in the same type.
- `map_err`: Maps this future's error value to a different value.
- `err_into`: Maps this future's [`Error`](TryFuture::Error) to a new error type
- `ok_into`: Maps this future's [`Ok`](TryFuture::Ok) to a new type
- `and_then`: Executes another future after this one resolves successfully. The
- `or_else`: Executes another future if this one resolves to an error. The
- `inspect_ok`: Do something with the success value of a future before passing it on.
- `inspect_err`: Do something with the error value of a future before passing it on.
- `try_flatten`: Flatten the execution of this future when the successful result of this
- `try_flatten_stream`: Flatten the execution of this future when the successful result of this
- `unwrap_or_else`: Unwraps this future's output, producing a future with this future's
- `into_future`: Wraps a [`TryFuture`] into a type that implements
- `try_poll_unpin`: A convenience method for calling [`TryFuture::try_poll`] on [`Unpin`]



## futures_util::future::try_future::UnwrapOrElse

*Struct*

Future for the [`unwrap_or_else`](TryFutureExt::unwrap_or_else) method.

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



