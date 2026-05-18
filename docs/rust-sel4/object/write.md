**object > write**

# Module: write

## Contents

**Modules**

- [`coff`](#coff) - Support for writing COFF files.
- [`elf`](#elf) - Support for writing ELF files.
- [`pe`](#pe) - Helper for writing PE files.

**Structs**

- [`Comdat`](#comdat) - A COMDAT section group.
- [`ComdatId`](#comdatid) - An identifier used to reference a COMDAT section group.
- [`Error`](#error) - The error type used within the write module.
- [`Object`](#object) - A writable relocatable object file.
- [`Relocation`](#relocation) - A relocation in an object file.
- [`Section`](#section) - A section in an object file.
- [`SectionId`](#sectionid) - An identifier used to reference a section.
- [`Symbol`](#symbol) - A symbol in an object file.
- [`SymbolId`](#symbolid) - An identifier used to reference a symbol.

**Enums**

- [`Mangling`](#mangling) - The symbol name mangling scheme.
- [`StandardSection`](#standardsection) - A standard section kind.
- [`StandardSegment`](#standardsegment) - A standard segment kind.
- [`SymbolSection`](#symbolsection) - The section where a symbol is defined.

**Type Aliases**

- [`Result`](#result) - The result type used within the write module.

---

## object::write::Comdat

*Struct*

A COMDAT section group.

**Fields:**
- `kind: ComdatKind` - The COMDAT selection kind.
- `symbol: SymbolId` - The COMDAT symbol.
- `sections: alloc::vec::Vec<SectionId>` - The sections in the group.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::ComdatId

*Struct*

An identifier used to reference a COMDAT section group.

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ComdatId`
- **Ord**
  - `fn cmp(self: &Self, other: &ComdatId) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ComdatId) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ComdatId) -> $crate::option::Option<$crate::cmp::Ordering>`



## object::write::Error

*Struct*

The error type used within the write module.

**Tuple Struct**: `()`

**Traits:** Eq, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::write::Mangling

*Enum*

The symbol name mangling scheme.

**Variants:**
- `None` - No symbol mangling.
- `Coff` - Windows COFF symbol mangling.
- `CoffI386` - Windows COFF i386 symbol mangling.
- `Elf` - ELF symbol mangling.
- `MachO` - Mach-O symbol mangling.
- `Xcoff` - Xcoff symbol mangling.

**Methods:**

- `fn default(format: BinaryFormat, architecture: Architecture) -> Self` - Return the default symboling mangling for the given format and architecture.
- `fn global_prefix(self: Self) -> Option<u8>` - Return the prefix to use for global symbols.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Mangling) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Mangling`



## object::write::Object

*Struct*

A writable relocatable object file.

**Generic Parameters:**
- 'a

**Fields:**
- `flags: FileFlags` - File flags that are specific to each file format.
- `mangling: Mangling` - The symbol name mangling scheme.

**Methods:**

- `fn add_coff_exports(self: & mut Self, style: CoffExportStyle)` - Appends linker directives to the `.drectve` section to tell the linker
- `fn add_elf_gnu_property_u32(self: & mut Self, property: u32, value: u32)` - Add a property with a u32 value to the ELF ".note.gnu.property" section.
- `fn set_macho_cpu_subtype(self: & mut Self, cpu_subtype: u32)` - Specify the Mach-O CPU subtype.
- `fn set_macho_build_version(self: & mut Self, info: MachOBuildVersion)` - Specify information for a Mach-O `LC_BUILD_VERSION` command.
- `fn new(format: BinaryFormat, architecture: Architecture, endian: Endianness) -> Object<'a>` - Create an empty object file.
- `fn format(self: &Self) -> BinaryFormat` - Return the file format.
- `fn architecture(self: &Self) -> Architecture` - Return the architecture.
- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` - Return the sub-architecture.
- `fn set_sub_architecture(self: & mut Self, sub_architecture: Option<SubArchitecture>)` - Specify the sub-architecture.
- `fn mangling(self: &Self) -> Mangling` - Return the current mangling setting.
- `fn set_mangling(self: & mut Self, mangling: Mangling)` - Specify the mangling setting.
- `fn segment_name(self: &Self, segment: StandardSegment) -> &'static [u8]` - Return the name for a standard segment.
- `fn section(self: &Self, section: SectionId) -> &Section<'a>` - Get the section with the given `SectionId`.
- `fn section_mut(self: & mut Self, section: SectionId) -> & mut Section<'a>` - Mutably get the section with the given `SectionId`.
- `fn set_section_data<T>(self: & mut Self, section: SectionId, data: T, align: u64)` - Set the data for an existing section.
- `fn append_section_data(self: & mut Self, section: SectionId, data: &[u8], align: u64) -> u64` - Append data to an existing section. Returns the section offset of the data.
- `fn append_section_bss(self: & mut Self, section: SectionId, size: u64, align: u64) -> u64` - Append zero-initialized data to an existing section. Returns the section offset of the data.
- `fn section_id(self: & mut Self, section: StandardSection) -> SectionId` - Return the `SectionId` of a standard section.
- `fn add_section(self: & mut Self, segment: Vec<u8>, name: Vec<u8>, kind: SectionKind) -> SectionId` - Add a new section and return its `SectionId`.
- `fn add_subsection(self: & mut Self, section: StandardSection, name: &[u8]) -> SectionId` - Add a subsection. Returns the `SectionId` and section offset of the data.
- `fn set_subsections_via_symbols(self: & mut Self)` - Enable subsections via symbols if supported.
- `fn default_section_flags(self: &Self, section: &Section) -> SectionFlags` - Return the default flags for a section.
- `fn section_flags(self: &Self, section: &Section) -> SectionFlags` - Return the flags for a section.
- `fn section_flags_mut(self: & mut Self, section_id: SectionId) -> & mut SectionFlags` - Mutably get the flags for a section.
- `fn comdat(self: &Self, comdat: ComdatId) -> &Comdat` - Get the COMDAT section group with the given `ComdatId`.
- `fn comdat_mut(self: & mut Self, comdat: ComdatId) -> & mut Comdat` - Mutably get the COMDAT section group with the given `ComdatId`.
- `fn add_comdat(self: & mut Self, comdat: Comdat) -> ComdatId` - Add a new COMDAT section group and return its `ComdatId`.
- `fn symbol_id(self: &Self, name: &[u8]) -> Option<SymbolId>` - Get the `SymbolId` of the symbol with the given name.
- `fn symbol(self: &Self, symbol: SymbolId) -> &Symbol` - Get the symbol with the given `SymbolId`.
- `fn symbol_mut(self: & mut Self, symbol: SymbolId) -> & mut Symbol` - Mutably get the symbol with the given `SymbolId`.
- `fn add_symbol(self: & mut Self, symbol: Symbol) -> SymbolId` - Add a new symbol and return its `SymbolId`.
- `fn default_symbol_flags(self: &Self, symbol: &Symbol) -> SymbolFlags<SectionId, SymbolId>` - Return the default flags for a symbol.
- `fn symbol_flags(self: &Self, symbol: &Symbol) -> SymbolFlags<SectionId, SymbolId>` - Return the flags for a symbol.
- `fn symbol_flags_mut(self: & mut Self, symbol_id: SymbolId) -> & mut SymbolFlags<SectionId, SymbolId>` - Mutably get the flags for a symbol.
- `fn has_uninitialized_tls(self: &Self) -> bool` - Return true if the file format supports `StandardSection::UninitializedTls`.
- `fn has_common(self: &Self) -> bool` - Return true if the file format supports `StandardSection::Common`.
- `fn add_common_symbol(self: & mut Self, symbol: Symbol, size: u64, align: u64) -> SymbolId` - Add a new common symbol and return its `SymbolId`.
- `fn add_file_symbol(self: & mut Self, name: Vec<u8>) -> SymbolId` - Add a new file symbol and return its `SymbolId`.
- `fn section_symbol(self: & mut Self, section_id: SectionId) -> SymbolId` - Get the symbol for a section.
- `fn add_symbol_data(self: & mut Self, symbol_id: SymbolId, section: SectionId, data: &[u8], align: u64) -> u64` - Append data to an existing section, and update a symbol to refer to it.
- `fn add_symbol_bss(self: & mut Self, symbol_id: SymbolId, section: SectionId, size: u64, align: u64) -> u64` - Append zero-initialized data to an existing section, and update a symbol to refer to it.
- `fn set_symbol_data(self: & mut Self, symbol_id: SymbolId, section: SectionId, offset: u64, size: u64)` - Update a symbol to refer to the given data within a section.
- `fn symbol_section_and_offset(self: & mut Self, symbol_id: SymbolId) -> Option<(SymbolId, u64)>` - Convert a symbol to a section symbol and offset.
- `fn add_relocation(self: & mut Self, section: SectionId, relocation: Relocation) -> Result<()>` - Add a relocation to a section.
- `fn write(self: &Self) -> Result<Vec<u8>>` - Write the object to a `Vec`.
- `fn write_stream<W>(self: &Self, w: W) -> result::Result<(), Box<dyn error::Error>>` - Write the object to a `Write` implementation.
- `fn emit(self: &Self, buffer: & mut dyn WritableBuffer) -> Result<()>` - Write the object to a `WritableBuffer`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::Relocation

*Struct*

A relocation in an object file.

**Fields:**
- `offset: u64` - The section offset of the place of the relocation.
- `symbol: SymbolId` - The symbol referred to by the relocation.
- `addend: i64` - The addend to use in the relocation calculation.
- `flags: RelocationFlags` - The fields that define the relocation type.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::Result

*Type Alias*: `result::Result<T, Error>`

The result type used within the write module.



## object::write::Section

*Struct*

A section in an object file.

**Generic Parameters:**
- 'a

**Fields:**
- `flags: SectionFlags` - Section flags that are specific to each file format.

**Methods:**

- `fn name(self: &Self) -> Option<&str>` - Try to convert the name to a utf8 string.
- `fn segment(self: &Self) -> Option<&str>` - Try to convert the segment to a utf8 string.
- `fn is_bss(self: &Self) -> bool` - Return true if this section contains zerofill data.
- `fn set_data<T>(self: & mut Self, data: T, align: u64)` - Set the data for a section.
- `fn append_data(self: & mut Self, append_data: &[u8], align: u64) -> u64` - Append data to a section.
- `fn append_bss(self: & mut Self, size: u64, align: u64) -> u64` - Append uninitialized data to a section.
- `fn data(self: &Self) -> &[u8]` - Returns the section as-built so far.
- `fn data_mut(self: & mut Self) -> & mut [u8]` - Returns the section as-built so far.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::SectionId

*Struct*

An identifier used to reference a section.

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionId`
- **Ord**
  - `fn cmp(self: &Self, other: &SectionId) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionId) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SectionId) -> $crate::option::Option<$crate::cmp::Ordering>`



## object::write::StandardSection

*Enum*

A standard section kind.

**Variants:**
- `Text`
- `Data`
- `ReadOnlyData`
- `ReadOnlyDataWithRel`
- `ReadOnlyString`
- `UninitializedData`
- `Tls`
- `UninitializedTls` - Zero-fill TLS initializers. Unsupported for COFF.
- `TlsVariables` - TLS variable structures. Only supported for Mach-O.
- `Common` - Common data. Only supported for Mach-O.
- `GnuProperty` - Notes for GNU properties. Only supported for ELF.

**Methods:**

- `fn kind(self: Self) -> SectionKind` - Return the section kind of a standard section.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> StandardSection`
- **Ord**
  - `fn cmp(self: &Self, other: &StandardSection) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &StandardSection) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &StandardSection) -> $crate::option::Option<$crate::cmp::Ordering>`



## object::write::StandardSegment

*Enum*

A standard segment kind.

**Variants:**
- `Text`
- `Data`
- `Debug`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StandardSegment) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &StandardSegment) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> StandardSegment`
- **Ord**
  - `fn cmp(self: &Self, other: &StandardSegment) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::Symbol

*Struct*

A symbol in an object file.

**Fields:**
- `name: alloc::vec::Vec<u8>` - The name of the symbol.
- `value: u64` - The value of the symbol.
- `size: u64` - The size of the symbol.
- `kind: SymbolKind` - The kind of the symbol.
- `scope: SymbolScope` - The scope of the symbol.
- `weak: bool` - Whether the symbol has weak binding.
- `section: SymbolSection` - The section containing the symbol.
- `flags: SymbolFlags<SectionId, SymbolId>` - Symbol flags that are specific to each file format.

**Methods:**

- `fn name(self: &Self) -> Option<&str>` - Try to convert the name to a utf8 string.
- `fn is_undefined(self: &Self) -> bool` - Return true if the symbol is undefined.
- `fn is_common(self: &Self) -> bool` - Return true if the symbol is common data.
- `fn is_local(self: &Self) -> bool` - Return true if the symbol scope is local.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::SymbolId

*Struct*

An identifier used to reference a symbol.

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolId) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SymbolId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> SymbolId`
- **Ord**
  - `fn cmp(self: &Self, other: &SymbolId) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::write::SymbolSection

*Enum*

The section where a symbol is defined.

**Variants:**
- `None` - The section is not applicable for this symbol (such as file symbols).
- `Undefined` - The symbol is undefined.
- `Absolute` - The symbol has an absolute value.
- `Common` - The symbol is a zero-initialized symbol that will be combined with duplicate definitions.
- `Section(SectionId)` - The symbol is defined in the given section.

**Methods:**

- `fn id(self: Self) -> Option<SectionId>` - Returns the section id for the section where the symbol is defined.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolSection) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SymbolSection`



## Module: coff

Support for writing COFF files.

Provides [`Writer`] for low level writing of COFF files.
This is also used to provide COFF support for [`write::Object`](crate::write::Object).



## Module: elf

Support for writing ELF files.

Provides [`Writer`] for low level writing of ELF files.
This is also used to provide ELF support for [`write::Object`](crate::write::Object).



## Module: pe

Helper for writing PE files.



