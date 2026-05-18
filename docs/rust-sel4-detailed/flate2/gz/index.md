*[flate2](../index.md) / [gz](index.md)*

---

# Module `gz`

## Contents

- [Modules](#modules)
  - [`bufread`](#bufread)
  - [`read`](#read)
  - [`write`](#write)
- [Structs](#structs)
  - [`GzHeader`](#gzheader)
  - [`GzHeaderParser`](#gzheaderparser)
  - [`GzBuilder`](#gzbuilder)
- [Enums](#enums)
  - [`GzHeaderState`](#gzheaderstate)
- [Functions](#functions)
  - [`read_into`](#read-into)
  - [`read_to_nul`](#read-to-nul)
  - [`parse_le_u16`](#parse-le-u16)
  - [`bad_header`](#bad-header)
  - [`corrupt`](#corrupt)
- [Constants](#constants)
  - [`MAX_HEADER_BUF`](#max-header-buf)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bufread`](#bufread) | mod |  |
| [`read`](#read) | mod |  |
| [`write`](#write) | mod |  |
| [`GzHeader`](#gzheader) | struct | A structure representing the header of a gzip stream. |
| [`GzHeaderParser`](#gzheaderparser) | struct |  |
| [`GzBuilder`](#gzbuilder) | struct | A builder structure to create a new gzip Encoder. |
| [`GzHeaderState`](#gzheaderstate) | enum |  |
| [`read_into`](#read-into) | fn |  |
| [`read_to_nul`](#read-to-nul) | fn |  |
| [`parse_le_u16`](#parse-le-u16) | fn |  |
| [`bad_header`](#bad-header) | fn |  |
| [`corrupt`](#corrupt) | fn |  |
| [`MAX_HEADER_BUF`](#max-header-buf) | const |  |

## Modules

- [`bufread`](bufread/index.md)
- [`read`](read/index.md)
- [`write`](write/index.md)

## Structs

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

- <span id="gzheader-clone"></span>`fn clone(&self) -> GzHeader` — [`GzHeader`](#gzheader)

##### `impl Debug for GzHeader`

- <span id="gzheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GzHeader`

- <span id="gzheader-default"></span>`fn default() -> GzHeader` — [`GzHeader`](#gzheader)

##### `impl PartialEq for GzHeader`

- <span id="gzheader-partialeq-eq"></span>`fn eq(&self, other: &GzHeader) -> bool` — [`GzHeader`](#gzheader)

##### `impl StructuralPartialEq for GzHeader`

### `GzHeaderParser`

```rust
struct GzHeaderParser {
    state: GzHeaderState,
    flags: u8,
    header: GzHeader,
}
```

#### Implementations

- <span id="gzheaderparser-new"></span>`fn new() -> Self`

- <span id="gzheaderparser-parse"></span>`fn parse<R: BufRead>(&mut self, r: &mut R) -> Result<()>`

- <span id="gzheaderparser-header"></span>`fn header(&self) -> Option<&GzHeader>` — [`GzHeader`](#gzheader)

#### Trait Implementations

##### `impl Debug for GzHeaderParser`

- <span id="gzheaderparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GzHeaderParser`

- <span id="gzheaderparser-default"></span>`fn default() -> GzHeaderParser` — [`GzHeaderParser`](#gzheaderparser)

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

- <span id="gzbuilder-new"></span>`fn new() -> GzBuilder` — [`GzBuilder`](#gzbuilder)

  Create a new blank builder with no header by default.

- <span id="gzbuilder-mtime"></span>`fn mtime(self, mtime: u32) -> GzBuilder` — [`GzBuilder`](#gzbuilder)

  Configure the `mtime` field in the gzip header.

- <span id="gzbuilder-operating-system"></span>`fn operating_system(self, os: u8) -> GzBuilder` — [`GzBuilder`](#gzbuilder)

  Configure the `operating_system` field in the gzip header.

- <span id="gzbuilder-extra"></span>`fn extra<T: Into<Vec<u8>>>(self, extra: T) -> GzBuilder` — [`GzBuilder`](#gzbuilder)

  Configure the `extra` field in the gzip header.

- <span id="gzbuilder-filename"></span>`fn filename<T: Into<Vec<u8>>>(self, filename: T) -> GzBuilder` — [`GzBuilder`](#gzbuilder)

  Configure the `filename` field in the gzip header.

  

  # Panics

  

  Panics if the `filename` slice contains a zero.

- <span id="gzbuilder-comment"></span>`fn comment<T: Into<Vec<u8>>>(self, comment: T) -> GzBuilder` — [`GzBuilder`](#gzbuilder)

  Configure the `comment` field in the gzip header.

  

  # Panics

  

  Panics if the `comment` slice contains a zero.

- <span id="gzbuilder-write"></span>`fn write<W: Write>(self, w: W, lvl: Compression) -> write::GzEncoder<W>` — [`Compression`](../index.md#compression), [`GzEncoder`](write/index.md#gzencoder)

  Consume this builder, creating a writer encoder in the process.

  

  The data written to the returned encoder will be compressed and then

  written out to the supplied parameter `w`.

- <span id="gzbuilder-read"></span>`fn read<R: Read>(self, r: R, lvl: Compression) -> read::GzEncoder<R>` — [`Compression`](../index.md#compression), [`GzEncoder`](read/index.md#gzencoder)

  Consume this builder, creating a reader encoder in the process.

  

  Data read from the returned encoder will be the compressed version of

  the data read from the given reader.

- <span id="gzbuilder-buf-read"></span>`fn buf_read<R>(self, r: R, lvl: Compression) -> bufread::GzEncoder<R>` — [`Compression`](../index.md#compression), [`GzEncoder`](bufread/index.md#gzencoder)

  Consume this builder, creating a reader encoder in the process.

  

  Data read from the returned encoder will be the compressed version of

  the data read from the given reader.

- <span id="gzbuilder-into-header"></span>`fn into_header(self, lvl: Compression) -> Vec<u8>` — [`Compression`](../index.md#compression)

#### Trait Implementations

##### `impl Debug for GzBuilder`

- <span id="gzbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GzBuilder`

- <span id="gzbuilder-default"></span>`fn default() -> GzBuilder` — [`GzBuilder`](#gzbuilder)

## Enums

### `GzHeaderState`

```rust
enum GzHeaderState {
    Start(u8, [u8; 10]),
    Xlen(Option<Box<crate::Crc>>, u8, [u8; 2]),
    Extra(Option<Box<crate::Crc>>, u16),
    Filename(Option<Box<crate::Crc>>),
    Comment(Option<Box<crate::Crc>>),
    Crc(Option<Box<crate::Crc>>, u8, [u8; 2]),
    Complete,
}
```

#### Trait Implementations

##### `impl Debug for GzHeaderState`

- <span id="gzheaderstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GzHeaderState`

- <span id="gzheaderstate-default"></span>`fn default() -> GzHeaderState` — [`GzHeaderState`](#gzheaderstate)

## Functions

### `read_into`

```rust
fn read_into<R: Read>(r: &mut R, buffer: &mut [u8]) -> std::io::Result<usize>
```

### `read_to_nul`

```rust
fn read_to_nul<R: BufRead>(r: &mut R, buffer: &mut Vec<u8>) -> std::io::Result<()>
```

### `parse_le_u16`

```rust
fn parse_le_u16(buffer: &[u8; 2]) -> u16
```

### `bad_header`

```rust
fn bad_header() -> std::io::Error
```

### `corrupt`

```rust
fn corrupt() -> std::io::Error
```

## Constants

### `MAX_HEADER_BUF`
```rust
const MAX_HEADER_BUF: usize = 65_535usize;
```

