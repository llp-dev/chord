**object > write > util**

# Module: write::util

## Contents

**Structs**

- [`StreamingBuffer`](#streamingbuffer) - A [`WritableBuffer`] that streams data to a [`Write`](std::io::Write) implementation.

**Traits**

- [`WritableBuffer`](#writablebuffer) - Trait for writable buffer.

---

## object::write::util::StreamingBuffer

*Struct*

A [`WritableBuffer`] that streams data to a [`Write`](std::io::Write) implementation.

[`Self::result`] must be called to determine if an I/O error occurred during writing.
Alternatively, [`Self::flush`] will both check for errors and flush.

It is advisable to use a buffered writer like [`BufWriter`](std::io::BufWriter)
instead of an unbuffered writer like [`File`](std::fs::File).

**Generic Parameters:**
- W

**Methods:**

- `fn flush(self: & mut Self) -> Result<(), io::Error>` - Flushes after first checking if any error previously occurred during writing.
- `fn new(writer: W) -> Self` - Create a new `StreamingBuffer` backed by the given writer.
- `fn into_inner(self: Self) -> W` - Unwraps this [`StreamingBuffer`] giving back the original writer.
- `fn result(self: & mut Self) -> Result<(), io::Error>` - Returns any error that occurred during writing.

**Trait Implementations:**

- **WritableBuffer**
  - `fn len(self: &Self) -> usize`
  - `fn reserve(self: & mut Self, _size: usize) -> Result<(), ()>`
  - `fn resize(self: & mut Self, new_len: usize)`
  - `fn write_bytes(self: & mut Self, val: &[u8])`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::util::WritableBuffer

*Trait*

Trait for writable buffer.

**Methods:**

- `len`: Returns position/offset for data to be written at.
- `reserve`: Reserves specified number of bytes in the buffer.
- `resize`: Writes zero bytes at the end of the buffer until the buffer
- `write_bytes`: Writes the specified slice of bytes at the end of the buffer.
- `write_pod`: Writes the specified `Pod` type at the end of the buffer.
- `write_pod_slice`: Writes the specified `Pod` slice at the end of the buffer.



