*[object](../../index.md) / [write](../index.md) / [pe](index.md)*

---

# Module `pe`

Helper for writing PE files.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Writer`](#writer) | struct | A helper for writing PE files. |
| [`NtHeaders`](#ntheaders) | struct | Information required for writing [`pe::ImageNtHeaders32`] or [`pe::ImageNtHeaders64`]. |
| [`DataDirectory`](#datadirectory) | struct |  |
| [`Section`](#section) | struct | Information required for writing [`pe::ImageSectionHeader`]. |
| [`SectionRange`](#sectionrange) | struct | The file range and virtual address range for a section. |
| [`RelocBlock`](#relocblock) | struct |  |

## Structs

### `Writer<'a>`

```rust
struct Writer<'a> {
    is_64: bool,
    section_alignment: u32,
    file_alignment: u32,
    buffer: &'a mut dyn WritableBuffer,
    len: u32,
    virtual_len: u32,
    headers_len: u32,
    code_address: u32,
    data_address: u32,
    code_len: u32,
    data_len: u32,
    bss_len: u32,
    nt_headers_offset: u32,
    data_directories: alloc::vec::Vec<DataDirectory>,
    section_header_num: u16,
    sections: alloc::vec::Vec<Section>,
    symbol_offset: u32,
    symbol_num: u32,
    reloc_blocks: alloc::vec::Vec<RelocBlock>,
    relocs: alloc::vec::Vec<U16<crate::endian::LittleEndian>>,
    reloc_offset: u32,
}
```

A helper for writing PE files.

Writing uses a two phase approach. The first phase reserves file ranges and virtual
address ranges for everything in the order that they will be written.

The second phase writes everything out in order. Thus the caller must ensure writing
is in the same order that file ranges were reserved.

#### Implementations

