**flate2 > zlib > bufread**

# Module: zlib::bufread

## Contents

**Structs**

- [`ZlibDecoder`](#zlibdecoder) - A ZLIB decoder, or decompressor.
- [`ZlibEncoder`](#zlibencoder) - A ZLIB encoder, or compressor.

---

## flate2::zlib::bufread::ZlibDecoder

*Struct*

A ZLIB decoder, or decompressor.

This structure implements a [`Read`] interface. When read from, it reads
compressed data from the underlying [`BufRead`] and provides the uncompressed data.

After reading a single member of the ZLIB data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

# fn main() {
# let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
# e.write_all(b"Hello World").unwrap();
# let bytes = e.finish().unwrap();
# println!("{}", decode_bufreader(bytes).unwrap());
# }
#
// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead

fn decode_bufreader(bytes: Vec<u8>) -> io::Result<String> {
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
- `fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>` - Creates a new decoder which will decompress data read from the given
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



## flate2::zlib::bufread::ZlibEncoder

*Struct*

A ZLIB encoder, or compressor.

This structure implements a [`Read`] interface. When read from, it reads
uncompressed data from the underlying [`BufRead`] and provides the compressed data.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

# Examples

```
use std::io::prelude::*;
use flate2::Compression;
use flate2::bufread::ZlibEncoder;
use std::fs::File;
use std::io::BufReader;

// Use a buffered file to compress contents into a Vec<u8>

# fn open_hello_world() -> std::io::Result<Vec<u8>> {
let f = File::open("examples/hello_world.txt")?;
let b = BufReader::new(f);
let mut z = ZlibEncoder::new(b, Compression::fast());
let mut buffer = Vec::new();
z.read_to_end(&mut buffer)?;
# Ok(buffer)
# }
```

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>` - Creates a new encoder which will read uncompressed data from the given
- `fn new_with_compress(r: R, compression: Compress) -> ZlibEncoder<R>` - Creates a new encoder with the given `compression` settings which will
- `fn reset(self: & mut Self, r: R) -> R` - Resets the state of this encoder entirely, swapping out the input
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying reader
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream
- `fn into_inner(self: Self) -> R` - Consumes this encoder, returning the underlying reader.
- `fn total_in(self: &Self) -> u64` - Returns the number of bytes that have been read into this compressor.
- `fn total_out(self: &Self) -> u64` - Returns the number of bytes that the compressor has produced.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`



