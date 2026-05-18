*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [elements](index.md)*

---

# Module `elements`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Element`](#element) | struct | Represents a core WebAssembly element segment. |
| [`ElementKind`](#elementkind) | enum | The kind of element segment. |
| [`ElementItems`](#elementitems) | enum | Represents the items of an element segment. |
| [`ElementSectionReader`](#elementsectionreader) | type | A reader for the element section of a WebAssembly module. |

## Structs

### `Element<'a>`

```rust
struct Element<'a> {
    pub kind: ElementKind<'a>,
    pub items: ElementItems<'a>,
    pub range: core::ops::Range<usize>,
}
```

Represents a core WebAssembly element segment.

#### Fields

- **`kind`**: `ElementKind<'a>`

  The kind of the element segment.

- **`items`**: `ElementItems<'a>`

  The initial elements of the element segment.

- **`range`**: `core::ops::Range<usize>`

  The range of the the element segment.

#### Trait Implementations

##### `impl Clone for Element<'a>`

- <span id="element-clone"></span>`fn clone(&self) -> Element<'a>` — [`Element`](../index.md#element)

##### `impl FromReader for Element<'a>`

- <span id="element-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Enums

### `ElementKind<'a>`

```rust
enum ElementKind<'a> {
    Passive,
    Active {
        table_index: Option<u32>,
        offset_expr: crate::ConstExpr<'a>,
    },
    Declared,
}
```

The kind of element segment.

#### Variants

- **`Passive`**

  The element segment is passive.

- **`Active`**

  The element segment is active.

- **`Declared`**

  The element segment is declared.

#### Trait Implementations

##### `impl Clone for ElementKind<'a>`

- <span id="elementkind-clone"></span>`fn clone(&self) -> ElementKind<'a>` — [`ElementKind`](../index.md#elementkind)

### `ElementItems<'a>`

```rust
enum ElementItems<'a> {
    Functions(crate::SectionLimited<'a, u32>),
    Expressions(crate::RefType, crate::SectionLimited<'a, crate::ConstExpr<'a>>),
}
```

Represents the items of an element segment.

#### Variants

- **`Functions`**

  This element contains function indices.

- **`Expressions`**

  This element contains constant expressions used to initialize the table.

#### Trait Implementations

##### `impl Clone for ElementItems<'a>`

- <span id="elementitems-clone"></span>`fn clone(&self) -> ElementItems<'a>` — [`ElementItems`](../index.md#elementitems)

## Type Aliases

### `ElementSectionReader<'a>`

```rust
type ElementSectionReader<'a> = crate::SectionLimited<'a, Element<'a>>;
```

A reader for the element section of a WebAssembly module.

