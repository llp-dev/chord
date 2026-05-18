*[ruzstd](../../index.md) / [fse](../index.md) / [fse_decoder](index.md)*

---

# Module `fse_decoder`

## Contents

- [Structs](#structs)
  - [`FSEDecoder`](#fsedecoder)
  - [`FSETable`](#fsetable)
  - [`Entry`](#entry)
- [Functions](#functions)
  - [`highest_bit_set`](#highest-bit-set)
  - [`next_position`](#next-position)
  - [`calc_baseline_and_numbits`](#calc-baseline-and-numbits)
- [Constants](#constants)
  - [`ACC_LOG_OFFSET`](#acc-log-offset)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FSEDecoder`](#fsedecoder) | struct |  |
| [`FSETable`](#fsetable) | struct | FSE decoding involves a decoding table that describes the probabilities of all literals from 0 to the highest present one |
| [`Entry`](#entry) | struct | A single entry in an FSE table. |
| [`highest_bit_set`](#highest-bit-set) | fn |  |
| [`next_position`](#next-position) | fn | Calculate the position of the next entry of the table given the current position and size of the table. |
| [`calc_baseline_and_numbits`](#calc-baseline-and-numbits) | fn |  |
| [`ACC_LOG_OFFSET`](#acc-log-offset) | const | This value is added to the first 4 bits of the stream to determine the `Accuracy_Log` |

## Structs

### `FSEDecoder<'table>`

```rust
struct FSEDecoder<'table> {
    pub state: Entry,
    table: &'table FSETable,
}
```

#### Fields

- **`state`**: `Entry`

  An FSE state value represents an index in the FSE table.

- **`table`**: `&'table FSETable`

  A reference to the table used for decoding.

#### Implementations

- <span id="fsedecoder-new"></span>`fn new(table: &'t FSETable) -> FSEDecoder<'t>` — [`FSETable`](../index.md#fsetable), [`FSEDecoder`](../index.md#fsedecoder)

  Initialize a new Finite State Entropy decoder.

- <span id="fsedecoder-decode-symbol"></span>`fn decode_symbol(&self) -> u8`

  Returns the byte associated with the symbol the internal cursor is pointing at.

- <span id="fsedecoder-init-state"></span>`fn init_state(&mut self, bits: &mut BitReaderReversed<'_>) -> Result<(), FSEDecoderError>` — [`BitReaderReversed`](../../bit_io/bit_reader_reverse/index.md#bitreaderreversed), [`FSEDecoderError`](../../decoding/errors/index.md#fsedecodererror)

  Initialize internal state and prepare for decoding. After this, `decode_symbol` can be called

  to read the first symbol and `update_state` can be called to prepare to read the next symbol.

- <span id="fsedecoder-update-state"></span>`fn update_state(&mut self, bits: &mut BitReaderReversed<'_>)` — [`BitReaderReversed`](../../bit_io/bit_reader_reverse/index.md#bitreaderreversed)

  Advance the internal state to decode the next symbol in the bitstream.

### `FSETable`

```rust
struct FSETable {
    max_symbol: u8,
    pub decode: alloc::vec::Vec<Entry>,
    pub accuracy_log: u8,
    pub symbol_probabilities: alloc::vec::Vec<i32>,
    symbol_counter: alloc::vec::Vec<u32>,
}
```

FSE decoding involves a decoding table that describes the probabilities of
all literals from 0 to the highest present one

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#fse-table-description>

#### Fields

- **`max_symbol`**: `u8`

  The maximum symbol in the table (inclusive). Limits the probabilities length to max_symbol + 1.

- **`decode`**: `alloc::vec::Vec<Entry>`

  The actual table containing the decoded symbol and the compression data
  connected to that symbol.

- **`accuracy_log`**: `u8`

  The size of the table is stored in logarithm base 2 format,
  with the **size of the table** being equal to `(1 << accuracy_log)`.
  This value is used so that the decoder knows how many bits to read from the bitstream.

- **`symbol_probabilities`**: `alloc::vec::Vec<i32>`

  In this context, probability refers to the likelihood that a symbol occurs in the given data.
  Given this info, the encoder can assign shorter codes to symbols that appear more often,
  and longer codes that appear less often, then the decoder can use the probability
  to determine what code was assigned to what symbol.
  
  The probability of a single symbol is a value representing the proportion of times the symbol
  would fall within the data.
  
  If a symbol probability is set to `-1`, it means that the probability of a symbol
  occurring in the data is less than one.

- **`symbol_counter`**: `alloc::vec::Vec<u32>`

  The number of times each symbol occurs (The first entry being 0x0, the second being 0x1) and so on
  up until the highest possible symbol (255).

#### Implementations

- <span id="fsetable-new"></span>`fn new(max_symbol: u8) -> FSETable` — [`FSETable`](../index.md#fsetable)

  Initialize a new empty Finite State Entropy decoding table.

- <span id="fsetable-reinit-from"></span>`fn reinit_from(&mut self, other: &Self)`

  Reset `self` and update `self`'s state to mirror the provided table.

- <span id="fsetable-reset"></span>`fn reset(&mut self)`

  Empty the table and clear all internal state.

- <span id="fsetable-build-decoder"></span>`fn build_decoder(&mut self, source: &[u8], max_log: u8) -> Result<usize, FSETableError>` — [`FSETableError`](../../decoding/errors/index.md#fsetableerror)

  returns how many BYTEs (not bits) were read while building the decoder

- <span id="fsetable-build-from-probabilities"></span>`fn build_from_probabilities(&mut self, acc_log: u8, probs: &[i32]) -> Result<(), FSETableError>` — [`FSETableError`](../../decoding/errors/index.md#fsetableerror)

  Given the provided accuracy log, build a decoding table from that log.

- <span id="fsetable-build-decoding-table"></span>`fn build_decoding_table(&mut self) -> Result<(), FSETableError>` — [`FSETableError`](../../decoding/errors/index.md#fsetableerror)

  Build the actual decoding table after probabilities have been read into the table.

  After this function is called, the decoding process can begin.

- <span id="fsetable-read-probabilities"></span>`fn read_probabilities(&mut self, source: &[u8], max_log: u8) -> Result<usize, FSETableError>` — [`FSETableError`](../../decoding/errors/index.md#fsetableerror)

  Read the accuracy log and the probability table from the source and return the number of bytes

  read. If the size of the table is larger than the provided `max_log`, return an error.

#### Trait Implementations

##### `impl Clone for FSETable`

- <span id="fsetable-clone"></span>`fn clone(&self) -> FSETable` — [`FSETable`](../index.md#fsetable)

##### `impl Debug for FSETable`

- <span id="fsetable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Entry`

```rust
struct Entry {
    pub base_line: u32,
    pub num_bits: u8,
    pub symbol: u8,
}
```

A single entry in an FSE table.

#### Fields

- **`base_line`**: `u32`

  This value is used as an offset value, and it is added
  to a value read from the stream to determine the next state value.

- **`num_bits`**: `u8`

  How many bits should be read from the stream when decoding this entry.

- **`symbol`**: `u8`

  The byte that should be put in the decode output when encountering this state.

#### Trait Implementations

##### `impl Clone for Entry`

- <span id="entry-clone"></span>`fn clone(&self) -> Entry` — [`Entry`](../index.md#entry)

##### `impl Copy for Entry`

##### `impl Debug for Entry`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `highest_bit_set`

```rust
fn highest_bit_set(x: u32) -> u32
```

### `next_position`

```rust
fn next_position(p: usize, table_size: usize) -> usize
```

Calculate the position of the next entry of the table given the current
position and size of the table.

### `calc_baseline_and_numbits`

```rust
fn calc_baseline_and_numbits(num_states_total: u32, num_states_symbol: u32, state_number: u32) -> (u32, u8)
```

## Constants

### `ACC_LOG_OFFSET`
```rust
const ACC_LOG_OFFSET: u8 = 5u8;
```

This value is added to the first 4 bits of the stream to determine the
`Accuracy_Log`

