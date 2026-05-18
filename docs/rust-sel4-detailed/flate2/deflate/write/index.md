*[flate2](../../index.md) / [deflate](../index.md) / [write](index.md)*

---

# Module `write`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DeflateEncoder`](#deflateencoder) | struct | A DEFLATE encoder, or compressor. |
| [`DeflateDecoder`](#deflatedecoder) | struct | A DEFLATE decoder, or decompressor. |

## Structs

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

- <span id="deflateencoder-new"></span>`fn new(w: W, level: crate::Compression) -> DeflateEncoder<W>` — [`Compression`](../../index.md#compression), [`DeflateEncoder`](#deflateencoder)

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

- <span id="deflatedecoder-new"></span>`fn new(w: W) -> DeflateDecoder<W>` — [`DeflateDecoder`](#deflatedecoder)

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

