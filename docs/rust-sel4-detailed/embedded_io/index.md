# Crate `embedded_io`

[![crates.io](https://img.shields.io/crates/d/embedded-io.svg)](https://crates.io/crates/embedded-io)
[![crates.io](https://img.shields.io/crates/v/embedded-io.svg)](https://crates.io/crates/embedded-io)
[![Documentation](https://docs.rs/embedded-io/badge.svg)](https://docs.rs/embedded-io)

# `embedded-io`

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

Input/Output traits for embedded systems.

Rust's `std::io` traits are not available in `no_std` targets, mainly because `std::io::Error`
requires allocation. This crate contains replacement equivalent traits, usable in `no_std`
targets.

## Differences with `std::io`

- `Error` is an associated type. This allows each implementor to return its own error type,
  while avoiding `dyn` or `Box`. This is consistent with how errors are handled in [`embedded-hal`](https://github.com/rust-embedded/embedded-hal/).
- In `std::io`, the `Read`/`Write` traits might be blocking or non-blocking (i.e. returning `WouldBlock` errors) depending on the file descriptor's mode, which is only known at run-time. This allows passing a non-blocking stream to code that expects a blocking
  stream, causing unexpected errors. To solve this, `embedded-io` specifies `Read`/`Write` are always blocking, and adds new `ReadReady`/`WriteReady` traits to allow using streams in a non-blocking way.

## Optional Cargo features

- **`std`**: Adds `From` impls to convert to/from `std::io` structs.
- **`alloc`**: Adds blanket impls for `Box`, adds `Write` impl to `Vec`.
- **`defmt`**: Derive `defmt::Format` from `defmt` 1.x for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.81 and up. It *might*
compile with older versions but that may change in any new patch release.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Contents

- [Modules](#modules)
  - [`impls`](#impls)
- [Enums](#enums)
  - [`SeekFrom`](#seekfrom)
  - [`ErrorKind`](#errorkind)
  - [`ReadExactError`](#readexacterror)
  - [`SliceWriteError`](#slicewriteerror)
  - [`WriteFmtError`](#writefmterror)
- [Traits](#traits)
  - [`Error`](#error)
  - [`ErrorType`](#errortype)
  - [`Read`](#read)
  - [`BufRead`](#bufread)
  - [`Write`](#write)
  - [`Seek`](#seek)
  - [`ReadReady`](#readready)
  - [`WriteReady`](#writeready)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impls`](#impls) | mod |  |
| [`SeekFrom`](#seekfrom) | enum | Enumeration of possible methods to seek within an I/O object. |
| [`ErrorKind`](#errorkind) | enum | Possible kinds of errors. |
| [`ReadExactError`](#readexacterror) | enum | Error returned by [`Read::read_exact`] |
| [`SliceWriteError`](#slicewriteerror) | enum | Errors that could be returned by `Write` on `&mut [u8]`. |
| [`WriteFmtError`](#writefmterror) | enum | Error returned by [`Write::write_fmt`] |
| [`Error`](#error) | trait | Error trait. |
| [`ErrorType`](#errortype) | trait | Base trait for all IO traits, defining the error type. |
| [`Read`](#read) | trait | Blocking reader. |
| [`BufRead`](#bufread) | trait | Blocking buffered reader. |
| [`Write`](#write) | trait | Blocking writer. |
| [`Seek`](#seek) | trait | Blocking seek within streams.\ |
| [`ReadReady`](#readready) | trait | Get whether a reader is ready. |
| [`WriteReady`](#writeready) | trait | Get whether a writer is ready. |

## Modules

- [`impls`](impls/index.md)

## Enums

### `SeekFrom`

```rust
enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
```

Enumeration of possible methods to seek within an I/O object.

This is the `embedded-io` equivalent of `std::io::SeekFrom`.

#### Variants

- **`Start`**

  Sets the offset to the provided number of bytes.

- **`End`**

  Sets the offset to the size of this object plus the specified number of bytes.

- **`Current`**

  Sets the offset to the current position plus the specified number of bytes.

#### Trait Implementations

##### `impl Clone for SeekFrom`

- <span id="seekfrom-clone"></span>`fn clone(&self) -> SeekFrom` — [`SeekFrom`](#seekfrom)

##### `impl Copy for SeekFrom`

##### `impl Debug for SeekFrom`

- <span id="seekfrom-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SeekFrom`

##### `impl PartialEq for SeekFrom`

- <span id="seekfrom-partialeq-eq"></span>`fn eq(&self, other: &SeekFrom) -> bool` — [`SeekFrom`](#seekfrom)

##### `impl StructuralPartialEq for SeekFrom`

### `ErrorKind`

```rust
enum ErrorKind {
    Other,
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    InvalidInput,
    InvalidData,
    TimedOut,
    Interrupted,
    Unsupported,
    OutOfMemory,
    WriteZero,
}
```

Possible kinds of errors.

This list is intended to grow over time and it is not recommended to
exhaustively match against it. In application code, use `match` for the `ErrorKind`
values you are expecting; use `_` to match "all other errors".

This is the `embedded-io` equivalent of `std::io::ErrorKind`, except with the following changes:

- `WouldBlock` is removed, since `embedded-io` traits are always blocking. See the [crate-level documentation](crate) for details.

#### Variants

- **`Other`**

  Unspecified error kind.

- **`NotFound`**

  An entity was not found, often a file.

- **`PermissionDenied`**

  The operation lacked the necessary privileges to complete.

- **`ConnectionRefused`**

  The connection was refused by the remote server.

- **`ConnectionReset`**

  The connection was reset by the remote server.

- **`ConnectionAborted`**

  The connection was aborted (terminated) by the remote server.

- **`NotConnected`**

  The network operation failed because it was not connected yet.

- **`AddrInUse`**

  A socket address could not be bound because the address is already in
  use elsewhere.

- **`AddrNotAvailable`**

  A nonexistent interface was requested or the requested address was not
  local.

- **`BrokenPipe`**

  The operation failed because a pipe was closed.

- **`AlreadyExists`**

  An entity already exists, often a file.

- **`InvalidInput`**

  A parameter was incorrect.

- **`InvalidData`**

  Data not valid for the operation were encountered.
  
  Unlike [`InvalidInput`](#invalidinput), this typically means that the operation
  parameters were valid, however the error was caused by malformed
  input data.
  
  For example, a function that reads a file into a string will error with
  `InvalidData` if the file's contents are not valid UTF-8.
  

- **`TimedOut`**

  The I/O operation's timeout expired, causing it to be canceled.

- **`Interrupted`**

  This operation was interrupted.
  
  Interrupted operations can typically be retried.

- **`Unsupported`**

  This operation is unsupported on this platform.
  
  This means that the operation can never succeed.

- **`OutOfMemory`**

  An operation could not be completed, because it failed
  to allocate enough memory.

- **`WriteZero`**

  An attempted write could not write any data.

#### Trait Implementations

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- <span id="errorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl Error for ErrorKind`

- <span id="errorkind-error-kind"></span>`fn kind(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-partialeq-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

### `ReadExactError<E>`

```rust
enum ReadExactError<E> {
    UnexpectedEof,
    Other(E),
}
```

Error returned by `Read::read_exact`

#### Variants

- **`UnexpectedEof`**

  An EOF error was encountered before reading the exact amount of requested bytes.

- **`Other`**

  Error returned by the inner Read.

#### Trait Implementations

##### `impl<E: clone::Clone> Clone for ReadExactError<E>`

- <span id="readexacterror-clone"></span>`fn clone(&self) -> ReadExactError<E>` — [`ReadExactError`](#readexacterror)

##### `impl<E: marker::Copy> Copy for ReadExactError<E>`

##### `impl<E: fmt::Debug> Debug for ReadExactError<E>`

- <span id="readexacterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: fmt::Debug> Display for ReadExactError<E>`

- <span id="readexacterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: cmp::Eq> Eq for ReadExactError<E>`

##### `impl<E: fmt::Debug> Error for ReadExactError<E>`

##### `impl<E: cmp::PartialEq> PartialEq for ReadExactError<E>`

- <span id="readexacterror-partialeq-eq"></span>`fn eq(&self, other: &ReadExactError<E>) -> bool` — [`ReadExactError`](#readexacterror)

##### `impl<E> StructuralPartialEq for ReadExactError<E>`

### `SliceWriteError`

```rust
enum SliceWriteError {
    Full,
}
```

Errors that could be returned by `Write` on `&mut [u8]`.

#### Variants

- **`Full`**

  The target slice was full and so could not receive any new data.

#### Trait Implementations

##### `impl Clone for SliceWriteError`

- <span id="slicewriteerror-clone"></span>`fn clone(&self) -> SliceWriteError` — [`SliceWriteError`](#slicewriteerror)

##### `impl Copy for SliceWriteError`

##### `impl Debug for SliceWriteError`

- <span id="slicewriteerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for crate::SliceWriteError`

- <span id="crateslicewriteerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for SliceWriteError`

##### `impl Error for crate::SliceWriteError`

- <span id="crateslicewriteerror-error-kind"></span>`fn kind(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl PartialEq for SliceWriteError`

- <span id="slicewriteerror-partialeq-eq"></span>`fn eq(&self, other: &SliceWriteError) -> bool` — [`SliceWriteError`](#slicewriteerror)

##### `impl StructuralPartialEq for SliceWriteError`

### `WriteFmtError<E>`

```rust
enum WriteFmtError<E> {
    FmtError,
    Other(E),
}
```

Error returned by `Write::write_fmt`

#### Variants

- **`FmtError`**

  An error was encountered while formatting.

- **`Other`**

  Error returned by the inner Write.

#### Trait Implementations

##### `impl<E: clone::Clone> Clone for WriteFmtError<E>`

- <span id="writefmterror-clone"></span>`fn clone(&self) -> WriteFmtError<E>` — [`WriteFmtError`](#writefmterror)

##### `impl<E: marker::Copy> Copy for WriteFmtError<E>`

##### `impl<E: fmt::Debug> Debug for WriteFmtError<E>`

- <span id="writefmterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: fmt::Debug> Display for WriteFmtError<E>`

- <span id="writefmterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: cmp::Eq> Eq for WriteFmtError<E>`

##### `impl<E: fmt::Debug> Error for WriteFmtError<E>`

##### `impl<E: cmp::PartialEq> PartialEq for WriteFmtError<E>`

- <span id="writefmterror-partialeq-eq"></span>`fn eq(&self, other: &WriteFmtError<E>) -> bool` — [`WriteFmtError`](#writefmterror)

##### `impl<E> StructuralPartialEq for WriteFmtError<E>`

## Traits

### `Error`

```rust
trait Error: core::error::Error { ... }
```

Error trait.

This trait allows generic code to do limited inspecting of errors,
to react differently to different kinds.

#### Required Methods

- `fn kind(&self) -> ErrorKind`

  Get the kind of this error.

#### Implementors

- [`ErrorKind`](#errorkind)
- [`SliceWriteError`](#slicewriteerror)
- `core::convert::Infallible`

### `ErrorType`

```rust
trait ErrorType { ... }
```

Base trait for all IO traits, defining the error type.

All IO operations of all traits return the error defined in this trait.

Having a shared trait instead of having every trait define its own
`Error` associated type enforces all impls on the same type use the same error.
This is very convenient when writing generic code, it means you have to
handle a single error type `T::Error`, instead of `<T as Read>::Error` and `<T as Write>::Error`
which might be different types.

#### Associated Types

- `type Error: 1`

#### Implementors

- `&[u8]`
- `&mut T`
- `&mut [u8]`

### `Read`

```rust
trait Read: ErrorType { ... }
```

Blocking reader.

This trait is the `embedded-io` equivalent of `std::io::Read`.

#### Required Methods

- `fn read(&mut self, buf: &mut [u8]) -> Result<usize, <Self as >::Error>`

  Read some bytes from this source into the specified buffer, returning how many bytes were read.

#### Provided Methods

- `fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadExactError<<Self as >::Error>>`

  Read the exact number of bytes required to fill `buf`.

#### Implementors

- `&[u8]`
- `&mut T`

### `BufRead`

```rust
trait BufRead: Read { ... }
```

Blocking buffered reader.

This trait is the `embedded-io` equivalent of `std::io::BufRead`.

#### Required Methods

- `fn fill_buf(&mut self) -> Result<&[u8], <Self as >::Error>`

  Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.

- `fn consume(&mut self, amt: usize)`

  Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`.

#### Implementors

- `&[u8]`
- `&mut T`

### `Write`

```rust
trait Write: ErrorType { ... }
```

Blocking writer.

This trait is the `embedded-io` equivalent of `std::io::Write`.

#### Required Methods

- `fn write(&mut self, buf: &[u8]) -> Result<usize, <Self as >::Error>`

  Write a buffer into this writer, returning how many bytes were written.

- `fn flush(&mut self) -> Result<(), <Self as >::Error>`

  Flush this output stream, blocking until all intermediately buffered contents reach their destination.

#### Provided Methods

- `fn write_all(&mut self, buf: &[u8]) -> Result<(), <Self as >::Error>`

  Write an entire buffer into this writer.

- `fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<(), WriteFmtError<<Self as >::Error>>`

  Write a formatted string into this writer, returning any error encountered.

#### Implementors

- `&mut T`
- `&mut [u8]`

### `Seek`

```rust
trait Seek: ErrorType { ... }
```

Blocking seek within streams.\

The `Seek` trait provides a cursor which can be moved within a stream of
bytes.

The stream typically has a fixed size, allowing seeking relative to either
end or the current offset.

This trait is the `embedded-io` equivalent of `std::io::Seek`.

#### Required Methods

- `fn seek(&mut self, pos: SeekFrom) -> Result<u64, <Self as >::Error>`

  Seek to an offset, in bytes, in a stream.

#### Provided Methods

- `fn rewind(&mut self) -> Result<(), <Self as >::Error>`

  Rewind to the beginning of a stream.

- `fn stream_position(&mut self) -> Result<u64, <Self as >::Error>`

  Returns the current seek position from the start of the stream.

- `fn seek_relative(&mut self, offset: i64) -> Result<(), <Self as >::Error>`

  Seeks relative to the current position.

#### Implementors

- `&mut T`

### `ReadReady`

```rust
trait ReadReady: ErrorType { ... }
```

Get whether a reader is ready.

This allows using a [`Read`](#read) or [`BufRead`](#bufread) in a nonblocking fashion, i.e. trying to read
only when it is ready.

#### Required Methods

- `fn read_ready(&mut self) -> Result<bool, <Self as >::Error>`

  Get whether the reader is ready for immediately reading.

#### Implementors

- `&[u8]`
- `&mut T`

### `WriteReady`

```rust
trait WriteReady: ErrorType { ... }
```

Get whether a writer is ready.

This allows using a [`Write`](#write) in a nonblocking fashion, i.e. trying to write
only when it is ready.

#### Required Methods

- `fn write_ready(&mut self) -> Result<bool, <Self as >::Error>`

  Get whether the writer is ready for immediately writing.

#### Implementors

- `&mut T`
- `&mut [u8]`

