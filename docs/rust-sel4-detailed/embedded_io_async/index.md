# Crate `embedded_io_async`

[![crates.io](https://img.shields.io/crates/d/embedded-io-async.svg)](https://crates.io/crates/embedded-io-async)
[![crates.io](https://img.shields.io/crates/v/embedded-io-async.svg)](https://crates.io/crates/embedded-io-async)
[![Documentation](https://docs.rs/embedded-io-async/badge.svg)](https://docs.rs/embedded-io-async)

# `embedded-io-async`

Async IO traits for embedded systems.

This crate contains asynchronous versions of the [`embedded-io`](https://crates.io/crates/embedded-io) traits and shares its scope and design goals.

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

## Optional Cargo features

- **`std`**: Adds `From` impls to convert to/from `std::io` structs.
- **`alloc`**: Adds blanket impls for `Box`, adds `Write` impl to `Vec`.
- **`defmt`**: Derive `defmt::Format` from `defmt` 1.0 for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.75 and up. It *might*
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
  - [`Error`](#error)
- [Structs](#structs)
  - [`ErrorKind`](#errorkind)
- [Traits](#traits)
  - [`Read`](#read)
  - [`BufRead`](#bufread)
  - [`Write`](#write)
  - [`Seek`](#seek)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impls`](#impls) | mod |  |
| [`Error`](#error) | mod |  |
| [`ErrorKind`](#errorkind) | struct |  |
| [`Read`](#read) | trait | Async reader. |
| [`BufRead`](#bufread) | trait | Async buffered reader. |
| [`Write`](#write) | trait | Async writer. |
| [`Seek`](#seek) | trait | Async seek within streams. |

## Modules

- [`impls`](impls/index.md)
- [`Error`](Error/index.md)

## Structs

### `ErrorKind`

```rust
struct ErrorKind;
```

*Re-exported from `aligned`*

4-byte alignment

#### Trait Implementations

##### `impl Alignment for A4`

- <span id="a4-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A4`

- <span id="a4-clone"></span>`fn clone(&self) -> A4` — [`ErrorKind`](#errorkind)

##### `impl Copy for A4`

##### `impl Sealed for super::A4`

## Traits

### `Read`

```rust
trait Read: ErrorType { ... }
```

Async reader.

This trait is the `embedded-io-async` equivalent of `std::io::Read`.

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

Async buffered reader.

This trait is the `embedded-io-async` equivalent of `std::io::BufRead`.

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

Async writer.

This trait is the `embedded-io-async` equivalent of `std::io::Write`.

#### Required Methods

- `fn write(&mut self, buf: &[u8]) -> Result<usize, <Self as >::Error>`

  Write a buffer into this writer, returning how many bytes were written.

- `fn flush(&mut self) -> Result<(), <Self as >::Error>`

  Flush this output stream, ensuring that all intermediately buffered contents reach their destination.

#### Provided Methods

- `fn write_all(&mut self, buf: &[u8]) -> Result<(), <Self as >::Error>`

  Write an entire buffer into this writer.

#### Implementors

- `&mut T`
- `&mut [u8]`

### `Seek`

```rust
trait Seek: ErrorType { ... }
```

Async seek within streams.

This trait is the `embedded-io-async` equivalent of `std::io::Seek`.

#### Required Methods

- `fn seek(&mut self, pos: SeekFrom) -> Result<u64, <Self as >::Error>`

  Seek to an offset, in bytes, in a stream.

#### Provided Methods

- `fn rewind(&mut self) -> Result<(), <Self as >::Error>`

  Rewind to the beginning of a stream.

- `fn stream_position(&mut self) -> Result<u64, <Self as >::Error>`

  Returns the current seek position from the start of the stream.

#### Implementors

- `&mut T`

