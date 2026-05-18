*[ruzstd](../../index.md) / [decoding](../index.md) / [scratch](index.md)*

---

# Module `scratch`

Structures that wrap around various decoders to make decoding easier.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DecoderScratch`](#decoderscratch) | struct | A block level decoding buffer. |
| [`HuffmanScratch`](#huffmanscratch) | struct |  |
| [`FSEScratch`](#fsescratch) | struct |  |

## Structs

### `DecoderScratch`

```rust
struct DecoderScratch {
    pub huf: HuffmanScratch,
    pub fse: FSEScratch,
    pub buffer: super::decode_buffer::DecodeBuffer,
    pub offset_hist: [u32; 3],
    pub literals_buffer: alloc::vec::Vec<u8>,
    pub sequences: alloc::vec::Vec<super::super::blocks::sequence_section::Sequence>,
    pub block_content_buffer: alloc::vec::Vec<u8>,
}
```

A block level decoding buffer.

#### Fields

- **`huf`**: `HuffmanScratch`

  The decoder used for Huffman blocks.

- **`fse`**: `FSEScratch`

  The decoder used for FSE blocks.

#### Implementations

- <span id="decoderscratch-new"></span>`fn new(window_size: usize) -> DecoderScratch` — [`DecoderScratch`](#decoderscratch)

- <span id="decoderscratch-reset"></span>`fn reset(&mut self, window_size: usize)`

- <span id="decoderscratch-init-from-dict"></span>`fn init_from_dict(&mut self, dict: &Dictionary)` — [`Dictionary`](../dictionary/index.md#dictionary)

### `HuffmanScratch`

```rust
struct HuffmanScratch {
    pub table: crate::huff0::HuffmanTable,
}
```

#### Implementations

- <span id="huffmanscratch-new"></span>`fn new() -> HuffmanScratch` — [`HuffmanScratch`](#huffmanscratch)

#### Trait Implementations

##### `impl Default for HuffmanScratch`

- <span id="huffmanscratch-default"></span>`fn default() -> Self`

### `FSEScratch`

```rust
struct FSEScratch {
    pub offsets: crate::fse::FSETable,
    pub of_rle: Option<u8>,
    pub literal_lengths: crate::fse::FSETable,
    pub ll_rle: Option<u8>,
    pub match_lengths: crate::fse::FSETable,
    pub ml_rle: Option<u8>,
}
```

#### Implementations

- <span id="fsescratch-new"></span>`fn new() -> FSEScratch` — [`FSEScratch`](#fsescratch)

- <span id="fsescratch-reinit-from"></span>`fn reinit_from(&mut self, other: &Self)`

#### Trait Implementations

##### `impl Default for FSEScratch`

- <span id="fsescratch-default"></span>`fn default() -> Self`

