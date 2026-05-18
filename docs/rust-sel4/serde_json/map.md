**serde_json > map**

# Module: map

## Contents

**Structs**

- [`IntoIter`](#intoiter) - An owning iterator over a serde_json::Map's entries.
- [`IntoValues`](#intovalues) - An owning iterator over a serde_json::Map's values.
- [`Iter`](#iter) - An iterator over a serde_json::Map's entries.
- [`IterMut`](#itermut) - A mutable iterator over a serde_json::Map's entries.
- [`Keys`](#keys) - An iterator over a serde_json::Map's keys.
- [`Map`](#map) - Represents a JSON key/value type.
- [`OccupiedEntry`](#occupiedentry) - An occupied Entry. It is part of the [`Entry`] enum.
- [`VacantEntry`](#vacantentry) - A vacant Entry. It is part of the [`Entry`] enum.
- [`Values`](#values) - An iterator over a serde_json::Map's values.
- [`ValuesMut`](#valuesmut) - A mutable iterator over a serde_json::Map's values.

**Enums**

- [`Entry`](#entry) - A view into a single entry in a map, which may either be vacant or occupied.

---

## serde_json::map::Entry

*Enum*

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the [`entry`] method on [`Map`].

[`entry`]: Map::entry

**Generic Parameters:**
- 'a

**Variants:**
- `Vacant(VacantEntry<'a>)` - A vacant Entry.
- `Occupied(OccupiedEntry<'a>)` - An occupied Entry.

**Methods:**

- `fn key(self: &Self) -> &String` - Returns a reference to this entry's key.
- `fn or_insert(self: Self, default: Value) -> &'a  mut Value` - Ensures a value is in the entry by inserting the default if empty, and
- `fn or_insert_with<F>(self: Self, default: F) -> &'a  mut Value` - Ensures a value is in the entry by inserting the result of the default
- `fn and_modify<F>(self: Self, f: F) -> Self` - Provides in-place mutable access to an occupied entry before any



## serde_json::map::IntoIter

*Struct*

An owning iterator over a serde_json::Map's entries.

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## serde_json::map::IntoValues

*Struct*

An owning iterator over a serde_json::Map's values.

**Traits:** FusedIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## serde_json::map::Iter

*Struct*

An iterator over a serde_json::Map's entries.

**Generic Parameters:**
- 'a

**Traits:** FusedIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> Iter<'a>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## serde_json::map::IterMut

*Struct*

A mutable iterator over a serde_json::Map's entries.

**Generic Parameters:**
- 'a

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## serde_json::map::Keys

*Struct*

An iterator over a serde_json::Map's keys.

**Generic Parameters:**
- 'a

**Traits:** FusedIterator

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> Keys<'a>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## serde_json::map::Map

*Struct*

Represents a JSON key/value type.

**Generic Parameters:**
- K
- V

**Methods:**

- `fn new() -> Self` - Makes a new empty Map.
- `fn with_capacity(capacity: usize) -> Self` - Makes a new empty Map with the given initial capacity.
- `fn clear(self: & mut Self)` - Clears the map, removing all values.
- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>` - Returns a reference to the value corresponding to the key.
- `fn contains_key<Q>(self: &Self, key: &Q) -> bool` - Returns true if the map contains a value for the specified key.
- `fn get_mut<Q>(self: & mut Self, key: &Q) -> Option<& mut Value>` - Returns a mutable reference to the value corresponding to the key.
- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>` - Returns the key-value pair matching the given key.
- `fn insert(self: & mut Self, k: String, v: Value) -> Option<Value>` - Inserts a key-value pair into the map.
- `fn remove<Q>(self: & mut Self, key: &Q) -> Option<Value>` - Removes a key from the map, returning the value at the key if the key
- `fn remove_entry<Q>(self: & mut Self, key: &Q) -> Option<(String, Value)>` - Removes a key from the map, returning the stored key and value if the
- `fn append(self: & mut Self, other: & mut Self)` - Moves all elements from other into self, leaving other empty.
- `fn entry<S>(self: & mut Self, key: S) -> Entry` - Gets the given key's corresponding entry in the map for in-place
- `fn len(self: &Self) -> usize` - Returns the number of elements in the map.
- `fn is_empty(self: &Self) -> bool` - Returns true if the map contains no elements.
- `fn iter(self: &Self) -> Iter` - Gets an iterator over the entries of the map.
- `fn iter_mut(self: & mut Self) -> IterMut` - Gets a mutable iterator over the entries of the map.
- `fn keys(self: &Self) -> Keys` - Gets an iterator over the keys of the map.
- `fn values(self: &Self) -> Values` - Gets an iterator over the values of the map.
- `fn values_mut(self: & mut Self) -> ValuesMut` - Gets an iterator over mutable values of the map.
- `fn into_values(self: Self) -> IntoValues` - Gets an iterator over the values of the map.
- `fn retain<F>(self: & mut Self, f: F)` - Retains only the elements specified by the predicate.
- `fn sort_keys(self: & mut Self)` - Sorts this map's entries in-place using `str`'s usual ordering.

**Traits:** Eq

**Trait Implementations:**

- **IndexMut**
  - `fn index_mut(self: & mut Self, index: &Q) -> & mut Value`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
  - `fn clone_from(self: & mut Self, source: &Self)`
- **Default**
  - `fn default() -> Self`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **IntoDeserializer**
  - `fn into_deserializer(self: Self) -> <Self as >::Deserializer`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, Error>`
- **Index**
  - `fn index(self: &Self, index: &Q) -> &Value`
- **Deserializer**
  - `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`
  - `fn deserialize_enum<V>(self: Self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`
  - `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
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
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> Result<(), fmt::Error>`



## serde_json::map::OccupiedEntry

*Struct*

An occupied Entry. It is part of the [`Entry`] enum.

**Generic Parameters:**
- 'a

**Methods:**

- `fn key(self: &Self) -> &String` - Gets a reference to the key in the entry.
- `fn get(self: &Self) -> &Value` - Gets a reference to the value in the entry.
- `fn get_mut(self: & mut Self) -> & mut Value` - Gets a mutable reference to the value in the entry.
- `fn into_mut(self: Self) -> &'a  mut Value` - Converts the entry into a mutable reference to its value.
- `fn insert(self: & mut Self, value: Value) -> Value` - Sets the value of the entry with the `OccupiedEntry`'s key, and returns
- `fn remove(self: Self) -> Value` - Takes the value of the entry out of the map, and returns it.
- `fn remove_entry(self: Self) -> (String, Value)` - Removes the entry from the map, returning the stored key and value.



## serde_json::map::VacantEntry

*Struct*

A vacant Entry. It is part of the [`Entry`] enum.

**Generic Parameters:**
- 'a

**Methods:**

- `fn key(self: &Self) -> &String` - Gets a reference to the key that would be used when inserting a value
- `fn insert(self: Self, value: Value) -> &'a  mut Value` - Sets the value of the entry with the VacantEntry's key, and returns a



## serde_json::map::Values

*Struct*

An iterator over a serde_json::Map's values.

**Generic Parameters:**
- 'a

**Traits:** FusedIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> Values<'a>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## serde_json::map::ValuesMut

*Struct*

A mutable iterator over a serde_json::Map's values.

**Generic Parameters:**
- 'a

**Traits:** FusedIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



