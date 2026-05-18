*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [producers](index.md)*

---

# Module `producers`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProducersField`](#producersfield) | struct | A field from the producers custom section. |
| [`ProducersFieldValue`](#producersfieldvalue) | struct | Represents a field value in the producers custom section. |
| [`ProducersSectionReader`](#producerssectionreader) | type | A reader for the producers custom section of a WebAssembly module. |

## Structs

### `ProducersField<'a>`

```rust
struct ProducersField<'a> {
    pub name: &'a str,
    pub values: crate::SectionLimited<'a, ProducersFieldValue<'a>>,
}
```

A field from the producers custom section.

#### Fields

- **`name`**: `&'a str`

  The name of the field.

- **`values`**: `crate::SectionLimited<'a, ProducersFieldValue<'a>>`

  The values specified for this field

#### Trait Implementations

##### `impl Clone for ProducersField<'a>`

- <span id="producersfield-clone"></span>`fn clone(&self) -> ProducersField<'a>` — [`ProducersField`](../index.md#producersfield)

##### `impl Debug for ProducersField<'a>`

- <span id="producersfield-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for ProducersField<'a>`

- <span id="producersfield-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `ProducersFieldValue<'a>`

```rust
struct ProducersFieldValue<'a> {
    pub name: &'a str,
    pub version: &'a str,
}
```

Represents a field value in the producers custom section.

#### Fields

- **`name`**: `&'a str`

  The field name.

- **`version`**: `&'a str`

  The field version.

#### Trait Implementations

##### `impl Clone for ProducersFieldValue<'a>`

- <span id="producersfieldvalue-clone"></span>`fn clone(&self) -> ProducersFieldValue<'a>` — [`ProducersFieldValue`](../index.md#producersfieldvalue)

##### `impl Copy for ProducersFieldValue<'a>`

##### `impl Debug for ProducersFieldValue<'a>`

- <span id="producersfieldvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for ProducersFieldValue<'a>`

- <span id="producersfieldvalue-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Type Aliases

### `ProducersSectionReader<'a>`

```rust
type ProducersSectionReader<'a> = crate::SectionLimited<'a, ProducersField<'a>>;
```

A reader for the producers custom section of a WebAssembly module.

# Examples

```rust
let data: &[u8] = &[0x01, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65,
    0x02, 0x03, 0x77, 0x61, 0x74, 0x01, 0x31, 0x01, 0x43, 0x03, 0x39, 0x2e, 0x30];
use wasmparser::{ProducersSectionReader, ProducersFieldValue, Result, BinaryReader};
let reader = BinaryReader::new(data, 0);
let reader = ProducersSectionReader::new(reader).expect("producers reader");
let field = reader.into_iter().next().unwrap().expect("producers field");
assert!(field.name == "language");
let value = field.values.into_iter().collect::<Result<Vec<_>>>().expect("values");
assert!(value.len() == 2);
assert!(value[0].name == "wat" && value[0].version == "1");
assert!(value[1].name == "C" && value[1].version == "9.0");
```

