**futures_util > future > select_all**

# Module: future::select_all

## Contents

**Structs**

- [`SelectAll`](#selectall) - Future for the [`select_all`] function.

**Functions**

- [`select_all`](#select_all) - Creates a new future which will select over a list of futures.

---

## futures_util::future::select_all::SelectAll

*Struct*

Future for the [`select_all`] function.

**Generic Parameters:**
- Fut

**Methods:**

- `fn into_inner(self: Self) -> Vec<Fut>` - Consumes this combinator, returning the underlying futures.

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`



## futures_util::future::select_all::select_all

*Function*

Creates a new future which will select over a list of futures.

The returned future will wait for any future within `iter` to be ready. Upon
completion the item resolved will be returned, along with the index of the
future that was ready and the list of all the remaining futures.

There are no guarantees provided on the order of the list with the remaining
futures. They might be swapped around, reversed, or completely random.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# Panics

This function will panic if the iterator specified contains no items.

```rust
fn select_all<I>(iter: I) -> SelectAll<<I as >::Item>
```



