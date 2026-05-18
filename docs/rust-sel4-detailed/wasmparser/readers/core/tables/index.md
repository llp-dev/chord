*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [tables](index.md)*

---

# Module `tables`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Table`](#table) | struct | Type information about a table defined in the table section of a WebAssembly module. |
| [`TableInit`](#tableinit) | enum | Different modes of initializing a table. |
| [`TableSectionReader`](#tablesectionreader) | type | A reader for the table section of a WebAssembly module. |

## Structs

### `Table<'a>`

```rust
struct Table<'a> {
    pub ty: crate::TableType,
    pub init: TableInit<'a>,
}
```

Type information about a table defined in the table section of a WebAssembly
module.

#### Fields

- **`ty`**: `crate::TableType`

  The type of this table, including its element type and its limits.

- **`init`**: `TableInit<'a>`

  The initialization expression for the table.

#### Trait Implementations

##### `impl Clone for Table<'a>`

- <span id="table-clone"></span>`fn clone(&self) -> Table<'a>` — [`Table`](../index.md#table)

##### `impl Debug for Table<'a>`

- <span id="table-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Table<'a>`

- <span id="table-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Enums

### `TableInit<'a>`

```rust
enum TableInit<'a> {
    RefNull,
    Expr(crate::ConstExpr<'a>),
}
```

Different modes of initializing a table.

#### Variants

- **`RefNull`**

  The table is initialized to all null elements.

- **`Expr`**

  Each element in the table is initialized with the specified constant
  expression.

#### Trait Implementations

##### `impl Clone for TableInit<'a>`

- <span id="tableinit-clone"></span>`fn clone(&self) -> TableInit<'a>` — [`TableInit`](../index.md#tableinit)

##### `impl Debug for TableInit<'a>`

- <span id="tableinit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `TableSectionReader<'a>`

```rust
type TableSectionReader<'a> = crate::SectionLimited<'a, Table<'a>>;
```

A reader for the table section of a WebAssembly module.

