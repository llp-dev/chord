**serde_json > value > ser**

# Module: value::ser

## Contents

**Structs**

- [`SerializeStructVariant`](#serializestructvariant)
- [`SerializeTupleVariant`](#serializetuplevariant)
- [`SerializeVec`](#serializevec)
- [`Serializer`](#serializer) - Serializer whose output is a `Value`.

**Enums**

- [`SerializeMap`](#serializemap)

---

## serde_json::value::ser::SerializeMap

*Enum*

**Variants:**
- `Map{ map: crate::map::Map<alloc::string::String, crate::value::Value>, next_key: Option<alloc::string::String> }`



## serde_json::value::ser::SerializeStructVariant

*Struct*



## serde_json::value::ser::SerializeTupleVariant

*Struct*



## serde_json::value::ser::SerializeVec

*Struct*



## serde_json::value::ser::Serializer

*Struct*

Serializer whose output is a `Value`.

This is the serializer that backs [`serde_json::to_value`][crate::to_value].
Unlike the main serde_json serializer which goes from some serializable
value of type `T` to JSON text, this one goes from `T` to
`serde_json::Value`.

The `to_value` function is implementable as:

```
use serde::Serialize;
use serde_json::{Error, Value};

pub fn to_value<T>(input: T) -> Result<Value, Error>
where
    T: Serialize,
{
    input.serialize(serde_json::value::Serializer)
}
```

**Unit Struct**

**Trait Implementations:**

- **Serializer**
  - `fn serialize_bool(self: Self, value: bool) -> Result<Value>`
  - `fn serialize_i8(self: Self, value: i8) -> Result<Value>`
  - `fn serialize_i16(self: Self, value: i16) -> Result<Value>`
  - `fn serialize_i32(self: Self, value: i32) -> Result<Value>`
  - `fn serialize_i64(self: Self, value: i64) -> Result<Value>`
  - `fn serialize_i128(self: Self, value: i128) -> Result<Value>`
  - `fn serialize_u8(self: Self, value: u8) -> Result<Value>`
  - `fn serialize_u16(self: Self, value: u16) -> Result<Value>`
  - `fn serialize_u32(self: Self, value: u32) -> Result<Value>`
  - `fn serialize_u64(self: Self, value: u64) -> Result<Value>`
  - `fn serialize_u128(self: Self, value: u128) -> Result<Value>`
  - `fn serialize_f32(self: Self, float: f32) -> Result<Value>`
  - `fn serialize_f64(self: Self, float: f64) -> Result<Value>`
  - `fn serialize_char(self: Self, value: char) -> Result<Value>`
  - `fn serialize_str(self: Self, value: &str) -> Result<Value>`
  - `fn serialize_bytes(self: Self, value: &[u8]) -> Result<Value>`
  - `fn serialize_unit(self: Self) -> Result<Value>`
  - `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result<Value>`
  - `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>`
  - `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<Value>`
  - `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>`
  - `fn serialize_none(self: Self) -> Result<Value>`
  - `fn serialize_some<T>(self: Self, value: &T) -> Result<Value>`
  - `fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>`
  - `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple>`
  - `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>`
  - `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>`
  - `fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap>`
  - `fn serialize_struct(self: Self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>`
  - `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>`
  - `fn collect_str<T>(self: Self, value: &T) -> Result<Value>`



