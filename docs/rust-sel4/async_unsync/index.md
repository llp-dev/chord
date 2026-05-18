# async_unsync

**async-unsync** - Asynchronous channels for single-threaded use.

This crate provides asynchronous but unsynchronized (`!Sync`) alternatives
to [`tokio::sync::mpsc`][1] channel types with almost identical APIs.

Using synchronized data-structures in context that are statically known to
always execute on a single thread has non-trivial overhead.
The specialized (and much simpler) implementations in this library are
primarily intended for use in high-performance async code utilizing thread
local tasks and `!Send` futures.

[1]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html

## Modules

### [`async_unsync`](async_unsync.md)

*2 modules*

### [`error`](error.md)

*1 struct, 2 enums*

### [`oneshot`](oneshot.md)

*1 function, 4 structs*

### [`semaphore`](semaphore.md)

*1 enum, 4 structs*

