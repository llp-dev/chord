**futures_util > future > try_maybe_done**

# Module: future::try_maybe_done

## Contents

**Enums**

- [`TryMaybeDone`](#trymaybedone) - A future that may have completed with an error.

**Functions**

- [`try_maybe_done`](#try_maybe_done) - Wraps a future into a `TryMaybeDone`

---

## futures_util::future::try_maybe_done::TryMaybeDone

*Enum*

A future that may have completed with an error.

This is created by the [`try_maybe_done()`] function.

**Generic Parameters:**
- Fut

**Variants:**
- `Future(Fut)` - A not-yet-completed future
- `Done(<Fut as >::Ok)` - The output of the completed future
- `Gone` - The empty variant after the result of a [`TryMaybeDone`] has been

**Methods:**

- `fn output_mut(self: Pin<& mut Self>) -> Option<& mut <Fut as >::Ok>` - Returns an [`Option`] containing a mutable reference to the output of the future.
- `fn take_output(self: Pin<& mut Self>) -> Option<<Fut as >::Ok>` - Attempt to take the output of a `TryMaybeDone` without driving it

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::future::try_maybe_done::try_maybe_done

*Function*

Wraps a future into a `TryMaybeDone`

```rust
fn try_maybe_done<Fut>(future: Fut) -> TryMaybeDone<Fut>
```



