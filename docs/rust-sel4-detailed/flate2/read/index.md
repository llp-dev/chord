*[flate2](../index.md) / [read](index.md)*

---

# Module `read`

Types which operate over `Read` streams, both encoders and decoders for
various formats.

Note that the `read` decoder types may read past the end of the compressed
data while decoding. If the caller requires subsequent reads to start
immediately following the compressed data  wrap the `Read` type in a
[`BufReader`](../bufreader/index.md) and use the `BufReader` with the equivalent decoder from the
`bufread` module and also for the subsequent reads.



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

- <span id="deflatedecoder-new"></span>`fn new(r: R) -> DeflateDecoder<R>` — [`DeflateDecoder`](../deflate/read/index.md#deflatedecoder)

  Creates a new decoder which will decompress data read from the given

  stream.

- <span id="deflatedecoder-new-with-buf"></span>`fn new_with_buf(r: R, buf: Vec<u8>) -> DeflateDecoder<R>` — [`DeflateDecoder`](../deflate/read/index.md#deflatedecoder)

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

- <span id="deflateencoder-new"></span>`fn new(r: R, level: crate::Compression) -> DeflateEncoder<R>` — [`Compression`](../index.md#compression), [`DeflateEncoder`](../deflate/read/index.md#deflateencoder)

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

### `GzDecoder<R>`

```rust
struct GzDecoder<R> {
    inner: bufread::GzDecoder<crate::bufreader::BufReader<R>>,
}
```

A decoder for a single member of a [gzip file].

This structure implements a [`Read`](../../embedded_hal/index.md) interface. When read from, it reads
compressed data from the underlying [`Read`](../../embedded_hal/index.md) and provides the uncompressed data.

After reading a single member of the gzip data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`GzDecoder` may have read additional bytes past the end of the gzip data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::GzDecoder` instead.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](../gz/read/index.md)
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;

fn main() {
   let mut e = GzEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = GzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- <span id="gzdecoder-new"></span>`fn new(r: R) -> GzDecoder<R>` — [`GzDecoder`](../gz/read/index.md#gzdecoder)

  Creates a new decoder from the given reader, immediately parsing the

  gzip header.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for GzDecoder<R>`

- <span id="gzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for GzDecoder<R>`

- <span id="gzdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: Read + Write> Write for GzDecoder<R>`

- <span id="gzdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="gzdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `GzEncoder<R>`

```rust
struct GzEncoder<R> {
    inner: bufread::GzEncoder<crate::bufreader::BufReader<R>>,
}
```

A gzip streaming encoder

This structure implements a `Read` interface. When read from, it reads
uncompressed data from the underlying `Read` and provides the compressed data.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::read::GzEncoder;

// Return a vector containing the GZ compressed version of hello world

fn gzencode_hello_world() -> io::Result<Vec<u8>> {
    let mut ret_vec = Vec::new();
    let bytestring = b"hello world";
    let mut gz = GzEncoder::new(&bytestring[..], Compression::fast());
    gz.read_to_end(&mut ret_vec)?;
    Ok(ret_vec)
}
```

#### Implementations

- <span id="gzencoder-new"></span>`fn new(r: R, level: Compression) -> GzEncoder<R>` — [`Compression`](../index.md#compression), [`GzEncoder`](../gz/read/index.md#gzencoder)

  Creates a new encoder which will use the given compression level.

  

  The encoder is not configured specially for the emitted header. For

  header configuration, see the `GzBuilder` type.

  

  The data read from the stream `r` will be compressed and available

  through the returned reader.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for GzEncoder<R>`

- <span id="gzencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for GzEncoder<R>`

- <span id="gzencoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: Read + Write> Write for GzEncoder<R>`

- <span id="gzencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="gzencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `MultiGzDecoder<R>`

```rust
struct MultiGzDecoder<R> {
    inner: bufread::MultiGzDecoder<crate::bufreader::BufReader<R>>,
}
```

A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

This structure implements a [`Read`](../../embedded_hal/index.md) interface. When read from, it reads
compressed data from the underlying [`Read`](../../embedded_hal/index.md) and provides the uncompressed
data.

A gzip file consists of a series of *members* concatenated one after another.
MultiGzDecoder decodes all members of a file and returns Ok(0) once the
underlying reader does.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::MultiGzDecoder;

fn main() {
   let mut e = GzEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = MultiGzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- <span id="multigzdecoder-new"></span>`fn new(r: R) -> MultiGzDecoder<R>` — [`MultiGzDecoder`](../gz/read/index.md#multigzdecoder)

  Creates a new decoder from the given reader, immediately parsing the

  (first) gzip header. If the gzip stream contains multiple members all will

  be decoded.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for MultiGzDecoder<R>`

- <span id="multigzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for MultiGzDecoder<R>`

- <span id="multigzdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: Read + Write> Write for MultiGzDecoder<R>`

- <span id="multigzdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="multigzdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

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

- <span id="zlibdecoder-new"></span>`fn new(r: R) -> ZlibDecoder<R>` — [`ZlibDecoder`](../zlib/read/index.md#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream.

- <span id="zlibdecoder-new-with-buf"></span>`fn new_with_buf(r: R, buf: Vec<u8>) -> ZlibDecoder<R>` — [`ZlibDecoder`](../zlib/read/index.md#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream `r`, using `buf` as backing to speed up reading.

  

  Note that the specified buffer will only be used up to its current

  length. The buffer's capacity will also not grow over time.

- <span id="zlibdecoder-new-with-decompress"></span>`fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>` — [`Decompress`](../mem/index.md#decompress), [`ZlibDecoder`](../zlib/read/index.md#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream `r`, along with `decompression` settings.

- <span id="zlibdecoder-new-with-decompress-and-buf"></span>`fn new_with_decompress_and_buf(r: R, buf: Vec<u8>, decompression: Decompress) -> ZlibDecoder<R>` — [`Decompress`](../mem/index.md#decompress), [`ZlibDecoder`](../zlib/read/index.md#zlibdecoder)

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

- <span id="zlibencoder-new"></span>`fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>` — [`Compression`](../index.md#compression), [`ZlibEncoder`](../zlib/read/index.md#zlibencoder)

  Creates a new encoder which will read uncompressed data from the given

  stream and emit the compressed stream.

- <span id="zlibencoder-new-with-compress"></span>`fn new_with_compress(r: R, compression: crate::Compress) -> ZlibEncoder<R>` — [`Compress`](../mem/index.md#compress), [`ZlibEncoder`](../zlib/read/index.md#zlibencoder)

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

