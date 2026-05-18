*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [resource](index.md)*

---

# Module `resource`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ResourceDirectory`](#resourcedirectory) | struct | The `.rsrc` section of a PE file. |
| [`ResourceDirectoryTable`](#resourcedirectorytable) | struct | A table of resource entries. |
| [`ResourceName`](#resourcename) | struct | A resource name. |
| [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata) | enum | Data associated with a resource directory entry. |
| [`ResourceNameOrId`](#resourcenameorid) | enum | A resource name or ID. |

## Structs

### `ResourceDirectory<'data>`

```rust
struct ResourceDirectory<'data> {
    data: &'data [u8],
}
```

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

#### Implementations

- <span id="resourcedirectory-new"></span>`fn new(data: &'data [u8]) -> Self`

  Construct from the data of the `.rsrc` section.

- <span id="resourcedirectory-root"></span>`fn root(&self) -> Result<ResourceDirectoryTable<'data>>` — [`Result`](../../../index.md#result), [`ResourceDirectoryTable`](../index.md#resourcedirectorytable)

  Parses the root resource directory.

#### Trait Implementations

##### `impl Clone for ResourceDirectory<'data>`

- <span id="resourcedirectory-clone"></span>`fn clone(&self) -> ResourceDirectory<'data>` — [`ResourceDirectory`](../index.md#resourcedirectory)

##### `impl Copy for ResourceDirectory<'data>`

##### `impl Debug for ResourceDirectory<'data>`

- <span id="resourcedirectory-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceDirectoryTable<'data>`

```rust
struct ResourceDirectoryTable<'data> {
    pub header: &'data pe::ImageResourceDirectory,
    pub entries: &'data [pe::ImageResourceDirectoryEntry],
}
```

A table of resource entries.

#### Fields

- **`header`**: `&'data pe::ImageResourceDirectory`

  The table header.

- **`entries`**: `&'data [pe::ImageResourceDirectoryEntry]`

  The table entries.

#### Implementations

- <span id="resourcedirectorytable-parse"></span>`fn parse(data: &'data [u8], offset: u32) -> Result<Self>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Clone for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-clone"></span>`fn clone(&self) -> ResourceDirectoryTable<'data>` — [`ResourceDirectoryTable`](../index.md#resourcedirectorytable)

##### `impl Debug for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceName`

```rust
struct ResourceName {
    offset: u32,
}
```

A resource name.

#### Implementations

- <span id="resourcename-to-string-lossy"></span>`fn to_string_lossy(&self, directory: ResourceDirectory<'_>) -> Result<String>` — [`ResourceDirectory`](../index.md#resourcedirectory), [`Result`](../../../index.md#result)

  Converts to a `String`.

- <span id="resourcename-data"></span>`fn data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` — [`ResourceDirectory`](../index.md#resourcedirectory), [`Result`](../../../index.md#result), [`U16Bytes`](../../../index.md#u16bytes), [`LittleEndian`](../../../index.md#littleendian)

  Returns the string unicode buffer.

- <span id="resourcename-raw-data"></span>`fn raw_data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` — [`ResourceDirectory`](../index.md#resourcedirectory), [`Result`](../../../index.md#result)

  Returns the string buffer as raw bytes.

#### Trait Implementations

##### `impl Clone for ResourceName`

- <span id="resourcename-clone"></span>`fn clone(&self) -> ResourceName` — [`ResourceName`](../index.md#resourcename)

##### `impl Copy for ResourceName`

##### `impl Debug for ResourceName`

- <span id="resourcename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ResourceDirectoryEntryData<'data>`

```rust
enum ResourceDirectoryEntryData<'data> {
    Table(ResourceDirectoryTable<'data>),
    Data(&'data pe::ImageResourceDataEntry),
}
```

Data associated with a resource directory entry.

#### Variants

- **`Table`**

  A subtable entry.

- **`Data`**

  A resource data entry.

#### Implementations

- <span id="resourcedirectoryentrydata-table"></span>`fn table(self) -> Option<ResourceDirectoryTable<'data>>` — [`ResourceDirectoryTable`](../index.md#resourcedirectorytable)

  Converts to an option of table.

  

  Helper for iterator filtering.

- <span id="resourcedirectoryentrydata-data"></span>`fn data(self) -> Option<&'data pe::ImageResourceDataEntry>` — [`ImageResourceDataEntry`](../../../pe/index.md#imageresourcedataentry)

  Converts to an option of data entry.

  

  Helper for iterator filtering.

#### Trait Implementations

##### `impl Clone for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-clone"></span>`fn clone(&self) -> ResourceDirectoryEntryData<'data>` — [`ResourceDirectoryEntryData`](../index.md#resourcedirectoryentrydata)

##### `impl Debug for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceNameOrId`

```rust
enum ResourceNameOrId {
    Name(ResourceName),
    Id(u16),
}
```

A resource name or ID.

Can be either a string or a numeric ID.

#### Variants

- **`Name`**

  A resource name.

- **`Id`**

  A resource ID.

#### Implementations

- <span id="resourcenameorid-name"></span>`fn name(self) -> Option<ResourceName>` — [`ResourceName`](../index.md#resourcename)

  Converts to an option of name.

  

  Helper for iterator filtering.

- <span id="resourcenameorid-id"></span>`fn id(self) -> Option<u16>`

  Converts to an option of ID.

  

  Helper for iterator filtering.

#### Trait Implementations

##### `impl Debug for ResourceNameOrId`

- <span id="resourcenameorid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

