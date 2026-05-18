*[flate2](../../index.md) / [gz](../index.md) / [bufread](index.md)*

---

# Module `bufread`

## Contents

- [Structs](#structs)
  - [`GzEncoder`](#gzencoder)
  - [`GzDecoder`](#gzdecoder)
  - [`MultiGzDecoder`](#multigzdecoder)
- [Enums](#enums)
  - [`GzState`](#gzstate)
- [Functions](#functions)
  - [`copy`](#copy)
  - [`gz_encoder`](#gz-encoder)
  - [`finish`](#finish)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GzEncoder`](#gzencoder) | struct | A gzip streaming encoder |
| [`GzDecoder`](#gzdecoder) | struct | A decoder for a single member of a [gzip file]. |
| [`MultiGzDecoder`](#multigzdecoder) | struct | A gzip streaming decoder that decodes a [gzip file] that may have multiple members. |
| [`GzState`](#gzstate) | enum |  |
| [`copy`](#copy) | fn |  |
| [`gz_encoder`](#gz-encoder) | fn |  |
| [`finish`](#finish) | fn |  |

## Structs

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

- <span id="gzencoder-new"></span>`fn new(r: R, level: Compression) -> GzEncoder<R>` — [`Compression`](../../index.md#compression), [`GzEncoder`](#gzencoder)

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

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](#multigzdecoder)
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

- <span id="gzdecoder-new"></span>`fn new(r: R) -> GzDecoder<R>` — [`GzDecoder`](#gzdecoder)

  Creates a new decoder from the given reader, immediately parsing the

  gzip header.

- <span id="gzdecoder-multi"></span>`fn multi(self, flag: bool) -> GzDecoder<R>` — [`GzDecoder`](#gzdecoder)

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for GzDecoder<R>`

- <span id="gzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for GzDecoder<R>`

- <span id="gzdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

##### `impl<R: BufRead + Write> Write for GzDecoder<R>`

- <span id="gzdecoder-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="gzdecoder-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

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

- <span id="multigzdecoder-new"></span>`fn new(r: R) -> MultiGzDecoder<R>` — [`MultiGzDecoder`](#multigzdecoder)

  Creates a new decoder from the given reader, immediately parsing the

  (first) gzip header. If the gzip stream contains multiple members all will

  be decoded.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for MultiGzDecoder<R>`

- <span id="multigzdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: BufRead> Read for MultiGzDecoder<R>`

- <span id="multigzdecoder-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

## Enums

### `GzState`

```rust
enum GzState {
    Header(super::GzHeaderParser),
    Body(super::GzHeader),
    Finished(super::GzHeader, usize, [u8; 8]),
    Err(io::Error),
    End(Option<super::GzHeader>),
}
```

#### Trait Implementations

##### `impl Debug for GzState`

- <span id="gzstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `copy`

```rust
fn copy(into: &mut [u8], from: &[u8], pos: &mut usize) -> usize
```

### `gz_encoder`

```rust
fn gz_encoder<R: BufRead>(header: Vec<u8>, r: R, lvl: crate::Compression) -> GzEncoder<R>
```

### `finish`

```rust
fn finish(buf: &[u8; 8]) -> (u32, u32)
```

