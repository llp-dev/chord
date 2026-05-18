**object > read > pe > export**

# Module: read::pe::export

## Contents

**Structs**

- [`Export`](#export) - An export from a PE file.
- [`ExportTable`](#exporttable) - A partially parsed PE export table.

**Enums**

- [`ExportTarget`](#exporttarget) - Where an export is pointing to.

---

## object::read::pe::export::Export

*Struct*

An export from a PE file.

There are multiple kinds of PE exports (with or without a name, and local or forwarded).

**Generic Parameters:**
- 'data

**Fields:**
- `ordinal: u32` - The ordinal of the export.
- `name: Option<&'data [u8]>` - The name of the export, if known.
- `target: ExportTarget<'data>` - The target of this export.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Export<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>`



## object::read::pe::export::ExportTable

*Struct*

A partially parsed PE export table.

Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table).

**Generic Parameters:**
- 'data

**Methods:**

- `fn parse(data: &'data [u8], virtual_address: u32) -> Result<Self>` - Parse the export table given its section data and address.
- `fn parse_directory(data: &'data [u8]) -> Result<&'data pe::ImageExportDirectory>` - Parse the export directory given its section data.
- `fn directory(self: &Self) -> &'data pe::ImageExportDirectory` - Returns the header of the export table.
- `fn ordinal_base(self: &Self) -> u32` - Returns the base value of ordinals.
- `fn addresses(self: &Self) -> &'data [U32Bytes<LE>]` - Returns the unparsed address table.
- `fn name_pointers(self: &Self) -> &'data [U32Bytes<LE>]` - Returns the unparsed name pointer table.
- `fn name_ordinals(self: &Self) -> &'data [U16Bytes<LE>]` - Returns the unparsed ordinal table.
- `fn name_iter(self: &Self) -> impl Trait` - Returns an iterator for the entries in the name pointer table and ordinal table.
- `fn address_by_index(self: &Self, index: u32) -> Result<u32>` - Returns the export address table entry at the given address index.
- `fn address_by_ordinal(self: &Self, ordinal: u32) -> Result<u32>` - Returns the export address table entry at the given ordinal.
- `fn target_by_index(self: &Self, index: u32) -> Result<ExportTarget<'data>>` - Returns the target of the export at the given address index.
- `fn target_by_ordinal(self: &Self, ordinal: u32) -> Result<ExportTarget<'data>>` - Returns the target of the export at the given ordinal.
- `fn target_from_address(self: &Self, address: u32) -> Result<ExportTarget<'data>>` - Convert an export address table entry into a target.
- `fn is_forward(self: &Self, address: u32) -> bool` - Return true if the export address table entry is a forward.
- `fn forward_string(self: &Self, address: u32) -> Result<Option<&'data [u8]>>` - Return the forward string if the export address table entry is a forward.
- `fn name_from_pointer(self: &Self, name_pointer: u32) -> Result<&'data [u8]>` - Convert an export name pointer table entry into a name.
- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` - Returns the parsed exports in this table.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ExportTable<'data>`



## object::read::pe::export::ExportTarget

*Enum*

Where an export is pointing to.

**Generic Parameters:**
- 'data

**Variants:**
- `Address(u32)` - The address of the export, relative to the image base.
- `ForwardByOrdinal(&'data [u8], u32)` - Forwarded to an export ordinal in another DLL.
- `ForwardByName(&'data [u8], &'data [u8])` - Forwarded to an export name in another DLL.

**Methods:**

- `fn is_address(self: &Self) -> bool` - Returns true if the target is an address.
- `fn is_forward(self: &Self) -> bool` - Returns true if the export is forwarded to another DLL.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ExportTarget<'data>`



