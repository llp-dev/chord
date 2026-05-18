# futures_util

Combinators and utilities for working with `Future`s, `Stream`s, `Sink`s,
and the `AsyncRead` and `AsyncWrite` traits.

## Modules

### [`futures_util`](futures_util.md)

*6 macros, 6 modules*

### [`abortable`](abortable.md)

*4 structs*

### [`fns`](fns.md)

*10 structs, 2 traits*

### [`future::abortable`](future/abortable.md)

*1 function*

### [`future::always_ready`](future/always_ready.md)

*1 function, 1 struct*

### [`future::either`](future/either.md)

*1 enum*

### [`future::future`](future/future.md)

*1 trait, 9 structs*

### [`future::future::flatten`](future/future/flatten.md)

*1 enum*

### [`future::future::fuse`](future/future/fuse.md)

*1 struct*

### [`future::future::map`](future/future/map.md)

*1 enum*

### [`future::join`](future/join.md)

*4 functions, 4 structs*

### [`future::join_all`](future/join_all.md)

*1 function, 1 struct*

### [`future::lazy`](future/lazy.md)

*1 function, 1 struct*

### [`future::maybe_done`](future/maybe_done.md)

*1 enum, 1 function*

### [`future::option`](future/option.md)

*1 struct*

### [`future::pending`](future/pending.md)

*1 function, 1 struct*

### [`future::poll_fn`](future/poll_fn.md)

*1 function, 1 struct*

### [`future::poll_immediate`](future/poll_immediate.md)

*1 function, 1 struct*

### [`future::ready`](future/ready.md)

*1 struct, 3 functions*

### [`future::select`](future/select.md)

*1 function, 1 struct*

### [`future::select_all`](future/select_all.md)

*1 function, 1 struct*

### [`future::select_ok`](future/select_ok.md)

*1 function, 1 struct*

### [`future::try_future`](future/try_future.md)

*1 trait, 14 structs*

### [`future::try_future::into_future`](future/try_future/into_future.md)

*1 struct*

### [`future::try_future::try_flatten`](future/try_future/try_flatten.md)

*1 enum*

### [`future::try_future::try_flatten_err`](future/try_future/try_flatten_err.md)

*1 enum*

### [`future::try_join`](future/try_join.md)

*4 functions, 4 structs*

### [`future::try_join_all`](future/try_join_all.md)

*1 function, 1 struct*

### [`future::try_maybe_done`](future/try_maybe_done.md)

*1 enum, 1 function*

### [`future::try_select`](future/try_select.md)

*1 function, 1 struct*

### [`never`](never.md)

*1 type alias*

### [`sink`](sink.md)

*1 trait*

### [`sink::buffer`](sink/buffer.md)

*1 struct*

### [`sink::close`](sink/close.md)

*1 struct*

### [`sink::drain`](sink/drain.md)

*1 function, 1 struct*

### [`sink::err_into`](sink/err_into.md)

*1 struct*

### [`sink::fanout`](sink/fanout.md)

*1 struct*

### [`sink::feed`](sink/feed.md)

*1 struct*

### [`sink::flush`](sink/flush.md)

*1 struct*

### [`sink::map_err`](sink/map_err.md)

*1 struct*

### [`sink::send`](sink/send.md)

*1 struct*

### [`sink::send_all`](sink/send_all.md)

*1 struct*

### [`sink::unfold`](sink/unfold.md)

*1 function, 1 struct*

### [`sink::with`](sink/with.md)

*1 struct*

### [`sink::with_flat_map`](sink/with_flat_map.md)

*1 struct*

### [`stream`](stream.md)

*2 modules*

### [`stream::abortable`](stream/abortable.md)

*1 function*

### [`stream::empty`](stream/empty.md)

*1 function, 1 struct*

### [`stream::futures_ordered`](stream/futures_ordered.md)

*1 struct*

### [`stream::futures_unordered`](stream/futures_unordered.md)

*1 struct*

### [`stream::futures_unordered::iter`](stream/futures_unordered/iter.md)

*5 structs*

### [`stream::iter`](stream/iter.md)

*1 function, 1 struct*

### [`stream::once`](stream/once.md)

*1 function, 1 struct*

### [`stream::pending`](stream/pending.md)

*1 function, 1 struct*

### [`stream::poll_fn`](stream/poll_fn.md)

*1 function, 1 struct*

### [`stream::poll_immediate`](stream/poll_immediate.md)

*1 function, 1 struct*

### [`stream::repeat`](stream/repeat.md)

*1 function, 1 struct*

### [`stream::repeat_with`](stream/repeat_with.md)

*1 function, 1 struct*

### [`stream::select`](stream/select.md)

*1 function, 1 struct*

### [`stream::select_all`](stream/select_all.md)

*1 function, 4 structs*

### [`stream::select_with_strategy`](stream/select_with_strategy.md)

*1 enum, 1 function, 1 struct*

### [`stream::stream`](stream/stream.md)

*1 trait, 5 structs*

### [`stream::stream::all`](stream/stream/all.md)

*1 struct*

### [`stream::stream::any`](stream/stream/any.md)

*1 struct*

### [`stream::stream::buffer_unordered`](stream/stream/buffer_unordered.md)

*1 struct*

### [`stream::stream::buffered`](stream/stream/buffered.md)

*1 struct*

