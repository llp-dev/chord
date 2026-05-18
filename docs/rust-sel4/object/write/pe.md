**object > write > pe**

# Module: write::pe

## Contents

**Structs**

- [`NtHeaders`](#ntheaders) - Information required for writing [`pe::ImageNtHeaders32`] or [`pe::ImageNtHeaders64`].
- [`Section`](#section) - Information required for writing [`pe::ImageSectionHeader`].
- [`SectionRange`](#sectionrange) - The file range and virtual address range for a section.
- [`Writer`](#writer) - A helper for writing PE files.

---

## object::write::pe::NtHeaders

*Struct*

Information required for writing [`pe::ImageNtHeaders32`] or [`pe::ImageNtHeaders64`].

**Fields:**
- `machine: u16`
- `time_date_stamp: u32`
- `characteristics: u16`
- `major_linker_version: u8`
- `minor_linker_version: u8`
- `address_of_entry_point: u32`
- `image_base: u64`
- `major_operating_system_version: u16`
- `minor_operating_system_version: u16`
- `major_image_version: u16`
- `minor_image_version: u16`
- `major_subsystem_version: u16`
- `minor_subsystem_version: u16`
- `subsystem: u16`
- `dll_characteristics: u16`
- `size_of_stack_reserve: u64`
- `size_of_stack_commit: u64`
- `size_of_heap_reserve: u64`
- `size_of_heap_commit: u64`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NtHeaders`



## object::write::pe::Section

*Struct*

Information required for writing [`pe::ImageSectionHeader`].

**Fields:**
- `name: [u8; 8]`
- `characteristics: u32`
- `range: SectionRange`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Section`



## object::write::pe::SectionRange

*Struct*

The file range and virtual address range for a section.

**Fields:**
- `virtual_address: u32`
- `virtual_size: u32`
- `file_offset: u32`
- `file_size: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionRange`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> SectionRange`



## object::write::pe::Writer

*Struct*

A helper for writing PE files.

Writing uses a two phase approach. The first phase reserves file ranges and virtual
address ranges for everything in the order that they will be written.

The second phase writes everything out in order. Thus the caller must ensure writing
is in the same order that file ranges were reserved.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(is_64: bool, section_alignment: u32, file_alignment: u32, buffer: &'a  mut dyn WritableBuffer) -> Self` - Create a new `Writer`.
- `fn virtual_len(self: &Self) -> u32` - Return the current virtual address size that has been reserved.
- `fn reserve_virtual(self: & mut Self, len: u32) -> u32` - Reserve a virtual address range with the given size.
- `fn reserve_virtual_until(self: & mut Self, address: u32)` - Reserve up to the given virtual address.
- `fn reserved_len(self: &Self) -> u32` - Return the current file length that has been reserved.
- `fn len(self: &Self) -> usize` - Return the current file length that has been written.
- `fn reserve(self: & mut Self, len: u32, align_start: u32) -> u32` - Reserve a file range with the given size and starting alignment.
- `fn reserve_file(self: & mut Self, len: u32) -> u32` - Reserve a file range with the given size and using the file alignment.
- `fn write(self: & mut Self, data: &[u8])` - Write data.
- `fn reserve_align(self: & mut Self, align_start: u32)` - Reserve alignment padding bytes.
- `fn write_align(self: & mut Self, align_start: u32)` - Write alignment padding bytes.
- `fn write_file_align(self: & mut Self)` - Write padding up to the next multiple of file alignment.
- `fn reserve_until(self: & mut Self, offset: u32)` - Reserve the file range up to the given file offset.
- `fn pad_until(self: & mut Self, offset: u32)` - Write padding up to the given file offset.
- `fn reserve_dos_header(self: & mut Self)` - Reserve the range for the DOS header.
- `fn write_custom_dos_header(self: & mut Self, dos_header: &pe::ImageDosHeader) -> Result<()>` - Write a custom DOS header.
- `fn write_empty_dos_header(self: & mut Self) -> Result<()>` - Write the DOS header for a file without a stub.
- `fn reserve_dos_header_and_stub(self: & mut Self)` - Reserve a fixed DOS header and stub.
- `fn write_dos_header_and_stub(self: & mut Self) -> Result<()>` - Write a fixed DOS header and stub.
- `fn nt_headers_offset(self: &Self) -> u32` - Return the offset of the NT headers, if reserved.
- `fn reserve_nt_headers(self: & mut Self, data_directory_num: usize)` - Reserve the range for the NT headers.
- `fn set_data_directory(self: & mut Self, index: usize, virtual_address: u32, size: u32)` - Set the virtual address and size of a data directory.
- `fn write_nt_headers(self: & mut Self, nt_headers: NtHeaders)` - Write the NT headers.
- `fn reserve_section_headers(self: & mut Self, section_header_num: u16)` - Reserve the section headers.
- `fn write_section_headers(self: & mut Self)` - Write the section headers.
- `fn reserve_section(self: & mut Self, name: [u8; 8], characteristics: u32, virtual_size: u32, data_size: u32) -> SectionRange` - Reserve a section.
- `fn write_section(self: & mut Self, offset: u32, data: &[u8])` - Write the data for a section.
- `fn reserve_text_section(self: & mut Self, size: u32) -> SectionRange` - Reserve a `.text` section.
- `fn reserve_data_section(self: & mut Self, virtual_size: u32, data_size: u32) -> SectionRange` - Reserve a `.data` section.
- `fn reserve_rdata_section(self: & mut Self, size: u32) -> SectionRange` - Reserve a `.rdata` section.
- `fn reserve_bss_section(self: & mut Self, size: u32) -> SectionRange` - Reserve a `.bss` section.
- `fn reserve_idata_section(self: & mut Self, size: u32) -> SectionRange` - Reserve an `.idata` section.
- `fn reserve_edata_section(self: & mut Self, size: u32) -> SectionRange` - Reserve an `.edata` section.
- `fn reserve_pdata_section(self: & mut Self, size: u32) -> SectionRange` - Reserve a `.pdata` section.
- `fn reserve_xdata_section(self: & mut Self, size: u32) -> SectionRange` - Reserve a `.xdata` section.
- `fn reserve_rsrc_section(self: & mut Self, size: u32) -> SectionRange` - Reserve a `.rsrc` section.
- `fn add_reloc(self: & mut Self, virtual_address: u32, typ: u16)` - Add a base relocation.
- `fn has_relocs(self: & mut Self) -> bool` - Return true if a base relocation has been added.
- `fn reserve_reloc_section(self: & mut Self) -> SectionRange` - Reserve a `.reloc` section.
- `fn write_reloc_section(self: & mut Self)` - Write a `.reloc` section.
- `fn reserve_certificate_table(self: & mut Self, size: u32)` - Reserve the certificate table.
- `fn write_certificate_table(self: & mut Self, data: &[u8])` - Write the certificate table.



