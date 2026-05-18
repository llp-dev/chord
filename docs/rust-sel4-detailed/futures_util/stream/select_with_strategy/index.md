*[futures_util](../../index.md) / [stream](../index.md) / [select_with_strategy](index.md)*

---

# Module `select_with_strategy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SelectWithStrategy`](#selectwithstrategy) | struct | Stream for the [`select_with_strategy()`] function. |
| [`PollNext`](#pollnext) | enum | Type to tell [`SelectWithStrategy`] which stream to poll next. |
| [`InternalState`](#internalstate) | enum |  |
| [`select_with_strategy`](#select-with-strategy) | fn | This function will attempt to pull items from both streams. |
| [`poll_side`](#poll-side) | fn |  |
| [`poll_inner`](#poll-inner) | fn |  |

## Structs

### `SelectWithStrategy<St1, St2, Clos, State>`

```rust
struct SelectWithStrategy<St1, St2, Clos, State> {
    stream1: St1,
    stream2: St2,
    internal_state: InternalState,
    state: State,
    clos: Clos,
}
```

Stream for the [`select_with_strategy()`](#select-with-strategy) function. See function docs for details.

#### Implementations

- <span id="selectwithstrategy-get-ref"></span>`fn get_ref(&self) -> (&St1, &St2)`

  Acquires a reference to the underlying streams that this combinator is

  pulling from.

- <span id="selectwithstrategy-get-mut"></span>`fn get_mut(&mut self) -> (&mut St1, &mut St2)`

  Acquires a mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="selectwithstrategy-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut St1>, Pin<&mut St2>)`

  Acquires a pinned mutable reference to the underlying streams that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

- <span id="selectwithstrategy-into-inner"></span>`fn into_inner(self) -> (St1, St2)`

  Consumes this combinator, returning the underlying streams.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St1, St2, Clos, State> Debug for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St1, St2, Clos, State> FusedStream for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St1, St2, Clos, State> Stream for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-stream-type-item"></span>`type Item = <St1 as Stream>::Item`

- <span id="selectwithstrategy-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St1 as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl StreamExt for SelectWithStrategy<St1, St2, Clos, State>`

##### `impl TryStream for SelectWithStrategy<St1, St2, Clos, State>`

- <span id="selectwithstrategy-trystream-type-ok"></span>`type Ok = T`

- <span id="selectwithstrategy-trystream-type-error"></span>`type Error = E`

- <span id="selectwithstrategy-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for SelectWithStrategy<St1, St2, Clos, State>`

##### `impl<St1, St2, Clos, State> Unpin for SelectWithStrategy<St1, St2, Clos, State>`

## Enums

### `PollNext`

```rust
enum PollNext {
    Left,
    Right,
}
```

Type to tell [`SelectWithStrategy`](#selectwithstrategy) which stream to poll next.

#### Variants

- **`Left`**

  Poll the first stream.

- **`Right`**

  Poll the second stream.

#### Implementations

- <span id="pollnext-toggle"></span>`fn toggle(&mut self) -> Self`

  Toggle the value and return the old one.

- <span id="pollnext-other"></span>`fn other(&self) -> Self`

#### Trait Implementations

##### `impl Clone for PollNext`

- <span id="pollnext-clone"></span>`fn clone(&self) -> PollNext` — [`PollNext`](#pollnext)

##### `impl Copy for PollNext`

##### `impl Debug for PollNext`

- <span id="pollnext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PollNext`

- <span id="pollnext-default"></span>`fn default() -> Self`

##### `impl Eq for PollNext`

##### `impl Hash for PollNext`

- <span id="pollnext-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for PollNext`

- <span id="pollnext-partialeq-eq"></span>`fn eq(&self, other: &PollNext) -> bool` — [`PollNext`](#pollnext)

##### `impl StructuralPartialEq for PollNext`

### `InternalState`

```rust
enum InternalState {
    Start,
    LeftFinished,
    RightFinished,
    BothFinished,
}
```

#### Implementations

- <span id="internalstate-finish"></span>`fn finish(&mut self, ps: PollNext)` — [`PollNext`](#pollnext)

## Functions

### `select_with_strategy`

```rust
fn select_with_strategy<St1, St2, Clos, State>(stream1: St1, stream2: St2, which: Clos) -> SelectWithStrategy<St1, St2, Clos, State>
where
    St1: Stream,
    St2: Stream<Item = <St1 as >::Item>,
    Clos: FnMut(&mut State) -> PollNext,
    State: Default
```

This function will attempt to pull items from both streams. You provide a
closure to tell [`SelectWithStrategy`](#selectwithstrategy) which stream to poll. The closure can
store state on `SelectWithStrategy` to which it will receive a `&mut` on every
invocation. This allows basing the strategy on prior choices.

After one of the two input streams completes, the remaining one will be
polled exclusively. The returned stream completes when both input
streams have completed.

Note that this function consumes both streams and returns a wrapped
version of them.

## Examples

### Priority
This example shows how to always prioritize the left stream.

```rust
futures::executor::block_on(async {
use futures::stream::{ repeat, select_with_strategy, PollNext, StreamExt };

let left = repeat(1);
let right = repeat(2);

// We don't need any state, so let's make it an empty tuple.
// We must provide some type here, as there is no way for the compiler
// to infer it. As we don't need to capture variables, we can just
// use a function pointer instead of a closure.
fn prio_left(_: &mut ()) -> PollNext { PollNext::Left }

let mut out = select_with_strategy(left, right, prio_left);

for _ in 0..100 {
    // Whenever we poll out, we will alwas get `1`.
    assert_eq!(1, out.select_next_some().await);
}
});
```

### Round Robin
This example shows how to select from both streams round robin.
Note: this special case is provided by [`stream::select`](crate::stream::select).

```rust
futures::executor::block_on(async {
use futures::stream::{ repeat, select_with_strategy, PollNext, StreamExt };

let left = repeat(1);
let right = repeat(2);

let rrobin = |last: &mut PollNext| last.toggle();

let mut out = select_with_strategy(left, right, rrobin);

for _ in 0..100 {
    // We should be alternating now.
    assert_eq!(1, out.select_next_some().await);
    assert_eq!(2, out.select_next_some().await);
}
});
```

### `poll_side`

```rust
fn poll_side<St1, St2, Clos, State>(select: &mut SelectWithStrategyProj<'_, St1, St2, Clos, State>, side: PollNext, cx: &mut futures_core::task::Context<'_>) -> futures_core::task::Poll<Option<<St1 as >::Item>>
where
    St1: Stream,
    St2: Stream<Item = <St1 as >::Item>
```

### `poll_inner`

```rust
fn poll_inner<St1, St2, Clos, State>(select: &mut SelectWithStrategyProj<'_, St1, St2, Clos, State>, side: PollNext, cx: &mut futures_core::task::Context<'_>) -> futures_core::task::Poll<Option<<St1 as >::Item>>
where
    St1: Stream,
    St2: Stream<Item = <St1 as >::Item>
```

