**sel4_async_io**

# Module: sel4_async_io

## Contents

**Structs**

- [`EmbeddedIOAsyncAdapter`](#embeddedioasyncadapter)
- [`EmbeddedIOAsyncAdapterUsingReady`](#embeddedioasyncadapterusingready)

**Traits**

- [`FlushCancelSafe`](#flushcancelsafe)
- [`Read`](#read)
- [`ReadCancelSafe`](#readcancelsafe)
- [`Write`](#write)
- [`WriteCancelSafe`](#writecancelsafe)

---

## sel4_async_io::EmbeddedIOAsyncAdapter

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** ErrorType, WriteCancelSafe, FlushCancelSafe, ReadCancelSafe, Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EmbeddedIOAsyncAdapter<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Write**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<Result<usize, <Self as >::Error>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &EmbeddedIOAsyncAdapter<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> Result<usize, <Self as >::Error>`
  - `fn flush(self: & mut Self) -> Result<(), <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> EmbeddedIOAsyncAdapter<T>`
- **Read**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut [u8]) -> Poll<Result<usize, <Self as >::Error>>`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> Result<usize, <Self as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &EmbeddedIOAsyncAdapter<T>) -> $crate::cmp::Ordering`



## sel4_async_io::EmbeddedIOAsyncAdapterUsingReady

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, ErrorType, Copy

**Trait Implementations:**

- **Write**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<Result<usize, <Self as >::Error>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &EmbeddedIOAsyncAdapterUsingReady<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &EmbeddedIOAsyncAdapterUsingReady<T>) -> $crate::cmp::Ordering`
- **Read**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut [u8]) -> Poll<Result<usize, <Self as >::Error>>`
- **PartialEq**
  - `fn eq(self: &Self, other: &EmbeddedIOAsyncAdapterUsingReady<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> EmbeddedIOAsyncAdapterUsingReady<T>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_async_io::FlushCancelSafe

*Trait*



## sel4_async_io::Read

*Trait*

**Methods:**

- `poll_read`



## sel4_async_io::ReadCancelSafe

*Trait*



## sel4_async_io::Write

*Trait*

**Methods:**

- `poll_write`
- `poll_flush`



## sel4_async_io::WriteCancelSafe

*Trait*



