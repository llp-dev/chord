*[miniz_oxide](../../index.md) / [inflate](../index.md) / [core](index.md)*

---

# Module `core`

Core decompression functionality.

# Using decompress with a wrapping buffer

[`decompress`](#decompress) and [`decompress_with_limit`](#decompress-with-limit) can be used with a wrapping buffer.

To decompress with a wrapping buffer you must:
- not pass `TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF` flag
- pass an output buffer with a size of a power of 2
- pass an output buffer  with a size greater or equal to the decompression window
  (which cannot be more than 32KiB, so 32KiB is a safe size)
- pass the same buffer on each call without modification

You must process return values so that:
- next call pass the input buffer without the first input bytes read skipped
- next call pass the same output buffer
- next call pass an out_pos incremented by the number of bytes output (wrapping to 0 if needed)
- do a next call only if return status is `NeedsMoreInput` or `NeedsMoreInput`

[`decompress`](#decompress) will write to any byte after `out_pos` in the output buffer, but will not
wrap around. This means that all bytes after `out_pos` must be saved while the ones before
do not have to.

[`decompress_with_limit`](#decompress-with-limit) will write to any byte after `out_pos` but not more than `out_max`
and will not wrap around. This means that you can use the buffer as a ring buffer for your
application usage, as long as you keep track of the number of disposable bytes.

## Contents

- [Modules](#modules)
  - [`inflate_flags`](#inflate-flags)
- [Structs](#structs)
  - [`HuffmanTable`](#huffmantable)
  - [`DecompressorOxide`](#decompressoroxide)
  - [`LocalVars`](#localvars)
- [Enums](#enums)
  - [`State`](#state)
  - [`Action`](#action)
- [Functions](#functions)
  - [`num_extra_bits_for_distance_code`](#num-extra-bits-for-distance-code)
  - [`read_u16_le`](#read-u16-le)
  - [`fill_bit_buffer`](#fill-bit-buffer)
  - [`validate_zlib_header`](#validate-zlib-header)
  - [`decode_huffman_code`](#decode-huffman-code)
  - [`read_byte`](#read-byte)
  - [`read_bits`](#read-bits)
  - [`pad_to_bytes`](#pad-to-bytes)
  - [`end_of_input`](#end-of-input)
  - [`undo_bytes`](#undo-bytes)
  - [`start_static_table`](#start-static-table)
  - [`reverse_bits`](#reverse-bits)
  - [`init_tree`](#init-tree)
  - [`transfer`](#transfer)
  - [`apply_match`](#apply-match)
  - [`decompress_fast`](#decompress-fast)
  - [`decompress`](#decompress)
  - [`decompress_with_limit`](#decompress-with-limit)
- [Type Aliases](#type-aliases)
  - [`BitBuffer`](#bitbuffer)
- [Constants](#constants)
  - [`TINFL_LZ_DICT_SIZE`](#tinfl-lz-dict-size)
  - [`MAX_HUFF_TABLES`](#max-huff-tables)
  - [`MAX_HUFF_SYMBOLS_0`](#max-huff-symbols-0)
  - [`MAX_HUFF_SYMBOLS_1`](#max-huff-symbols-1)
  - [`MAX_HUFF_SYMBOLS_2`](#max-huff-symbols-2)
  - [`FAST_LOOKUP_BITS`](#fast-lookup-bits)
  - [`FAST_LOOKUP_SIZE`](#fast-lookup-size)
  - [`MAX_HUFF_TREE_SIZE`](#max-huff-tree-size)
  - [`LITLEN_TABLE`](#litlen-table)
  - [`DIST_TABLE`](#dist-table)
  - [`HUFFLEN_TABLE`](#hufflen-table)
  - [`LEN_CODES_SIZE`](#len-codes-size)
  - [`LEN_CODES_MASK`](#len-codes-mask)
  - [`MIN_TABLE_SIZES`](#min-table-sizes)
  - [`LENGTH_BASE`](#length-base)
  - [`LENGTH_EXTRA`](#length-extra)
  - [`DIST_BASE`](#dist-base)
  - [`BASE_EXTRA_MASK`](#base-extra-mask)
- [Macros](#macros)
  - [`generate_state!`](#generate-state)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inflate_flags`](#inflate-flags) | mod | Flags to [`decompress()`] to control how inflation works. |
| [`HuffmanTable`](#huffmantable) | struct | A struct containing huffman code lengths and the huffman code tree used by the decompressor. |
| [`DecompressorOxide`](#decompressoroxide) | struct | Main decompression struct. |
| [`LocalVars`](#localvars) | struct |  |
| [`State`](#state) | enum |  |
| [`Action`](#action) | enum |  |
| [`num_extra_bits_for_distance_code`](#num-extra-bits-for-distance-code) | fn | Get the number of extra bits used for a distance code. |
| [`read_u16_le`](#read-u16-le) | fn | Read an le u16 value from the slice iterator. |
| [`fill_bit_buffer`](#fill-bit-buffer) | fn | Ensure that there is data in the bit buffer. |
| [`validate_zlib_header`](#validate-zlib-header) | fn | Check that the zlib header is correct and that there is enough space in the buffer for the window size specified in the header. |
| [`decode_huffman_code`](#decode-huffman-code) | fn | Try to decode the next huffman code, and puts it in the counter field of the decompressor if successful. |
| [`read_byte`](#read-byte) | fn | Try to read one byte from `in_iter` and call `f` with the read byte as an argument, returning the result. |
| [`read_bits`](#read-bits) | fn | Try to read `amount` number of bits from `in_iter` and call the function `f` with the bits as an an argument after reading, returning the result of that function, or `Action::End` if there are not enough bytes left. |
| [`pad_to_bytes`](#pad-to-bytes) | fn |  |
| [`end_of_input`](#end-of-input) | fn |  |
| [`undo_bytes`](#undo-bytes) | fn |  |
| [`start_static_table`](#start-static-table) | fn |  |
| [`reverse_bits`](#reverse-bits) | fn |  |
| [`init_tree`](#init-tree) | fn |  |
| [`transfer`](#transfer) | fn |  |
| [`apply_match`](#apply-match) | fn | Presumes that there is at least match_len bytes in output left. |
| [`decompress_fast`](#decompress-fast) | fn | Fast inner decompression loop which is run  while there is at least 259 bytes left in the output buffer, and at least 6 bytes left in the input buffer (The maximum one match would need + 1). |
| [`decompress`](#decompress) | fn | Main decompression function. |
| [`decompress_with_limit`](#decompress-with-limit) | fn | Same as [`decompress()`] with a maximum decompressed byte count. |
| [`BitBuffer`](#bitbuffer) | type |  |
| [`TINFL_LZ_DICT_SIZE`](#tinfl-lz-dict-size) | const |  |
| [`MAX_HUFF_TABLES`](#max-huff-tables) | const | The number of huffman tables used. |
| [`MAX_HUFF_SYMBOLS_0`](#max-huff-symbols-0) | const | The length of the first (literal/length) huffman table. |
| [`MAX_HUFF_SYMBOLS_1`](#max-huff-symbols-1) | const | The length of the second (distance) huffman table. |
| [`MAX_HUFF_SYMBOLS_2`](#max-huff-symbols-2) | const | The length of the last (huffman code length) huffman table. |
| [`FAST_LOOKUP_BITS`](#fast-lookup-bits) | const | The maximum length of a code that can be looked up in the fast lookup table. |
| [`FAST_LOOKUP_SIZE`](#fast-lookup-size) | const | The size of the fast lookup table. |
| [`MAX_HUFF_TREE_SIZE`](#max-huff-tree-size) | const |  |
| [`LITLEN_TABLE`](#litlen-table) | const |  |
| [`DIST_TABLE`](#dist-table) | const |  |
| [`HUFFLEN_TABLE`](#hufflen-table) | const |  |
| [`LEN_CODES_SIZE`](#len-codes-size) | const |  |
| [`LEN_CODES_MASK`](#len-codes-mask) | const |  |
| [`MIN_TABLE_SIZES`](#min-table-sizes) | const |  |
| [`LENGTH_BASE`](#length-base) | const | Base length for each length code. |
| [`LENGTH_EXTRA`](#length-extra) | const | Number of extra bits for each length code. |
| [`DIST_BASE`](#dist-base) | const | Base length for each distance code. |
| [`BASE_EXTRA_MASK`](#base-extra-mask) | const | The mask used when indexing the base/extra arrays. |
| [`generate_state!`](#generate-state) | macro |  |

## Modules

- [`inflate_flags`](inflate_flags/index.md) — Flags to [`decompress()`] to control how inflation works.

## Structs

### `HuffmanTable`

```rust
struct HuffmanTable {
    pub look_up: [i16; 1024],
    pub tree: [i16; 576],
}
```

A struct containing huffman code lengths and the huffman code tree used by the decompressor.

#### Fields

- **`look_up`**: `[i16; 1024]`

  Fast lookup table for shorter huffman codes.
  
  See `HuffmanTable::fast_lookup`.

- **`tree`**: `[i16; 576]`

  Full huffman tree.
  
  Positive values are edge nodes/symbols, negative values are
  parent nodes/references to other nodes.

#### Implementations

- <span id="huffmantable-new"></span>`const fn new() -> HuffmanTable` — [`HuffmanTable`](#huffmantable)

- <span id="huffmantable-fast-lookup"></span>`const fn fast_lookup(&self, bit_buf: u64) -> i16`

  Look for a symbol in the fast lookup table.

  The symbol is stored in the lower 9 bits, the length in the next 6.

  If the returned value is negative, the code wasn't found in the

  fast lookup table and the full tree has to be traversed to find the code.

- <span id="huffmantable-tree-lookup"></span>`fn tree_lookup(&self, fast_symbol: i32, bit_buf: u64, code_len: u8) -> (i32, u32)`

  Get the symbol and the code length from the huffman tree.

- <span id="huffmantable-lookup"></span>`fn lookup(&self, bit_buf: u64) -> (i32, u32)`

  Look up a symbol and code length from the bits in the provided bit buffer.

  

  Returns Some(symbol, length) on success,

  None if the length is 0.

  

  It's possible we could avoid checking for 0 if we can guarantee a sane table.

  TODO: Check if a smaller type for code_len helps performance.

#### Trait Implementations

##### `impl Clone for HuffmanTable`

- <span id="huffmantable-clone"></span>`fn clone(&self) -> HuffmanTable` — [`HuffmanTable`](#huffmantable)

### `DecompressorOxide`

```rust
struct DecompressorOxide {
    state: core::State,
    num_bits: u32,
    z_header0: u32,
    z_header1: u32,
    z_adler32: u32,
    finish: u8,
    block_type: u8,
    check_adler32: u32,
    dist: u32,
    counter: u32,
    num_extra: u8,
    table_sizes: [u16; 3],
    bit_buf: u64,
    tables: [HuffmanTable; 3],
    code_size_literal: [u8; 288],
    code_size_dist: [u8; 32],
    code_size_huffman: [u8; 19],
    raw_header: [u8; 4],
    len_codes: [u8; 512],
}
```

Main decompression struct.


#### Fields

- **`state`**: `core::State`

  Current state of the decompressor.

- **`num_bits`**: `u32`

  Number of bits in the bit buffer.

- **`z_header0`**: `u32`

  Zlib CMF

- **`z_header1`**: `u32`

  Zlib FLG

- **`z_adler32`**: `u32`

  Adler32 checksum from the zlib header.

- **`finish`**: `u8`

  1 if the current block is the last block, 0 otherwise.

- **`block_type`**: `u8`

  The type of the current block.
  or if in a dynamic block, which huffman table we are currently

- **`check_adler32`**: `u32`

  1 if the adler32 value should be checked.

- **`dist`**: `u32`

  Last match distance.

- **`counter`**: `u32`

  Variable used for match length, symbols, and a number of other things.

- **`num_extra`**: `u8`

  Number of extra bits for the last length or distance code.

- **`table_sizes`**: `[u16; 3]`

  Number of entries in each huffman table.

- **`bit_buf`**: `u64`

  Buffer of input data.

- **`tables`**: `[HuffmanTable; 3]`

  Huffman tables.

- **`raw_header`**: `[u8; 4]`

  Raw block header.

- **`len_codes`**: `[u8; 512]`

  Huffman length codes.

#### Implementations

- <span id="decompressoroxide-new"></span>`fn new() -> DecompressorOxide` — [`DecompressorOxide`](#decompressoroxide)

  Create a new tinfl_decompressor with all fields set to 0.

- <span id="decompressoroxide-init"></span>`fn init(&mut self)`

  Set the current state to `Start`.

- <span id="decompressoroxide-adler32"></span>`fn adler32(&self) -> Option<u32>`

  Returns the adler32 checksum of the currently decompressed data.

  Note: Will return Some(1) if decompressing zlib but ignoring adler32.

- <span id="decompressoroxide-adler32-header"></span>`fn adler32_header(&self) -> Option<u32>`

  Returns the adler32 that was read from the zlib header if it exists.

#### Trait Implementations

##### `impl Clone for DecompressorOxide`

- <span id="decompressoroxide-clone"></span>`fn clone(&self) -> DecompressorOxide` — [`DecompressorOxide`](#decompressoroxide)

##### `impl Default for DecompressorOxide`

- <span id="decompressoroxide-default"></span>`fn default() -> Self`

  Create a new tinfl_decompressor with all fields set to 0.

### `LocalVars`

```rust
struct LocalVars {
    pub bit_buf: u64,
    pub num_bits: u32,
    pub dist: u32,
    pub counter: u32,
    pub num_extra: u8,
}
```

#### Trait Implementations

##### `impl Clone for LocalVars`

- <span id="localvars-clone"></span>`fn clone(&self) -> LocalVars` — [`LocalVars`](#localvars)

##### `impl Copy for LocalVars`

## Enums

### `State`

```rust
enum State {
    Start,
    ReadZlibCmf,
    ReadZlibFlg,
    ReadBlockHeader,
    BlockTypeNoCompression,
    RawHeader,
    RawMemcpy1,
    RawMemcpy2,
    ReadTableSizes,
    ReadHufflenTableCodeSize,
    ReadLitlenDistTablesCodeSize,
    ReadExtraBitsCodeSize,
    DecodeLitlen,
    WriteSymbol,
    ReadExtraBitsLitlen,
    DecodeDistance,
    ReadExtraBitsDistance,
    RawReadFirstByte,
    RawStoreFirstByte,
    WriteLenBytesToEnd,
    BlockDone,
    HuffDecodeOuterLoop1,
    HuffDecodeOuterLoop2,
    ReadAdler32,
    DoneForever,
    BlockTypeUnexpected,
    BadCodeSizeSum,
    BadDistOrLiteralTableLength,
    BadTotalSymbols,
    BadZlibHeader,
    DistanceOutOfBounds,
    BadRawLength,
    BadCodeSizeDistPrevLookup,
    InvalidLitlen,
    InvalidDist,
}
```

#### Implementations

- <span id="state-is-failure"></span>`const fn is_failure(self) -> bool`

- <span id="state-begin"></span>`fn begin(&mut self, new_state: State)` — [`State`](#state)

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for State`

##### `impl PartialEq for State`

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

### `Action`

```rust
enum Action {
    None,
    Jump(State),
    End(TINFLStatus),
}
```

## Functions

### `num_extra_bits_for_distance_code`

```rust
const fn num_extra_bits_for_distance_code(code: u8) -> u8
```

Get the number of extra bits used for a distance code.
(Code numbers above `NUM_DISTANCE_CODES` will give some garbage
value.)

### `read_u16_le`

```rust
fn read_u16_le(iter: &mut self::output_buffer::InputWrapper<'_>) -> u16
```

Read an le u16 value from the slice iterator.

# Panics
Panics if there are less than two bytes left.

### `fill_bit_buffer`

```rust
fn fill_bit_buffer(l: &mut LocalVars, in_iter: &mut self::output_buffer::InputWrapper<'_>)
```

Ensure that there is data in the bit buffer.

On 64-bit platform, we use a 64-bit value so this will
result in there being at least 32 bits in the bit buffer.
This function assumes that there is at least 4 bytes left in the input buffer.

### `validate_zlib_header`

```rust
const fn validate_zlib_header(cmf: u32, flg: u32, flags: u32, mask: usize) -> Action
```

Check that the zlib header is correct and that there is enough space in the buffer
for the window size specified in the header.

See https://tools.ietf.org/html/rfc1950

### `decode_huffman_code`

```rust
fn decode_huffman_code<F>(r: &mut DecompressorOxide, l: &mut LocalVars, table: usize, flags: u32, in_iter: &mut self::output_buffer::InputWrapper<'_>, f: F) -> Action
where
    F: FnOnce(&mut DecompressorOxide, &mut LocalVars, i32) -> Action
```

Try to decode the next huffman code, and puts it in the counter field of the decompressor
if successful.

# Returns
The specified action returned from `f` on success,
`Action::End` if there are not enough data left to decode a symbol.

### `read_byte`

```rust
fn read_byte<F>(in_iter: &mut self::output_buffer::InputWrapper<'_>, flags: u32, f: F) -> Action
where
    F: FnOnce(u8) -> Action
```

Try to read one byte from `in_iter` and call `f` with the read byte as an argument,
returning the result.
If reading fails, `Action::End is returned`

### `read_bits`

```rust
fn read_bits<F>(l: &mut LocalVars, amount: u32, in_iter: &mut self::output_buffer::InputWrapper<'_>, flags: u32, f: F) -> Action
where
    F: FnOnce(&mut LocalVars, u64) -> Action
```

Try to read `amount` number of bits from `in_iter` and call the function `f` with the bits as an
an argument after reading, returning the result of that function, or `Action::End` if there are
not enough bytes left.

### `pad_to_bytes`

```rust
fn pad_to_bytes<F>(l: &mut LocalVars, in_iter: &mut self::output_buffer::InputWrapper<'_>, flags: u32, f: F) -> Action
where
    F: FnOnce(&mut LocalVars) -> Action
```

### `end_of_input`

```rust
const fn end_of_input(flags: u32) -> Action
```

### `undo_bytes`

```rust
fn undo_bytes(l: &mut LocalVars, max: u32) -> u32
```

### `start_static_table`

```rust
fn start_static_table(r: &mut DecompressorOxide)
```

### `reverse_bits`

```rust
fn reverse_bits(n: u16) -> u16
```

### `init_tree`

```rust
fn init_tree(r: &mut DecompressorOxide, l: &mut LocalVars) -> Option<Action>
```

### `transfer`

```rust
fn transfer(out_slice: &mut [u8], source_pos: usize, out_pos: usize, match_len: usize, out_buf_size_mask: usize)
```

### `apply_match`

```rust
fn apply_match(out_slice: &mut [u8], out_pos: usize, dist: usize, match_len: usize, out_buf_size_mask: usize)
```

Presumes that there is at least match_len bytes in output left.

### `decompress_fast`

```rust
fn decompress_fast(r: &mut DecompressorOxide, in_iter: &mut self::output_buffer::InputWrapper<'_>, out_buf: &mut self::output_buffer::OutputBuffer<'_>, flags: u32, local_vars: &mut LocalVars, out_buf_size_mask: usize) -> (TINFLStatus, State)
```

Fast inner decompression loop which is run  while there is at least
259 bytes left in the output buffer, and at least 6 bytes left in the input buffer
(The maximum one match would need + 1).

This was inspired by a similar optimization in zlib, which uses this info to do
faster unchecked copies of multiple bytes at a time.
Currently we don't do this here, but this function does avoid having to jump through the
big match loop on each state change(as rust does not have fallthrough or gotos at the moment),
and already improves decompression speed a fair bit.

### `decompress`

```rust
fn decompress(r: &mut DecompressorOxide, in_buf: &[u8], out: &mut [u8], out_pos: usize, flags: u32) -> (TINFLStatus, usize, usize)
```

Main decompression function. Keeps decompressing data from `in_buf` until the `in_buf` is
empty, `out` is full, the end of the deflate stream is hit, or there is an error in the
deflate stream.

# Arguments

`r` is a [`DecompressorOxide`](#decompressoroxide) struct with the state of this stream.

`in_buf` is a reference to the compressed data that is to be decompressed. The decompressor will
start at the first byte of this buffer.

`out` is a reference to the buffer that will store the decompressed data, and that
stores previously decompressed data if any.

* The offset given by `out_pos` indicates where in the output buffer slice writing should start.
* If [`TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`](inflate_flags/index.md) is not set, the output buffer is used in a
  wrapping manner, and it's size is required to be a power of 2.
* The decompression function normally needs access to 32KiB of the previously decompressed data
  (or to the beginning of the decompressed data if less than 32KiB has been decompressed.)
    - If this data is not available, decompression may fail.
    - Some deflate compressors allow specifying a window size which limits match distances to
      less than this, or alternatively an RLE mode where matches will only refer to the previous byte
      and thus allows a smaller output buffer. The window size can be specified in the zlib
      header structure, however, the header data should not be relied on to be correct.

`flags` indicates settings and status to the decompression function.
* The [`TINFL_FLAG_HAS_MORE_INPUT`](inflate_flags/index.md) has to be specified if more compressed data is to be provided
  in a subsequent call to this function.
* See the the [`inflate_flags`](inflate_flags/index.md) module for details on other flags.

# Returns

Returns a tuple containing the status of the compressor, the number of input bytes read, and the
number of bytes output to `out`.

### `decompress_with_limit`

```rust
fn decompress_with_limit(r: &mut DecompressorOxide, in_buf: &[u8], out: &mut [u8], out_pos: usize, out_max: usize, flags: u32) -> (TINFLStatus, usize, usize)
```

Same as [`decompress()`](#decompress) with a maximum decompressed byte count.

By default [`decompress()`](#decompress) decompress untill end of `out` buffer if possible.
`decompress_with_limit` will stop when `out_max` bytes have been decompressed,
or when `out` buffer is full, whichever comes first.

This is especially useful when using a wrapping output buffer. This helps keeping
some data that has not yet been consumed in the buffer while decompressing new bytes.

`out_max` is the maximum number of *bytes* that decompress will write

## Type Aliases

### `BitBuffer`

```rust
type BitBuffer = u64;
```

## Constants

### `TINFL_LZ_DICT_SIZE`
```rust
const TINFL_LZ_DICT_SIZE: usize = 32_768usize;
```

### `MAX_HUFF_TABLES`
```rust
const MAX_HUFF_TABLES: usize = 3usize;
```

The number of huffman tables used.

### `MAX_HUFF_SYMBOLS_0`
```rust
const MAX_HUFF_SYMBOLS_0: usize = 288usize;
```

The length of the first (literal/length) huffman table.

### `MAX_HUFF_SYMBOLS_1`
```rust
const MAX_HUFF_SYMBOLS_1: usize = 32usize;
```

The length of the second (distance) huffman table.

### `MAX_HUFF_SYMBOLS_2`
```rust
const MAX_HUFF_SYMBOLS_2: usize = 19usize;
```

The length of the last (huffman code length) huffman table.

### `FAST_LOOKUP_BITS`
```rust
const FAST_LOOKUP_BITS: u8 = 10u8;
```

The maximum length of a code that can be looked up in the fast lookup table.

### `FAST_LOOKUP_SIZE`
```rust
const FAST_LOOKUP_SIZE: u16 = 1_024u16;
```

The size of the fast lookup table.

### `MAX_HUFF_TREE_SIZE`
```rust
const MAX_HUFF_TREE_SIZE: usize = 576usize;
```

### `LITLEN_TABLE`
```rust
const LITLEN_TABLE: usize = 0usize;
```

### `DIST_TABLE`
```rust
const DIST_TABLE: usize = 1usize;
```

### `HUFFLEN_TABLE`
```rust
const HUFFLEN_TABLE: usize = 2usize;
```

### `LEN_CODES_SIZE`
```rust
const LEN_CODES_SIZE: usize = 512usize;
```

### `LEN_CODES_MASK`
```rust
const LEN_CODES_MASK: usize = 511usize;
```

### `MIN_TABLE_SIZES`
```rust
const MIN_TABLE_SIZES: [u16; 3];
```

### `LENGTH_BASE`
```rust
const LENGTH_BASE: [u16; 32];
```

Base length for each length code.

The base is used together with the value of the extra bits to decode the actual
length/distance values in a match.

### `LENGTH_EXTRA`
```rust
const LENGTH_EXTRA: [u8; 32];
```

Number of extra bits for each length code.

### `DIST_BASE`
```rust
const DIST_BASE: [u16; 30];
```

Base length for each distance code.

### `BASE_EXTRA_MASK`
```rust
const BASE_EXTRA_MASK: usize = 31usize;
```

The mask used when indexing the base/extra arrays.

## Macros

### `generate_state!`

