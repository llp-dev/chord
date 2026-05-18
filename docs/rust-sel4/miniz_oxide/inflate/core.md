**miniz_oxide > inflate > core**

# Module: inflate::core

## Contents

**Modules**

- [`inflate_flags`](#inflate_flags) - Flags to [`decompress()`] to control how inflation works.

**Structs**

- [`DecompressorOxide`](#decompressoroxide) - Main decompression struct.

**Functions**

- [`decompress`](#decompress) - Main decompression function. Keeps decompressing data from `in_buf` until the `in_buf` is

**Constants**

- [`TINFL_LZ_DICT_SIZE`](#tinfl_lz_dict_size)

---

## miniz_oxide::inflate::core::DecompressorOxide

*Struct*

Main decompression struct.


**Methods:**

- `fn new() -> DecompressorOxide` - Create a new tinfl_decompressor with all fields set to 0.
- `fn init(self: & mut Self)` - Set the current state to `Start`.
- `fn adler32(self: &Self) -> Option<u32>` - Returns the adler32 checksum of the currently decompressed data.
- `fn adler32_header(self: &Self) -> Option<u32>` - Returns the adler32 that was read from the zlib header if it exists.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self` - Create a new tinfl_decompressor with all fields set to 0.
- **Clone**
  - `fn clone(self: &Self) -> DecompressorOxide`



## miniz_oxide::inflate::core::TINFL_LZ_DICT_SIZE

*Constant*: `usize`



## miniz_oxide::inflate::core::decompress

*Function*

Main decompression function. Keeps decompressing data from `in_buf` until the `in_buf` is
empty, `out` is full, the end of the deflate stream is hit, or there is an error in the
deflate stream.

# Arguments

`r` is a [`DecompressorOxide`] struct with the state of this stream.

`in_buf` is a reference to the compressed data that is to be decompressed. The decompressor will
start at the first byte of this buffer.

`out` is a reference to the buffer that will store the decompressed data, and that
stores previously decompressed data if any.

* The offset given by `out_pos` indicates where in the output buffer slice writing should start.
* If [`TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`] is not set, the output buffer is used in a
  wrapping manner, and it's size is required to be a power of 2.
* The decompression function normally needs access to 32KiB of the previously decompressed data
  (or to the beginning of the decompressed data if less than 32KiB has been decompressed.)
    - If this data is not available, decompression may fail.
    - Some deflate compressors allow specifying a window size which limits match distances to
      less than this, or alternatively an RLE mode where matches will only refer to the previous byte
      and thus allows a smaller output buffer. The window size can be specified in the zlib
      header structure, however, the header data should not be relied on to be correct.

`flags` indicates settings and status to the decompression function.
* The [`TINFL_FLAG_HAS_MORE_INPUT`] has to be specified if more compressed data is to be provided
  in a subsequent call to this function.
* See the the [`inflate_flags`] module for details on other flags.

# Returns

Returns a tuple containing the status of the compressor, the number of input bytes read, and the
number of bytes output to `out`.

```rust
fn decompress(r: & mut DecompressorOxide, in_buf: &[u8], out: & mut [u8], out_pos: usize, flags: u32) -> (TINFLStatus, usize, usize)
```



## Module: inflate_flags

Flags to [`decompress()`] to control how inflation works.

These define bits for a bitmask argument.



