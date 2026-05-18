*[ruzstd](../../index.md) / [decoding](../index.md) / [sequence_section_decoder](index.md)*

---

# Module `sequence_section_decoder`

## Contents

- [Functions](#functions)
  - [`decode_sequences`](#decode-sequences)
  - [`decode_sequences_with_rle`](#decode-sequences-with-rle)
  - [`decode_sequences_without_rle`](#decode-sequences-without-rle)
  - [`lookup_ll_code`](#lookup-ll-code)
  - [`lookup_ml_code`](#lookup-ml-code)
  - [`maybe_update_fse_tables`](#maybe-update-fse-tables)
- [Constants](#constants)
  - [`LL_MAX_LOG`](#ll-max-log)
  - [`ML_MAX_LOG`](#ml-max-log)
  - [`OF_MAX_LOG`](#of-max-log)
  - [`LL_DEFAULT_ACC_LOG`](#ll-default-acc-log)
  - [`LITERALS_LENGTH_DEFAULT_DISTRIBUTION`](#literals-length-default-distribution)
  - [`ML_DEFAULT_ACC_LOG`](#ml-default-acc-log)
  - [`MATCH_LENGTH_DEFAULT_DISTRIBUTION`](#match-length-default-distribution)
  - [`OF_DEFAULT_ACC_LOG`](#of-default-acc-log)
  - [`OFFSET_DEFAULT_DISTRIBUTION`](#offset-default-distribution)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decode_sequences`](#decode-sequences) | fn | Decode the provided source as a series of sequences into the supplied `target`. |
| [`decode_sequences_with_rle`](#decode-sequences-with-rle) | fn |  |
| [`decode_sequences_without_rle`](#decode-sequences-without-rle) | fn |  |
| [`lookup_ll_code`](#lookup-ll-code) | fn | Look up the provided state value from a literal length table predefined by the Zstandard reference document. |
| [`lookup_ml_code`](#lookup-ml-code) | fn | Look up the provided state value from a match length table predefined by the Zstandard reference document. |
| [`maybe_update_fse_tables`](#maybe-update-fse-tables) | fn |  |
| [`LL_MAX_LOG`](#ll-max-log) | const | "The maximum allowed accuracy log for literals length and match length tables is 9" |
| [`ML_MAX_LOG`](#ml-max-log) | const | "The maximum allowed accuracy log for literals length and match length tables is 9" |
| [`OF_MAX_LOG`](#of-max-log) | const | "The maximum accuracy log for the offset table is 8." |
| [`LL_DEFAULT_ACC_LOG`](#ll-default-acc-log) | const |  |
| [`LITERALS_LENGTH_DEFAULT_DISTRIBUTION`](#literals-length-default-distribution) | const | If [ModeType::Predefined] is selected for a symbol type, its FSE decoding table is generated using a predefined distribution table. |
| [`ML_DEFAULT_ACC_LOG`](#ml-default-acc-log) | const |  |
| [`MATCH_LENGTH_DEFAULT_DISTRIBUTION`](#match-length-default-distribution) | const | If [ModeType::Predefined] is selected for a symbol type, its FSE decoding table is generated using a predefined distribution table. |
| [`OF_DEFAULT_ACC_LOG`](#of-default-acc-log) | const |  |
| [`OFFSET_DEFAULT_DISTRIBUTION`](#offset-default-distribution) | const | If [ModeType::Predefined] is selected for a symbol type, its FSE decoding table is generated using a predefined distribution table. |

## Functions

### `decode_sequences`

```rust
fn decode_sequences(section: &super::super::blocks::sequence_section::SequencesHeader, source: &[u8], scratch: &mut super::scratch::FSEScratch, target: &mut alloc::vec::Vec<super::super::blocks::sequence_section::Sequence>) -> Result<(), crate::decoding::errors::DecodeSequenceError>
```

Decode the provided source as a series of sequences into the supplied `target`.

### `decode_sequences_with_rle`

```rust
fn decode_sequences_with_rle(section: &super::super::blocks::sequence_section::SequencesHeader, br: &mut crate::bit_io::BitReaderReversed<'_>, scratch: &super::scratch::FSEScratch, target: &mut alloc::vec::Vec<super::super::blocks::sequence_section::Sequence>) -> Result<(), crate::decoding::errors::DecodeSequenceError>
```

### `decode_sequences_without_rle`

```rust
fn decode_sequences_without_rle(section: &super::super::blocks::sequence_section::SequencesHeader, br: &mut crate::bit_io::BitReaderReversed<'_>, scratch: &super::scratch::FSEScratch, target: &mut alloc::vec::Vec<super::super::blocks::sequence_section::Sequence>) -> Result<(), crate::decoding::errors::DecodeSequenceError>
```

### `lookup_ll_code`

```rust
fn lookup_ll_code(code: u8) -> (u32, u8)
```

Look up the provided state value from a literal length table predefined
by the Zstandard reference document. Returns a tuple of (value, number of bits).

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#appendix-a---decoding-tables-for-predefined-codes>

### `lookup_ml_code`

```rust
fn lookup_ml_code(code: u8) -> (u32, u8)
```

Look up the provided state value from a match length table predefined
by the Zstandard reference document. Returns a tuple of (value, number of bits).

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#appendix-a---decoding-tables-for-predefined-codes>

### `maybe_update_fse_tables`

```rust
fn maybe_update_fse_tables(section: &super::super::blocks::sequence_section::SequencesHeader, source: &[u8], scratch: &mut super::scratch::FSEScratch) -> Result<usize, crate::decoding::errors::DecodeSequenceError>
```

## Constants

### `LL_MAX_LOG`
```rust
const LL_MAX_LOG: u8 = 9u8;
```

"The maximum allowed accuracy log for literals length and match length tables is 9"

### `ML_MAX_LOG`
```rust
const ML_MAX_LOG: u8 = 9u8;
```

"The maximum allowed accuracy log for literals length and match length tables is 9"

### `OF_MAX_LOG`
```rust
const OF_MAX_LOG: u8 = 8u8;
```

"The maximum accuracy log for the offset table is 8."

### `LL_DEFAULT_ACC_LOG`
```rust
const LL_DEFAULT_ACC_LOG: u8 = 6u8;
```

### `LITERALS_LENGTH_DEFAULT_DISTRIBUTION`
```rust
const LITERALS_LENGTH_DEFAULT_DISTRIBUTION: [i32; 36];
```

If [ModeType::Predefined] is selected for a symbol type, its FSE decoding
table is generated using a predefined distribution table.

https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#literals-length

### `ML_DEFAULT_ACC_LOG`
```rust
const ML_DEFAULT_ACC_LOG: u8 = 6u8;
```

### `MATCH_LENGTH_DEFAULT_DISTRIBUTION`
```rust
const MATCH_LENGTH_DEFAULT_DISTRIBUTION: [i32; 53];
```

If [ModeType::Predefined] is selected for a symbol type, its FSE decoding
table is generated using a predefined distribution table.

https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#match-length

### `OF_DEFAULT_ACC_LOG`
```rust
const OF_DEFAULT_ACC_LOG: u8 = 5u8;
```

### `OFFSET_DEFAULT_DISTRIBUTION`
```rust
const OFFSET_DEFAULT_DISTRIBUTION: [i32; 29];
```

If [ModeType::Predefined] is selected for a symbol type, its FSE decoding
table is generated using a predefined distribution table.

https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#match-length

