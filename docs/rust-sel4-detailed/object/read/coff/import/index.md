*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [import](index.md)*

---

# Module `import`

Support for reading short import files.

These are used by some Windows linkers as a more compact way to describe
dynamically imported symbols.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImportFile`](#importfile) | struct | A Windows short form description of a symbol to import. |
| [`ImportObjectData`](#importobjectdata) | struct | The data following [`pe::ImportObjectHeader`]. |
| [`ImportName`](#importname) | enum | The name or ordinal to import from a DLL. |
| [`ImportType`](#importtype) | enum | The kind of import symbol. |

## Structs

### `ImportFile<'data>`

```rust
struct ImportFile<'data> {
    header: &'data pe::ImportObjectHeader,
    kind: ImportType,
    dll: crate::read::ByteString<'data>,
    symbol: crate::read::ByteString<'data>,
    import: Option<crate::read::ByteString<'data>>,
}
```

A Windows short form description of a symbol to import.

Used in Windows import libraries to provide a mapping from
a symbol name to a DLL export. This is not an object file.

This is a file that starts with [`pe::ImportObjectHeader`](../../../pe/index.md), and corresponds
to [`crate::FileKind::CoffImport`](../../../index.md).

#### Implementations

- <span id="importfile-parse"></span>`fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse it.

- <span id="importfile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

  Get the machine type.

- <span id="importfile-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md#subarchitecture)

  Get the sub machine type, if available.

- <span id="importfile-symbol"></span>`fn symbol(&self) -> &'data [u8]`

  The public symbol name.

- <span id="importfile-dll"></span>`fn dll(&self) -> &'data [u8]`

  The name of the DLL to import the symbol from.

- <span id="importfile-import"></span>`fn import(&self) -> ImportName<'data>` — [`ImportName`](../index.md#importname)

  The name exported from the DLL.

- <span id="importfile-import-type"></span>`fn import_type(&self) -> ImportType` — [`ImportType`](../index.md#importtype)

  The type of import. Usually either a function or data.

#### Trait Implementations

##### `impl Clone for ImportFile<'data>`

- <span id="importfile-clone"></span>`fn clone(&self) -> ImportFile<'data>` — [`ImportFile`](../index.md#importfile)

##### `impl Debug for ImportFile<'data>`

- <span id="importfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportObjectData<'data>`

```rust
struct ImportObjectData<'data> {
    symbol: crate::read::ByteString<'data>,
    dll: crate::read::ByteString<'data>,
    export: Option<crate::read::ByteString<'data>>,
}
```

The data following [`pe::ImportObjectHeader`](../../../pe/index.md).

#### Implementations

- <span id="importobjectdata-symbol"></span>`fn symbol(&self) -> &'data [u8]`

  The public symbol name.

- <span id="importobjectdata-dll"></span>`fn dll(&self) -> &'data [u8]`

  The name of the DLL to import the symbol from.

- <span id="importobjectdata-export"></span>`fn export(&self) -> Option<&'data [u8]>`

  The name exported from the DLL.

  

  This is only set if the name is not derived from the symbol name.

#### Trait Implementations

##### `impl Clone for ImportObjectData<'data>`

- <span id="importobjectdata-clone"></span>`fn clone(&self) -> ImportObjectData<'data>` — [`ImportObjectData`](../index.md#importobjectdata)

##### `impl Debug for ImportObjectData<'data>`

- <span id="importobjectdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ImportName<'data>`

```rust
enum ImportName<'data> {
    Ordinal(u16),
    Name(&'data [u8]),
}
```

The name or ordinal to import from a DLL.

#### Variants

- **`Ordinal`**

  Import by ordinal. Ordinarily this is a 1-based index.

- **`Name`**

  Import by name.

#### Trait Implementations

##### `impl Clone for ImportName<'data>`

- <span id="importname-clone"></span>`fn clone(&self) -> ImportName<'data>` — [`ImportName`](../index.md#importname)

##### `impl Copy for ImportName<'data>`

##### `impl Debug for ImportName<'data>`

- <span id="importname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImportName<'data>`

##### `impl<K> Equivalent for ImportName<'data>`

- <span id="importname-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for ImportName<'data>`

- <span id="importname-partialeq-eq"></span>`fn eq(&self, other: &ImportName<'data>) -> bool` — [`ImportName`](../index.md#importname)

##### `impl StructuralPartialEq for ImportName<'data>`

### `ImportType`

```rust
enum ImportType {
    Code,
    Data,
    Const,
}
```

The kind of import symbol.

#### Variants

- **`Code`**

  An executable code symbol.

- **`Data`**

  A data symbol.

- **`Const`**

  A constant value.

#### Trait Implementations

##### `impl Clone for ImportType`

- <span id="importtype-clone"></span>`fn clone(&self) -> ImportType` — [`ImportType`](../index.md#importtype)

##### `impl Copy for ImportType`

##### `impl Debug for ImportType`

- <span id="importtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImportType`

##### `impl<K> Equivalent for ImportType`

- <span id="importtype-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ImportType`

- <span id="importtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ImportType`

- <span id="importtype-partialeq-eq"></span>`fn eq(&self, other: &ImportType) -> bool` — [`ImportType`](../index.md#importtype)

##### `impl StructuralPartialEq for ImportType`

