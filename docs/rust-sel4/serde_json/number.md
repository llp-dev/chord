**serde_json > number**

# Module: number

## Contents

**Structs**

- [`Number`](#number) - Represents a JSON number, whether integer or floating point.

---

## serde_json::number::Number

*Struct*

Represents a JSON number, whether integer or floating point.

**Methods:**

- `fn is_i64(self: &Self) -> bool` - Returns true if the `Number` is an integer between `i64::MIN` and
- `fn is_u64(self: &Self) -> bool` - Returns true if the `Number` is an integer between zero and `u64::MAX`.
- `fn is_f64(self: &Self) -> bool` - Returns true if the `Number` can be represented by f64.
- `fn as_i64(self: &Self) -> Option<i64>` - If the `Number` is an integer, represent it as i64 if possible. Returns
- `fn as_u64(self: &Self) -> Option<u64>` - If the `Number` is an integer, represent it as u64 if possible. Returns
- `fn as_f64(self: &Self) -> Option<f64>` - Represents the number as f64 if possible. Returns None otherwise.
- `fn from_f64(f: f64) -> Option<Number>` - Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON
- `fn as_i128(self: &Self) -> Option<i128>` - If the `Number` is an integer, represent it as i128 if possible. Returns
- `fn as_u128(self: &Self) -> Option<u128>` - If the `Number` is an integer, represent it as u128 if possible. Returns
- `fn from_i128(i: i128) -> Option<Number>` - Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or
- `fn from_u128(i: u128) -> Option<Number>` - Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Number`
- **From**
  - `fn from(i: isize) -> Self`
- **From**
  - `fn from(i: i32) -> Self`
- **From**
  - `fn from(i: i8) -> Self`
- **From**
  - `fn from(u: u64) -> Self`
- **From**
  - `fn from(u: u16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Number) -> bool`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **FromStr**
  - `fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`
- **From**
  - `fn from(i: i64) -> Self`
- **From**
  - `fn from(i: i16) -> Self`
- **From**
  - `fn from(u: usize) -> Self`
- **From**
  - `fn from(u: u32) -> Self`
- **From**
  - `fn from(u: u8) -> Self`
- **Deserializer**
  - `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`



