**flate2 > gz > write**

# Module: gz::write

## Contents

**Structs**

- [`GzDecoder`](#gzdecoder) - A decoder for a single member of a [gzip file].
- [`GzEncoder`](#gzencoder) - A gzip streaming encoder
- [`MultiGzDecoder`](#multigzdecoder) - A gzip streaming decoder that decodes a [gzip file] with multiple members.

---

## flate2::gz::write::GzDecoder

*Struct*

A decoder for a single member of a [gzip file].

This structure exposes a [`Write`] interface, receiving compressed data and
writing uncompressed data to the underlying writer.

After decoding a single member of the gzip data this writer will return the number of bytes up to
to the end of the gzip member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the gzip member.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`]
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

# Examples

```
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};

# fn main() {
#    let mut e = GzEncoder::new(Vec::new(), Compression::default());
#    e.write(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    assert_eq!("Hello World", decode_writer(bytes).unwrap());
# }
// Uncompresses a gzip encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write
fn decode_writer(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut decoder = GzDecoder::new(writer);
   decoder.write_all(&bytes[..])?;
   writer = decoder.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> GzDecoder<W>` - Creates a new decoder which will write uncompressed data to the stream.
- `fn header(self: &Self) -> Option<&GzHeader>` - Returns the header associated with this stream.
- `fn get_ref(self: &Self) -> &W` - Acquires a reference to the underlying writer.
- `fn get_mut(self: & mut Self) -> & mut W` - Acquires a mutable reference to the underlying writer.
- `fn try_finish(self: & mut Self) -> io::Result<()>` - Attempt to finish this output stream, writing out final chunks of data.
- `fn finish(self: Self) -> io::Result<W>` - Consumes this decoder, flushing the output stream.

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`



## flate2::gz::write::GzEncoder

*Struct*

A gzip streaming encoder

This structure exposes a [`Write`] interface that will emit compressed data
to the underlying writer `W`.

[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

# Examples

```
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;

// Vec<u8> implements Write to print the compressed bytes of sample string
# fn main() {

let mut e = GzEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World").unwrap();
println!("{:?}", e.finish().unwrap());
# }
```

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W, level: Compression) -> GzEncoder<W>` - Creates a new encoder which will use the given compression level.
- `fn get_ref(self: &Self) -> &W` - Acquires a reference to the underlying writer.
- `fn get_mut(self: & mut Self) -> & mut W` - Acquires a mutable reference to the underlying writer.
- `fn try_finish(self: & mut Self) -> io::Result<()>` - Attempt to finish this output stream, writing out final chunks of data.
- `fn finish(self: Self) -> io::Result<W>` - Finish encoding this stream, returning the underlying writer once the

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## flate2::gz::write::MultiGzDecoder

*Struct*

A gzip streaming decoder that decodes a [gzip file] with multiple members.

This structure exposes a [`Write`] interface that will consume compressed data and
write uncompressed data to the underlying writer.

A gzip file consists of a series of *members* concatenated one after another.
`MultiGzDecoder` decodes all members of a file and writes them to the
underlying writer one after another.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> MultiGzDecoder<W>` - Creates a new decoder which will write uncompressed data to the stream.
- `fn header(self: &Self) -> Option<&GzHeader>` - Returns the header associated with the current member.
- `fn get_ref(self: &Self) -> &W` - Acquires a reference to the underlying writer.
- `fn get_mut(self: & mut Self) -> & mut W` - Acquires a mutable reference to the underlying writer.
- `fn try_finish(self: & mut Self) -> io::Result<()>` - Attempt to finish this output stream, writing out final chunks of data.
- `fn finish(self: Self) -> io::Result<W>` - Consumes this decoder, flushing the output stream.

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



