**futures_util > stream > select_with_strategy**

# Module: stream::select_with_strategy

## Contents

**Structs**

- [`SelectWithStrategy`](#selectwithstrategy) - Stream for the [`select_with_strategy()`] function. See function docs for details.

**Enums**

- [`PollNext`](#pollnext) - Type to tell [`SelectWithStrategy`] which stream to poll next.

**Functions**

- [`select_with_strategy`](#select_with_strategy) - This function will attempt to pull items from both streams. You provide a

---

## futures_util::stream::select_with_strategy::PollNext

*Enum*

Type to tell [`SelectWithStrategy`] which stream to poll next.

**Variants:**
- `Left` - Poll the first stream.
- `Right` - Poll the second stream.

**Methods:**

- `fn toggle(self: & mut Self) -> Self` - Toggle the value and return the old one.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PollNext`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PollNext) -> bool`



## futures_util::stream::select_with_strategy::SelectWithStrategy

*Struct*

Stream for the [`select_with_strategy()`] function. See function docs for details.

**Generic Parameters:**
- St1
- St2
- Clos
- State

**Methods:**

- `fn get_ref(self: &Self) -> (&St1, &St2)` - Acquires a reference to the underlying streams that this combinator is
- `fn get_mut(self: & mut Self) -> (& mut St1, & mut St2)` - Acquires a mutable reference to the underlying streams that this
- `fn get_pin_mut(self: Pin<& mut Self>) -> (Pin<& mut St1>, Pin<& mut St2>)` - Acquires a pinned mutable reference to the underlying streams that this
- `fn into_inner(self: Self) -> (St1, St2)` - Consumes this combinator, returning the underlying streams.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<St1 as >::Item>>`



## futures_util::stream::select_with_strategy::select_with_strategy

*Function*

This function will attempt to pull items from both streams. You provide a
closure to tell [`SelectWithStrategy`] which stream to poll. The closure can
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
# futures::executor::block_on(async {
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
# });
```

### Round Robin
This example shows how to select from both streams round robin.
Note: this special case is provided by [`stream::select`](crate::stream::select).

```rust
# futures::executor::block_on(async {
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
# });
```

```rust
fn select_with_strategy<St1, St2, Clos, State>(stream1: St1, stream2: St2, which: Clos) -> SelectWithStrategy<St1, St2, Clos, State>
```



