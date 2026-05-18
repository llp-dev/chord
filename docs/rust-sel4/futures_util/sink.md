**futures_util > sink**

# Module: sink

## Contents

**Traits**

- [`SinkExt`](#sinkext) - An extension trait for `Sink`s that provides a variety of convenient

---

## futures_util::sink::SinkExt

*Trait*

An extension trait for `Sink`s that provides a variety of convenient
combinator functions.

**Methods:**

- `with`: Composes a function *in front of* the sink.
- `with_flat_map`: Composes a function *in front of* the sink.
- `sink_map_err`: Transforms the error returned by the sink.
- `sink_err_into`: Map this sink's error to a different error type using the `Into` trait.
- `buffer`: Adds a fixed-size buffer to the current sink.
- `close`: Close the sink.
- `fanout`: Fanout items to multiple sinks.
- `flush`: Flush the sink, processing all pending items.
- `send`: A future that completes after the given item has been fully processed
- `feed`: A future that completes after the given item has been received
- `send_all`: A future that completes after the given stream has been fully processed
- `left_sink`: Wrap this sink in an `Either` sink, making it the left-hand variant
- `right_sink`: Wrap this stream in an `Either` stream, making it the right-hand variant
- `poll_ready_unpin`: A convenience method for calling [`Sink::poll_ready`] on [`Unpin`]
- `start_send_unpin`: A convenience method for calling [`Sink::start_send`] on [`Unpin`]
- `poll_flush_unpin`: A convenience method for calling [`Sink::poll_flush`] on [`Unpin`]
- `poll_close_unpin`: A convenience method for calling [`Sink::poll_close`] on [`Unpin`]



