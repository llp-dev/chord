**miniz_oxide > deflate**

# Module: deflate

## Contents

**Modules**

- [`core`](#core) - Streaming compression functionality.
- [`stream`](#stream) - Extra streaming compression functionality.

**Enums**

- [`CompressionLevel`](#compressionlevel) - How much processing the compressor should do to compress the data.

**Functions**

- [`compress_to_vec`](#compress_to_vec) - Compress the input data to a vector, using the specified compression level (0-10).
- [`compress_to_vec_zlib`](#compress_to_vec_zlib) - Compress the input data to a vector, using the specified compression level (0-10), and with a

---

## miniz_oxide::deflate::CompressionLevel

*Enum*

How much processing the compressor should do to compress the data.
`NoCompression` and `Bestspeed` have special meanings, the other levels determine the number
of checks for matches in the hash chains and whether to use lazy or greedy parsing.

**Variants:**
- `NoCompression` - Don't do any compression, only output uncompressed blocks.
- `BestSpeed` - Fast compression. Uses a special compression routine that is optimized for speed.
- `BestCompression` - Slow/high compression. Do a lot of checks to try to find good matches.
- `UberCompression` - Even more checks, can be very slow.
- `DefaultLevel` - Default compromise between speed and compression.
- `DefaultCompression` - Use the default compression level.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CompressionLevel`
- **PartialEq**
  - `fn eq(self: &Self, other: &CompressionLevel) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## miniz_oxide::deflate::compress_to_vec

*Function*

Compress the input data to a vector, using the specified compression level (0-10).

```rust
fn compress_to_vec(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8>
```



## miniz_oxide::deflate::compress_to_vec_zlib

*Function*

Compress the input data to a vector, using the specified compression level (0-10), and with a
zlib wrapper.

```rust
fn compress_to_vec_zlib(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8>
```



## Module: core

Streaming compression functionality.



## Module: stream

Extra streaming compression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.

There is no DeflateState as the needed state is contained in the compressor struct itself.



