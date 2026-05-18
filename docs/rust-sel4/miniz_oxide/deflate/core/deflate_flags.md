**miniz_oxide > deflate > core > deflate_flags**

# Module: deflate::core::deflate_flags

## Contents

**Constants**

- [`TDEFL_COMPUTE_ADLER32`](#tdefl_compute_adler32) - Should we compute the adler32 checksum.
- [`TDEFL_FILTER_MATCHES`](#tdefl_filter_matches) - Only use matches that are at least 6 bytes long.
- [`TDEFL_FORCE_ALL_RAW_BLOCKS`](#tdefl_force_all_raw_blocks) - Force the compressor to only output raw/uncompressed blocks.
- [`TDEFL_FORCE_ALL_STATIC_BLOCKS`](#tdefl_force_all_static_blocks) - Force the compressor to only output static blocks. (Blocks using the default huffman codes
- [`TDEFL_GREEDY_PARSING_FLAG`](#tdefl_greedy_parsing_flag) - Should we use greedy parsing (as opposed to lazy parsing where look ahead one or more
- [`TDEFL_NONDETERMINISTIC_PARSING_FLAG`](#tdefl_nondeterministic_parsing_flag) - Used in miniz to skip zero-initializing hash and dict. We don't do this here, so
- [`TDEFL_RLE_MATCHES`](#tdefl_rle_matches) - Only look for matches with a distance of 0.
- [`TDEFL_WRITE_ZLIB_HEADER`](#tdefl_write_zlib_header) - Whether to use a zlib wrapper.

---

## miniz_oxide::deflate::core::deflate_flags::TDEFL_COMPUTE_ADLER32

*Constant*: `u32`

Should we compute the adler32 checksum.



## miniz_oxide::deflate::core::deflate_flags::TDEFL_FILTER_MATCHES

*Constant*: `u32`

Only use matches that are at least 6 bytes long.



## miniz_oxide::deflate::core::deflate_flags::TDEFL_FORCE_ALL_RAW_BLOCKS

*Constant*: `u32`

Force the compressor to only output raw/uncompressed blocks.



## miniz_oxide::deflate::core::deflate_flags::TDEFL_FORCE_ALL_STATIC_BLOCKS

*Constant*: `u32`

Force the compressor to only output static blocks. (Blocks using the default huffman codes
specified in the deflate specification.)



## miniz_oxide::deflate::core::deflate_flags::TDEFL_GREEDY_PARSING_FLAG

*Constant*: `u32`

Should we use greedy parsing (as opposed to lazy parsing where look ahead one or more
bytes to check for better matches.)



## miniz_oxide::deflate::core::deflate_flags::TDEFL_NONDETERMINISTIC_PARSING_FLAG

*Constant*: `u32`

Used in miniz to skip zero-initializing hash and dict. We don't do this here, so
this flag is ignored.



## miniz_oxide::deflate::core::deflate_flags::TDEFL_RLE_MATCHES

*Constant*: `u32`

Only look for matches with a distance of 0.



## miniz_oxide::deflate::core::deflate_flags::TDEFL_WRITE_ZLIB_HEADER

*Constant*: `u32`

Whether to use a zlib wrapper.



