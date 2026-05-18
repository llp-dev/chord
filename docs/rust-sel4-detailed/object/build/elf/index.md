*[object](../../index.md) / [build](../index.md) / [elf](index.md)*

---

# Module `elf`

This module provides a [`Builder`](#builder) for reading, modifying, and then writing ELF files.

## Contents

- [Structs](#structs)
  - [`Builder`](#builder)
  - [`Header`](#header)
  - [`SegmentId`](#segmentid)
  - [`Segment`](#segment)
  - [`SectionId`](#sectionid)
  - [`Section`](#section)
  - [`SymbolId`](#symbolid)
  - [`Symbol`](#symbol)
  - [`Relocation`](#relocation)
  - [`VersionFileId`](#versionfileid)
  - [`VersionFile`](#versionfile)
  - [`VersionId`](#versionid)
  - [`Version`](#version)
  - [`VersionDef`](#versiondef)
  - [`VersionNeed`](#versionneed)
  - [`AttributesSection`](#attributessection)
  - [`AttributesSubsection`](#attributessubsection)
  - [`AttributesSubsubsection`](#attributessubsubsection)
- [Enums](#enums)
  - [`SectionData`](#sectiondata)
  - [`Dynamic`](#dynamic)
  - [`VersionData`](#versiondata)
  - [`AttributeTag`](#attributetag)
- [Type Aliases](#type-aliases)
  - [`Segments`](#segments)
  - [`Sections`](#sections)
  - [`Symbols`](#symbols)
  - [`DynamicSymbolId`](#dynamicsymbolid)
  - [`DynamicSymbol`](#dynamicsymbol)
  - [`DynamicSymbols`](#dynamicsymbols)
  - [`DynamicRelocation`](#dynamicrelocation)
  - [`VersionFiles`](#versionfiles)
  - [`Versions`](#versions)
- [Constants](#constants)
  - [`VERSION_ID_BASE`](#version-id-base)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Builder`](#builder) | struct | A builder for reading, modifying, and then writing ELF files. |
| [`Header`](#header) | struct | ELF file header. |
| [`SegmentId`](#segmentid) | struct | An ID for referring to a segment in [`Segments`]. |
| [`Segment`](#segment) | struct | A segment in [`Segments`]. |
| [`SectionId`](#sectionid) | struct | An ID for referring to a section in [`Sections`]. |
| [`Section`](#section) | struct | A section in [`Sections`]. |
| [`SymbolId`](#symbolid) | struct | An ID for referring to a symbol in [`Symbols`]. |
| [`Symbol`](#symbol) | struct | A symbol in [`Symbols`]. |
| [`Relocation`](#relocation) | struct | A relocation stored in a [`Section`]. |
| [`VersionFileId`](#versionfileid) | struct | An ID for referring to a filename in [`VersionFiles`]. |
| [`VersionFile`](#versionfile) | struct | A filename used for GNU versioning. |
| [`VersionId`](#versionid) | struct | An ID for referring to a version in [`Versions`]. |
| [`Version`](#version) | struct | A version for a symbol. |
| [`VersionDef`](#versiondef) | struct | A GNU version definition. |
| [`VersionNeed`](#versionneed) | struct | A GNU version dependency. |
| [`AttributesSection`](#attributessection) | struct | The contents of an attributes section. |
| [`AttributesSubsection`](#attributessubsection) | struct | A subsection of an attributes section. |
| [`AttributesSubsubsection`](#attributessubsubsection) | struct | A sub-subsection in an attributes section. |
| [`SectionData`](#sectiondata) | enum | The data for a [`Section`]. |
| [`Dynamic`](#dynamic) | enum | An entry in the dynamic section. |
| [`VersionData`](#versiondata) | enum | The data for a version for a symbol. |
| [`AttributeTag`](#attributetag) | enum | The tag for a sub-subsection in an attributes section. |
| [`Segments`](#segments) | type | A segment table. |
| [`Sections`](#sections) | type | A section table. |
| [`Symbols`](#symbols) | type | A symbol table. |
| [`DynamicSymbolId`](#dynamicsymbolid) | type | A dynamic symbol ID. |
| [`DynamicSymbol`](#dynamicsymbol) | type | A dynamic symbol. |
| [`DynamicSymbols`](#dynamicsymbols) | type | A dynamic symbol table. |
| [`DynamicRelocation`](#dynamicrelocation) | type | A dynamic relocation. |
| [`VersionFiles`](#versionfiles) | type | A table of filenames used for GNU versioning. |
| [`Versions`](#versions) | type | A table of versions that are referenced by symbols. |
| [`VERSION_ID_BASE`](#version-id-base) | const |  |

## Structs

### `Builder<'data>`

```rust
struct Builder<'data> {
    pub endian: crate::Endianness,
    pub is_64: bool,
    pub load_align: u64,
    pub header: Header,
    pub segments: Segments<'data>,
    pub sections: Sections<'data>,
    pub symbols: Symbols<'data>,
    pub dynamic_symbols: DynamicSymbols<'data>,
    pub version_base: Option<crate::build::ByteString<'data>>,
    pub versions: Versions<'data>,
    pub version_files: VersionFiles<'data>,
    pub hash_bucket_count: u32,
    pub gnu_hash_bloom_shift: u32,
    pub gnu_hash_bloom_count: u32,
    pub gnu_hash_bucket_count: u32,
    marker: core::marker::PhantomData<()>,
}
```

A builder for reading, modifying, and then writing ELF files.

Public fields are available for modifying the values that will be written.
Methods are available to add elements to tables, and elements can be deleted
from tables by setting the `delete` field in the element.

#### Fields

- **`endian`**: `crate::Endianness`

  The endianness.
  
  Used to set the data encoding when writing the ELF file.

- **`is_64`**: `bool`

  Whether file is 64-bit.
  
  Use to set the file class when writing the ELF file.

- **`load_align`**: `u64`

  The alignment of [`elf::PT_LOAD`](../../elf/index.md) segments.
  
  This is an informational field and is not used when writing the ELF file.
  It can optionally be used when calling `Segments::add_load_segment`.
  
  It is determined heuristically when reading the ELF file. Currently,
  if all load segments have the same alignment, that alignment is used,
  otherwise it is set to 1.

- **`header`**: `Header`

  The file header.

- **`segments`**: `Segments<'data>`

  The segment table.

- **`sections`**: `Sections<'data>`

  The section table.

- **`symbols`**: `Symbols<'data>`

  The symbol table.

- **`dynamic_symbols`**: `DynamicSymbols<'data>`

  The dynamic symbol table.

- **`version_base`**: `Option<crate::build::ByteString<'data>>`

  The base version for the GNU version definitions.
  
  This will be written as a version definition with index 1.

- **`versions`**: `Versions<'data>`

  The GNU version definitions and dependencies.

- **`version_files`**: `VersionFiles<'data>`

  The filenames used in the GNU version definitions.

- **`hash_bucket_count`**: `u32`

  The bucket count parameter for the hash table.

- **`gnu_hash_bloom_shift`**: `u32`

  The bloom shift parameter for the GNU hash table.

- **`gnu_hash_bloom_count`**: `u32`

  The bloom count parameter for the GNU hash table.

- **`gnu_hash_bucket_count`**: `u32`

  The bucket count parameter for the GNU hash table.

#### Implementations

- <span id="builder-new"></span>`fn new(endian: Endianness, is_64: bool) -> Self` — [`Endianness`](../../index.md#endianness)

  Create a new ELF builder.

- <span id="builder-read"></span>`fn read<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../error/index.md#result)

  Read the ELF file from file data.

- <span id="builder-read32"></span>`fn read32<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../error/index.md#result)

  Read a 32-bit ELF file from file data.

- <span id="builder-read64"></span>`fn read64<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../error/index.md#result)

  Read a 64-bit ELF file from file data.

- <span id="builder-read-file"></span>`fn read_file<Elf, R>(data: R) -> Result<Self>` — [`Result`](../error/index.md#result)

- <span id="builder-read-relocations"></span>`fn read_relocations<Elf, Rel, R>(index: read::SectionIndex, endian: <Elf as >::Endian, is_mips64el: bool, section: &'data <Elf as >::SectionHeader, rels: &'data [Rel], link: read::SectionIndex, symbols: &read::elf::SymbolTable<'data, Elf, R>, dynamic_symbols: &read::elf::SymbolTable<'data, Elf, R>) -> Result<SectionData<'data>>` — [`SectionIndex`](../../index.md#sectionindex), [`FileHeader`](../../read/elf/index.md#fileheader), [`SymbolTable`](../../read/elf/index.md#symboltable), [`Result`](../error/index.md#result), [`SectionData`](#sectiondata)

- <span id="builder-read-relocations-impl"></span>`fn read_relocations_impl<Elf, Rel, const DYNAMIC: bool>(index: read::SectionIndex, endian: <Elf as >::Endian, is_mips64el: bool, rels: &'data [Rel], symbols_len: usize) -> Result<Vec<Relocation<DYNAMIC>>>` — [`SectionIndex`](../../index.md#sectionindex), [`FileHeader`](../../read/elf/index.md#fileheader), [`Result`](../error/index.md#result), [`Relocation`](#relocation)

- <span id="builder-read-dynamics"></span>`fn read_dynamics<Elf, R>(endian: <Elf as >::Endian, dyns: &'data [<Elf as >::Dyn], strings: read::StringTable<'data, R>) -> Result<SectionData<'data>>` — [`FileHeader`](../../read/elf/index.md#fileheader), [`StringTable`](../../read/index.md#stringtable), [`Result`](../error/index.md#result), [`SectionData`](#sectiondata)

- <span id="builder-read-symbols"></span>`fn read_symbols<Elf, R, const DYNAMIC: bool>(endian: <Elf as >::Endian, symbols: &read::elf::SymbolTable<'data, Elf, R>, builder_symbols: &mut Symbols<'data, DYNAMIC>, sections_len: usize) -> Result<()>` — [`FileHeader`](../../read/elf/index.md#fileheader), [`SymbolTable`](../../read/elf/index.md#symboltable), [`Symbols`](#symbols), [`Result`](../error/index.md#result)

- <span id="builder-read-attributes"></span>`fn read_attributes<Elf>(index: read::SectionIndex, attributes: read::elf::AttributesSection<'data, Elf>, sections_len: usize, symbols_len: usize) -> Result<SectionData<'data>>` — [`SectionIndex`](../../index.md#sectionindex), [`AttributesSection`](../../read/elf/index.md#attributessection), [`Result`](../error/index.md#result), [`SectionData`](#sectiondata)

- <span id="builder-read-gnu-versions"></span>`fn read_gnu_versions<Elf, R>(&mut self, endian: <Elf as >::Endian, data: R, sections: &read::elf::SectionTable<'data, Elf, R>, dynamic_symbols: &read::elf::SymbolTable<'data, Elf, R>) -> Result<()>` — [`FileHeader`](../../read/elf/index.md#fileheader), [`SectionTable`](../../read/elf/index.md#sectiontable), [`SymbolTable`](../../read/elf/index.md#symboltable), [`Result`](../error/index.md#result)

- <span id="builder-write"></span>`fn write(self, buffer: &mut dyn write::WritableBuffer) -> Result<()>` — [`WritableBuffer`](../../write/index.md#writablebuffer), [`Result`](../error/index.md#result)

  Write the ELF file to the buffer.

- <span id="builder-delete-orphans"></span>`fn delete_orphans(&mut self)`

  Delete segments, symbols, relocations, and dynamics that refer

  to deleted items.

  

  This calls `delete_orphan_segments`, `delete_orphan_symbols`,

  `delete_orphan_relocations`, and `delete_orphan_dynamics`.

- <span id="builder-delete-orphan-segments"></span>`fn delete_orphan_segments(&mut self)`

  Set the delete flag for segments that only refer to deleted sections.

- <span id="builder-delete-orphan-symbols"></span>`fn delete_orphan_symbols(&mut self)`

  Set the delete flag for symbols that refer to deleted sections.

- <span id="builder-delete-orphan-relocations"></span>`fn delete_orphan_relocations(&mut self)`

  Delete relocations that refer to deleted symbols.

- <span id="builder-delete-orphan-dynamics"></span>`fn delete_orphan_dynamics(&mut self)`

  Delete dynamic entries that refer to deleted sections.

- <span id="builder-delete-unused-versions"></span>`fn delete_unused_versions(&mut self)`

  Delete unused GNU version entries.

- <span id="builder-class"></span>`fn class(&self) -> write::elf::Class` — [`Class`](../../write/elf/index.md#class)

  Return the ELF file class that will be written.

  

  This can be useful for calculating sizes.

- <span id="builder-file-header-size"></span>`fn file_header_size(&self) -> usize`

  Calculate the size of the file header.

- <span id="builder-program-headers-size"></span>`fn program_headers_size(&self) -> usize`

  Calculate the size of the program headers.

- <span id="builder-dynamic-symbol-size"></span>`fn dynamic_symbol_size(&self) -> usize`

  Calculate the size of the dynamic symbol table.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols`.

- <span id="builder-dynamic-string-size"></span>`fn dynamic_string_size(&self) -> usize`

  Calculate the size of the dynamic string table.

  

  This adds all of the currently used dynamic strings to a string table,

  calculates the size of the string table, and discards the string table.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols` and `Self::delete_unused_versions`.

- <span id="builder-hash-size"></span>`fn hash_size(&self) -> usize`

  Calculate the size of the hash table.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols`.

- <span id="builder-gnu-hash-size"></span>`fn gnu_hash_size(&self) -> usize`

  Calculate the size of the GNU hash table.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols`.

- <span id="builder-gnu-versym-size"></span>`fn gnu_versym_size(&self) -> usize`

  Calculate the size of the GNU symbol version section.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols` and `Self::delete_unused_versions`.

- <span id="builder-gnu-verdef-size"></span>`fn gnu_verdef_size(&self) -> usize`

  Calculate the size of the GNU version definition section.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols` and `Self::delete_unused_versions`.

- <span id="builder-gnu-verneed-size"></span>`fn gnu_verneed_size(&self) -> usize`

  Calculate the size of the GNU version dependency section.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols` and `Self::delete_unused_versions`.

- <span id="builder-section-size"></span>`fn section_size(&self, section: &Section<'_>) -> usize` — [`Section`](#section)

  Calculate the memory size of a section.

  

  Returns 0 for sections that are deleted or aren't allocated.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols` and `Self::delete_unused_versions`.

- <span id="builder-set-section-sizes"></span>`fn set_section_sizes(&mut self)`

  Set the `sh_size` field for every allocated section.

  

  This is useful to call prior to doing memory layout.

  

  To get an accurate result, you may need to first call

  `Self::delete_orphan_symbols` and `Self::delete_unused_versions`.

- <span id="builder-dynamic-section"></span>`fn dynamic_section(&self) -> Option<SectionId>` — [`SectionId`](#sectionid)

  Find the section containing the dynamic table.

  

  This uses the `PT_DYNAMIC` program header to find the dynamic section.

- <span id="builder-dynamic-data"></span>`fn dynamic_data(&self) -> Option<&[Dynamic<'data>]>` — [`Dynamic`](#dynamic)

  Find the dynamic table entries.

  

  This uses the `PT_DYNAMIC` program header to find the dynamic section,

- <span id="builder-dynamic-data-mut"></span>`fn dynamic_data_mut(&mut self) -> Option<&mut Vec<Dynamic<'data>>>` — [`Dynamic`](#dynamic)

  Find the dynamic table entries.

  

  This uses the `PT_DYNAMIC` program header to find the dynamic section,

- <span id="builder-interp-section"></span>`fn interp_section(&self) -> Option<SectionId>` — [`SectionId`](#sectionid)

  Find the section containing the interpreter path.

  

  This uses the `PT_INTERP` program header to find the interp section.

- <span id="builder-interp-data"></span>`fn interp_data(&self) -> Option<&[u8]>`

  Find the interpreter path.

  

  This uses the `PT_INTERP` program header to find the interp section.

- <span id="builder-interp-data-mut"></span>`fn interp_data_mut(&mut self) -> Option<&mut Bytes<'data>>` — [`Bytes`](../bytes/index.md#bytes)

  Find the interpreter path.

  

  This uses the `PT_INTERP` program header to find the interp section.

#### Trait Implementations

##### `impl Debug for Builder<'data>`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Header`

```rust
struct Header {
    pub os_abi: u8,
    pub abi_version: u8,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_entry: u64,
    pub e_flags: u32,
    pub e_phoff: u64,
}
```

ELF file header.

This corresponds to fields in [`elf::FileHeader32`](../../elf/index.md) or [`elf::FileHeader64`](../../elf/index.md).
This only contains the ELF file header fields that can be modified.
The other fields are automatically calculated.

#### Fields

- **`os_abi`**: `u8`

  The OS ABI field in the file header.
  
  One of the `ELFOSABI*` constants.

- **`abi_version`**: `u8`

  The ABI version field in the file header.
  
  The meaning of this field depends on the `os_abi` value.

- **`e_type`**: `u16`

  The object file type in the file header.
  
  One of the `ET_*` constants.

- **`e_machine`**: `u16`

  The architecture in the file header.
  
  One of the `EM_*` constants.

- **`e_entry`**: `u64`

  Entry point virtual address in the file header.

- **`e_flags`**: `u32`

  The processor-specific flags in the file header.
  
  A combination of the `EF_*` constants.

- **`e_phoff`**: `u64`

  The file offset of the program header table.
  
  Writing will fail if the program header table cannot be placed at this offset.

#### Trait Implementations

##### `impl Debug for Header`

- <span id="header-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Header`

- <span id="header-default"></span>`fn default() -> Header` — [`Header`](#header)

### `SegmentId`

```rust
struct SegmentId(usize);
```

An ID for referring to a segment in [`Segments`](#segments).

#### Trait Implementations

##### `impl Clone for SegmentId`

- <span id="segmentid-clone"></span>`fn clone(&self) -> SegmentId` — [`SegmentId`](#segmentid)

##### `impl Copy for SegmentId`

##### `impl Debug for SegmentId`

- <span id="segmentid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SegmentId`

##### `impl<K> Equivalent for SegmentId`

- <span id="segmentid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Id for SegmentId`

- <span id="segmentid-id-index"></span>`fn index(&self) -> usize`

##### `impl IdPrivate for SegmentId`

- <span id="segmentid-idprivate-new"></span>`fn new(id: usize) -> Self`

##### `impl PartialEq for SegmentId`

- <span id="segmentid-partialeq-eq"></span>`fn eq(&self, other: &SegmentId) -> bool` — [`SegmentId`](#segmentid)

##### `impl StructuralPartialEq for SegmentId`

### `Segment<'data>`

```rust
struct Segment<'data> {
    id: SegmentId,
    pub delete: bool,
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
    pub sections: alloc::vec::Vec<SectionId>,
    marker: core::marker::PhantomData<&'data ()>,
}
```

A segment in [`Segments`](#segments).

This corresponds to [`elf::ProgramHeader32`](../../elf/index.md) or [`elf::ProgramHeader64`](../../elf/index.md).

#### Fields

- **`delete`**: `bool`

  Ignore this segment when writing the ELF file.

- **`p_type`**: `u32`

  The `p_type` field in the ELF program header.
  
  One of the `PT_*` constants.

- **`p_flags`**: `u32`

  The `p_flags` field in the ELF program header.
  
  A combination of the `PF_*` constants.

- **`p_offset`**: `u64`

  The `p_offset` field in the ELF program header.
  
  This is the file offset of the data in the segment. This should
  correspond to the file offset of the sections that are placed in
  this segment. Currently there is no support for section data
  that is not contained in sections.

- **`p_vaddr`**: `u64`

  The `p_vaddr` field in the ELF program header.

- **`p_paddr`**: `u64`

  The `p_paddr` field in the ELF program header.

- **`p_filesz`**: `u64`

  The `p_filesz` field in the ELF program header.

- **`p_memsz`**: `u64`

  The `p_memsz` field in the ELF program header.

- **`p_align`**: `u64`

  The `p_align` field in the ELF program header.

- **`sections`**: `alloc::vec::Vec<SectionId>`

  The sections contained in this segment.

#### Implementations

- <span id="segment-id"></span>`fn id(&self) -> SegmentId` — [`SegmentId`](#segmentid)

  The ID used for referring to this segment.

- <span id="segment-is-load"></span>`fn is_load(&self) -> bool`

  Returns true if the segment type is `PT_LOAD`.

- <span id="segment-contains-offset"></span>`fn contains_offset(&self, offset: u64) -> bool`

  Returns true if the segment contains the given file offset.

- <span id="segment-address-from-offset"></span>`fn address_from_offset(&self, offset: u64) -> u64`

  Return the address corresponding to the given file offset.

  

  This will return a meaningless value if `contains_offset` is false.

- <span id="segment-contains-address"></span>`fn contains_address(&self, address: u64) -> bool`

  Returns true if the segment contains the given address.

- <span id="segment-remove-sections"></span>`fn remove_sections(&mut self)`

  Remove all sections from the segment, and set its size to zero.

- <span id="segment-append-section"></span>`fn append_section(&mut self, section: &mut Section<'_>)` — [`Section`](#section)

  Add a section to the segment.

  

  If this is a [`elf::PT_LOAD`](../../elf/index.md) segment, then the file offset and address of the

  section is changed to be at the end of the segment.

  

  The segment's file and address ranges are extended to include the section.

  This uses the `sh_size` field of the section, not the size of the section data.

  

  The section's id is added to the segment's list of sections.

- <span id="segment-append-section-range"></span>`fn append_section_range(&mut self, section: &Section<'_>)` — [`Section`](#section)

  Extend this segment's file and address ranges to include the given section.

  

  If the segment's `p_memsz` is zero, then this signifies that the segment

  has no file or address range yet. In this case, the segment's file and address

  ranges are set equal to the section. Otherwise, the segment's file and address

  ranges are extended to include the section.

  

  This uses the `sh_size` field of the section, not the size of the section data.

- <span id="segment-recalculate-ranges"></span>`fn recalculate_ranges(&mut self, sections: &Sections<'data>)` — [`Sections`](#sections)

  Recalculate the file and address ranges of the segment.

  

  Resets the segment's file and address ranges to zero, and then

  calls `append_section_range` for each section in the segment.

#### Trait Implementations

##### `impl Debug for Segment<'data>`

- <span id="segment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Item for Segment<'data>`

- <span id="segment-item-type-id"></span>`type Id = SegmentId`

- <span id="segment-item-is-deleted"></span>`fn is_deleted(&self) -> bool`

### `SectionId`

```rust
struct SectionId(usize);
```

An ID for referring to a section in [`Sections`](#sections).

#### Trait Implementations

##### `impl Clone for SectionId`

- <span id="sectionid-clone"></span>`fn clone(&self) -> SectionId` — [`SectionId`](#sectionid)

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- <span id="sectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionId`

##### `impl<K> Equivalent for SectionId`

- <span id="sectionid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Id for SectionId`

- <span id="sectionid-id-index"></span>`fn index(&self) -> usize`

##### `impl IdPrivate for SectionId`

- <span id="sectionid-idprivate-new"></span>`fn new(id: usize) -> Self`

##### `impl PartialEq for SectionId`

- <span id="sectionid-partialeq-eq"></span>`fn eq(&self, other: &SectionId) -> bool` — [`SectionId`](#sectionid)

##### `impl StructuralPartialEq for SectionId`

### `Section<'data>`

```rust
struct Section<'data> {
    id: SectionId,
    pub delete: bool,
    pub name: crate::build::ByteString<'data>,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link_section: Option<SectionId>,
    pub sh_info: u32,
    pub sh_info_section: Option<SectionId>,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
    pub data: SectionData<'data>,
}
```

A section in [`Sections`](#sections).

This corresponds to [`elf::SectionHeader32`](../../elf/index.md) or [`elf::SectionHeader64`](../../elf/index.md).

#### Fields

- **`delete`**: `bool`

  Ignore this section when writing the ELF file.

- **`name`**: `crate::build::ByteString<'data>`

  The name of the section.
  
  This is automatically added to the section header string table,
  and the resulting string table offset is used to set the `sh_name`
  field in the ELF section header.

- **`sh_type`**: `u32`

  The `sh_type` field in the ELF section header.
  
  One of the `SHT_*` constants.

- **`sh_flags`**: `u64`

  The `sh_flags` field in the ELF section header.
  
  A combination of the `SHF_*` constants.

- **`sh_addr`**: `u64`

  The `sh_addr` field in the ELF section header.

- **`sh_offset`**: `u64`

  The `sh_offset` field in the ELF section header.
  
  This is the file offset of the data in the section.
  Writing will fail if the data cannot be placed at this offset.
  
  This is only used for sections that have `SHF_ALLOC` set.
  For other sections, the section data is written at the next available
  offset.

- **`sh_size`**: `u64`

  The `sh_size` field in the ELF section header.
  
  This size is not used when writing. The size of the `data` field is
  used instead.

- **`sh_link_section`**: `Option<SectionId>`

  The ID of the section linked to by the `sh_link` field in the ELF section header.

- **`sh_info`**: `u32`

  The `sh_info` field in the ELF section header.
  
  Only used if `sh_info_section` is `None`.

- **`sh_info_section`**: `Option<SectionId>`

  The ID of the section linked to by the `sh_info` field in the ELF section header.

- **`sh_addralign`**: `u64`

  The `sh_addralign` field in the ELF section header.

- **`sh_entsize`**: `u64`

  The `sh_entsize` field in the ELF section header.

- **`data`**: `SectionData<'data>`

  The section data.

#### Implementations

- <span id="section-id"></span>`fn id(&self) -> SectionId` — [`SectionId`](#sectionid)

  The ID used for referring to this section.

- <span id="section-is-alloc"></span>`fn is_alloc(&self) -> bool`

  Returns true if the section flags include `SHF_ALLOC`.

- <span id="section-p-flags"></span>`fn p_flags(&self) -> u32`

  Return the segment permission flags that are equivalent to the section flags.

#### Trait Implementations

##### `impl Debug for Section<'data>`

- <span id="section-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Item for Section<'data>`

- <span id="section-item-type-id"></span>`type Id = SectionId`

- <span id="section-item-is-deleted"></span>`fn is_deleted(&self) -> bool`

### `SymbolId<const DYNAMIC: bool>`

```rust
struct SymbolId<const DYNAMIC: bool>(usize);
```

An ID for referring to a symbol in [`Symbols`](#symbols).

#### Trait Implementations

##### `impl Clone for SymbolId<DYNAMIC>`

- <span id="symbolid-clone"></span>`fn clone(&self) -> SymbolId<DYNAMIC>` — [`SymbolId`](#symbolid)

##### `impl Copy for SymbolId<DYNAMIC>`

##### `impl Debug for SymbolId<DYNAMIC>`

- <span id="symbolid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolId<DYNAMIC>`

##### `impl<K> Equivalent for SymbolId<DYNAMIC>`

- <span id="symbolid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Id for SymbolId<DYNAMIC>`

- <span id="symbolid-id-index"></span>`fn index(&self) -> usize`

##### `impl IdPrivate for SymbolId<DYNAMIC>`

- <span id="symbolid-idprivate-new"></span>`fn new(id: usize) -> Self`

##### `impl PartialEq for SymbolId<DYNAMIC>`

- <span id="symbolid-partialeq-eq"></span>`fn eq(&self, other: &SymbolId<DYNAMIC>) -> bool` — [`SymbolId`](#symbolid)

##### `impl StructuralPartialEq for SymbolId<DYNAMIC>`

### `Symbol<'data, const DYNAMIC: bool>`

```rust
struct Symbol<'data, const DYNAMIC: bool> {
    id: SymbolId<DYNAMIC>,
    pub delete: bool,
    pub name: crate::build::ByteString<'data>,
    pub section: Option<SectionId>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
    pub version: VersionId,
    pub version_hidden: bool,
}
```

A symbol in [`Symbols`](#symbols).

This corresponds to [`elf::Sym32`](../../elf/index.md) or [`elf::Sym64`](../../elf/index.md).

#### Fields

- **`delete`**: `bool`

  Ignore this symbol when writing the ELF file.

- **`name`**: `crate::build::ByteString<'data>`

  The name of the symbol.

- **`section`**: `Option<SectionId>`

  The section referenced by the symbol.
  
  Used to set the `st_shndx` field in the ELF symbol.

- **`st_info`**: `u8`

  The `st_info` field in the ELF symbol.

- **`st_other`**: `u8`

  The `st_other` field in the ELF symbol.

- **`st_shndx`**: `u16`

  The `st_shndx` field in the ELF symbol.
  
  Only used if `Self::section` is `None`.

- **`st_value`**: `u64`

  The `st_value` field in the ELF symbol.

- **`st_size`**: `u64`

  The `st_size` field in the ELF symbol.

- **`version`**: `VersionId`

  GNU version for dynamic symbols.

- **`version_hidden`**: `bool`

  Set the [`elf::VERSYM_HIDDEN`](../../elf/index.md) flag for this symbol.

#### Implementations

- <span id="symbol-id"></span>`fn id(&self) -> SymbolId<DYNAMIC>` — [`SymbolId`](#symbolid)

  The ID used for referring to this symbol.

- <span id="symbol-st-bind"></span>`fn st_bind(&self) -> u8`

  Get the `st_bind` component of the `st_info` field.

- <span id="symbol-st-type"></span>`fn st_type(&self) -> u8`

  Get the `st_type` component of the `st_info` field.

- <span id="symbol-set-st-info"></span>`fn set_st_info(&mut self, st_bind: u8, st_type: u8)`

  Set the `st_info` field given the `st_bind` and `st_type` components.

#### Trait Implementations

##### `impl Debug for Symbol<'data, DYNAMIC>`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Item for Symbol<'data, DYNAMIC>`

- <span id="symbol-item-type-id"></span>`type Id = SymbolId<DYNAMIC>`

- <span id="symbol-item-is-deleted"></span>`fn is_deleted(&self) -> bool`

### `Relocation<const DYNAMIC: bool>`

```rust
struct Relocation<const DYNAMIC: bool> {
    pub r_offset: u64,
    pub symbol: Option<SymbolId<DYNAMIC>>,
    pub r_type: u32,
    pub r_addend: i64,
}
```

A relocation stored in a [`Section`](#section).

This corresponds to [`elf::Rel32`](../../elf/index.md), [`elf::Rela32`](../../elf/index.md), [`elf::Rel64`](../../elf/index.md) or [`elf::Rela64`](../../elf/index.md).

#### Fields

- **`r_offset`**: `u64`

  The `r_offset` field in the ELF relocation.

- **`symbol`**: `Option<SymbolId<DYNAMIC>>`

  The symbol referenced by the ELF relocation.

- **`r_type`**: `u32`

  The `r_type` field in the ELF relocation.

- **`r_addend`**: `i64`

  The `r_addend` field in the ELF relocation.
  
  Only used if the section type is `SHT_RELA`.

#### Trait Implementations

##### `impl Clone for Relocation<DYNAMIC>`

- <span id="relocation-clone"></span>`fn clone(&self) -> Relocation<DYNAMIC>` — [`Relocation`](#relocation)

##### `impl Copy for Relocation<DYNAMIC>`

##### `impl Debug for Relocation<DYNAMIC>`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Relocation<DYNAMIC>`

##### `impl<K> Equivalent for Relocation<DYNAMIC>`

- <span id="relocation-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Relocation<DYNAMIC>`

- <span id="relocation-partialeq-eq"></span>`fn eq(&self, other: &Relocation<DYNAMIC>) -> bool` — [`Relocation`](#relocation)

##### `impl StructuralPartialEq for Relocation<DYNAMIC>`

### `VersionFileId`

```rust
struct VersionFileId(usize);
```

An ID for referring to a filename in [`VersionFiles`](#versionfiles).

#### Trait Implementations

##### `impl Clone for VersionFileId`

- <span id="versionfileid-clone"></span>`fn clone(&self) -> VersionFileId` — [`VersionFileId`](#versionfileid)

##### `impl Copy for VersionFileId`

##### `impl Debug for VersionFileId`

- <span id="versionfileid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for VersionFileId`

##### `impl<K> Equivalent for VersionFileId`

- <span id="versionfileid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Id for VersionFileId`

- <span id="versionfileid-id-index"></span>`fn index(&self) -> usize`

##### `impl IdPrivate for VersionFileId`

- <span id="versionfileid-idprivate-new"></span>`fn new(id: usize) -> Self`

##### `impl PartialEq for VersionFileId`

- <span id="versionfileid-partialeq-eq"></span>`fn eq(&self, other: &VersionFileId) -> bool` — [`VersionFileId`](#versionfileid)

##### `impl StructuralPartialEq for VersionFileId`

### `VersionFile<'data>`

```rust
struct VersionFile<'data> {
    id: VersionFileId,
    pub delete: bool,
    pub name: crate::build::ByteString<'data>,
}
```

A filename used for GNU versioning.

Stored in [`VersionFiles`](#versionfiles).

#### Fields

- **`delete`**: `bool`

  Ignore this file when writing the ELF file.

- **`name`**: `crate::build::ByteString<'data>`

  The filename.

#### Implementations

- <span id="versionfile-id"></span>`fn id(&self) -> VersionFileId` — [`VersionFileId`](#versionfileid)

  The ID used for referring to this filename.

#### Trait Implementations

##### `impl Debug for VersionFile<'data>`

- <span id="versionfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Item for VersionFile<'data>`

- <span id="versionfile-item-type-id"></span>`type Id = VersionFileId`

- <span id="versionfile-item-is-deleted"></span>`fn is_deleted(&self) -> bool`

### `VersionId`

```rust
struct VersionId(usize);
```

An ID for referring to a version in [`Versions`](#versions).

#### Implementations

- <span id="versionid-is-special"></span>`fn is_special(&self) -> bool`

  Return `True` if this is a special version that does not exist in the version table.

- <span id="versionid-local"></span>`fn local() -> Self`

  Return the ID for a version index of [`elf::VER_NDX_LOCAL`](../../elf/index.md).

- <span id="versionid-global"></span>`fn global() -> Self`

  Return the ID for a version index of [`elf::VER_NDX_GLOBAL`](../../elf/index.md).

#### Trait Implementations

##### `impl Clone for VersionId`

- <span id="versionid-clone"></span>`fn clone(&self) -> VersionId` — [`VersionId`](#versionid)

##### `impl Copy for VersionId`

##### `impl Debug for VersionId`

- <span id="versionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for VersionId`

##### `impl<K> Equivalent for VersionId`

- <span id="versionid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Id for VersionId`

- <span id="versionid-id-index"></span>`fn index(&self) -> usize`

##### `impl IdPrivate for VersionId`

- <span id="versionid-idprivate-new"></span>`fn new(id: usize) -> Self`

##### `impl PartialEq for VersionId`

- <span id="versionid-partialeq-eq"></span>`fn eq(&self, other: &VersionId) -> bool` — [`VersionId`](#versionid)

##### `impl StructuralPartialEq for VersionId`

### `Version<'data>`

```rust
struct Version<'data> {
    id: VersionId,
    pub data: VersionData<'data>,
    pub delete: bool,
}
```

A version for a symbol.

#### Fields

- **`data`**: `VersionData<'data>`

  The data for this version.

- **`delete`**: `bool`

  Ignore this version when writing the ELF file.

#### Implementations

- <span id="version-id"></span>`fn id(&self) -> VersionId` — [`VersionId`](#versionid)

  The ID used for referring to this version.

#### Trait Implementations

##### `impl Debug for Version<'data>`

- <span id="version-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Item for Version<'data>`

- <span id="version-item-type-id"></span>`type Id = VersionId`

- <span id="version-item-is-deleted"></span>`fn is_deleted(&self) -> bool`

### `VersionDef<'data>`

```rust
struct VersionDef<'data> {
    pub names: alloc::vec::Vec<crate::build::ByteString<'data>>,
    pub flags: u16,
}
```

A GNU version definition.

#### Fields

- **`names`**: `alloc::vec::Vec<crate::build::ByteString<'data>>`

  The names for the version.
  
  This usually has two elements. The first element is the name of this
  version, and the second element is the name of the previous version
  in the tree of versions.

- **`flags`**: `u16`

  The version flags.
  
  A combination of the `VER_FLG_*` constants.

#### Implementations

- <span id="versiondef-is-shared"></span>`fn is_shared(&self, index: usize, base: Option<&ByteString<'_>>) -> bool` — [`ByteString`](../bytes/index.md#bytestring)

  Optimise for the common case where the first version is the same as the base version.

#### Trait Implementations

##### `impl Debug for VersionDef<'data>`

- <span id="versiondef-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `VersionNeed<'data>`

```rust
struct VersionNeed<'data> {
    pub file: VersionFileId,
    pub name: crate::build::ByteString<'data>,
    pub flags: u16,
}
```

A GNU version dependency.

#### Fields

- **`file`**: `VersionFileId`

  The filename of the library providing this version.

- **`name`**: `crate::build::ByteString<'data>`

  The name of the version.

- **`flags`**: `u16`

  The version flags.
  
  A combination of the `VER_FLG_*` constants.

#### Trait Implementations

##### `impl Debug for VersionNeed<'data>`

- <span id="versionneed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributesSection<'data>`

```rust
struct AttributesSection<'data> {
    pub subsections: alloc::vec::Vec<AttributesSubsection<'data>>,
}
```

The contents of an attributes section.

#### Fields

- **`subsections`**: `alloc::vec::Vec<AttributesSubsection<'data>>`

  The subsections.

#### Implementations

- <span id="attributessection-new"></span>`fn new() -> Self`

  Create a new attributes section.

#### Trait Implementations

##### `impl Clone for AttributesSection<'data>`

- <span id="attributessection-clone"></span>`fn clone(&self) -> AttributesSection<'data>` — [`AttributesSection`](#attributessection)

##### `impl Debug for AttributesSection<'data>`

- <span id="attributessection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AttributesSection<'data>`

- <span id="attributessection-default"></span>`fn default() -> AttributesSection<'data>` — [`AttributesSection`](#attributessection)

### `AttributesSubsection<'data>`

```rust
struct AttributesSubsection<'data> {
    pub vendor: crate::build::ByteString<'data>,
    pub subsubsections: alloc::vec::Vec<AttributesSubsubsection<'data>>,
}
```

A subsection of an attributes section.

#### Fields

- **`vendor`**: `crate::build::ByteString<'data>`

  The vendor namespace for these attributes.

- **`subsubsections`**: `alloc::vec::Vec<AttributesSubsubsection<'data>>`

  The sub-subsections.

#### Implementations

- <span id="attributessubsection-new"></span>`fn new(vendor: ByteString<'data>) -> Self` — [`ByteString`](../bytes/index.md#bytestring)

  Create a new subsection.

#### Trait Implementations

##### `impl Clone for AttributesSubsection<'data>`

- <span id="attributessubsection-clone"></span>`fn clone(&self) -> AttributesSubsection<'data>` — [`AttributesSubsection`](#attributessubsection)

##### `impl Debug for AttributesSubsection<'data>`

- <span id="attributessubsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributesSubsubsection<'data>`

```rust
struct AttributesSubsubsection<'data> {
    pub tag: AttributeTag,
    pub data: crate::build::Bytes<'data>,
}
```

A sub-subsection in an attributes section.

#### Fields

- **`tag`**: `AttributeTag`

  The sub-subsection tag.

- **`data`**: `crate::build::Bytes<'data>`

  The data containing the attributes.

#### Trait Implementations

##### `impl Clone for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-clone"></span>`fn clone(&self) -> AttributesSubsubsection<'data>` — [`AttributesSubsubsection`](#attributessubsubsection)

##### `impl Debug for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `SectionData<'data>`

```rust
enum SectionData<'data> {
    Data(crate::build::Bytes<'data>),
    UninitializedData(u64),
    Relocation(alloc::vec::Vec<Relocation>),
    DynamicRelocation(alloc::vec::Vec<DynamicRelocation>),
    Note(crate::build::Bytes<'data>),
    Dynamic(alloc::vec::Vec<Dynamic<'data>>),
    Attributes(AttributesSection<'data>),
    SectionString,
    Symbol,
    SymbolSectionIndex,
    String,
    DynamicSymbol,
    DynamicString,
    Hash,
    GnuHash,
    GnuVersym,
    GnuVerdef,
    GnuVerneed,
}
```

The data for a [`Section`](#section).

#### Variants

- **`Data`**

  The section contains the given raw data bytes.

- **`UninitializedData`**

  The section contains uninitialised data bytes of the given length.

- **`Relocation`**

  The section contains relocations.

- **`DynamicRelocation`**

  The section contains dynamic relocations.

- **`Note`**

  The section contains notes.

- **`Dynamic`**

  The section contains dynamic entries.

- **`Attributes`**

  The section contains attributes.
  
  This may be GNU attributes or other vendor-specific attributes.

- **`SectionString`**

  The section contains the strings for the section headers.

- **`Symbol`**

  The section contains the symbol table.

- **`SymbolSectionIndex`**

  The section contains the extended section index for the symbol table.

- **`String`**

  The section contains the strings for symbol table.

- **`DynamicSymbol`**

  The section contains the dynamic symbol table.

- **`DynamicString`**

  The section contains the dynamic string table.

- **`Hash`**

  The section contains the hash table.

- **`GnuHash`**

  The section contains the GNU hash table.

- **`GnuVersym`**

  The section contains the GNU symbol versions.

- **`GnuVerdef`**

  The section contains the GNU version definitions.

- **`GnuVerneed`**

  The section contains the GNU version dependencies.

#### Trait Implementations

##### `impl Clone for SectionData<'data>`

- <span id="sectiondata-clone"></span>`fn clone(&self) -> SectionData<'data>` — [`SectionData`](#sectiondata)

##### `impl Debug for SectionData<'data>`

- <span id="sectiondata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Dynamic<'data>`

```rust
enum Dynamic<'data> {
    Auto {
        tag: u32,
    },
    Integer {
        tag: u32,
        val: u64,
    },
    String {
        tag: u32,
        val: crate::build::ByteString<'data>,
    },
}
```

An entry in the dynamic section.

This corresponds to [`elf::Dyn32`](../../elf/index.md) or [`elf::Dyn64`](../../elf/index.md).

#### Variants

- **`Auto`**

  The value is an automatically generated integer.
  
  Writing will fail if the value cannot be automatically generated.

- **`Integer`**

  The value is an integer.

- **`String`**

  The value is a string.

#### Implementations

- <span id="dynamic-tag"></span>`fn tag(&self) -> u32`

  The `d_tag` field in the dynamic entry.

  

  One of the `DT_*` values.

#### Trait Implementations

##### `impl Clone for Dynamic<'data>`

- <span id="dynamic-clone"></span>`fn clone(&self) -> Dynamic<'data>` — [`Dynamic`](#dynamic)

##### `impl Debug for Dynamic<'data>`

- <span id="dynamic-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Dynamic<'data>`

##### `impl<K> Equivalent for Dynamic<'data>`

- <span id="dynamic-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Dynamic<'data>`

- <span id="dynamic-partialeq-eq"></span>`fn eq(&self, other: &Dynamic<'data>) -> bool` — [`Dynamic`](#dynamic)

##### `impl StructuralPartialEq for Dynamic<'data>`

### `VersionData<'data>`

```rust
enum VersionData<'data> {
    Def(VersionDef<'data>),
    Need(VersionNeed<'data>),
}
```

The data for a version for a symbol.

#### Variants

- **`Def`**

  The version for a defined symbol.

- **`Need`**

  The version for an undefined symbol.

#### Trait Implementations

##### `impl Debug for VersionData<'data>`

- <span id="versiondata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributeTag`

```rust
enum AttributeTag {
    File,
    Section(alloc::vec::Vec<SectionId>),
    Symbol(alloc::vec::Vec<SymbolId>),
}
```

The tag for a sub-subsection in an attributes section.

#### Variants

- **`File`**

  The attributes apply to the whole file.
  
  Correspeonds to [`elf::Tag_File`](../../elf/index.md).

- **`Section`**

  The attributes apply to the given sections.
  
  Correspeonds to [`elf::Tag_Section`](../../elf/index.md).

- **`Symbol`**

  The attributes apply to the given symbols.
  
  Correspeonds to [`elf::Tag_Symbol`](../../elf/index.md).

#### Implementations

- <span id="attributetag-tag"></span>`fn tag(&self) -> u8`

  Return the corresponding `elf::Tag_*` value for this tag.

#### Trait Implementations

##### `impl Clone for AttributeTag`

- <span id="attributetag-clone"></span>`fn clone(&self) -> AttributeTag` — [`AttributeTag`](#attributetag)

##### `impl Debug for AttributeTag`

- <span id="attributetag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AttributeTag`

##### `impl<K> Equivalent for AttributeTag`

- <span id="attributetag-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for AttributeTag`

- <span id="attributetag-partialeq-eq"></span>`fn eq(&self, other: &AttributeTag) -> bool` — [`AttributeTag`](#attributetag)

##### `impl StructuralPartialEq for AttributeTag`

## Type Aliases

### `Segments<'data>`

```rust
type Segments<'data> = crate::build::Table<Segment<'data>>;
```

A segment table.

### `Sections<'data>`

```rust
type Sections<'data> = crate::build::Table<Section<'data>>;
```

A section table.

### `Symbols<'data, const DYNAMIC: bool>`

```rust
type Symbols<'data, const DYNAMIC: bool> = crate::build::Table<Symbol<'data, DYNAMIC>>;
```

A symbol table.

### `DynamicSymbolId`

```rust
type DynamicSymbolId = SymbolId<true>;
```

A dynamic symbol ID.

### `DynamicSymbol<'data>`

```rust
type DynamicSymbol<'data> = Symbol<'data, true>;
```

A dynamic symbol.

### `DynamicSymbols<'data>`

```rust
type DynamicSymbols<'data> = Symbols<'data, true>;
```

A dynamic symbol table.

### `DynamicRelocation`

```rust
type DynamicRelocation = Relocation<true>;
```

A dynamic relocation.

### `VersionFiles<'data>`

```rust
type VersionFiles<'data> = crate::build::Table<VersionFile<'data>>;
```

A table of filenames used for GNU versioning.

### `Versions<'data>`

```rust
type Versions<'data> = crate::build::Table<Version<'data>>;
```

A table of versions that are referenced by symbols.

## Constants

### `VERSION_ID_BASE`
```rust
const VERSION_ID_BASE: usize = 2usize;
```

