*[ruzstd](../../index.md) / [decoding](../index.md) / [dictionary](index.md)*

---

# Module `dictionary`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Dictionary`](#dictionary) | struct | Zstandard includes support for "raw content" dictionaries, that store bytes optionally used during sequence execution. |
| [`MAGIC_NUM`](#magic-num) | const | This 4 byte (little endian) magic number refers to the start of a dictionary |

## Structs

### `Dictionary`

```rust
struct Dictionary {
    pub id: u32,
    pub fse: crate::decoding::scratch::FSEScratch,
    pub huf: crate::decoding::scratch::HuffmanScratch,
    pub dict_content: alloc::vec::Vec<u8>,
    pub offset_hist: [u32; 3],
}
```

Zstandard includes support for "raw content" dictionaries, that store bytes optionally used
during sequence execution.

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#dictionary-format>

#### Fields

- **`id`**: `u32`

  A 4 byte value used by decoders to check if they can use
  the correct dictionary. This value must not be zero.

- **`fse`**: `crate::decoding::scratch::FSEScratch`

  A dictionary can contain an entropy table, either FSE or
  Huffman.

- **`huf`**: `crate::decoding::scratch::HuffmanScratch`

  A dictionary can contain an entropy table, either FSE or
  Huffman.

- **`dict_content`**: `alloc::vec::Vec<u8>`

  The content of a dictionary acts as a "past" in front of data
  to compress or decompress,
  so it can be referenced in sequence commands.
  As long as the amount of data decoded from this frame is less than or
  equal to Window_Size, sequence commands may specify offsets longer than
  the total length of decoded output so far to reference back to the
  dictionary, even parts of the dictionary with offsets larger than Window_Size.
  After the total output has surpassed Window_Size however,
  this is no longer allowed and the dictionary is no longer accessible

- **`offset_hist`**: `[u32; 3]`

  The 3 most recent offsets are stored so that they can be used
  during sequence execution, see
  <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#repeat-offsets>
  for more.

#### Implementations

- <span id="dictionary-decode-dict"></span>`fn decode_dict(raw: &[u8]) -> Result<Dictionary, DictionaryDecodeError>` — [`Dictionary`](#dictionary), [`DictionaryDecodeError`](../errors/index.md#dictionarydecodeerror)

  Parses the dictionary from `raw` and set the tables

  it returns the dict_id for checking with the frame's `dict_id``

## Constants

### `MAGIC_NUM`
```rust
const MAGIC_NUM: [u8; 4];
```

This 4 byte (little endian) magic number refers to the start of a dictionary

