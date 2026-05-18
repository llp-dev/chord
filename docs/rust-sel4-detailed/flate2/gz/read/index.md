*[flate2](../../index.md) / [gz](../index.md) / [read](index.md)*

---

# Module `read`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GzEncoder`](#gzencoder) | struct | A gzip streaming encoder |
| [`GzDecoder`](#gzdecoder) | struct | A decoder for a single member of a [gzip file]. |
| [`MultiGzDecoder`](#multigzdecoder) | struct | A gzip streaming decoder that decodes a [gzip file] that may have multiple members. |
| [`gz_encoder`](#gz-encoder) | fn |  |

## Structs

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

- <span id="gzencoder-new"></span>`fn new(r: R, level: Compression) -> GzEncoder<R>` — [`Compression`](../../index.md#compression), [`GzEncoder`](#gzencoder)

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

### `GzDecoder<R>`

```rust
struct GzDecoder<R> {
    inner: bufread::GzDecoder<crate::bufreader::BufReader<R>>,
}
```

A decoder for a single member of a [gzip file].

This structure implements a [`Read`](../../../embedded_hal/index.md) interface. When read from, it reads
compressed data from the underlying [`Read`](../../../embedded_hal/index.md) and provides the uncompressed data.

After reading a single member of the gzip data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`GzDecoder` may have read additional bytes past the end of the gzip data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::GzDecoder` instead.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](#multigzdecoder)
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

- <span id="gzdecoder-new"></span>`fn new(r: R) -> GzDecoder<R>` — [`GzDecoder`](#gzdecoder)

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

### `MultiGzDecoder<R>`

```rust
struct MultiGzDecoder<R> {
    inner: bufread::MultiGzDecoder<crate::bufreader::BufReader<R>>,
}
```

A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

This structure implements a [`Read`](../../../embedded_hal/index.md) interface. When read from, it reads
compressed data from the underlying [`Read`](../../../embedded_hal/index.md) and provides the uncompressed
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

- <span id="multigzdecoder-new"></span>`fn new(r: R) -> MultiGzDecoder<R>` — [`MultiGzDecoder`](#multigzdecoder)

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

## Functions

### `gz_encoder`

```rust
fn gz_encoder<R: Read>(inner: bufread::GzEncoder<crate::bufreader::BufReader<R>>) -> GzEncoder<R>
```

