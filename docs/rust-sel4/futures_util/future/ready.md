**futures_util > future > ready**

# Module: future::ready

## Contents

**Structs**

- [`Ready`](#ready) - Future for the [`ready`](ready()) function.

**Functions**

- [`err`](#err) - Create a future that is immediately ready with an error value.
- [`ok`](#ok) - Create a future that is immediately ready with a success value.
- [`ready`](#ready) - Creates a future that is immediately ready with a value.

---

## futures_util::future::ready::Ready

*Struct*

Future for the [`ready`](ready()) function.

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn into_inner(self: Self) -> T` - Unwraps the value from this immediately ready future.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Ready<T>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, _cx: & mut Context) -> Poll<T>`



## futures_util::future::ready::err

*Function*

Create a future that is immediately ready with an error value.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = future::err::<i32, i32>(1);
assert_eq!(a.await, Err(1));
# });
```

```rust
fn err<T, E>(err: E) -> Ready<Result<T, E>>
```



## futures_util::future::ready::ok

*Function*

Create a future that is immediately ready with a success value.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = future::ok::<i32, i32>(1);
assert_eq!(a.await, Ok(1));
# });
```

```rust
fn ok<T, E>(t: T) -> Ready<Result<T, E>>
```



## futures_util::future::ready::ready

*Function*

Creates a future that is immediately ready with a value.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = future::ready(1);
assert_eq!(a.await, 1);
# });
```

```rust
fn ready<T>(t: T) -> Ready<T>
```



