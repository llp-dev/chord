**object > read > pe > resource**

# Module: read::pe::resource

## Contents

**Structs**

- [`ResourceDirectory`](#resourcedirectory) - The `.rsrc` section of a PE file.
- [`ResourceDirectoryTable`](#resourcedirectorytable) - A table of resource entries.
- [`ResourceName`](#resourcename) - A resource name.

**Enums**

- [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata) - Data associated with a resource directory entry.
- [`ResourceNameOrId`](#resourcenameorid) - A resource name or ID.

---

## object::read::pe::resource::ResourceDirectory

*Struct*

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

**Generic Parameters:**
- 'data

**Methods:**

- `fn new(data: &'data [u8]) -> Self` - Construct from the data of the `.rsrc` section.
- `fn root(self: &Self) -> Result<ResourceDirectoryTable<'data>>` - Parses the root resource directory.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ResourceDirectory<'data>`



## object::read::pe::resource::ResourceDirectoryEntryData

*Enum*

Data associated with a resource directory entry.

**Generic Parameters:**
- 'data

**Variants:**
- `Table(ResourceDirectoryTable<'data>)` - A subtable entry.
- `Data(&'data pe::ImageResourceDataEntry)` - A resource data entry.

**Methods:**

- `fn table(self: Self) -> Option<ResourceDirectoryTable<'data>>` - Converts to an option of table.
- `fn data(self: Self) -> Option<&'data pe::ImageResourceDataEntry>` - Converts to an option of data entry.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ResourceDirectoryEntryData<'data>`



## object::read::pe::resource::ResourceDirectoryTable

*Struct*

A table of resource entries.

**Generic Parameters:**
- 'data

**Fields:**
- `header: &'data pe::ImageResourceDirectory` - The table header.
- `entries: &'data [pe::ImageResourceDirectoryEntry]` - The table entries.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ResourceDirectoryTable<'data>`



## object::read::pe::resource::ResourceName

*Struct*

A resource name.

**Methods:**

- `fn to_string_lossy(self: &Self, directory: ResourceDirectory) -> Result<String>` - Converts to a `String`.
- `fn data<'data>(self: &Self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` - Returns the string unicode buffer.
- `fn raw_data<'data>(self: &Self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` - Returns the string buffer as raw bytes.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ResourceName`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::pe::resource::ResourceNameOrId

*Enum*

A resource name or ID.

Can be either a string or a numeric ID.

**Variants:**
- `Name(ResourceName)` - A resource name.
- `Id(u16)` - A resource ID.

**Methods:**

- `fn name(self: Self) -> Option<ResourceName>` - Converts to an option of name.
- `fn id(self: Self) -> Option<u16>` - Converts to an option of ID.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



