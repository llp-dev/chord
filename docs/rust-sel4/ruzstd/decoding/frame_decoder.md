**ruzstd > decoding > frame_decoder**

# Module: decoding::frame_decoder

## Contents

**Structs**

- [`FrameDecoder`](#framedecoder) - Low level Zstandard decoder that can be used to decompress frames with fine control over when and how many bytes are decoded.

**Enums**

- [`BlockDecodingStrategy`](#blockdecodingstrategy)

---

## ruzstd::decoding::frame_decoder::BlockDecodingStrategy

*Enum*

**Variants:**
- `All`
- `UptoBlocks(usize)`
- `UptoBytes(usize)`



## ruzstd::decoding::frame_decoder::FrameDecoder

*Struct*

Low level Zstandard decoder that can be used to decompress frames with fine control over when and how many bytes are decoded.

This decoder is able to decode frames only partially and gives control
over how many bytes/blocks will be decoded at a time (so you don't have to decode a 10GB file into memory all at once).
It reads bytes as needed from a provided source and can be read from to collect partial results.

If you want to just read the whole frame with an `io::Read` without having to deal with manually calling [FrameDecoder::decode_blocks]
you can use the provided [crate::decoding::StreamingDecoder] wich wraps this FrameDecoder.

Workflow is as follows:
```
use ruzstd::decoding::BlockDecodingStrategy;

# #[cfg(feature = "std")]
use std::io::{Read, Write};

// no_std environments can use the crate's own Read traits
# #[cfg(not(feature = "std"))]
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
# #[cfg(feature = "std")]
    std::io::stdout().write_all(data).unwrap();
}
```

**Methods:**

- `fn new() -> FrameDecoder` - This will create a new decoder without allocating anything yet.
- `fn init<impl Read>(self: & mut Self, source: impl Trait) -> Result<(), FrameDecoderError>` - init() will allocate all needed buffers if it is the first time this decoder is used
- `fn reset<impl Read>(self: & mut Self, source: impl Trait) -> Result<(), FrameDecoderError>` - reset() will allocate all needed buffers if it is the first time this decoder is used
- `fn add_dict(self: & mut Self, dict: Dictionary) -> Result<(), FrameDecoderError>` - Add a dict to the FrameDecoder that can be used when needed. The FrameDecoder uses the appropriate one dynamically
- `fn force_dict(self: & mut Self, dict_id: u32) -> Result<(), FrameDecoderError>`
- `fn content_size(self: &Self) -> u64` - Returns how many bytes the frame contains after decompression
- `fn get_checksum_from_data(self: &Self) -> Option<u32>` - Returns the checksum that was read from the data. Only available after all bytes have been read. It is the last 4 bytes of a zstd-frame
- `fn get_calculated_checksum(self: &Self) -> Option<u32>` - Returns the checksum that was calculated while decoding.
- `fn bytes_read_from_source(self: &Self) -> u64` - Counter for how many bytes have been consumed while decoding the frame
- `fn is_finished(self: &Self) -> bool` - Whether the current frames last block has been decoded yet
- `fn blocks_decoded(self: &Self) -> usize` - Counter for how many blocks have already been decoded
- `fn decode_blocks<impl Read>(self: & mut Self, source: impl Trait, strat: BlockDecodingStrategy) -> Result<bool, FrameDecoderError>` - Decodes blocks from a reader. It requires that the framedecoder has been initialized first.
- `fn collect(self: & mut Self) -> Option<Vec<u8>>` - Collect bytes and retain window_size bytes while decoding is still going on.
- `fn collect_to_writer<impl Write>(self: & mut Self, w: impl Trait) -> Result<usize, Error>` - Collect bytes and retain window_size bytes while decoding is still going on.
- `fn can_collect(self: &Self) -> usize` - How many bytes can currently be collected from the decodebuffer, while decoding is going on this will be lower than the actual decodbuffer size
- `fn decode_from_to(self: & mut Self, source: &[u8], target: & mut [u8]) -> Result<(usize, usize), FrameDecoderError>` - Decodes as many blocks as possible from the source slice and reads from the decodebuffer into the target slice
- `fn decode_all(self: & mut Self, input: &[u8], output: & mut [u8]) -> Result<usize, FrameDecoderError>` - Decode multiple frames into the output slice.
- `fn decode_all_to_vec(self: & mut Self, input: &[u8], output: & mut Vec<u8>) -> Result<(), FrameDecoderError>` - Decode multiple frames into the extra capacity of the output vector.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Read**
  - `fn read(self: & mut Self, target: & mut [u8]) -> Result<usize, Error>`



