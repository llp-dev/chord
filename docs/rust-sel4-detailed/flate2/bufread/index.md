*[flate2](../index.md) / [bufread](index.md)*

---

# Module `bufread`

Types which operate over `BufRead` streams, both encoders and decoders for
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

- <span id="deflatedecoder-new"></span>`fn new(r: R) -> DeflateDecoder<R>` — [`DeflateDecoder`](../deflate/bufread/index.md#deflatedecoder)

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

- <span id="deflateencoder-new"></span>`fn new(r: R, level: crate::Compression) -> DeflateEncoder<R>` — [`Compression`](../index.md#compression), [`DeflateEncoder`](../deflate/bufread/index.md#deflateencoder)

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

### `GzDecoder<R>`

```rust
struct GzDecoder<R> {
    state: GzState,
    reader: crate::crc::CrcReader<deflate::bufread::DeflateDecoder<R>>,
    multi: bool,
}
```

A decoder for a single member of a [gzip file].

This structure implements a `Read` interface. When read from, it reads
compressed data from the underlying `BufRead` and provides the uncompressed data.

After reading a single member of the gzip data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](../gz/bufread/index.md)
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).



# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::bufread::GzDecoder;

fn main() {
  let mut e = GzEncoder::new(Vec::new(), Compression::default());
  e.write_all(b"Hello World").unwrap();
  let bytes = e.finish().unwrap();
  println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = GzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- <span id="gzdecoder-new"></span>`fn new(r: R) -> GzDecoder<R>` — [`GzDecoder`](../gz/bufread/index.md#gzdecoder)

  Creates a new decoder from the given reader, immediately parsing the

  gzip header.

- <span id="gzdecoder-multi"></span>`fn multi(self, flag: bool) -> GzDecoder<R>` — [`GzDecoder`](../gz/bufread/index.md#gzdecoder)

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for GzDecoder<R>`

- <span id="gzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for GzDecoder<R>`

- <span id="gzdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: BufRead + Write> Write for GzDecoder<R>`

- <span id="gzdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="gzdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `GzEncoder<R>`

```rust
struct GzEncoder<R> {
    inner: deflate::bufread::DeflateEncoder<crate::crc::CrcReader<R>>,
    header: Vec<u8>,
    pos: usize,
    eof: bool,
}
```

A gzip streaming encoder

This structure implements a `Read` interface. When read from, it reads
uncompressed data from the underlying `BufRead` and provides the compressed data.


# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::bufread::GzEncoder;
use std::fs::File;
use std::io::BufReader;

// Opens sample file, compresses the contents and returns a Vector or error
// File wrapped in a BufReader implements BufRead

fn open_hello_world() -> io::Result<Vec<u8>> {
    let f = File::open("examples/hello_world.txt")?;
    let b = BufReader::new(f);
    let mut gz = GzEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;
    Ok(buffer)
}
```

#### Implementations

- <span id="gzencoder-new"></span>`fn new(r: R, level: Compression) -> GzEncoder<R>` — [`Compression`](../index.md#compression), [`GzEncoder`](../gz/bufread/index.md#gzencoder)

  Creates a new encoder which will use the given compression level.

  

  The encoder is not configured specially for the emitted header. For

  header configuration, see the `GzBuilder` type.

  

  The data read from the stream `r` will be compressed and available

  through the returned reader.

- <span id="gzencoder-read-footer"></span>`fn read_footer(&mut self, into: &mut [u8]) -> io::Result<usize>`

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for GzEncoder<R>`

- <span id="gzencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for GzEncoder<R>`

- <span id="gzencoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: BufRead + Write> Write for GzEncoder<R>`

- <span id="gzencoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="gzencoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `MultiGzDecoder<R>`

```rust
struct MultiGzDecoder<R>(GzDecoder<R>);
```

A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

This structure implements a `Read` interface. When read from, it reads
compressed data from the underlying `BufRead` and provides the uncompressed data.

A gzip file consists of a series of *members* concatenated one after another.
MultiGzDecoder decodes all members from the data and only returns Ok(0) when the
underlying reader does. For a file, this reads to the end of the file.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).



# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::bufread::MultiGzDecoder;

fn main() {
  let mut e = GzEncoder::new(Vec::new(), Compression::default());
  e.write_all(b"Hello World").unwrap();
  let bytes = e.finish().unwrap();
  println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = MultiGzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- <span id="multigzdecoder-new"></span>`fn new(r: R) -> MultiGzDecoder<R>` — [`MultiGzDecoder`](../gz/bufread/index.md#multigzdecoder)

  Creates a new decoder from the given reader, immediately parsing the

  (first) gzip header. If the gzip stream contains multiple members all will

  be decoded.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for MultiGzDecoder<R>`

- <span id="multigzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for MultiGzDecoder<R>`

- <span id="multigzdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

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

- <span id="zlibdecoder-new"></span>`fn new(r: R) -> ZlibDecoder<R>` — [`ZlibDecoder`](../zlib/bufread/index.md#zlibdecoder)

  Creates a new decoder which will decompress data read from the given

  stream.

- <span id="zlibdecoder-new-with-decompress"></span>`fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>` — [`Decompress`](../mem/index.md#decompress), [`ZlibDecoder`](../zlib/bufread/index.md#zlibdecoder)

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

- <span id="zlibencoder-new"></span>`fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>` — [`Compression`](../index.md#compression), [`ZlibEncoder`](../zlib/bufread/index.md#zlibencoder)

  Creates a new encoder which will read uncompressed data from the given

  stream and emit the compressed stream.

- <span id="zlibencoder-new-with-compress"></span>`fn new_with_compress(r: R, compression: Compress) -> ZlibEncoder<R>` — [`Compress`](../mem/index.md#compress), [`ZlibEncoder`](../zlib/bufread/index.md#zlibencoder)

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

