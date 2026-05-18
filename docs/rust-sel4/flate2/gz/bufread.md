**flate2 > gz > bufread**

# Module: gz::bufread

## Contents

**Structs**

- [`GzDecoder`](#gzdecoder) - A decoder for a single member of a [gzip file].
- [`GzEncoder`](#gzencoder) - A gzip streaming encoder
- [`MultiGzDecoder`](#multigzdecoder) - A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

---

## flate2::gz::bufread::GzDecoder

*Struct*

A decoder for a single member of a [gzip file].

This structure implements a [`Read`] interface. When read from, it reads
compressed data from the underlying [`BufRead`] and provides the uncompressed data.

After reading a single member of the gzip data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`]
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::GzEncoder;
use flate2::bufread::GzDecoder;

# fn main() {
#   let mut e = GzEncoder::new(Vec::new(), Compression::default());
#   e.write_all(b"Hello World").unwrap();
#   let bytes = e.finish().unwrap();
#   println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead

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

- `fn new(r: R) -> GzDecoder<R>` - Creates a new decoder from the given reader, immediately parsing the
- `fn header(self: &Self) -> Option<&GzHeader>` - Returns the header associated with this stream, if it was valid
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream.
- `fn into_inner(self: Self) -> R` - Consumes this decoder, returning the underlying reader.

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::gz::bufread::GzEncoder

*Struct*

A gzip streaming encoder

This structure implements a [`Read`] interface. When read from, it reads
uncompressed data from the underlying [`BufRead`] and provides the compressed data.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

# Examples

```
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



## flate2::gz::bufread::MultiGzDecoder

*Struct*

A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

This structure implements a [`Read`] interface. When read from, it reads
compressed data from the underlying [`BufRead`] and provides the uncompressed data.

A gzip file consists of a series of *members* concatenated one after another.
MultiGzDecoder decodes all members from the data and only returns Ok(0) when the
underlying reader does. For a file, this reads to the end of the file.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::GzEncoder;
use flate2::bufread::MultiGzDecoder;

# fn main() {
#   let mut e = GzEncoder::new(Vec::new(), Compression::default());
#   e.write_all(b"Hello World").unwrap();
#   let bytes = e.finish().unwrap();
#   println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = MultiGzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

**Generic Parameters:**
- R

**Tuple Struct**: `()`

**Methods:**

- `fn header(self: &Self) -> Option<&GzHeader>` - Returns the current header associated with this stream, if it's valid
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream.
- `fn into_inner(self: Self) -> R` - Consumes this decoder, returning the underlying reader.
- `fn new(r: R) -> MultiGzDecoder<R>` - Creates a new decoder from the given reader, immediately parsing the

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`



