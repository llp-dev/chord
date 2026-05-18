**object > read > coff > import**

# Module: read::coff::import

## Contents

**Structs**

- [`ImportFile`](#importfile) - A Windows short form description of a symbol to import.
- [`ImportObjectData`](#importobjectdata) - The data following [`pe::ImportObjectHeader`].

**Enums**

- [`ImportName`](#importname) - The name or ordinal to import from a DLL.
- [`ImportType`](#importtype) - The kind of import symbol.

---

## object::read::coff::import::ImportFile

*Struct*

A Windows short form description of a symbol to import.

Used in Windows import libraries to provide a mapping from
a symbol name to a DLL export. This is not an object file.

This is a file that starts with [`pe::ImportObjectHeader`], and corresponds
to [`crate::FileKind::CoffImport`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn parse<R>(data: R) -> Result<Self>` - Parse it.
- `fn architecture(self: &Self) -> Architecture` - Get the machine type.
- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` - Get the sub machine type, if available.
- `fn symbol(self: &Self) -> &'data [u8]` - The public symbol name.
- `fn dll(self: &Self) -> &'data [u8]` - The name of the DLL to import the symbol from.
- `fn import(self: &Self) -> ImportName<'data>` - The name exported from the DLL.
- `fn import_type(self: &Self) -> ImportType` - The type of import. Usually either a function or data.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImportFile<'data>`



## object::read::coff::import::ImportName

*Enum*

The name or ordinal to import from a DLL.

**Generic Parameters:**
- 'data

**Variants:**
- `Ordinal(u16)` - Import by ordinal. Ordinarily this is a 1-based index.
- `Name(&'data [u8])` - Import by name.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ImportName<'data>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ImportName<'data>`



## object::read::coff::import::ImportObjectData

*Struct*

The data following [`pe::ImportObjectHeader`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn symbol(self: &Self) -> &'data [u8]` - The public symbol name.
- `fn dll(self: &Self) -> &'data [u8]` - The name of the DLL to import the symbol from.
- `fn export(self: &Self) -> Option<&'data [u8]>` - The name exported from the DLL.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImportObjectData<'data>`



## object::read::coff::import::ImportType

*Enum*

The kind of import symbol.

**Variants:**
- `Code` - An executable code symbol.
- `Data` - A data symbol.
- `Const` - A constant value.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImportType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ImportType) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



