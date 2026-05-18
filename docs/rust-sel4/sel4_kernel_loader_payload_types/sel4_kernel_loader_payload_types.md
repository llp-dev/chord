**sel4_kernel_loader_payload_types**

# Module: sel4_kernel_loader_payload_types

## Contents

**Structs**

- [`DirectRegionContent`](#directregioncontent)
- [`ImageInfo`](#imageinfo)
- [`IndirectRegionContent`](#indirectregioncontent)
- [`Payload`](#payload)
- [`PayloadInfo`](#payloadinfo)
- [`Region`](#region)

**Traits**

- [`RegionContent`](#regioncontent)

**Constants**

- [`DEFAULT_MAX_NUM_REGIONS`](#default_max_num_regions)

---

## sel4_kernel_loader_payload_types::DEFAULT_MAX_NUM_REGIONS

*Constant*: `usize`



## sel4_kernel_loader_payload_types::DirectRegionContent

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `content: &'a [u8]`

**Traits:** Eq

**Trait Implementations:**

- **RegionContent**
  - `fn len(self: &Self) -> usize`
  - `fn copy_out(self: &Self, _source: &<Self as >::Source, dst: & mut [u8])`
- **PartialEq**
  - `fn eq(self: &Self, other: &DirectRegionContent<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DirectRegionContent<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_kernel_loader_payload_types::ImageInfo

*Struct*

**Generic Parameters:**
- T

**Fields:**
- `phys_addr_range: core::ops::Range<T>`
- `phys_to_virt_offset: T`
- `virt_entry: T`

**Methods:**

- `fn virt_addr_range(self: &Self) -> Range<T>`
- `fn phys_to_virt(self: &Self, paddr: T) -> T`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ImageInfo<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ImageInfo<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_kernel_loader_payload_types::IndirectRegionContent

*Struct*

**Generic Parameters:**
- T

**Fields:**
- `content_range: core::ops::Range<T>`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IndirectRegionContent<T>) -> bool`
- **RegionContent**
  - `fn len(self: &Self) -> usize`
  - `fn copy_out(self: &Self, source: &<Self as >::Source, dst: & mut [u8])`
- **Clone**
  - `fn clone(self: &Self) -> IndirectRegionContent<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_kernel_loader_payload_types::Payload

*Struct*

**Generic Parameters:**
- T
- U
- const N

**Fields:**
- `info: PayloadInfo<T>`
- `data: heapless::Vec<Region<T, U>, N>`

**Methods:**

- `fn copy_data_out(self: &Self, region_content_source: &<U as >::Source)`
- `fn sanity_check<T>(self: &Self, platform_info: &PlatformInfo<T>, own_footprint: Range<usize>)`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Payload<T, U, N>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Payload<T, U, N>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_kernel_loader_payload_types::PayloadInfo

*Struct*

**Generic Parameters:**
- T

**Fields:**
- `kernel_image: ImageInfo<T>`
- `user_image: ImageInfo<T>`
- `fdt_phys_addr_range: Option<core::ops::Range<T>>`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PayloadInfo<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PayloadInfo<T>) -> bool`



## sel4_kernel_loader_payload_types::Region

*Struct*

**Generic Parameters:**
- T
- U

**Fields:**
- `phys_addr_range: core::ops::Range<T>`
- `content: Option<U>`

**Methods:**

- `fn traverse<V, E, impl FnMut(&U) -> Result<V, E>>(self: &Self, f: impl Trait) -> Result<Region<T, V>, E>`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Region<T, U>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Region<T, U>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_kernel_loader_payload_types::RegionContent

*Trait*

**Methods:**

- `Source`
- `len`
- `copy_out`



