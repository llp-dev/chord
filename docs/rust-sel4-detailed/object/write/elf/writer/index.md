*[object](../../../index.md) / [write](../../index.md) / [elf](../index.md) / [writer](index.md)*

---

# Module `writer`

Helper for writing ELF files.

## Contents

- [Structs](#structs)
  - [`SectionIndex`](#sectionindex)
  - [`SymbolIndex`](#symbolindex)
  - [`Writer`](#writer)
  - [`AttributesWriter`](#attributeswriter)
  - [`Class`](#class)
  - [`FileHeader`](#fileheader)
  - [`ProgramHeader`](#programheader)
  - [`SectionHeader`](#sectionheader)
  - [`Sym`](#sym)
  - [`Rel`](#rel)
  - [`Verdef`](#verdef)
  - [`Verneed`](#verneed)
  - [`Vernaux`](#vernaux)
- [Constants](#constants)
  - [`ALIGN_SYMTAB_SHNDX`](#align-symtab-shndx)
  - [`ALIGN_HASH`](#align-hash)
  - [`ALIGN_GNU_VERSYM`](#align-gnu-versym)
  - [`ALIGN_GNU_VERDEF`](#align-gnu-verdef)
  - [`ALIGN_GNU_VERNEED`](#align-gnu-verneed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SectionIndex`](#sectionindex) | struct | The index of an ELF section. |
| [`SymbolIndex`](#symbolindex) | struct | The index of an ELF symbol. |
| [`Writer`](#writer) | struct | A helper for writing ELF files. |
| [`AttributesWriter`](#attributeswriter) | struct | A helper for writing an attributes section. |
| [`Class`](#class) | struct | An ELF file class. |
| [`FileHeader`](#fileheader) | struct | Native endian version of [`elf::FileHeader64`]. |
| [`ProgramHeader`](#programheader) | struct | Native endian version of [`elf::ProgramHeader64`]. |
| [`SectionHeader`](#sectionheader) | struct | Native endian version of [`elf::SectionHeader64`]. |
| [`Sym`](#sym) | struct | Native endian version of [`elf::Sym64`]. |
| [`Rel`](#rel) | struct | Unified native endian version of [`elf::Rel64`] and [`elf::Rela64`]. |
| [`Verdef`](#verdef) | struct | Information required for writing [`elf::Verdef`]. |
| [`Verneed`](#verneed) | struct | Information required for writing [`elf::Verneed`]. |
| [`Vernaux`](#vernaux) | struct | Information required for writing [`elf::Vernaux`]. |
| [`ALIGN_SYMTAB_SHNDX`](#align-symtab-shndx) | const |  |
| [`ALIGN_HASH`](#align-hash) | const |  |
| [`ALIGN_GNU_VERSYM`](#align-gnu-versym) | const |  |
| [`ALIGN_GNU_VERDEF`](#align-gnu-verdef) | const |  |
| [`ALIGN_GNU_VERNEED`](#align-gnu-verneed) | const |  |

## Structs

### `SectionIndex`

```rust
struct SectionIndex(u32);
```

The index of an ELF section.

#### Trait Implementations

##### `impl Clone for SectionIndex`

- <span id="sectionindex-clone"></span>`fn clone(&self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

##### `impl<K> Comparable for SectionIndex`

- <span id="sectionindex-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for SectionIndex`

##### `impl Debug for SectionIndex`

- <span id="sectionindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionIndex`

- <span id="sectionindex-default"></span>`fn default() -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

##### `impl Eq for SectionIndex`

##### `impl<K> Equivalent for SectionIndex`

- <span id="sectionindex-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SectionIndex`

- <span id="sectionindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SectionIndex`

- <span id="sectionindex-ord-cmp"></span>`fn cmp(&self, other: &SectionIndex) -> cmp::Ordering` — [`SectionIndex`](../index.md#sectionindex)

##### `impl PartialEq for SectionIndex`

- <span id="sectionindex-partialeq-eq"></span>`fn eq(&self, other: &SectionIndex) -> bool` — [`SectionIndex`](../index.md#sectionindex)

##### `impl PartialOrd for SectionIndex`

- <span id="sectionindex-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SectionIndex) -> option::Option<cmp::Ordering>` — [`SectionIndex`](../index.md#sectionindex)

##### `impl StructuralPartialEq for SectionIndex`

### `SymbolIndex`

```rust
struct SymbolIndex(u32);
```

The index of an ELF symbol.

#### Trait Implementations

##### `impl Clone for SymbolIndex`

- <span id="symbolindex-clone"></span>`fn clone(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

##### `impl<K> Comparable for SymbolIndex`

- <span id="symbolindex-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for SymbolIndex`

##### `impl Debug for SymbolIndex`

- <span id="symbolindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SymbolIndex`

- <span id="symbolindex-default"></span>`fn default() -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

##### `impl Eq for SymbolIndex`

##### `impl<K> Equivalent for SymbolIndex`

- <span id="symbolindex-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolIndex`

- <span id="symbolindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SymbolIndex`

- <span id="symbolindex-ord-cmp"></span>`fn cmp(&self, other: &SymbolIndex) -> cmp::Ordering` — [`SymbolIndex`](../index.md#symbolindex)

##### `impl PartialEq for SymbolIndex`

- <span id="symbolindex-partialeq-eq"></span>`fn eq(&self, other: &SymbolIndex) -> bool` — [`SymbolIndex`](../index.md#symbolindex)

##### `impl PartialOrd for SymbolIndex`

- <span id="symbolindex-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SymbolIndex) -> option::Option<cmp::Ordering>` — [`SymbolIndex`](../index.md#symbolindex)

##### `impl StructuralPartialEq for SymbolIndex`

### `Writer<'a>`

```rust
struct Writer<'a> {
    endian: Endianness,
    is_64: bool,
    is_mips64el: bool,
    elf_align: usize,
    buffer: &'a mut dyn WritableBuffer,
    len: usize,
    segment_offset: usize,
    segment_num: u32,
    section_offset: usize,
    section_num: u32,
    shstrtab: crate::write::string::StringTable<'a>,
    shstrtab_str_id: Option<crate::write::string::StringId>,
    shstrtab_index: SectionIndex,
    shstrtab_offset: usize,
    shstrtab_data: alloc::vec::Vec<u8>,
    need_strtab: bool,
    strtab: crate::write::string::StringTable<'a>,
    strtab_str_id: Option<crate::write::string::StringId>,
    strtab_index: SectionIndex,
    strtab_offset: usize,
    strtab_data: alloc::vec::Vec<u8>,
    symtab_str_id: Option<crate::write::string::StringId>,
    symtab_index: SectionIndex,
    symtab_offset: usize,
    symtab_num: u32,
    need_symtab_shndx: bool,
    symtab_shndx_str_id: Option<crate::write::string::StringId>,
    symtab_shndx_offset: usize,
    symtab_shndx_data: alloc::vec::Vec<u8>,
    need_dynstr: bool,
    dynstr: crate::write::string::StringTable<'a>,
    dynstr_str_id: Option<crate::write::string::StringId>,
    dynstr_index: SectionIndex,
    dynstr_offset: usize,
    dynstr_data: alloc::vec::Vec<u8>,
    dynsym_str_id: Option<crate::write::string::StringId>,
    dynsym_index: SectionIndex,
    dynsym_offset: usize,
    dynsym_num: u32,
    dynamic_str_id: Option<crate::write::string::StringId>,
    dynamic_offset: usize,
    dynamic_num: usize,
    hash_str_id: Option<crate::write::string::StringId>,
    hash_offset: usize,
    hash_size: usize,
    gnu_hash_str_id: Option<crate::write::string::StringId>,
    gnu_hash_offset: usize,
    gnu_hash_size: usize,
    gnu_versym_str_id: Option<crate::write::string::StringId>,
    gnu_versym_offset: usize,
    gnu_verdef_str_id: Option<crate::write::string::StringId>,
    gnu_verdef_offset: usize,
    gnu_verdef_size: usize,
    gnu_verdef_count: u16,
    gnu_verdef_remaining: u16,
    gnu_verdaux_remaining: u16,
    gnu_verneed_str_id: Option<crate::write::string::StringId>,
    gnu_verneed_offset: usize,
    gnu_verneed_size: usize,
    gnu_verneed_count: u16,
    gnu_verneed_remaining: u16,
    gnu_vernaux_remaining: u16,
    gnu_attributes_str_id: Option<crate::write::string::StringId>,
    gnu_attributes_offset: usize,
    gnu_attributes_size: usize,
}
```

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

#### Implementations

- <span id="writer-new"></span>`fn new(endian: Endianness, is_64: bool, buffer: &'a mut dyn WritableBuffer) -> Self` — [`Endianness`](../../../index.md#endianness), [`WritableBuffer`](../../index.md#writablebuffer)

  Create a new `Writer` for the given endianness and ELF class.

- <span id="writer-class"></span>`fn class(&self) -> Class` — [`Class`](../index.md#class)

  Get the file class that will be written.

- <span id="writer-reserved-len"></span>`fn reserved_len(&self) -> usize`

  Return the current file length that has been reserved.

- <span id="writer-len"></span>`fn len(&self) -> usize`

  Return the current file length that has been written.

- <span id="writer-reserve"></span>`fn reserve(&mut self, len: usize, align_start: usize) -> usize`

  Reserve a file range with the given size and starting alignment.

  

  Returns the aligned offset of the start of the range.

  

  `align_start` must be a power of two.

- <span id="writer-write-align"></span>`fn write_align(&mut self, align_start: usize)`

  Write alignment padding bytes.

- <span id="writer-write"></span>`fn write(&mut self, data: &[u8])`

  Write data.

  

  This is typically used to write section data.

- <span id="writer-reserve-until"></span>`fn reserve_until(&mut self, offset: usize)`

  Reserve the file range up to the given file offset.

- <span id="writer-pad-until"></span>`fn pad_until(&mut self, offset: usize)`

  Write padding up to the given file offset.

- <span id="writer-reserve-file-header"></span>`fn reserve_file_header(&mut self)`

  Reserve the range for the file header.

  

  This must be at the start of the file.

- <span id="writer-write-file-header"></span>`fn write_file_header(&mut self, header: &FileHeader) -> Result<()>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../index.md#result)

  Write the file header.

  

  This must be at the start of the file.

  

  Fields that can be derived from known information are automatically set by this function.

- <span id="writer-reserve-program-headers"></span>`fn reserve_program_headers(&mut self, num: u32)`

  Reserve the range for the program headers.

- <span id="writer-write-align-program-headers"></span>`fn write_align_program_headers(&mut self)`

  Write alignment padding bytes prior to the program headers.

- <span id="writer-write-program-header"></span>`fn write_program_header(&mut self, header: &ProgramHeader)` — [`ProgramHeader`](../index.md#programheader)

  Write a program header.

- <span id="writer-reserve-null-section-index"></span>`fn reserve_null_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the null section header.

  

  The null section header is usually automatically reserved,

  but this can be used to force an empty section table.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-reserve-section-index"></span>`fn reserve_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve a section table index.

  

  Automatically also reserves the null section header if required.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-reserve-section-headers"></span>`fn reserve_section_headers(&mut self)`

  Reserve the range for the section headers.

  

  This function does nothing if no sections were reserved.

  This must be called after `Self::reserve_section_index`

  and other functions that reserve section indices.

- <span id="writer-write-null-section-header"></span>`fn write_null_section_header(&mut self)`

  Write the null section header.

  

  This must be the first section header that is written.

  This function does nothing if no sections were reserved.

- <span id="writer-write-section-header"></span>`fn write_section_header(&mut self, section: &SectionHeader)` — [`SectionHeader`](../index.md#sectionheader)

  Write a section header.

- <span id="writer-add-section-name"></span>`fn add_section_name(&mut self, name: &'a [u8]) -> StringId` — [`StringId`](../../string/index.md#stringid)

  Add a section name to the section header string table.

  

  This will be stored in the `.shstrtab` section.

  

  This must be called before `Self::reserve_shstrtab`.

- <span id="writer-reserve-shstrtab"></span>`fn reserve_shstrtab(&mut self)`

  Reserve the range for the section header string table.

  

  This range is used for a section named `.shstrtab`.

  

  This function does nothing if no sections were reserved.

  This must be called after `Self::add_section_name`.

  and other functions that reserve section names and indices.

- <span id="writer-write-shstrtab"></span>`fn write_shstrtab(&mut self)`

  Write the section header string table.

  

  This function does nothing if the section was not reserved.

- <span id="writer-reserve-shstrtab-section-index"></span>`fn reserve_shstrtab_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the section header string table.

  

  This must be called before `Self::reserve_shstrtab`

  and `Self::reserve_section_headers`.

- <span id="writer-reserve-shstrtab-section-index-with-name"></span>`fn reserve_shstrtab_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the section header string table.

  

  This must be called before `Self::reserve_shstrtab`

  and `Self::reserve_section_headers`.

- <span id="writer-write-shstrtab-section-header"></span>`fn write_shstrtab_section_header(&mut self)`

  Write the section header for the section header string table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-add-string"></span>`fn add_string(&mut self, name: &'a [u8]) -> StringId` — [`StringId`](../../string/index.md#stringid)

  Add a string to the string table.

  

  This will be stored in the `.strtab` section.

  

  This must be called before `Self::reserve_strtab`.

- <span id="writer-strtab-needed"></span>`fn strtab_needed(&self) -> bool`

  Return true if `.strtab` is needed.

- <span id="writer-require-strtab"></span>`fn require_strtab(&mut self)`

  Require the string table even if no strings were added.

- <span id="writer-reserve-strtab"></span>`fn reserve_strtab(&mut self)`

  Reserve the range for the string table.

  

  This range is used for a section named `.strtab`.

  

  This function does nothing if a string table is not required.

  This must be called after `Self::add_string`.

- <span id="writer-write-strtab"></span>`fn write_strtab(&mut self)`

  Write the string table.

  

  This function does nothing if the section was not reserved.

- <span id="writer-reserve-strtab-section-index"></span>`fn reserve_strtab_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the string table.

  

  You should check `Self::strtab_needed` before calling this

  unless you have other means of knowing if this section is needed.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-reserve-strtab-section-index-with-name"></span>`fn reserve_strtab_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the string table.

  

  You should check `Self::strtab_needed` before calling this

  unless you have other means of knowing if this section is needed.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-write-strtab-section-header"></span>`fn write_strtab_section_header(&mut self)`

  Write the section header for the string table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-null-symbol-index"></span>`fn reserve_null_symbol_index(&mut self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

  Reserve the null symbol table entry.

  

  This will be stored in the `.symtab` section.

  

  The null symbol table entry is usually automatically reserved,

  but this can be used to force an empty symbol table.

  

  This must be called before `Self::reserve_symtab`.

- <span id="writer-reserve-symbol-index"></span>`fn reserve_symbol_index(&mut self, section_index: Option<SectionIndex>) -> SymbolIndex` — [`SectionIndex`](../index.md#sectionindex), [`SymbolIndex`](../index.md#symbolindex)

  Reserve a symbol table entry.

  

  This will be stored in the `.symtab` section.

  

  `section_index` is used to determine whether `.symtab_shndx` is required.

  

  Automatically also reserves the null symbol if required.

  Callers may assume that the returned indices will be sequential

  starting at 1.

  

  This must be called before `Self::reserve_symtab` and

  `Self::reserve_symtab_shndx`.

- <span id="writer-symbol-count"></span>`fn symbol_count(&self) -> u32`

  Return the number of reserved symbol table entries.

  

  Includes the null symbol.

- <span id="writer-reserve-symtab"></span>`fn reserve_symtab(&mut self)`

  Reserve the range for the symbol table.

  

  This range is used for a section named `.symtab`.

  This function does nothing if no symbols were reserved.

  This must be called after `Self::reserve_symbol_index`.

- <span id="writer-write-null-symbol"></span>`fn write_null_symbol(&mut self)`

  Write the null symbol.

  

  This must be the first symbol that is written.

  This function does nothing if no symbols were reserved.

- <span id="writer-write-symbol"></span>`fn write_symbol(&mut self, sym: &Sym)` — [`Sym`](../index.md#sym)

  Write a symbol.

- <span id="writer-reserve-symtab-section-index"></span>`fn reserve_symtab_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the symbol table.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-reserve-symtab-section-index-with-name"></span>`fn reserve_symtab_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the symbol table.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-symtab-index"></span>`fn symtab_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Return the section index of the symbol table.

- <span id="writer-write-symtab-section-header"></span>`fn write_symtab_section_header(&mut self, num_local: u32)`

  Write the section header for the symbol table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-symtab-shndx-needed"></span>`fn symtab_shndx_needed(&self) -> bool`

  Return true if `.symtab_shndx` is needed.

- <span id="writer-require-symtab-shndx"></span>`fn require_symtab_shndx(&mut self)`

  Require the extended section indices for the symbol table even

  if no section indices are too large.

- <span id="writer-reserve-symtab-shndx"></span>`fn reserve_symtab_shndx(&mut self)`

  Reserve the range for the extended section indices for the symbol table.

  

  This range is used for a section named `.symtab_shndx`.

  This also reserves a section index.

  

  This function does nothing if extended section indices are not needed.

  This must be called after `Self::reserve_symbol_index`.

- <span id="writer-write-symtab-shndx"></span>`fn write_symtab_shndx(&mut self)`

  Write the extended section indices for the symbol table.

  

  This function does nothing if the section was not reserved.

- <span id="writer-reserve-symtab-shndx-section-index"></span>`fn reserve_symtab_shndx_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the extended section indices symbol table.

  

  You should check `Self::symtab_shndx_needed` before calling this

  unless you have other means of knowing if this section is needed.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-reserve-symtab-shndx-section-index-with-name"></span>`fn reserve_symtab_shndx_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the extended section indices symbol table.

  

  You should check `Self::symtab_shndx_needed` before calling this

  unless you have other means of knowing if this section is needed.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-write-symtab-shndx-section-header"></span>`fn write_symtab_shndx_section_header(&mut self)`

  Write the section header for the extended section indices for the symbol table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-add-dynamic-string"></span>`fn add_dynamic_string(&mut self, name: &'a [u8]) -> StringId` — [`StringId`](../../string/index.md#stringid)

  Add a string to the dynamic string table.

  

  This will be stored in the `.dynstr` section.

  

  This must be called before `Self::reserve_dynstr`.

- <span id="writer-get-dynamic-string"></span>`fn get_dynamic_string(&self, name: &'a [u8]) -> StringId` — [`StringId`](../../string/index.md#stringid)

  Get a string that was previously added to the dynamic string table.

  

  Panics if the string was not added.

- <span id="writer-dynstr-needed"></span>`fn dynstr_needed(&self) -> bool`

  Return true if `.dynstr` is needed.

- <span id="writer-require-dynstr"></span>`fn require_dynstr(&mut self)`

  Require the dynamic string table even if no strings were added.

- <span id="writer-reserve-dynstr"></span>`fn reserve_dynstr(&mut self) -> usize`

  Reserve the range for the dynamic string table.

  

  This range is used for a section named `.dynstr`.

  

  This function does nothing if no dynamic strings were defined.

  This must be called after `Self::add_dynamic_string`.

- <span id="writer-dynstr-len"></span>`fn dynstr_len(&mut self) -> usize`

  Return the size of the dynamic string table.

  

  This must be called after `Self::reserve_dynstr`.

- <span id="writer-write-dynstr"></span>`fn write_dynstr(&mut self)`

  Write the dynamic string table.

  

  This function does nothing if the section was not reserved.

- <span id="writer-reserve-dynstr-section-index"></span>`fn reserve_dynstr_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the dynamic string table.

  

  You should check `Self::dynstr_needed` before calling this

  unless you have other means of knowing if this section is needed.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-reserve-dynstr-section-index-with-name"></span>`fn reserve_dynstr_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the dynamic string table.

  

  You should check `Self::dynstr_needed` before calling this

  unless you have other means of knowing if this section is needed.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-dynstr-index"></span>`fn dynstr_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Return the section index of the dynamic string table.

- <span id="writer-write-dynstr-section-header"></span>`fn write_dynstr_section_header(&mut self, sh_addr: u64)`

  Write the section header for the dynamic string table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-null-dynamic-symbol-index"></span>`fn reserve_null_dynamic_symbol_index(&mut self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

  Reserve the null dynamic symbol table entry.

  

  This will be stored in the `.dynsym` section.

  

  The null dynamic symbol table entry is usually automatically reserved,

  but this can be used to force an empty dynamic symbol table.

  

  This must be called before `Self::reserve_dynsym`.

- <span id="writer-reserve-dynamic-symbol-index"></span>`fn reserve_dynamic_symbol_index(&mut self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

  Reserve a dynamic symbol table entry.

  

  This will be stored in the `.dynsym` section.

  

  Automatically also reserves the null symbol if required.

  Callers may assume that the returned indices will be sequential

  starting at 1.

  

  This must be called before `Self::reserve_dynsym`.

- <span id="writer-dynamic-symbol-count"></span>`fn dynamic_symbol_count(&mut self) -> u32`

  Return the number of reserved dynamic symbols.

  

  Includes the null symbol.

- <span id="writer-reserve-dynsym"></span>`fn reserve_dynsym(&mut self) -> usize`

  Reserve the range for the dynamic symbol table.

  

  This range is used for a section named `.dynsym`.

  

  This function does nothing if no dynamic symbols were reserved.

  This must be called after `Self::reserve_dynamic_symbol_index`.

- <span id="writer-write-null-dynamic-symbol"></span>`fn write_null_dynamic_symbol(&mut self)`

  Write the null dynamic symbol.

  

  This must be the first dynamic symbol that is written.

  This function does nothing if no dynamic symbols were reserved.

- <span id="writer-write-dynamic-symbol"></span>`fn write_dynamic_symbol(&mut self, sym: &Sym)` — [`Sym`](../index.md#sym)

  Write a dynamic symbol.

- <span id="writer-reserve-dynsym-section-index"></span>`fn reserve_dynsym_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the dynamic symbol table.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-reserve-dynsym-section-index-with-name"></span>`fn reserve_dynsym_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the dynamic symbol table.

  

  This must be called before `Self::reserve_section_headers`.

- <span id="writer-dynsym-index"></span>`fn dynsym_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Return the section index of the dynamic symbol table.

- <span id="writer-write-dynsym-section-header"></span>`fn write_dynsym_section_header(&mut self, sh_addr: u64, num_local: u32)`

  Write the section header for the dynamic symbol table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-dynamic"></span>`fn reserve_dynamic(&mut self, dynamic_num: usize) -> usize`

  Reserve the range for the `.dynamic` section.

  

  This function does nothing if `dynamic_num` is zero.

- <span id="writer-write-align-dynamic"></span>`fn write_align_dynamic(&mut self)`

  Write alignment padding bytes prior to the `.dynamic` section.

  

  This function does nothing if the section was not reserved.

- <span id="writer-reserve-dynamics"></span>`fn reserve_dynamics(&mut self, dynamic_num: usize) -> usize`

  Reserve a file range for the given number of dynamic entries.

  

  Returns the offset of the range.

- <span id="writer-write-dynamic-string"></span>`fn write_dynamic_string(&mut self, tag: u32, id: StringId)` — [`StringId`](../../string/index.md#stringid)

  Write a dynamic string entry.

- <span id="writer-write-dynamic"></span>`fn write_dynamic(&mut self, d_tag: u32, d_val: u64)`

  Write a dynamic value entry.

- <span id="writer-reserve-dynamic-section-index"></span>`fn reserve_dynamic_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the dynamic table.

- <span id="writer-write-dynamic-section-header"></span>`fn write_dynamic_section_header(&mut self, sh_addr: u64)`

  Write the section header for the dynamic table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-hash"></span>`fn reserve_hash(&mut self, bucket_count: u32, chain_count: u32) -> usize`

  Reserve a file range for a SysV hash section.

  

  `symbol_count` is the number of symbols in the hash,

  not the total number of symbols.

- <span id="writer-write-hash"></span>`fn write_hash<F>(&mut self, bucket_count: u32, chain_count: u32, hash: F)`

  Write a SysV hash section.

  

  `chain_count` is the number of symbols in the hash.

  The argument to `hash` will be in the range `0..chain_count`.

- <span id="writer-reserve-hash-section-index"></span>`fn reserve_hash_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the SysV hash table.

- <span id="writer-reserve-hash-section-index-with-name"></span>`fn reserve_hash_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the SysV hash table.

- <span id="writer-write-hash-section-header"></span>`fn write_hash_section_header(&mut self, sh_addr: u64)`

  Write the section header for the SysV hash table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-gnu-hash"></span>`fn reserve_gnu_hash(&mut self, bloom_count: u32, bucket_count: u32, symbol_count: u32) -> usize`

  Reserve a file range for a GNU hash section.

  

  `symbol_count` is the number of symbols in the hash,

  not the total number of symbols.

- <span id="writer-write-gnu-hash"></span>`fn write_gnu_hash<F>(&mut self, symbol_base: u32, bloom_shift: u32, bloom_count: u32, bucket_count: u32, symbol_count: u32, hash: F)`

  Write a GNU hash section.

  

  `symbol_count` is the number of symbols in the hash.

  The argument to `hash` will be in the range `0..symbol_count`.

  

  This requires that symbols are already sorted by bucket.

- <span id="writer-reserve-gnu-hash-section-index"></span>`fn reserve_gnu_hash_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the GNU hash table.

- <span id="writer-reserve-gnu-hash-section-index-with-name"></span>`fn reserve_gnu_hash_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the GNU hash table.

- <span id="writer-write-gnu-hash-section-header"></span>`fn write_gnu_hash_section_header(&mut self, sh_addr: u64)`

  Write the section header for the GNU hash table.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-gnu-versym"></span>`fn reserve_gnu_versym(&mut self) -> usize`

  Reserve the range for the `.gnu.version` section.

  

  This function does nothing if no dynamic symbols were reserved.

- <span id="writer-write-null-gnu-versym"></span>`fn write_null_gnu_versym(&mut self)`

  Write the null symbol version entry.

  

  This must be the first symbol version that is written.

  This function does nothing if no dynamic symbols were reserved.

- <span id="writer-write-gnu-versym"></span>`fn write_gnu_versym(&mut self, versym: u16)`

  Write a symbol version entry.

- <span id="writer-reserve-gnu-versym-section-index"></span>`fn reserve_gnu_versym_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.version` section.

- <span id="writer-reserve-gnu-versym-section-index-with-name"></span>`fn reserve_gnu_versym_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.version` section.

- <span id="writer-write-gnu-versym-section-header"></span>`fn write_gnu_versym_section_header(&mut self, sh_addr: u64)`

  Write the section header for the `.gnu.version` section.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-gnu-verdef"></span>`fn reserve_gnu_verdef(&mut self, verdef_count: usize, verdaux_count: usize) -> usize`

  Reserve the range for the `.gnu.version_d` section.

- <span id="writer-write-align-gnu-verdef"></span>`fn write_align_gnu_verdef(&mut self)`

  Write alignment padding bytes prior to a `.gnu.version_d` section.

- <span id="writer-write-gnu-verdef"></span>`fn write_gnu_verdef(&mut self, verdef: &Verdef)` — [`Verdef`](../index.md#verdef)

  Write a version definition entry.

- <span id="writer-write-gnu-verdef-shared"></span>`fn write_gnu_verdef_shared(&mut self, verdef: &Verdef)` — [`Verdef`](../index.md#verdef)

  Write a version definition entry that shares the names of the next definition.

  

  This is typically useful when there are only two versions (including the base)

  and they have the same name.

- <span id="writer-write-gnu-verdaux"></span>`fn write_gnu_verdaux(&mut self, name: StringId)` — [`StringId`](../../string/index.md#stringid)

  Write a version definition auxiliary entry.

- <span id="writer-reserve-gnu-verdef-section-index"></span>`fn reserve_gnu_verdef_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.version_d` section.

- <span id="writer-reserve-gnu-verdef-section-index-with-name"></span>`fn reserve_gnu_verdef_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.version_d` section.

- <span id="writer-write-gnu-verdef-section-header"></span>`fn write_gnu_verdef_section_header(&mut self, sh_addr: u64)`

  Write the section header for the `.gnu.version_d` section.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-gnu-verneed"></span>`fn reserve_gnu_verneed(&mut self, verneed_count: usize, vernaux_count: usize) -> usize`

  Reserve the range for the `.gnu.version_r` section.

- <span id="writer-write-align-gnu-verneed"></span>`fn write_align_gnu_verneed(&mut self)`

  Write alignment padding bytes prior to a `.gnu.version_r` section.

- <span id="writer-write-gnu-verneed"></span>`fn write_gnu_verneed(&mut self, verneed: &Verneed)` — [`Verneed`](../index.md#verneed)

  Write a version need entry.

- <span id="writer-write-gnu-vernaux"></span>`fn write_gnu_vernaux(&mut self, vernaux: &Vernaux)` — [`Vernaux`](../index.md#vernaux)

  Write a version need auxiliary entry.

- <span id="writer-reserve-gnu-verneed-section-index"></span>`fn reserve_gnu_verneed_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.version_r` section.

- <span id="writer-reserve-gnu-verneed-section-index-with-name"></span>`fn reserve_gnu_verneed_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.version_r` section.

- <span id="writer-write-gnu-verneed-section-header"></span>`fn write_gnu_verneed_section_header(&mut self, sh_addr: u64)`

  Write the section header for the `.gnu.version_r` section.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-reserve-gnu-attributes-section-index"></span>`fn reserve_gnu_attributes_section_index(&mut self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.attributes` section.

- <span id="writer-reserve-gnu-attributes-section-index-with-name"></span>`fn reserve_gnu_attributes_section_index_with_name(&mut self, name: &'a [u8]) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

  Reserve the section index for the `.gnu.attributes` section.

- <span id="writer-reserve-gnu-attributes"></span>`fn reserve_gnu_attributes(&mut self, gnu_attributes_size: usize) -> usize`

  Reserve the range for the `.gnu.attributes` section.

- <span id="writer-write-gnu-attributes-section-header"></span>`fn write_gnu_attributes_section_header(&mut self)`

  Write the section header for the `.gnu.attributes` section.

  

  This function does nothing if the section index was not reserved.

- <span id="writer-write-gnu-attributes"></span>`fn write_gnu_attributes(&mut self, data: &[u8])`

  Write the data for the `.gnu.attributes` section.

- <span id="writer-reserve-relocations"></span>`fn reserve_relocations(&mut self, count: usize, is_rela: bool) -> usize`

  Reserve a file range for the given number of relocations.

  

  Returns the offset of the range.

- <span id="writer-write-align-relocation"></span>`fn write_align_relocation(&mut self)`

  Write alignment padding bytes prior to a relocation section.

- <span id="writer-write-relocation"></span>`fn write_relocation(&mut self, is_rela: bool, rel: &Rel)` — [`Rel`](../index.md#rel)

  Write a relocation.

- <span id="writer-write-relocation-section-header"></span>`fn write_relocation_section_header(&mut self, name: StringId, section: SectionIndex, symtab: SectionIndex, offset: usize, count: usize, is_rela: bool)` — [`StringId`](../../string/index.md#stringid), [`SectionIndex`](../index.md#sectionindex)

  Write the section header for a relocation section.

  

  `section` is the index of the section the relocations apply to,

  or 0 if none.

  

  `symtab` is the index of the symbol table the relocations refer to,

  or 0 if none.

  

  `offset` is the file offset of the relocations.

- <span id="writer-write-relative-relocation-section-header"></span>`fn write_relative_relocation_section_header(&mut self, name: StringId, offset: usize, size: usize)` — [`StringId`](../../string/index.md#stringid)

  Write the section header for a relative relocation section.

  

  `offset` is the file offset of the relocations.

  `size` is the size of the section in bytes.

- <span id="writer-reserve-comdat"></span>`fn reserve_comdat(&mut self, count: usize) -> usize`

  Reserve a file range for a COMDAT section.

  

  `count` is the number of sections in the COMDAT group.

  

  Returns the offset of the range.

- <span id="writer-write-comdat-header"></span>`fn write_comdat_header(&mut self)`

  Write `GRP_COMDAT` at the start of the COMDAT section.

- <span id="writer-write-comdat-entry"></span>`fn write_comdat_entry(&mut self, entry: SectionIndex)` — [`SectionIndex`](../index.md#sectionindex)

  Write an entry in a COMDAT section.

- <span id="writer-write-comdat-section-header"></span>`fn write_comdat_section_header(&mut self, name: StringId, symtab: SectionIndex, symbol: SymbolIndex, offset: usize, count: usize)` — [`StringId`](../../string/index.md#stringid), [`SectionIndex`](../index.md#sectionindex), [`SymbolIndex`](../index.md#symbolindex)

  Write the section header for a COMDAT section.

- <span id="writer-attributes-writer"></span>`fn attributes_writer(&self) -> AttributesWriter` — [`AttributesWriter`](../index.md#attributeswriter)

  Return a helper for writing an attributes section.

### `AttributesWriter`

```rust
struct AttributesWriter {
    endian: Endianness,
    data: alloc::vec::Vec<u8>,
    subsection_offset: usize,
    subsubsection_offset: usize,
}
```

A helper for writing an attributes section.

Attributes have a variable length encoding, so it is awkward to write them in a
single pass. Instead, we build the entire attributes section data in memory, using
placeholders for unknown lengths that are filled in later.

#### Implementations

- <span id="attributeswriter-new"></span>`fn new(endian: Endianness) -> Self` — [`Endianness`](../../../index.md#endianness)

  Create a new `AttributesWriter` for the given endianness.

- <span id="attributeswriter-start-subsection"></span>`fn start_subsection(&mut self, vendor: &[u8])`

  Start a new subsection with the given vendor name.

- <span id="attributeswriter-end-subsection"></span>`fn end_subsection(&mut self)`

  End the subsection.

  

  The subsection length is automatically calculated and written.

- <span id="attributeswriter-start-subsubsection"></span>`fn start_subsubsection(&mut self, tag: u8)`

  Start a new sub-subsection with the given tag.

- <span id="attributeswriter-write-subsubsection-index"></span>`fn write_subsubsection_index(&mut self, index: u32)`

  Write a section or symbol index to the sub-subsection.

  

  The user must also call this function to write the terminating 0 index.

- <span id="attributeswriter-write-subsubsection-indices"></span>`fn write_subsubsection_indices(&mut self, indices: &[u8])`

  Write raw index data to the sub-subsection.

  

  The terminating 0 index is automatically written.

- <span id="attributeswriter-write-attribute-tag"></span>`fn write_attribute_tag(&mut self, tag: u64)`

  Write an attribute tag to the sub-subsection.

- <span id="attributeswriter-write-attribute-integer"></span>`fn write_attribute_integer(&mut self, value: u64)`

  Write an attribute integer value to the sub-subsection.

- <span id="attributeswriter-write-attribute-string"></span>`fn write_attribute_string(&mut self, value: &[u8])`

  Write an attribute string value to the sub-subsection.

  

  The value must not include the null terminator.

- <span id="attributeswriter-write-subsubsection-attributes"></span>`fn write_subsubsection_attributes(&mut self, attributes: &[u8])`

  Write raw attribute data to the sub-subsection.

- <span id="attributeswriter-end-subsubsection"></span>`fn end_subsubsection(&mut self)`

  End the sub-subsection.

  

  The sub-subsection length is automatically calculated and written.

- <span id="attributeswriter-data"></span>`fn data(self) -> Vec<u8>`

  Return the completed section data.

### `Class`

```rust
struct Class {
    pub is_64: bool,
}
```

An ELF file class.

#### Fields

- **`is_64`**: `bool`

  Whether the file is 64-bit.

#### Implementations

- <span id="class-align"></span>`fn align(self) -> usize`

  Return the alignment size.

- <span id="class-file-header-size"></span>`fn file_header_size(self) -> usize`

  Return the size of the file header.

- <span id="class-program-header-size"></span>`fn program_header_size(self) -> usize`

  Return the size of a program header.

- <span id="class-section-header-size"></span>`fn section_header_size(self) -> usize`

  Return the size of a section header.

- <span id="class-sym-size"></span>`fn sym_size(self) -> usize`

  Return the size of a symbol.

- <span id="class-rel-size"></span>`fn rel_size(self, is_rela: bool) -> usize`

  Return the size of a relocation entry.

- <span id="class-relr-size"></span>`fn relr_size(self) -> usize`

  Return the size of a relative relocation entry.

- <span id="class-dyn-size"></span>`fn dyn_size(self) -> usize`

  Return the size of a dynamic entry.

- <span id="class-hash-size"></span>`fn hash_size(self, bucket_count: u32, chain_count: u32) -> usize`

  Return the size of a hash table.

- <span id="class-gnu-hash-size"></span>`fn gnu_hash_size(self, bloom_count: u32, bucket_count: u32, symbol_count: u32) -> usize`

  Return the size of a GNU hash table.

- <span id="class-gnu-versym-size"></span>`fn gnu_versym_size(self, symbol_count: usize) -> usize`

  Return the size of a GNU symbol version section.

- <span id="class-gnu-verdef-size"></span>`fn gnu_verdef_size(self, verdef_count: usize, verdaux_count: usize) -> usize`

  Return the size of a GNU version definition section.

- <span id="class-gnu-verneed-size"></span>`fn gnu_verneed_size(self, verneed_count: usize, vernaux_count: usize) -> usize`

  Return the size of a GNU version dependency section.

#### Trait Implementations

##### `impl Clone for Class`

- <span id="class-clone"></span>`fn clone(&self) -> Class` — [`Class`](../index.md#class)

##### `impl Copy for Class`

##### `impl Debug for Class`

- <span id="class-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Class`

- <span id="class-default"></span>`fn default() -> Class` — [`Class`](../index.md#class)

##### `impl Eq for Class`

##### `impl<K> Equivalent for Class`

- <span id="class-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Class`

- <span id="class-partialeq-eq"></span>`fn eq(&self, other: &Class) -> bool` — [`Class`](../index.md#class)

##### `impl StructuralPartialEq for Class`

### `FileHeader`

```rust
struct FileHeader {
    pub os_abi: u8,
    pub abi_version: u8,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_entry: u64,
    pub e_flags: u32,
}
```

Native endian version of [`elf::FileHeader64`](../../../elf/index.md).

#### Trait Implementations

##### `impl Clone for FileHeader`

- <span id="fileheader-clone"></span>`fn clone(&self) -> FileHeader` — [`FileHeader`](../index.md#fileheader)

##### `impl Debug for FileHeader`

- <span id="fileheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ProgramHeader`

```rust
struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}
```

Native endian version of [`elf::ProgramHeader64`](../../../elf/index.md).

#### Trait Implementations

##### `impl Clone for ProgramHeader`

- <span id="programheader-clone"></span>`fn clone(&self) -> ProgramHeader` — [`ProgramHeader`](../index.md#programheader)

##### `impl Debug for ProgramHeader`

- <span id="programheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SectionHeader`

```rust
struct SectionHeader {
    pub name: Option<crate::write::string::StringId>,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}
```

Native endian version of [`elf::SectionHeader64`](../../../elf/index.md).

#### Trait Implementations

##### `impl Clone for SectionHeader`

- <span id="sectionheader-clone"></span>`fn clone(&self) -> SectionHeader` — [`SectionHeader`](../index.md#sectionheader)

##### `impl Debug for SectionHeader`

- <span id="sectionheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Sym`

```rust
struct Sym {
    pub name: Option<crate::write::string::StringId>,
    pub section: Option<SectionIndex>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}
```

Native endian version of [`elf::Sym64`](../../../elf/index.md).

#### Trait Implementations

##### `impl Clone for Sym`

- <span id="sym-clone"></span>`fn clone(&self) -> Sym` — [`Sym`](../index.md#sym)

##### `impl Debug for Sym`

- <span id="sym-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Rel`

```rust
struct Rel {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
    pub r_addend: i64,
}
```

Unified native endian version of [`elf::Rel64`](../../../elf/index.md) and [`elf::Rela64`](../../../elf/index.md).

#### Trait Implementations

##### `impl Clone for Rel`

- <span id="rel-clone"></span>`fn clone(&self) -> Rel` — [`Rel`](../index.md#rel)

##### `impl Debug for Rel`

- <span id="rel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Verdef`

```rust
struct Verdef {
    pub version: u16,
    pub flags: u16,
    pub index: u16,
    pub aux_count: u16,
    pub name: crate::write::string::StringId,
}
```

Information required for writing [`elf::Verdef`](../../../elf/index.md).

#### Fields

- **`name`**: `crate::write::string::StringId`

  The name for the first [`elf::Verdaux`](../../../elf/index.md) entry.

#### Trait Implementations

##### `impl Clone for Verdef`

- <span id="verdef-clone"></span>`fn clone(&self) -> Verdef` — [`Verdef`](../index.md#verdef)

##### `impl Debug for Verdef`

- <span id="verdef-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Verneed`

```rust
struct Verneed {
    pub version: u16,
    pub aux_count: u16,
    pub file: crate::write::string::StringId,
}
```

Information required for writing [`elf::Verneed`](../../../elf/index.md).

#### Trait Implementations

##### `impl Clone for Verneed`

- <span id="verneed-clone"></span>`fn clone(&self) -> Verneed` — [`Verneed`](../index.md#verneed)

##### `impl Debug for Verneed`

- <span id="verneed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Vernaux`

```rust
struct Vernaux {
    pub flags: u16,
    pub index: u16,
    pub name: crate::write::string::StringId,
}
```

Information required for writing [`elf::Vernaux`](../../../elf/index.md).

#### Trait Implementations

##### `impl Clone for Vernaux`

- <span id="vernaux-clone"></span>`fn clone(&self) -> Vernaux` — [`Vernaux`](../index.md#vernaux)

##### `impl Debug for Vernaux`

- <span id="vernaux-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Constants

### `ALIGN_SYMTAB_SHNDX`
```rust
const ALIGN_SYMTAB_SHNDX: usize = 4usize;
```

### `ALIGN_HASH`
```rust
const ALIGN_HASH: usize = 4usize;
```

### `ALIGN_GNU_VERSYM`
```rust
const ALIGN_GNU_VERSYM: usize = 2usize;
```

### `ALIGN_GNU_VERDEF`
```rust
const ALIGN_GNU_VERDEF: usize = 4usize;
```

### `ALIGN_GNU_VERNEED`
```rust
const ALIGN_GNU_VERNEED: usize = 4usize;
```

