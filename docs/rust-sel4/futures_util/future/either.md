**futures_util > future > either**

# Module: future::either

## Contents

**Enums**

- [`Either`](#either) - Combines two different futures, streams, or sinks having the same associated types into a single type.

---

## futures_util::future::either::Either

*Enum*

Combines two different futures, streams, or sinks having the same associated types into a single type.

This is useful when conditionally choosing between two distinct future types:

```rust
use futures::future::Either;

# futures::executor::block_on(async {
let cond = true;

let fut = if cond {
    Either::Left(async move { 12 })
} else {
    Either::Right(async move { 44 })
};

assert_eq!(fut.await, 12);
# })
```

**Generic Parameters:**
- A
- B

**Variants:**
- `Left(A)` - First branch of the type
- `Right(B)` - Second branch of the type

**Methods:**

- `fn into_inner(self: Self) -> T` - Extract the value of an either over two equivalent types.
- `fn factor_first(self: Self) -> (T, Either<A, B>)` - Factor out a homogeneous type from an either of pairs.
- `fn factor_second(self: Self) -> (Either<A, B>, T)` - Factor out a homogeneous type from an either of pairs.
- `fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&A>, Pin<&B>>` - Convert `Pin<&Either<A, B>>` to `Either<Pin<&A>, Pin<&B>>`,
- `fn as_pin_mut(self: Pin<& mut Self>) -> Either<Pin<& mut A>, Pin<& mut B>>` - Convert `Pin<&mut Either<A, B>>` to `Either<Pin<&mut A>, Pin<&mut B>>`,

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Either<A, B>`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`