### [`stream::stream::chain`](stream/stream/chain.md)

*1 struct*

### [`stream::stream::chunks`](stream/stream/chunks.md)

*1 struct*

### [`stream::stream::collect`](stream/stream/collect.md)

*1 struct*

### [`stream::stream::concat`](stream/stream/concat.md)

*1 struct*

### [`stream::stream::count`](stream/stream/count.md)

*1 struct*

### [`stream::stream::cycle`](stream/stream/cycle.md)

*1 struct*

### [`stream::stream::enumerate`](stream/stream/enumerate.md)

*1 struct*

### [`stream::stream::filter`](stream/stream/filter.md)

*1 struct*

### [`stream::stream::filter_map`](stream/stream/filter_map.md)

*1 struct*

### [`stream::stream::flatten`](stream/stream/flatten.md)

*1 struct*

### [`stream::stream::flatten_unordered`](stream/stream/flatten_unordered.md)

*1 enum, 1 struct, 1 trait, 1 type alias*

### [`stream::stream::fold`](stream/stream/fold.md)

*1 struct*

### [`stream::stream::for_each`](stream/stream/for_each.md)

*1 struct*

### [`stream::stream::for_each_concurrent`](stream/stream/for_each_concurrent.md)

*1 struct*

### [`stream::stream::forward`](stream/stream/forward.md)

*1 struct*

### [`stream::stream::fuse`](stream/stream/fuse.md)

*1 struct*

### [`stream::stream::into_future`](stream/stream/into_future.md)

*1 struct*

### [`stream::stream::map`](stream/stream/map.md)

*1 struct*

### [`stream::stream::next`](stream/stream/next.md)

*1 struct*

### [`stream::stream::peek`](stream/stream/peek.md)

*5 structs*

### [`stream::stream::ready_chunks`](stream/stream/ready_chunks.md)

*1 struct*

### [`stream::stream::scan`](stream/stream/scan.md)

*1 struct*

### [`stream::stream::select_next_some`](stream/stream/select_next_some.md)

*1 struct*

### [`stream::stream::skip`](stream/stream/skip.md)

*1 struct*

### [`stream::stream::skip_while`](stream/stream/skip_while.md)

*1 struct*

### [`stream::stream::split`](stream/stream/split.md)

*3 structs*

### [`stream::stream::take`](stream/stream/take.md)

*1 struct*

### [`stream::stream::take_until`](stream/stream/take_until.md)

*1 struct*

### [`stream::stream::take_while`](stream/stream/take_while.md)

*1 struct*

### [`stream::stream::then`](stream/stream/then.md)

*1 struct*

### [`stream::stream::unzip`](stream/stream/unzip.md)

*1 struct*

### [`stream::stream::zip`](stream/stream/zip.md)

*1 struct*

### [`stream::try_stream`](stream/try_stream.md)

*1 trait, 5 structs*

### [`stream::try_stream::and_then`](stream/try_stream/and_then.md)

*1 struct*

### [`stream::try_stream::into_stream`](stream/try_stream/into_stream.md)

*1 struct*

### [`stream::try_stream::or_else`](stream/try_stream/or_else.md)

*1 struct*

### [`stream::try_stream::try_all`](stream/try_stream/try_all.md)

*1 struct*

### [`stream::try_stream::try_any`](stream/try_stream/try_any.md)

*1 struct*

### [`stream::try_stream::try_buffer_unordered`](stream/try_stream/try_buffer_unordered.md)

*1 struct*

### [`stream::try_stream::try_buffered`](stream/try_stream/try_buffered.md)

*1 struct*

### [`stream::try_stream::try_chunks`](stream/try_stream/try_chunks.md)

*2 structs*

### [`stream::try_stream::try_collect`](stream/try_stream/try_collect.md)

*1 struct*

### [`stream::try_stream::try_concat`](stream/try_stream/try_concat.md)

*1 struct*

### [`stream::try_stream::try_filter`](stream/try_stream/try_filter.md)

*1 struct*

### [`stream::try_stream::try_filter_map`](stream/try_stream/try_filter_map.md)

*1 struct*

### [`stream::try_stream::try_flatten`](stream/try_stream/try_flatten.md)

*1 struct*

### [`stream::try_stream::try_flatten_unordered`](stream/try_stream/try_flatten_unordered.md)

*4 structs*

### [`stream::try_stream::try_fold`](stream/try_stream/try_fold.md)

*1 struct*

### [`stream::try_stream::try_for_each`](stream/try_stream/try_for_each.md)

*1 struct*

### [`stream::try_stream::try_for_each_concurrent`](stream/try_stream/try_for_each_concurrent.md)

*1 struct*

### [`stream::try_stream::try_next`](stream/try_stream/try_next.md)

*1 struct*

### [`stream::try_stream::try_ready_chunks`](stream/try_stream/try_ready_chunks.md)

*2 structs*

### [`stream::try_stream::try_skip_while`](stream/try_stream/try_skip_while.md)

*1 struct*

### [`stream::try_stream::try_take_while`](stream/try_stream/try_take_while.md)

*1 struct*

### [`stream::try_stream::try_unfold`](stream/try_stream/try_unfold.md)

*1 function, 1 struct*

### [`stream::unfold`](stream/unfold.md)

*1 function, 1 struct*

### [`task::spawn`](task/spawn.md)

*2 traits*

