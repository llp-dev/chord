**object > read > elf > section**

# Module: read::elf::section

## Contents

**Structs**

- [`ElfSection`](#elfsection) - A section in an [`ElfFile`].
- [`ElfSectionIterator`](#elfsectioniterator) - An iterator for the sections in an [`ElfFile`].
- [`SectionTable`](#sectiontable) - The table of section headers in an ELF file.

**Traits**

- [`SectionHeader`](#sectionheader) - A trait for generic access to [`elf::SectionHeader32`] and [`elf::SectionHeader64`].

**Type Aliases**

- [`ElfSection32`](#elfsection32) - A section in an [`ElfFile32`](super::ElfFile32).
- [`ElfSection64`](#elfsection64) - A section in an [`ElfFile64`](super::ElfFile64).
- [`ElfSectionIterator32`](#elfsectioniterator32) - An iterator for the sections in an [`ElfFile32`](super::ElfFile32).
- [`ElfSectionIterator64`](#elfsectioniterator64) - An iterator for the sections in an [`ElfFile64`](super::ElfFile64).

---

## object::read::elf::section::ElfSection

*Struct*

A section in an [`ElfFile`].

Most functionality is provided by the [`ObjectSection`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Methods:**

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` - Get the ELF file containing this section.
- `fn elf_section_header(self: &Self) -> &'data <Elf as >::SectionHeader` - Get the raw ELF section header.
- `fn elf_relocation_section_index(self: &Self) -> read::Result<Option<SectionIndex>>` - Get the index of the relocation section that references this section.
- `fn elf_relocation_section(self: &Self) -> read::Result<Option<&'data <Elf as >::SectionHeader>>` - Get the relocation section that references this section.
- `fn elf_linked_rel(self: &Self) -> read::Result<&'data [<Elf as >::Rel]>` - Get the `Elf::Rel` entries that apply to this section.
- `fn elf_linked_rela(self: &Self) -> read::Result<&'data [<Elf as >::Rela]>` - Get the `Elf::Rela` entries that apply to this section.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ObjectSection**
  - `fn index(self: &Self) -> SectionIndex`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> Option<(u64, u64)>`
  - `fn data(self: &Self) -> read::Result<&'data [u8]>`
  - `fn data_range(self: &Self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>`
  - `fn compressed_file_range(self: &Self) -> read::Result<CompressedFileRange>`
  - `fn compressed_data(self: &Self) -> read::Result<CompressedData<'data>>`
  - `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>`
  - `fn name(self: &Self) -> read::Result<&'data str>`
  - `fn segment_name_bytes(self: &Self) -> read::Result<Option<&[u8]>>`
  - `fn segment_name(self: &Self) -> read::Result<Option<&str>>`
  - `fn kind(self: &Self) -> SectionKind`
  - `fn relocations(self: &Self) -> ElfSectionRelocationIterator<'data, 'file, Elf, R>`
  - `fn relocation_map(self: &Self) -> read::Result<RelocationMap>`
  - `fn flags(self: &Self) -> SectionFlags`



## object::read::elf::section::ElfSection32

*Type Alias*: `ElfSection<'data, 'file, elf::FileHeader32<Endian>, R>`

A section in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::section::ElfSection64

*Type Alias*: `ElfSection<'data, 'file, elf::FileHeader64<Endian>, R>`

A section in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::section::ElfSectionIterator

*Struct*

An iterator for the sections in an [`ElfFile`].

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::section::ElfSectionIterator32

*Type Alias*: `ElfSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>`

An iterator for the sections in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::section::ElfSectionIterator64

*Type Alias*: `ElfSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>`

An iterator for the sections in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::section::SectionHeader

*Trait*

A trait for generic access to [`elf::SectionHeader32`] and [`elf::SectionHeader64`].

**Methods:**

- `Elf`
- `Word`
- `Endian`
- `sh_name`
- `sh_type`
- `sh_flags`
- `sh_addr`
- `sh_offset`
- `sh_size`
- `sh_link`
- `sh_info`
- `sh_addralign`
- `sh_entsize`
- `name`: Parse the section name from the string table.
- `link`: Get the `sh_link` field as a section index.
- `has_info_link`: Return true if the `SHF_INFO_LINK` flag is set.
- `info_link`: Get the `sh_info` field as a section index.
- `file_range`: Return the offset and size of the section in the file.
- `data`: Return the section data.
- `data_as_array`: Return the section data as a slice of the given type.
- `strings`: Return the strings in the section.
- `symbols`: Return the symbols in the section.
- `rel`: Return the `Elf::Rel` entries in the section.
- `rela`: Return the `Elf::Rela` entries in the section.
- `relr`: Return the `Elf::Relr` entries in the section.
- `crel`: Return the `Crel` entries in the section.
- `dynamic`: Return entries in a dynamic section.
- `notes`: Return a note iterator for the section data.
- `group`: Return the contents of a group section.
- `hash_header`: Return the header of a SysV hash section.
- `hash`: Return the contents of a SysV hash section.
- `gnu_hash_header`: Return the header of a GNU hash section.
- `gnu_hash`: Return the contents of a GNU hash section.
- `gnu_versym`: Return the contents of a `SHT_GNU_VERSYM` section.
- `gnu_verdef`: Return an iterator for the entries of a `SHT_GNU_VERDEF` section.
- `gnu_verneed`: Return an iterator for the entries of a `SHT_GNU_VERNEED` section.
- `gnu_attributes`: Return the contents of a `SHT_GNU_ATTRIBUTES` section.
- `attributes`: Parse the contents of the section as attributes.
- `compression`: Parse the compression header if present.



## object::read::elf::section::SectionTable

*Struct*

The table of section headers in an ELF file.

Also includes the string table used for the section names.

Returned by [`FileHeader::sections`].

**Generic Parameters:**
- 'data
- Elf
- R

**Methods:**

- `fn new(sections: &'data [<Elf as >::SectionHeader], strings: StringTable<'data, R>) -> Self` - Create a new section table.
- `fn iter(self: &Self) -> slice::Iter<'data, <Elf as >::SectionHeader>` - Iterate over the section headers.
- `fn enumerate(self: &Self) -> impl Trait` - Iterate over the section headers and their indices.
- `fn is_empty(self: &Self) -> bool` - Return true if the section table is empty.
- `fn len(self: &Self) -> usize` - The number of section headers.
- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data <Elf as >::SectionHeader>` - Get the section header at the given index.
- `fn section_by_name(self: &Self, endian: <Elf as >::Endian, name: &[u8]) -> Option<(SectionIndex, &'data <Elf as >::SectionHeader)>` - Return the section header with the given name.
- `fn section_name(self: &Self, endian: <Elf as >::Endian, section: &<Elf as >::SectionHeader) -> read::Result<&'data [u8]>` - Return the section name for the given section header.
- `fn strings(self: &Self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<StringTable<'data, R>>` - Return the string table at the given section index.
- `fn symbols(self: &Self, endian: <Elf as >::Endian, data: R, sh_type: u32) -> read::Result<SymbolTable<'data, Elf, R>>` - Return the symbol table of the given section type.
- `fn symbol_table_by_index(self: &Self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<SymbolTable<'data, Elf, R>>` - Return the symbol table at the given section index.
- `fn relocation_sections(self: &Self, endian: <Elf as >::Endian, symbol_section: SectionIndex) -> read::Result<RelocationSections>` - Create a mapping from section index to associated relocation sections.
- `fn dynamic(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [<Elf as >::Dyn], SectionIndex)>>` - Return the contents of a dynamic section.
- `fn hash_header(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Elf as >::Endian>>>` - Return the header of a SysV hash section.
- `fn hash(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, Elf>, SectionIndex)>>` - Return the contents of a SysV hash section.
- `fn gnu_hash_header(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Elf as >::Endian>>>` - Return the header of a GNU hash section.
- `fn gnu_hash(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, Elf>, SectionIndex)>>` - Return the contents of a GNU hash section.
- `fn gnu_versym(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Elf as >::Endian>], SectionIndex)>>` - Return the contents of a `SHT_GNU_VERSYM` section.
- `fn gnu_verdef(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, Elf>, SectionIndex)>>` - Return the contents of a `SHT_GNU_VERDEF` section.
- `fn gnu_verneed(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, Elf>, SectionIndex)>>` - Return the contents of a `SHT_GNU_VERNEED` section.
- `fn versions(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<VersionTable<'data, Elf>>>` - Returns the symbol version table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionTable<'data, Elf, R>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



