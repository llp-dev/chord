*[flate2](../../index.md) / [zlib](../index.md) / [bufread](index.md)*

---

# Module `bufread`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ZlibEncoder`](#zlibencoder) | struct | A ZLIB encoder, or compressor. |
| [`ZlibDecoder`](#zlibdecoder) | struct | A ZLIB decoder, or decompressor. |
| [`reset_encoder_data`](#reset-encoder-data) | fn |  |
| [`reset_decoder_data`](#reset-decoder-data) | fn |  |

## Structs

### `ZlibEncoder<R>`

```rust
struct ZlibEncoder<R> {
    obj: R,
    data: crate::Compress,
}
```

A ZLIB encoder, or compressor.

This structure implements a `Read` interface. When read from, it reads
uncompressed data from the underlying `BufRead` and provides the compressed data.


# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::bufread::ZlibEncoder;
use std::fs::File;
use std::io::BufReader;

// Use a buffered file to compress contents into a Vec<u8>

fn open_hello_world() -> std::io::Result<Vec<u8>> {
let f = File::open("examples/hello_world.txt")?;
let b = BufReader::new(f);
let mut z = ZlibEncoder::new(b, Compression::fast());
let mut buffer = Vec::new();
z.read_to_end(&mut buffer)?;
Ok(buffer)
}
```

#### Implementations

- <span id="zlibencoder-new"></span>`fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>` — [`Compression`](../../index.md#compression), [`ZlibEncoder`](#zlibencoder)

  Creates a new encoder which will read uncompressed data from the given

  stream and emit the compressed stream.

- <span id="zlibencoder-new-with-compress"></span>`fn new_with_compress(r: R, compression: Compress) -> ZlibEncoder<R>` — [`Compress`](../../mem/index.md#compress), [`ZlibEncoder`](#zlibencoder)

  Creates a new encoder with the given `compression` settings which will

  read uncompressed data from the given stream `r` and emit the compressed stream.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for ZlibEncoder<R>`

- <span id="zlibencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for ZlibEncoder<R>`

- <span id="zlibencoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<R: BufRead + Write> Write for ZlibEncoder<R>`

- <span id="zlibencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="zlibencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `ZlibDecoder<R>`

```rust
struct ZlibDecoder<R> {
    obj: R,
    data: crate::Decompress,
}
```

A ZLIB decoder, or decompressor.

This structure implements a `Read` interface. When read from, it reads
compressed data from the underlying `BufRead` and provides the uncompressed data.

After reading a single member of the ZLIB data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.


# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

fn main() {
let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World").unwrap();
let bytes = e.finish().unwrap();
println!("{}", decode_bufreader(bytes).unwrap());
}

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead

fn decode_bufreader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
```

#### Implementations

- <span id="zlibdecoder-new"></span>`fn new(r: R) -> ZlibDecoder<R>` — [`ZlibDecoder`](#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream.

- <span id="zlibdecoder-new-with-decompress"></span>`fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>` — [`Decompress`](../../mem/index.md#decompress), [`ZlibDecoder`](#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream, using the given `decompression` settings.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for ZlibDecoder<R>`

- <span id="zlibdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for ZlibDecoder<R>`

- <span id="zlibdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: BufRead + Write> Write for ZlibDecoder<R>`

- <span id="zlibdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="zlibdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

## Functions

### `reset_encoder_data`

```rust
fn reset_encoder_data<R>(zlib: &mut ZlibEncoder<R>)
```

### `reset_decoder_data`

```rust
fn reset_decoder_data<R>(zlib: &mut ZlibDecoder<R>)
```

