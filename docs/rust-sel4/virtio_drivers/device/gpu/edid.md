**virtio_drivers > device > gpu > edid**

# Module: device::gpu::edid

## Contents

**Structs**

- [`DetailedTiming`](#detailedtiming) - A Detailed Timing Descriptor from the EDID base block.
- [`Edid`](#edid) - Parsed EDID data from a display device.
- [`StandardTiming`](#standardtiming) - A single standard timing entry from the EDID.

**Enums**

- [`AspectRatio`](#aspectratio) - Aspect ratio encoded in a Standard Timing entry.

**Constants**

- [`DTD1_OFFSET`](#dtd1_offset) - The byte offset of the first Detailed Timing Descriptor in the base block.
- [`DTD_LEN`](#dtd_len) - The byte length of a single Detailed Timing Descriptor.
- [`NUM_STANDARD_TIMINGS`](#num_standard_timings) - The number of standard timing entries in the base EDID block.
- [`STANDARD_TIMINGS_OFFSET`](#standard_timings_offset) - The byte offset of the first Standard Timing entry in the base block.
- [`STANDARD_TIMING_LEN`](#standard_timing_len) - The byte length of a single Standard Timing entry.

---

## virtio_drivers::device::gpu::edid::AspectRatio

*Enum*

Aspect ratio encoded in a Standard Timing entry.

**Variants:**
- `Ratio16x10` - 16:10
- `Ratio4x3` - 4:3
- `Ratio5x4` - 5:4
- `Ratio16x9` - 16:9

**Methods:**

- `fn from_bits(bits: u8) -> Self`
- `fn v_pixels(self: Self, h_pixels: u32) -> u32` - Compute vertical pixels from horizontal pixels and this aspect ratio.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AspectRatio) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AspectRatio`



## virtio_drivers::device::gpu::edid::DTD1_OFFSET

*Constant*: `usize`

The byte offset of the first Detailed Timing Descriptor in the base block.



## virtio_drivers::device::gpu::edid::DTD_LEN

*Constant*: `usize`

The byte length of a single Detailed Timing Descriptor.



## virtio_drivers::device::gpu::edid::DetailedTiming

*Struct*

A Detailed Timing Descriptor from the EDID base block.

Reference: VESA E-EDID, Section 3.10.2 "Detailed Timing Definitions".

**Fields:**
- `h_active: u32` - Horizontal active pixels.
- `v_active: u32` - Vertical active pixels.

**Methods:**

- `fn parse(bytes: &[u8; 18]) -> Option<Self>` - Parse a detailed timing descriptor from its 18-byte encoding.



## virtio_drivers::device::gpu::edid::Edid

*Struct*

Parsed EDID data from a display device.

Wraps the raw EDID byte blob and provides methods to extract display
information such as preferred resolution and supported standard timings.

**Fields:**
- `data: [u8; 1024]`
- `size: u32`

**Methods:**

- `fn has_base_block(self: &Self) -> bool` - Whether the base EDID block (128 bytes) is present.
- `fn first_detailed_timing(self: &Self) -> Option<DetailedTiming>` - Parse the first Detailed Timing Descriptor.
- `fn standard_timing(self: &Self, index: usize) -> Option<StandardTiming>` - Parse a single standard timing entry by index (0-7).
- `fn preferred_resolution(self: &Self) -> Result<(u32, u32)>` - Get the preferred resolution from the EDID data.
- `fn standard_timings(self: &Self) -> Vec<(u32, u32)>` - Get the list of supported resolutions from EDID standard timings.



## virtio_drivers::device::gpu::edid::NUM_STANDARD_TIMINGS

*Constant*: `usize`

The number of standard timing entries in the base EDID block.



## virtio_drivers::device::gpu::edid::STANDARD_TIMINGS_OFFSET

*Constant*: `usize`

The byte offset of the first Standard Timing entry in the base block.



## virtio_drivers::device::gpu::edid::STANDARD_TIMING_LEN

*Constant*: `usize`

The byte length of a single Standard Timing entry.



## virtio_drivers::device::gpu::edid::StandardTiming

*Struct*

A single standard timing entry from the EDID.

Reference: VESA E-EDID, Section 3.9 "Standard Timing Identification".

**Fields:**
- `h_pixels: u32` - Horizontal active pixels.
- `v_pixels: u32` - Vertical active pixels (derived from aspect ratio).

**Methods:**

- `fn parse(bytes: &[u8; 2]) -> Option<Self>` - Parse a standard timing from its 2-byte encoding.



