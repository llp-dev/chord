*[ruzstd](../../index.md) / [decoding](../index.md) / [frame](index.md)*

---

# Module `frame`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FrameHeader`](#frameheader) | struct | A frame header has a variable size, with a minimum of 2 bytes, and a maximum of 14 bytes. |
| [`FrameDescriptor`](#framedescriptor) | struct | The first byte is called the `Frame Header Descriptor`, and it describes what other fields are present. |
| [`read_frame_header`](#read-frame-header) | fn | Read a single serialized frame from the reader and return a tuple containing the parsed frame and the number of bytes read. |

## Structs

### `FrameHeader`

```rust
struct FrameHeader {
    pub descriptor: FrameDescriptor,
    window_descriptor: u8,
    dict_id: Option<u32>,
    frame_content_size: u64,
}
```

A frame header has a variable size, with a minimum of 2 bytes, and a maximum of 14 bytes.

#### Fields

- **`window_descriptor`**: `u8`

  The `Window_Descriptor` field contains the minimum size of a memory buffer needed to
  decompress the entire frame.
  
  This byte is not included in the frame header when the `Single_Segment_flag` is set.
  
  Bits 7-3 refer to the `Exponent`, where bits 2-0 refer to the `Mantissa`.
  
  To determine the size of a window, the following formula can be used:
  ```text
  windowLog = 10 + Exponent;
  windowBase = 1 << windowLog;
  windowAdd = (windowBase / 8) * Mantissa;
  Window_Size = windowBase + windowAdd;
  ```
  <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#window_descriptor>

- **`dict_id`**: `Option<u32>`

  The `Dictionary_ID` field contains the ID of the dictionary to be used to decode the frame.
  When this value is not present, it's up to the decoder to know which dictionary to use.

- **`frame_content_size`**: `u64`

  The size of the original/uncompressed content.

#### Implementations

- <span id="frameheader-window-size"></span>`fn window_size(&self) -> Result<u64, FrameHeaderError>` — [`FrameHeaderError`](../errors/index.md#frameheadererror)

  Read the size of the window from the header or the total frame content size,

  whichever is defined, returning the size in bytes.

- <span id="frameheader-dictionary-id"></span>`fn dictionary_id(&self) -> Option<u32>`

  The ID (if provided) of the dictionary required to decode this frame.

- <span id="frameheader-frame-content-size"></span>`fn frame_content_size(&self) -> u64`

  Obtain the uncompressed size (in bytes) of the frame contents.

### `FrameDescriptor`

```rust
struct FrameDescriptor(u8);
```

The first byte is called the `Frame Header Descriptor`, and it describes what other fields
are present.

#### Implementations

- <span id="framedescriptor-frame-content-size-flag"></span>`fn frame_content_size_flag(&self) -> u8`

  Read the `Frame_Content_Size_flag` from the frame header descriptor.

  

  This is a 2 bit flag, specifying if the `Frame_Content_Size` field is present

  within the header. It notates the number of bytes used by `Frame_Content_size`

  

  When this value is is 0, `FCS_Field_Size` depends on Single_Segment_flag.

  If the `Single_Segment_flag` field is set in the frame header descriptor,

  the size of the `Frame_Content_Size` field of the header is 1 byte.

  Otherwise, `FCS_Field_Size` is 0, and the `Frame_Content_Size` is not provided.

  

  | Flag Value (decimal) | Size of the `Frame_Content_Size` field in bytes |

  | -- | -- |

  | 0 | 0 or 1 (see above) |

  | 1 | 2 |

  | 2 | 4 |

  | 3 | 8 |

- <span id="framedescriptor-reserved-flag"></span>`fn reserved_flag(&self) -> bool`

  This bit is reserved for some future feature, a compliant decoder **must ensure**

  that this value is set to zero.

- <span id="framedescriptor-single-segment-flag"></span>`fn single_segment_flag(&self) -> bool`

  If this flag is set, data must be regenerated within a single continuous memory segment.

  

  In this case, the `Window_Descriptor` byte is skipped, but `Frame_Content_Size` is present.

  The decoder must allocate a memory segment equal to or larger than `Frame_Content_Size`.

- <span id="framedescriptor-content-checksum-flag"></span>`fn content_checksum_flag(&self) -> bool`

  If this flag is set, a 32 bit `Content_Checksum` will be present at the end of the frame.

- <span id="framedescriptor-dict-id-flag"></span>`fn dict_id_flag(&self) -> u8`

  This is a two bit flag telling if a dictionary ID is provided within the header. It also

  specifies the size of this field

  

  | Value (Decimal) | `DID_Field_Size` (bytes) |

  | -- | -- |

  | 0 | 0 |

  | 1 | 1 |

  | 2 | 2 |

  | 3 | 4 |

- <span id="framedescriptor-frame-content-size-bytes"></span>`fn frame_content_size_bytes(&self) -> Result<u8, FrameDescriptorError>` — [`FrameDescriptorError`](../errors/index.md#framedescriptorerror)

  Read the size of the `Frame_Content_size` field from the frame header descriptor, returning

  the size in bytes.

  If this value is zero, then the `Frame_Content_Size` field is not present within the header.

- <span id="framedescriptor-dictionary-id-bytes"></span>`fn dictionary_id_bytes(&self) -> Result<u8, FrameDescriptorError>` — [`FrameDescriptorError`](../errors/index.md#framedescriptorerror)

  Read the size of the `Dictionary_ID` field from the frame header descriptor, returning the size in bytes.

  If this value is zero, then the dictionary id is not present within the header,

  and "It's up to the decoder to know which dictionary to use."

## Functions

### `read_frame_header`

```rust
fn read_frame_header(r: impl Read) -> Result<(FrameHeader, u8), crate::decoding::errors::ReadFrameHeaderError>
```

Read a single serialized frame from the reader and return a tuple containing the parsed frame and the number of bytes read.

