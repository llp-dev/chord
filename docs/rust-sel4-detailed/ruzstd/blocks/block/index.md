*[ruzstd](../../index.md) / [blocks](../index.md) / [block](index.md)*

---

# Module `block`

Block header definitions.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BlockHeader`](#blockheader) | struct | A representation of a single block header. |
| [`BlockType`](#blocktype) | enum | There are 4 different kinds of blocks, and the type of block influences the meaning of `Block_Size`. |

## Structs

### `BlockHeader`

```rust
struct BlockHeader {
    pub last_block: bool,
    pub block_type: BlockType,
    pub decompressed_size: u32,
    pub content_size: u32,
}
```

A representation of a single block header. As well as containing a frame header,
each Zstandard frame contains one or more blocks.

#### Fields

- **`last_block`**: `bool`

  Whether this block is the last block in the frame.
  It may be followed by an optional `Content_Checksum` if it is.

- **`decompressed_size`**: `u32`

  The size of the decompressed data. If the block type
  is [BlockType::Reserved] or [BlockType::Compressed],
  this value is set to zero and should not be referenced.

- **`content_size`**: `u32`

  The size of the block. If the block is [BlockType::RLE],
  this value will be 1.

## Enums

### `BlockType`

```rust
enum BlockType {
    Raw,
    RLE,
    Compressed,
    Reserved,
}
```

There are 4 different kinds of blocks, and the type of block influences the meaning of `Block_Size`.

#### Variants

- **`Raw`**

  An uncompressed block.

- **`RLE`**

  A single byte, repeated `Block_Size` times (Run Length Encoding).

- **`Compressed`**

  A Zstandard compressed block. `Block_Size` is the length of the compressed data.

- **`Reserved`**

  This is not a valid block, and this value should not be used.
  If this value is present, it should be considered corrupted data.

#### Trait Implementations

##### `impl Clone for BlockType`

- <span id="blocktype-clone"></span>`fn clone(&self) -> BlockType` — [`BlockType`](#blocktype)

##### `impl Copy for BlockType`

##### `impl Debug for BlockType`

- <span id="blocktype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BlockType`

- <span id="blocktype-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

##### `impl Eq for BlockType`

##### `impl PartialEq for BlockType`

- <span id="blocktype-partialeq-eq"></span>`fn eq(&self, other: &BlockType) -> bool` — [`BlockType`](#blocktype)

##### `impl StructuralPartialEq for BlockType`

##### `impl ToString for BlockType`

- <span id="blocktype-tostring-to-string"></span>`fn to_string(&self) -> String`

