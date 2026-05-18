**serde_json > value**

# Module: value

## Contents

**Enums**

- [`Value`](#value) - Represents any valid JSON value.

**Functions**

- [`from_value`](#from_value) - Interpret a `serde_json::Value` as an instance of type `T`.
- [`to_value`](#to_value) - Convert a `T` into `serde_json::Value` which is an enum that can represent

---

## serde_json::value::Value

*Enum*

Represents any valid JSON value.

See the [`serde_json::value` module documentation](self) for usage examples.

**Variants:**
- `Null` - Represents a JSON null value.
- `Bool(bool)` - Represents a JSON boolean.
- `Number(Number)` - Represents a JSON number, whether integer or floating point.
- `String(alloc::string::String)` - Represents a JSON string.
- `Array(alloc::vec::Vec<Value>)` - Represents a JSON array.
- `Object(Map<alloc::string::String, Value>)` - Represents a JSON object.

**Methods:**

- `fn get<I>(self: &Self, index: I) -> Option<&Value>` - Index into a JSON array or map. A string index can be used to access a
- `fn get_mut<I>(self: & mut Self, index: I) -> Option<& mut Value>` - Mutably index into a JSON array or map. A string index can be used to
- `fn is_object(self: &Self) -> bool` - Returns true if the `Value` is an Object. Returns false otherwise.
- `fn as_object(self: &Self) -> Option<&Map<String, Value>>` - If the `Value` is an Object, returns the associated Map. Returns None
- `fn as_object_mut(self: & mut Self) -> Option<& mut Map<String, Value>>` - If the `Value` is an Object, returns the associated mutable Map.
- `fn is_array(self: &Self) -> bool` - Returns true if the `Value` is an Array. Returns false otherwise.
- `fn as_array(self: &Self) -> Option<&Vec<Value>>` - If the `Value` is an Array, returns the associated vector. Returns None
- `fn as_array_mut(self: & mut Self) -> Option<& mut Vec<Value>>` - If the `Value` is an Array, returns the associated mutable vector.
- `fn is_string(self: &Self) -> bool` - Returns true if the `Value` is a String. Returns false otherwise.
- `fn as_str(self: &Self) -> Option<&str>` - If the `Value` is a String, returns the associated str. Returns None
- `fn is_number(self: &Self) -> bool` - Returns true if the `Value` is a Number. Returns false otherwise.
- `fn as_number(self: &Self) -> Option<&Number>` - If the `Value` is a Number, returns the associated [`Number`]. Returns
- `fn is_i64(self: &Self) -> bool` - Returns true if the `Value` is an integer between `i64::MIN` and
- `fn is_u64(self: &Self) -> bool` - Returns true if the `Value` is an integer between zero and `u64::MAX`.
- `fn is_f64(self: &Self) -> bool` - Returns true if the `Value` is a number that can be represented by f64.
- `fn as_i64(self: &Self) -> Option<i64>` - If the `Value` is an integer, represent it as i64 if possible. Returns
- `fn as_u64(self: &Self) -> Option<u64>` - If the `Value` is an integer, represent it as u64 if possible. Returns
- `fn as_f64(self: &Self) -> Option<f64>` - If the `Value` is a number, represent it as f64 if possible. Returns
- `fn is_boolean(self: &Self) -> bool` - Returns true if the `Value` is a Boolean. Returns false otherwise.
- `fn as_bool(self: &Self) -> Option<bool>` - If the `Value` is a Boolean, returns the associated bool. Returns None
- `fn is_null(self: &Self) -> bool` - Returns true if the `Value` is a Null. Returns false otherwise.
- `fn as_null(self: &Self) -> Option<()>` - If the `Value` is a Null, returns (). Returns None otherwise.
- `fn pointer(self: &Self, pointer: &str) -> Option<&Value>` - Looks up a value by a JSON Pointer.
- `fn pointer_mut(self: & mut Self, pointer: &str) -> Option<& mut Value>` - Looks up a value by a JSON Pointer and returns a mutable reference to
- `fn take(self: & mut Self) -> Value` - Takes the value out of the `Value`, leaving a `Null` in its place.
- `fn sort_all_objects(self: & mut Self)` - Reorders the entries of all `Value::Object` nested within this JSON

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &isize) -> bool`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Value) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &i64) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Value`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Value, Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &i32) -> bool`
- **From**
  - `fn from(f: &str) -> Self` - Convert string slice to `Value::String`.
- **From**
  - `fn from(f: bool) -> Self` - Convert boolean to `Value::Bool`.
- **PartialEq**
  - `fn eq(self: &Self, other: &i16) -> bool`
- **From**
  - `fn from(opt: Option<T>) -> Self`
- **From**
  - `fn from(f: f32) -> Self` - Convert 32-bit floating point number to `Value::Number`, or
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self` - Create a `Value::Object` by collecting an iterator of key-value pairs.
- **From**
  - `fn from(n: u64) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &i8) -> bool`
- **From**
  - `fn from(f: &[T]) -> Self` - Convert a slice to `Value::Array`.
- **From**
  - `fn from(n: u16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &String) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &bool) -> bool`
- **From**
  - `fn from(f: Vec<T>) -> Self` - Convert a `Vec` to `Value::Array`.
- **From**
  - `fn from(n: isize) -> Self`
- **From**
  - `fn from(f: Cow<'a, str>) -> Self` - Convert copy-on-write string to `Value::String`.
- **From**
  - `fn from(n: i32) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &f64) -> bool`
- **From**
  - `fn from(n: i8) -> Self`
- **Index**
  - `fn index(self: &Self, index: I) -> &Value` - Index into a `serde_json::Value` using the syntax `value[0]` or
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Display a JSON value as a string.
- **PartialEq**
  - `fn eq(self: &Self, other: &f32) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
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
  - `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_unit_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_tuple_struct<V>(self: Self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_struct<V>(self: Self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &usize) -> bool`
- **IntoDeserializer**
  - `fn into_deserializer(self: Self) -> <Self as >::Deserializer`
- **PartialEq**
  - `fn eq(self: &Self, other: &u64) -> bool`
- **From**
  - `fn from(f: String) -> Self` - Convert `String` to `Value::String`.
- **From**
  - `fn from(f: Map<String, Value>) -> Self` - Convert map (with string keys) to `Value::Object`.
- **From**
  - `fn from(f: f64) -> Self` - Convert 64-bit floating point number to `Value::Number`, or
- **PartialEq**
  - `fn eq(self: &Self, other: &u32) -> bool`
- **From**
  - `fn from((): ()) -> Self` - Convert `()` to `Value::Null`.
- **From**
  - `fn from(n: usize) -> Self`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self` - Create a `Value::Array` by collecting an iterator of array elements.
- **From**
  - `fn from(n: u32) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &u16) -> bool`
- **From**
  - `fn from(array: [T; N]) -> Self`
- **From**
  - `fn from(n: u8) -> Self`
- **From**
  - `fn from(f: Number) -> Self` - Convert `Number` to `Value::Number`.
- **From**
  - `fn from(n: i64) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &&str) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &u8) -> bool`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>`
- **From**
  - `fn from(n: i16) -> Self`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: I) -> & mut Value` - Write into a `serde_json::Value` using the syntax `value[0] = ...` or
- **Default**
  - `fn default() -> Value`



## serde_json::value::from_value

*Function*

Interpret a `serde_json::Value` as an instance of type `T`.

# Example

```
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `serde_json::Value`
    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let u: User = serde_json::from_value(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the Value does not match the
structure expected by `T`, for example if `T` is a struct type but the Value
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_value<T>(value: Value) -> Result<T, crate::error::Error>
```



## serde_json::value::to_value

*Function*

Convert a `T` into `serde_json::Value` which is an enum that can represent
any valid JSON data.

# Example

```
use serde::Serialize;
use serde_json::json;
use std::error::Error;

#[derive(Serialize)]
struct User {
    fingerprint: String,
    location: String,
}

fn compare_json_values() -> Result<(), Box<dyn Error>> {
    let u = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };

    // The type of `expected` is `serde_json::Value`
    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    });

    let v = serde_json::to_value(u).unwrap();
    assert_eq!(v, expected);

    Ok(())
}
#
# compare_json_values().unwrap();
```

# Errors

This conversion can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```
use std::collections::BTreeMap;

fn main() {
    // The keys in this map are vectors, not strings.
    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");

    println!("{}", serde_json::to_value(map).unwrap_err());
}
```

```rust
fn to_value<T>(value: T) -> Result<Value, crate::error::Error>
```



