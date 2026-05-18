**flate2 > gz > read**

# Module: gz::read

## Contents

**Structs**

- [`GzDecoder`](#gzdecoder) - A decoder for a single member of a [gzip file].
- [`GzEncoder`](#gzencoder) - A gzip streaming encoder
- [`MultiGzDecoder`](#multigzdecoder) - A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

---

## flate2::gz::read::GzDecoder

*Struct*

A decoder for a single member of a [gzip file].

This structure implements a [`Read`] interface. When read from, it reads
compressed data from the underlying [`Read`] and provides the uncompressed data.

After reading a single member of the gzip data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`GzDecoder` may have read additional bytes past the end of the gzip data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::GzDecoder` instead.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`]
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::GzEncoder;
use flate2::read::GzDecoder;

# fn main() {
#    let mut e = GzEncoder::new(Vec::new(), Compression::default());
#    e.write_all(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = GzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

**Generic Parameters:**
- R

**Methods:**

- `fn header(self: &Self) -> Option<&GzHeader>` - Returns the header associated with this stream, if it was valid.
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream.
- `fn into_inner(self: Self) -> R` - Consumes this decoder, returning the underlying reader.
- `fn new(r: R) -> GzDecoder<R>` - Creates a new decoder from the given reader, immediately parsing the

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::gz::read::GzEncoder

*Struct*

A gzip streaming encoder

This structure implements a [`Read`] interface. When read from, it reads
uncompressed data from the underlying [`Read`] and provides the compressed data.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html

# Examples

```
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

**Generic Parameters:**
- R

**Methods:**

- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying reader.
- `fn into_inner(self: Self) -> R` - Returns the underlying stream, consuming this encoder
- `fn new(r: R, level: Compression) -> GzEncoder<R>` - Creates a new encoder which will use the given compression level.

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::gz::read::MultiGzDecoder

*Struct*

A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

This structure implements a [`Read`] interface. When read from, it reads
compressed data from the underlying [`Read`] and provides the uncompressed
data.

A gzip file consists of a series of *members* concatenated one after another.
MultiGzDecoder decodes all members of a file and returns Ok(0) once the
underlying reader does.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::GzEncoder;
use flate2::read::MultiGzDecoder;

# fn main() {
#    let mut e = GzEncoder::new(Vec::new(), Compression::default());
#    e.write_all(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = MultiGzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

**Generic Parameters:**
- R

**Methods:**

- `fn header(self: &Self) -> Option<&GzHeader>` - Returns the current header associated with this stream, if it's valid.
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream.
- `fn into_inner(self: Self) -> R` - Consumes this decoder, returning the underlying reader.
- `fn new(r: R) -> MultiGzDecoder<R>` - Creates a new decoder from the given reader, immediately parsing the

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`



