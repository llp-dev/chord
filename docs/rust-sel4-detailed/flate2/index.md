# Crate `flate2`

A DEFLATE-based stream compression/decompression library

This library provides support for compression and decompression of
DEFLATE-based streams:

* the DEFLATE format itself
* the zlib format
* gzip

These three formats are all closely related and largely only differ in their
headers/footers. This crate has three types in each submodule for dealing
with these three formats.

# Implementation

In addition to supporting three formats, this crate supports several different
backends, controlled through this crate's *features flags*:

* `default`, or `rust_backend` - this implementation currently uses the `miniz_oxide`
  crate which is a port of `miniz.c` to Rust. This feature does not
  require a C compiler, and only uses safe Rust code.

  Note that the `rust_backend` feature may at some point be switched to use `zlib-rs`,
  and that `miniz_oxide` should be used explicitly if this is not desired.

* `zlib-rs` - this implementation utilizes the `zlib-rs` crate, a Rust rewrite of zlib.
  This backend is the fastest, at the cost of some `unsafe` Rust code.

Several backends implemented in C are also available.
These are useful in case you are already using a specific C implementation
and need the result of compression to be bit-identical.
See the crate's README for details on the available C backends.

The `zlib-rs` backend typically outperforms all the C implementations.

# Feature Flags
Activate the `document-features` cargo feature to see feature docs here

## Ambiguous feature selection

As Cargo features are additive, while backends are not, there is an order in which backends
become active if multiple are selected.

* zlib-ng
* zlib-rs
* cloudflare_zlib
* miniz_oxide

# Organization

This crate consists of three main modules: `bufread`, `read`, and `write`. Each module
implements DEFLATE, zlib, and gzip for [`std::io::BufRead`](../embedded_io/index.md) input types, [`std::io::Read`](../embedded_hal/index.md) input
types, and [`std::io::Write`](../embedded_hal/index.md) output types respectively.

Use the [`mod@bufread`](bufread/index.md) implementations if you can provide a `BufRead` type for the input.
The `&[u8]` slice type implements the `BufRead` trait.

