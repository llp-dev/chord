*[serde_json](../../index.md) / [value](../index.md) / [de](index.md)*

---

# Module `de`

## Contents

- [Structs](#structs)
  - [`EnumDeserializer`](#enumdeserializer)
  - [`VariantDeserializer`](#variantdeserializer)
  - [`SeqDeserializer`](#seqdeserializer)
  - [`MapDeserializer`](#mapdeserializer)
  - [`EnumRefDeserializer`](#enumrefdeserializer)
  - [`VariantRefDeserializer`](#variantrefdeserializer)
  - [`SeqRefDeserializer`](#seqrefdeserializer)
  - [`MapRefDeserializer`](#maprefdeserializer)
  - [`MapKeyDeserializer`](#mapkeydeserializer)
  - [`KeyClassifier`](#keyclassifier)
  - [`BorrowedCowStrDeserializer`](#borrowedcowstrdeserializer)
  - [`UnitOnly`](#unitonly)
- [Enums](#enums)
  - [`KeyClass`](#keyclass)
- [Functions](#functions)
  - [`visit_array`](#visit-array)
  - [`visit_array_ref`](#visit-array-ref)
- [Macros](#macros)
  - [`deserialize_number!`](#deserialize-number)
  - [`deserialize_value_ref_number!`](#deserialize-value-ref-number)
  - [`deserialize_numeric_key!`](#deserialize-numeric-key)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EnumDeserializer`](#enumdeserializer) | struct |  |
| [`VariantDeserializer`](#variantdeserializer) | struct |  |
| [`SeqDeserializer`](#seqdeserializer) | struct |  |
| [`MapDeserializer`](#mapdeserializer) | struct |  |
| [`EnumRefDeserializer`](#enumrefdeserializer) | struct |  |
| [`VariantRefDeserializer`](#variantrefdeserializer) | struct |  |
| [`SeqRefDeserializer`](#seqrefdeserializer) | struct |  |
| [`MapRefDeserializer`](#maprefdeserializer) | struct |  |
| [`MapKeyDeserializer`](#mapkeydeserializer) | struct |  |
| [`KeyClassifier`](#keyclassifier) | struct |  |
| [`BorrowedCowStrDeserializer`](#borrowedcowstrdeserializer) | struct |  |
| [`UnitOnly`](#unitonly) | struct |  |
| [`KeyClass`](#keyclass) | enum |  |
| [`visit_array`](#visit-array) | fn |  |
| [`visit_array_ref`](#visit-array-ref) | fn |  |
| [`deserialize_number!`](#deserialize-number) | macro |  |
| [`deserialize_value_ref_number!`](#deserialize-value-ref-number) | macro |  |
| [`deserialize_numeric_key!`](#deserialize-numeric-key) | macro |  |

## Structs

### `EnumDeserializer`

```rust
struct EnumDeserializer {
    variant: alloc::string::String,
    value: Option<crate::value::Value>,
}
```

#### Trait Implementations

##### `impl EnumAccess for EnumDeserializer`

- <span id="enumdeserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="enumdeserializer-enumaccess-type-variant"></span>`type Variant = VariantDeserializer`

- <span id="enumdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, VariantDeserializer), Error>` — [`VariantDeserializer`](#variantdeserializer), [`Error`](../../error/index.md#error)

### `VariantDeserializer`

```rust
struct VariantDeserializer {
    value: Option<crate::value::Value>,
}
```

#### Trait Implementations

##### `impl VariantAccess for VariantDeserializer`

- <span id="variantdeserializer-variantaccess-type-error"></span>`type Error = Error`

- <span id="variantdeserializer-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="variantdeserializer-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantdeserializer-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantdeserializer-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

### `SeqDeserializer`

```rust
struct SeqDeserializer {
    iter: vec::IntoIter<crate::value::Value>,
}
```

#### Implementations

- <span id="seqdeserializer-new"></span>`fn new(vec: Vec<Value>) -> Self` — [`Value`](../index.md#value)

#### Trait Implementations

##### `impl SeqAccess for SeqDeserializer`

- <span id="seqdeserializer-seqaccess-type-error"></span>`type Error = Error`

- <span id="seqdeserializer-seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="seqdeserializer-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `MapDeserializer`

```rust
struct MapDeserializer {
    iter: <crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<crate::value::Value>,
}
```

#### Implementations

- <span id="mapdeserializer-new"></span>`fn new(map: Map<String, Value>) -> Self` — [`Map`](../../map/index.md#map), [`Value`](../index.md#value)

#### Trait Implementations

##### `impl MapAccess for MapDeserializer`

- <span id="mapdeserializer-mapaccess-type-error"></span>`type Error = Error`

- <span id="mapdeserializer-mapaccess-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapdeserializer-mapaccess-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapdeserializer-mapaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `EnumRefDeserializer<'de>`

```rust
struct EnumRefDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de crate::value::Value>,
}
```

#### Trait Implementations

##### `impl EnumAccess for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="enumrefdeserializer-enumaccess-type-variant"></span>`type Variant = VariantRefDeserializer<'de>`

- <span id="enumrefdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, <Self as >::Variant), Error>` — [`Error`](../../error/index.md#error)

### `VariantRefDeserializer<'de>`

```rust
struct VariantRefDeserializer<'de> {
    value: Option<&'de crate::value::Value>,
}
```

#### Trait Implementations

##### `impl VariantAccess for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-variantaccess-type-error"></span>`type Error = Error`

- <span id="variantrefdeserializer-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="variantrefdeserializer-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantrefdeserializer-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantrefdeserializer-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

### `SeqRefDeserializer<'de>`

```rust
struct SeqRefDeserializer<'de> {
    iter: slice::Iter<'de, crate::value::Value>,
}
```

#### Implementations

- <span id="seqrefdeserializer-new"></span>`fn new(slice: &'de [Value]) -> Self` — [`Value`](../index.md#value)

#### Trait Implementations

##### `impl SeqAccess for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-seqaccess-type-error"></span>`type Error = Error`

- <span id="seqrefdeserializer-seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="seqrefdeserializer-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `MapRefDeserializer<'de>`

```rust
struct MapRefDeserializer<'de> {
    iter: <&'de crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<&'de crate::value::Value>,
}
```

#### Implementations

- <span id="maprefdeserializer-new"></span>`fn new(map: &'de Map<String, Value>) -> Self` — [`Map`](../../map/index.md#map), [`Value`](../index.md#value)

#### Trait Implementations

##### `impl MapAccess for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-mapaccess-type-error"></span>`type Error = Error`

- <span id="maprefdeserializer-mapaccess-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="maprefdeserializer-mapaccess-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="maprefdeserializer-mapaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `MapKeyDeserializer<'de>`

```rust
struct MapKeyDeserializer<'de> {
    key: alloc::borrow::Cow<'de, str>,
}
```

#### Trait Implementations

##### `impl Deserializer for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="mapkeydeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

### `KeyClassifier`

```rust
struct KeyClassifier;
```

#### Trait Implementations

##### `impl DeserializeSeed for KeyClassifier`

- <span id="keyclassifier-deserializeseed-type-value"></span>`type Value = KeyClass`

- <span id="keyclassifier-deserializeseed-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

##### `impl Expected for KeyClassifier`

- <span id="keyclassifier-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for KeyClassifier`

- <span id="keyclassifier-visitor-type-value"></span>`type Value = KeyClass`

- <span id="keyclassifier-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="keyclassifier-visitor-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>`

- <span id="keyclassifier-visitor-visit-string"></span>`fn visit_string<E>(self, s: String) -> Result<<Self as >::Value, E>`

### `BorrowedCowStrDeserializer<'de>`

```rust
struct BorrowedCowStrDeserializer<'de> {
    value: alloc::borrow::Cow<'de, str>,
}
```

#### Implementations

- <span id="borrowedcowstrdeserializer-new"></span>`fn new(value: Cow<'de, str>) -> Self`

#### Trait Implementations

##### `impl Deserializer for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl EnumAccess for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="borrowedcowstrdeserializer-enumaccess-type-variant"></span>`type Variant = UnitOnly`

- <span id="borrowedcowstrdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), Error>` — [`Error`](../../error/index.md#error)

### `UnitOnly`

```rust
struct UnitOnly;
```

#### Trait Implementations

##### `impl VariantAccess for UnitOnly`

- <span id="unitonly-variantaccess-type-error"></span>`type Error = Error`

- <span id="unitonly-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="unitonly-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="unitonly-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="unitonly-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

## Enums

### `KeyClass`

```rust
enum KeyClass {
    Map(alloc::string::String),
}
```

## Functions

### `visit_array`

```rust
fn visit_array<'de, V>(array: alloc::vec::Vec<crate::value::Value>, visitor: V) -> Result<<V as >::Value, crate::error::Error>
where
    V: Visitor<'de>
```

### `visit_array_ref`

```rust
fn visit_array_ref<'de, V>(array: &'de [crate::value::Value], visitor: V) -> Result<<V as >::Value, crate::error::Error>
where
    V: Visitor<'de>
```

## Macros

### `deserialize_number!`

### `deserialize_value_ref_number!`

### `deserialize_numeric_key!`

