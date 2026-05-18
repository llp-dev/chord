**miniz_oxide > deflate > core**

# Module: deflate::core

## Contents

**Modules**

- [`deflate_flags`](#deflate_flags)

**Structs**

- [`CallbackFunc`](#callbackfunc) - Callback function and user used in `compress_to_output`.
- [`CompressorOxide`](#compressoroxide) - Main compression struct.

**Enums**

- [`CompressionStrategy`](#compressionstrategy) - Strategy setting for compression.
- [`TDEFLFlush`](#tdeflflush) - A list of deflate flush types.
- [`TDEFLStatus`](#tdeflstatus) - Return status of compression.

**Functions**

- [`compress`](#compress) - Main compression function. Tries to compress as much as possible from `in_buf` and
- [`compress_to_output`](#compress_to_output) - Main compression function. Callbacks output.
- [`create_comp_flags_from_zip_params`](#create_comp_flags_from_zip_params) - Create a set of compression flags using parameters used by zlib and other compressors.

---

## miniz_oxide::deflate::core::CallbackFunc

*Struct*

Callback function and user used in `compress_to_output`.

**Generic Parameters:**
- 'a

**Fields:**
- `put_buf_func: &'a  mut dyn FnMut`



## miniz_oxide::deflate::core::CompressionStrategy

*Enum*

Strategy setting for compression.

The non-default settings offer some special-case compression variants.

**Variants:**
- `Default` - Don't use any of the special strategies.
- `Filtered` - Only use matches that are at least 5 bytes long.
- `HuffmanOnly` - Don't look for matches, only huffman encode the literals.
- `RLE` - Only look for matches with a distance of 1, i.e do run-length encoding only.
- `Fixed` - Only use static/fixed blocks. (Blocks using the default huffman codes

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &CompressionStrategy) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CompressionStrategy`



## miniz_oxide::deflate::core::CompressorOxide

*Struct*

Main compression struct.

**Methods:**

- `fn new(flags: u32) -> Self` - Create a new `CompressorOxide` with the given flags.
- `fn adler32(self: &Self) -> u32` - Get the adler32 checksum of the currently encoded data.
- `fn prev_return_status(self: &Self) -> TDEFLStatus` - Get the return status of the previous [`compress`](fn.compress.html)
- `fn flags(self: &Self) -> i32` - Get the raw compressor flags.
- `fn data_format(self: &Self) -> DataFormat` - Returns whether the compressor is wrapping the data in a zlib format or not.
- `fn reset(self: & mut Self)` - Reset the state of the compressor, keeping the same parameters.
- `fn set_compression_level(self: & mut Self, level: CompressionLevel)` - Set the compression level of the compressor.
- `fn set_compression_level_raw(self: & mut Self, level: u8)` - Set the compression level of the compressor using an integer value.
- `fn set_format_and_level(self: & mut Self, data_format: DataFormat, level: u8)` - Update the compression settings of the compressor.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self` - Initialize the compressor with a level of 4, zlib wrapper and



## miniz_oxide::deflate::core::TDEFLFlush

*Enum*

A list of deflate flush types.

**Variants:**
- `None` - Normal operation.
- `Sync` - Try to flush all the current data and output an empty raw block.
- `Full` - Same as [`Sync`][Self::Sync], but reset the dictionary so that the following data does not
- `Finish` - Try to flush everything and end the deflate stream.

**Methods:**

- `fn new(flush: i32) -> core::result::Result<Self, MZError>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> TDEFLFlush`
- **From**
  - `fn from(flush: MZFlush) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &TDEFLFlush) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## miniz_oxide::deflate::core::TDEFLStatus

*Enum*

Return status of compression.

**Variants:**
- `BadParam` - Usage error.
- `PutBufFailed` - Error putting data into output buffer.
- `Okay` - Compression succeeded normally.
- `Done` - Compression succeeded and the deflate stream was ended.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &TDEFLStatus) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> TDEFLStatus`



## miniz_oxide::deflate::core::compress

*Function*

Main compression function. Tries to compress as much as possible from `in_buf` and
puts compressed output into `out_buf`.

The value of `flush` determines if the compressor should attempt to flush all output
and alternatively try to finish the stream.

Use [`TDEFLFlush::Finish`] on the final call to signal that the stream is finishing.

Note that this function does not keep track of whether a flush marker has been output, so
if called using [`TDEFLFlush::Sync`], the caller needs to ensure there is enough space in the
output buffer if they want to avoid repeated flush markers.
See #105 for details.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer and the current position in the output buffer.

```rust
fn compress(d: & mut CompressorOxide, in_buf: &[u8], out_buf: & mut [u8], flush: TDEFLFlush) -> (TDEFLStatus, usize, usize)
```



## miniz_oxide::deflate::core::compress_to_output

*Function*

Main compression function. Callbacks output.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer.

The caller is responsible for ensuring the `CallbackFunc` struct will not cause undefined
behaviour.

```rust
fn compress_to_output<impl FnMut(&[u8]) -> bool>(d: & mut CompressorOxide, in_buf: &[u8], flush: TDEFLFlush, callback_func: impl Trait) -> (TDEFLStatus, usize)
```



## miniz_oxide::deflate::core::create_comp_flags_from_zip_params

*Function*

Create a set of compression flags using parameters used by zlib and other compressors.
Mainly intended for use with transition from c libraries as it deals with raw integers.

# Parameters
`level` determines compression level. Clamped to maximum of 10. Negative values result in
`CompressionLevel::DefaultLevel`.
`window_bits`: Above 0, wraps the stream in a zlib wrapper, 0 or negative for a raw deflate
stream.
`strategy`: Sets the strategy if this conforms to any of the values in `CompressionStrategy`.

# Notes
This function may be removed or moved to the `miniz_oxide_c_api` in the future.

```rust
fn create_comp_flags_from_zip_params(level: i32, window_bits: i32, strategy: i32) -> u32
```



## Module: deflate_flags



