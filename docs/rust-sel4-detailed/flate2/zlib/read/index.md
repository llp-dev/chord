*[flate2](../../index.md) / [zlib](../index.md) / [read](index.md)*

---

# Module `read`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ZlibEncoder`](#zlibencoder) | struct | A ZLIB encoder, or compressor. |
| [`ZlibDecoder`](#zlibdecoder) | struct | A ZLIB decoder, or decompressor. |

## Structs

### `ZlibEncoder<R>`

```rust
struct ZlibEncoder<R> {
    inner: bufread::ZlibEncoder<crate::bufreader::BufReader<R>>,
}
```

A ZLIB encoder, or compressor.

This structure implements a `Read` interface. When read from, it reads
uncompressed data from the underlying `Read` and provides the compressed data.

# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::read::ZlibEncoder;
use std::fs::File;

// Open example file and compress the contents using Read interface

fn open_hello_world() -> std::io::Result<Vec<u8>> {
let f = File::open("examples/hello_world.txt")?;
let mut z = ZlibEncoder::new(f, Compression::fast());
let mut buffer = Vec::new();
z.read_to_end(&mut buffer)?;
Ok(buffer)
}
```

#### Implementations

- <span id="zlibencoder-new"></span>`fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>` — [`Compression`](../../index.md#compression), [`ZlibEncoder`](#zlibencoder)

  Creates a new encoder which will read uncompressed data from the given

  stream and emit the compressed stream.

- <span id="zlibencoder-new-with-compress"></span>`fn new_with_compress(r: R, compression: crate::Compress) -> ZlibEncoder<R>` — [`Compress`](../../mem/index.md#compress), [`ZlibEncoder`](#zlibencoder)

  Creates a new encoder with the given `compression` settings which will

  read uncompressed data from the given stream `r` and emit the compressed stream.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for ZlibEncoder<R>`

- <span id="zlibencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for ZlibEncoder<R>`

- <span id="zlibencoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl<W: Read + Write> Write for ZlibEncoder<W>`

- <span id="zlibencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="zlibencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `ZlibDecoder<R>`

```rust
struct ZlibDecoder<R> {
    inner: bufread::ZlibDecoder<crate::bufreader::BufReader<R>>,
}
```

A ZLIB decoder, or decompressor.

This structure implements a `Read` interface. When read from, it reads
compressed data from the underlying `Read` and provides the uncompressed data.

After reading a single member of the ZLIB data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`ZlibDecoder` may have read additional bytes past the end of the ZLIB data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::ZlibDecoder` instead.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

fn main() {
let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World").unwrap();
let bytes = e.finish().unwrap();
println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
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

- <span id="zlibdecoder-new-with-buf"></span>`fn new_with_buf(r: R, buf: Vec<u8>) -> ZlibDecoder<R>` — [`ZlibDecoder`](#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream `r`, using `buf` as backing to speed up reading.

  

  Note that the specified buffer will only be used up to its current

  length. The buffer's capacity will also not grow over time.

- <span id="zlibdecoder-new-with-decompress"></span>`fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>` — [`Decompress`](../../mem/index.md#decompress), [`ZlibDecoder`](#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream `r`, along with `decompression` settings.

- <span id="zlibdecoder-new-with-decompress-and-buf"></span>`fn new_with_decompress_and_buf(r: R, buf: Vec<u8>, decompression: Decompress) -> ZlibDecoder<R>` — [`Decompress`](../../mem/index.md#decompress), [`ZlibDecoder`](#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream `r`, using `buf` as backing to speed up reading,

  along with `decompression` settings to configure decoder.

  

  Note that the specified buffer will only be used up to its current

  length. The buffer's capacity will also not grow over time.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for ZlibDecoder<R>`

- <span id="zlibdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for ZlibDecoder<R>`

- <span id="zlibdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: Read + Write> Write for ZlibDecoder<R>`

- <span id="zlibdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="zlibdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

