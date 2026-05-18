**flate2 > gz**

# Module: gz

## Contents

**Structs**

- [`GzBuilder`](#gzbuilder) - A builder structure to create a new gzip Encoder.
- [`GzHeader`](#gzheader) - A structure representing the header of a gzip stream.
- [`GzHeaderParser`](#gzheaderparser)

---

## flate2::gz::GzBuilder

*Struct*

A builder structure to create a new gzip Encoder.

This structure controls header configuration options such as the filename.

# Examples

```
use std::io::prelude::*;
# use std::io;
use std::fs::File;
use flate2::GzBuilder;
use flate2::Compression;

// GzBuilder opens a file and writes a sample string using GzBuilder pattern

# fn sample_builder() -> Result<(), io::Error> {
let f = File::create("examples/hello_world.gz")?;
let mut gz = GzBuilder::new()
                .filename("hello_world.txt")
                .comment("test file, please delete")
                .write(f, Compression::default());
gz.write_all(b"hello world")?;
gz.finish()?;
# Ok(())
# }
```

**Methods:**

- `fn new() -> GzBuilder` - Create a new blank builder with no header by default.
- `fn mtime(self: Self, mtime: u32) -> GzBuilder` - Configure the `mtime` field in the gzip header.
- `fn operating_system(self: Self, os: u8) -> GzBuilder` - Configure the `operating_system` field in the gzip header.
- `fn extra<T>(self: Self, extra: T) -> GzBuilder` - Configure the `extra` field in the gzip header.
- `fn filename<T>(self: Self, filename: T) -> GzBuilder` - Configure the `filename` field in the gzip header.
- `fn comment<T>(self: Self, comment: T) -> GzBuilder` - Configure the `comment` field in the gzip header.
- `fn write<W>(self: Self, w: W, lvl: Compression) -> write::GzEncoder<W>` - Consume this builder, creating a writer encoder in the process.
- `fn read<R>(self: Self, r: R, lvl: Compression) -> read::GzEncoder<R>` - Consume this builder, creating a reader encoder in the process.
- `fn buf_read<R>(self: Self, r: R, lvl: Compression) -> bufread::GzEncoder<R>` - Consume this builder, creating a reader encoder in the process.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> GzBuilder`



## flate2::gz::GzHeader

*Struct*

A structure representing the header of a gzip stream.

The header can contain metadata about the file that was compressed, if
present.

**Methods:**

- `fn filename(self: &Self) -> Option<&[u8]>` - Returns the `filename` field of this gzip stream's header, if present.
- `fn extra(self: &Self) -> Option<&[u8]>` - Returns the `extra` field of this gzip stream's header, if present.
- `fn comment(self: &Self) -> Option<&[u8]>` - Returns the `comment` field of this gzip stream's header, if present.
- `fn operating_system(self: &Self) -> u8` - Returns the `operating_system` field of this gzip stream's header.
- `fn mtime(self: &Self) -> u32` - This gives the most recent modification time of the original file being compressed.
- `fn mtime_as_datetime(self: &Self) -> Option<time::SystemTime>` - Returns the most recent modification time represented by a date-time type.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &GzHeader) -> bool`
- **Default**
  - `fn default() -> GzHeader`
- **Clone**
  - `fn clone(self: &Self) -> GzHeader`



## flate2::gz::GzHeaderParser

*Struct*