- <span id="writer-new"></span>`fn new(is_64: bool, section_alignment: u32, file_alignment: u32, buffer: &'a mut dyn WritableBuffer) -> Self` — [`WritableBuffer`](../index.md#writablebuffer)

  Create a new `Writer`.

  

  The alignment values must be powers of two.

- <span id="writer-virtual-len"></span>`fn virtual_len(&self) -> u32`

  Return the current virtual address size that has been reserved.

  

  This is only valid after section headers have been reserved.

- <span id="writer-reserve-virtual"></span>`fn reserve_virtual(&mut self, len: u32) -> u32`

  Reserve a virtual address range with the given size.

  

  The reserved length will be increased to match the section alignment.

  

  Returns the aligned offset of the start of the range.

- <span id="writer-reserve-virtual-until"></span>`fn reserve_virtual_until(&mut self, address: u32)`

  Reserve up to the given virtual address.

  

  The reserved length will be increased to match the section alignment.

- <span id="writer-reserved-len"></span>`fn reserved_len(&self) -> u32`

  Return the current file length that has been reserved.

- <span id="writer-len"></span>`fn len(&self) -> usize`

  Return the current file length that has been written.

- <span id="writer-reserve"></span>`fn reserve(&mut self, len: u32, align_start: u32) -> u32`

  Reserve a file range with the given size and starting alignment.

  

  Returns the aligned offset of the start of the range.

- <span id="writer-reserve-file"></span>`fn reserve_file(&mut self, len: u32) -> u32`

  Reserve a file range with the given size and using the file alignment.

  

  Returns the aligned offset of the start of the range.

- <span id="writer-write"></span>`fn write(&mut self, data: &[u8])`

  Write data.

- <span id="writer-reserve-align"></span>`fn reserve_align(&mut self, align_start: u32)`

  Reserve alignment padding bytes.

- <span id="writer-write-align"></span>`fn write_align(&mut self, align_start: u32)`

  Write alignment padding bytes.

- <span id="writer-write-file-align"></span>`fn write_file_align(&mut self)`

  Write padding up to the next multiple of file alignment.

- <span id="writer-reserve-until"></span>`fn reserve_until(&mut self, offset: u32)`

  Reserve the file range up to the given file offset.

- <span id="writer-pad-until"></span>`fn pad_until(&mut self, offset: u32)`

  Write padding up to the given file offset.

- <span id="writer-reserve-dos-header"></span>`fn reserve_dos_header(&mut self)`

  Reserve the range for the DOS header.

  

  This must be at the start of the file.

  

  When writing, you may use `write_custom_dos_header` or `write_empty_dos_header`.

- <span id="writer-write-custom-dos-header"></span>`fn write_custom_dos_header(&mut self, dos_header: &pe::ImageDosHeader) -> Result<()>` — [`ImageDosHeader`](../../pe/index.md#imagedosheader), [`Result`](../index.md#result)

  Write a custom DOS header.

  

  This must be at the start of the file.

- <span id="writer-write-empty-dos-header"></span>`fn write_empty_dos_header(&mut self) -> Result<()>` — [`Result`](../index.md#result)

  Write the DOS header for a file without a stub.

  

  This must be at the start of the file.

  

  Uses default values for all fields.

- <span id="writer-reserve-dos-header-and-stub"></span>`fn reserve_dos_header_and_stub(&mut self)`

  Reserve a fixed DOS header and stub.

  

  Use `reserve_dos_header` and `reserve` if you need a custom stub.

- <span id="writer-write-dos-header-and-stub"></span>`fn write_dos_header_and_stub(&mut self) -> Result<()>` — [`Result`](../index.md#result)

  Write a fixed DOS header and stub.

  

  Use `write_custom_dos_header` and `write` if you need a custom stub.

- <span id="writer-nt-headers-size"></span>`fn nt_headers_size(&self) -> u32`

- <span id="writer-optional-header-size"></span>`fn optional_header_size(&self) -> u32`

- <span id="writer-nt-headers-offset"></span>`fn nt_headers_offset(&self) -> u32`

  Return the offset of the NT headers, if reserved.

- <span id="writer-reserve-nt-headers"></span>`fn reserve_nt_headers(&mut self, data_directory_num: usize)`

  Reserve the range for the NT headers.

- <span id="writer-set-data-directory"></span>`fn set_data_directory(&mut self, index: usize, virtual_address: u32, size: u32)`

  Set the virtual address and size of a data directory.

- <span id="writer-write-nt-headers"></span>`fn write_nt_headers(&mut self, nt_headers: NtHeaders)` — [`NtHeaders`](#ntheaders)

  Write the NT headers.

- <span id="writer-reserve-section-headers"></span>`fn reserve_section_headers(&mut self, section_header_num: u16)`

  Reserve the section headers.

  

  The number of reserved section headers must be the same as the number of sections that

  are later reserved.

- <span id="writer-write-section-headers"></span>`fn write_section_headers(&mut self)`

  Write the section headers.

  

  This uses information that was recorded when the sections were reserved.

- <span id="writer-reserve-section"></span>`fn reserve_section(&mut self, name: [u8; 8], characteristics: u32, virtual_size: u32, data_size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a section.

  

  Returns the file range and virtual address range that are reserved

  for the section.

- <span id="writer-write-section"></span>`fn write_section(&mut self, offset: u32, data: &[u8])`

  Write the data for a section.

- <span id="writer-reserve-text-section"></span>`fn reserve_text_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.text` section.

  

  Contains executable code.

- <span id="writer-reserve-data-section"></span>`fn reserve_data_section(&mut self, virtual_size: u32, data_size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.data` section.

  

  Contains initialized data.

  

  May also contain uninitialized data if `virtual_size` is greater than `data_size`.

- <span id="writer-reserve-rdata-section"></span>`fn reserve_rdata_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.rdata` section.

  

  Contains read-only initialized data.

- <span id="writer-reserve-bss-section"></span>`fn reserve_bss_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.bss` section.

  

  Contains uninitialized data.

- <span id="writer-reserve-idata-section"></span>`fn reserve_idata_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve an `.idata` section.

  

  Contains import tables. Note that it is permissible to store import tables in a different

  section.

  

  This also sets the `pe::IMAGE_DIRECTORY_ENTRY_IMPORT` data directory.

- <span id="writer-reserve-edata-section"></span>`fn reserve_edata_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve an `.edata` section.

  

  Contains export tables.

  

  This also sets the `pe::IMAGE_DIRECTORY_ENTRY_EXPORT` data directory.

- <span id="writer-reserve-pdata-section"></span>`fn reserve_pdata_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.pdata` section.

  

  Contains exception information.

  

  This also sets the `pe::IMAGE_DIRECTORY_ENTRY_EXCEPTION` data directory.

- <span id="writer-reserve-xdata-section"></span>`fn reserve_xdata_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.xdata` section.

  

  Contains exception information.

- <span id="writer-reserve-rsrc-section"></span>`fn reserve_rsrc_section(&mut self, size: u32) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.rsrc` section.

  

  Contains the resource directory.

  

  This also sets the `pe::IMAGE_DIRECTORY_ENTRY_RESOURCE` data directory.

- <span id="writer-add-reloc"></span>`fn add_reloc(&mut self, virtual_address: u32, typ: u16)`

  Add a base relocation.

  

  `typ` must be one of the `IMAGE_REL_BASED_*` constants.

- <span id="writer-has-relocs"></span>`fn has_relocs(&mut self) -> bool`

  Return true if a base relocation has been added.

- <span id="writer-reserve-reloc-section"></span>`fn reserve_reloc_section(&mut self) -> SectionRange` — [`SectionRange`](#sectionrange)

  Reserve a `.reloc` section.

  

  This contains the base relocations that were added with `add_reloc`.

  

  This also sets the `pe::IMAGE_DIRECTORY_ENTRY_BASERELOC` data directory.

- <span id="writer-write-reloc-section"></span>`fn write_reloc_section(&mut self)`

  Write a `.reloc` section.

  

  This contains the base relocations that were added with `add_reloc`.

- <span id="writer-reserve-certificate-table"></span>`fn reserve_certificate_table(&mut self, size: u32)`

  Reserve the certificate table.

  

  This also sets the `pe::IMAGE_DIRECTORY_ENTRY_SECURITY` data directory.

- <span id="writer-write-certificate-table"></span>`fn write_certificate_table(&mut self, data: &[u8])`

  Write the certificate table.

### `NtHeaders`

```rust
struct NtHeaders {
    pub machine: u16,
    pub time_date_stamp: u32,
    pub characteristics: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub address_of_entry_point: u32,
    pub image_base: u64,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
}
```

Information required for writing [`pe::ImageNtHeaders32`](../../pe/index.md) or [`pe::ImageNtHeaders64`](../../pe/index.md).

#### Trait Implementations

##### `impl Clone for NtHeaders`

- <span id="ntheaders-clone"></span>`fn clone(&self) -> NtHeaders` — [`NtHeaders`](#ntheaders)

##### `impl Debug for NtHeaders`

- <span id="ntheaders-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DataDirectory`

```rust
struct DataDirectory {
    virtual_address: u32,
    size: u32,
}
```

#### Trait Implementations

##### `impl Clone for DataDirectory`

- <span id="datadirectory-clone"></span>`fn clone(&self) -> DataDirectory` — [`DataDirectory`](#datadirectory)

##### `impl Copy for DataDirectory`

##### `impl Default for DataDirectory`

- <span id="datadirectory-default"></span>`fn default() -> DataDirectory` — [`DataDirectory`](#datadirectory)

### `Section`

```rust
struct Section {
    pub name: [u8; 8],
    pub characteristics: u32,
    pub range: SectionRange,
}
```

Information required for writing [`pe::ImageSectionHeader`](../../pe/index.md).

#### Trait Implementations

##### `impl Clone for Section`

- <span id="section-clone"></span>`fn clone(&self) -> Section` — [`Section`](#section)

##### `impl Debug for Section`

- <span id="section-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SectionRange`

```rust
struct SectionRange {
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub file_offset: u32,
    pub file_size: u32,
}
```

The file range and virtual address range for a section.

#### Trait Implementations

##### `impl Clone for SectionRange`

- <span id="sectionrange-clone"></span>`fn clone(&self) -> SectionRange` — [`SectionRange`](#sectionrange)

##### `impl Copy for SectionRange`

##### `impl Debug for SectionRange`

- <span id="sectionrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionRange`

- <span id="sectionrange-default"></span>`fn default() -> SectionRange` — [`SectionRange`](#sectionrange)

### `RelocBlock`

```rust
struct RelocBlock {
    virtual_address: u32,
    count: u32,
}
```

#### Implementations

- <span id="relocblock-size"></span>`fn size(&self) -> u32`

