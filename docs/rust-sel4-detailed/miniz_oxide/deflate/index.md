*[miniz_oxide](../index.md) / [deflate](index.md)*

---

# Module `deflate`

This module contains functionality for compression.

## Contents

- [Modules](#modules)
  - [`buffer`](#buffer)
  - [`core`](#core)
  - [`stored`](#stored)
  - [`stream`](#stream)
  - [`zlib`](#zlib)
- [Enums](#enums)
  - [`CompressionLevel`](#compressionlevel)
- [Functions](#functions)
  - [`compress_to_vec`](#compress-to-vec)
  - [`compress_to_vec_zlib`](#compress-to-vec-zlib)
  - [`compress_to_vec_inner`](#compress-to-vec-inner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`buffer`](#buffer) | mod | Buffer wrappers implementing default so we can allocate the buffers with `Box::default()` to avoid stack copies. |
| [`core`](#core) | mod | Streaming compression functionality. |
| [`stored`](#stored) | mod |  |
| [`stream`](#stream) | mod | Extra streaming compression functionality. |
| [`zlib`](#zlib) | mod |  |
| [`CompressionLevel`](#compressionlevel) | enum | How much processing the compressor should do to compress the data. |
| [`compress_to_vec`](#compress-to-vec) | fn | Compress the input data to a vector, using the specified compression level (0-10). |
| [`compress_to_vec_zlib`](#compress-to-vec-zlib) | fn | Compress the input data to a vector, using the specified compression level (0-10), and with a zlib wrapper. |
| [`compress_to_vec_inner`](#compress-to-vec-inner) | fn | Simple function to compress data to a vec. |

## Modules

- [`buffer`](buffer/index.md) — Buffer wrappers implementing default so we can allocate the buffers with `Box::default()`
- [`core`](core/index.md) — Streaming compression functionality.
- [`stored`](stored/index.md)
- [`stream`](stream/index.md) — Extra streaming compression functionality.
- [`zlib`](zlib/index.md)

## Enums

### `CompressionLevel`

```rust
enum CompressionLevel {
    NoCompression,
    BestSpeed,
    BestCompression,
    UberCompression,
    DefaultLevel,
    DefaultCompression,
}
```

How much processing the compressor should do to compress the data.
`NoCompression` and `Bestspeed` have special meanings, the other levels determine the number
of checks for matches in the hash chains and whether to use lazy or greedy parsing.

#### Variants

- **`NoCompression`**

  Don't do any compression, only output uncompressed blocks.

- **`BestSpeed`**

  Fast compression. Uses a special compression routine that is optimized for speed.

- **`BestCompression`**

  Slow/high compression. Do a lot of checks to try to find good matches.

- **`UberCompression`**

  Even more checks, can be very slow.

- **`DefaultLevel`**

  Default compromise between speed and compression.

- **`DefaultCompression`**

  Use the default compression level.

#### Trait Implementations

##### `impl Clone for CompressionLevel`

- <span id="compressionlevel-clone"></span>`fn clone(&self) -> CompressionLevel` — [`CompressionLevel`](#compressionlevel)

##### `impl Copy for CompressionLevel`

##### `impl Debug for CompressionLevel`

- <span id="compressionlevel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressionLevel`

##### `impl Hash for CompressionLevel`

- <span id="compressionlevel-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressionLevel`

- <span id="compressionlevel-partialeq-eq"></span>`fn eq(&self, other: &CompressionLevel) -> bool` — [`CompressionLevel`](#compressionlevel)

##### `impl StructuralPartialEq for CompressionLevel`

## Functions

### `compress_to_vec`

```rust
fn compress_to_vec(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8>
```

Compress the input data to a vector, using the specified compression level (0-10).

### `compress_to_vec_zlib`

```rust
fn compress_to_vec_zlib(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8>
```

Compress the input data to a vector, using the specified compression level (0-10), and with a
zlib wrapper.

### `compress_to_vec_inner`

```rust
fn compress_to_vec_inner(input: &[u8], level: u8, window_bits: i32, strategy: i32) -> crate::alloc::vec::Vec<u8>
```

Simple function to compress data to a vec.

