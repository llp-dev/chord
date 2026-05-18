**serde_core > ser**

# Module: ser

## Contents

**Traits**

- [`Error`](#error) - Trait used by `Serialize` implementations to generically construct
- [`Serialize`](#serialize) - A **data structure** that can be serialized into any data format supported
- [`SerializeMap`](#serializemap) - Returned from `Serializer::serialize_map`.
- [`SerializeSeq`](#serializeseq) - Returned from `Serializer::serialize_seq`.
- [`SerializeStruct`](#serializestruct) - Returned from `Serializer::serialize_struct`.
- [`SerializeStructVariant`](#serializestructvariant) - Returned from `Serializer::serialize_struct_variant`.
- [`SerializeTuple`](#serializetuple) - Returned from `Serializer::serialize_tuple`.
- [`SerializeTupleStruct`](#serializetuplestruct) - Returned from `Serializer::serialize_tuple_struct`.
- [`SerializeTupleVariant`](#serializetuplevariant) - Returned from `Serializer::serialize_tuple_variant`.
- [`Serializer`](#serializer) - A **data format** that can serialize any data structure supported by Serde.

---

## serde_core::ser::Error

*Trait*

Trait used by `Serialize` implementations to generically construct
errors belonging to the `Serializer` against which they are
currently running.

# Example implementation

The [example data format] presented on the website shows an error
type appropriate for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `custom`: Used when a [`Serialize`] implementation encounters any error



## serde_core::ser::Serialize

*Trait*

A **data structure** that can be serialized into any data format supported
by Serde.

Serde provides `Serialize` implementations for many Rust primitive and
standard library types. The complete list is [here][crate::ser]. All of
these can be serialized using Serde out of the box.

Additionally, Serde provides a procedural macro called [`serde_derive`] to
automatically generate `Serialize` implementations for structs and enums in
your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement `Serialize` manually for some
type in your program. See the [Implementing `Serialize`] section of the
manual for more about this.

Third-party crates may provide `Serialize` implementations for types that
they expose. For example the [`linked-hash-map`] crate provides a
[`LinkedHashMap<K, V>`] type that is serializable by Serde because the crate
provides an implementation of `Serialize` for it.

[Implementing `Serialize`]: https://serde.rs/impl-serialize.html
[`LinkedHashMap<K, V>`]: https://docs.rs/linked-hash-map/*/linked_hash_map/struct.LinkedHashMap.html
[`linked-hash-map`]: https://crates.io/crates/linked-hash-map
[`serde_derive`]: https://crates.io/crates/serde_derive
[derive section of the manual]: https://serde.rs/derive.html

**Methods:**

- `serialize`: Serialize this value into the given Serde serializer.



## serde_core::ser::SerializeMap

*Trait*

Returned from `Serializer::serialize_map`.

# Example use

```edition2021
# use std::marker::PhantomData;
#
# struct HashMap<K, V>(PhantomData<K>, PhantomData<V>);
#
# impl<K, V> HashMap<K, V> {
#     fn len(&self) -> usize {
#         unimplemented!()
#     }
# }
#
# impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
#     type Item = (&'a K, &'a V);
#     type IntoIter = Box<dyn Iterator<Item = (&'a K, &'a V)>>;
#
#     fn into_iter(self) -> Self::IntoIter {
#         unimplemented!()
#     }
# }
#
use serde::ser::{Serialize, SerializeMap, Serializer};

impl<K, V> Serialize for HashMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeMap` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: Must match the `Ok` type of our `Serializer`.
- `Error`: Must match the `Error` type of our `Serializer`.
- `serialize_key`: Serialize a map key.
- `serialize_value`: Serialize a map value.
- `serialize_entry`: Serialize a map entry consisting of a key and a value.
- `end`: Finish serializing a map.



## serde_core::ser::SerializeSeq

*Trait*

Returned from `Serializer::serialize_seq`.

# Example use

```edition2021
# use std::marker::PhantomData;
#
# struct Vec<T>(PhantomData<T>);
#
# impl<T> Vec<T> {
#     fn len(&self) -> usize {
#         unimplemented!()
#     }
# }
#
# impl<'a, T> IntoIterator for &'a Vec<T> {
#     type Item = &'a T;
#     type IntoIter = Box<dyn Iterator<Item = &'a T>>;
#     fn into_iter(self) -> Self::IntoIter {
#         unimplemented!()
#     }
# }
#
use serde::ser::{Serialize, SerializeSeq, Serializer};

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeSeq` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: Must match the `Ok` type of our `Serializer`.
- `Error`: Must match the `Error` type of our `Serializer`.
- `serialize_element`: Serialize a sequence element.
- `end`: Finish serializing a sequence.



## serde_core::ser::SerializeStruct

*Trait*

Returned from `Serializer::serialize_struct`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeStruct, Serializer};

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("Rgb", 3)?;
        rgb.serialize_field("r", &self.r)?;
        rgb.serialize_field("g", &self.g)?;
        rgb.serialize_field("b", &self.b)?;
        rgb.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeStruct` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: Must match the `Ok` type of our `Serializer`.
- `Error`: Must match the `Error` type of our `Serializer`.
- `serialize_field`: Serialize a struct field.
- `skip_field`: Indicate that a struct field has been skipped.
- `end`: Finish serializing a struct.



## serde_core::ser::SerializeStructVariant

*Trait*

Returned from `Serializer::serialize_struct_variant`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeStructVariant, Serializer};

enum E {
    S { r: u8, g: u8, b: u8 },
}

impl Serialize for E {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            E::S {
                ref r,
                ref g,
                ref b,
            } => {
                let mut sv = serializer.serialize_struct_variant("E", 0, "S", 3)?;
                sv.serialize_field("r", r)?;
                sv.serialize_field("g", g)?;
                sv.serialize_field("b", b)?;
                sv.end()
            }
        }
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeStructVariant` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: Must match the `Ok` type of our `Serializer`.
- `Error`: Must match the `Error` type of our `Serializer`.
- `serialize_field`: Serialize a struct variant field.
- `skip_field`: Indicate that a struct variant field has been skipped.
- `end`: Finish serializing a struct variant.



## serde_core::ser::SerializeTuple

*Trait*

Returned from `Serializer::serialize_tuple`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeTuple, Serializer};

# mod fool {
#     trait Serialize {}
impl<A, B, C> Serialize for (A, B, C)
#     {}
# }
#
# struct Tuple3<A, B, C>(A, B, C);
#
# impl<A, B, C> Serialize for Tuple3<A, B, C>
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(3)?;
        tup.serialize_element(&self.0)?;
        tup.serialize_element(&self.1)?;
        tup.serialize_element(&self.2)?;
        tup.end()
    }
}
```

```edition2021
# use std::marker::PhantomData;
#
# struct Array<T>(PhantomData<T>);
#
# impl<T> Array<T> {
#     fn len(&self) -> usize {
#         unimplemented!()
#     }
# }
#
# impl<'a, T> IntoIterator for &'a Array<T> {
#     type Item = &'a T;
#     type IntoIter = Box<dyn Iterator<Item = &'a T>>;
#     fn into_iter(self) -> Self::IntoIter {
#         unimplemented!()
#     }
# }
#
use serde::ser::{Serialize, SerializeTuple, Serializer};

# mod fool {
#     trait Serialize {}
impl<T> Serialize for [T; 16]
#     {}
# }
#
# impl<T> Serialize for Array<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(16)?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeTuple` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: Must match the `Ok` type of our `Serializer`.
- `Error`: Must match the `Error` type of our `Serializer`.
- `serialize_element`: Serialize a tuple element.
- `end`: Finish serializing a tuple.



## serde_core::ser::SerializeTupleStruct

*Trait*

Returned from `Serializer::serialize_tuple_struct`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeTupleStruct, Serializer};

struct Rgb(u8, u8, u8);

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ts = serializer.serialize_tuple_struct("Rgb", 3)?;
        ts.serialize_field(&self.0)?;
        ts.serialize_field(&self.1)?;
        ts.serialize_field(&self.2)?;
        ts.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeTupleStruct` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: Must match the `Ok` type of our `Serializer`.
- `Error`: Must match the `Error` type of our `Serializer`.
- `serialize_field`: Serialize a tuple struct field.
- `end`: Finish serializing a tuple struct.



## serde_core::ser::SerializeTupleVariant

*Trait*

Returned from `Serializer::serialize_tuple_variant`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeTupleVariant, Serializer};

enum E {
    T(u8, u8),
    U(String, u32, u32),
}

impl Serialize for E {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            E::T(ref a, ref b) => {
                let mut tv = serializer.serialize_tuple_variant("E", 0, "T", 2)?;
                tv.serialize_field(a)?;
                tv.serialize_field(b)?;
                tv.end()
            }
            E::U(ref a, ref b, ref c) => {
                let mut tv = serializer.serialize_tuple_variant("E", 1, "U", 3)?;
                tv.serialize_field(a)?;
                tv.serialize_field(b)?;
                tv.serialize_field(c)?;
                tv.end()
            }
        }
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeTupleVariant` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: Must match the `Ok` type of our `Serializer`.
- `Error`: Must match the `Error` type of our `Serializer`.
- `serialize_field`: Serialize a tuple variant field.
- `end`: Finish serializing a tuple variant.



## serde_core::ser::Serializer

*Trait*

A **data format** that can serialize any data structure supported by Serde.

The role of this trait is to define the serialization half of the [Serde
data model], which is a way to categorize every Rust data structure into one
of 29 possible types. Each method of the `Serializer` trait corresponds to
one of the types of the data model.

Implementations of `Serialize` map themselves into this data model by
invoking exactly one of the `Serializer` methods.

The types that make up the Serde data model are:

 - **14 primitive types**
   - bool
   - i8, i16, i32, i64, i128
   - u8, u16, u32, u64, u128
   - f32, f64
   - char
 - **string**
   - UTF-8 bytes with a length and no null terminator.
   - When serializing, all strings are handled equally. When deserializing,
     there are three flavors of strings: transient, owned, and borrowed.
 - **byte array** - \[u8\]
   - Similar to strings, during deserialization byte arrays can be
     transient, owned, or borrowed.
 - **option**
   - Either none or some value.
 - **unit**
   - The type of `()` in Rust. It represents an anonymous value containing
     no data.
 - **unit_struct**
   - For example `struct Unit` or `PhantomData<T>`. It represents a named
     value containing no data.
 - **unit_variant**
   - For example the `E::A` and `E::B` in `enum E { A, B }`.
 - **newtype_struct**
   - For example `struct Millimeters(u8)`.
 - **newtype_variant**
   - For example the `E::N` in `enum E { N(u8) }`.
 - **seq**
   - A variably sized heterogeneous sequence of values, for example
     `Vec<T>` or `HashSet<T>`. When serializing, the length may or may not
     be known before iterating through all the data. When deserializing,
     the length is determined by looking at the serialized data.
 - **tuple**
   - A statically sized heterogeneous sequence of values for which the
     length will be known at deserialization time without looking at the
     serialized data, for example `(u8,)` or `(String, u64, Vec<T>)` or
     `[u64; 10]`.
 - **tuple_struct**
   - A named tuple, for example `struct Rgb(u8, u8, u8)`.
 - **tuple_variant**
   - For example the `E::T` in `enum E { T(u8, u8) }`.
 - **map**
   - A heterogeneous key-value pairing, for example `BTreeMap<K, V>`.
 - **struct**
   - A heterogeneous key-value pairing in which the keys are strings and
     will be known at deserialization time without looking at the
     serialized data, for example `struct S { r: u8, g: u8, b: u8 }`.
 - **struct_variant**
   - For example the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`.

Many Serde serializers produce text or binary data as output, for example
JSON or Postcard. This is not a requirement of the `Serializer` trait, and
there are serializers that do not produce text or binary output. One example
is the `serde_json::value::Serializer` (distinct from the main `serde_json`
serializer) that produces a `serde_json::Value` data structure in memory as
output.

[Serde data model]: https://serde.rs/data-model.html

# Example implementation

The [example data format] presented on the website contains example code for
a basic JSON `Serializer`.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Ok`: The output type produced by this `Serializer` during successful
- `Error`: The error type when some error occurs during serialization.
- `SerializeSeq`: Type returned from [`serialize_seq`] for serializing the content of the
- `SerializeTuple`: Type returned from [`serialize_tuple`] for serializing the content of
- `SerializeTupleStruct`: Type returned from [`serialize_tuple_struct`] for serializing the
- `SerializeTupleVariant`: Type returned from [`serialize_tuple_variant`] for serializing the
- `SerializeMap`: Type returned from [`serialize_map`] for serializing the content of the
- `SerializeStruct`: Type returned from [`serialize_struct`] for serializing the content of
- `SerializeStructVariant`: Type returned from [`serialize_struct_variant`] for serializing the
- `serialize_bool`: Serialize a `bool` value.
- `serialize_i8`: Serialize an `i8` value.
- `serialize_i16`: Serialize an `i16` value.
- `serialize_i32`: Serialize an `i32` value.
- `serialize_i64`: Serialize an `i64` value.
- `serialize_i128`: Serialize an `i128` value.
- `serialize_u8`: Serialize a `u8` value.
- `serialize_u16`: Serialize a `u16` value.
- `serialize_u32`: Serialize a `u32` value.
- `serialize_u64`: Serialize a `u64` value.
- `serialize_u128`: Serialize a `u128` value.
- `serialize_f32`: Serialize an `f32` value.
- `serialize_f64`: Serialize an `f64` value.
- `serialize_char`: Serialize a character.
- `serialize_str`: Serialize a `&str`.
- `serialize_bytes`: Serialize a chunk of raw byte data.
- `serialize_none`: Serialize a [`None`] value.
- `serialize_some`: Serialize a [`Some(T)`] value.
- `serialize_unit`: Serialize a `()` value.
- `serialize_unit_struct`: Serialize a unit struct like `struct Unit` or `PhantomData<T>`.
- `serialize_unit_variant`: Serialize a unit variant like `E::A` in `enum E { A, B }`.
- `serialize_newtype_struct`: Serialize a newtype struct like `struct Millimeters(u8)`.
- `serialize_newtype_variant`: Serialize a newtype variant like `E::N` in `enum E { N(u8) }`.
- `serialize_seq`: Begin to serialize a variably sized sequence. This call must be
- `serialize_tuple`: Begin to serialize a statically sized sequence whose length will be
- `serialize_tuple_struct`: Begin to serialize a tuple struct like `struct Rgb(u8, u8, u8)`. This
- `serialize_tuple_variant`: Begin to serialize a tuple variant like `E::T` in `enum E { T(u8, u8)
- `serialize_map`: Begin to serialize a map. This call must be followed by zero or more
- `serialize_struct`: Begin to serialize a struct like `struct Rgb { r: u8, g: u8, b: u8 }`.
- `serialize_struct_variant`: Begin to serialize a struct variant like `E::S` in `enum E { S { r: u8,
- `collect_seq`: Collect an iterator as a sequence.
- `collect_map`: Collect an iterator as a map.
- `collect_str`: Serialize a string produced by an implementation of `Display`.
- `is_human_readable`: Determine whether `Serialize` implementations should serialize in



