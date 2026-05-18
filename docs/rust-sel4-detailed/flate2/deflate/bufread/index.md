*[flate2](../../index.md) / [deflate](../index.md) / [bufread](index.md)*

---

# Module `bufread`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DeflateEncoder`](#deflateencoder) | struct | A DEFLATE encoder, or compressor. |
| [`DeflateDecoder`](#deflatedecoder) | struct | A DEFLATE decoder, or decompressor. |
| [`reset_encoder_data`](#reset-encoder-data) | fn |  |
| [`reset_decoder_data`](#reset-decoder-data) | fn |  |

## Structs

### `DeflateEncoder<R>`

```rust
struct DeflateEncoder<R> {
    obj: R,
    data: crate::Compress,
}
```

A DEFLATE encoder, or compressor.

This structure implements a `Read` interface. When read from, it reads
uncompressed data from the underlying `BufRead` and provides the compressed data.


# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::bufread::DeflateEncoder;
use std::fs::File;
use std::io::BufReader;

fn main() {
   println!("{:?}", open_hello_world().unwrap());
}

// Opens sample file, compresses the contents and returns a Vector
fn open_hello_world() -> io::Result<Vec<u8>> {
   let f = File::open("examples/hello_world.txt")?;
   let b = BufReader::new(f);
   let mut deflater = DeflateEncoder::new(b, Compression::fast());
   let mut buffer = Vec::new();
   deflater.read_to_end(&mut buffer)?;
   Ok(buffer)
}
```

#### Implementations

- <span id="deflateencoder-new"></span>`fn new(r: R, level: crate::Compression) -> DeflateEncoder<R>` — [`Compression`](../../index.md#compression), [`DeflateEncoder`](#deflateencoder)

  Creates a new encoder which will read uncompressed data from the given

  stream and emit the compressed stream.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for DeflateEncoder<R>`

- <span id="deflateencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for DeflateEncoder<R>`

- <span id="deflateencoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: BufRead + Write> Write for DeflateEncoder<W>`

- <span id="deflateencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="deflateencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `DeflateDecoder<R>`

```rust
struct DeflateDecoder<R> {
    obj: R,
    data: crate::Decompress,
}
```

A DEFLATE decoder, or decompressor.

This structure implements a `Read` interface. When read from, it reads
compressed data from the underlying `BufRead` and provides the uncompressed data.

After reading a single member of the DEFLATE data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.


# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::bufread::DeflateDecoder;

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

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for DeflateDecoder<R>`

- <span id="deflatedecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for DeflateDecoder<R>`

- <span id="deflatedecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<W: BufRead + Write> Write for DeflateDecoder<W>`

- <span id="deflatedecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="deflatedecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

## Functions

### `reset_encoder_data`

```rust
fn reset_encoder_data<R>(zlib: &mut DeflateEncoder<R>)
```

### `reset_decoder_data`

```rust
fn reset_decoder_data<R>(zlib: &mut DeflateDecoder<R>)
```

