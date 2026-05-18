**futures_task > future_obj**

# Module: future_obj

## Contents

**Structs**

- [`FutureObj`](#futureobj) - A custom trait object for polling futures, roughly akin to
- [`LocalFutureObj`](#localfutureobj) - A custom trait object for polling futures, roughly akin to

**Traits**

- [`UnsafeFutureObj`](#unsafefutureobj) - A custom implementation of a future trait object for `FutureObj`, providing

---

## futures_task::future_obj::FutureObj

*Struct*

A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + Send + 'a>`.

This custom trait object was introduced as currently it is not possible to
take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
contexts.

You should generally not need to use this type outside of `no_std` or when
implementing `Spawn`, consider using `BoxFuture` instead.

**Generic Parameters:**
- 'a
- T

**Tuple Struct**: `()`

**Methods:**

- `fn new<F>(f: F) -> Self` - Create a `FutureObj` from a custom trait object representation.

**Traits:** Send

**Trait Implementations:**

- **From**
  - `fn from(boxed: Box<F>) -> Self`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(boxed: Pin<Box<dyn Future>>) -> Self`
- **From**
  - `fn from(boxed: Pin<Box<F>>) -> Self`
- **From**
  - `fn from(boxed: Box<dyn Future>) -> Self`



## futures_task::future_obj::LocalFutureObj

*Struct*

A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + 'a>`.

This custom trait object was introduced as currently it is not possible to
take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
contexts.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new<F>(f: F) -> Self` - Create a `LocalFutureObj` from a custom trait object representation.
- `fn into_future_obj(self: Self) -> FutureObj<'a, T>` - Converts the `LocalFutureObj` into a `FutureObj`.

**Trait Implementations:**

- **From**
  - `fn from(boxed: Pin<Box<dyn Future>>) -> Self`
- **From**
  - `fn from(boxed: Pin<Box<F>>) -> Self`
- **From**
  - `fn from(boxed: Box<dyn Future>) -> Self`
- **From**
  - `fn from(f: FutureObj<'a, T>) -> Self`
- **From**
  - `fn from(boxed: Box<F>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<T>`



## futures_task::future_obj::UnsafeFutureObj

*Trait*

A custom implementation of a future trait object for `FutureObj`, providing
a vtable with drop support.

This custom representation is typically used only in `no_std` contexts,
where the default `Box`-based implementation is not available.

# Safety

See the safety notes on individual methods for what guarantees an
implementor must provide.

**Methods:**

- `into_raw`: Convert an owned instance into a (conceptually owned) fat pointer.
- `drop`: Drops the future represented by the given fat pointer.



