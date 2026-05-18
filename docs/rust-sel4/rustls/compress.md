**rustls > compress**

# Module: compress

## Contents

**Structs**

- [`CompressionFailed`](#compressionfailed) - A content-less error for when `CertCompressor::compress` fails.
- [`DecompressionFailed`](#decompressionfailed) - A content-less error for when `CertDecompressor::decompress` fails.

**Enums**

- [`CompressionCache`](#compressioncache) - An LRU cache for compressions.
- [`CompressionLevel`](#compressionlevel) - A hint for how many resources to dedicate to a compression.

**Functions**

- [`default_cert_compressors`](#default_cert_compressors) - Returns the supported `CertCompressor` implementations enabled
- [`default_cert_decompressors`](#default_cert_decompressors) - Returns the supported `CertDecompressor` implementations enabled

**Traits**

- [`CertCompressor`](#certcompressor) - An available certificate compression algorithm.
- [`CertDecompressor`](#certdecompressor) - An available certificate decompression algorithm.

---

## rustls::compress::CertCompressor

*Trait*

An available certificate compression algorithm.

**Methods:**

- `compress`: Compress `input`, returning the result.
- `algorithm`: Which algorithm this compressor handles.



## rustls::compress::CertDecompressor

*Trait*

An available certificate decompression algorithm.

**Methods:**

- `decompress`: Decompress `input`, writing the result to `output`.
- `algorithm`: Which algorithm this decompressor handles.



## rustls::compress::CompressionCache

*Enum*

An LRU cache for compressions.

The prospect of being able to reuse a given compression for many connections
means we can afford to spend more time on that compression (by passing
`CompressionLevel::Amortized` to the compressor).

**Variants:**
- `Disabled` - No caching happens, and compression happens each time using

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## rustls::compress::CompressionFailed

*Struct*

A content-less error for when `CertCompressor::compress` fails.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::compress::CompressionLevel

*Enum*

A hint for how many resources to dedicate to a compression.

**Variants:**
- `Interactive` - This compression is happening interactively during a handshake.
- `Amortized` - The compression may be amortized over many connections.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CompressionLevel) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CompressionLevel`



## rustls::compress::DecompressionFailed

*Struct*

A content-less error for when `CertDecompressor::decompress` fails.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::compress::default_cert_compressors

*Function*

Returns the supported `CertCompressor` implementations enabled
by crate features.

```rust
fn default_cert_compressors() -> &'static [&'static dyn CertCompressor]
```



## rustls::compress::default_cert_decompressors

*Function*

Returns the supported `CertDecompressor` implementations enabled
by crate features.

```rust
fn default_cert_decompressors() -> &'static [&'static dyn CertDecompressor]
```



