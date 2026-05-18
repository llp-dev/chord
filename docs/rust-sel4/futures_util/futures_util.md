**futures_util**

# Module: futures_util

## Contents

**Modules**

- [`future`](#future) - Asynchronous values.
- [`lock`](#lock) - Futures-powered synchronization primitives.
- [`never`](#never) - This module contains the `Never` type.
- [`sink`](#sink) - Asynchronous sinks.
- [`stream`](#stream) - Asynchronous streams.
- [`task`](#task) - Tools for working with tasks.

**Macros**

- [`join`](#join) - Polls multiple futures simultaneously, returning a tuple
- [`pending`](#pending) - A macro which yields to the event loop once.
- [`pin_mut`](#pin_mut) - Pins a value on the stack.
- [`poll`](#poll) - A macro which returns the result of polling a future once within the
- [`select_biased`](#select_biased) - Polls multiple futures and streams simultaneously, executing the branch
- [`try_join`](#try_join) - Polls multiple futures simultaneously, resolving to a [`Result`] containing

---

## Module: future

Asynchronous values.

This module contains:

- The [`Future`] trait.
- The [`FutureExt`] and [`TryFutureExt`] trait, which provides adapters for
  chaining and composing futures.
- Top-level future combinators like [`lazy`](lazy()) which creates a future
  from a closure that defines its return value, and [`ready`](ready()),
  which constructs a future with an immediate defined value.



## futures_util::join

*Declarative Macro*

Polls multiple futures simultaneously, returning a tuple
of all results once complete.

While `join!(a, b)` is similar to `(a.await, b.await)`,
`join!` polls both futures concurrently and therefore is more efficient.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

```
# futures::executor::block_on(async {
use futures::join;

let a = async { 1 };
let b = async { 2 };
assert_eq!(join!(a, b), (1, 2));

// `join!` is variadic, so you can pass any number of futures
let c = async { 3 };
let d = async { 4 };
let e = async { 5 };
assert_eq!(join!(c, d, e), (3, 4, 5));
# });
```

```rust
macro_rules! join {
    ($($tokens:tt)*) => { ... };
}
```



## Module: lock

Futures-powered synchronization primitives.

This module is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.



## Module: never

This module contains the `Never` type.

Values of this type can never be created and will never exist.



## futures_util::pending

*Declarative Macro*

A macro which yields to the event loop once.

This is equivalent to returning [`Poll::Pending`](futures_core::task::Poll)
from a [`Future::poll`](futures_core::future::Future::poll) implementation.
Similarly, when using this macro, it must be ensured that [`wake`](std::task::Waker::wake)
is called somewhere when further progress can be made.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

```rust
macro_rules! pending {
    () => { ... };
}
```



## futures_util::pin_mut

*Declarative Macro*

Pins a value on the stack.

Can safely pin values that are not `Unpin` by taking ownership.

**Note:** Since Rust 1.68, this macro is soft-deprecated in favor of
[`pin!`](https://doc.rust-lang.org/std/pin/macro.pin.html) macro
in the standard library.

# Example

```rust
# use futures_util::pin_mut;
# use core::pin::Pin;
# struct Foo {}
let foo = Foo { /* ... */ };
pin_mut!(foo);
let _: Pin<&mut Foo> = foo;
```

```rust
macro_rules! pin_mut {
    ($($x:ident),* $(,)?) => { ... };
}
```



## futures_util::poll

*Declarative Macro*

A macro which returns the result of polling a future once within the
current `async` context.

This macro is only usable inside of `async` functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

If you need the result of polling a [`Stream`](crate::stream::Stream),
you can use this macro with the [`next`](crate::stream::StreamExt::next) method:
`poll!(stream.next())`.

```rust
macro_rules! poll {
    ($x:expr $(,)?) => { ... };
}
```



## futures_util::select_biased

*Declarative Macro*

Polls multiple futures and streams simultaneously, executing the branch
for the future that finishes first. Unlike [`select!`], if multiple futures are ready,
one will be selected in order of declaration. Futures directly
passed to `select_biased!` must be `Unpin` and implement `FusedFuture`.

If an expression which yields a `Future` is passed to `select_biased!`
(e.g. an `async fn` call) instead of a `Future` by name the `Unpin`
requirement is relaxed, since the macro will pin the resulting `Future`
on the stack. However the `Future` returned by the expression must
still implement `FusedFuture`.

Futures and streams which are not already fused can be fused using the
`.fuse()` method. Note, though, that fusing a future or stream directly
in the call to `select_biased!` will not be enough to prevent it from being
polled after completion if the `select_biased!` call is in a loop, so when
`select_biased!`ing in a loop, users should take care to `fuse()` outside of
the loop.

`select_biased!` can be used as an expression and will return the return
value of the selected branch. For this reason the return type of every
branch in a `select_biased!` must be the same.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

```
# futures::executor::block_on(async {
use futures::future;
use futures::select_biased;
let mut a = future::ready(4);
let mut b = future::pending::<()>();

let res = select_biased! {
    a_res = a => a_res + 1,
    _ = b => 0,
};
assert_eq!(res, 5);
# });
```

```
# futures::executor::block_on(async {
use futures::future;
use futures::stream::{self, StreamExt};
use futures::select_biased;
let mut st = stream::iter(vec![2]).fuse();
let mut fut = future::pending::<()>();

select_biased! {
    x = st.next() => assert_eq!(Some(2), x),
    _ = fut => panic!(),
}
# });
```

As described earlier, `select_biased` can directly select on expressions
which return `Future`s - even if those do not implement `Unpin`:

```
# futures::executor::block_on(async {
use futures::future::FutureExt;
use futures::select_biased;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let res = select_biased! {
    a_res = async_identity_fn(62).fuse() => a_res + 1,
    b_res = async_identity_fn(13).fuse() => b_res,
};
assert_eq!(res, 63);
# });
```

If a similar async function is called outside of `select_biased` to produce
a `Future`, the `Future` must be pinned in order to be able to pass
it to `select_biased`. This can be achieved via `Box::pin` for pinning a
`Future` on the heap or the `pin!` macro for pinning a `Future`
on the stack.

```
# futures::executor::block_on(async {
use core::pin::pin;

use futures::future::FutureExt;
use futures::select_biased;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let fut_1 = async_identity_fn(1).fuse();
let fut_2 = async_identity_fn(2).fuse();
let mut fut_1 = Box::pin(fut_1); // Pins the Future on the heap
let mut fut_2 = pin!(fut_2); // Pins the Future on the stack

let res = select_biased! {
    a_res = fut_1 => a_res,
    b_res = fut_2 => b_res,
};
assert!(res == 1 || res == 2);
# });
```

`select_biased` also accepts a `complete` branch and a `default` branch.
`complete` will run if all futures and streams have already been
exhausted. `default` will run if no futures or streams are
immediately ready. `complete` takes priority over `default` in
the case where all futures have completed.
A motivating use-case for passing `Future`s by name as well as for
`complete` blocks is to call `select_biased!` in a loop, which is
demonstrated in the following example:

```
# futures::executor::block_on(async {
use futures::future;
use futures::select_biased;
let mut a_fut = future::ready(4);
let mut b_fut = future::ready(6);
let mut total = 0;

loop {
    select_biased! {
        a = a_fut => total += a,
        b = b_fut => total += b,
        complete => break,
        default => panic!(), // never runs (futures run first, then complete)
    }
}
assert_eq!(total, 10);
# });
```

Note that the futures that have been matched over can still be mutated
from inside the `select_biased!` block's branches. This can be used to implement
more complex behavior such as timer resets or writing into the head of
a stream.

[`select!`]: macro.select.html

```rust
macro_rules! select_biased {
    ($($tokens:tt)*) => { ... };
}
```



## Module: sink

Asynchronous sinks.

This module contains:

- The [`Sink`] trait, which allows you to asynchronously write data.
- The [`SinkExt`] trait, which provides adapters for chaining and composing
  sinks.



## Module: stream

Asynchronous streams.

This module contains:

- The [`Stream`] trait, for objects that can asynchronously produce a
  sequence of values.
- The [`StreamExt`] and [`TryStreamExt`] trait, which provides adapters for
  chaining and composing streams.
- Top-level stream constructors like [`iter`](iter()) which creates a
  stream from an iterator.



## Module: task

Tools for working with tasks.

This module contains:

- [`Spawn`], a trait for spawning new tasks.
- [`Context`], a context of an asynchronous task,
  including a handle for waking up the task.
- [`Waker`], a handle for waking up a task.

The remaining types and traits in the module are used for implementing
executors or dealing with synchronization issues around task wakeup.



## futures_util::try_join

*Declarative Macro*

Polls multiple futures simultaneously, resolving to a [`Result`] containing
either a tuple of the successful outputs or an error.

`try_join!` is similar to [`join!`], but completes immediately if any of
the futures return an error.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

When used on multiple futures that return `Ok`, `try_join!` will return
`Ok` of a tuple of the values:

```
# futures::executor::block_on(async {
use futures::try_join;

let a = async { Ok::<i32, i32>(1) };
let b = async { Ok::<i32, i32>(2) };
assert_eq!(try_join!(a, b), Ok((1, 2)));

// `try_join!` is variadic, so you can pass any number of futures
let c = async { Ok::<i32, i32>(3) };
let d = async { Ok::<i32, i32>(4) };
let e = async { Ok::<i32, i32>(5) };
assert_eq!(try_join!(c, d, e), Ok((3, 4, 5)));
# });
```

If one of the futures resolves to an error, `try_join!` will return
that error:

```
# futures::executor::block_on(async {
use futures::try_join;

let a = async { Ok::<i32, i32>(1) };
let b = async { Err::<u64, i32>(2) };

assert_eq!(try_join!(a, b), Err(2));
# });
```

```rust
macro_rules! try_join {
    ($($tokens:tt)*) => { ... };
}
```



