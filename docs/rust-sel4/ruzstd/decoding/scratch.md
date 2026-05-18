**ruzstd > decoding > scratch**

# Module: decoding::scratch

## Contents

**Structs**

- [`FSEScratch`](#fsescratch)
- [`HuffmanScratch`](#huffmanscratch)

---

## ruzstd::decoding::scratch::FSEScratch

*Struct*

**Fields:**
- `offsets: crate::fse::FSETable`
- `of_rle: Option<u8>`
- `literal_lengths: crate::fse::FSETable`
- `ll_rle: Option<u8>`
- `match_lengths: crate::fse::FSETable`
- `ml_rle: Option<u8>`



## ruzstd::decoding::scratch::HuffmanScratch

*Struct*

**Fields:**
- `table: crate::huff0::HuffmanTable`



