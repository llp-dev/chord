*[ruzstd](../../index.md) / [blocks](../index.md) / [sequence_section](index.md)*

---

# Module `sequence_section`

Utilities and representations for the second half of a block, the sequence section.
This section copies literals from the literals section into the decompressed output.

## Contents

- [Structs](#structs)
  - [`SequencesHeader`](#sequencesheader)
  - [`Sequence`](#sequence)
  - [`CompressionModes`](#compressionmodes)
- [Enums](#enums)
  - [`ModeType`](#modetype)
- [Constants](#constants)
  - [`MAX_LITERAL_LENGTH_CODE`](#max-literal-length-code)
  - [`MAX_MATCH_LENGTH_CODE`](#max-match-length-code)
  - [`MAX_OFFSET_CODE`](#max-offset-code)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SequencesHeader`](#sequencesheader) | struct |  |
| [`Sequence`](#sequence) | struct | A sequence represents potentially redundant data, and it can be broken up into 2 steps: - A copy step, where data is copied from the literals section to the decompressed output - A *match* copy step that copies data from within the previously decompressed output. |
| [`CompressionModes`](#compressionmodes) | struct | This byte defines the compression mode of each symbol type |
| [`ModeType`](#modetype) | enum | The compression mode used for symbol compression |
| [`MAX_LITERAL_LENGTH_CODE`](#max-literal-length-code) | const |  |
| [`MAX_MATCH_LENGTH_CODE`](#max-match-length-code) | const |  |
| [`MAX_OFFSET_CODE`](#max-offset-code) | const |  |

## Structs

### `SequencesHeader`

```rust
struct SequencesHeader {
    pub num_sequences: u32,
    pub modes: Option<CompressionModes>,
}
```

#### Implementations

- <span id="sequencesheader-new"></span>`fn new() -> SequencesHeader` — [`SequencesHeader`](#sequencesheader)

  Create a new [SequencesHeader].

- <span id="sequencesheader-parse-from-header"></span>`fn parse_from_header(&mut self, source: &[u8]) -> Result<u8, SequencesHeaderParseError>` — [`SequencesHeaderParseError`](../../decoding/errors/index.md#sequencesheaderparseerror)

  Attempt to deserialize the provided buffer into `self`, returning the number of bytes read.

#### Trait Implementations

##### `impl Default for SequencesHeader`

- <span id="sequencesheader-default"></span>`fn default() -> Self`

### `Sequence`

```rust
struct Sequence {
    pub ll: u32,
    pub ml: u32,
    pub of: u32,
}
```

A sequence represents potentially redundant data, and it can be broken up into 2 steps:
- A copy step, where data is copied from the literals section to the decompressed output
- A *match* copy step that copies data from within the previously decompressed output.

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#sequence-execution>

#### Fields

- **`ll`**: `u32`

  Literal length, or the number of bytes to be copied from the literals section
  in the copy step.

- **`ml`**: `u32`

  The length of the match to make during the match copy step.

- **`of`**: `u32`

  How far back to go in the decompressed data to read from the match copy step.
  If this value is greater than 3, then the offset is `of -3`. If `of` is from 1-3,
  then it has special handling:
  
  The first 3 values define 3 different repeated offsets, with 1 referring to the most
  recent, 2 the second recent, and so on. When the current sequence has a literal length of 0,
  then the repeated offsets are shifted by 1. So an offset value of 1 refers to 2, 2 refers to 3,
  and 3 refers to the most recent offset minus one. If that value is equal to zero, the data
  is considered corrupted.

#### Trait Implementations

##### `impl Clone for Sequence`

- <span id="sequence-clone"></span>`fn clone(&self) -> Sequence` — [`Sequence`](#sequence)

##### `impl Copy for Sequence`

##### `impl Display for Sequence`

- <span id="sequence-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

##### `impl ToString for Sequence`

- <span id="sequence-tostring-to-string"></span>`fn to_string(&self) -> String`

### `CompressionModes`

```rust
struct CompressionModes(u8);
```

This byte defines the compression mode of each symbol type

#### Implementations

- <span id="compressionmodes-decode-mode"></span>`fn decode_mode(m: u8) -> ModeType` — [`ModeType`](#modetype)

  Deserialize a two bit mode value into a [ModeType]

- <span id="compressionmodes-ll-mode"></span>`fn ll_mode(self) -> ModeType` — [`ModeType`](#modetype)

  Read the compression mode of the literal lengths field.

- <span id="compressionmodes-of-mode"></span>`fn of_mode(self) -> ModeType` — [`ModeType`](#modetype)

  Read the compression mode of the offset value field.

- <span id="compressionmodes-ml-mode"></span>`fn ml_mode(self) -> ModeType` — [`ModeType`](#modetype)

  Read the compression mode of the match lengths field.

#### Trait Implementations

##### `impl Clone for CompressionModes`

- <span id="compressionmodes-clone"></span>`fn clone(&self) -> CompressionModes` — [`CompressionModes`](#compressionmodes)

##### `impl Copy for CompressionModes`

## Enums

### `ModeType`

```rust
enum ModeType {
    Predefined,
    RLE,
    FSECompressed,
    Repeat,
}
```

The compression mode used for symbol compression

#### Variants

- **`Predefined`**

  A predefined FSE distribution table is used, and no distribution table
  will be present.

- **`RLE`**

  The table consists of a single byte, which contains the symbol's value.

- **`FSECompressed`**

  Standard FSE compression, a distribution table will be present. This
  mode should not be used when only one symbol is present.

- **`Repeat`**

  The table used in the previous compressed block with at least one sequence
  will be used again. If this is the first block, the table in the dictionary will
  be used.

## Constants

### `MAX_LITERAL_LENGTH_CODE`
```rust
const MAX_LITERAL_LENGTH_CODE: u8 = 35u8;
```

### `MAX_MATCH_LENGTH_CODE`
```rust
const MAX_MATCH_LENGTH_CODE: u8 = 52u8;
```

### `MAX_OFFSET_CODE`
```rust
const MAX_OFFSET_CODE: u8 = 31u8;
```

