**object > read > pe > data_directory**

# Module: read::pe::data_directory

## Contents

**Structs**

- [`DataDirectories`](#datadirectories) - The table of data directories in a PE file.

---

## object::read::pe::data_directory::DataDirectories

*Struct*

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

**Generic Parameters:**
- 'data

**Methods:**

- `fn parse(data: &'data [u8], number: u32) -> Result<Self>` - Parse the data directory table.
- `fn len(self: &Self) -> usize` - The number of data directories.
- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageDataDirectory>` - Iterator over the data directories.
- `fn enumerate(self: &Self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` - Iterator which gives the directories as well as their index (one of the IMAGE_DIRECTORY_ENTRY_* constants).
- `fn get(self: &Self, index: usize) -> Option<&'data pe::ImageDataDirectory>` - Returns the data directory at the given index.
- `fn export_directory<R>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` - Returns the unparsed export directory.
- `fn export_table<R>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` - Returns the partially parsed export directory.
- `fn import_table<R>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` - Returns the partially parsed import directory.
- `fn delay_load_import_table<R>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` - Returns the partially parsed delay-load import directory.
- `fn relocation_blocks<R>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` - Returns the blocks in the base relocation directory.
- `fn resource_directory<R>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` - Returns the resource directory.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DataDirectories<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



