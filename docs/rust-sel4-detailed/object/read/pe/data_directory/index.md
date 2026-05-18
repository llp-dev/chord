*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [data_directory](index.md)*

---

# Module `data_directory`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DataDirectories`](#datadirectories) | struct | The table of data directories in a PE file. |

## Structs

### `DataDirectories<'data>`

```rust
struct DataDirectories<'data> {
    entries: &'data [pe::ImageDataDirectory],
}
```

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

#### Implementations

- <span id="datadirectories-parse"></span>`fn parse(data: &'data [u8], number: u32) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the data directory table.

  

  `data` must be the remaining optional data following the

  [optional header](pe::ImageOptionalHeader64).  `number` must be from the

  [`number_of_rva_and_sizes`](pe::ImageOptionalHeader64::number_of_rva_and_sizes)

  field of the optional header.

- <span id="datadirectories-len"></span>`fn len(&self) -> usize`

  The number of data directories.

- <span id="datadirectories-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

  Iterator over the data directories.

- <span id="datadirectories-enumerate"></span>`fn enumerate(&self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

  Iterator which gives the directories as well as their index (one of the IMAGE_DIRECTORY_ENTRY_* constants).

- <span id="datadirectories-get"></span>`fn get(&self, index: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

  Returns the data directory at the given index.

  

  Index should be one of the `IMAGE_DIRECTORY_ENTRY_*` constants.

  

  Returns `None` if the index is larger than the table size,

  or if the entry at the index has a zero virtual address.

- <span id="datadirectories-export-directory"></span>`fn export_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ImageExportDirectory`](../../../pe/index.md#imageexportdirectory)

  Returns the unparsed export directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-export-table"></span>`fn export_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ExportTable`](../index.md#exporttable)

  Returns the partially parsed export directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-import-table"></span>`fn import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ImportTable`](../index.md#importtable)

  Returns the partially parsed import directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-delay-load-import-table"></span>`fn delay_load_import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`DelayLoadImportTable`](../index.md#delayloadimporttable)

  Returns the partially parsed delay-load import directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-relocation-blocks"></span>`fn relocation_blocks<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`RelocationBlockIterator`](../index.md#relocationblockiterator)

  Returns the blocks in the base relocation directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-resource-directory"></span>`fn resource_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ResourceDirectory`](../index.md#resourcedirectory)

  Returns the resource directory.

  

  `data` must be the entire file data.

#### Trait Implementations

##### `impl Clone for DataDirectories<'data>`

- <span id="datadirectories-clone"></span>`fn clone(&self) -> DataDirectories<'data>` — [`DataDirectories`](../index.md#datadirectories)

##### `impl Copy for DataDirectories<'data>`

##### `impl Debug for DataDirectories<'data>`

- <span id="datadirectories-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

