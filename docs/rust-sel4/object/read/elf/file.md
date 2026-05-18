**object > read > elf > file**

# Module: read::elf::file

## Contents

**Structs**

- [`ElfFile`](#elffile) - A partially parsed ELF file.

**Traits**

- [`FileHeader`](#fileheader) - A trait for generic access to [`elf::FileHeader32`] and [`elf::FileHeader64`].

**Type Aliases**

- [`ElfFile32`](#elffile32) - A 32-bit ELF object file.
- [`ElfFile64`](#elffile64) - A 64-bit ELF object file.

---

## object::read::elf::file::ElfFile

*Struct*

A partially parsed ELF file.

Most functionality is provided by the [`Object`] trait implementation.

**Generic Parameters:**
- 'data
- Elf
- R

**Methods:**

- `fn parse(data: R) -> read::Result<Self>` - Parse the raw ELF file data.
- `fn endian(self: &Self) -> <Elf as >::Endian` - Returns the endianness.
- `fn data(self: &Self) -> R` - Returns the raw data.
- `fn raw_header(self: &Self) -> &'data Elf` - Returns the raw ELF file header.
- `fn raw_segments(self: &Self) -> &'data [<Elf as >::ProgramHeader]` - Returns the raw ELF segments.
- `fn elf_header(self: &Self) -> &'data Elf` - Get the raw ELF file header.
- `fn elf_program_headers(self: &Self) -> &'data [<Elf as >::ProgramHeader]` - Get the raw ELF program headers.
- `fn elf_section_table(self: &Self) -> &SectionTable<'data, Elf, R>` - Get the ELF section table.
- `fn elf_symbol_table(self: &Self) -> &SymbolTable<'data, Elf, R>` - Get the ELF symbol table.
- `fn elf_dynamic_symbol_table(self: &Self) -> &SymbolTable<'data, Elf, R>` - Get the ELF dynamic symbol table.
- `fn elf_relocation_sections(self: &Self) -> &RelocationSections` - Get a mapping for linked relocation sections.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Object**
  - `fn architecture(self: &Self) -> Architecture`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn is_64(self: &Self) -> bool`
  - `fn kind(self: &Self) -> ObjectKind`
  - `fn segments(self: &Self) -> ElfSegmentIterator<'data, Elf, R>`
  - `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>`
  - `fn section_by_index(self: &Self, index: SectionIndex) -> read::Result<ElfSection<'data, Elf, R>>`
  - `fn sections(self: &Self) -> ElfSectionIterator<'data, Elf, R>`
  - `fn comdats(self: &Self) -> ElfComdatIterator<'data, Elf, R>`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<ElfSymbol<'data, Elf, R>>`
  - `fn symbols(self: &Self) -> ElfSymbolIterator<'data, Elf, R>`
  - `fn symbol_table(self: &Self) -> Option<ElfSymbolTable<'data, Elf, R>>`
  - `fn dynamic_symbols(self: &Self) -> ElfSymbolIterator<'data, Elf, R>`
  - `fn dynamic_symbol_table(self: &Self) -> Option<ElfSymbolTable<'data, Elf, R>>`
  - `fn dynamic_relocations<'file>(self: &'file Self) -> Option<ElfDynamicRelocationIterator<'data, 'file, Elf, R>>`
  - `fn imports(self: &Self) -> read::Result<Vec<Import<'data>>>`
  - `fn exports(self: &Self) -> read::Result<Vec<Export<'data>>>`
  - `fn has_debug_symbols(self: &Self) -> bool`
  - `fn build_id(self: &Self) -> read::Result<Option<&'data [u8]>>`
  - `fn gnu_debuglink(self: &Self) -> read::Result<Option<(&'data [u8], u32)>>`
  - `fn gnu_debugaltlink(self: &Self) -> read::Result<Option<(&'data [u8], &'data [u8])>>`
  - `fn relative_address_base(self: &Self) -> u64`
  - `fn entry(self: &Self) -> u64`
  - `fn flags(self: &Self) -> FileFlags`



## object::read::elf::file::ElfFile32

*Type Alias*: `ElfFile<'data, elf::FileHeader32<Endian>, R>`

A 32-bit ELF object file.

This is a file that starts with [`elf::FileHeader32`], and corresponds
to [`crate::FileKind::Elf32`].



## object::read::elf::file::ElfFile64

*Type Alias*: `ElfFile<'data, elf::FileHeader64<Endian>, R>`

A 64-bit ELF object file.

This is a file that starts with [`elf::FileHeader64`], and corresponds
to [`crate::FileKind::Elf64`].



## object::read::elf::file::FileHeader

*Trait*

A trait for generic access to [`elf::FileHeader32`] and [`elf::FileHeader64`].

**Methods:**

- `Word`
- `Sword`
- `Endian`
- `ProgramHeader`
- `SectionHeader`
- `CompressionHeader`
- `NoteHeader`
- `Dyn`
- `Sym`
- `Rel`
- `Rela`
- `Relr`
- `is_type_64`: Return true if this type is a 64-bit header.
- `is_type_64_sized`: Return true if this type is a 64-bit header.
- `e_ident`
- `e_type`
- `e_machine`
- `e_version`
- `e_entry`
- `e_phoff`
- `e_shoff`
- `e_flags`
- `e_ehsize`
- `e_phentsize`
- `e_phnum`
- `e_shentsize`
- `e_shnum`
- `e_shstrndx`
- `parse`: Read the file header.
- `is_supported`: Check that the ident field in the file header is a supported format.
- `is_class_32`
- `is_class_64`
- `is_little_endian`
- `is_big_endian`
- `endian`
- `section_0`: Return the first section header, if present.
- `phnum`: Return the `e_phnum` field of the header. Handles extended values.
- `shnum`: Return the `e_shnum` field of the header. Handles extended values.
- `shstrndx`: Return the `e_shstrndx` field of the header. Handles extended values.
- `program_headers`: Return the slice of program headers.
- `section_headers`: Return the slice of section headers.
- `section_strings_index`: Get the section index of the section header string table.
- `section_strings`: Return the string table for the section headers.
- `sections`: Return the section table.
- `is_mips64el`: Returns whether this is a mips64el elf file.



