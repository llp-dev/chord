*[flate2](../../index.md) / [deflate](../index.md) / [read](index.md)*

---

# Module `read`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DeflateEncoder`](#deflateencoder) | struct | A DEFLATE encoder, or compressor. |
| [`DeflateDecoder`](#deflatedecoder) | struct | A DEFLATE decoder, or decompressor. |

## Structs

### `DeflateEncoder<R>`

```rust
struct DeflateEncoder<R> {
    inner: bufread::DeflateEncoder<crate::bufreader::BufReader<R>>,
}
```

A DEFLATE encoder, or compressor.

This structure implements a `Read` interface. When read from, it reads
uncompressed data from the underlying `Read` and provides the compressed data.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::read::DeflateEncoder;

fn main() {
   println!("{:?}", deflateencoder_read_hello_world().unwrap());
}

// Return a vector containing the Deflate compressed version of hello world
fn deflateencoder_read_hello_world() -> io::Result<Vec<u8>> {
   let mut ret_vec = Vec::new();
   let c = b"hello world";
   let mut deflater = DeflateEncoder::new(&c[..], Compression::fast());
   deflater.read_to_end(&mut ret_vec)?;
   Ok(ret_vec)
}
```

#### Implementations

- <span id="deflateencoder-new"></span>`fn new(r: R, level: crate::Compression) -> DeflateEncoder<R>` — [`Compression`](../../index.md#compression), [`DeflateEncoder`](#deflateencoder)

  Creates a new encoder which will read uncompressed data from the given

  stream and emit the compressed stream.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for DeflateEncoder<R>`

- <span id="deflateencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for DeflateEncoder<R>`

- <span id="deflateencoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Read + Write> Write for DeflateEncoder<W>`

- <span id="deflateencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="deflateencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `DeflateDecoder<R>`

```rust
struct DeflateDecoder<R> {
    inner: bufread::DeflateDecoder<crate::bufreader::BufReader<R>>,
}
```

A DEFLATE decoder, or decompressor.

This structure implements a `Read` interface. When read from, it reads
compressed data from the underlying `Read` and provides the uncompressed data.

After reading a single member of the DEFLATE data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`DeflateDecoder` may have read additional bytes past the end of the DEFLATE data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::DeflateDecoder` instead.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;

fn main() {
   let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   println!("{}", decode_reader(bytes).unwrap());
}
// Uncompresses a Deflate Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read
fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut deflater = DeflateDecoder::new(&bytes[..]);
   let mut s = String::new();
   deflater.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- <span id="deflatedecoder-new"></span>`fn new(r: R) -> DeflateDecoder<R>` — [`DeflateDecoder`](#deflatedecoder)

  Creates a new decoder which will decompress data read from the given

  stream.

- <span id="deflatedecoder-new-with-buf"></span>`fn new_with_buf(r: R, buf: Vec<u8>) -> DeflateDecoder<R>` — [`DeflateDecoder`](#deflatedecoder)

  Same as `new`, but the intermediate buffer for data is specified.

  

  Note that the capacity of the intermediate buffer is never increased,

  and it is recommended for it to be large.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for DeflateDecoder<R>`

- <span id="deflatedecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for DeflateDecoder<R>`

- <span id="deflatedecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Read + Write> Write for DeflateDecoder<W>`

- <span id="deflatedecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="deflatedecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

