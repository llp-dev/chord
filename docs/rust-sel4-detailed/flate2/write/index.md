*[flate2](../index.md) / [write](index.md)*

---

# Module `write`

Types which operate over [`Write`]() streams, both encoders and decoders for
various formats.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DeflateDecoder`](#deflatedecoder) | struct |  |
| [`DeflateEncoder`](#deflateencoder) | struct |  |
| [`GzDecoder`](#gzdecoder) | struct |  |
| [`GzEncoder`](#gzencoder) | struct |  |
| [`MultiGzDecoder`](#multigzdecoder) | struct |  |
| [`ZlibDecoder`](#zlibdecoder) | struct |  |
| [`ZlibEncoder`](#zlibencoder) | struct |  |

## Structs

### `DeflateDecoder<W: Write>`

```rust
struct DeflateDecoder<W: Write> {
    inner: zio::Writer<W, crate::Decompress>,
}
```

A DEFLATE decoder, or decompressor.

This structure implements a [`Write`]() and will emit a stream of decompressed
data when fed a stream of compressed data.

After decoding a single member of the DEFLATE data this writer will return the number of bytes up to
to the end of the DEFLATE member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the DEFLATE member.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::write::DeflateDecoder;

fn main() {
   let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   println!("{}", decode_writer(bytes).unwrap());
}
// Uncompresses a Deflate Encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write
fn decode_writer(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut deflater = DeflateDecoder::new(writer);
   deflater.write_all(&bytes[..])?;
   writer = deflater.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

#### Implementations

- <span id="deflatedecoder-new"></span>`fn new(w: W) -> DeflateDecoder<W>` — [`DeflateDecoder`](../deflate/write/index.md#deflatedecoder)

  Creates a new decoder which will write uncompressed data to the stream.

  

  When this encoder is dropped or unwrapped the final pieces of data will

  be flushed.

- <span id="deflatedecoder-get-ref"></span>`fn get_ref(&self) -> &W`

  Acquires a reference to the underlying writer.

- <span id="deflatedecoder-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Acquires a mutable reference to the underlying writer.

  

  Note that mutating the output/input state of the stream may corrupt this

  object, so care must be taken when using this method.

- <span id="deflatedecoder-reset"></span>`fn reset(&mut self, w: W) -> io::Result<W>`

  Resets the state of this decoder entirely, swapping out the output

  stream for another.

  

  This function will finish encoding the current stream into the current

  output stream before swapping out the two output streams.

  

  This will then reset the internal state of this decoder and replace the

  output stream with the one provided, returning the previous output

  stream. Future data written to this decoder will be decompressed into

  the output stream `w`.

  

  # Errors

  

  This function will perform I/O to finish the stream, and if that I/O

  returns an error then that will be returned from this function.

- <span id="deflatedecoder-try-finish"></span>`fn try_finish(&mut self) -> io::Result<()>`

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

- <span id="deflatedecoder-finish"></span>`fn finish(self) -> io::Result<W>`

  Consumes this encoder, flushing the output stream.

  

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

- <span id="deflatedecoder-total-in"></span>`fn total_in(&self) -> u64`

  Returns the number of bytes that the decompressor has consumed for

  decompression.

  

  Note that this will likely be smaller than the number of bytes

  successfully written to this stream due to internal buffering.

- <span id="deflatedecoder-total-out"></span>`fn total_out(&self) -> u64`

  Returns the number of bytes that the decompressor has written to its

  output stream.

#### Trait Implementations

##### `impl<W: fmt::Debug + Write> Debug for DeflateDecoder<W>`

- <span id="deflatedecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Read + Write> Read for DeflateDecoder<W>`

- <span id="deflatedecoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Write> Write for DeflateDecoder<W>`

- <span id="deflatedecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="deflatedecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `DeflateEncoder<W: Write>`

```rust
struct DeflateEncoder<W: Write> {
    inner: zio::Writer<W, crate::Compress>,
}
```

A DEFLATE encoder, or compressor.

This structure implements a [`Write`]() interface and takes a stream of
uncompressed data, writing the compressed data to the wrapped writer.

# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::DeflateEncoder;

// Vec<u8> implements Write to print the compressed bytes of sample string
fn main() {

let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World").unwrap();
println!("{:?}", e.finish().unwrap());
}
```

#### Implementations

- <span id="deflateencoder-new"></span>`fn new(w: W, level: crate::Compression) -> DeflateEncoder<W>` — [`Compression`](../index.md#compression), [`DeflateEncoder`](../deflate/write/index.md#deflateencoder)

  Creates a new encoder which will write compressed data to the stream

  given at the given compression level.

  

  When this encoder is dropped or unwrapped the final pieces of data will

  be flushed.

- <span id="deflateencoder-get-ref"></span>`fn get_ref(&self) -> &W`

  Acquires a reference to the underlying writer.

- <span id="deflateencoder-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Acquires a mutable reference to the underlying writer.

  

  Note that mutating the output/input state of the stream may corrupt this

  object, so care must be taken when using this method.

- <span id="deflateencoder-reset"></span>`fn reset(&mut self, w: W) -> io::Result<W>`

  Resets the state of this encoder entirely, swapping out the output

  stream for another.

  

  This function will finish encoding the current stream into the current

  output stream before swapping out the two output streams. If the stream

  cannot be finished an error is returned.

  

  After the current stream has been finished, this will reset the internal

  state of this encoder and replace the output stream with the one

  provided, returning the previous output stream. Future data written to

  this encoder will be the compressed into the stream `w` provided.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="deflateencoder-try-finish"></span>`fn try_finish(&mut self) -> io::Result<()>`

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

- <span id="deflateencoder-finish"></span>`fn finish(self) -> io::Result<W>`

  Consumes this encoder, flushing the output stream.

  

  This will flush the underlying data stream, close off the compressed

  stream and, if successful, return the contained writer.

  

  Note that this function may not be suitable to call in a situation where

  the underlying stream is an asynchronous I/O stream. To finish a stream

  the `try_finish` (or `shutdown`) method should be used instead. To

  re-acquire ownership of a stream it is safe to call this method after

  `try_finish` or `shutdown` has returned `Ok`.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="deflateencoder-flush-finish"></span>`fn flush_finish(self) -> io::Result<W>`

  Consumes this encoder, flushing the output stream.

  

  This will flush the underlying data stream and then return the contained

  writer if the flush succeeded.

  The compressed stream will not closed but only flushed. This

  means that obtained byte array can by extended by another deflated

  stream. To close the stream add the two bytes 0x3 and 0x0.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="deflateencoder-total-in"></span>`fn total_in(&self) -> u64`

  Returns the number of bytes that have been written to this compressor.

  

  Note that not all bytes written to this object may be accounted for,

  there may still be some active buffering.

- <span id="deflateencoder-total-out"></span>`fn total_out(&self) -> u64`

  Returns the number of bytes that the compressor has produced.

  

  Note that not all bytes may have been written yet, some may still be

  buffered.

#### Trait Implementations

##### `impl<W: fmt::Debug + Write> Debug for DeflateEncoder<W>`

- <span id="deflateencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Read + Write> Read for DeflateEncoder<W>`

- <span id="deflateencoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Write> Write for DeflateEncoder<W>`

- <span id="deflateencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="deflateencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

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

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](../gz/write/index.md)
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

