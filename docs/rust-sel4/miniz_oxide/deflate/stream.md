**miniz_oxide > deflate > stream**

# Module: deflate::stream

## Contents

**Functions**

- [`deflate`](#deflate) - Try to compress from input to output with the given [`CompressorOxide`].

---

## miniz_oxide::deflate::stream::deflate

*Function*

Try to compress from input to output with the given [`CompressorOxide`].

# Errors

Returns [`MZError::Buf`] If the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called without [`MZFlush::Finish`] after the compression
was already finished.

Returns [`MZError::Param`] if the compressor parameters are set wrong.

Returns [`MZError::Stream`] when lower-level decompressor returns a
[`TDEFLStatus::PutBufFailed`]; may not actually be possible.

```rust
fn deflate(compressor: & mut crate::deflate::core::CompressorOxide, input: &[u8], output: & mut [u8], flush: crate::MZFlush) -> crate::StreamResult
```



