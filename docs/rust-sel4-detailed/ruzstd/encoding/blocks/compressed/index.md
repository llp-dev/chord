*[ruzstd](../../../index.md) / [encoding](../../index.md) / [blocks](../index.md) / [compressed](index.md)*

---

# Module `compressed`

## Contents

- [Enums](#enums)
  - [`FseTableMode`](#fsetablemode)
- [Functions](#functions)
  - [`compress_block`](#compress-block)
  - [`choose_table`](#choose-table)
  - [`encode_table`](#encode-table)
  - [`encode_fse_table_modes`](#encode-fse-table-modes)
  - [`encode_sequences`](#encode-sequences)
  - [`encode_seqnum`](#encode-seqnum)
  - [`encode_literal_length`](#encode-literal-length)
  - [`encode_match_len`](#encode-match-len)
  - [`encode_offset`](#encode-offset)
  - [`raw_literals`](#raw-literals)
  - [`compress_literals`](#compress-literals)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FseTableMode`](#fsetablemode) | enum |  |
| [`compress_block`](#compress-block) | fn | A block of [`crate::common::BlockType::Compressed`] |
| [`choose_table`](#choose-table) | fn |  |
| [`encode_table`](#encode-table) | fn |  |
| [`encode_fse_table_modes`](#encode-fse-table-modes) | fn |  |
| [`encode_sequences`](#encode-sequences) | fn |  |
| [`encode_seqnum`](#encode-seqnum) | fn |  |
| [`encode_literal_length`](#encode-literal-length) | fn |  |
| [`encode_match_len`](#encode-match-len) | fn |  |
| [`encode_offset`](#encode-offset) | fn |  |
| [`raw_literals`](#raw-literals) | fn |  |
| [`compress_literals`](#compress-literals) | fn |  |

## Enums

### `FseTableMode<'a>`

```rust
enum FseTableMode<'a> {
    Predefined(&'a crate::fse::fse_encoder::FSETable),
    Encoded(crate::fse::fse_encoder::FSETable),
    RepeateLast(&'a crate::fse::fse_encoder::FSETable),
}
```

#### Implementations

- <span id="fsetablemode-as-ref"></span>`fn as_ref(&self) -> &FSETable` — [`FSETable`](../../../fse/fse_encoder/index.md#fsetable)

#### Trait Implementations

##### `impl Clone for FseTableMode<'a>`

- <span id="fsetablemode-clone"></span>`fn clone(&self) -> FseTableMode<'a>` — [`FseTableMode`](#fsetablemode)

## Functions

### `compress_block`

```rust
fn compress_block<M: Matcher>(state: &mut crate::encoding::frame_compressor::CompressState<M>, output: &mut alloc::vec::Vec<u8>)
```

A block of [`crate::common::BlockType::Compressed`](../../../blocks/block/index.md#compressed)

### `choose_table`

```rust
fn choose_table<'a>(previous: Option<&'a crate::fse::fse_encoder::FSETable>, default_table: &'a crate::fse::fse_encoder::FSETable, data: impl Iterator<Item = u8>, max_log: u8) -> FseTableMode<'a>
```

### `encode_table`

```rust
fn encode_table(mode: &FseTableMode<'_>, writer: &mut crate::bit_io::BitWriter<&mut alloc::vec::Vec<u8>>)
```

### `encode_fse_table_modes`

```rust
fn encode_fse_table_modes(ll_mode: &FseTableMode<'_>, ml_mode: &FseTableMode<'_>, of_mode: &FseTableMode<'_>) -> u8
```

### `encode_sequences`

```rust
fn encode_sequences(sequences: &[crate::blocks::sequence_section::Sequence], writer: &mut crate::bit_io::BitWriter<&mut alloc::vec::Vec<u8>>, ll_table: &crate::fse::fse_encoder::FSETable, ml_table: &crate::fse::fse_encoder::FSETable, of_table: &crate::fse::fse_encoder::FSETable)
```

### `encode_seqnum`

```rust
fn encode_seqnum(seqnum: usize, writer: &mut crate::bit_io::BitWriter<impl AsMut<alloc::vec::Vec<u8>>>)
```

### `encode_literal_length`

```rust
fn encode_literal_length(len: u32) -> (u8, u32, usize)
```

### `encode_match_len`

```rust
fn encode_match_len(len: u32) -> (u8, u32, usize)
```

### `encode_offset`

```rust
fn encode_offset(len: u32) -> (u8, u32, usize)
```

### `raw_literals`

```rust
fn raw_literals(literals: &[u8], writer: &mut crate::bit_io::BitWriter<&mut alloc::vec::Vec<u8>>)
```

### `compress_literals`

```rust
fn compress_literals(literals: &[u8], last_table: Option<&huff0_encoder::HuffmanTable>, writer: &mut crate::bit_io::BitWriter<&mut alloc::vec::Vec<u8>>) -> Option<huff0_encoder::HuffmanTable>
```

