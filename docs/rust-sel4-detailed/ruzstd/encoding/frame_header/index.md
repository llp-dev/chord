*[ruzstd](../../index.md) / [encoding](../index.md) / [frame_header](index.md)*

---

# Module `frame_header`

Utilities and representations for a frame header.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FrameHeader`](#frameheader) | struct | A header for a single Zstandard frame. |
| [`minify_val_fcs`](#minify-val-fcs) | fn | Identical to [`minify_val`], but it implements the following edge case |

## Structs

### `FrameHeader`

```rust
struct FrameHeader {
    pub frame_content_size: Option<u64>,
    pub single_segment: bool,
    pub content_checksum: bool,
    pub dictionary_id: Option<u64>,
    pub window_size: Option<u64>,
}
```

A header for a single Zstandard frame.

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#frame_header>

#### Fields

- **`frame_content_size`**: `Option<u64>`

  Optionally, the original (uncompressed) size of the data within the frame in bytes.
  If not present, `window_size` must be set.

- **`single_segment`**: `bool`

  If set to true, data must be regenerated within a single
  continuous memory segment.

- **`content_checksum`**: `bool`

  If set to true, a 32 bit content checksum will be present
  at the end of the frame.

- **`dictionary_id`**: `Option<u64>`

  If a dictionary ID is provided, the ID of that dictionary.

- **`window_size`**: `Option<u64>`

  The minimum memory buffer required to compress a frame. If not present,
  `single_segment` will be set to true. If present, this value must be greater than 1KB
  and less than 3.75TB. Encoders should not generate a frame that requires a window size larger than
  8mb.

#### Implementations

- <span id="frameheader-serialize"></span>`fn serialize(self, output: &mut Vec<u8>)`

  Writes the serialized frame header into the provided buffer.

  

  The returned header *does include* a frame header descriptor.

- <span id="frameheader-descriptor"></span>`fn descriptor(&self) -> u8`

  Generate a serialized frame header descriptor for the frame header.

  

  https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#frame_header_descriptor

#### Trait Implementations

##### `impl Debug for FrameHeader`

- <span id="frameheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `minify_val_fcs`

```rust
fn minify_val_fcs(val: u64) -> alloc::vec::Vec<u8>
```

Identical to [`minify_val`](../util/index.md), but it implements the following edge case:

> When FCS_Field_Size is 1, 4 or 8 bytes, the value is read directly. When FCS_Field_Size is 2, the offset of 256 is added.

https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#frame_content_size

