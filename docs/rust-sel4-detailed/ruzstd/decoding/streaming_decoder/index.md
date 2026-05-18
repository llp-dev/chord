*[ruzstd](../../index.md) / [decoding](../index.md) / [streaming_decoder](index.md)*

---

# Module `streaming_decoder`

The [StreamingDecoder] wraps a [FrameDecoder] and provides a Read impl that decodes data when necessary

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StreamingDecoder`](#streamingdecoder) | struct | High level Zstandard frame decoder that can be used to decompress a given Zstandard frame. |

## Structs

### `StreamingDecoder<READ: Read, DEC: BorrowMut<crate::decoding::FrameDecoder>>`

```rust
struct StreamingDecoder<READ: Read, DEC: BorrowMut<crate::decoding::FrameDecoder>> {
    pub decoder: DEC,
    source: READ,
}
```

High level Zstandard frame decoder that can be used to decompress a given Zstandard frame.

This decoder implements `io::Read`, so you can interact with it by calling
`io::Read::read_to_end` / `io::Read::read_exact` or passing this to another library / module as a source for the decoded content

If you need more control over how decompression takes place, you can use
the lower level [FrameDecoder], which allows for greater control over how
decompression takes place but the implementor must call
[FrameDecoder::decode_blocks] repeatedly to decode the entire frame.

## Caveat
[StreamingDecoder] expects the underlying stream to only contain a single frame,
yet the specification states that a single archive may contain multiple frames.

To decode all the frames in a finite stream, the calling code needs to recreate
the instance of the decoder and handle
[crate::decoding::errors::ReadFrameHeaderError::SkipFrame]
errors by skipping forward the `length` amount of bytes, see <https://github.com/KillingSpark/zstd-rs/issues/57>

```no_run
// `read_to_end` is not implemented by the no_std implementation.
#[cfg(feature = "std")]
{
    use std::fs::File;
    use std::io::Read;
    use ruzstd::decoding::StreamingDecoder;

    // Read a Zstandard archive from the filesystem then decompress it into a vec.
    let mut f: File = todo!("Read a .zstd archive from somewhere");
    let mut decoder = StreamingDecoder::new(f).unwrap();
    let mut result = Vec::new();
    Read::read_to_end(&mut decoder, &mut result).unwrap();
}
```

#### Implementations

- <span id="streamingdecoder-new-with-decoder"></span>`fn new_with_decoder(source: READ, decoder: DEC) -> Result<StreamingDecoder<READ, DEC>, FrameDecoderError>` — [`StreamingDecoder`](#streamingdecoder), [`FrameDecoderError`](../errors/index.md#framedecodererror)

#### Trait Implementations

##### `impl<READ: Read, DEC: BorrowMut<crate::decoding::FrameDecoder>> Read for StreamingDecoder<READ, DEC>`

- <span id="streamingdecoder-read"></span>`fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>` — [`Error`](../../io_std/index.md#error)

