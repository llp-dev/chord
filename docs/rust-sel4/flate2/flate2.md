**flate2**

# Module: flate2

## Contents

**Modules**

- [`bufread`](#bufread) - Types which operate over [`BufRead`] streams, both encoders and decoders for
- [`read`](#read) - Types which operate over [`Read`] streams, both encoders and decoders for
- [`write`](#write) - Types which operate over [`Write`] streams, both encoders and decoders for

**Structs**

- [`Compression`](#compression) - When compressing data, the compression level can be specified by a value in

---

## flate2::Compression

*Struct*

When compressing data, the compression level can be specified by a value in
this struct.

**Tuple Struct**: `()`

**Methods:**

- `fn new(level: u32) -> Compression` - Creates a new description of the compression level with an explicitly
- `fn none() -> Compression` - No compression is to be performed, this may actually inflate data
- `fn fast() -> Compression` - Optimize for the best speed of encoding.
- `fn best() -> Compression` - Optimize for the size of data being encoded.
- `fn level(self: &Self) -> u32` - Returns an integer representing the compression level, typically on a

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Compression`
- **Clone**
  - `fn clone(self: &Self) -> Compression`
- **PartialEq**
  - `fn eq(self: &Self, other: &Compression) -> bool`



## Module: bufread

Types which operate over [`BufRead`] streams, both encoders and decoders for
various formats.

[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html



## Module: read

Types which operate over [`Read`] streams, both encoders and decoders for
various formats.

Note that the `read` decoder types may read past the end of the compressed
data while decoding. If the caller requires subsequent reads to start
immediately following the compressed data  wrap the `Read` type in a
[`BufReader`] and use the `BufReader` with the equivalent decoder from the
`bufread` module and also for the subsequent reads.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html



## Module: write

Types which operate over [`Write`] streams, both encoders and decoders for
various formats.

[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html



