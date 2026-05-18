*[ruzstd](../../../index.md) / [encoding](../../index.md) / [levels](../index.md) / [fastest](index.md)*

---

# Module `fastest`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`compress_fastest`](#compress-fastest) | fn | Compresses a single block at [`crate::encoding::CompressionLevel::Fastest`]. |

## Functions

### `compress_fastest`

```rust
fn compress_fastest<M: Matcher>(state: &mut crate::encoding::frame_compressor::CompressState<M>, last_block: bool, uncompressed_data: alloc::vec::Vec<u8>, output: &mut alloc::vec::Vec<u8>)
```

Compresses a single block at [`crate::encoding::CompressionLevel::Fastest`](../../../index.md).

# Parameters
- `state`: [`CompressState`](../../frame_compressor/index.md) so the compressor can refer to data before
  the start of this block
- `last_block`: Whether or not this block is going to be the last block in the frame
  (needed because this info is written into the block header)
- `uncompressed_data`: A block's worth of uncompressed data, taken from the
  larger input
- `output`: As `uncompressed_data` is compressed, it's appended to `output`.

