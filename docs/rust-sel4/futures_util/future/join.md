**futures_util > future > join**

# Module: future::join

## Contents

**Structs**

- [`Join`](#join) - Future for the [`join`](join()) function.
- [`Join3`](#join3) - Future for the [`join3`] function.
- [`Join4`](#join4) - Future for the [`join4`] function.
- [`Join5`](#join5) - Future for the [`join5`] function.

**Functions**

- [`join`](#join) - Joins the result of two futures, waiting for them both to complete.
- [`join3`](#join3) - Same as [`join`](join()), but with more futures.
- [`join4`](#join4) - Same as [`join`](join()), but with more futures.
- [`join5`](#join5) - Same as [`join`](join()), but with more futures.

---

## futures_util::future::join::Join

*Struct*

Future for the [`join`](join()) function.

**Generic Parameters:**
- Fut1
- Fut2

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::future::join::Join3

*Struct*

Future for the [`join3`] function.

**Generic Parameters:**
- Fut1
- Fut2
- Fut3

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::future::join::Join4

*Struct*

Future for the [`join4`] function.

**Generic Parameters:**
- Fut1
- Fut2
- Fut3
- Fut4

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::future::join::Join5

*Struct*

Future for the [`join5`] function.

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
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::future::join::join

*Function*

Joins the result of two futures, waiting for them both to complete.

This function will return a new future which awaits both futures to
complete. The returned future will finish with a tuple of both results.

Note that this function consumes the passed futures and returns a
wrapped version of it.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let pair = future::join(a, b);

assert_eq!(pair.await, (1, 2));
# });
```

```rust
fn join<Fut1, Fut2>(future1: Fut1, future2: Fut2) -> Join<Fut1, Fut2>
```



## futures_util::future::join::join3

*Function*

Same as [`join`](join()), but with more futures.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let tuple = future::join3(a, b, c);

assert_eq!(tuple.await, (1, 2, 3));
# });
```

```rust
fn join3<Fut1, Fut2, Fut3>(future1: Fut1, future2: Fut2, future3: Fut3) -> Join3<Fut1, Fut2, Fut3>
```



## futures_util::future::join::join4

*Function*

Same as [`join`](join()), but with more futures.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let d = async { 4 };
let tuple = future::join4(a, b, c, d);

assert_eq!(tuple.await, (1, 2, 3, 4));
# });
```

```rust
fn join4<Fut1, Fut2, Fut3, Fut4>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4) -> Join4<Fut1, Fut2, Fut3, Fut4>
```



## futures_util::future::join::join5

*Function*

Same as [`join`](join()), but with more futures.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let d = async { 4 };
let e = async { 5 };
let tuple = future::join5(a, b, c, d, e);

assert_eq!(tuple.await, (1, 2, 3, 4, 5));
# });
```

```rust
fn join5<Fut1, Fut2, Fut3, Fut4, Fut5>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4, future5: Fut5) -> Join5<Fut1, Fut2, Fut3, Fut4, Fut5>
```



