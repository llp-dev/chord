**object > read > pe > import**

# Module: read::pe::import

## Contents

**Structs**

- [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator) - A fallible iterator for the descriptors in the delay-load data directory.
- [`DelayLoadImportTable`](#delayloadimporttable) - Information for parsing a PE delay-load import table.
- [`ImportDescriptorIterator`](#importdescriptoriterator) - A fallible iterator for the descriptors in the import data directory.
- [`ImportTable`](#importtable) - Information for parsing a PE import table.
- [`ImportThunkList`](#importthunklist) - A list of import thunks.

**Enums**

- [`Import`](#import) - A parsed import thunk.

**Traits**

- [`ImageThunkData`](#imagethunkdata) - A trait for generic access to [`pe::ImageThunkData32`] and [`pe::ImageThunkData64`].

---

## object::read::pe::import::DelayLoadDescriptorIterator

*Struct*

A fallible iterator for the descriptors in the delay-load data directory.

**Generic Parameters:**
- 'data

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<&'data pe::ImageDelayloadDescriptor>>` - Return the next descriptor.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DelayLoadDescriptorIterator<'data>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::pe::import::DelayLoadImportTable

*Struct*

Information for parsing a PE delay-load import table.

Returned by
[`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table).

**Generic Parameters:**
- 'data

**Methods:**

- `fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self` - Create a new delay load import table parser.
- `fn descriptors(self: &Self) -> Result<DelayLoadDescriptorIterator<'data>>` - Return an iterator for the import descriptors.
- `fn name(self: &Self, address: u32) -> Result<&'data [u8]>` - Return a library name given its address.
- `fn thunks(self: &Self, address: u32) -> Result<ImportThunkList<'data>>` - Return a list of thunks given its address.
- `fn import<Pe>(self: &Self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` - Parse a thunk.
- `fn hint_name(self: &Self, address: u32) -> Result<(u16, &'data [u8])>` - Return the hint and name at the given address.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DelayLoadImportTable<'data>`



## object::read::pe::import::ImageThunkData

*Trait*

A trait for generic access to [`pe::ImageThunkData32`] and [`pe::ImageThunkData64`].

**Methods:**

- `raw`: Return the raw thunk value.
- `is_ordinal`: Returns true if the ordinal flag is set.
- `ordinal`: Return the ordinal portion of the thunk.
- `address`: Return the RVA portion of the thunk.



## object::read::pe::import::Import

*Enum*

A parsed import thunk.

**Generic Parameters:**
- 'data

**Variants:**
- `Ordinal(u16)` - Import by ordinal.
- `Name(u16, &'data [u8])` - Import by name.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Import<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::pe::import::ImportDescriptorIterator

*Struct*

A fallible iterator for the descriptors in the import data directory.

**Generic Parameters:**
- 'data

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<&'data pe::ImageImportDescriptor>>` - Return the next descriptor.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImportDescriptorIterator<'data>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::pe::import::ImportTable

*Struct*

Information for parsing a PE import table.

Returned by [`DataDirectories::import_table`](super::DataDirectories::import_table).

**Generic Parameters:**
- 'data

**Methods:**

- `fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self` - Create a new import table parser.
- `fn descriptors(self: &Self) -> Result<ImportDescriptorIterator<'data>>` - Return an iterator for the import descriptors.
- `fn name(self: &Self, address: u32) -> Result<&'data [u8]>` - Return a library name given its address.
- `fn thunks(self: &Self, address: u32) -> Result<ImportThunkList<'data>>` - Return a list of thunks given its address.
- `fn import<Pe>(self: &Self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` - Parse a thunk.
- `fn hint_name(self: &Self, address: u32) -> Result<(u16, &'data [u8])>` - Return the hint and name at the given address.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImportTable<'data>`



## object::read::pe::import::ImportThunkList

*Struct*

A list of import thunks.

These may be in the import lookup table, or the import address table.

**Generic Parameters:**
- 'data

**Methods:**

- `fn get<Pe>(self: &Self, index: usize) -> Result<<Pe as >::ImageThunkData>` - Get the thunk at the given index.
- `fn next<Pe>(self: & mut Self) -> Result<Option<<Pe as >::ImageThunkData>>` - Return the first thunk in the list, and update `self` to point after it.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImportThunkList<'data>`



