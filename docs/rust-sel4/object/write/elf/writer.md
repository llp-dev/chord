**object > write > elf > writer**

# Module: write::elf::writer

## Contents

**Structs**

- [`AttributesWriter`](#attributeswriter) - A helper for writing an attributes section.
- [`Class`](#class) - An ELF file class.
- [`FileHeader`](#fileheader) - Native endian version of [`elf::FileHeader64`].
- [`ProgramHeader`](#programheader) - Native endian version of [`elf::ProgramHeader64`].
- [`Rel`](#rel) - Unified native endian version of [`elf::Rel64`] and [`elf::Rela64`].
- [`SectionHeader`](#sectionheader) - Native endian version of [`elf::SectionHeader64`].
- [`SectionIndex`](#sectionindex) - The index of an ELF section.
- [`Sym`](#sym) - Native endian version of [`elf::Sym64`].
- [`SymbolIndex`](#symbolindex) - The index of an ELF symbol.
- [`Verdef`](#verdef) - Information required for writing [`elf::Verdef`].
- [`Vernaux`](#vernaux) - Information required for writing [`elf::Vernaux`].
- [`Verneed`](#verneed) - Information required for writing [`elf::Verneed`].
- [`Writer`](#writer) - A helper for writing ELF files.

---

## object::write::elf::writer::AttributesWriter

*Struct*

A helper for writing an attributes section.

Attributes have a variable length encoding, so it is awkward to write them in a
single pass. Instead, we build the entire attributes section data in memory, using
placeholders for unknown lengths that are filled in later.

**Methods:**

- `fn new(endian: Endianness) -> Self` - Create a new `AttributesWriter` for the given endianness.
- `fn start_subsection(self: & mut Self, vendor: &[u8])` - Start a new subsection with the given vendor name.
- `fn end_subsection(self: & mut Self)` - End the subsection.
- `fn start_subsubsection(self: & mut Self, tag: u8)` - Start a new sub-subsection with the given tag.
- `fn write_subsubsection_index(self: & mut Self, index: u32)` - Write a section or symbol index to the sub-subsection.
- `fn write_subsubsection_indices(self: & mut Self, indices: &[u8])` - Write raw index data to the sub-subsection.
- `fn write_attribute_tag(self: & mut Self, tag: u64)` - Write an attribute tag to the sub-subsection.
- `fn write_attribute_integer(self: & mut Self, value: u64)` - Write an attribute integer value to the sub-subsection.
- `fn write_attribute_string(self: & mut Self, value: &[u8])` - Write an attribute string value to the sub-subsection.
- `fn write_subsubsection_attributes(self: & mut Self, attributes: &[u8])` - Write raw attribute data to the sub-subsection.
- `fn end_subsubsection(self: & mut Self)` - End the sub-subsection.
- `fn data(self: Self) -> Vec<u8>` - Return the completed section data.



## object::write::elf::writer::Class

*Struct*

An ELF file class.

**Fields:**
- `is_64: bool` - Whether the file is 64-bit.

**Methods:**

- `fn align(self: Self) -> usize` - Return the alignment size.
- `fn file_header_size(self: Self) -> usize` - Return the size of the file header.
- `fn program_header_size(self: Self) -> usize` - Return the size of a program header.
- `fn section_header_size(self: Self) -> usize` - Return the size of a section header.
- `fn sym_size(self: Self) -> usize` - Return the size of a symbol.
- `fn rel_size(self: Self, is_rela: bool) -> usize` - Return the size of a relocation entry.
- `fn relr_size(self: Self) -> usize` - Return the size of a relative relocation entry.
- `fn dyn_size(self: Self) -> usize` - Return the size of a dynamic entry.
- `fn hash_size(self: Self, bucket_count: u32, chain_count: u32) -> usize` - Return the size of a hash table.
- `fn gnu_hash_size(self: Self, bloom_count: u32, bucket_count: u32, symbol_count: u32) -> usize` - Return the size of a GNU hash table.
- `fn gnu_versym_size(self: Self, symbol_count: usize) -> usize` - Return the size of a GNU symbol version section.
- `fn gnu_verdef_size(self: Self, verdef_count: usize, verdaux_count: usize) -> usize` - Return the size of a GNU version definition section.
- `fn gnu_verneed_size(self: Self, verneed_count: usize, vernaux_count: usize) -> usize` - Return the size of a GNU version dependency section.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Class`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Class`
- **PartialEq**
  - `fn eq(self: &Self, other: &Class) -> bool`



## object::write::elf::writer::FileHeader

*Struct*

Native endian version of [`elf::FileHeader64`].

**Fields:**
- `os_abi: u8`
- `abi_version: u8`
- `e_type: u16`
- `e_machine: u16`
- `e_entry: u64`
- `e_flags: u32`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FileHeader`



## object::write::elf::writer::ProgramHeader

*Struct*

Native endian version of [`elf::ProgramHeader64`].

**Fields:**
- `p_type: u32`
- `p_flags: u32`
- `p_offset: u64`
- `p_vaddr: u64`
- `p_paddr: u64`
- `p_filesz: u64`
- `p_memsz: u64`
- `p_align: u64`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ProgramHeader`



## object::write::elf::writer::Rel

*Struct*

Unified native endian version of [`elf::Rel64`] and [`elf::Rela64`].

**Fields:**
- `r_offset: u64`
- `r_sym: u32`
- `r_type: u32`
- `r_addend: i64`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Rel`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::elf::writer::SectionHeader

*Struct*

Native endian version of [`elf::SectionHeader64`].

**Fields:**
- `name: Option<crate::write::string::StringId>`
- `sh_type: u32`
- `sh_flags: u64`
- `sh_addr: u64`
- `sh_offset: u64`
- `sh_size: u64`
- `sh_link: u32`
- `sh_info: u32`
- `sh_addralign: u64`
- `sh_entsize: u64`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SectionHeader`



## object::write::elf::writer::SectionIndex

*Struct*

The index of an ELF section.

**Tuple Struct**: `(u32)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> SectionIndex`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionIndex) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SectionIndex) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> SectionIndex`
- **Ord**
  - `fn cmp(self: &Self, other: &SectionIndex) -> $crate::cmp::Ordering`



## object::write::elf::writer::Sym

*Struct*

Native endian version of [`elf::Sym64`].

**Fields:**
- `name: Option<crate::write::string::StringId>`
- `section: Option<SectionIndex>`
- `st_info: u8`
- `st_other: u8`
- `st_shndx: u16`
- `st_value: u64`
- `st_size: u64`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Sym`



## object::write::elf::writer::SymbolIndex

*Struct*

The index of an ELF symbol.

**Tuple Struct**: `(u32)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> SymbolIndex`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolIndex) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SymbolIndex) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> SymbolIndex`
- **Ord**
  - `fn cmp(self: &Self, other: &SymbolIndex) -> $crate::cmp::Ordering`



## object::write::elf::writer::Verdef

*Struct*

Information required for writing [`elf::Verdef`].

**Fields:**
- `version: u16`
- `flags: u16`
- `index: u16`
- `aux_count: u16`
- `name: crate::write::string::StringId` - The name for the first [`elf::Verdaux`] entry.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Verdef`



## object::write::elf::writer::Vernaux

*Struct*

Information required for writing [`elf::Vernaux`].

**Fields:**
- `flags: u16`
- `index: u16`
- `name: crate::write::string::StringId`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Vernaux`



## object::write::elf::writer::Verneed

*Struct*

Information required for writing [`elf::Verneed`].

**Fields:**
- `version: u16`
- `aux_count: u16`
- `file: crate::write::string::StringId`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Verneed`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::elf::writer::Writer

*Struct*

A helper for writing ELF files.

Writing uses a two phase approach. The first phase builds up all of the information
that may need to be known ahead of time:
- build string tables
- reserve section indices
- reserve symbol indices
- reserve file ranges for headers and sections

Some of the information has ordering requirements. For example, strings must be added
to string tables before reserving the file range for the string table. Symbol indices
must be reserved after reserving the section indices they reference. There are debug
asserts to check some of these requirements.

The second phase writes everything out in order. Thus the caller must ensure writing
is in the same order that file ranges were reserved. There are debug asserts to assist
with checking this.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(endian: Endianness, is_64: bool, buffer: &'a  mut dyn WritableBuffer) -> Self` - Create a new `Writer` for the given endianness and ELF class.
- `fn reserved_len(self: &Self) -> usize` - Return the current file length that has been reserved.
- `fn len(self: &Self) -> usize` - Return the current file length that has been written.
- `fn reserve(self: & mut Self, len: usize, align_start: usize) -> usize` - Reserve a file range with the given size and starting alignment.
- `fn write_align(self: & mut Self, align_start: usize)` - Write alignment padding bytes.
- `fn write(self: & mut Self, data: &[u8])` - Write data.
- `fn reserve_until(self: & mut Self, offset: usize)` - Reserve the file range up to the given file offset.
- `fn pad_until(self: & mut Self, offset: usize)` - Write padding up to the given file offset.
- `fn reserve_file_header(self: & mut Self)` - Reserve the range for the file header.
- `fn write_file_header(self: & mut Self, header: &FileHeader) -> Result<()>` - Write the file header.
- `fn reserve_program_headers(self: & mut Self, num: u32)` - Reserve the range for the program headers.
- `fn write_align_program_headers(self: & mut Self)` - Write alignment padding bytes prior to the program headers.
- `fn write_program_header(self: & mut Self, header: &ProgramHeader)` - Write a program header.
- `fn reserve_null_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the null section header.
- `fn reserve_section_index(self: & mut Self) -> SectionIndex` - Reserve a section table index.
- `fn reserve_section_headers(self: & mut Self)` - Reserve the range for the section headers.
- `fn write_null_section_header(self: & mut Self)` - Write the null section header.
- `fn write_section_header(self: & mut Self, section: &SectionHeader)` - Write a section header.
- `fn add_section_name(self: & mut Self, name: &'a [u8]) -> StringId` - Add a section name to the section header string table.
- `fn reserve_shstrtab(self: & mut Self)` - Reserve the range for the section header string table.
- `fn write_shstrtab(self: & mut Self)` - Write the section header string table.
- `fn reserve_shstrtab_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the section header string table.
- `fn reserve_shstrtab_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the section header string table.
- `fn write_shstrtab_section_header(self: & mut Self)` - Write the section header for the section header string table.
- `fn add_string(self: & mut Self, name: &'a [u8]) -> StringId` - Add a string to the string table.
- `fn strtab_needed(self: &Self) -> bool` - Return true if `.strtab` is needed.
- `fn require_strtab(self: & mut Self)` - Require the string table even if no strings were added.
- `fn reserve_strtab(self: & mut Self)` - Reserve the range for the string table.
- `fn write_strtab(self: & mut Self)` - Write the string table.
- `fn reserve_strtab_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the string table.
- `fn reserve_strtab_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the string table.
- `fn write_strtab_section_header(self: & mut Self)` - Write the section header for the string table.
- `fn reserve_null_symbol_index(self: & mut Self) -> SymbolIndex` - Reserve the null symbol table entry.
- `fn reserve_symbol_index(self: & mut Self, section_index: Option<SectionIndex>) -> SymbolIndex` - Reserve a symbol table entry.
- `fn symbol_count(self: &Self) -> u32` - Return the number of reserved symbol table entries.
- `fn reserve_symtab(self: & mut Self)` - Reserve the range for the symbol table.
- `fn write_null_symbol(self: & mut Self)` - Write the null symbol.
- `fn write_symbol(self: & mut Self, sym: &Sym)` - Write a symbol.
- `fn reserve_symtab_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the symbol table.
- `fn reserve_symtab_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the symbol table.
- `fn symtab_index(self: & mut Self) -> SectionIndex` - Return the section index of the symbol table.
- `fn write_symtab_section_header(self: & mut Self, num_local: u32)` - Write the section header for the symbol table.
- `fn symtab_shndx_needed(self: &Self) -> bool` - Return true if `.symtab_shndx` is needed.
- `fn require_symtab_shndx(self: & mut Self)` - Require the extended section indices for the symbol table even
- `fn reserve_symtab_shndx(self: & mut Self)` - Reserve the range for the extended section indices for the symbol table.
- `fn write_symtab_shndx(self: & mut Self)` - Write the extended section indices for the symbol table.
- `fn reserve_symtab_shndx_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the extended section indices symbol table.
- `fn reserve_symtab_shndx_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the extended section indices symbol table.
- `fn write_symtab_shndx_section_header(self: & mut Self)` - Write the section header for the extended section indices for the symbol table.
- `fn add_dynamic_string(self: & mut Self, name: &'a [u8]) -> StringId` - Add a string to the dynamic string table.
- `fn get_dynamic_string(self: &Self, name: &'a [u8]) -> StringId` - Get a string that was previously added to the dynamic string table.
- `fn dynstr_needed(self: &Self) -> bool` - Return true if `.dynstr` is needed.
- `fn require_dynstr(self: & mut Self)` - Require the dynamic string table even if no strings were added.
- `fn reserve_dynstr(self: & mut Self) -> usize` - Reserve the range for the dynamic string table.
- `fn dynstr_len(self: & mut Self) -> usize` - Return the size of the dynamic string table.
- `fn write_dynstr(self: & mut Self)` - Write the dynamic string table.
- `fn reserve_dynstr_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the dynamic string table.
- `fn reserve_dynstr_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the dynamic string table.
- `fn dynstr_index(self: & mut Self) -> SectionIndex` - Return the section index of the dynamic string table.
- `fn write_dynstr_section_header(self: & mut Self, sh_addr: u64)` - Write the section header for the dynamic string table.
- `fn reserve_null_dynamic_symbol_index(self: & mut Self) -> SymbolIndex` - Reserve the null dynamic symbol table entry.
- `fn reserve_dynamic_symbol_index(self: & mut Self) -> SymbolIndex` - Reserve a dynamic symbol table entry.
- `fn dynamic_symbol_count(self: & mut Self) -> u32` - Return the number of reserved dynamic symbols.
- `fn reserve_dynsym(self: & mut Self) -> usize` - Reserve the range for the dynamic symbol table.
- `fn write_null_dynamic_symbol(self: & mut Self)` - Write the null dynamic symbol.
- `fn write_dynamic_symbol(self: & mut Self, sym: &Sym)` - Write a dynamic symbol.
- `fn reserve_dynsym_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the dynamic symbol table.
- `fn reserve_dynsym_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the dynamic symbol table.
- `fn dynsym_index(self: & mut Self) -> SectionIndex` - Return the section index of the dynamic symbol table.
- `fn write_dynsym_section_header(self: & mut Self, sh_addr: u64, num_local: u32)` - Write the section header for the dynamic symbol table.
- `fn reserve_dynamic(self: & mut Self, dynamic_num: usize) -> usize` - Reserve the range for the `.dynamic` section.
- `fn write_align_dynamic(self: & mut Self)` - Write alignment padding bytes prior to the `.dynamic` section.
- `fn reserve_dynamics(self: & mut Self, dynamic_num: usize) -> usize` - Reserve a file range for the given number of dynamic entries.
- `fn write_dynamic_string(self: & mut Self, tag: u32, id: StringId)` - Write a dynamic string entry.
- `fn write_dynamic(self: & mut Self, d_tag: u32, d_val: u64)` - Write a dynamic value entry.
- `fn reserve_dynamic_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the dynamic table.
- `fn write_dynamic_section_header(self: & mut Self, sh_addr: u64)` - Write the section header for the dynamic table.
- `fn reserve_hash(self: & mut Self, bucket_count: u32, chain_count: u32) -> usize` - Reserve a file range for a SysV hash section.
- `fn write_hash<F>(self: & mut Self, bucket_count: u32, chain_count: u32, hash: F)` - Write a SysV hash section.
- `fn reserve_hash_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the SysV hash table.
- `fn reserve_hash_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the SysV hash table.
- `fn write_hash_section_header(self: & mut Self, sh_addr: u64)` - Write the section header for the SysV hash table.
- `fn reserve_gnu_hash(self: & mut Self, bloom_count: u32, bucket_count: u32, symbol_count: u32) -> usize` - Reserve a file range for a GNU hash section.
- `fn write_gnu_hash<F>(self: & mut Self, symbol_base: u32, bloom_shift: u32, bloom_count: u32, bucket_count: u32, symbol_count: u32, hash: F)` - Write a GNU hash section.
- `fn reserve_gnu_hash_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the GNU hash table.
- `fn reserve_gnu_hash_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the GNU hash table.
- `fn write_gnu_hash_section_header(self: & mut Self, sh_addr: u64)` - Write the section header for the GNU hash table.
- `fn reserve_gnu_versym(self: & mut Self) -> usize` - Reserve the range for the `.gnu.version` section.
- `fn write_null_gnu_versym(self: & mut Self)` - Write the null symbol version entry.
- `fn write_gnu_versym(self: & mut Self, versym: u16)` - Write a symbol version entry.
- `fn reserve_gnu_versym_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the `.gnu.version` section.
- `fn reserve_gnu_versym_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the `.gnu.version` section.
- `fn write_gnu_versym_section_header(self: & mut Self, sh_addr: u64)` - Write the section header for the `.gnu.version` section.
- `fn reserve_gnu_verdef(self: & mut Self, verdef_count: usize, verdaux_count: usize) -> usize` - Reserve the range for the `.gnu.version_d` section.
- `fn write_align_gnu_verdef(self: & mut Self)` - Write alignment padding bytes prior to a `.gnu.version_d` section.
- `fn write_gnu_verdef(self: & mut Self, verdef: &Verdef)` - Write a version definition entry.
- `fn write_gnu_verdef_shared(self: & mut Self, verdef: &Verdef)` - Write a version definition entry that shares the names of the next definition.
- `fn write_gnu_verdaux(self: & mut Self, name: StringId)` - Write a version definition auxiliary entry.
- `fn reserve_gnu_verdef_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the `.gnu.version_d` section.
- `fn reserve_gnu_verdef_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the `.gnu.version_d` section.
- `fn write_gnu_verdef_section_header(self: & mut Self, sh_addr: u64)` - Write the section header for the `.gnu.version_d` section.
- `fn reserve_gnu_verneed(self: & mut Self, verneed_count: usize, vernaux_count: usize) -> usize` - Reserve the range for the `.gnu.version_r` section.
- `fn write_align_gnu_verneed(self: & mut Self)` - Write alignment padding bytes prior to a `.gnu.version_r` section.
- `fn write_gnu_verneed(self: & mut Self, verneed: &Verneed)` - Write a version need entry.
- `fn write_gnu_vernaux(self: & mut Self, vernaux: &Vernaux)` - Write a version need auxiliary entry.
- `fn reserve_gnu_verneed_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the `.gnu.version_r` section.
- `fn reserve_gnu_verneed_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the `.gnu.version_r` section.
- `fn write_gnu_verneed_section_header(self: & mut Self, sh_addr: u64)` - Write the section header for the `.gnu.version_r` section.
- `fn reserve_gnu_attributes_section_index(self: & mut Self) -> SectionIndex` - Reserve the section index for the `.gnu.attributes` section.
- `fn reserve_gnu_attributes_section_index_with_name(self: & mut Self, name: &'a [u8]) -> SectionIndex` - Reserve the section index for the `.gnu.attributes` section.
- `fn reserve_gnu_attributes(self: & mut Self, gnu_attributes_size: usize) -> usize` - Reserve the range for the `.gnu.attributes` section.
- `fn write_gnu_attributes_section_header(self: & mut Self)` - Write the section header for the `.gnu.attributes` section.
- `fn write_gnu_attributes(self: & mut Self, data: &[u8])` - Write the data for the `.gnu.attributes` section.
- `fn reserve_relocations(self: & mut Self, count: usize, is_rela: bool) -> usize` - Reserve a file range for the given number of relocations.
- `fn write_align_relocation(self: & mut Self)` - Write alignment padding bytes prior to a relocation section.
- `fn write_relocation(self: & mut Self, is_rela: bool, rel: &Rel)` - Write a relocation.
- `fn write_relocation_section_header(self: & mut Self, name: StringId, section: SectionIndex, symtab: SectionIndex, offset: usize, count: usize, is_rela: bool)` - Write the section header for a relocation section.
- `fn write_relative_relocation_section_header(self: & mut Self, name: StringId, offset: usize, size: usize)` - Write the section header for a relative relocation section.
- `fn reserve_comdat(self: & mut Self, count: usize) -> usize` - Reserve a file range for a COMDAT section.
- `fn write_comdat_header(self: & mut Self)` - Write `GRP_COMDAT` at the start of the COMDAT section.
- `fn write_comdat_entry(self: & mut Self, entry: SectionIndex)` - Write an entry in a COMDAT section.
- `fn write_comdat_section_header(self: & mut Self, name: StringId, symtab: SectionIndex, symbol: SymbolIndex, offset: usize, count: usize)` - Write the section header for a COMDAT section.
- `fn attributes_writer(self: &Self) -> AttributesWriter` - Return a helper for writing an attributes section.



