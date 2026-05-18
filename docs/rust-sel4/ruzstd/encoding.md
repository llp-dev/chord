**ruzstd > encoding**

# Module: encoding

## Contents

**Enums**

- [`CompressionLevel`](#compressionlevel) - The compression mode used impacts the speed of compression,
- [`Sequence`](#sequence) - Sequences that a [`Matcher`] can produce

**Functions**

- [`compress`](#compress) - Convenience function to compress some source into a target without reusing any resources of the compressor
- [`compress_to_vec`](#compress_to_vec) - Convenience function to compress some source into a Vec without reusing any resources of the compressor

**Traits**

- [`Matcher`](#matcher) - Trait used by the encoder that users can use to extend the matching facilities with their own algorithm

---

## ruzstd::encoding::CompressionLevel

*Enum*

The compression mode used impacts the speed of compression,
and resulting compression ratios. Faster compression will result
in worse compression ratios, and vice versa.

**Variants:**
- `Uncompressed` - This level does not compress the data at all, and simply wraps
- `Fastest` - This level is roughly equivalent to Zstd compression level 1
- `Default` - This level is roughly equivalent to Zstd level 3,
- `Better` - This level is roughly equivalent to Zstd level 7.
- `Best` - This level is roughly equivalent to Zstd level 11.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CompressionLevel`



## ruzstd::encoding::Matcher

*Trait*

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

**Methods:**

- `get_next_space`: Get a space where we can put data to be matched on. Will be encoded as one block. The maximum allowed size is 128 kB.
- `get_last_space`: Get a reference to the last commited space
- `commit_space`: Commit a space to the matcher so it can be matched against
- `skip_matching`: Just process the data in the last commited space for future matching
- `start_matching`: Process the data in the last commited space for future matching AND generate matches for the data
- `reset`: Reset this matcher so it can be used for the next new frame
- `window_size`: The size of the window the decoder will need to execute all sequences produced by this matcher



## ruzstd::encoding::Sequence

*Enum*

Sequences that a [`Matcher`] can produce

**Generic Parameters:**
- 'data

**Variants:**
- `Triple{ literals: &'data [u8], offset: usize, match_len: usize }` - Is encoded as a sequence for the decoder sequence execution.
- `Literals{ literals: &'data [u8] }` - This is returned as the last sequence in a block

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Sequence<'data>) -> bool`



## ruzstd::encoding::compress

*Function*

Convenience function to compress some source into a target without reusing any resources of the compressor
```rust
use ruzstd::encoding::{compress, CompressionLevel};
let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
let mut target = Vec::new();
compress(data, &mut target, CompressionLevel::Fastest);
```

```rust
fn compress<R, W>(source: R, target: W, level: CompressionLevel)
```



## ruzstd::encoding::compress_to_vec

*Function*

Convenience function to compress some source into a Vec without reusing any resources of the compressor
```rust
use ruzstd::encoding::{compress_to_vec, CompressionLevel};
let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
let compressed = compress_to_vec(data, CompressionLevel::Fastest);
```

```rust
fn compress_to_vec<R>(source: R, level: CompressionLevel) -> alloc::vec::Vec<u8>
```



