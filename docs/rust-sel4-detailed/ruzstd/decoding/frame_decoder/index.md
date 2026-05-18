*[ruzstd](../../index.md) / [decoding](../index.md) / [frame_decoder](index.md)*

---

# Module `frame_decoder`

Framedecoder is the main low-level struct users interact with to decode zstd frames

Zstandard compressed data is made of one or more frames. Each frame is independent and can be
decompressed independently of other frames. This module contains structures
and utilities that can be used to decode a frame.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FrameDecoder`](#framedecoder) | struct | Low level Zstandard decoder that can be used to decompress frames with fine control over when and how many bytes are decoded. |
| [`FrameDecoderState`](#framedecoderstate) | struct |  |
| [`BlockDecodingStrategy`](#blockdecodingstrategy) | enum |  |
| [`MAXIMUM_ALLOWED_WINDOW_SIZE`](#maximum-allowed-window-size) | const | While the maximum window size allowed by the spec is significantly larger, our implementation limits it to 100mb to protect against malformed frames. |

## Structs

### `FrameDecoder`

```rust
struct FrameDecoder {
    state: Option<FrameDecoderState>,
    dicts: alloc::collections::BTreeMap<u32, crate::decoding::dictionary::Dictionary>,
}
```

Low level Zstandard decoder that can be used to decompress frames with fine control over when and how many bytes are decoded.

This decoder is able to decode frames only partially and gives control
over how many bytes/blocks will be decoded at a time (so you don't have to decode a 10GB file into memory all at once).
It reads bytes as needed from a provided source and can be read from to collect partial results.

If you want to just read the whole frame with an `io::Read` without having to deal with manually calling [FrameDecoder::decode_blocks]
you can use the provided [crate::decoding::StreamingDecoder] wich wraps this FrameDecoder.

Workflow is as follows:
```rust
use ruzstd::decoding::BlockDecodingStrategy;

#[cfg(feature = "std")]
use std::io::{Read, Write};

// no_std environments can use the crate's own Read traits
#[cfg(not(feature = "std"))]
use ruzstd::io::{Read, Write};

fn decode_this(mut file: impl Read) {
    //Create a new decoder
    let mut frame_dec = ruzstd::decoding::FrameDecoder::new();
    let mut result = Vec::new();

    // Use reset or init to make the decoder ready to decode the frame from the io::Read
    frame_dec.reset(&mut file).unwrap();

    // Loop until the frame has been decoded completely
    while !frame_dec.is_finished() {
        // decode (roughly) batch_size many bytes
        frame_dec.decode_blocks(&mut file, BlockDecodingStrategy::UptoBytes(1024)).unwrap();

        // read from the decoder to collect bytes from the internal buffer
        let bytes_read = frame_dec.read(result.as_mut_slice()).unwrap();

        // then do something with it
        do_something(&result[0..bytes_read]);
    }

    // handle the last chunk of data
    while frame_dec.can_collect() > 0 {
        let x = frame_dec.read(result.as_mut_slice()).unwrap();

        do_something(&result[0..x]);
    }
}

