**object > read > xcoff > file**

# Module: read::xcoff::file

## Contents

**Structs**

- [`XcoffFile`](#xcofffile) - A partially parsed XCOFF file.

**Traits**

- [`AuxHeader`](#auxheader) - A trait for generic access to [`xcoff::AuxHeader32`] and [`xcoff::AuxHeader64`].
- [`FileHeader`](#fileheader) - A trait for generic access to [`xcoff::FileHeader32`] and [`xcoff::FileHeader64`].

**Type Aliases**

- [`XcoffFile32`](#xcofffile32) - A 32-bit XCOFF object file.
- [`XcoffFile64`](#xcofffile64) - A 64-bit XCOFF object file.

---

## object::read::xcoff::file::AuxHeader

*Trait*

A trait for generic access to [`xcoff::AuxHeader32`] and [`xcoff::AuxHeader64`].

**Methods:**

- `Word`
- `o_mflag`
- `o_vstamp`
- `o_tsize`
- `o_dsize`
- `o_bsize`
- `o_entry`
- `o_text_start`
- `o_data_start`
- `o_toc`
- `o_snentry`
- `o_sntext`
- `o_sndata`
- `o_sntoc`
- `o_snloader`
- `o_snbss`
- `o_algntext`
- `o_algndata`
- `o_modtype`
- `o_cpuflag`
- `o_cputype`
- `o_maxstack`
- `o_maxdata`
- `o_debugger`
- `o_textpsize`
- `o_datapsize`
- `o_stackpsize`
- `o_flags`
- `o_sntdata`
- `o_sntbss`
- `o_x64flags`



## object::read::xcoff::file::FileHeader

*Trait*

A trait for generic access to [`xcoff::FileHeader32`] and [`xcoff::FileHeader64`].

**Methods:**

- `Word`
- `AuxHeader`
- `SectionHeader`
- `Symbol`
- `FileAux`
- `CsectAux`
- `Rel`
- `is_type_64`: Return true if this type is a 64-bit header.
- `f_magic`
- `f_nscns`
- `f_timdat`
- `f_symptr`
- `f_nsyms`
- `f_opthdr`
- `f_flags`
- `parse`: Read the file header.
- `is_supported`
- `aux_header`: Read the auxiliary file header.
- `sections`: Read the section table.
- `symbols`: Return the symbol table.



## object::read::xcoff::file::XcoffFile

*Struct*

A partially parsed XCOFF file.

Most functionality is provided by the [`Object`] trait implementation.

**Generic Parameters:**
- 'data
- Xcoff
- R

**Methods:**

- `fn parse(data: R) -> Result<Self>` - Parse the raw XCOFF file data.
- `fn data(self: &Self) -> R` - Returns the raw data.
- `fn raw_header(self: &Self) -> &'data Xcoff` - Returns the raw XCOFF file header.
- `fn xcoff_header(self: &Self) -> &'data Xcoff` - Get the raw XCOFF file header.
- `fn xcoff_aux_header(self: &Self) -> Option<&'data <Xcoff as >::AuxHeader>` - Get the raw XCOFF auxiliary header.
- `fn xcoff_section_table(self: &Self) -> &SectionTable<'data, Xcoff>` - Get the XCOFF section table.
- `fn xcoff_symbol_table(self: &Self) -> &SymbolTable<'data, Xcoff, R>` - Get the XCOFF symbol table.

**Trait Implementations:**

- **Object**
  - `fn architecture(self: &Self) -> Architecture`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn is_64(self: &Self) -> bool`
  - `fn kind(self: &Self) -> ObjectKind`
  - `fn segments(self: &Self) -> XcoffSegmentIterator<'data, Xcoff, R>`
  - `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>`
  - `fn section_by_index(self: &Self, index: SectionIndex) -> Result<XcoffSection<'data, Xcoff, R>>`
  - `fn sections(self: &Self) -> XcoffSectionIterator<'data, Xcoff, R>`
  - `fn comdats(self: &Self) -> XcoffComdatIterator<'data, Xcoff, R>`
  - `fn symbol_table(self: &Self) -> Option<XcoffSymbolTable<'data, Xcoff, R>>`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<XcoffSymbol<'data, Xcoff, R>>`
  - `fn symbols(self: &Self) -> XcoffSymbolIterator<'data, Xcoff, R>`
  - `fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>`
  - `fn dynamic_symbols(self: &Self) -> XcoffSymbolIterator<'data, Xcoff, R>`
  - `fn dynamic_relocations(self: &Self) -> Option<<Self as >::DynamicRelocationIterator>`
  - `fn imports(self: &Self) -> Result<alloc::vec::Vec<Import<'data>>>`
  - `fn exports(self: &Self) -> Result<alloc::vec::Vec<Export<'data>>>`
  - `fn has_debug_symbols(self: &Self) -> bool`
  - `fn relative_address_base(self: &Self) -> u64`
  - `fn entry(self: &Self) -> u64`
  - `fn flags(self: &Self) -> FileFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::xcoff::file::XcoffFile32

*Type Alias*: `XcoffFile<'data, xcoff::FileHeader32, R>`

A 32-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader32`], and corresponds
to [`crate::FileKind::Xcoff32`].



## object::read::xcoff::file::XcoffFile64

*Type Alias*: `XcoffFile<'data, xcoff::FileHeader64, R>`

A 64-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader64`], and corresponds
to [`crate::FileKind::Xcoff64`].



