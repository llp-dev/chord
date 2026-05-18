**ruzstd > decoding > dictionary**

# Module: decoding::dictionary

## Contents

**Structs**

- [`Dictionary`](#dictionary) - Zstandard includes support for "raw content" dictionaries, that store bytes optionally used

---

## ruzstd::decoding::dictionary::Dictionary

*Struct*

Zstandard includes support for "raw content" dictionaries, that store bytes optionally used
during sequence execution.

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#dictionary-format>

**Fields:**
- `id: u32` - A 4 byte value used by decoders to check if they can use
- `fse: crate::decoding::scratch::FSEScratch` - A dictionary can contain an entropy table, either FSE or
- `huf: crate::decoding::scratch::HuffmanScratch` - A dictionary can contain an entropy table, either FSE or
- `dict_content: alloc::vec::Vec<u8>` - The content of a dictionary acts as a "past" in front of data
- `offset_hist: [u32; 3]` - The 3 most recent offsets are stored so that they can be used



