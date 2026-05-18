**futures_util > sink > fanout**

# Module: sink::fanout

## Contents

**Structs**

- [`Fanout`](#fanout) - Sink that clones incoming items and forwards them to two sinks at the same time.

---

## futures_util::sink::fanout::Fanout

*Struct*

Sink that clones incoming items and forwards them to two sinks at the same time.

Backpressure from any downstream sink propagates up, which means that this sink
can only process items as fast as its _slowest_ downstream sink.

**Generic Parameters:**
- Si1
- Si2

**Methods:**

- `fn get_ref(self: &Self) -> (&Si1, &Si2)` - Get a shared reference to the inner sinks.
- `fn get_mut(self: & mut Self) -> (& mut Si1, & mut Si2)` - Get a mutable reference to the inner sinks.
- `fn get_pin_mut(self: Pin<& mut Self>) -> (Pin<& mut Si1>, Pin<& mut Si2>)` - Get a pinned mutable reference to the inner sinks.
- `fn into_inner(self: Self) -> (Si1, Si2)` - Consumes this combinator, returning the underlying sinks.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> FmtResult`
- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`



