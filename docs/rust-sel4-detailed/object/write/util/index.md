*[object](../../index.md) / [write](../index.md) / [util](index.md)*

---

# Module `util`

## Contents

- [Structs](#structs)
  - [`StreamingBuffer`](#streamingbuffer)
- [Traits](#traits)
  - [`WritableBuffer`](#writablebuffer)
  - [`BytesMut`](#bytesmut)
- [Functions](#functions)
  - [`write_uleb128`](#write-uleb128)
  - [`write_sleb128`](#write-sleb128)
  - [`align`](#align)
  - [`align_u32`](#align-u32)
  - [`align_u64`](#align-u64)
  - [`write_align`](#write-align)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StreamingBuffer`](#streamingbuffer) | struct | A [`WritableBuffer`] that streams data to a [`Write`](std::io::Write) implementation. |
| [`WritableBuffer`](#writablebuffer) | trait | Trait for writable buffer. |
| [`BytesMut`](#bytesmut) | trait | A trait for mutable byte slices. |
| [`write_uleb128`](#write-uleb128) | fn | Write an unsigned number using the LEB128 encoding to a buffer. |
| [`write_sleb128`](#write-sleb128) | fn | Write a signed number using the LEB128 encoding to a buffer. |
| [`align`](#align) | fn |  |
| [`align_u32`](#align-u32) | fn |  |
| [`align_u64`](#align-u64) | fn |  |
| [`write_align`](#write-align) | fn |  |

## Structs

### `StreamingBuffer<W>`

```rust
struct StreamingBuffer<W> {
    writer: W,
    len: usize,
    result: Result<(), io::Error>,
}
```

A [`WritableBuffer`](../index.md) that streams data to a [`Write`](std::io::Write) implementation.

`Self::result` must be called to determine if an I/O error occurred during writing.
Alternatively, `Self::flush` will both check for errors and flush.

It is advisable to use a buffered writer like [`BufWriter`](std::io::BufWriter)
instead of an unbuffered writer like [`File`](std::fs::File).

#### Implementations

- <span id="streamingbuffer-new"></span>`fn new(writer: W) -> Self`

  Create a new `StreamingBuffer` backed by the given writer.

- <span id="streamingbuffer-into-inner"></span>`fn into_inner(self) -> W`

  Unwraps this [`StreamingBuffer`](../index.md) giving back the original writer.

- <span id="streamingbuffer-result"></span>`fn result(&mut self) -> Result<(), io::Error>`

  Returns any error that occurred during writing.

#### Trait Implementations

##### `impl<W: fmt::Debug> Debug for StreamingBuffer<W>`

- <span id="streamingbuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: io::Write> WritableBuffer for StreamingBuffer<W>`

- <span id="streamingbuffer-writablebuffer-len"></span>`fn len(&self) -> usize`

- <span id="streamingbuffer-writablebuffer-reserve"></span>`fn reserve(&mut self, _size: usize) -> Result<(), ()>`

- <span id="streamingbuffer-writablebuffer-resize"></span>`fn resize(&mut self, new_len: usize)`

- <span id="streamingbuffer-writablebuffer-write-bytes"></span>`fn write_bytes(&mut self, val: &[u8])`

## Traits

### `WritableBuffer`

```rust
trait WritableBuffer { ... }
```

Trait for writable buffer.

#### Required Methods

- `fn len(&self) -> usize`

  Returns position/offset for data to be written at.

- `fn reserve(&mut self, size: usize) -> Result<(), ()>`

  Reserves specified number of bytes in the buffer.

- `fn resize(&mut self, new_len: usize)`

  Writes zero bytes at the end of the buffer until the buffer

- `fn write_bytes(&mut self, val: &[u8])`

  Writes the specified slice of bytes at the end of the buffer.

#### Provided Methods

- `fn write_pod<T: Pod>(&mut self, val: &T)`

  Writes the specified `Pod` type at the end of the buffer.

- `fn write_pod_slice<T: Pod>(&mut self, val: &[T])`

  Writes the specified `Pod` slice at the end of the buffer.

#### Implementors

- [`StreamingBuffer`](../index.md#streamingbuffer)
- `alloc::vec::Vec<u8>`

### `BytesMut`

```rust
trait BytesMut { ... }
```

A trait for mutable byte slices.

It provides convenience methods for `Pod` types.

#### Required Methods

- `fn write_at<T: Pod>(self, offset: usize, val: &T) -> Result<(), ()>`

#### Implementors

- `&'a mut [u8]`

## Functions

### `write_uleb128`

```rust
fn write_uleb128(buf: &mut alloc::vec::Vec<u8>, val: u64) -> usize
```

Write an unsigned number using the LEB128 encoding to a buffer.

Returns the number of bytes written.

### `write_sleb128`

```rust
fn write_sleb128(buf: &mut alloc::vec::Vec<u8>, val: i64) -> usize
```

Write a signed number using the LEB128 encoding to a buffer.

Returns the number of bytes written.

### `align`

```rust
fn align(offset: usize, size: usize) -> usize
```

### `align_u32`

```rust
fn align_u32(offset: u32, size: u32) -> u32
```

### `align_u64`

```rust
fn align_u64(offset: u64, size: u64) -> u64
```

### `write_align`

```rust
fn write_align(buffer: &mut dyn WritableBuffer, size: usize)
```

