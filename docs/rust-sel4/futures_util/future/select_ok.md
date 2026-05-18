**futures_util > future > select_ok**

# Module: future::select_ok

## Contents

**Structs**

- [`SelectOk`](#selectok) - Future for the [`select_ok`] function.

**Functions**

- [`select_ok`](#select_ok) - Creates a new future which will select the first successful future over a list of futures.

---

## futures_util::future::select_ok::SelectOk

*Struct*

Future for the [`select_ok`] function.

**Generic Parameters:**
- Fut

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`



## futures_util::future::select_ok::select_ok

*Function*

Creates a new future which will select the first successful future over a list of futures.

The returned future will wait for any future within `iter` to be ready and Ok. Unlike
`select_all`, this will only return the first successful completion, or the last
failure. This is useful in contexts where any success is desired and failures
are ignored, unless all the futures fail.

 This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# Panics

This function will panic if the iterator specified contains no items.

```rust
fn select_ok<I>(iter: I) -> SelectOk<<I as >::Item>
```



