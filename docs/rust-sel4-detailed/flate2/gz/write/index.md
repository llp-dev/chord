*[flate2](../../index.md) / [gz](../index.md) / [write](index.md)*

---

# Module `write`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GzEncoder`](#gzencoder) | struct | A gzip streaming encoder |
| [`GzDecoder`](#gzdecoder) | struct | A decoder for a single member of a [gzip file]. |
| [`MultiGzDecoder`](#multigzdecoder) | struct | A gzip streaming decoder that decodes a [gzip file] with multiple members. |
| [`gz_encoder`](#gz-encoder) | fn |  |
| [`CRC_BYTES_LEN`](#crc-bytes-len) | const |  |

## Structs

### `GzEncoder<W: Write>`

```rust
struct GzEncoder<W: Write> {
    inner: zio::Writer<W, crate::Compress>,
    crc: crate::crc::Crc,
    crc_bytes_written: usize,
    header: Vec<u8>,
}
```

A gzip streaming encoder

This structure exposes a [`Write`]() interface that will emit compressed data
to the underlying writer `W`.

# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;

// Vec<u8> implements Write to print the compressed bytes of sample string
fn main() {

let mut e = GzEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World").unwrap();
println!("{:?}", e.finish().unwrap());
}
```

#### Implementations

- <span id="gzencoder-new"></span>`fn new(w: W, level: Compression) -> GzEncoder<W>` — [`Compression`](../../index.md#compression), [`GzEncoder`](#gzencoder)

  Creates a new encoder which will use the given compression level.

  

  The encoder is not configured specially for the emitted header. For

  header configuration, see the `GzBuilder` type.

  

  The data written to the returned encoder will be compressed and then

  written to the stream `w`.

- <span id="gzencoder-get-ref"></span>`fn get_ref(&self) -> &W`

  Acquires a reference to the underlying writer.

- <span id="gzencoder-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Acquires a mutable reference to the underlying writer.

  

  Note that mutation of the writer may result in surprising results if

  this encoder is continued to be used.

- <span id="gzencoder-try-finish"></span>`fn try_finish(&mut self) -> io::Result<()>`

  Attempt to finish this output stream, writing out final chunks of data.

  

  Note that this function can only be used once data has finished being

  written to the output stream. After this function is called then further

  calls to `write` may result in a panic.

  

  # Panics

  

  Attempts to write data to this stream may result in a panic after this

  function is called.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="gzencoder-finish"></span>`fn finish(self) -> io::Result<W>`

  Finish encoding this stream, returning the underlying writer once the

  encoding is done.

  

  Note that this function may not be suitable to call in a situation where

  the underlying stream is an asynchronous I/O stream. To finish a stream

  the `try_finish` (or `shutdown`) method should be used instead. To

  re-acquire ownership of a stream it is safe to call this method after

  `try_finish` or `shutdown` has returned `Ok`.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="gzencoder-write-header"></span>`fn write_header(&mut self) -> io::Result<()>`

#### Trait Implementations

##### `impl<W: fmt::Debug + Write> Debug for GzEncoder<W>`

- <span id="gzencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Write> Drop for GzEncoder<W>`

- <span id="gzencoder-drop"></span>`fn drop(&mut self)`

##### `impl<R: Read + Write> Read for GzEncoder<R>`

- <span id="gzencoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Write> Write for GzEncoder<W>`

- <span id="gzencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="gzencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `GzDecoder<W: Write>`

```rust
struct GzDecoder<W: Write> {
    inner: zio::Writer<crate::crc::CrcWriter<W>, crate::Decompress>,
    crc_bytes: Vec<u8>,
    header_parser: super::GzHeaderParser,
}
```

A decoder for a single member of a [gzip file].

This structure exposes a [`Write`]() interface, receiving compressed data and
writing uncompressed data to the underlying writer.

After decoding a single member of the gzip data this writer will return the number of bytes up to
to the end of the gzip member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the gzip member.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](#multigzdecoder)
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).


# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};

fn main() {
   let mut e = GzEncoder::new(Vec::new(), Compression::default());
   e.write(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   assert_eq!("Hello World", decode_writer(bytes).unwrap());
}
// Uncompresses a gzip encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write
fn decode_writer(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut decoder = GzDecoder::new(writer);
   decoder.write_all(&bytes[..])?;
   writer = decoder.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

#### Implementations

- <span id="gzdecoder-new"></span>`fn new(w: W) -> GzDecoder<W>` — [`GzDecoder`](#gzdecoder)

  Creates a new decoder which will write uncompressed data to the stream.

  

  When this encoder is dropped or unwrapped the final pieces of data will

  be flushed.

- <span id="gzdecoder-header"></span>`fn header(&self) -> Option<&GzHeader>` — [`GzHeader`](../index.md#gzheader)

  Returns the header associated with this stream.

- <span id="gzdecoder-get-ref"></span>`fn get_ref(&self) -> &W`

  Acquires a reference to the underlying writer.

- <span id="gzdecoder-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Acquires a mutable reference to the underlying writer.

  

  Note that mutating the output/input state of the stream may corrupt this

  object, so care must be taken when using this method.

- <span id="gzdecoder-try-finish"></span>`fn try_finish(&mut self) -> io::Result<()>`

  Attempt to finish this output stream, writing out final chunks of data.

  

  Note that this function can only be used once data has finished being

  written to the output stream. After this function is called then further

  calls to `write` may result in a panic.

  

  # Panics

  

  Attempts to write data to this stream may result in a panic after this

  function is called.

  

  # Errors

  

  This function will perform I/O to finish the stream, returning any

  errors which happen.

- <span id="gzdecoder-finish"></span>`fn finish(self) -> io::Result<W>`

  Consumes this decoder, flushing the output stream.

  

  This will flush the underlying data stream and then return the contained

  writer if the flush succeeded.

  

  Note that this function may not be suitable to call in a situation where

  the underlying stream is an asynchronous I/O stream. To finish a stream

  the `try_finish` (or `shutdown`) method should be used instead. To

  re-acquire ownership of a stream it is safe to call this method after

  `try_finish` or `shutdown` has returned `Ok`.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="gzdecoder-finish-and-check-crc"></span>`fn finish_and_check_crc(&mut self) -> io::Result<()>`

#### Trait Implementations

##### `impl<W: fmt::Debug + Write> Debug for GzDecoder<W>`

- <span id="gzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Read + Write> Read for GzDecoder<W>`

- <span id="gzdecoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Write> Write for GzDecoder<W>`

- <span id="gzdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="gzdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `MultiGzDecoder<W: Write>`

```rust
struct MultiGzDecoder<W: Write> {
    inner: GzDecoder<W>,
}
```

A gzip streaming decoder that decodes a [gzip file] with multiple members.

This structure exposes a [`Write`](../../../embedded_hal/index.md) interface that will consume compressed data and
write uncompressed data to the underlying writer.

A gzip file consists of a series of *members* concatenated one after another.
`MultiGzDecoder` decodes all members of a file and writes them to the
underlying writer one after another.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).


#### Implementations

- <span id="multigzdecoder-new"></span>`fn new(w: W) -> MultiGzDecoder<W>` — [`MultiGzDecoder`](#multigzdecoder)

  Creates a new decoder which will write uncompressed data to the stream.

  If the gzip stream contains multiple members all will be decoded.

- <span id="multigzdecoder-header"></span>`fn header(&self) -> Option<&GzHeader>` — [`GzHeader`](../index.md#gzheader)

  Returns the header associated with the current member.

- <span id="multigzdecoder-get-ref"></span>`fn get_ref(&self) -> &W`

  Acquires a reference to the underlying writer.

- <span id="multigzdecoder-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Acquires a mutable reference to the underlying writer.

  

  Note that mutating the output/input state of the stream may corrupt this

  object, so care must be taken when using this method.

- <span id="multigzdecoder-try-finish"></span>`fn try_finish(&mut self) -> io::Result<()>`

  Attempt to finish this output stream, writing out final chunks of data.

  

  Note that this function can only be used once data has finished being

  written to the output stream. After this function is called then further

  calls to `write` may result in a panic.

  

  # Panics

  

  Attempts to write data to this stream may result in a panic after this

  function is called.

  

  # Errors

  

  This function will perform I/O to finish the stream, returning any

  errors which happen.

- <span id="multigzdecoder-finish"></span>`fn finish(self) -> io::Result<W>`

  Consumes this decoder, flushing the output stream.

  

  This will flush the underlying data stream and then return the contained

  writer if the flush succeeded.

  

  Note that this function may not be suitable to call in a situation where

  the underlying stream is an asynchronous I/O stream. To finish a stream

  the `try_finish` (or `shutdown`) method should be used instead. To

  re-acquire ownership of a stream it is safe to call this method after

  `try_finish` or `shutdown` has returned `Ok`.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

#### Trait Implementations

##### `impl<W: fmt::Debug + Write> Debug for MultiGzDecoder<W>`

- <span id="multigzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Write> Write for MultiGzDecoder<W>`

- <span id="multigzdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="multigzdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

## Functions

### `gz_encoder`

```rust
fn gz_encoder<W: Write>(header: Vec<u8>, w: W, lvl: crate::Compression) -> GzEncoder<W>
```

## Constants

### `CRC_BYTES_LEN`
```rust
const CRC_BYTES_LEN: usize = 8usize;
```