- <span id="gzdecoder-new"></span>`fn new(w: W) -> GzDecoder<W>` — [`GzDecoder`](../gz/write/index.md#gzdecoder)

  Creates a new decoder which will write uncompressed data to the stream.

  

  When this encoder is dropped or unwrapped the final pieces of data will

  be flushed.

- <span id="gzdecoder-header"></span>`fn header(&self) -> Option<&GzHeader>` — [`GzHeader`](../gz/index.md#gzheader)

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

- <span id="gzencoder-new"></span>`fn new(w: W, level: Compression) -> GzEncoder<W>` — [`Compression`](../index.md#compression), [`GzEncoder`](../gz/write/index.md#gzencoder)

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

### `MultiGzDecoder<W: Write>`

```rust
struct MultiGzDecoder<W: Write> {
    inner: GzDecoder<W>,
}
```

A gzip streaming decoder that decodes a [gzip file] with multiple members.

This structure exposes a [`Write`](../../embedded_hal/index.md) interface that will consume compressed data and
write uncompressed data to the underlying writer.

A gzip file consists of a series of *members* concatenated one after another.
`MultiGzDecoder` decodes all members of a file and writes them to the
underlying writer one after another.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).


#### Implementations

- <span id="multigzdecoder-new"></span>`fn new(w: W) -> MultiGzDecoder<W>` — [`MultiGzDecoder`](../gz/write/index.md#multigzdecoder)

  Creates a new decoder which will write uncompressed data to the stream.

  If the gzip stream contains multiple members all will be decoded.

- <span id="multigzdecoder-header"></span>`fn header(&self) -> Option<&GzHeader>` — [`GzHeader`](../gz/index.md#gzheader)

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

### `ZlibDecoder<W: Write>`

```rust
struct ZlibDecoder<W: Write> {
    inner: zio::Writer<W, crate::Decompress>,
}
```

A ZLIB decoder, or decompressor.

This structure implements a [`Write`]() and will emit a stream of decompressed
data when fed a stream of compressed data.

After decoding a single member of the ZLIB data this writer will return the number of bytes up
to the end of the ZLIB member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the ZLIB member.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::write::ZlibDecoder;

fn main() {
   let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut z = ZlibDecoder::new(writer);
   z.write_all(&bytes[..])?;
   writer = z.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

#### Implementations

- <span id="zlibdecoder-new"></span>`fn new(w: W) -> ZlibDecoder<W>` — [`ZlibDecoder`](../zlib/write/index.md#zlibdecoder)

  Creates a new decoder which will write uncompressed data to the stream.

  

  When this decoder is dropped or unwrapped the final pieces of data will

  be flushed.

- <span id="zlibdecoder-new-with-decompress"></span>`fn new_with_decompress(w: W, decompression: Decompress) -> ZlibDecoder<W>` — [`Decompress`](../mem/index.md#decompress), [`ZlibDecoder`](../zlib/write/index.md#zlibdecoder)

  Creates a new decoder which will write uncompressed data to the stream `w`

  using the given `decompression` settings.

  

  When this decoder is dropped or unwrapped the final pieces of data will

  be flushed.

- <span id="zlibdecoder-get-ref"></span>`fn get_ref(&self) -> &W`

  Acquires a reference to the underlying writer.

- <span id="zlibdecoder-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Acquires a mutable reference to the underlying writer.

  

  Note that mutating the output/input state of the stream may corrupt this

  object, so care must be taken when using this method.

- <span id="zlibdecoder-reset"></span>`fn reset(&mut self, w: W) -> io::Result<W>`

  Resets the state of this decoder entirely, swapping out the output

  stream for another.

  

  This will reset the internal state of this decoder and replace the

  output stream with the one provided, returning the previous output

  stream. Future data written to this decoder will be decompressed into

  the output stream `w`.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="zlibdecoder-try-finish"></span>`fn try_finish(&mut self) -> io::Result<()>`

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

- <span id="zlibdecoder-finish"></span>`fn finish(self) -> io::Result<W>`

  Consumes this encoder, flushing the output stream.

  

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

- <span id="zlibdecoder-total-in"></span>`fn total_in(&self) -> u64`

  Returns the number of bytes that the decompressor has consumed for

  decompression.

  

  Note that this will likely be smaller than the number of bytes

  successfully written to this stream due to internal buffering.

- <span id="zlibdecoder-total-out"></span>`fn total_out(&self) -> u64`

  Returns the number of bytes that the decompressor has written to its

  output stream.

#### Trait Implementations

##### `impl<W: fmt::Debug + Write> Debug for ZlibDecoder<W>`

- <span id="zlibdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Read + Write> Read for ZlibDecoder<W>`

- <span id="zlibdecoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Write> Write for ZlibDecoder<W>`

- <span id="zlibdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="zlibdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `ZlibEncoder<W: Write>`

```rust
struct ZlibEncoder<W: Write> {
    inner: zio::Writer<W, crate::Compress>,
}
```

A ZLIB encoder, or compressor.

This structure implements a [`Write`]() interface and takes a stream of
uncompressed data, writing the compressed data to the wrapped writer.

# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

// Vec<u8> implements Write, assigning the compressed bytes of sample string

fn zlib_encoding() -> std::io::Result<()> {
let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World")?;
let compressed = e.finish()?;
Ok(())
}
```

#### Implementations

- <span id="zlibencoder-new"></span>`fn new(w: W, level: crate::Compression) -> ZlibEncoder<W>` — [`Compression`](../index.md#compression), [`ZlibEncoder`](../zlib/write/index.md#zlibencoder)

  Creates a new encoder which will write compressed data to the stream

  given at the given compression level.

  

  When this encoder is dropped or unwrapped the final pieces of data will

  be flushed.

- <span id="zlibencoder-new-with-compress"></span>`fn new_with_compress(w: W, compression: Compress) -> ZlibEncoder<W>` — [`Compress`](../mem/index.md#compress), [`ZlibEncoder`](../zlib/write/index.md#zlibencoder)

  Creates a new encoder which will write compressed data to the stream

  `w` with the given `compression` settings.

- <span id="zlibencoder-get-ref"></span>`fn get_ref(&self) -> &W`

  Acquires a reference to the underlying writer.

- <span id="zlibencoder-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Acquires a mutable reference to the underlying writer.

  

  Note that mutating the output/input state of the stream may corrupt this

  object, so care must be taken when using this method.

- <span id="zlibencoder-reset"></span>`fn reset(&mut self, w: W) -> io::Result<W>`

  Resets the state of this encoder entirely, swapping out the output

  stream for another.

  

  This function will finish encoding the current stream into the current

  output stream before swapping out the two output streams.

  

  After the current stream has been finished, this will reset the internal

  state of this encoder and replace the output stream with the one

  provided, returning the previous output stream. Future data written to

  this encoder will be the compressed into the stream `w` provided.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="zlibencoder-try-finish"></span>`fn try_finish(&mut self) -> io::Result<()>`

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

- <span id="zlibencoder-finish"></span>`fn finish(self) -> io::Result<W>`

  Consumes this encoder, flushing the output stream.

  

  This will flush the underlying data stream, close off the compressed

  stream and, if successful, return the contained writer.

  

  Note that this function may not be suitable to call in a situation where

  the underlying stream is an asynchronous I/O stream. To finish a stream

  the `try_finish` (or `shutdown`) method should be used instead. To

  re-acquire ownership of a stream it is safe to call this method after

  `try_finish` or `shutdown` has returned `Ok`.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="zlibencoder-flush-finish"></span>`fn flush_finish(self) -> io::Result<W>`

  Consumes this encoder, flushing the output stream.

  

  This will flush the underlying data stream and then return the contained

  writer if the flush succeeded.

  The compressed stream will not be closed but only flushed. This

  means that obtained byte array can by extended by another deflated

  stream. To close the stream add the two bytes 0x3 and 0x0.

  

  # Errors

  

  This function will perform I/O to complete this stream, and any I/O

  errors which occur will be returned from this function.

- <span id="zlibencoder-total-in"></span>`fn total_in(&self) -> u64`

  Returns the number of bytes that have been written to this compressor.

  

  Note that not all bytes written to this object may be accounted for,

  there may still be some active buffering.

- <span id="zlibencoder-total-out"></span>`fn total_out(&self) -> u64`

  Returns the number of bytes that the compressor has produced.

  

  Note that not all bytes may have been written yet, some may still be

  buffered.

#### Trait Implementations

##### `impl<W: fmt::Debug + Write> Debug for ZlibEncoder<W>`

- <span id="zlibencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Read + Write> Read for ZlibEncoder<W>`

- <span id="zlibencoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Write> Write for ZlibEncoder<W>`

- <span id="zlibencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="zlibencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

