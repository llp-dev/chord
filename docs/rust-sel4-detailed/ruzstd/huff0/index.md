*[ruzstd](../index.md) / [huff0](index.md)*

---

# Module `huff0`

## Contents

- [Modules](#modules)
  - [`huff0_decoder`](#huff0-decoder)
  - [`huff0_encoder`](#huff0-encoder)
- [Structs](#structs)
  - [`HuffmanDecoder`](#huffmandecoder)
  - [`HuffmanTable`](#huffmantable)
  - [`Entry`](#entry)
- [Functions](#functions)
  - [`highest_bit_set`](#highest-bit-set)
- [Constants](#constants)
  - [`MAX_MAX_NUM_BITS`](#max-max-num-bits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`huff0_decoder`](#huff0-decoder) | mod | Huffman coding is a method of encoding where symbols are assigned a code, and more commonly used symbols get shorter codes, and less commonly used symbols get longer codes. |
| [`huff0_encoder`](#huff0-encoder) | mod |  |
| [`HuffmanDecoder`](#huffmandecoder) | struct |  |
| [`HuffmanTable`](#huffmantable) | struct | A Huffman decoding table contains a list of Huffman prefix codes and their associated values |
| [`Entry`](#entry) | struct | A single entry in the table contains the decoded symbol/literal and the size of the prefix code. |
| [`highest_bit_set`](#highest-bit-set) | fn | Assert that the provided value is greater than zero, and returns the 32 - the number of leading zeros |
| [`MAX_MAX_NUM_BITS`](#max-max-num-bits) | const | The Zstandard specification limits the maximum length of a code to 11 bits. |

## Modules

- [`huff0_decoder`](huff0_decoder/index.md) — Huffman coding is a method of encoding where symbols are assigned a code,
- [`huff0_encoder`](huff0_encoder/index.md)

## Structs

### `HuffmanDecoder<'table>`

```rust
struct HuffmanDecoder<'table> {
    table: &'table HuffmanTable,
    pub state: u64,
}
```

#### Fields

- **`state`**: `u64`

  State is used to index into the table.

#### Implementations

- <span id="huffmandecoder-new"></span>`fn new(table: &'t HuffmanTable) -> HuffmanDecoder<'t>` — [`HuffmanTable`](#huffmantable), [`HuffmanDecoder`](#huffmandecoder)

  Create a new decoder with the provided table

- <span id="huffmandecoder-decode-symbol"></span>`fn decode_symbol(&mut self) -> u8`

  Decode the symbol the internal state (cursor) is pointed at and return the

  decoded literal.

- <span id="huffmandecoder-init-state"></span>`fn init_state(&mut self, br: &mut BitReaderReversed<'_>) -> u8` — [`BitReaderReversed`](../bit_io/bit_reader_reverse/index.md#bitreaderreversed)

  Initialize internal state and prepare to decode data. Then, `decode_symbol` can be called

  to read the byte the internal cursor is pointing at, and `next_state` can be called to advance

  the cursor until the max number of bits has been read.

- <span id="huffmandecoder-next-state"></span>`fn next_state(&mut self, br: &mut BitReaderReversed<'_>) -> u8` — [`BitReaderReversed`](../bit_io/bit_reader_reverse/index.md#bitreaderreversed)

  Advance the internal cursor to the next symbol. After this, you can call `decode_symbol`

  to read from the new position.

### `HuffmanTable`

```rust
struct HuffmanTable {
    decode: alloc::vec::Vec<Entry>,
    weights: alloc::vec::Vec<u8>,
    pub max_num_bits: u8,
    bits: alloc::vec::Vec<u8>,
    bit_ranks: alloc::vec::Vec<u32>,
    rank_indexes: alloc::vec::Vec<usize>,
    fse_table: crate::fse::FSETable,
}
```

A Huffman decoding table contains a list of Huffman prefix codes and their associated values

#### Fields

- **`weights`**: `alloc::vec::Vec<u8>`

  The weight of a symbol is the number of occurences in a table.
  This value is used in constructing a binary tree referred to as
  a Huffman tree. Once this tree is constructed, it can be used to build the
  lookup table

- **`max_num_bits`**: `u8`

  The maximum size in bits a prefix code in the encoded data can be.
  This value is used so that the decoder knows how many bits
  to read from the bitstream before checking the table. This
  value must be 11 or lower.

- **`fse_table`**: `crate::fse::FSETable`

  In some cases, the list of weights is compressed using FSE compression.

#### Implementations

- <span id="huffmantable-new"></span>`fn new() -> HuffmanTable` — [`HuffmanTable`](#huffmantable)

  Create a new, empty table.

- <span id="huffmantable-reinit-from"></span>`fn reinit_from(&mut self, other: &Self)`

  Completely empty the table then repopulate as a replica

  of `other`.

- <span id="huffmantable-reset"></span>`fn reset(&mut self)`

  Completely empty the table of all data.

- <span id="huffmantable-build-decoder"></span>`fn build_decoder(&mut self, source: &[u8]) -> Result<u32, HuffmanTableError>` — [`HuffmanTableError`](../decoding/errors/index.md#huffmantableerror)

  Read from `source` and decode the input, populating the huffman decoding table.

  

  Returns the number of bytes read.

- <span id="huffmantable-read-weights"></span>`fn read_weights(&mut self, source: &[u8]) -> Result<u32, HuffmanTableError>` — [`HuffmanTableError`](../decoding/errors/index.md#huffmantableerror)

  Read weights from the provided source.

  

  The huffman table is represented in the input data as a list of weights.

  After the header, weights are read, then a Huffman decoding table

  can be constructed using that list of weights.

  

  Returns the number of bytes read.

- <span id="huffmantable-build-table-from-weights"></span>`fn build_table_from_weights(&mut self) -> Result<(), HuffmanTableError>` — [`HuffmanTableError`](../decoding/errors/index.md#huffmantableerror)

  Once the weights have been read from the data, you can decode the weights

  into a table, and use that table to decode the actual compressed data.

  

  This function populates the rest of the table from the series of weights.

#### Trait Implementations

##### `impl Default for HuffmanTable`

- <span id="huffmantable-default"></span>`fn default() -> Self`

### `Entry`

```rust
struct Entry {
    symbol: u8,
    num_bits: u8,
}
```

A single entry in the table contains the decoded symbol/literal and the
size of the prefix code.

#### Fields

- **`symbol`**: `u8`

  The byte that the prefix code replaces during encoding.

- **`num_bits`**: `u8`

  The number of bits the prefix code occupies.

#### Trait Implementations

##### `impl Clone for Entry`

- <span id="entry-clone"></span>`fn clone(&self) -> Entry` — [`Entry`](#entry)

##### `impl Copy for Entry`

##### `impl Debug for Entry`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `highest_bit_set`

```rust
fn highest_bit_set(x: u32) -> u32
```

Assert that the provided value is greater than zero, and returns the
32 - the number of leading zeros

## Constants

### `MAX_MAX_NUM_BITS`
```rust
const MAX_MAX_NUM_BITS: u8 = 11u8;
```

The Zstandard specification limits the maximum length of a code to 11 bits.

