**miniz_oxide > inflate > core > inflate_flags**

# Module: inflate::core::inflate_flags

## Contents

**Constants**

- [`TINFL_FLAG_COMPUTE_ADLER32`](#tinfl_flag_compute_adler32) - Calculate the adler32 checksum of the output data even if we're not inflating a zlib stream.
- [`TINFL_FLAG_HAS_MORE_INPUT`](#tinfl_flag_has_more_input) - There will be more input that hasn't been given to the decompressor yet.
- [`TINFL_FLAG_IGNORE_ADLER32`](#tinfl_flag_ignore_adler32) - Ignore adler32 checksum even if we are inflating a zlib stream.
- [`TINFL_FLAG_PARSE_ZLIB_HEADER`](#tinfl_flag_parse_zlib_header) - Should we try to parse a zlib header?
- [`TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`](#tinfl_flag_using_non_wrapping_output_buf) - The output buffer should not wrap around.

---

## miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_COMPUTE_ADLER32

*Constant*: `u32`

Calculate the adler32 checksum of the output data even if we're not inflating a zlib stream.

If [`TINFL_FLAG_IGNORE_ADLER32`] is specified, it will override this.

NOTE: Enabling/disabling this between calls to decompress will result in an incorrect
checksum.



## miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_HAS_MORE_INPUT

*Constant*: `u32`

There will be more input that hasn't been given to the decompressor yet.

This is useful when you want to decompress what you have so far,
even if you know there is probably more input that hasn't gotten here yet (_e.g._, over a
network connection).  When [`decompress()`][super::decompress] reaches the end of the input
without finding the end of the compressed stream, it will return
[`TINFLStatus::NeedsMoreInput`][super::TINFLStatus::NeedsMoreInput] if this is set,
indicating that you should get more data before calling again.  If not set, it will return
[`TINFLStatus::FailedCannotMakeProgress`][super::TINFLStatus::FailedCannotMakeProgress]
suggesting the stream is corrupt, since you claimed it was all there.



## miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_IGNORE_ADLER32

*Constant*: `u32`

Ignore adler32 checksum even if we are inflating a zlib stream.

Overrides [`TINFL_FLAG_COMPUTE_ADLER32`] if both are enabled.

NOTE: This flag does not exist in miniz as it does not support this and is a
custom addition for miniz_oxide.

NOTE: Should not be changed from enabled to disabled after decompression has started,
this will result in checksum failure (outside the unlikely event where the checksum happens
to match anyway).



## miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER

*Constant*: `u32`

Should we try to parse a zlib header?

If unset, the function will expect an RFC1951 deflate stream.  If set, it will expect a
RFC1950 zlib wrapper around the deflate stream.



## miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF

*Constant*: `u32`

The output buffer should not wrap around.



