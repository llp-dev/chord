**ruzstd > blocks > block**

# Module: blocks::block

## Contents

**Enums**

- [`BlockType`](#blocktype) - There are 4 different kinds of blocks, and the type of block influences the meaning of `Block_Size`.

---

## ruzstd::blocks::block::BlockType

*Enum*

There are 4 different kinds of blocks, and the type of block influences the meaning of `Block_Size`.

**Variants:**
- `Raw` - An uncompressed block.
- `RLE` - A single byte, repeated `Block_Size` times (Run Length Encoding).
- `Compressed` - A Zstandard compressed block. `Block_Size` is the length of the compressed data.
- `Reserved` - This is not a valid block, and this value should not be used.



