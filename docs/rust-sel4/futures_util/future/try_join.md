**futures_util > future > try_join**

# Module: future::try_join

## Contents

**Structs**

- [`TryJoin`](#tryjoin) - Future for the [`try_join`](try_join()) function.
- [`TryJoin3`](#tryjoin3) - Future for the [`try_join3`] function.
- [`TryJoin4`](#tryjoin4) - Future for the [`try_join4`] function.
- [`TryJoin5`](#tryjoin5) - Future for the [`try_join5`] function.

**Functions**

- [`try_join`](#try_join) - Joins the result of two futures, waiting for them both to complete or
- [`try_join3`](#try_join3) - Same as [`try_join`](try_join()), but with more futures.
- [`try_join4`](#try_join4) - Same as [`try_join`](try_join()), but with more futures.
- [`try_join5`](#try_join5) - Same as [`try_join`](try_join()), but with more futures.

---

## futures_util::future::try_join::TryJoin

*Struct*

Future for the [`try_join`](try_join()) function.

**Generic Parameters:**
- Fut1
- Fut2

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::future::try_join::TryJoin3

*Struct*

Future for the [`try_join3`] function.

**Generic Parameters:**
- Fut1
- Fut2
- Fut3

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::future::try_join::TryJoin4

*Struct*

Future for the [`try_join4`] function.

**Generic Parameters:**
- Fut1
- Fut2
- Fut3
- Fut4

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## futures_util::future::try_join::TryJoin5

*Struct*

Future for the [`try_join5`] function.

**Generic Parameters:**
- Fut1
- Fut2
- Fut3
- Fut4
- Fut5

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::future::try_join::try_join

*Function*

Joins the result of two futures, waiting for them both to complete or
for one to produce an error.

This function will return a new future which awaits both futures to
complete. If successful, the returned future will finish with a tuple of
both results. If unsuccessful, it will complete with the first error
encountered.

Note that this function consumes the passed futures and returns a
wrapped version of it.

# Examples

When used on multiple futures that return [`Ok`], `try_join` will return
[`Ok`] of a tuple of the values:

```
# futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let pair = future::try_join(a, b);

assert_eq!(pair.await, Ok((1, 2)));
# });
```

If one of the futures resolves to an error, `try_join` will return
that error:

```
# futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Err::<i32, i32>(2));
let pair = future::try_join(a, b);

assert_eq!(pair.await, Err(2));
# });
```

```rust
fn try_join<Fut1, Fut2>(future1: Fut1, future2: Fut2) -> TryJoin<Fut1, Fut2>
```



## futures_util::future::try_join::try_join3

*Function*

Same as [`try_join`](try_join()), but with more futures.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let tuple = future::try_join3(a, b, c);

assert_eq!(tuple.await, Ok((1, 2, 3)));
# });
```

```rust
fn try_join3<Fut1, Fut2, Fut3>(future1: Fut1, future2: Fut2, future3: Fut3) -> TryJoin3<Fut1, Fut2, Fut3>
```



## futures_util::future::try_join::try_join4

*Function*

Same as [`try_join`](try_join()), but with more futures.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let d = future::ready(Ok::<i32, i32>(4));
let tuple = future::try_join4(a, b, c, d);

assert_eq!(tuple.await, Ok((1, 2, 3, 4)));
# });
```

```rust
fn try_join4<Fut1, Fut2, Fut3, Fut4>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4) -> TryJoin4<Fut1, Fut2, Fut3, Fut4>
```



## futures_util::future::try_join::try_join5

*Function*

Same as [`try_join`](try_join()), but with more futures.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let d = future::ready(Ok::<i32, i32>(4));
let e = future::ready(Ok::<i32, i32>(5));
let tuple = future::try_join5(a, b, c, d, e);

assert_eq!(tuple.await, Ok((1, 2, 3, 4, 5)));
# });
```

```rust
fn try_join5<Fut1, Fut2, Fut3, Fut4, Fut5>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4, future5: Fut5) -> TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>
```



