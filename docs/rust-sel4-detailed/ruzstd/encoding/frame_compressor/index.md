*[ruzstd](../../index.md) / [encoding](../index.md) / [frame_compressor](index.md)*

---

# Module `frame_compressor`

Utilities and interfaces for encoding an entire frame. Allows reusing resources

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FrameCompressor`](#framecompressor) | struct | An interface for compressing arbitrary data with the ZStandard compression algorithm. |
| [`FseTables`](#fsetables) | struct |  |
| [`CompressState`](#compressstate) | struct |  |

## Structs

### `FrameCompressor<R: Read, W: Write, M: Matcher>`

```rust
struct FrameCompressor<R: Read, W: Write, M: Matcher> {
    uncompressed_data: Option<R>,
    compressed_data: Option<W>,
    compression_level: super::CompressionLevel,
    state: CompressState<M>,
    hasher: twox_hash::XxHash64,
}
```

An interface for compressing arbitrary data with the ZStandard compression algorithm.

`FrameCompressor` will generally be used by:
1. Initializing a compressor by providing a buffer of data using `FrameCompressor::new()`
2. Starting compression and writing that compression into a vec using `FrameCompressor::begin`

# Examples
```rust
use ruzstd::encoding::{FrameCompressor, CompressionLevel};
let mock_data: &[_] = &[0x1, 0x2, 0x3, 0x4];
let mut output = std::vec::Vec::new();
// Initialize a compressor.
let mut compressor = FrameCompressor::new(CompressionLevel::Uncompressed);
compressor.set_source(mock_data);
compressor.set_drain(&mut output);

// `compress` writes the compressed output into the provided buffer.
compressor.compress();
```

#### Implementations

- <span id="framecompressor-new"></span>`fn new(compression_level: CompressionLevel) -> Self` — [`CompressionLevel`](../index.md#compressionlevel)

  Create a new `FrameCompressor`

### `FseTables`

```rust
struct FseTables {
    ll_default: crate::fse::fse_encoder::FSETable,
    ll_previous: Option<crate::fse::fse_encoder::FSETable>,
    ml_default: crate::fse::fse_encoder::FSETable,
    ml_previous: Option<crate::fse::fse_encoder::FSETable>,
    of_default: crate::fse::fse_encoder::FSETable,
    of_previous: Option<crate::fse::fse_encoder::FSETable>,
}
```

#### Implementations

- <span id="fsetables-new"></span>`fn new() -> Self`

### `CompressState<M: Matcher>`

```rust
struct CompressState<M: Matcher> {
    matcher: M,
    last_huff_table: Option<crate::huff0::huff0_encoder::HuffmanTable>,
    fse_tables: FseTables,
}
```

