**object > read > coff > file**

# Module: read::coff::file

## Contents

**Structs**

- [`CoffFile`](#cofffile) - A COFF object file.

**Functions**

- [`anon_object_class_id`](#anon_object_class_id) - Read the `class_id` field from a [`pe::AnonObjectHeader`].

**Traits**

- [`CoffHeader`](#coffheader) - A trait for generic access to [`pe::ImageFileHeader`] and [`pe::AnonObjectHeaderBigobj`].

**Type Aliases**

- [`CoffBigFile`](#coffbigfile) - A COFF bigobj object file with 32-bit section numbers.

---

## object::read::coff::file::CoffBigFile

*Type Alias*: `CoffFile<'data, R, pe::AnonObjectHeaderBigobj>`

A COFF bigobj object file with 32-bit section numbers.

This is a file that starts with [`pe::AnonObjectHeaderBigobj`], and corresponds
to [`crate::FileKind::CoffBig`].

Most functionality is provided by the [`Object`] trait implementation.



## object::read::coff::file::CoffFile

*Struct*

A COFF object file.

This is a file that starts with [`pe::ImageFileHeader`], and corresponds
to [`crate::FileKind::Coff`].

Most functionality is provided by the [`Object`] trait implementation.

**Generic Parameters:**
- 'data
- R
- Coff

**Methods:**

- `fn parse(data: R) -> Result<Self>` - Parse the raw COFF file data.
- `fn coff_header(self: &Self) -> &'data Coff` - Get the raw COFF file header.
- `fn coff_section_table(self: &Self) -> SectionTable<'data>` - Get the COFF section table.
- `fn coff_symbol_table(self: &Self) -> &SymbolTable<'data, R, Coff>` - Get the COFF symbol table.

**Trait Implementations:**

- **Object**
  - `fn architecture(self: &Self) -> Architecture`
  - `fn sub_architecture(self: &Self) -> Option<SubArchitecture>`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn is_64(self: &Self) -> bool`
  - `fn kind(self: &Self) -> ObjectKind`
  - `fn segments(self: &Self) -> CoffSegmentIterator<'data, R, Coff>`
  - `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<CoffSection<'data, 'file, R, Coff>>`
  - `fn section_by_index(self: &Self, index: SectionIndex) -> Result<CoffSection<'data, R, Coff>>`
  - `fn sections(self: &Self) -> CoffSectionIterator<'data, R, Coff>`
  - `fn comdats(self: &Self) -> CoffComdatIterator<'data, R, Coff>`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<CoffSymbol<'data, R, Coff>>`
  - `fn symbols(self: &Self) -> CoffSymbolIterator<'data, R, Coff>`
  - `fn symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, R, Coff>>`
  - `fn dynamic_symbols(self: &Self) -> CoffSymbolIterator<'data, R, Coff>`
  - `fn dynamic_symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, R, Coff>>`
  - `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>`
  - `fn imports(self: &Self) -> Result<Vec<Import<'data>>>`
  - `fn exports(self: &Self) -> Result<Vec<Export<'data>>>`
  - `fn has_debug_symbols(self: &Self) -> bool`
  - `fn relative_address_base(self: &Self) -> u64`
  - `fn entry(self: &Self) -> u64`
  - `fn flags(self: &Self) -> FileFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::coff::file::CoffHeader

*Trait*

A trait for generic access to [`pe::ImageFileHeader`] and [`pe::AnonObjectHeaderBigobj`].

**Methods:**

- `ImageSymbol`
- `ImageSymbolBytes`
- `is_type_bigobj`: Return true if this type is [`pe::AnonObjectHeaderBigobj`].
- `machine`
- `number_of_sections`
- `pointer_to_symbol_table`
- `number_of_symbols`
- `characteristics`
- `parse`: Read the file header.
- `sections`: Read the section table.
- `symbols`: Read the symbol table and string table.



## object::read::coff::file::anon_object_class_id

*Function*

Read the `class_id` field from a [`pe::AnonObjectHeader`].

This can be used to determine the format of the header.

```rust
fn anon_object_class_id<'data, R>(data: R) -> crate::read::Result<pe::ClsId>
```



