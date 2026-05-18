**ruzstd > encoding > frame_compressor**

# Module: encoding::frame_compressor

## Contents

**Structs**

- [`FrameCompressor`](#framecompressor) - An interface for compressing arbitrary data with the ZStandard compression algorithm.

---

## ruzstd::encoding::frame_compressor::FrameCompressor

*Struct*

An interface for compressing arbitrary data with the ZStandard compression algorithm.

`FrameCompressor` will generally be used by:
1. Initializing a compressor by providing a buffer of data using `FrameCompressor::new()`
2. Starting compression and writing that compression into a vec using `FrameCompressor::begin`

# Examples
```
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

**Generic Parameters:**
- R
- W
- M

**Methods:**

- `fn new_with_matcher(matcher: M, compression_level: CompressionLevel) -> Self` - Create a new `FrameCompressor` with a custom matching algorithm implementation
- `fn set_source(self: & mut Self, uncompressed_data: R) -> Option<R>` - Before calling [FrameCompressor::compress] you need to set the source.
- `fn set_drain(self: & mut Self, compressed_data: W) -> Option<W>` - Before calling [FrameCompressor::compress] you need to set the drain.
- `fn compress(self: & mut Self)` - Compress the uncompressed data from the provided source as one Zstd frame and write it to the provided drain
- `fn source_mut(self: & mut Self) -> Option<& mut R>` - Get a mutable reference to the source
- `fn drain_mut(self: & mut Self) -> Option<& mut W>` - Get a mutable reference to the drain
- `fn source(self: &Self) -> Option<&R>` - Get a reference to the source
- `fn drain(self: &Self) -> Option<&W>` - Get a reference to the drain
- `fn take_source(self: & mut Self) -> Option<R>` - Retrieve the source
- `fn take_drain(self: & mut Self) -> Option<W>` - Retrieve the drain
- `fn replace_matcher(self: & mut Self, match_generator: M) -> M` - Before calling [FrameCompressor::compress] you can replace the matcher
- `fn set_compression_level(self: & mut Self, compression_level: CompressionLevel) -> CompressionLevel` - Before calling [FrameCompressor::compress] you can replace the compression level
- `fn compression_level(self: &Self) -> CompressionLevel` - Get the current compression level
- `fn new(compression_level: CompressionLevel) -> Self` - Create a new `FrameCompressor`



