*[serde_json](../index.md) / [number](index.md)*

---

# Module `number`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Number`](#number) | struct | Represents a JSON number, whether integer or floating point. |
| [`N`](#n) | enum |  |
| [`deserialize_any!`](#deserialize-any) | macro |  |
| [`deserialize_number!`](#deserialize-number) | macro |  |
| [`impl_from_unsigned!`](#impl-from-unsigned) | macro |  |
| [`impl_from_signed!`](#impl-from-signed) | macro |  |

## Structs

### `Number`

```rust
struct Number {
    n: N,
}
```

Represents a JSON number, whether integer or floating point.

#### Implementations

- <span id="number-is-i64"></span>`fn is_i64(&self) -> bool`

  Returns true if the `Number` is an integer between `i64::MIN` and

  `i64::MAX`.

  

  For any Number on which `is_i64` returns true, `as_i64` is guaranteed to

  return the integer value.

- <span id="number-is-u64"></span>`fn is_u64(&self) -> bool`

  Returns true if the `Number` is an integer between zero and `u64::MAX`.

  

  For any Number on which `is_u64` returns true, `as_u64` is guaranteed to

  return the integer value.

- <span id="number-is-f64"></span>`fn is_f64(&self) -> bool`

  Returns true if the `Number` can be represented by f64.

  

  For any Number on which `is_f64` returns true, `as_f64` is guaranteed to

  return the floating point value.

  

  Currently this function returns true if and only if both `is_i64` and

  `is_u64` return false but this is not a guarantee in the future.

- <span id="number-as-i64"></span>`fn as_i64(&self) -> Option<i64>`

  If the `Number` is an integer, represent it as i64 if possible. Returns

  None otherwise.

- <span id="number-as-u64"></span>`fn as_u64(&self) -> Option<u64>`

  If the `Number` is an integer, represent it as u64 if possible. Returns

  None otherwise.

- <span id="number-as-f64"></span>`fn as_f64(&self) -> Option<f64>`

  Represents the number as f64 if possible. Returns None otherwise.

- <span id="number-from-f64"></span>`fn from_f64(f: f64) -> Option<Number>` — [`Number`](#number)

  Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON

  numbers.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_f64(256.0).is_some());

  

  assert!(Number::from_f64(f64::NAN).is_none());

  ```

- <span id="number-as-i128"></span>`fn as_i128(&self) -> Option<i128>`

  If the `Number` is an integer, represent it as i128 if possible. Returns

  None otherwise.

- <span id="number-as-u128"></span>`fn as_u128(&self) -> Option<u128>`

  If the `Number` is an integer, represent it as u128 if possible. Returns

  None otherwise.

- <span id="number-from-i128"></span>`fn from_i128(i: i128) -> Option<Number>` — [`Number`](#number)

  Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or

  larger than u64::MAX can only be represented in `Number` if serde_json's

  "arbitrary_precision" feature is enabled.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_i128(256).is_some());

  ```

- <span id="number-from-u128"></span>`fn from_u128(i: u128) -> Option<Number>` — [`Number`](#number)

  Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only

  be represented in `Number` if serde_json's "arbitrary_precision" feature

  is enabled.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_u128(256).is_some());

  ```

- <span id="number-as-f32"></span>`fn as_f32(&self) -> Option<f32>`

- <span id="number-from-f32"></span>`fn from_f32(f: f32) -> Option<Number>` — [`Number`](#number)

#### Trait Implementations

##### `impl Clone for Number`

- <span id="number-clone"></span>`fn clone(&self) -> Number` — [`Number`](#number)

##### `impl Debug for Number`

- <span id="number-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Number`

- <span id="number-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](#number)

##### `impl DeserializeOwned for Number`

##### `impl Deserializer for Number`

- <span id="number-deserializer-type-error"></span>`type Error = Error`

- <span id="number-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Display for Number`

- <span id="number-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Number`

##### `impl FromStr for crate::number::Number`

- <span id="cratenumbernumber-fromstr-type-err"></span>`type Err = Error`

- <span id="cratenumbernumber-fromstr-from-str"></span>`fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- <span id="number-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Number`

- <span id="number-partialeq-eq"></span>`fn eq(&self, other: &Number) -> bool` — [`Number`](#number)

##### `impl Serialize for Number`

- <span id="number-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl ToString for Number`

- <span id="number-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `N`

```rust
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}
```

#### Variants

- **`NegInt`**

  Always less than zero.

- **`Float`**

  Always finite.

#### Trait Implementations

##### `impl Clone for N`

- <span id="n-clone"></span>`fn clone(&self) -> N` — [`N`](#n)

##### `impl Copy for N`

##### `impl Eq for N`

##### `impl Hash for N`

- <span id="n-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl PartialEq for N`

- <span id="n-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Macros

### `deserialize_any!`

### `deserialize_number!`

### `impl_from_unsigned!`

### `impl_from_signed!`

