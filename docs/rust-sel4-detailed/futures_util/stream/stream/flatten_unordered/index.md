*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [flatten_unordered](index.md)*

---

# Module `flatten_unordered`

## Contents

- [Structs](#structs)
  - [`SharedPollState`](#sharedpollstate)
  - [`PollStateBomb`](#pollstatebomb)
  - [`WrappedWaker`](#wrappedwaker)
  - [`PollStreamFut`](#pollstreamfut)
  - [`FlattenUnorderedWithFlowController`](#flattenunorderedwithflowcontroller)
- [Enums](#enums)
  - [`FlowStep`](#flowstep)
- [Traits](#traits)
  - [`FlowController`](#flowcontroller)
- [Type Aliases](#type-aliases)
  - [`FlattenUnordered`](#flattenunordered)
- [Constants](#constants)
  - [`NONE`](#none)
  - [`NEED_TO_POLL_INNER_STREAMS`](#need-to-poll-inner-streams)
  - [`NEED_TO_POLL_STREAM`](#need-to-poll-stream)
  - [`NEED_TO_POLL_ALL`](#need-to-poll-all)
  - [`POLLING`](#polling)
  - [`WAKING`](#waking)
  - [`WOKEN`](#woken)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SharedPollState`](#sharedpollstate) | struct | Internal polling state of the stream. |
| [`PollStateBomb`](#pollstatebomb) | struct | Used to execute some function on the given state when dropped. |
| [`WrappedWaker`](#wrappedwaker) | struct | Will update state with the provided value on `wake_by_ref` call and then, if there is a need, call `inner_waker`. |
| [`PollStreamFut`](#pollstreamfut) | struct | Future which polls optional inner stream. |
| [`FlattenUnorderedWithFlowController`](#flattenunorderedwithflowcontroller) | struct | Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered) method with ability to specify flow controller. |
| [`FlowStep`](#flowstep) | enum | Describes the next flow step. |
| [`FlowController`](#flowcontroller) | trait | Returns the next flow step based on the received item. |
| [`FlattenUnordered`](#flattenunordered) | type | Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered) method. |
| [`NONE`](#none) | const | There is nothing to poll and stream isn't being polled/waking/woken at the moment. |
| [`NEED_TO_POLL_INNER_STREAMS`](#need-to-poll-inner-streams) | const | Inner streams need to be polled. |
| [`NEED_TO_POLL_STREAM`](#need-to-poll-stream) | const | The base stream needs to be polled. |
| [`NEED_TO_POLL_ALL`](#need-to-poll-all) | const | Both base stream and inner streams need to be polled. |
| [`POLLING`](#polling) | const | The current stream is being polled at the moment. |
| [`WAKING`](#waking) | const | Stream is being woken at the moment. |
| [`WOKEN`](#woken) | const | The stream was waked and will be polled. |

## Structs

### `SharedPollState`

```rust
struct SharedPollState {
    state: alloc::sync::Arc<core::sync::atomic::AtomicU8>,
}
```

Internal polling state of the stream.

#### Implementations

- <span id="sharedpollstate-new"></span>`fn new(value: u8) -> Self`

  Constructs new `SharedPollState` with the given state.

- <span id="sharedpollstate-start-polling"></span>`fn start_polling(&self) -> Option<(u8, PollStateBomb<'_, impl FnOnce(&Self) -> u8>)>` — [`PollStateBomb`](#pollstatebomb)

  Attempts to start polling, returning stored state in case of success.

  Returns `None` if either waker is waking at the moment.

- <span id="sharedpollstate-start-waking"></span>`fn start_waking(&self, to_poll: u8) -> Option<(u8, PollStateBomb<'_, impl FnOnce(&Self) -> u8>)>` — [`PollStateBomb`](#pollstatebomb)

  Attempts to start the waking process and performs bitwise or with the given value.

  

  If some waker is already in progress or stream is already woken/being polled, waking process won't start, however

  state will be disjuncted with the given value.

- <span id="sharedpollstate-stop-polling"></span>`fn stop_polling(&self, to_poll: u8, will_be_woken: bool) -> u8`

  Sets current state to

  - `!POLLING` allowing to use wakers

  - `WOKEN` if the state was changed during `POLLING` phase as waker will be called,

    or `will_be_woken` flag supplied

  - `!WAKING` as

    * Wakers called during the `POLLING` phase won't propagate their calls

    * `POLLING` phase can't start if some of the wakers are active

      So no wrapped waker can touch the inner waker's cell, it's safe to poll again.

- <span id="sharedpollstate-stop-waking"></span>`fn stop_waking(&self) -> u8`

  Toggles state to non-waking, allowing to start polling.

- <span id="sharedpollstate-reset"></span>`fn reset(&self) -> u8`

  Resets current state allowing to poll the stream and wake up wakers.

#### Trait Implementations

##### `impl Clone for SharedPollState`

- <span id="sharedpollstate-clone"></span>`fn clone(&self) -> SharedPollState` — [`SharedPollState`](#sharedpollstate)

##### `impl Debug for SharedPollState`

- <span id="sharedpollstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PollStateBomb<'a, F: FnOnce(&SharedPollState) -> u8>`

```rust
struct PollStateBomb<'a, F: FnOnce(&SharedPollState) -> u8> {
    state: &'a SharedPollState,
    drop: Option<F>,
}
```

Used to execute some function on the given state when dropped.

#### Implementations

- <span id="pollstatebomb-new"></span>`fn new(state: &'a SharedPollState, drop: F) -> Self` — [`SharedPollState`](#sharedpollstate)

  Constructs new bomb with the given state.

- <span id="pollstatebomb-deactivate"></span>`fn deactivate(self)`

  Deactivates bomb, forces it to not call provided function when dropped.

#### Trait Implementations

##### `impl<F: FnOnce(&SharedPollState) -> u8> Drop for PollStateBomb<'_, F>`

- <span id="pollstatebomb-drop"></span>`fn drop(&mut self)`

### `WrappedWaker`

```rust
struct WrappedWaker {
    inner_waker: core::cell::UnsafeCell<Option<futures_core::task::Waker>>,
    poll_state: SharedPollState,
    need_to_poll: u8,
}
```

Will update state with the provided value on `wake_by_ref` call
and then, if there is a need, call `inner_waker`.

#### Implementations

- <span id="wrappedwaker-replace-waker"></span>`unsafe fn replace_waker(self_arc: &mut Arc<Self>, cx: &Context<'_>)` — [`Context`](../../../task/index.md#context)

  Replaces given waker's inner_waker for polling stream/futures which will

  update poll state on `wake_by_ref` call. Use only if you need several

  contexts.

  

  ## Safety

  

  This function will modify waker's `inner_waker` via `UnsafeCell`, so

  it should be used only during `POLLING` phase by one thread at the time.

- <span id="wrappedwaker-start-waking"></span>`fn start_waking(&self) -> Option<(u8, PollStateBomb<'_, impl FnOnce(&SharedPollState) -> u8>)>` — [`PollStateBomb`](#pollstatebomb), [`SharedPollState`](#sharedpollstate)

  Attempts to start the waking process for the waker with the given value.

  If succeeded, then the stream isn't yet woken and not being polled at the moment.

#### Trait Implementations

##### `impl ArcWake for WrappedWaker`

- <span id="wrappedwaker-arcwake-wake-by-ref"></span>`fn wake_by_ref(self_arc: &Arc<Self>)`

##### `impl Send for WrappedWaker`

##### `impl Sync for WrappedWaker`

### `PollStreamFut<St>`

```rust
struct PollStreamFut<St> {
    stream: Option<St>,
}
```

Future which polls optional inner stream.

If it's `Some`, it will attempt to call `poll_next` on it,
returning `Some((item, next_item_fut))` in case of `Poll::Ready(Some(...))`
or `None` in case of `Poll::Ready(None)`.

If `poll_next` will return `Poll::Pending`, it will be forwarded to
the future and current task will be notified by waker.

#### Implementations

- <span id="pollstreamfut-new"></span>`fn new(stream: impl Into<Option<St>>) -> Self`

  Constructs new `PollStreamFut` using given `stream`.

#### Trait Implementations

##### `impl<St: Stream + Unpin> Future for PollStreamFut<St>`

- <span id="pollstreamfut-future-type-output"></span>`type Output = Option<(<St as Stream>::Item, PollStreamFut<St>)>`

- <span id="pollstreamfut-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for PollStreamFut<St>`

##### `impl IntoFuture for PollStreamFut<St>`

- <span id="pollstreamfut-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="pollstreamfut-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="pollstreamfut-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St> Unpin for PollStreamFut<St>`

### `FlattenUnorderedWithFlowController<St, Fc>`

```rust
struct FlattenUnorderedWithFlowController<St, Fc>
where
    St: Stream {
    inner_streams: crate::stream::FuturesUnordered<PollStreamFut<<St as >::Item>>,
    stream: St,
    poll_state: SharedPollState,
    limit: Option<core::num::NonZeroUsize>,
    is_stream_done: bool,
    inner_streams_waker: alloc::sync::Arc<WrappedWaker>,
    stream_waker: alloc::sync::Arc<WrappedWaker>,
    flow_controller: core::marker::PhantomData<Fc>,
}
```

Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)
method with ability to specify flow controller.

#### Implementations

- <span id="flattenunorderedwithflowcontroller-new"></span>`fn new(stream: St, limit: Option<usize>) -> Self`

- <span id="flattenunorderedwithflowcontroller-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="flattenunorderedwithflowcontroller-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="flattenunorderedwithflowcontroller-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="flattenunorderedwithflowcontroller-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fc> Debug for FlattenUnorderedWithFlowController<St, Fc>`

- <span id="flattenunorderedwithflowcontroller-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fc> FusedStream for FlattenUnorderedWithFlowController<St, Fc>`

- <span id="flattenunorderedwithflowcontroller-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Item, Fc> Sink for FlattenUnorderedWithFlowController<St, Fc>`

- <span id="flattenunorderedwithflowcontroller-sink-type-error"></span>`type Error = <St as Sink>::Error`

- <span id="flattenunorderedwithflowcontroller-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="flattenunorderedwithflowcontroller-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="flattenunorderedwithflowcontroller-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="flattenunorderedwithflowcontroller-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for FlattenUnorderedWithFlowController<St, Fc>`

##### `impl<St, Fc> Stream for FlattenUnorderedWithFlowController<St, Fc>`

- <span id="flattenunorderedwithflowcontroller-stream-type-item"></span>`type Item = <<St as Stream>::Item as Stream>::Item`

- <span id="flattenunorderedwithflowcontroller-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

##### `impl StreamExt for FlattenUnorderedWithFlowController<St, Fc>`

##### `impl TryStream for FlattenUnorderedWithFlowController<St, Fc>`

- <span id="flattenunorderedwithflowcontroller-trystream-type-ok"></span>`type Ok = T`

- <span id="flattenunorderedwithflowcontroller-trystream-type-error"></span>`type Error = E`

- <span id="flattenunorderedwithflowcontroller-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for FlattenUnorderedWithFlowController<St, Fc>`

##### `impl<St, Fc> Unpin for FlattenUnorderedWithFlowController<St, Fc>`

## Enums

### `FlowStep<C, R>`

```rust
enum FlowStep<C, R> {
    Continue(C),
    Return(R),
}
```

Describes the next flow step.

#### Variants

- **`Continue`**

  Just yields an item and continues standard flow.

- **`Return`**

  Immediately returns an underlying item from the function.

#### Trait Implementations

##### `impl<C: clone::Clone, R: clone::Clone> Clone for FlowStep<C, R>`

- <span id="flowstep-clone"></span>`fn clone(&self) -> FlowStep<C, R>` — [`FlowStep`](#flowstep)

##### `impl<C: fmt::Debug, R: fmt::Debug> Debug for FlowStep<C, R>`

- <span id="flowstep-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `FlowController<I, O>`

```rust
trait FlowController<I, O> { ... }
```

Returns the next flow step based on the received item.

#### Required Methods

- `fn next_step(item: I) -> FlowStep<I, O>`

  Handles an item producing `FlowStep` describing the next flow step.

#### Implementors

- [`PropagateBaseStreamError`](../../try_stream/try_flatten_unordered/index.md#propagatebasestreamerror)
- `()`

## Type Aliases

### `FlattenUnordered<St>`

```rust
type FlattenUnordered<St> = FlattenUnorderedWithFlowController<St, ()>;
```

Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)
method.

## Constants

### `NONE`
```rust
const NONE: u8 = 0u8;
```

There is nothing to poll and stream isn't being polled/waking/woken at the moment.

### `NEED_TO_POLL_INNER_STREAMS`
```rust
const NEED_TO_POLL_INNER_STREAMS: u8 = 1u8;
```

Inner streams need to be polled.

### `NEED_TO_POLL_STREAM`
```rust
const NEED_TO_POLL_STREAM: u8 = 2u8;
```

The base stream needs to be polled.

### `NEED_TO_POLL_ALL`
```rust
const NEED_TO_POLL_ALL: u8 = 3u8;
```

Both base stream and inner streams need to be polled.

### `POLLING`
```rust
const POLLING: u8 = 4u8;
```

The current stream is being polled at the moment.

### `WAKING`
```rust
const WAKING: u8 = 8u8;
```

Stream is being woken at the moment.

### `WOKEN`
```rust
const WOKEN: u8 = 16u8;
```

The stream was waked and will be polled.

