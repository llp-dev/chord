**serde_json > read**

# Module: read

## Contents

**Structs**

- [`IoRead`](#ioread) - JSON input source that reads from a std::io input stream.
- [`Position`](#position)
- [`SliceRead`](#sliceread) - JSON input source that reads from a slice of bytes.
- [`StrRead`](#strread) - JSON input source that reads from a UTF-8 string.

**Enums**

- [`Reference`](#reference)

**Traits**

- [`Fused`](#fused) - Marker for whether StreamDeserializer can implement FusedIterator.
- [`Read`](#read) - Trait used by the deserializer for iterating over input. This is manually

---

## serde_json::read::Fused

*Trait*

Marker for whether StreamDeserializer can implement FusedIterator.



## serde_json::read::IoRead

*Struct*

JSON input source that reads from a std::io input stream.

**Generic Parameters:**
- R

**Methods:**

- `fn new(reader: R) -> Self` - Create a JSON input source to read from a std::io input stream.

**Traits:** Read



## serde_json::read::Position

*Struct*

**Fields:**
- `line: usize`
- `column: usize`



## serde_json::read::Read

*Trait*

Trait used by the deserializer for iterating over input. This is manually
"specialized" for iterating over `&[u8]`. Once feature(specialization) is
stable we can use actual specialization.

This trait is sealed and cannot be implemented for types outside of
`serde_json`.



## serde_json::read::Reference

*Enum*

**Generic Parameters:**
- 'b
- 'c
- T

**Variants:**
- `Borrowed(&'b T)`
- `Copied(&'c T)`



## serde_json::read::SliceRead

*Struct*

JSON input source that reads from a slice of bytes.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(slice: &'a [u8]) -> Self` - Create a JSON input source to read from a slice of bytes.

**Traits:** Read



## serde_json::read::StrRead

*Struct*

JSON input source that reads from a UTF-8 string.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(s: &'a str) -> Self` - Create a JSON input source to read from a UTF-8 string.

**Traits:** Read