The [`mod@read`](read/index.md) implementations conveniently wrap a `Read` type in a `BufRead` implementation.
However, the `read` implementations may
[read past the end of the input data](https://github.com/rust-lang/flate2-rs/issues/338),
making the `Read` type useless for subsequent reads of the input. If you need to re-use the
`Read` type, wrap it in a [`std::io::BufReader`](bufreader/index.md), use the `bufread` implementations,
and perform subsequent reads on the `BufReader`.

The [`mod@write`](write/index.md) implementations are most useful when there is no way to create a `BufRead`
type, notably when reading async iterators (streams).

```rust
use futures::{Stream, StreamExt};
use std::io::{Result, Write as _};

async fn decompress_gzip_stream<S, I>(stream: S) -> Result<Vec<u8>>
where
    S: Stream<Item = I>,
    I: AsRef<[u8]>
{
    let mut stream = std::pin::pin!(stream);
    let mut w = Vec::<u8>::new();
    let mut decoder = flate2::write::GzDecoder::new(w);
    while let Some(input) = stream.next().await {
        decoder.write_all(input.as_ref())?;
    }
    decoder.finish()
}
```


Note that types which operate over a specific trait often implement the mirroring trait as well.
For example a `bufread::DeflateDecoder<T>` *also* implements the
[`Write`]() trait if `T: Write`. That is, the "dual trait" is forwarded directly
to the underlying object if available.

# About multi-member Gzip files

While most `gzip` files one encounters will have a single *member* that can be read
with the [`GzDecoder`](gz/bufread/index.md), there may be some files which have multiple members.

A [`GzDecoder`](gz/bufread/index.md) will only read the first member of gzip data, which may unexpectedly
provide partial results when a multi-member gzip file is encountered. `GzDecoder` is appropriate
for data that is designed to be read as single members from a multi-member file. `bufread::GzDecoder`
and `write::GzDecoder` also allow non-gzip data following gzip data to be handled.

The [`MultiGzDecoder`](gz/write/index.md) on the other hand will decode all members of a `gzip` file
into one consecutive stream of bytes, which hides the underlying *members* entirely.
If a file contains non-gzip data after the gzip data, MultiGzDecoder will
emit an error after decoding the gzip data. This behavior matches the `gzip`,
`gunzip`, and `zcat` command line tools.







## Contents

- [Modules](#modules)
  - [`bufreader`](#bufreader)
  - [`crc`](#crc)
  - [`deflate`](#deflate)
  - [`ffi`](#ffi)
  - [`gz`](#gz)
  - [`mem`](#mem)
  - [`zio`](#zio)
  - [`zlib`](#zlib)
  - [`read`](#read)
  - [`write`](#write)
  - [`bufread`](#bufread)
- [Structs](#structs)
  - [`Crc`](#crc)
  - [`CrcReader`](#crcreader)
  - [`CrcWriter`](#crcwriter)
  - [`GzBuilder`](#gzbuilder)
  - [`GzHeader`](#gzheader)
  - [`Compress`](#compress)
  - [`CompressError`](#compresserror)
  - [`Decompress`](#decompress)
  - [`DecompressError`](#decompresserror)
  - [`Compression`](#compression)
- [Enums](#enums)
  - [`Status`](#status)
  - [`FlushCompress`](#flushcompress)
  - [`FlushDecompress`](#flushdecompress)
- [Functions](#functions)
  - [`_assert_send_sync`](#assert-send-sync)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bufreader`](#bufreader) | mod |  |
| [`crc`](#crc) | mod | Simple CRC bindings backed by miniz.c |
| [`deflate`](#deflate) | mod |  |
| [`ffi`](#ffi) | mod | This module contains backend-specific code. |
| [`gz`](#gz) | mod |  |
| [`mem`](#mem) | mod |  |
| [`zio`](#zio) | mod |  |
| [`zlib`](#zlib) | mod |  |
| [`read`](#read) | mod | Types which operate over [`Read`] streams, both encoders and decoders for various formats. |
| [`write`](#write) | mod | Types which operate over [`Write`] streams, both encoders and decoders for various formats. |
| [`bufread`](#bufread) | mod | Types which operate over [`BufRead`] streams, both encoders and decoders for various formats. |
| [`Crc`](#crc) | struct |  |
| [`CrcReader`](#crcreader) | struct |  |
| [`CrcWriter`](#crcwriter) | struct |  |
| [`GzBuilder`](#gzbuilder) | struct |  |
| [`GzHeader`](#gzheader) | struct |  |
| [`Compress`](#compress) | struct |  |
| [`CompressError`](#compresserror) | struct |  |
| [`Decompress`](#decompress) | struct |  |
| [`DecompressError`](#decompresserror) | struct |  |
| [`Compression`](#compression) | struct | When compressing data, the compression level can be specified by a value in this struct. |
| [`Status`](#status) | enum |  |
| [`FlushCompress`](#flushcompress) | enum |  |
| [`FlushDecompress`](#flushdecompress) | enum |  |
| [`_assert_send_sync`](#assert-send-sync) | fn |  |

## Modules

- [`bufreader`](bufreader/index.md)
- [`crc`](crc/index.md) — Simple CRC bindings backed by miniz.c
- [`deflate`](deflate/index.md)
- [`ffi`](ffi/index.md) — This module contains backend-specific code.
- [`gz`](gz/index.md)
- [`mem`](mem/index.md)
- [`zio`](zio/index.md)
- [`zlib`](zlib/index.md)
- [`read`](read/index.md) — Types which operate over [`Read`] streams, both encoders and decoders for
- [`write`](write/index.md) — Types which operate over [`Write`] streams, both encoders and decoders for
- [`bufread`](bufread/index.md) — Types which operate over [`BufRead`] streams, both encoders and decoders for

## Structs

### `Crc`

```rust
struct Crc {
    amt: u32,
    hasher: crc32fast::Hasher,
}
```

The CRC calculated by a [`CrcReader`](crc/index.md).


#### Implementations

- <span id="crc-new"></span>`fn new() -> Self`

  Create a new CRC.

- <span id="crc-sum"></span>`fn sum(&self) -> u32`

  Returns the current crc32 checksum.

- <span id="crc-amount"></span>`fn amount(&self) -> u32`

  The number of bytes that have been used to calculate the CRC.

  This value is only accurate if the amount is lower than 2<sup>32</sup>.

- <span id="crc-update"></span>`fn update(&mut self, data: &[u8])`

  Update the CRC with the bytes in `data`.

- <span id="crc-reset"></span>`fn reset(&mut self)`

  Reset the CRC.

- <span id="crc-combine"></span>`fn combine(&mut self, additional_crc: &Self)`

  Combine the CRC with the CRC for the subsequent block of bytes.

#### Trait Implementations

##### `impl Debug for Crc`

- <span id="crc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Crc`

- <span id="crc-default"></span>`fn default() -> Crc` — [`Crc`](crc/impl_crc32fast/index.md#crc)

### `CrcReader<R>`

```rust
struct CrcReader<R> {
    inner: R,
    crc: Crc,
}
```

A wrapper around a `Read` that calculates the CRC.


#### Implementations

- <span id="crcreader-new"></span>`fn new(r: R) -> CrcReader<R>` — [`CrcReader`](crc/index.md#crcreader)

  Create a new `CrcReader`.

#### Trait Implementations

##### `impl<R: BufRead> BufRead for CrcReader<R>`

- <span id="crcreader-bufread-fill-buf"></span>`fn fill_buf(&mut self) -> io::Result<&[u8]>`

- <span id="crcreader-bufread-consume"></span>`fn consume(&mut self, amt: usize)`

##### `impl<R: fmt::Debug> Debug for CrcReader<R>`

- <span id="crcreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for CrcReader<R>`

- <span id="crcreader-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

### `CrcWriter<W>`

```rust
struct CrcWriter<W> {
    inner: W,
    crc: Crc,
}
```

A wrapper around a [`Write`]() that calculates the CRC.


#### Implementations

- <span id="crcwriter-crc"></span>`fn crc(&self) -> &Crc` — [`Crc`](crc/impl_crc32fast/index.md#crc)

  Get the Crc for this `CrcWriter`.

- <span id="crcwriter-into-inner"></span>`fn into_inner(self) -> W`

  Get the writer that is wrapped by this `CrcWriter`.

- <span id="crcwriter-get-ref"></span>`fn get_ref(&self) -> &W`

  Get the writer that is wrapped by this `CrcWriter` by reference.

- <span id="crcwriter-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Get a mutable reference to the writer that is wrapped by this `CrcWriter`.

- <span id="crcwriter-reset"></span>`fn reset(&mut self)`

  Reset the Crc in this `CrcWriter`.

#### Trait Implementations

##### `impl<W: fmt::Debug> Debug for CrcWriter<W>`

- <span id="crcwriter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Write> Write for CrcWriter<W>`

- <span id="crcwriter-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="crcwriter-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `GzBuilder`

```rust
struct GzBuilder {
    extra: Option<Vec<u8>>,
    filename: Option<std::ffi::CString>,
    comment: Option<std::ffi::CString>,
    operating_system: Option<u8>,
    mtime: u32,
}
```

A builder structure to create a new gzip Encoder.

This structure controls header configuration options such as the filename.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use std::fs::File;
use flate2::GzBuilder;
use flate2::Compression;

// GzBuilder opens a file and writes a sample string using GzBuilder pattern

fn sample_builder() -> Result<(), io::Error> {
let f = File::create("examples/hello_world.gz")?;
let mut gz = GzBuilder::new()
                .filename("hello_world.txt")
                .comment("test file, please delete")
                .write(f, Compression::default());
gz.write_all(b"hello world")?;
gz.finish()?;
Ok(())
}
```

#### Implementations

- <span id="gzbuilder-new"></span>`fn new() -> GzBuilder` — [`GzBuilder`](gz/index.md#gzbuilder)

  Create a new blank builder with no header by default.

- <span id="gzbuilder-mtime"></span>`fn mtime(self, mtime: u32) -> GzBuilder` — [`GzBuilder`](gz/index.md#gzbuilder)

  Configure the `mtime` field in the gzip header.

- <span id="gzbuilder-operating-system"></span>`fn operating_system(self, os: u8) -> GzBuilder` — [`GzBuilder`](gz/index.md#gzbuilder)

  Configure the `operating_system` field in the gzip header.

- <span id="gzbuilder-extra"></span>`fn extra<T: Into<Vec<u8>>>(self, extra: T) -> GzBuilder` — [`GzBuilder`](gz/index.md#gzbuilder)

  Configure the `extra` field in the gzip header.

- <span id="gzbuilder-filename"></span>`fn filename<T: Into<Vec<u8>>>(self, filename: T) -> GzBuilder` — [`GzBuilder`](gz/index.md#gzbuilder)

  Configure the `filename` field in the gzip header.

  

  # Panics

  

  Panics if the `filename` slice contains a zero.

- <span id="gzbuilder-comment"></span>`fn comment<T: Into<Vec<u8>>>(self, comment: T) -> GzBuilder` — [`GzBuilder`](gz/index.md#gzbuilder)

  Configure the `comment` field in the gzip header.

  

  # Panics

  

  Panics if the `comment` slice contains a zero.

- <span id="gzbuilder-write"></span>`fn write<W: Write>(self, w: W, lvl: Compression) -> write::GzEncoder<W>` — [`Compression`](#compression), [`GzEncoder`](gz/write/index.md#gzencoder)

  Consume this builder, creating a writer encoder in the process.

  

  The data written to the returned encoder will be compressed and then

  written out to the supplied parameter `w`.

- <span id="gzbuilder-read"></span>`fn read<R: Read>(self, r: R, lvl: Compression) -> read::GzEncoder<R>` — [`Compression`](#compression), [`GzEncoder`](gz/read/index.md#gzencoder)

  Consume this builder, creating a reader encoder in the process.

  

  Data read from the returned encoder will be the compressed version of

  the data read from the given reader.

- <span id="gzbuilder-buf-read"></span>`fn buf_read<R>(self, r: R, lvl: Compression) -> bufread::GzEncoder<R>` — [`Compression`](#compression), [`GzEncoder`](gz/bufread/index.md#gzencoder)

  Consume this builder, creating a reader encoder in the process.

  

  Data read from the returned encoder will be the compressed version of

  the data read from the given reader.

- <span id="gzbuilder-into-header"></span>`fn into_header(self, lvl: Compression) -> Vec<u8>` — [`Compression`](#compression)

#### Trait Implementations

##### `impl Debug for GzBuilder`

- <span id="gzbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GzBuilder`

- <span id="gzbuilder-default"></span>`fn default() -> GzBuilder` — [`GzBuilder`](gz/index.md#gzbuilder)

### `GzHeader`

```rust
struct GzHeader {
    extra: Option<Vec<u8>>,
    filename: Option<Vec<u8>>,
    comment: Option<Vec<u8>>,
    operating_system: u8,
    mtime: u32,
}
```

A structure representing the header of a gzip stream.

The header can contain metadata about the file that was compressed, if
present.

#### Implementations

- <span id="gzheader-filename"></span>`fn filename(&self) -> Option<&[u8]>`

  Returns the `filename` field of this gzip stream's header, if present.

- <span id="gzheader-extra"></span>`fn extra(&self) -> Option<&[u8]>`

  Returns the `extra` field of this gzip stream's header, if present.

- <span id="gzheader-comment"></span>`fn comment(&self) -> Option<&[u8]>`

  Returns the `comment` field of this gzip stream's header, if present.

- <span id="gzheader-operating-system"></span>`fn operating_system(&self) -> u8`

  Returns the `operating_system` field of this gzip stream's header.

  

  There are predefined values for various operating systems.

  255 means that the value is unknown.

- <span id="gzheader-mtime"></span>`fn mtime(&self) -> u32`

  This gives the most recent modification time of the original file being compressed.

  

  The time is in Unix format, i.e., seconds since 00:00:00 GMT, Jan. 1, 1970.

  (Note that this may cause problems for MS-DOS and other systems that use local

  rather than Universal time.) If the compressed data did not come from a file,

  `mtime` is set to the time at which compression started.

  `mtime` = 0 means no time stamp is available.

  

  The usage of `mtime` is discouraged because of Year 2038 problem.

- <span id="gzheader-mtime-as-datetime"></span>`fn mtime_as_datetime(&self) -> Option<time::SystemTime>`

  Returns the most recent modification time represented by a date-time type.

  Returns `None` if the value of the underlying counter is 0,

  indicating no time stamp is available.

  

  

  The time is measured as seconds since 00:00:00 GMT, Jan. 1 1970.

  See [`mtime`](#method.mtime) for more detail.

#### Trait Implementations

##### `impl Clone for GzHeader`

- <span id="gzheader-clone"></span>`fn clone(&self) -> GzHeader` — [`GzHeader`](gz/index.md#gzheader)

##### `impl Debug for GzHeader`

- <span id="gzheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GzHeader`

- <span id="gzheader-default"></span>`fn default() -> GzHeader` — [`GzHeader`](gz/index.md#gzheader)

##### `impl PartialEq for GzHeader`

- <span id="gzheader-partialeq-eq"></span>`fn eq(&self, other: &GzHeader) -> bool` — [`GzHeader`](gz/index.md#gzheader)

##### `impl StructuralPartialEq for GzHeader`

### `Compress`

```rust
struct Compress {
    inner: crate::ffi::Deflate,
}
```

Raw in-memory compression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the `Read`/[`Write`]() API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.



#### Implementations

- <span id="compress-new"></span>`fn new(level: Compression, zlib_header: bool) -> Compress` — [`Compression`](#compression), [`Compress`](mem/index.md#compress)

  Creates a new object ready for compressing data that it's given.

  

  The `level` argument here indicates what level of compression is going

  to be performed, and the `zlib_header` argument indicates whether the

  output data should have a zlib header or not.

- <span id="compress-total-in"></span>`fn total_in(&self) -> u64`

  Returns the total number of input bytes which have been processed by

  this compression object.

- <span id="compress-total-out"></span>`fn total_out(&self) -> u64`

  Returns the total number of output bytes which have been produced by

  this compression object.

- <span id="compress-reset"></span>`fn reset(&mut self)`

  Quickly resets this compressor without having to reallocate anything.

  

  This is equivalent to dropping this object and then creating a new one.

- <span id="compress-compress"></span>`fn compress(&mut self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](mem/index.md#flushcompress), [`Status`](mem/index.md#status), [`CompressError`](mem/index.md#compresserror)

  Compresses the input data into the output, consuming only as much

  input as needed and writing as much output as possible.

  

  The flush option can be any of the available `FlushCompress` parameters.

  

  To learn how much data was consumed or how much output was produced, use

  the `total_in` and `total_out` functions before/after this is called.

- <span id="compress-compress-uninit"></span>`fn compress_uninit(&mut self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](mem/index.md#flushcompress), [`Status`](mem/index.md#status), [`CompressError`](mem/index.md#compresserror)

  Similar to `Self::compress` but accepts uninitialized buffer.

  

  If you want to avoid the overhead of zero initializing the

  buffer and you don't want to use a [`Vec`](../addr2line/maybe_small/index.md), then please use

  this API.

- <span id="compress-compress-vec"></span>`fn compress_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](mem/index.md#flushcompress), [`Status`](mem/index.md#status), [`CompressError`](mem/index.md#compresserror)

  Compresses the input data into the extra space of the output, consuming

  only as much input as needed and writing as much output as possible.

  

  This function has the same semantics as `compress`, except that the

  length of `vec` is managed by this function. This will not reallocate

  the vector provided or attempt to grow it, so space for the output must

  be reserved in the output vector by the caller before calling this

  function.

#### Trait Implementations

##### `impl Debug for Compress`

- <span id="compress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Ops for crate::Compress`

- <span id="cratecompress-ops-type-error"></span>`type Error = CompressError`

- <span id="cratecompress-ops-type-flush"></span>`type Flush = FlushCompress`

- <span id="cratecompress-ops-total-in"></span>`fn total_in(&self) -> u64`

- <span id="cratecompress-ops-total-out"></span>`fn total_out(&self) -> u64`

- <span id="cratecompress-ops-run"></span>`fn run(&mut self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](mem/index.md#flushcompress), [`Status`](mem/index.md#status), [`CompressError`](mem/index.md#compresserror)

- <span id="cratecompress-ops-run-vec"></span>`fn run_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](mem/index.md#flushcompress), [`Status`](mem/index.md#status), [`CompressError`](mem/index.md#compresserror)

### `CompressError`

```rust
struct CompressError {
    msg: crate::ffi::ErrorMessage,
}
```

Error returned when a compression object is used incorrectly or otherwise
generates an error.

#### Implementations

- <span id="compresserror-message"></span>`fn message(&self) -> Option<&str>`

  Retrieve the implementation's message about why the operation failed, if one exists.

#### Trait Implementations

##### `impl Clone for CompressError`

- <span id="compresserror-clone"></span>`fn clone(&self) -> CompressError` — [`CompressError`](mem/index.md#compresserror)

##### `impl Debug for CompressError`

- <span id="compresserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CompressError`

- <span id="compresserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for CompressError`

##### `impl ToString for CompressError`

- <span id="compresserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Decompress`

```rust
struct Decompress {
    inner: crate::ffi::Inflate,
}
```

Raw in-memory decompression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the `Read`/[`Write`]() API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.



#### Implementations

- <span id="decompress-new"></span>`fn new(zlib_header: bool) -> Decompress` — [`Decompress`](mem/index.md#decompress)

  Creates a new object ready for decompressing data that it's given.

  

  The `zlib_header` argument indicates whether the input data is expected

  to have a zlib header or not.

- <span id="decompress-total-in"></span>`fn total_in(&self) -> u64`

  Returns the total number of input bytes which have been processed by

  this decompression object.

- <span id="decompress-total-out"></span>`fn total_out(&self) -> u64`

  Returns the total number of output bytes which have been produced by

  this decompression object.

- <span id="decompress-decompress"></span>`fn decompress(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](mem/index.md#flushdecompress), [`Status`](mem/index.md#status), [`DecompressError`](mem/index.md#decompresserror)

  Decompresses the input data into the output, consuming only as much

  input as needed and writing as much output as possible.

  

  The flush option can be any of the available `FlushDecompress` parameters.

  

  If the first call passes `FlushDecompress::Finish` it is assumed that

  the input and output buffers are both sized large enough to decompress

  the entire stream in a single call.

  

  A flush value of `FlushDecompress::Finish` indicates that there are no

  more source bytes available beside what's already in the input buffer,

  and the output buffer is large enough to hold the rest of the

  decompressed data.

  

  To learn how much data was consumed or how much output was produced, use

  the `total_in` and `total_out` functions before/after this is called.

  

  # Errors

  

  If the input data to this instance of `Decompress` is not a valid

  zlib/deflate stream then this function may return an instance of

  `DecompressError` to indicate that the stream of input bytes is corrupted.

- <span id="decompress-decompress-uninit"></span>`fn decompress_uninit(&mut self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](mem/index.md#flushdecompress), [`Status`](mem/index.md#status), [`DecompressError`](mem/index.md#decompresserror)

  Similar to `Self::decompress` but accepts uninitialized buffer

  

  If you want to avoid the overhead of zero initializing the

  buffer and you don't want to use a [`Vec`](../addr2line/maybe_small/index.md), then please use

  this API.

- <span id="decompress-decompress-vec"></span>`fn decompress_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](mem/index.md#flushdecompress), [`Status`](mem/index.md#status), [`DecompressError`](mem/index.md#decompresserror)

  Decompresses the input data into the extra space in the output vector

  specified by `output`.

  

  This function has the same semantics as `decompress`, except that the

  length of `vec` is managed by this function. This will not reallocate

  the vector provided or attempt to grow it, so space for the output must

  be reserved in the output vector by the caller before calling this

  function.

  

  # Errors

  

  If the input data to this instance of `Decompress` is not a valid

  zlib/deflate stream then this function may return an instance of

  `DecompressError` to indicate that the stream of input bytes is corrupted.

- <span id="decompress-reset"></span>`fn reset(&mut self, zlib_header: bool)`

  Performs the equivalent of replacing this decompression state with a

  freshly allocated copy.

  

  This function may not allocate memory, though, and attempts to reuse any

  previously existing resources.

  

  The argument provided here indicates whether the reset state will

  attempt to decode a zlib header first or not.

#### Trait Implementations

##### `impl Debug for Decompress`

- <span id="decompress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Ops for crate::Decompress`

- <span id="cratedecompress-ops-type-error"></span>`type Error = DecompressError`

- <span id="cratedecompress-ops-type-flush"></span>`type Flush = FlushDecompress`

- <span id="cratedecompress-ops-total-in"></span>`fn total_in(&self) -> u64`

- <span id="cratedecompress-ops-total-out"></span>`fn total_out(&self) -> u64`

- <span id="cratedecompress-ops-run"></span>`fn run(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](mem/index.md#flushdecompress), [`Status`](mem/index.md#status), [`DecompressError`](mem/index.md#decompresserror)

- <span id="cratedecompress-ops-run-vec"></span>`fn run_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](mem/index.md#flushdecompress), [`Status`](mem/index.md#status), [`DecompressError`](mem/index.md#decompresserror)

### `DecompressError`

```rust
struct DecompressError(DecompressErrorInner);
```

Error returned when a decompression object finds that the input stream of
bytes was not a valid input stream of bytes.

#### Implementations

- <span id="decompresserror-needs-dictionary"></span>`fn needs_dictionary(&self) -> Option<u32>`

  Indicates whether decompression failed due to requiring a dictionary.

  

  The resulting integer is the Adler-32 checksum of the dictionary

  required.

#### Trait Implementations

##### `impl Clone for DecompressError`

- <span id="decompresserror-clone"></span>`fn clone(&self) -> DecompressError` — [`DecompressError`](mem/index.md#decompresserror)

##### `impl Debug for DecompressError`

- <span id="decompresserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecompressError`

- <span id="decompresserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for DecompressError`

##### `impl ToString for DecompressError`

- <span id="decompresserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Compression`

```rust
struct Compression(u32);
```

When compressing data, the compression level can be specified by a value in
this struct.

#### Implementations

- <span id="compression-new"></span>`const fn new(level: u32) -> Compression` — [`Compression`](#compression)

  Creates a new description of the compression level with an explicitly

  specified integer.

  

  The integer here is typically on a scale of 0-9 where 0 means "no

  compression" and 9 means "take as long as you'd like".

- <span id="compression-none"></span>`const fn none() -> Compression` — [`Compression`](#compression)

  No compression is to be performed, this may actually inflate data

  slightly when encoding.

- <span id="compression-fast"></span>`const fn fast() -> Compression` — [`Compression`](#compression)

  Optimize for the best speed of encoding.

- <span id="compression-best"></span>`const fn best() -> Compression` — [`Compression`](#compression)

  Optimize for the size of data being encoded.

- <span id="compression-level"></span>`fn level(&self) -> u32`

  Returns an integer representing the compression level, typically on a

  scale of 0-9. See [`new`](Self::new) for details about compression levels.

#### Trait Implementations

##### `impl Clone for Compression`

- <span id="compression-clone"></span>`fn clone(&self) -> Compression` — [`Compression`](#compression)

##### `impl Copy for Compression`

##### `impl Debug for Compression`

- <span id="compression-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Compression`

- <span id="compression-default"></span>`fn default() -> Compression` — [`Compression`](#compression)

##### `impl Eq for Compression`

##### `impl PartialEq for Compression`

- <span id="compression-partialeq-eq"></span>`fn eq(&self, other: &Compression) -> bool` — [`Compression`](#compression)

##### `impl StructuralPartialEq for Compression`

## Enums

### `Status`

```rust
enum Status {
    Ok,
    BufError,
    StreamEnd,
}
```

Possible status results of compressing some data or successfully
decompressing a block of data.

#### Variants

- **`Ok`**

  Indicates success.
  
  Means that more input may be needed but isn't available
  and/or there's more output to be written but the output buffer is full.

- **`BufError`**

  Indicates that forward progress is not possible due to input or output
  buffers being empty.
  
  For compression it means the input buffer needs some more data or the
  output buffer needs to be freed up before trying again.
  
  For decompression this means that more input is needed to continue or
  the output buffer isn't large enough to contain the result. The function
  can be called again after fixing both.

- **`StreamEnd`**

  Indicates that all input has been consumed and all output bytes have
  been written. Decompression/compression should not be called again.
  
  For decompression with zlib streams the adler-32 of the decompressed
  data has also been verified.

#### Trait Implementations

##### `impl Clone for Status`

- <span id="status-clone"></span>`fn clone(&self) -> Status` — [`Status`](mem/index.md#status)

##### `impl Copy for Status`

##### `impl Debug for Status`

- <span id="status-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Status`

##### `impl PartialEq for Status`

- <span id="status-partialeq-eq"></span>`fn eq(&self, other: &Status) -> bool` — [`Status`](mem/index.md#status)

##### `impl StructuralPartialEq for Status`

### `FlushCompress`

```rust
enum FlushCompress {
    None,
    Partial,
    Sync,
    Full,
    Finish,
}
```

Values which indicate the form of flushing to be used when compressing
in-memory data.

#### Variants

- **`None`**

  A typical parameter for passing to compression/decompression functions,
  this indicates that the underlying stream to decide how much data to
  accumulate before producing output in order to maximize compression.

- **`Partial`**

  All pending output is flushed to the output buffer, but the output is
  not aligned to a byte boundary.
  
  All input data so far will be available to the decompressor (as with
  `Flush::Sync`). This completes the current deflate block and follows it
  with an empty fixed codes block that is 10 bits long, and it assures
  that enough bytes are output in order for the decompressor to finish the
  block before the empty fixed code block.

- **`Sync`**

  All pending output is flushed to the output buffer and the output is
  aligned on a byte boundary so that the decompressor can get all input
  data available so far.
  
  Flushing may degrade compression for some compression algorithms and so
  it should only be used when necessary. This will complete the current
  deflate block and follow it with an empty stored block.

- **`Full`**

  All output is flushed as with `Flush::Sync` and the compression state is
  reset so decompression can restart from this point if previous
  compressed data has been damaged or if random access is desired.
  
  Using this option too often can seriously degrade compression.

- **`Finish`**

  Pending input is processed and pending output is flushed.
  
  The return value may indicate that the stream is not yet done and more
  data has yet to be processed.

#### Trait Implementations

##### `impl Clone for FlushCompress`

- <span id="flushcompress-clone"></span>`fn clone(&self) -> FlushCompress` — [`FlushCompress`](mem/index.md#flushcompress)

##### `impl Copy for FlushCompress`

##### `impl Debug for FlushCompress`

- <span id="flushcompress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlushCompress`

##### `impl Flush for crate::FlushCompress`

- <span id="crateflushcompress-flush-none"></span>`fn none() -> Self`

- <span id="crateflushcompress-flush-sync"></span>`fn sync() -> Self`

- <span id="crateflushcompress-flush-finish"></span>`fn finish() -> Self`

##### `impl PartialEq for FlushCompress`

- <span id="flushcompress-partialeq-eq"></span>`fn eq(&self, other: &FlushCompress) -> bool` — [`FlushCompress`](mem/index.md#flushcompress)

##### `impl StructuralPartialEq for FlushCompress`

### `FlushDecompress`

```rust
enum FlushDecompress {
    None,
    Sync,
    Finish,
}
```

Values which indicate the form of flushing to be used when
decompressing in-memory data.

#### Variants

- **`None`**

  A typical parameter for passing to compression/decompression functions,
  this indicates that the underlying stream to decide how much data to
  accumulate before producing output in order to maximize compression.

- **`Sync`**

  All pending output is flushed to the output buffer and the output is
  aligned on a byte boundary so that the decompressor can get all input
  data available so far.
  
  Flushing may degrade compression for some compression algorithms and so
  it should only be used when necessary. This will complete the current
  deflate block and follow it with an empty stored block.

- **`Finish`**

  Pending input is processed and pending output is flushed.
  
  The return value may indicate that the stream is not yet done and more
  data has yet to be processed.

#### Trait Implementations

##### `impl Clone for FlushDecompress`

- <span id="flushdecompress-clone"></span>`fn clone(&self) -> FlushDecompress` — [`FlushDecompress`](mem/index.md#flushdecompress)

##### `impl Copy for FlushDecompress`

##### `impl Debug for FlushDecompress`

- <span id="flushdecompress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlushDecompress`

##### `impl Flush for crate::FlushDecompress`

- <span id="crateflushdecompress-flush-none"></span>`fn none() -> Self`

- <span id="crateflushdecompress-flush-sync"></span>`fn sync() -> Self`

- <span id="crateflushdecompress-flush-finish"></span>`fn finish() -> Self`

##### `impl PartialEq for FlushDecompress`

- <span id="flushdecompress-partialeq-eq"></span>`fn eq(&self, other: &FlushDecompress) -> bool` — [`FlushDecompress`](mem/index.md#flushdecompress)

##### `impl StructuralPartialEq for FlushDecompress`

## Functions

### `_assert_send_sync`

```rust
fn _assert_send_sync()
```

