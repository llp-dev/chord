*[miniz_oxide](../../index.md) / [deflate](../index.md) / [core](index.md)*

---

# Module `core`

Streaming compression functionality.

## Contents

- [Modules](#modules)
  - [`deflate_flags`](#deflate-flags)
- [Structs](#structs)
  - [`Error`](#error)
  - [`SymFreq`](#symfreq)
  - [`CompressorOxide`](#compressoroxide)
  - [`CallbackFunc`](#callbackfunc)
  - [`CallbackBuf`](#callbackbuf)
  - [`CallbackOxide`](#callbackoxide)
  - [`OutputBufferOxide`](#outputbufferoxide)
  - [`SavedOutputBufferOxide`](#savedoutputbufferoxide)
  - [`BitBuffer`](#bitbuffer)
  - [`HuffmanOxide`](#huffmanoxide)
  - [`Rle`](#rle)
  - [`DictOxide`](#dictoxide)
  - [`ParamsOxide`](#paramsoxide)
  - [`LZOxide`](#lzoxide)
- [Enums](#enums)
  - [`CompressionStrategy`](#compressionstrategy)
  - [`TDEFLFlush`](#tdeflflush)
  - [`TDEFLStatus`](#tdeflstatus)
  - [`CallbackOut`](#callbackout)
- [Functions](#functions)
  - [`read_u16_le`](#read-u16-le)
  - [`change_window_bits_from_format`](#change-window-bits-from-format)
  - [`limit_level_by_window_bits`](#limit-level-by-window-bits)
  - [`write`](#write)
  - [`probes_from_flags`](#probes-from-flags)
  - [`compress_lz_codes`](#compress-lz-codes)
  - [`compress_block`](#compress-block)
  - [`flush_block`](#flush-block)
  - [`record_literal`](#record-literal)
  - [`record_match`](#record-match)
  - [`compress_normal`](#compress-normal)
  - [`compress_fast`](#compress-fast)
  - [`flush_output_buffer`](#flush-output-buffer)
  - [`compress`](#compress)
  - [`compress_to_output`](#compress-to-output)
  - [`compress_inner`](#compress-inner)
  - [`create_comp_flags_from_zip_params`](#create-comp-flags-from-zip-params)
  - [`window_bits_from_flags`](#window-bits-from-flags)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Constants](#constants)
  - [`MAX_PROBES_MASK`](#max-probes-mask)
  - [`MAX_SUPPORTED_HUFF_CODESIZE`](#max-supported-huff-codesize)
  - [`LEN_SYM`](#len-sym)
  - [`LEN_SYM_OFFSET`](#len-sym-offset)
  - [`LEN_EXTRA`](#len-extra)
  - [`SMALL_DIST_SYM`](#small-dist-sym)
  - [`SMALL_DIST_EXTRA`](#small-dist-extra)
  - [`LARGE_DIST_SYM`](#large-dist-sym)
  - [`LARGE_DIST_EXTRA`](#large-dist-extra)
  - [`BITMASKS`](#bitmasks)
  - [`NUM_PROBES`](#num-probes)
  - [`MAX_HUFF_SYMBOLS`](#max-huff-symbols)
  - [`LEVEL1_HASH_SIZE_MASK`](#level1-hash-size-mask)
  - [`MAX_HUFF_TABLES`](#max-huff-tables)
  - [`MAX_HUFF_SYMBOLS_0`](#max-huff-symbols-0)
  - [`MAX_HUFF_SYMBOLS_1`](#max-huff-symbols-1)
  - [`MAX_HUFF_SYMBOLS_2`](#max-huff-symbols-2)
  - [`LZ_DICT_SIZE`](#lz-dict-size)
  - [`LZ_DICT_SIZE_MASK`](#lz-dict-size-mask)
  - [`MIN_MATCH_LEN`](#min-match-len)
  - [`MAX_MATCH_LEN`](#max-match-len)
  - [`DEFAULT_FLAGS`](#default-flags)
  - [`LITLEN_TABLE`](#litlen-table)
  - [`DIST_TABLE`](#dist-table)
  - [`HUFF_CODES_TABLE`](#huff-codes-table)
  - [`COMP_FAST_LOOKAHEAD_SIZE`](#comp-fast-lookahead-size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deflate_flags`](#deflate-flags) | mod |  |
| [`Error`](#error) | struct |  |
| [`SymFreq`](#symfreq) | struct |  |
| [`CompressorOxide`](#compressoroxide) | struct | Main compression struct. |
| [`CallbackFunc`](#callbackfunc) | struct | Callback function and user used in `compress_to_output`. |
| [`CallbackBuf`](#callbackbuf) | struct |  |
| [`CallbackOxide`](#callbackoxide) | struct |  |
| [`OutputBufferOxide`](#outputbufferoxide) | struct |  |
| [`SavedOutputBufferOxide`](#savedoutputbufferoxide) | struct |  |
| [`BitBuffer`](#bitbuffer) | struct |  |
| [`HuffmanOxide`](#huffmanoxide) | struct | A struct containing data about huffman codes and symbol frequencies. |
| [`Rle`](#rle) | struct | Status of RLE encoding of huffman code lengths. |
| [`DictOxide`](#dictoxide) | struct |  |
| [`ParamsOxide`](#paramsoxide) | struct |  |
| [`LZOxide`](#lzoxide) | struct |  |
| [`CompressionStrategy`](#compressionstrategy) | enum | Strategy setting for compression. |
| [`TDEFLFlush`](#tdeflflush) | enum | A list of deflate flush types. |
| [`TDEFLStatus`](#tdeflstatus) | enum | Return status of compression. |
| [`CallbackOut`](#callbackout) | enum |  |
| [`read_u16_le`](#read-u16-le) | fn |  |
| [`change_window_bits_from_format`](#change-window-bits-from-format) | fn |  |
| [`limit_level_by_window_bits`](#limit-level-by-window-bits) | fn | Limit compression settings by window_bits as a simple way to implement smaller window sizes |
| [`write`](#write) | fn |  |
| [`probes_from_flags`](#probes-from-flags) | fn |  |
| [`compress_lz_codes`](#compress-lz-codes) | fn |  |
| [`compress_block`](#compress-block) | fn |  |
| [`flush_block`](#flush-block) | fn |  |
| [`record_literal`](#record-literal) | fn |  |
| [`record_match`](#record-match) | fn |  |
| [`compress_normal`](#compress-normal) | fn |  |
| [`compress_fast`](#compress-fast) | fn |  |
| [`flush_output_buffer`](#flush-output-buffer) | fn |  |
| [`compress`](#compress) | fn | Main compression function. |
| [`compress_to_output`](#compress-to-output) | fn | Main compression function. |
| [`compress_inner`](#compress-inner) | fn |  |
| [`create_comp_flags_from_zip_params`](#create-comp-flags-from-zip-params) | fn | Create a set of compression flags using parameters used by zlib and other compressors. |
| [`window_bits_from_flags`](#window-bits-from-flags) | fn | Check if the window is |
| [`Result`](#result) | type |  |
| [`MAX_PROBES_MASK`](#max-probes-mask) | const |  |
| [`MAX_SUPPORTED_HUFF_CODESIZE`](#max-supported-huff-codesize) | const |  |
| [`LEN_SYM`](#len-sym) | const |  |
| [`LEN_SYM_OFFSET`](#len-sym-offset) | const |  |
| [`LEN_EXTRA`](#len-extra) | const | Number of extra bits for length values. |
| [`SMALL_DIST_SYM`](#small-dist-sym) | const | Distance codes for distances smaller than 512. |
| [`SMALL_DIST_EXTRA`](#small-dist-extra) | const | Number of extra bits for distances smaller than 512. |
| [`LARGE_DIST_SYM`](#large-dist-sym) | const | Base values to calculate distances above 512. |
| [`LARGE_DIST_EXTRA`](#large-dist-extra) | const | Number of extra bits distances above 512. |
| [`BITMASKS`](#bitmasks) | const |  |
| [`NUM_PROBES`](#num-probes) | const | The maximum number of checks for matches in the hash table the compressor will make for each compression level. |
| [`MAX_HUFF_SYMBOLS`](#max-huff-symbols) | const |  |
| [`LEVEL1_HASH_SIZE_MASK`](#level1-hash-size-mask) | const | Size of hash chain for fast compression mode. |
| [`MAX_HUFF_TABLES`](#max-huff-tables) | const | The number of huffman tables used by the compressor. |
| [`MAX_HUFF_SYMBOLS_0`](#max-huff-symbols-0) | const | Literal/length codes |
| [`MAX_HUFF_SYMBOLS_1`](#max-huff-symbols-1) | const | Distance codes. |
| [`MAX_HUFF_SYMBOLS_2`](#max-huff-symbols-2) | const | Huffman length values. |
| [`LZ_DICT_SIZE`](#lz-dict-size) | const | Size of the chained hash table. |
| [`LZ_DICT_SIZE_MASK`](#lz-dict-size-mask) | const | Mask used when stepping through the hash chains. |
| [`MIN_MATCH_LEN`](#min-match-len) | const | The minimum length of a match. |
| [`MAX_MATCH_LEN`](#max-match-len) | const | The maximum length of a match. |
| [`DEFAULT_FLAGS`](#default-flags) | const |  |
| [`LITLEN_TABLE`](#litlen-table) | const | Tables used for literal/lengths in `HuffmanOxide`. |
| [`DIST_TABLE`](#dist-table) | const | Tables for distances. |
| [`HUFF_CODES_TABLE`](#huff-codes-table) | const | Tables for the run-length encoded huffman lengths for literals/lengths/distances. |
| [`COMP_FAST_LOOKAHEAD_SIZE`](#comp-fast-lookahead-size) | const |  |

## Modules

- [`deflate_flags`](deflate_flags/index.md)

## Structs

### `Error`

```rust
struct Error {
}
```

### `SymFreq`

```rust
struct SymFreq {
    key: u16,
    sym_index: u16,
}
```

#### Trait Implementations

##### `impl Clone for SymFreq`

- <span id="symfreq-clone"></span>`fn clone(&self) -> SymFreq` — [`SymFreq`](#symfreq)

##### `impl Copy for SymFreq`

### `CompressorOxide`

```rust
struct CompressorOxide {
    lz: LZOxide,
    params: ParamsOxide,
    huff: alloc::boxed::Box<HuffmanOxide>,
    dict: DictOxide,
}
```

Main compression struct.

#### Fields

- **`huff`**: `alloc::boxed::Box<HuffmanOxide>`

  Put HuffmanOxide on the heap with default trick to avoid
  excessive stack copies.

#### Implementations

- <span id="compressoroxide-new"></span>`fn new(flags: u32) -> Self`

  Create a new `CompressorOxide` with the given flags.

  

  # Notes

  This function may be changed to take different parameters in the future.

- <span id="compressoroxide-with-format-and-level"></span>`fn with_format_and_level(data_format: DataFormat, level: CompressionLevel) -> CompressorOxide` — [`DataFormat`](../../index.md#dataformat), [`CompressionLevel`](../index.md#compressionlevel), [`CompressorOxide`](#compressoroxide)

  Create a new `CompressorOxide` with the given flags.

- <span id="compressoroxide-with-params"></span>`fn with_params(data_format: DataFormat, level: u8, strategy: CompressionStrategy, window_bits: u8) -> CompressorOxide` — [`DataFormat`](../../index.md#dataformat), [`CompressionStrategy`](#compressionstrategy), [`CompressorOxide`](#compressoroxide)

  Create a new 'CompressorOxide with the current format, level, strategy and window bits

  

  

  Level will is limited to 10, and window bits is clamped to 15

- <span id="compressoroxide-adler32"></span>`const fn adler32(&self) -> u32`

  Get the adler32 checksum of the currently encoded data.

- <span id="compressoroxide-prev-return-status"></span>`const fn prev_return_status(&self) -> TDEFLStatus` — [`TDEFLStatus`](#tdeflstatus)

  Get the return status of the previous [`compress`](#compress)

  call with this compressor.

- <span id="compressoroxide-flags"></span>`const fn flags(&self) -> i32`

  Get the raw compressor flags.

  

  # Notes

  This function may be deprecated or changed in the future to use more rust-style flags.

- <span id="compressoroxide-data-format"></span>`const fn data_format(&self) -> DataFormat` — [`DataFormat`](../../index.md#dataformat)

  Returns whether the compressor is wrapping the data in a zlib format or not.

- <span id="compressoroxide-reset"></span>`fn reset(&mut self)`

  Reset the state of the compressor, keeping the same parameters.

  

  This avoids re-allocating data.

- <span id="compressoroxide-set-compression-level"></span>`fn set_compression_level(&mut self, level: CompressionLevel)` — [`CompressionLevel`](../index.md#compressionlevel)

  Set the compression level of the compressor.

  

  Changing compression level after compression has started will likely result in failure.

  # Notes

  The compression strategy will be reset to the default one when this is called.

  

  If using the zlib wrapper, increasing the compression level

  to one that requires a higher window_bits value

  than the max set at initialization will fail and leave the

  compressor at the current level.

- <span id="compressoroxide-set-compression-level-raw"></span>`fn set_compression_level_raw(&mut self, level: u8)`

  Set the compression level of the compressor using an integer value.

  

  Changing compression level after compression has started will likely result in failure.

  # Notes

  The compression strategy will be reset to the default one when this is called.

  

  If using the zlib wrapper, increasing the compression level

  to one that requires a higher window_bits value

  than the max set at initialization will fail and leave the

  compressor at the current level.

- <span id="compressoroxide-set-format-and-level"></span>`fn set_format_and_level(&mut self, data_format: DataFormat, level: u8)` — [`DataFormat`](../../index.md#dataformat)

  Update the compression settings of the compressor.

  

  Changing the `DataFormat` after compression has started will result in

  a corrupted stream.

  

  Changing compression level after compression has started will likely result in failure.

  

  # Notes

  This function mainly intended for setting the initial settings after e.g creating with

  `default` or after calling `CompressorOxide::reset()`, and behaviour may be changed

  to disallow calling it after starting compression in the future.

  

  If using the zlib wrapper, increasing the compression level

  to one that requires a higher window_bits value

  than the max set at initialization will fail and leave the

  compressor at the current level.

- <span id="compressoroxide-unwritten-bit-count"></span>`const fn unwritten_bit_count(&self) -> u32`

  Check the number of unwritten bits after the last flush.

  After a `NoSync` flush it can be used to test whether the

  stream is aligned with a byte boundary.

#### Trait Implementations

##### `impl Clone for CompressorOxide`

- <span id="compressoroxide-clone"></span>`fn clone(&self) -> CompressorOxide` — [`CompressorOxide`](#compressoroxide)

##### `impl Default for CompressorOxide`

- <span id="compressoroxide-default"></span>`fn default() -> Self`

  Initialize the compressor with a level of 4, zlib wrapper and

  the default strategy.

### `CallbackFunc<'a>`

```rust
struct CallbackFunc<'a> {
    pub put_buf_func: &'a mut dyn FnMut(&[u8]) -> bool,
}
```

Callback function and user used in `compress_to_output`.

#### Implementations

- <span id="callbackfunc-flush-output"></span>`fn flush_output(&mut self, saved_output: SavedOutputBufferOxide, params: &mut ParamsOxide) -> i32` — [`SavedOutputBufferOxide`](#savedoutputbufferoxide), [`ParamsOxide`](#paramsoxide)

### `CallbackBuf<'a>`

```rust
struct CallbackBuf<'a> {
    pub out_buf: &'a mut [u8],
}
```

#### Implementations

- <span id="callbackbuf-flush-output"></span>`fn flush_output(&mut self, saved_output: SavedOutputBufferOxide, params: &mut ParamsOxide) -> i32` — [`SavedOutputBufferOxide`](#savedoutputbufferoxide), [`ParamsOxide`](#paramsoxide)

### `CallbackOxide<'a>`

```rust
struct CallbackOxide<'a> {
    in_buf: Option<&'a [u8]>,
    in_buf_size: Option<&'a mut usize>,
    out_buf_size: Option<&'a mut usize>,
    out: CallbackOut<'a>,
}
```

#### Implementations

- <span id="callbackoxide-new-callback-buf"></span>`fn new_callback_buf(in_buf: &'a [u8], out_buf: &'a mut [u8]) -> Self`

- <span id="callbackoxide-new-callback-func"></span>`fn new_callback_func(in_buf: &'a [u8], callback_func: CallbackFunc<'a>) -> Self` — [`CallbackFunc`](#callbackfunc)

- <span id="callbackoxide-update-size"></span>`fn update_size(&mut self, in_size: Option<usize>, out_size: Option<usize>)`

- <span id="callbackoxide-flush-output"></span>`fn flush_output(&mut self, saved_output: SavedOutputBufferOxide, params: &mut ParamsOxide) -> i32` — [`SavedOutputBufferOxide`](#savedoutputbufferoxide), [`ParamsOxide`](#paramsoxide)

- <span id="callbackoxide-buf"></span>`fn buf(&mut self) -> Option<&'a [u8]>`

### `OutputBufferOxide<'a>`

```rust
struct OutputBufferOxide<'a> {
    pub inner: &'a mut [u8],
    pub inner_pos: usize,
    pub local: bool,
    pub bit_buffer: u32,
    pub bits_in: u32,
}
```

#### Implementations

- <span id="outputbufferoxide-put-bits"></span>`fn put_bits(&mut self, bits: u32, len: u32)`

  Write bits to the bit buffer and flushes

  the bit buffer so any whole bytes are output

  to the underlying buffer.

- <span id="outputbufferoxide-put-bits-no-flush"></span>`fn put_bits_no_flush(&mut self, bits: u32, len: u32)`

  Write the provided bits to the bit buffer without flushing

  anything. Does not check if there is actually space for it.

- <span id="outputbufferoxide-save"></span>`const fn save(&self) -> SavedOutputBufferOxide` — [`SavedOutputBufferOxide`](#savedoutputbufferoxide)

- <span id="outputbufferoxide-load"></span>`fn load(&mut self, saved: SavedOutputBufferOxide)` — [`SavedOutputBufferOxide`](#savedoutputbufferoxide)

- <span id="outputbufferoxide-pad-to-bytes"></span>`fn pad_to_bytes(&mut self)`

  Pad the bit buffer to a whole byte with

  zeroes and write that byte to the output buffer.

- <span id="outputbufferoxide-is-byte-aligned"></span>`const fn is_byte_aligned(&self) -> bool`

  Test whether the output is currently on a byte boundary,

  i.e. all current data has been output

- <span id="outputbufferoxide-write-bytes"></span>`fn write_bytes(&mut self, bytes: &[u8])`

### `SavedOutputBufferOxide`

```rust
struct SavedOutputBufferOxide {
    pub pos: usize,
    pub bit_buffer: u32,
    pub bits_in: u32,
    pub local: bool,
}
```

### `BitBuffer`

```rust
struct BitBuffer {
    pub bit_buffer: u64,
    pub bits_in: u32,
}
```

#### Implementations

- <span id="bitbuffer-put-fast"></span>`fn put_fast(&mut self, bits: u64, len: u32)`

- <span id="bitbuffer-flush"></span>`fn flush(&mut self, output: &mut OutputBufferOxide<'_>) -> core::result::Result<(), Error>` — [`OutputBufferOxide`](#outputbufferoxide), [`Error`](#error)

### `HuffmanOxide`

```rust
struct HuffmanOxide {
    pub count: [[u16; 288]; 3],
    pub codes: [[u16; 288]; 3],
    pub code_sizes: [[u8; 288]; 3],
}
```

A struct containing data about huffman codes and symbol frequencies.

NOTE: Only the literal/lengths have enough symbols to actually use
the full array. It's unclear why it's defined like this in miniz,
it could be for cache/alignment reasons.

#### Fields

- **`count`**: `[[u16; 288]; 3]`

  Number of occurrences of each symbol.

- **`codes`**: `[[u16; 288]; 3]`

  The bits of the huffman code assigned to the symbol

- **`code_sizes`**: `[[u8; 288]; 3]`

  The length of the huffman code assigned to the symbol.

#### Implementations

- <span id="huffmanoxide-radix-sort-symbols"></span>`fn radix_sort_symbols<'a>(symbols0: &'a mut [SymFreq], symbols1: &'a mut [SymFreq]) -> &'a mut [SymFreq]` — [`SymFreq`](#symfreq)

- <span id="huffmanoxide-calculate-minimum-redundancy"></span>`fn calculate_minimum_redundancy(symbols: &mut [SymFreq])` — [`SymFreq`](#symfreq)

- <span id="huffmanoxide-enforce-max-code-size"></span>`fn enforce_max_code_size(num_codes: &mut [i32], code_list_len: usize, max_code_size: usize)`

- <span id="huffmanoxide-optimize-table"></span>`fn optimize_table(&mut self, table_num: usize, table_len: usize, code_size_limit: usize, static_table: bool)`

- <span id="huffmanoxide-start-static-block"></span>`fn start_static_block(&mut self, output: &mut OutputBufferOxide<'_>)` — [`OutputBufferOxide`](#outputbufferoxide)

- <span id="huffmanoxide-start-dynamic-block"></span>`fn start_dynamic_block(&mut self, output: &mut OutputBufferOxide<'_>) -> core::result::Result<(), Error>` — [`OutputBufferOxide`](#outputbufferoxide), [`Error`](#error)

#### Trait Implementations

##### `impl Clone for HuffmanOxide`

- <span id="huffmanoxide-clone"></span>`fn clone(&self) -> HuffmanOxide` — [`HuffmanOxide`](#huffmanoxide)

##### `impl Default for HuffmanOxide`

- <span id="huffmanoxide-default"></span>`fn default() -> Self`

### `Rle`

```rust
struct Rle {
    pub z_count: u32,
    pub repeat_count: u16,
    pub prev_code_size: u8,
}
```

Status of RLE encoding of huffman code lengths.

#### Implementations

- <span id="rle-prev-code-size"></span>`fn prev_code_size(&mut self, packed_code_sizes: &mut [u8], packed_pos: &mut usize, h: &mut HuffmanOxide) -> core::result::Result<(), Error>` — [`HuffmanOxide`](#huffmanoxide), [`Error`](#error)

- <span id="rle-zero-code-size"></span>`fn zero_code_size(&mut self, packed_code_sizes: &mut [u8], packed_pos: &mut usize, h: &mut HuffmanOxide) -> core::result::Result<(), Error>` — [`HuffmanOxide`](#huffmanoxide), [`Error`](#error)

### `DictOxide`

```rust
struct DictOxide {
    pub max_probes: [u32; 2],
    pub b: crate::deflate::buffer::HashBuffers,
    pub code_buf_dict_pos: usize,
    pub lookahead_size: usize,
    pub lookahead_pos: usize,
    pub size: usize,
    loop_len: u8,
}
```

#### Fields

- **`max_probes`**: `[u32; 2]`

  The maximum number of checks in the hash chain, for the initial,
  and the lazy match respectively.

- **`b`**: `crate::deflate::buffer::HashBuffers`

  Buffer of input data.
  Padded with 1 byte to simplify matching code in `compress_fast`.

#### Implementations

- <span id="dictoxide-new"></span>`fn new(flags: u32) -> Self`

- <span id="dictoxide-update-flags"></span>`fn update_flags(&mut self, flags: u32)`

- <span id="dictoxide-reset"></span>`fn reset(&mut self)`

- <span id="dictoxide-read-unaligned-u32"></span>`fn read_unaligned_u32(&self, pos: usize) -> u32`

  Do an unaligned read of the data at `pos` in the dictionary and treat it as if it was of

  type T.

- <span id="dictoxide-read-unaligned-u64"></span>`fn read_unaligned_u64(&self, pos: usize) -> u64`

  Do an unaligned read of the data at `pos` in the dictionary and treat it as if it was of

  type T.

- <span id="dictoxide-find-match"></span>`fn find_match(&self, lookahead_pos: usize, max_dist: usize, max_match_len: u32, match_dist: u32, match_len: u32) -> (u32, u32)`

  Try to find a match for the data at lookahead_pos in the dictionary that is

  longer than `match_len`.

  Returns a tuple containing (match_distance, match_length). Will be equal to the input

  values if no better matches were found.

#### Trait Implementations

##### `impl Clone for DictOxide`

- <span id="dictoxide-clone"></span>`fn clone(&self) -> DictOxide` — [`DictOxide`](#dictoxide)

### `ParamsOxide`

```rust
struct ParamsOxide {
    pub flags: u32,
    pub greedy_parsing: bool,
    pub window_bits_max: u8,
    pub block_index: u32,
    pub saved_match_dist: u32,
    pub saved_match_len: u32,
    pub saved_lit: u8,
    pub flush: TDEFLFlush,
    pub flush_ofs: u32,
    pub flush_remaining: u32,
    pub finished: bool,
    pub adler32: u32,
    pub src_pos: usize,
    pub out_buf_ofs: usize,
    pub prev_return_status: TDEFLStatus,
    pub saved_bit_buffer: u32,
    pub saved_bits_in: u32,
    pub local_buf: alloc::boxed::Box<crate::deflate::buffer::LocalBuf>,
}
```

#### Implementations

- <span id="paramsoxide-new"></span>`fn new(flags: u32, window_bits: u8) -> Self`

- <span id="paramsoxide-update-flags"></span>`fn update_flags(&mut self, flags: u32)`

- <span id="paramsoxide-reset"></span>`fn reset(&mut self)`

  Reset state, saving settings.

#### Trait Implementations

##### `impl Clone for ParamsOxide`

- <span id="paramsoxide-clone"></span>`fn clone(&self) -> ParamsOxide` — [`ParamsOxide`](#paramsoxide)

### `LZOxide`

```rust
struct LZOxide {
    pub codes: [u8; 65536],
    pub code_position: usize,
    pub flag_position: usize,
    pub total_bytes: u32,
    pub num_flags_left: u32,
}
```

#### Implementations

- <span id="lzoxide-new"></span>`const fn new() -> Self`

- <span id="lzoxide-write-code"></span>`fn write_code(&mut self, val: u8)`

- <span id="lzoxide-init-flag"></span>`fn init_flag(&mut self)`

- <span id="lzoxide-get-flag"></span>`fn get_flag(&mut self) -> &mut u8`

- <span id="lzoxide-plant-flag"></span>`fn plant_flag(&mut self)`

- <span id="lzoxide-consume-flag"></span>`fn consume_flag(&mut self)`

#### Trait Implementations

##### `impl Clone for LZOxide`

- <span id="lzoxide-clone"></span>`fn clone(&self) -> LZOxide` — [`LZOxide`](#lzoxide)

## Enums

### `CompressionStrategy`

```rust
enum CompressionStrategy {
    Default,
    Filtered,
    HuffmanOnly,
    RLE,
    Fixed,
}
```

Strategy setting for compression.

The non-default settings offer some special-case compression variants.

#### Variants

- **`Default`**

  Don't use any of the special strategies.

- **`Filtered`**

  Only use matches that are at least 5 bytes long.

- **`HuffmanOnly`**

  Don't look for matches, only huffman encode the literals.
  (This is not optimally implemented at the moment and will not provide much of a speedup)

- **`RLE`**

  Only look for matches with a distance of 1, i.e do run-length encoding only.

- **`Fixed`**

  Only use static/fixed blocks. (Blocks using the default huffman codes
  specified in the deflate specification.)

#### Trait Implementations

##### `impl Clone for CompressionStrategy`

- <span id="compressionstrategy-clone"></span>`fn clone(&self) -> CompressionStrategy` — [`CompressionStrategy`](#compressionstrategy)

##### `impl Copy for CompressionStrategy`

##### `impl Debug for CompressionStrategy`

- <span id="compressionstrategy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressionStrategy`

##### `impl Hash for CompressionStrategy`

- <span id="compressionstrategy-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressionStrategy`

- <span id="compressionstrategy-partialeq-eq"></span>`fn eq(&self, other: &CompressionStrategy) -> bool` — [`CompressionStrategy`](#compressionstrategy)

##### `impl StructuralPartialEq for CompressionStrategy`

### `TDEFLFlush`

```rust
enum TDEFLFlush {
    None,
    Partial,
    Sync,
    Full,
    Finish,
    PartialOpt,
    SyncOpt,
    NoSync,
}
```

A list of deflate flush types.

#### Variants

- **`None`**

  Normal operation.
  
  Compress as much as there is space for, and then return waiting for more input.

- **`Partial`**

  Try to flush all the current data and output an empty fixed
  block (10 bits) to synchonize the stream.

- **`Sync`**

  Try to flush all the current data and output an empty raw
  block (3-10 bits + 32 bits) to synchonize the stream.

- **`Full`**

  Same as `Sync`, but reset the dictionary so that the following data does not
  depend on previous data.

- **`Finish`**

  Try to flush everything and end the deflate stream.
  
  On success this will yield a [`TDEFLStatus::Done`](../../index.md) return status.

- **`PartialOpt`**

  Try to flush all the current data and, if data is unaligned,
  output an empty fixed block (10 bits) to synchonize the
  stream.

- **`SyncOpt`**

  Try to flush all the current data and, if data is unaligned,
  output an empty raw block (3-10 bits + 32 bits) to synchonize
  the stream.

- **`NoSync`**

  Try to flush the current data but without any sync, which may
  leave up to 7 bits of data not output.  You can use
  `TDEFLFlush::PartialOpt` or `TDEFLFlush::SyncOpt` to add a
  sync sequence on a future call if you later decide that you
  have space downstream to forward that final byte.

#### Implementations

- <span id="tdeflflush-new"></span>`const fn new(flush: i32) -> core::result::Result<Self, MZError>` — [`MZError`](../../index.md#mzerror)

#### Trait Implementations

##### `impl Clone for TDEFLFlush`

- <span id="tdeflflush-clone"></span>`fn clone(&self) -> TDEFLFlush` — [`TDEFLFlush`](#tdeflflush)

##### `impl Copy for TDEFLFlush`

##### `impl Debug for TDEFLFlush`

- <span id="tdeflflush-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TDEFLFlush`

##### `impl Hash for TDEFLFlush`

- <span id="tdeflflush-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TDEFLFlush`

- <span id="tdeflflush-partialeq-eq"></span>`fn eq(&self, other: &TDEFLFlush) -> bool` — [`TDEFLFlush`](#tdeflflush)

##### `impl StructuralPartialEq for TDEFLFlush`

### `TDEFLStatus`

```rust
enum TDEFLStatus {
    BadParam,
    PutBufFailed,
    Okay,
    Done,
}
```

Return status of compression.

#### Variants

- **`BadParam`**

  Usage error.
  
  This indicates that either the [`CompressorOxide`](#compressoroxide) experienced a previous error, or the
  stream has already been [`TDEFLFlush::Finish`](../../index.md)'d.

- **`PutBufFailed`**

  Error putting data into output buffer.
  
  This usually indicates a too-small buffer.

- **`Okay`**

  Compression succeeded normally.

- **`Done`**

  Compression succeeded and the deflate stream was ended.
  
  This is the result of calling compression with [`TDEFLFlush::Finish`](../../index.md).

#### Trait Implementations

##### `impl Clone for TDEFLStatus`

- <span id="tdeflstatus-clone"></span>`fn clone(&self) -> TDEFLStatus` — [`TDEFLStatus`](#tdeflstatus)

##### `impl Copy for TDEFLStatus`

##### `impl Debug for TDEFLStatus`

- <span id="tdeflstatus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TDEFLStatus`

##### `impl Hash for TDEFLStatus`

- <span id="tdeflstatus-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TDEFLStatus`

- <span id="tdeflstatus-partialeq-eq"></span>`fn eq(&self, other: &TDEFLStatus) -> bool` — [`TDEFLStatus`](#tdeflstatus)

##### `impl StructuralPartialEq for TDEFLStatus`

### `CallbackOut<'a>`

```rust
enum CallbackOut<'a> {
    Func(CallbackFunc<'a>),
    Buf(CallbackBuf<'a>),
}
```

#### Implementations

- <span id="callbackout-new-output-buffer"></span>`fn new_output_buffer<'b>(self: &'b mut Self, local_buf: &'b mut [u8], out_buf_ofs: usize) -> OutputBufferOxide<'b>` — [`OutputBufferOxide`](#outputbufferoxide)

## Functions

### `read_u16_le`

```rust
const fn read_u16_le<const N: usize>(slice: &[u8; N], pos: usize) -> u16
```

### `change_window_bits_from_format`

```rust
const fn change_window_bits_from_format(window_bits: u8, data_format: crate::DataFormat) -> i32
```

### `limit_level_by_window_bits`

```rust
fn limit_level_by_window_bits(window_bits: u8, current_level: i32, current_strategy: CompressionStrategy) -> (i32, CompressionStrategy)
```

Limit compression settings by window_bits as a simple way to implement smaller window sizes

### `write`

```rust
fn write(src: &[u8], dst: &mut [u8], dst_pos: &mut usize) -> core::result::Result<(), Error>
```

### `probes_from_flags`

```rust
const fn probes_from_flags(flags: u32) -> [u32; 2]
```

### `compress_lz_codes`

```rust
fn compress_lz_codes(huff: &HuffmanOxide, output: &mut OutputBufferOxide<'_>, lz_code_buf: &[u8; 65536], lz_code_buf_used_len: usize) -> core::result::Result<bool, Error>
```

### `compress_block`

```rust
fn compress_block(huff: &mut HuffmanOxide, output: &mut OutputBufferOxide<'_>, lz: &LZOxide, static_block: bool) -> core::result::Result<bool, Error>
```

### `flush_block`

```rust
fn flush_block(d: &mut CompressorOxide, callback: &mut CallbackOxide<'_>, flush: TDEFLFlush) -> core::result::Result<i32, Error>
```

### `record_literal`

```rust
fn record_literal(h: &mut HuffmanOxide, lz: &mut LZOxide, lit: u8)
```

### `record_match`

```rust
fn record_match(h: &mut HuffmanOxide, lz: &mut LZOxide, match_len: u32, match_dist: u32)
```

### `compress_normal`

```rust
fn compress_normal(d: &mut CompressorOxide, callback: &mut CallbackOxide<'_>) -> bool
```

### `compress_fast`

```rust
fn compress_fast(d: &mut CompressorOxide, callback: &mut CallbackOxide<'_>) -> bool
```

### `flush_output_buffer`

```rust
fn flush_output_buffer(c: &mut CallbackOxide<'_>, p: &mut ParamsOxide) -> (TDEFLStatus, usize, usize)
```

### `compress`

```rust
fn compress(d: &mut CompressorOxide, in_buf: &[u8], out_buf: &mut [u8], flush: TDEFLFlush) -> (TDEFLStatus, usize, usize)
```

Main compression function. Tries to compress as much as possible from `in_buf` and
puts compressed output into `out_buf`.

The value of `flush` determines if the compressor should attempt to flush all output
and alternatively try to finish the stream.

Use [`TDEFLFlush::Finish`](../../index.md) on the final call to signal that the stream is finishing.

Note that this function does not keep track of whether a flush marker has been output, so
if called using [`TDEFLFlush::Sync`](../../index.md), the caller needs to ensure there is enough space in the
output buffer if they want to avoid repeated flush markers.
See #105 for details.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer and the current position in the output buffer.
A result of [`TDEFLStatus::Done`](../../index.md) indicates that compression is finished, and further calls to this function will
result in [`TDEFLStatus::BadParam`](../../index.md).
See [`TDEFLStatus`](#tdeflstatus) for other return values.

### `compress_to_output`

```rust
fn compress_to_output(d: &mut CompressorOxide, in_buf: &[u8], flush: TDEFLFlush, callback_func: impl FnMut(&[u8]) -> bool) -> (TDEFLStatus, usize)
```

Main compression function. Callbacks output.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer.

The caller is responsible for ensuring the `CallbackFunc` struct will not cause undefined
behaviour.

### `compress_inner`

```rust
fn compress_inner(d: &mut CompressorOxide, callback: &mut CallbackOxide<'_>, flush: TDEFLFlush) -> (TDEFLStatus, usize, usize)
```

### `create_comp_flags_from_zip_params`

```rust
const fn create_comp_flags_from_zip_params(level: i32, window_bits: i32, strategy: i32) -> u32
```

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

### `window_bits_from_flags`

```rust
const fn window_bits_from_flags(flags: u32) -> u8
```

Check if the window is

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = core::result::Result<T, E>;
```

## Constants

### `MAX_PROBES_MASK`
```rust
const MAX_PROBES_MASK: u32 = 4_095u32;
```

### `MAX_SUPPORTED_HUFF_CODESIZE`
```rust
const MAX_SUPPORTED_HUFF_CODESIZE: usize = 15usize;
```

### `LEN_SYM`
```rust
const LEN_SYM: [u8; 256];
```

### `LEN_SYM_OFFSET`
```rust
const LEN_SYM_OFFSET: usize = 256usize;
```

### `LEN_EXTRA`
```rust
const LEN_EXTRA: [u8; 256];
```

Number of extra bits for length values.

### `SMALL_DIST_SYM`
```rust
const SMALL_DIST_SYM: [u8; 512];
```

Distance codes for distances smaller than 512.

### `SMALL_DIST_EXTRA`
```rust
const SMALL_DIST_EXTRA: [u8; 128];
```

Number of extra bits for distances smaller than 512.
Reduced by 4 using bit shift

### `LARGE_DIST_SYM`
```rust
const LARGE_DIST_SYM: [u8; 128];
```

Base values to calculate distances above 512.

### `LARGE_DIST_EXTRA`
```rust
const LARGE_DIST_EXTRA: [u8; 128];
```

Number of extra bits distances above 512.

### `BITMASKS`
```rust
const BITMASKS: [u32; 17];
```

### `NUM_PROBES`
```rust
const NUM_PROBES: [u16; 11];
```

The maximum number of checks for matches in the hash table the compressor will make for each
compression level.

### `MAX_HUFF_SYMBOLS`
```rust
const MAX_HUFF_SYMBOLS: usize = 288usize;
```

### `LEVEL1_HASH_SIZE_MASK`
```rust
const LEVEL1_HASH_SIZE_MASK: u32 = 4_095u32;
```

Size of hash chain for fast compression mode.

### `MAX_HUFF_TABLES`
```rust
const MAX_HUFF_TABLES: usize = 3usize;
```

The number of huffman tables used by the compressor.
Literal/length, Distances and Length of the huffman codes for the other two tables.

### `MAX_HUFF_SYMBOLS_0`
```rust
const MAX_HUFF_SYMBOLS_0: usize = 288usize;
```

Literal/length codes

### `MAX_HUFF_SYMBOLS_1`
```rust
const MAX_HUFF_SYMBOLS_1: usize = 32usize;
```

Distance codes.

### `MAX_HUFF_SYMBOLS_2`
```rust
const MAX_HUFF_SYMBOLS_2: usize = 19usize;
```

Huffman length values.

### `LZ_DICT_SIZE`
```rust
const LZ_DICT_SIZE: usize = 32_768usize;
```

Size of the chained hash table.

### `LZ_DICT_SIZE_MASK`
```rust
const LZ_DICT_SIZE_MASK: usize = 32_767usize;
```

Mask used when stepping through the hash chains.

### `MIN_MATCH_LEN`
```rust
const MIN_MATCH_LEN: u8 = 3u8;
```

The minimum length of a match.

### `MAX_MATCH_LEN`
```rust
const MAX_MATCH_LEN: usize = 258usize;
```

The maximum length of a match.

### `DEFAULT_FLAGS`
```rust
const DEFAULT_FLAGS: u32 = 4_112u32;
```

### `LITLEN_TABLE`
```rust
const LITLEN_TABLE: usize = 0usize;
```

Tables used for literal/lengths in `HuffmanOxide`.

### `DIST_TABLE`
```rust
const DIST_TABLE: usize = 1usize;
```

Tables for distances.

### `HUFF_CODES_TABLE`
```rust
const HUFF_CODES_TABLE: usize = 2usize;
```

Tables for the run-length encoded huffman lengths for literals/lengths/distances.

### `COMP_FAST_LOOKAHEAD_SIZE`
```rust
const COMP_FAST_LOOKAHEAD_SIZE: usize = 4_096usize;
```

