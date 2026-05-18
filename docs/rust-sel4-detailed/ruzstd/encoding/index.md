*[ruzstd](../index.md) / [encoding](index.md)*

---

# Module `encoding`

Structures and utilities used for compressing/encoding data into the Zstd format.

## Contents

- [Modules](#modules)
  - [`block_header`](#block-header)
  - [`blocks`](#blocks)
  - [`frame_header`](#frame-header)
  - [`match_generator`](#match-generator)
  - [`util`](#util)
  - [`frame_compressor`](#frame-compressor)
  - [`levels`](#levels)
- [Structs](#structs)
  - [`FrameCompressor`](#framecompressor)
  - [`MatchGeneratorDriver`](#matchgeneratordriver)
- [Enums](#enums)
  - [`CompressionLevel`](#compressionlevel)
  - [`Sequence`](#sequence)
- [Traits](#traits)
  - [`Matcher`](#matcher)
- [Functions](#functions)
  - [`compress`](#compress)
  - [`compress_to_vec`](#compress-to-vec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`block_header`](#block-header) | mod |  |
| [`blocks`](#blocks) | mod | After Magic_Number and Frame_Header, there are some number of blocks. |
| [`frame_header`](#frame-header) | mod | Utilities and representations for a frame header. |
| [`match_generator`](#match-generator) | mod | Matching algorithm used find repeated parts in the original data |
| [`util`](#util) | mod |  |
| [`frame_compressor`](#frame-compressor) | mod | Utilities and interfaces for encoding an entire frame. |
| [`levels`](#levels) | mod |  |
| [`FrameCompressor`](#framecompressor) | struct |  |
| [`MatchGeneratorDriver`](#matchgeneratordriver) | struct |  |
| [`CompressionLevel`](#compressionlevel) | enum | The compression mode used impacts the speed of compression, and resulting compression ratios. |
| [`Sequence`](#sequence) | enum | Sequences that a [`Matcher`] can produce |
| [`Matcher`](#matcher) | trait | Trait used by the encoder that users can use to extend the matching facilities with their own algorithm making their own tradeoffs between runtime, memory usage and compression ratio |
| [`compress`](#compress) | fn | Convenience function to compress some source into a target without reusing any resources of the compressor ```rust use ruzstd::encoding::{compress, CompressionLevel}; let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0]; let mut target = Vec::new(); compress(data, &mut target, CompressionLevel::Fastest); ``` |
| [`compress_to_vec`](#compress-to-vec) | fn | Convenience function to compress some source into a Vec without reusing any resources of the compressor ```rust use ruzstd::encoding::{compress_to_vec, CompressionLevel}; let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0]; let compressed = compress_to_vec(data, CompressionLevel::Fastest); ``` |

## Modules

- [`block_header`](block_header/index.md)
- [`blocks`](blocks/index.md) — After Magic_Number and Frame_Header, there are some number of blocks. Each frame must have at least one block,
- [`frame_header`](frame_header/index.md) — Utilities and representations for a frame header.
- [`match_generator`](match_generator/index.md) — Matching algorithm used find repeated parts in the original data
- [`util`](util/index.md)
- [`frame_compressor`](frame_compressor/index.md) — Utilities and interfaces for encoding an entire frame. Allows reusing resources
- [`levels`](levels/index.md)

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

- <span id="framecompressor-new"></span>`fn new(compression_level: CompressionLevel) -> Self` — [`CompressionLevel`](#compressionlevel)

  Create a new `FrameCompressor`

### `MatchGeneratorDriver`

```rust
struct MatchGeneratorDriver {
    vec_pool: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    suffix_pool: alloc::vec::Vec<SuffixStore>,
    match_generator: MatchGenerator,
    slice_size: usize,
}
```

This is the default implementation of the `Matcher` trait. It allocates and reuses the buffers when possible.

#### Implementations

- <span id="matchgeneratordriver-new"></span>`fn new(slice_size: usize, max_slices_in_window: usize) -> Self`

  slice_size says how big the slices should be that are allocated to work with

  max_slices_in_window says how many slices should at most be used while looking for matches

#### Trait Implementations

##### `impl Matcher for MatchGeneratorDriver`

- <span id="matchgeneratordriver-matcher-reset"></span>`fn reset(&mut self, _level: CompressionLevel)` — [`CompressionLevel`](#compressionlevel)

- <span id="matchgeneratordriver-matcher-window-size"></span>`fn window_size(&self) -> u64`

- <span id="matchgeneratordriver-matcher-get-next-space"></span>`fn get_next_space(&mut self) -> Vec<u8>`

- <span id="matchgeneratordriver-matcher-get-last-space"></span>`fn get_last_space(&mut self) -> &[u8]`

- <span id="matchgeneratordriver-matcher-commit-space"></span>`fn commit_space(&mut self, space: Vec<u8>)`

- <span id="matchgeneratordriver-matcher-start-matching"></span>`fn start_matching(&mut self, handle_sequence: impl FnMut(Sequence<'a>))` — [`Sequence`](#sequence)

- <span id="matchgeneratordriver-matcher-skip-matching"></span>`fn skip_matching(&mut self)`

## Enums

### `CompressionLevel`

```rust
enum CompressionLevel {
    Uncompressed,
    Fastest,
    Default,
    Better,
    Best,
}
```

The compression mode used impacts the speed of compression,
and resulting compression ratios. Faster compression will result
in worse compression ratios, and vice versa.

#### Variants

- **`Uncompressed`**

  This level does not compress the data at all, and simply wraps
  it in a Zstandard frame.

- **`Fastest`**

  This level is roughly equivalent to Zstd compression level 1

- **`Default`**

  This level is roughly equivalent to Zstd level 3,
  or the one used by the official compressor when no level
  is specified.
  
  UNIMPLEMENTED

- **`Better`**

  This level is roughly equivalent to Zstd level 7.
  
  UNIMPLEMENTED

- **`Best`**

  This level is roughly equivalent to Zstd level 11.
  
  UNIMPLEMENTED

#### Trait Implementations

##### `impl Clone for CompressionLevel`

- <span id="compressionlevel-clone"></span>`fn clone(&self) -> CompressionLevel` — [`CompressionLevel`](#compressionlevel)

##### `impl Copy for CompressionLevel`

### `Sequence<'data>`

```rust
enum Sequence<'data> {
    Triple {
        literals: &'data [u8],
        offset: usize,
        match_len: usize,
    },
    Literals {
        literals: &'data [u8],
    },
}
```

Sequences that a [`Matcher`](#matcher) can produce

#### Variants

- **`Triple`**

  Is encoded as a sequence for the decoder sequence execution.
  
  First the literals will be copied to the decoded data,
  then `match_len` bytes are copied from `offset` bytes back in the decoded data

- **`Literals`**

  This is returned as the last sequence in a block
  
  These literals will just be copied at the end of the sequence execution by the decoder

#### Trait Implementations

##### `impl Debug for Sequence<'data>`

- <span id="sequence-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Sequence<'data>`

##### `impl PartialEq for Sequence<'data>`

- <span id="sequence-partialeq-eq"></span>`fn eq(&self, other: &Sequence<'data>) -> bool` — [`Sequence`](#sequence)

##### `impl StructuralPartialEq for Sequence<'data>`

## Traits

### `Matcher`

```rust
trait Matcher { ... }
```

Trait used by the encoder that users can use to extend the matching facilities with their own algorithm
making their own tradeoffs between runtime, memory usage and compression ratio

This trait operates on buffers that represent the chunks of data the matching algorithm wants to work on.
Each one of these buffers is referred to as a *space*. One or more of these buffers represent the window
the decoder will need to decode the data again.

This library asks the Matcher for a new buffer using `get_next_space` to allow reusing of allocated buffers when they are no longer part of the
window of data that is being used for matching.

The library fills the buffer with data that is to be compressed and commits them back to the matcher using `commit_space`.

Then it will either call `start_matching` or, if the space is deemed not worth compressing, `skip_matching` is called.

This is repeated until no more data is left to be compressed.

#### Required Methods

- `fn get_next_space(&mut self) -> alloc::vec::Vec<u8>`

  Get a space where we can put data to be matched on. Will be encoded as one block. The maximum allowed size is 128 kB.

- `fn get_last_space(&mut self) -> &[u8]`

  Get a reference to the last commited space

- `fn commit_space(&mut self, space: alloc::vec::Vec<u8>)`

  Commit a space to the matcher so it can be matched against

- `fn skip_matching(&mut self)`

  Just process the data in the last commited space for future matching

- `fn start_matching(&mut self, handle_sequence: impl FnMut(Sequence<'a>))`

  Process the data in the last commited space for future matching AND generate matches for the data

- `fn reset(&mut self, level: CompressionLevel)`

  Reset this matcher so it can be used for the next new frame

- `fn window_size(&self) -> u64`

  The size of the window the decoder will need to execute all sequences produced by this matcher

#### Implementors

- [`MatchGeneratorDriver`](match_generator/index.md#matchgeneratordriver)

## Functions

### `compress`

```rust
fn compress<R: Read, W: Write>(source: R, target: W, level: CompressionLevel)
```

Convenience function to compress some source into a target without reusing any resources of the compressor
```rust
use ruzstd::encoding::{compress, CompressionLevel};
let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
let mut target = Vec::new();
compress(data, &mut target, CompressionLevel::Fastest);
```

### `compress_to_vec`

```rust
fn compress_to_vec<R: Read>(source: R, level: CompressionLevel) -> alloc::vec::Vec<u8>
```

Convenience function to compress some source into a Vec without reusing any resources of the compressor
```rust
use ruzstd::encoding::{compress_to_vec, CompressionLevel};
let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
let compressed = compress_to_vec(data, CompressionLevel::Fastest);
```

