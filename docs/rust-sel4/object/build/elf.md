**object > build > elf**

# Module: build::elf

## Contents

**Structs**

- [`AttributesSection`](#attributessection) - The contents of an attributes section.
- [`AttributesSubsection`](#attributessubsection) - A subsection of an attributes section.
- [`AttributesSubsubsection`](#attributessubsubsection) - A sub-subsection in an attributes section.
- [`Builder`](#builder) - A builder for reading, modifying, and then writing ELF files.
- [`Header`](#header) - ELF file header.
- [`Relocation`](#relocation) - A relocation stored in a [`Section`].
- [`Section`](#section) - A section in [`Sections`].
- [`SectionId`](#sectionid) - An ID for referring to a section in [`Sections`].
- [`Segment`](#segment) - A segment in [`Segments`].
- [`SegmentId`](#segmentid) - An ID for referring to a segment in [`Segments`].
- [`Symbol`](#symbol) - A symbol in [`Symbols`].
- [`SymbolId`](#symbolid) - An ID for referring to a symbol in [`Symbols`].
- [`Version`](#version) - A version for a symbol.
- [`VersionDef`](#versiondef) - A GNU version definition.
- [`VersionFile`](#versionfile) - A filename used for GNU versioning.
- [`VersionFileId`](#versionfileid) - An ID for referring to a filename in [`VersionFiles`].
- [`VersionId`](#versionid) - An ID for referring to a version in [`Versions`].
- [`VersionNeed`](#versionneed) - A GNU version dependency.

**Enums**

- [`AttributeTag`](#attributetag) - The tag for a sub-subsection in an attributes section.
- [`Dynamic`](#dynamic) - An entry in the dynamic section.
- [`SectionData`](#sectiondata) - The data for a [`Section`].
- [`VersionData`](#versiondata) - The data for a version for a symbol.

**Type Aliases**

- [`DynamicRelocation`](#dynamicrelocation) - A dynamic relocation.
- [`DynamicSymbol`](#dynamicsymbol) - A dynamic symbol.
- [`DynamicSymbolId`](#dynamicsymbolid) - A dynamic symbol ID.
- [`DynamicSymbols`](#dynamicsymbols) - A dynamic symbol table.
- [`Sections`](#sections) - A section table.
- [`Segments`](#segments) - A segment table.
- [`Symbols`](#symbols) - A symbol table.
- [`VersionFiles`](#versionfiles) - A table of filenames used for GNU versioning.
- [`Versions`](#versions) - A table of versions that are referenced by symbols.

---

## object::build::elf::AttributeTag

*Enum*

The tag for a sub-subsection in an attributes section.

**Variants:**
- `File` - The attributes apply to the whole file.
- `Section(alloc::vec::Vec<SectionId>)` - The attributes apply to the given sections.
- `Symbol(alloc::vec::Vec<SymbolId>)` - The attributes apply to the given symbols.

**Methods:**

- `fn tag(self: &Self) -> u8` - Return the corresponding `elf::Tag_*` value for this tag.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AttributeTag`
- **PartialEq**
  - `fn eq(self: &Self, other: &AttributeTag) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::AttributesSection

*Struct*

The contents of an attributes section.

**Generic Parameters:**
- 'data

**Fields:**
- `subsections: alloc::vec::Vec<AttributesSubsection<'data>>` - The subsections.

**Methods:**

- `fn new() -> Self` - Create a new attributes section.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AttributesSection<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> AttributesSection<'data>`



## object::build::elf::AttributesSubsection

*Struct*

A subsection of an attributes section.

**Generic Parameters:**
- 'data

**Fields:**
- `vendor: crate::build::ByteString<'data>` - The vendor namespace for these attributes.
- `subsubsections: alloc::vec::Vec<AttributesSubsubsection<'data>>` - The sub-subsections.

**Methods:**

- `fn new(vendor: ByteString<'data>) -> Self` - Create a new subsection.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AttributesSubsection<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::AttributesSubsubsection

*Struct*

A sub-subsection in an attributes section.

**Generic Parameters:**
- 'data

**Fields:**
- `tag: AttributeTag` - The sub-subsection tag.
- `data: crate::build::Bytes<'data>` - The data containing the attributes.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributesSubsubsection<'data>`



## object::build::elf::Builder

*Struct*

A builder for reading, modifying, and then writing ELF files.

Public fields are available for modifying the values that will be written.
Methods are available to add elements to tables, and elements can be deleted
from tables by setting the `delete` field in the element.

**Generic Parameters:**
- 'data

**Fields:**
- `endian: crate::Endianness` - The endianness.
- `is_64: bool` - Whether file is 64-bit.
- `load_align: u64` - The alignment of [`elf::PT_LOAD`] segments.
- `header: Header` - The file header.
- `segments: Segments<'data>` - The segment table.
- `sections: Sections<'data>` - The section table.
- `symbols: Symbols<'data>` - The symbol table.
- `dynamic_symbols: DynamicSymbols<'data>` - The dynamic symbol table.
- `version_base: Option<crate::build::ByteString<'data>>` - The base version for the GNU version definitions.
- `versions: Versions<'data>` - The GNU version definitions and dependencies.
- `version_files: VersionFiles<'data>` - The filenames used in the GNU version definitions.
- `hash_bucket_count: u32` - The bucket count parameter for the hash table.
- `gnu_hash_bloom_shift: u32` - The bloom shift parameter for the GNU hash table.
- `gnu_hash_bloom_count: u32` - The bloom count parameter for the GNU hash table.
- `gnu_hash_bucket_count: u32` - The bucket count parameter for the GNU hash table.

**Methods:**

- `fn new(endian: Endianness, is_64: bool) -> Self` - Create a new ELF builder.
- `fn read<R>(data: R) -> Result<Self>` - Read the ELF file from file data.
- `fn read32<R>(data: R) -> Result<Self>` - Read a 32-bit ELF file from file data.
- `fn read64<R>(data: R) -> Result<Self>` - Read a 64-bit ELF file from file data.
- `fn write(self: Self, buffer: & mut dyn write::WritableBuffer) -> Result<()>` - Write the ELF file to the buffer.
- `fn delete_orphans(self: & mut Self)` - Delete segments, symbols, relocations, and dynamics that refer
- `fn delete_orphan_segments(self: & mut Self)` - Set the delete flag for segments that only refer to deleted sections.
- `fn delete_orphan_symbols(self: & mut Self)` - Set the delete flag for symbols that refer to deleted sections.
- `fn delete_orphan_relocations(self: & mut Self)` - Delete relocations that refer to deleted symbols.
- `fn delete_orphan_dynamics(self: & mut Self)` - Delete dynamic entries that refer to deleted sections.
- `fn delete_unused_versions(self: & mut Self)` - Delete unused GNU version entries.
- `fn class(self: &Self) -> write::elf::Class` - Return the ELF file class that will be written.
- `fn file_header_size(self: &Self) -> usize` - Calculate the size of the file header.
- `fn program_headers_size(self: &Self) -> usize` - Calculate the size of the program headers.
- `fn dynamic_symbol_size(self: &Self) -> usize` - Calculate the size of the dynamic symbol table.
- `fn dynamic_string_size(self: &Self) -> usize` - Calculate the size of the dynamic string table.
- `fn hash_size(self: &Self) -> usize` - Calculate the size of the hash table.
- `fn gnu_hash_size(self: &Self) -> usize` - Calculate the size of the GNU hash table.
- `fn gnu_versym_size(self: &Self) -> usize` - Calculate the size of the GNU symbol version section.
- `fn gnu_verdef_size(self: &Self) -> usize` - Calculate the size of the GNU version definition section.
- `fn gnu_verneed_size(self: &Self) -> usize` - Calculate the size of the GNU version dependency section.
- `fn section_size(self: &Self, section: &Section) -> usize` - Calculate the memory size of a section.
- `fn set_section_sizes(self: & mut Self)` - Set the `sh_size` field for every allocated section.
- `fn dynamic_section(self: &Self) -> Option<SectionId>` - Find the section containing the dynamic table.
- `fn dynamic_data(self: &Self) -> Option<&[Dynamic<'data>]>` - Find the dynamic table entries.
- `fn dynamic_data_mut(self: & mut Self) -> Option<& mut Vec<Dynamic<'data>>>` - Find the dynamic table entries.
- `fn interp_section(self: &Self) -> Option<SectionId>` - Find the section containing the interpreter path.
- `fn interp_data(self: &Self) -> Option<&[u8]>` - Find the interpreter path.
- `fn interp_data_mut(self: & mut Self) -> Option<& mut Bytes<'data>>` - Find the interpreter path.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::Dynamic

*Enum*

An entry in the dynamic section.

This corresponds to [`elf::Dyn32`] or [`elf::Dyn64`].

**Generic Parameters:**
- 'data

**Variants:**
- `Auto{ tag: u32 }` - The value is an automatically generated integer.
- `Integer{ tag: u32, val: u64 }` - The value is an integer.
- `String{ tag: u32, val: crate::build::ByteString<'data> }` - The value is a string.

**Methods:**

- `fn tag(self: &Self) -> u32` - The `d_tag` field in the dynamic entry.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Dynamic<'data>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Dynamic<'data>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::DynamicRelocation

*Type Alias*: `Relocation<true>`

A dynamic relocation.



## object::build::elf::DynamicSymbol

*Type Alias*: `Symbol<'data, true>`

A dynamic symbol.



## object::build::elf::DynamicSymbolId

*Type Alias*: `SymbolId<true>`

A dynamic symbol ID.



## object::build::elf::DynamicSymbols

*Type Alias*: `Symbols<'data, true>`

A dynamic symbol table.



## object::build::elf::Header

*Struct*

ELF file header.

This corresponds to fields in [`elf::FileHeader32`] or [`elf::FileHeader64`].
This only contains the ELF file header fields that can be modified.
The other fields are automatically calculated.

**Fields:**
- `os_abi: u8` - The OS ABI field in the file header.
- `abi_version: u8` - The ABI version field in the file header.
- `e_type: u16` - The object file type in the file header.
- `e_machine: u16` - The architecture in the file header.
- `e_entry: u64` - Entry point virtual address in the file header.
- `e_flags: u32` - The processor-specific flags in the file header.
- `e_phoff: u64` - The file offset of the program header table.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Header`



## object::build::elf::Relocation

*Struct*

A relocation stored in a [`Section`].

This corresponds to [`elf::Rel32`], [`elf::Rela32`], [`elf::Rel64`] or [`elf::Rela64`].

**Generic Parameters:**
- const DYNAMIC

**Fields:**
- `r_offset: u64` - The `r_offset` field in the ELF relocation.
- `symbol: Option<SymbolId<DYNAMIC>>` - The symbol referenced by the ELF relocation.
- `r_type: u32` - The `r_type` field in the ELF relocation.
- `r_addend: i64` - The `r_addend` field in the ELF relocation.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Relocation<DYNAMIC>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Relocation<DYNAMIC>`



## object::build::elf::Section

*Struct*

A section in [`Sections`].

This corresponds to [`elf::SectionHeader32`] or [`elf::SectionHeader64`].

**Generic Parameters:**
- 'data

**Fields:**
- `delete: bool` - Ignore this section when writing the ELF file.
- `name: crate::build::ByteString<'data>` - The name of the section.
- `sh_type: u32` - The `sh_type` field in the ELF section header.
- `sh_flags: u64` - The `sh_flags` field in the ELF section header.
- `sh_addr: u64` - The `sh_addr` field in the ELF section header.
- `sh_offset: u64` - The `sh_offset` field in the ELF section header.
- `sh_size: u64` - The `sh_size` field in the ELF section header.
- `sh_link_section: Option<SectionId>` - The ID of the section linked to by the `sh_link` field in the ELF section header.
- `sh_info: u32` - The `sh_info` field in the ELF section header.
- `sh_info_section: Option<SectionId>` - The ID of the section linked to by the `sh_info` field in the ELF section header.
- `sh_addralign: u64` - The `sh_addralign` field in the ELF section header.
- `sh_entsize: u64` - The `sh_entsize` field in the ELF section header.
- `data: SectionData<'data>` - The section data.

**Methods:**

- `fn id(self: &Self) -> SectionId` - The ID used for referring to this section.
- `fn is_alloc(self: &Self) -> bool` - Returns true if the section flags include `SHF_ALLOC`.
- `fn p_flags(self: &Self) -> u32` - Return the segment permission flags that are equivalent to the section flags.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Item**
  - `fn is_deleted(self: &Self) -> bool`



## object::build::elf::SectionData

*Enum*

The data for a [`Section`].

**Generic Parameters:**
- 'data

**Variants:**
- `Data(crate::build::Bytes<'data>)` - The section contains the given raw data bytes.
- `UninitializedData(u64)` - The section contains uninitialised data bytes of the given length.
- `Relocation(alloc::vec::Vec<Relocation>)` - The section contains relocations.
- `DynamicRelocation(alloc::vec::Vec<DynamicRelocation>)` - The section contains dynamic relocations.
- `Note(crate::build::Bytes<'data>)` - The section contains notes.
- `Dynamic(alloc::vec::Vec<Dynamic<'data>>)` - The section contains dynamic entries.
- `Attributes(AttributesSection<'data>)` - The section contains attributes.
- `SectionString` - The section contains the strings for the section headers.
- `Symbol` - The section contains the symbol table.
- `SymbolSectionIndex` - The section contains the extended section index for the symbol table.
- `String` - The section contains the strings for symbol table.
- `DynamicSymbol` - The section contains the dynamic symbol table.
- `DynamicString` - The section contains the dynamic string table.
- `Hash` - The section contains the hash table.
- `GnuHash` - The section contains the GNU hash table.
- `GnuVersym` - The section contains the GNU symbol versions.
- `GnuVerdef` - The section contains the GNU version definitions.
- `GnuVerneed` - The section contains the GNU version dependencies.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SectionData<'data>`



## object::build::elf::SectionId

*Struct*

An ID for referring to a section in [`Sections`].

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionId`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionId) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Id**
  - `fn index(self: &Self) -> usize`



## object::build::elf::Sections

*Type Alias*: `crate::build::Table<Section<'data>>`

A section table.



## object::build::elf::Segment

*Struct*

A segment in [`Segments`].

This corresponds to [`elf::ProgramHeader32`] or [`elf::ProgramHeader64`].

**Generic Parameters:**
- 'data

**Fields:**
- `delete: bool` - Ignore this segment when writing the ELF file.
- `p_type: u32` - The `p_type` field in the ELF program header.
- `p_flags: u32` - The `p_flags` field in the ELF program header.
- `p_offset: u64` - The `p_offset` field in the ELF program header.
- `p_vaddr: u64` - The `p_vaddr` field in the ELF program header.
- `p_paddr: u64` - The `p_paddr` field in the ELF program header.
- `p_filesz: u64` - The `p_filesz` field in the ELF program header.
- `p_memsz: u64` - The `p_memsz` field in the ELF program header.
- `p_align: u64` - The `p_align` field in the ELF program header.
- `sections: alloc::vec::Vec<SectionId>` - The sections contained in this segment.

**Methods:**

- `fn id(self: &Self) -> SegmentId` - The ID used for referring to this segment.
- `fn is_load(self: &Self) -> bool` - Returns true if the segment type is `PT_LOAD`.
- `fn contains_offset(self: &Self, offset: u64) -> bool` - Returns true if the segment contains the given file offset.
- `fn address_from_offset(self: &Self, offset: u64) -> u64` - Return the address corresponding to the given file offset.
- `fn contains_address(self: &Self, address: u64) -> bool` - Returns true if the segment contains the given address.
- `fn remove_sections(self: & mut Self)` - Remove all sections from the segment, and set its size to zero.
- `fn append_section(self: & mut Self, section: & mut Section)` - Add a section to the segment.
- `fn append_section_range(self: & mut Self, section: &Section)` - Extend this segment's file and address ranges to include the given section.
- `fn recalculate_ranges(self: & mut Self, sections: &Sections<'data>)` - Recalculate the file and address ranges of the segment.

**Trait Implementations:**

- **Item**
  - `fn is_deleted(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::SegmentId

*Struct*

An ID for referring to a segment in [`Segments`].

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SegmentId`
- **Id**
  - `fn index(self: &Self) -> usize`
- **PartialEq**
  - `fn eq(self: &Self, other: &SegmentId) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::build::elf::Segments

*Type Alias*: `crate::build::Table<Segment<'data>>`

A segment table.



## object::build::elf::Symbol

*Struct*

A symbol in [`Symbols`].

This corresponds to [`elf::Sym32`] or [`elf::Sym64`].

**Generic Parameters:**
- 'data
- const DYNAMIC

**Fields:**
- `delete: bool` - Ignore this symbol when writing the ELF file.
- `name: crate::build::ByteString<'data>` - The name of the symbol.
- `section: Option<SectionId>` - The section referenced by the symbol.
- `st_info: u8` - The `st_info` field in the ELF symbol.
- `st_other: u8` - The `st_other` field in the ELF symbol.
- `st_shndx: u16` - The `st_shndx` field in the ELF symbol.
- `st_value: u64` - The `st_value` field in the ELF symbol.
- `st_size: u64` - The `st_size` field in the ELF symbol.
- `version: VersionId` - GNU version for dynamic symbols.
- `version_hidden: bool` - Set the [`elf::VERSYM_HIDDEN`] flag for this symbol.

**Methods:**

- `fn id(self: &Self) -> SymbolId<DYNAMIC>` - The ID used for referring to this symbol.
- `fn st_bind(self: &Self) -> u8` - Get the `st_bind` component of the `st_info` field.
- `fn st_type(self: &Self) -> u8` - Get the `st_type` component of the `st_info` field.
- `fn set_st_info(self: & mut Self, st_bind: u8, st_type: u8)` - Set the `st_info` field given the `st_bind` and `st_type` components.

**Trait Implementations:**

- **Item**
  - `fn is_deleted(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::SymbolId

*Struct*

An ID for referring to a symbol in [`Symbols`].

**Generic Parameters:**
- const DYNAMIC

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Id**
  - `fn index(self: &Self) -> usize`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolId<DYNAMIC>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SymbolId<DYNAMIC>`



## object::build::elf::Symbols

*Type Alias*: `crate::build::Table<Symbol<'data, DYNAMIC>>`

A symbol table.



## object::build::elf::Version

*Struct*

A version for a symbol.

**Generic Parameters:**
- 'data

**Fields:**
- `data: VersionData<'data>` - The data for this version.
- `delete: bool` - Ignore this version when writing the ELF file.

**Methods:**

- `fn id(self: &Self) -> VersionId` - The ID used for referring to this version.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Item**
  - `fn is_deleted(self: &Self) -> bool`



## object::build::elf::VersionData

*Enum*

The data for a version for a symbol.

**Generic Parameters:**
- 'data

**Variants:**
- `Def(VersionDef<'data>)` - The version for a defined symbol.
- `Need(VersionNeed<'data>)` - The version for an undefined symbol.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::VersionDef

*Struct*

A GNU version definition.

**Generic Parameters:**
- 'data

**Fields:**
- `names: alloc::vec::Vec<crate::build::ByteString<'data>>` - The names for the version.
- `flags: u16` - The version flags.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::VersionFile

*Struct*

A filename used for GNU versioning.

Stored in [`VersionFiles`].

**Generic Parameters:**
- 'data

**Fields:**
- `delete: bool` - Ignore this file when writing the ELF file.
- `name: crate::build::ByteString<'data>` - The filename.

**Methods:**

- `fn id(self: &Self) -> VersionFileId` - The ID used for referring to this filename.

**Trait Implementations:**

- **Item**
  - `fn is_deleted(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::VersionFileId

*Struct*

An ID for referring to a filename in [`VersionFiles`].

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VersionFileId`
- **Id**
  - `fn index(self: &Self) -> usize`
- **PartialEq**
  - `fn eq(self: &Self, other: &VersionFileId) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::build::elf::VersionFiles

*Type Alias*: `crate::build::Table<VersionFile<'data>>`

A table of filenames used for GNU versioning.



## object::build::elf::VersionId

*Struct*

An ID for referring to a version in [`Versions`].

**Tuple Struct**: `()`

**Methods:**

- `fn is_special(self: &Self) -> bool` - Return `True` if this is a special version that does not exist in the version table.
- `fn local() -> Self` - Return the ID for a version index of [`elf::VER_NDX_LOCAL`].
- `fn global() -> Self` - Return the ID for a version index of [`elf::VER_NDX_GLOBAL`].

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VersionId) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VersionId`
- **Id**
  - `fn index(self: &Self) -> usize`



## object::build::elf::VersionNeed

*Struct*

A GNU version dependency.

**Generic Parameters:**
- 'data

**Fields:**
- `file: VersionFileId` - The filename of the library providing this version.
- `name: crate::build::ByteString<'data>` - The name of the version.
- `flags: u16` - The version flags.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::elf::Versions

*Type Alias*: `crate::build::Table<Version<'data>>`

A table of versions that are referenced by symbols.



