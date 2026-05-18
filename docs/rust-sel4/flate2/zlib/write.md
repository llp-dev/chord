**flate2 > zlib > write**

# Module: zlib::write

## Contents

**Structs**

- [`ZlibDecoder`](#zlibdecoder) - A ZLIB decoder, or decompressor.
- [`ZlibEncoder`](#zlibencoder) - A ZLIB encoder, or compressor.

---

## flate2::zlib::write::ZlibDecoder

*Struct*

A ZLIB decoder, or decompressor.

This structure implements a [`Write`] and will emit a stream of decompressed
data when fed a stream of compressed data.

After decoding a single member of the ZLIB data this writer will return the number of bytes up
to the end of the ZLIB member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the ZLIB member.

[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::ZlibEncoder;
use flate2::write::ZlibDecoder;

# fn main() {
#    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
#    e.write_all(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut z = ZlibDecoder::new(writer);
   z.write_all(&bytes[..])?;
   writer = z.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> ZlibDecoder<W>` - Creates a new decoder which will write uncompressed data to the stream.
- `fn new_with_decompress(w: W, decompression: Decompress) -> ZlibDecoder<W>` - Creates a new decoder which will write uncompressed data to the stream `w`
- `fn get_ref(self: &Self) -> &W` - Acquires a reference to the underlying writer.
- `fn get_mut(self: & mut Self) -> & mut W` - Acquires a mutable reference to the underlying writer.
- `fn reset(self: & mut Self, w: W) -> io::Result<W>` - Resets the state of this decoder entirely, swapping out the output
- `fn try_finish(self: & mut Self) -> io::Result<()>` - Attempt to finish this output stream, writing out final chunks of data.
- `fn finish(self: Self) -> io::Result<W>` - Consumes this encoder, flushing the output stream.
- `fn total_in(self: &Self) -> u64` - Returns the number of bytes that the decompressor has consumed for
- `fn total_out(self: &Self) -> u64` - Returns the number of bytes that the decompressor has written to its

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`



## flate2::zlib::write::ZlibEncoder

*Struct*

A ZLIB encoder, or compressor.

This structure implements a [`Write`] interface and takes a stream of
uncompressed data, writing the compressed data to the wrapped writer.

[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

# Examples

```
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

// Vec<u8> implements Write, assigning the compressed bytes of sample string

# fn zlib_encoding() -> std::io::Result<()> {
let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World")?;
let compressed = e.finish()?;
# Ok(())
# }
```

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W, level: crate::Compression) -> ZlibEncoder<W>` - Creates a new encoder which will write compressed data to the stream
- `fn new_with_compress(w: W, compression: Compress) -> ZlibEncoder<W>` - Creates a new encoder which will write compressed data to the stream
- `fn get_ref(self: &Self) -> &W` - Acquires a reference to the underlying writer.
- `fn get_mut(self: & mut Self) -> & mut W` - Acquires a mutable reference to the underlying writer.
- `fn reset(self: & mut Self, w: W) -> io::Result<W>` - Resets the state of this encoder entirely, swapping out the output
- `fn try_finish(self: & mut Self) -> io::Result<()>` - Attempt to finish this output stream, writing out final chunks of data.
- `fn finish(self: Self) -> io::Result<W>` - Consumes this encoder, flushing the output stream.
- `fn flush_finish(self: Self) -> io::Result<W>` - Consumes this encoder, flushing the output stream.
- `fn total_in(self: &Self) -> u64` - Returns the number of bytes that have been written to this compressor.
- `fn total_out(self: &Self) -> u64` - Returns the number of bytes that the compressor has produced.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`



