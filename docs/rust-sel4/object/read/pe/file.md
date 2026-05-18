**object > read > pe > file**

# Module: read::pe::file

## Contents

**Structs**

- [`PeComdat`](#pecomdat) - A COMDAT section group in a [`PeFile`].
- [`PeComdatIterator`](#pecomdatiterator) - An iterator for the COMDAT section groups in a [`PeFile`].
- [`PeComdatSectionIterator`](#pecomdatsectioniterator) - An iterator for the sections in a COMDAT section group in a [`PeFile`].
- [`PeFile`](#pefile) - A PE image file.

**Functions**

- [`optional_header_magic`](#optional_header_magic) - Find the optional header and read its `magic` field.

**Traits**

- [`ImageNtHeaders`](#imagentheaders) - A trait for generic access to [`pe::ImageNtHeaders32`] and [`pe::ImageNtHeaders64`].
- [`ImageOptionalHeader`](#imageoptionalheader) - A trait for generic access to [`pe::ImageOptionalHeader32`] and [`pe::ImageOptionalHeader64`].

**Type Aliases**

- [`PeComdat32`](#pecomdat32) - A COMDAT section group in a [`PeFile32`].
- [`PeComdat64`](#pecomdat64) - A COMDAT section group in a [`PeFile64`].
- [`PeComdatIterator32`](#pecomdatiterator32) - An iterator for the COMDAT section groups in a [`PeFile32`].
- [`PeComdatIterator64`](#pecomdatiterator64) - An iterator for the COMDAT section groups in a [`PeFile64`].
- [`PeComdatSectionIterator32`](#pecomdatsectioniterator32) - An iterator for the sections in a COMDAT section group in a [`PeFile32`].
- [`PeComdatSectionIterator64`](#pecomdatsectioniterator64) - An iterator for the sections in a COMDAT section group in a [`PeFile64`].
- [`PeFile32`](#pefile32) - A PE32 (32-bit) image file.
- [`PeFile64`](#pefile64) - A PE32+ (64-bit) image file.

---

## object::read::pe::file::ImageNtHeaders

*Trait*

A trait for generic access to [`pe::ImageNtHeaders32`] and [`pe::ImageNtHeaders64`].

**Methods:**

- `ImageOptionalHeader`
- `ImageThunkData`
- `is_type_64`: Return true if this type is a 64-bit header.
- `is_valid_optional_magic`: Return true if the magic field in the optional header is valid.
- `signature`: Return the signature
- `file_header`: Return the file header.
- `optional_header`: Return the optional header.
- `parse`: Read the NT headers, including the data directories.
- `sections`: Read the section table.
- `symbols`: Read the COFF symbol table and string table.



## object::read::pe::file::ImageOptionalHeader

*Trait*

A trait for generic access to [`pe::ImageOptionalHeader32`] and [`pe::ImageOptionalHeader64`].

**Methods:**

- `magic`
- `major_linker_version`
- `minor_linker_version`
- `size_of_code`
- `size_of_initialized_data`
- `size_of_uninitialized_data`
- `address_of_entry_point`
- `base_of_code`
- `base_of_data`
- `image_base`
- `section_alignment`
- `file_alignment`
- `major_operating_system_version`
- `minor_operating_system_version`
- `major_image_version`
- `minor_image_version`
- `major_subsystem_version`
- `minor_subsystem_version`
- `win32_version_value`
- `size_of_image`
- `size_of_headers`
- `check_sum`
- `subsystem`
- `dll_characteristics`
- `size_of_stack_reserve`
- `size_of_stack_commit`
- `size_of_heap_reserve`
- `size_of_heap_commit`
- `loader_flags`
- `number_of_rva_and_sizes`



## object::read::pe::file::PeComdat

*Struct*

A COMDAT section group in a [`PeFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Pe
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ObjectComdat**
  - `fn kind(self: &Self) -> ComdatKind`
  - `fn symbol(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn sections(self: &Self) -> <Self as >::SectionIterator`



## object::read::pe::file::PeComdat32

*Type Alias*: `PeComdat<'data, 'file, pe::ImageNtHeaders32, R>`

A COMDAT section group in a [`PeFile32`].



## object::read::pe::file::PeComdat64

*Type Alias*: `PeComdat<'data, 'file, pe::ImageNtHeaders64, R>`

A COMDAT section group in a [`PeFile64`].



## object::read::pe::file::PeComdatIterator

*Struct*

An iterator for the COMDAT section groups in a [`PeFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Pe
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::pe::file::PeComdatIterator32

*Type Alias*: `PeComdatIterator<'data, 'file, pe::ImageNtHeaders32, R>`

An iterator for the COMDAT section groups in a [`PeFile32`].



## object::read::pe::file::PeComdatIterator64

*Type Alias*: `PeComdatIterator<'data, 'file, pe::ImageNtHeaders64, R>`

An iterator for the COMDAT section groups in a [`PeFile64`].



## object::read::pe::file::PeComdatSectionIterator

*Struct*

An iterator for the sections in a COMDAT section group in a [`PeFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Pe
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::pe::file::PeComdatSectionIterator32

*Type Alias*: `PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>`

An iterator for the sections in a COMDAT section group in a [`PeFile32`].



## object::read::pe::file::PeComdatSectionIterator64

*Type Alias*: `PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>`

An iterator for the sections in a COMDAT section group in a [`PeFile64`].



## object::read::pe::file::PeFile

*Struct*

A PE image file.

Most functionality is provided by the [`Object`] trait implementation.

**Generic Parameters:**
- 'data
- Pe
- R

**Methods:**

- `fn parse(data: R) -> Result<Self>` - Parse the raw PE file data.
- `fn data(self: &Self) -> R` - Returns this binary data.
- `fn dos_header(self: &Self) -> &'data pe::ImageDosHeader` - Return the DOS header of this file.
- `fn nt_headers(self: &Self) -> &'data Pe` - Return the NT Headers of this file.
- `fn rich_header_info(self: &Self) -> Option<RichHeaderInfo>` - Returns information about the rich header of this file (if any).
- `fn section_table(self: &Self) -> SectionTable<'data>` - Returns the section table of this binary.
- `fn data_directories(self: &Self) -> DataDirectories<'data>` - Returns the data directories of this file.
- `fn data_directory(self: &Self, id: usize) -> Option<&'data pe::ImageDataDirectory>` - Returns the data directory at the given index.
- `fn export_table(self: &Self) -> Result<Option<ExportTable<'data>>>` - Returns the export table of this file.
- `fn import_table(self: &Self) -> Result<Option<ImportTable<'data>>>` - Returns the import table of this file.

**Trait Implementations:**

- **Object**
  - `fn architecture(self: &Self) -> Architecture`
  - `fn sub_architecture(self: &Self) -> Option<SubArchitecture>`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn is_64(self: &Self) -> bool`
  - `fn kind(self: &Self) -> ObjectKind`
  - `fn segments(self: &Self) -> PeSegmentIterator<'data, Pe, R>`
  - `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<PeSection<'data, 'file, Pe, R>>`
  - `fn section_by_index(self: &Self, index: SectionIndex) -> Result<PeSection<'data, Pe, R>>`
  - `fn sections(self: &Self) -> PeSectionIterator<'data, Pe, R>`
  - `fn comdats(self: &Self) -> PeComdatIterator<'data, Pe, R>`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<CoffSymbol<'data, R>>`
  - `fn symbols(self: &Self) -> CoffSymbolIterator<'data, R>`
  - `fn symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, R>>`
  - `fn dynamic_symbols(self: &Self) -> CoffSymbolIterator<'data, R>`
  - `fn dynamic_symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, R>>`
  - `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>`
  - `fn imports(self: &Self) -> Result<Vec<Import<'data>>>`
  - `fn exports(self: &Self) -> Result<Vec<Export<'data>>>`
  - `fn pdb_info(self: &Self) -> Result<Option<CodeView>>`
  - `fn has_debug_symbols(self: &Self) -> bool`
  - `fn relative_address_base(self: &Self) -> u64`
  - `fn entry(self: &Self) -> u64`
  - `fn flags(self: &Self) -> FileFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::pe::file::PeFile32

*Type Alias*: `PeFile<'data, pe::ImageNtHeaders32, R>`

A PE32 (32-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders32`], and corresponds
to [`crate::FileKind::Pe32`].



## object::read::pe::file::PeFile64

*Type Alias*: `PeFile<'data, pe::ImageNtHeaders64, R>`

A PE32+ (64-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders64`], and corresponds
to [`crate::FileKind::Pe64`].



## object::read::pe::file::optional_header_magic

*Function*

Find the optional header and read its `magic` field.

It can be useful to know this magic value before trying to
fully parse the NT headers.

```rust
fn optional_header_magic<'data, R>(data: R) -> crate::read::Result<u16>
```



