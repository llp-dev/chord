**ruzstd > fse > fse_decoder**

# Module: fse::fse_decoder

## Contents

**Structs**

- [`Entry`](#entry) - A single entry in an FSE table.
- [`FSETable`](#fsetable) - FSE decoding involves a decoding table that describes the probabilities of

---

## ruzstd::fse::fse_decoder::Entry

*Struct*

A single entry in an FSE table.

**Fields:**
- `base_line: u32` - This value is used as an offset value, and it is added
- `num_bits: u8` - How many bits should be read from the stream when decoding this entry.
- `symbol: u8` - The byte that should be put in the decode output when encountering this state.



## ruzstd::fse::fse_decoder::FSETable

*Struct*

FSE decoding involves a decoding table that describes the probabilities of
all literals from 0 to the highest present one

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#fse-table-description>

**Fields:**
- `decode: alloc::vec::Vec<Entry>` - The actual table containing the decoded symbol and the compression data
- `accuracy_log: u8` - The size of the table is stored in logarithm base 2 format,
- `symbol_probabilities: alloc::vec::Vec<i32>` - In this context, probability refers to the likelihood that a symbol occurs in the given data.



