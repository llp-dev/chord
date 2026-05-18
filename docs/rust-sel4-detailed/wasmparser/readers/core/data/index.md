*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [data](index.md)*

---

# Module `data`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Data`](#data) | struct | Represents a data segment in a core WebAssembly module. |
| [`DataKind`](#datakind) | enum | The kind of data segment. |
| [`DataSectionReader`](#datasectionreader) | type | A reader for the data section of a WebAssembly module. |

## Structs

### `Data<'a>`

```rust
struct Data<'a> {
    pub kind: DataKind<'a>,
    pub data: &'a [u8],
    pub range: core::ops::Range<usize>,
}
```

Represents a data segment in a core WebAssembly module.

#### Fields

- **`kind`**: `DataKind<'a>`

  The kind of data segment.

- **`data`**: `&'a [u8]`

  The data of the data segment.

- **`range`**: `core::ops::Range<usize>`

  The range of the data segment.

#### Trait Implementations

##### `impl Clone for Data<'a>`

- <span id="data-clone"></span>`fn clone(&self) -> Data<'a>` — [`Data`](../index.md#data)

##### `impl Debug for Data<'a>`

- <span id="data-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Data<'a>`

- <span id="data-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Enums

### `DataKind<'a>`

```rust
enum DataKind<'a> {
    Passive,
    Active {
        memory_index: u32,
        offset_expr: crate::ConstExpr<'a>,
    },
}
```

The kind of data segment.

#### Variants

- **`Passive`**

  The data segment is passive.

- **`Active`**

  The data segment is active.

#### Trait Implementations

##### `impl Clone for DataKind<'a>`

- <span id="datakind-clone"></span>`fn clone(&self) -> DataKind<'a>` — [`DataKind`](../index.md#datakind)

##### `impl Debug for DataKind<'a>`

- <span id="datakind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `DataSectionReader<'a>`

```rust
type DataSectionReader<'a> = crate::SectionLimited<'a, Data<'a>>;
```

A reader for the data section of a WebAssembly module.

