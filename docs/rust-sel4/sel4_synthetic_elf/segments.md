**sel4_synthetic_elf > segments**

# Module: segments

## Contents

**Structs**

- [`Segment`](#segment)
- [`Segments`](#segments)

**Enums**

- [`SegmentsError`](#segmentserror)

---

## sel4_synthetic_elf::segments::Segment

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `p_flags: u32`
- `p_vaddr: u64`
- `p_paddr: u64`
- `p_memsz: u64`
- `p_align: u64`
- `data: std::borrow::Cow<'a, [u8]>`

**Methods:**

- `fn from_phdr<T, R>(phdr: &T, endian: <T as >::Endian, data: R) -> Result<Self, SegmentsError>`
- `fn simple(vaddr: u64, data: Cow<'a, [u8]>) -> Self`



## sel4_synthetic_elf::segments::Segments

*Struct*

**Generic Parameters:**
- 'a

**Methods:**

- `fn new() -> Self`
- `fn add_segment(self: & mut Self, segment: Segment<'a>)`
- `fn add_segments_from_phdrs<T, R>(self: & mut Self, elf_file: &ElfFile<'a, T, R>) -> Result<(), SegmentsError>`
- `fn footprint(self: &Self) -> Option<Range<u64>>`
- `fn build(self: &Self, endianness: Endianness, is_64: bool, ehdr: &object::write::elf::FileHeader, discard_p_align: bool) -> Result<Vec<u8>, SegmentsError>`
- `fn build_using_ehdr<'data, T, R>(self: &Self, elf_file: &ElfFile<'data, T, R>, discard_p_align: bool) -> Result<Vec<u8>, SegmentsError>`

**Trait Implementations:**

- **Default**
  - `fn default() -> Segments<'a>`



## sel4_synthetic_elf::segments::SegmentsError

*Enum*

**Variants:**
- `ReadError(object::read::Error)`
- `WriteError(object::write::Error)`
- `FileDataError`



