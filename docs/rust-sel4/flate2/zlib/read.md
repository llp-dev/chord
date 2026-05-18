**flate2 > zlib > read**

# Module: zlib::read

## Contents

**Structs**

- [`ZlibDecoder`](#zlibdecoder) - A ZLIB decoder, or decompressor.
- [`ZlibEncoder`](#zlibencoder) - A ZLIB encoder, or compressor.

---

## flate2::zlib::read::ZlibDecoder

*Struct*

A ZLIB decoder, or decompressor.

This structure implements a [`Read`] interface. When read from, it reads
compressed data from the underlying [`Read`] and provides the uncompressed data.

After reading a single member of the ZLIB data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`ZlibDecoder` may have read additional bytes past the end of the ZLIB data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::ZlibDecoder` instead.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

# fn main() {
# let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
# e.write_all(b"Hello World").unwrap();
# let bytes = e.finish().unwrap();
# println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
```

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> ZlibDecoder<R>` - Creates a new decoder which will decompress data read from the given
- `fn new_with_buf(r: R, buf: Vec<u8>) -> ZlibDecoder<R>` - Creates a new decoder which will decompress data read from the given
- `fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>` - Creates a new decoder which will decompress data read from the given
- `fn new_with_decompress_and_buf(r: R, buf: Vec<u8>, decompression: Decompress) -> ZlibDecoder<R>` - Creates a new decoder which will decompress data read from the given
- `fn reset(self: & mut Self, r: R) -> R` - Resets the state of this decoder entirely, swapping out the input
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying stream
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream
- `fn into_inner(self: Self) -> R` - Consumes this decoder, returning the underlying reader.
- `fn total_in(self: &Self) -> u64` - Returns the number of bytes that the decompressor has consumed.
- `fn total_out(self: &Self) -> u64` - Returns the number of bytes that the decompressor has produced.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`



## flate2::zlib::read::ZlibEncoder

*Struct*

A ZLIB encoder, or compressor.

This structure implements a [`Read`] interface. When read from, it reads
uncompressed data from the underlying [`Read`] and provides the compressed data.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html

# Examples

```
use std::io::prelude::*;
use flate2::Compression;
use flate2::read::ZlibEncoder;
use std::fs::File;

// Open example file and compress the contents using Read interface

# fn open_hello_world() -> std::io::Result<Vec<u8>> {
let f = File::open("examples/hello_world.txt")?;
let mut z = ZlibEncoder::new(f, Compression::fast());
let mut buffer = Vec::new();
z.read_to_end(&mut buffer)?;
# Ok(buffer)
# }
```

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>` - Creates a new encoder which will read uncompressed data from the given
- `fn new_with_compress(r: R, compression: crate::Compress) -> ZlibEncoder<R>` - Creates a new encoder with the given `compression` settings which will
- `fn reset(self: & mut Self, r: R) -> R` - Resets the state of this encoder entirely, swapping out the input
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying stream
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream
- `fn into_inner(self: Self) -> R` - Consumes this encoder, returning the underlying reader.
- `fn total_in(self: &Self) -> u64` - Returns the number of bytes that have been read into this compressor.
- `fn total_out(self: &Self) -> u64` - Returns the number of bytes that the compressor has produced.

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`



