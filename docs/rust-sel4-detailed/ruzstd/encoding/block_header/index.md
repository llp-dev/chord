*[ruzstd](../../index.md) / [encoding](../index.md) / [block_header](index.md)*

---

# Module `block_header`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BlockHeader`](#blockheader) | struct |  |

## Structs

### `BlockHeader`

```rust
struct BlockHeader {
    pub last_block: bool,
    pub block_type: crate::blocks::block::BlockType,
    pub block_size: u32,
}
```

#### Fields

- **`last_block`**: `bool`

  Signals if this block is the last one.
  The frame will end after this block.

- **`block_type`**: `crate::blocks::block::BlockType`

  Influences the meaning of `block_size`.

- **`block_size`**: `u32`

  - For `Raw` blocks, this is the size of the block's
    content in bytes.
  - For `RLE` blocks, there will be a single byte follwing
    the header, repeated `block_size` times.
  - For `Compressed` blocks, this is the length of
    the compressed data.
  
  **This value must not be greater than 21 bits in length.**

#### Implementations

- <span id="blockheader-serialize"></span>`fn serialize(self, output: &mut Vec<u8>)`

  Write encoded binary representation of this header into the provided buffer.

#### Trait Implementations

##### `impl Debug for BlockHeader`

- <span id="blockheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

