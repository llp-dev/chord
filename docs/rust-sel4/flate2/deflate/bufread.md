**flate2 > deflate > bufread**

# Module: deflate::bufread

## Contents

**Structs**

- [`DeflateDecoder`](#deflatedecoder) - A DEFLATE decoder, or decompressor.
- [`DeflateEncoder`](#deflateencoder) - A DEFLATE encoder, or compressor.

---

## flate2::deflate::bufread::DeflateDecoder

*Struct*

A DEFLATE decoder, or decompressor.

This structure implements a [`Read`] interface. When read from, it reads
compressed data from the underlying [`BufRead`] and provides the uncompressed data.

After reading a single member of the DEFLATE data this reader will return
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
# use flate2::write::DeflateEncoder;
use flate2::bufread::DeflateDecoder;

# fn main() {
#    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
#    e.write_all(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    println!("{}", decode_reader(bytes).unwrap());
# }
// Uncompresses a Deflate Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read
fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut deflater = DeflateDecoder::new(&bytes[..]);
   let mut s = String::new();
   deflater.read_to_string(&mut s)?;
   Ok(s)
}
```

**Generic Parameters:**
- R

**Methods:**

- `fn reset(self: & mut Self, r: R) -> R` - Resets the state of this decoder entirely, swapping out the input
- `fn reset_data(self: & mut Self)` - Resets the state of this decoder's data
- `fn get_ref(self: &Self) -> &R` - Acquires a reference to the underlying stream
- `fn get_mut(self: & mut Self) -> & mut R` - Acquires a mutable reference to the underlying stream
- `fn into_inner(self: Self) -> R` - Consumes this decoder, returning the underlying reader.
- `fn total_in(self: &Self) -> u64` - Returns the number of bytes that the decompressor has consumed.
- `fn total_out(self: &Self) -> u64` - Returns the number of bytes that the decompressor has produced.
- `fn new(r: R) -> DeflateDecoder<R>` - Creates a new decoder which will decompress data read from the given

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`



## flate2::deflate::bufread::DeflateEncoder

*Struct*

A DEFLATE encoder, or compressor.

This structure implements a [`Read`] interface. When read from, it reads
uncompressed data from the underlying [`BufRead`] and provides the compressed data.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

# Examples

```
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::bufread::DeflateEncoder;
use std::fs::File;
use std::io::BufReader;

# fn main() {
#    println!("{:?}", open_hello_world().unwrap());
# }
#
// Opens sample file, compresses the contents and returns a Vector
fn open_hello_world() -> io::Result<Vec<u8>> {
   let f = File::open("examples/hello_world.txt")?;
   let b = BufReader::new(f);
   let mut deflater = DeflateEncoder::new(b, Compression::fast());
   let mut buffer = Vec::new();
   deflater.read_to_end(&mut buffer)?;
   Ok(buffer)
}
```

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R, level: crate::Compression) -> DeflateEncoder<R>` - Creates a new encoder which will read uncompressed data from the given
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



