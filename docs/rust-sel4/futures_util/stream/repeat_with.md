**futures_util > stream > repeat_with**

# Module: stream::repeat_with

## Contents

**Structs**

- [`RepeatWith`](#repeatwith) - An stream that repeats elements of type `A` endlessly by

**Functions**

- [`repeat_with`](#repeat_with) - Creates a new stream that repeats elements of type `A` endlessly by

---

## futures_util::stream::repeat_with::RepeatWith

*Struct*

An stream that repeats elements of type `A` endlessly by
applying the provided closure `F: FnMut() -> A`.

This `struct` is created by the [`repeat_with()`] function.
See its documentation for more.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RepeatWith<F>`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, _: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## futures_util::stream::repeat_with::repeat_with

*Function*

Creates a new stream that repeats elements of type `A` endlessly by
applying the provided closure, the repeater, `F: FnMut() -> A`.

The `repeat_with()` function calls the repeater over and over again.

Infinite stream like `repeat_with()` are often used with adapters like
[`stream.take()`](crate::stream::StreamExt::take), in order to make them finite.

If the element type of the stream you need implements [`Clone`], and
it is OK to keep the source element in memory, you should instead use
the [`stream::repeat()`](crate::stream::repeat) function.

# Examples

Basic usage:

```
# futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

// let's assume we have some value of a type that is not `Clone`
// or which don't want to have in memory just yet because it is expensive:
#[derive(PartialEq, Debug)]
struct Expensive;

// a particular value forever:
let mut things = stream::repeat_with(|| Expensive);

assert_eq!(Some(Expensive), things.next().await);
assert_eq!(Some(Expensive), things.next().await);
assert_eq!(Some(Expensive), things.next().await);
# });
```

Using mutation and going finite:

```rust
# futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

// From the zeroth to the third power of two:
let mut curr = 1;
let mut pow2 = stream::repeat_with(|| { let tmp = curr; curr *= 2; tmp })
                    .take(4);

assert_eq!(Some(1), pow2.next().await);
assert_eq!(Some(2), pow2.next().await);
assert_eq!(Some(4), pow2.next().await);
assert_eq!(Some(8), pow2.next().await);

// ... and now we're done
assert_eq!(None, pow2.next().await);
# });
```

```rust
fn repeat_with<A, F>(repeater: F) -> RepeatWith<F>
```



