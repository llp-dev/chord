*[ruzstd](../../index.md) / [decoding](../index.md) / [block_decoder](index.md)*

---

# Module `block_decoder`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BlockDecoder`](#blockdecoder) | struct |  |
| [`DecoderState`](#decoderstate) | enum |  |
| [`new`](#new) | fn | Create a new [BlockDecoder]. |

## Structs

### `BlockDecoder`

```rust
struct BlockDecoder {
    header_buffer: [u8; 3],
    internal_state: DecoderState,
}
```

#### Implementations

- <span id="blockdecoder-decode-block-content"></span>`fn decode_block_content(&mut self, header: &BlockHeader, workspace: &mut DecoderScratch, source: impl Read) -> Result<u64, DecodeBlockContentError>` — [`BlockHeader`](../../blocks/block/index.md#blockheader), [`DecoderScratch`](../scratch/index.md#decoderscratch), [`Read`](../../io_std/index.md#read), [`DecodeBlockContentError`](../errors/index.md#decodeblockcontenterror)

- <span id="blockdecoder-decompress-block"></span>`fn decompress_block(&mut self, header: &BlockHeader, workspace: &mut DecoderScratch, source: impl Read) -> Result<(), DecompressBlockError>` — [`BlockHeader`](../../blocks/block/index.md#blockheader), [`DecoderScratch`](../scratch/index.md#decoderscratch), [`Read`](../../io_std/index.md#read), [`DecompressBlockError`](../errors/index.md#decompressblockerror)

- <span id="blockdecoder-read-block-header"></span>`fn read_block_header(&mut self, r: impl Read) -> Result<(BlockHeader, u8), BlockHeaderReadError>` — [`Read`](../../io_std/index.md#read), [`BlockHeader`](../../blocks/block/index.md#blockheader), [`BlockHeaderReadError`](../errors/index.md#blockheaderreaderror)

  Reads 3 bytes from the provided reader and returns

  the deserialized header and the number of bytes read.

- <span id="blockdecoder-reset-buffer"></span>`fn reset_buffer(&mut self)`

- <span id="blockdecoder-is-last"></span>`fn is_last(&self) -> bool`

- <span id="blockdecoder-block-type"></span>`fn block_type(&self) -> Result<BlockType, BlockTypeError>` — [`BlockType`](../../blocks/block/index.md#blocktype), [`BlockTypeError`](../errors/index.md#blocktypeerror)

- <span id="blockdecoder-block-content-size"></span>`fn block_content_size(&self) -> Result<u32, BlockSizeError>` — [`BlockSizeError`](../errors/index.md#blocksizeerror)

- <span id="blockdecoder-block-content-size-unchecked"></span>`fn block_content_size_unchecked(&self) -> u32`

## Enums

### `DecoderState`

```rust
enum DecoderState {
    ReadyToDecodeNextHeader,
    ReadyToDecodeNextBody,
    Failed,
}
```

## Functions

### `new`

```rust
fn new() -> BlockDecoder
```

Create a new [BlockDecoder].