fn do_something(data: &[u8]) {
#[cfg(feature = "std")]
    std::io::stdout().write_all(data).unwrap();
}
```

#### Implementations

- <span id="framedecoder-new"></span>`fn new() -> FrameDecoder` — [`FrameDecoder`](#framedecoder)

  This will create a new decoder without allocating anything yet.

  init()/reset() will allocate all needed buffers if it is the first time this decoder is used

  else they just reset these buffers with not further allocations

- <span id="framedecoder-init"></span>`fn init(&mut self, source: impl Read) -> Result<(), FrameDecoderError>` — [`Read`](../../io_std/index.md#read), [`FrameDecoderError`](../errors/index.md#framedecodererror)

  init() will allocate all needed buffers if it is the first time this decoder is used

  else they just reset these buffers with not further allocations

  

  Note that all bytes currently in the decodebuffer from any previous frame will be lost. Collect them with collect()/collect_to_writer()

  

  equivalent to reset()

- <span id="framedecoder-reset"></span>`fn reset(&mut self, source: impl Read) -> Result<(), FrameDecoderError>` — [`Read`](../../io_std/index.md#read), [`FrameDecoderError`](../errors/index.md#framedecodererror)

  reset() will allocate all needed buffers if it is the first time this decoder is used

  else they just reset these buffers with not further allocations

  

  Note that all bytes currently in the decodebuffer from any previous frame will be lost. Collect them with collect()/collect_to_writer()

  

  equivalent to init()

- <span id="framedecoder-add-dict"></span>`fn add_dict(&mut self, dict: Dictionary) -> Result<(), FrameDecoderError>` — [`Dictionary`](../dictionary/index.md#dictionary), [`FrameDecoderError`](../errors/index.md#framedecodererror)

  Add a dict to the FrameDecoder that can be used when needed. The FrameDecoder uses the appropriate one dynamically

- <span id="framedecoder-force-dict"></span>`fn force_dict(&mut self, dict_id: u32) -> Result<(), FrameDecoderError>` — [`FrameDecoderError`](../errors/index.md#framedecodererror)

- <span id="framedecoder-content-size"></span>`fn content_size(&self) -> u64`

  Returns how many bytes the frame contains after decompression

- <span id="framedecoder-get-checksum-from-data"></span>`fn get_checksum_from_data(&self) -> Option<u32>`

  Returns the checksum that was read from the data. Only available after all bytes have been read. It is the last 4 bytes of a zstd-frame

- <span id="framedecoder-get-calculated-checksum"></span>`fn get_calculated_checksum(&self) -> Option<u32>`

  Returns the checksum that was calculated while decoding.

  Only a sensible value after all decoded bytes have been collected/read from the FrameDecoder

- <span id="framedecoder-bytes-read-from-source"></span>`fn bytes_read_from_source(&self) -> u64`

  Counter for how many bytes have been consumed while decoding the frame

- <span id="framedecoder-is-finished"></span>`fn is_finished(&self) -> bool`

  Whether the current frames last block has been decoded yet

  If this returns true you can call the drain* functions to get all content

  (the read() function will drain automatically if this returns true)

- <span id="framedecoder-blocks-decoded"></span>`fn blocks_decoded(&self) -> usize`

  Counter for how many blocks have already been decoded

- <span id="framedecoder-decode-blocks"></span>`fn decode_blocks(&mut self, source: impl Read, strat: BlockDecodingStrategy) -> Result<bool, FrameDecoderError>` — [`Read`](../../io_std/index.md#read), [`BlockDecodingStrategy`](#blockdecodingstrategy), [`FrameDecoderError`](../errors/index.md#framedecodererror)

  Decodes blocks from a reader. It requires that the framedecoder has been initialized first.

  The Strategy influences how many blocks will be decoded before the function returns

  This is important if you want to manage memory consumption carefully. If you don't care

  about that you can just choose the strategy "All" and have all blocks of the frame decoded into the buffer

- <span id="framedecoder-collect"></span>`fn collect(&mut self) -> Option<Vec<u8>>`

  Collect bytes and retain window_size bytes while decoding is still going on.

  After decoding of the frame (is_finished() == true) has finished it will collect all remaining bytes

- <span id="framedecoder-collect-to-writer"></span>`fn collect_to_writer(&mut self, w: impl Write) -> Result<usize, Error>` — [`Write`](../../io_std/index.md#write), [`Error`](../../io_std/index.md#error)

  Collect bytes and retain window_size bytes while decoding is still going on.

  After decoding of the frame (is_finished() == true) has finished it will collect all remaining bytes

- <span id="framedecoder-can-collect"></span>`fn can_collect(&self) -> usize`

  How many bytes can currently be collected from the decodebuffer, while decoding is going on this will be lower than the actual decodbuffer size

  because window_size bytes need to be retained for decoding.

  After decoding of the frame (is_finished() == true) has finished it will report all remaining bytes

- <span id="framedecoder-decode-from-to"></span>`fn decode_from_to(&mut self, source: &[u8], target: &mut [u8]) -> Result<(usize, usize), FrameDecoderError>` — [`FrameDecoderError`](../errors/index.md#framedecodererror)

  Decodes as many blocks as possible from the source slice and reads from the decodebuffer into the target slice

  The source slice may contain only parts of a frame but must contain at least one full block to make progress

  

  By all means use decode_blocks if you have a io.Reader available. This is just for compatibility with other decompressors

  which try to serve an old-style c api

  

  Returns (read, written), if read == 0 then the source did not contain a full block and further calls with the same

  input will not make any progress!

  

  Note that no kind of block can be bigger than 128kb.

  So to be safe use at least 128*1024 (max block content size) + 3 (block_header size) + 18 (max frame_header size) bytes as your source buffer

  

  You may call this function with an empty source after all bytes have been decoded. This is equivalent to just call decoder.read(&mut target)

- <span id="framedecoder-decode-all"></span>`fn decode_all(&mut self, input: &[u8], output: &mut [u8]) -> Result<usize, FrameDecoderError>` — [`FrameDecoderError`](../errors/index.md#framedecodererror)

  Decode multiple frames into the output slice.

  

  `input` must contain an exact number of frames.

  

  `output` must be large enough to hold the decompressed data. If you don't know

  how large the output will be, use `FrameDecoder::decode_blocks` instead.

  

  This calls `FrameDecoder::init`, and all bytes currently in the decoder will be lost.

  

  Returns the number of bytes written to `output`.

- <span id="framedecoder-decode-all-to-vec"></span>`fn decode_all_to_vec(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<(), FrameDecoderError>` — [`FrameDecoderError`](../errors/index.md#framedecodererror)

  Decode multiple frames into the extra capacity of the output vector.

  

  `input` must contain an exact number of frames.

  

  `output` must have enough extra capacity to hold the decompressed data.

  This function will not reallocate or grow the vector. If you don't know

  how large the output will be, use `FrameDecoder::decode_blocks` instead.

  

  This calls `FrameDecoder::init`, and all bytes currently in the decoder will be lost.

  

  The length of the output vector is updated to include the decompressed data.

  The length is not changed if an error occurs.

#### Trait Implementations

##### `impl Default for FrameDecoder`

- <span id="framedecoder-default"></span>`fn default() -> Self`

##### `impl Read for FrameDecoder`

- <span id="framedecoder-read"></span>`fn read(&mut self, target: &mut [u8]) -> Result<usize, Error>` — [`Error`](../../io_std/index.md#error)

### `FrameDecoderState`

```rust
struct FrameDecoderState {
    pub frame_header: frame::FrameHeader,
    decoder_scratch: crate::decoding::scratch::DecoderScratch,
    frame_finished: bool,
    block_counter: usize,
    bytes_read_counter: u64,
    check_sum: Option<u32>,
    using_dict: Option<u32>,
}
```

#### Implementations

- <span id="framedecoderstate-new"></span>`fn new(source: impl Read) -> Result<FrameDecoderState, FrameDecoderError>` — [`Read`](../../io_std/index.md#read), [`FrameDecoderState`](#framedecoderstate), [`FrameDecoderError`](../errors/index.md#framedecodererror)

- <span id="framedecoderstate-reset"></span>`fn reset(&mut self, source: impl Read) -> Result<(), FrameDecoderError>` — [`Read`](../../io_std/index.md#read), [`FrameDecoderError`](../errors/index.md#framedecodererror)

## Enums

### `BlockDecodingStrategy`

```rust
enum BlockDecodingStrategy {
    All,
    UptoBlocks(usize),
    UptoBytes(usize),
}
```

## Constants

### `MAXIMUM_ALLOWED_WINDOW_SIZE`
```rust
const MAXIMUM_ALLOWED_WINDOW_SIZE: u64 = 104_857_600u64;
```

While the maximum window size allowed by the spec is significantly larger,
our implementation limits it to 100mb to protect against malformed frames.

