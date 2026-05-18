**serde_core > de**

# Module: de

## Contents

**Modules**

- [`value`](#value) - Building blocks for deserializing basic values using the `IntoDeserializer`

**Enums**

- [`Unexpected`](#unexpected) - `Unexpected` represents an unexpected invocation of any one of the `Visitor`

**Traits**

- [`Deserialize`](#deserialize) - A **data structure** that can be deserialized from any data format supported
- [`DeserializeOwned`](#deserializeowned) - A data structure that can be deserialized without borrowing any data from
- [`DeserializeSeed`](#deserializeseed) - `DeserializeSeed` is the stateful form of the `Deserialize` trait. If you
- [`Deserializer`](#deserializer) - A **data format** that can deserialize any data structure supported by
- [`EnumAccess`](#enumaccess) - Provides a `Visitor` access to the data of an enum in the input.
- [`Error`](#error) - The `Error` trait allows `Deserialize` implementations to create descriptive
- [`Expected`](#expected) - `Expected` represents an explanation of what data a `Visitor` was expecting
- [`IntoDeserializer`](#intodeserializer) - Converts an existing value into a `Deserializer` from which other values can
- [`MapAccess`](#mapaccess) - Provides a `Visitor` access to each entry of a map in the input.
- [`SeqAccess`](#seqaccess) - Provides a `Visitor` access to each element of a sequence in the input.
- [`VariantAccess`](#variantaccess) - `VariantAccess` is a visitor that is created by the `Deserializer` and
- [`Visitor`](#visitor) - This trait represents a visitor that walks through a deserializer.

---

## serde_core::de::Deserialize

*Trait*

A **data structure** that can be deserialized from any data format supported
by Serde.

Serde provides `Deserialize` implementations for many Rust primitive and
standard library types. The complete list is [here][crate::de]. All of these
can be deserialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Deserialize` implementations for structs and enums
in your program. See the [derive section of the manual][derive] for how to
use this.

In rare cases it may be necessary to implement `Deserialize` manually for
some type in your program. See the [Implementing
`Deserialize`][impl-deserialize] section of the manual for more about this.

Third-party crates may provide `Deserialize` implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is deserializable by Serde because the crate
provides an implementation of `Deserialize` for it.

[derive]: https://serde.rs/derive.html
[impl-deserialize]: https://serde.rs/impl-deserialize.html

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by `Self` when deserialized. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

**Methods:**

- `deserialize`: Deserialize this value from the given Serde deserializer.



## serde_core::de::DeserializeOwned

*Trait*

A data structure that can be deserialized without borrowing any data from
the deserializer.

This is primarily useful for trait bounds on functions. For example a
`from_str` function may be able to deserialize a data structure that borrows
from the input string, but a `from_reader` function may only deserialize
owned data.

```edition2021
# use serde::de::{Deserialize, DeserializeOwned};
# use std::io::{Read, Result};
#
# trait Ignore {
fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>;

fn from_reader<R, T>(rdr: R) -> Result<T>
where
    R: Read,
    T: DeserializeOwned;
# }
```

# Lifetime

The relationship between `Deserialize` and `DeserializeOwned` in trait
bounds is explained in more detail on the page [Understanding deserializer
lifetimes].

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html



## serde_core::de::DeserializeSeed

*Trait*

`DeserializeSeed` is the stateful form of the `Deserialize` trait. If you
ever find yourself looking for a way to pass data into a `Deserialize` impl,
this trait is the way to do it.

As one example of stateful deserialization consider deserializing a JSON
array into an existing buffer. Using the `Deserialize` trait we could
deserialize a JSON array into a `Vec<T>` but it would be a freshly allocated
`Vec<T>`; there is no way for `Deserialize` to reuse a previously allocated
buffer. Using `DeserializeSeed` instead makes this possible as in the
example code below.

The canonical API for stateless deserialization looks like this:

```edition2021
# use serde::Deserialize;
#
# enum Error {}
#
fn func<'de, T: Deserialize<'de>>() -> Result<T, Error>
# {
#     unimplemented!()
# }
```

Adjusting an API like this to support stateful deserialization is a matter
of accepting a seed as input:

```edition2021
# use serde::de::DeserializeSeed;
#
# enum Error {}
#
fn func_seed<'de, T: DeserializeSeed<'de>>(seed: T) -> Result<T::Value, Error>
# {
#     let _ = seed;
#     unimplemented!()
# }
```

In practice the majority of deserialization is stateless. An API expecting a
seed can be appeased by passing `std::marker::PhantomData` as a seed in the
case of stateless deserialization.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by `Self::Value` when deserialized. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example

Suppose we have JSON that looks like `[[1, 2], [3, 4, 5], [6]]` and we need
to deserialize it into a flat representation like `vec![1, 2, 3, 4, 5, 6]`.
Allocating a brand new `Vec<T>` for each subarray would be slow. Instead we
would like to allocate a single `Vec<T>` and then deserialize each subarray
into it. This requires stateful deserialization using the `DeserializeSeed`
trait.

```edition2021
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

// A DeserializeSeed implementation that uses stateful deserialization to
// append array elements onto the end of an existing vector. The preexisting
// state ("seed") in this case is the Vec<T>. The `deserialize` method of
// `ExtendVec` will be traversing the inner arrays of the JSON input and
// appending each integer into the existing Vec.
struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);

impl<'de, 'a, T> DeserializeSeed<'de> for ExtendVec<'a, T>
where
    T: Deserialize<'de>,
{
    // The return type of the `deserialize` method. This implementation
    // appends onto an existing vector but does not create any new data
    // structure, so the return type is ().
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Visitor implementation that will walk an inner array of the JSON
        // input.
        struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);

        impl<'de, 'a, T> Visitor<'de> for ExtendVecVisitor<'a, T>
        where
            T: Deserialize<'de>,
        {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of integers")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
            where
                A: SeqAccess<'de>,
            {
                // Decrease the number of reallocations if there are many elements
                if let Some(size_hint) = seq.size_hint() {
                    self.0.reserve(size_hint);
                }

                // Visit each element in the inner array and push it onto
                // the existing vector.
                while let Some(elem) = seq.next_element()? {
                    self.0.push(elem);
                }
                Ok(())
            }
        }

        deserializer.deserialize_seq(ExtendVecVisitor(self.0))
    }
}

// Visitor implementation that will walk the outer array of the JSON input.
struct FlattenedVecVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for FlattenedVecVisitor<T>
where
    T: Deserialize<'de>,
{
    // This Visitor constructs a single Vec<T> to hold the flattened
    // contents of the inner arrays.
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an array of arrays")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Vec<T>, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // Create a single Vec to hold the flattened contents.
        let mut vec = Vec::new();

        // Each iteration through this loop is one inner array.
        while let Some(()) = seq.next_element_seed(ExtendVec(&mut vec))? {
            // Nothing to do; inner array has been appended into `vec`.
        }

        // Return the finished vec.
        Ok(vec)
    }
}

# fn example<'de, D>(deserializer: D) -> Result<(), D::Error>
# where
#     D: Deserializer<'de>,
# {
let visitor = FlattenedVecVisitor(PhantomData);
let flattened: Vec<u64> = deserializer.deserialize_seq(visitor)?;
#     Ok(())
# }
```

**Methods:**

- `Value`: The type produced by using this seed.
- `deserialize`: Equivalent to the more common `Deserialize::deserialize` method, except



## serde_core::de::Deserializer

*Trait*

A **data format** that can deserialize any data structure supported by
Serde.

The role of this trait is to define the deserialization half of the [Serde
data model], which is a way to categorize every Rust data type into one of
29 possible types. Each method of the `Deserializer` trait corresponds to one
of the types of the data model.

Implementations of `Deserialize` map themselves into this data model by
passing to the `Deserializer` a `Visitor` implementation that can receive
these various types.

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
   - A variably sized heterogeneous sequence of values, for example `Vec<T>`
     or `HashSet<T>`. When serializing, the length may or may not be known
     before iterating through all the data. When deserializing, the length
     is determined by looking at the serialized data.
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
     will be known at deserialization time without looking at the serialized
     data, for example `struct S { r: u8, g: u8, b: u8 }`.
 - **struct_variant**
   - For example the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`.

The `Deserializer` trait supports two entry point styles which enables
different kinds of deserialization.

1. The `deserialize_any` method. Self-describing data formats like JSON are
   able to look at the serialized data and tell what it represents. For
   example the JSON deserializer may see an opening curly brace (`{`) and
   know that it is seeing a map. If the data format supports
   `Deserializer::deserialize_any`, it will drive the Visitor using whatever
   type it sees in the input. JSON uses this approach when deserializing
   `serde_json::Value` which is an enum that can represent any JSON
   document. Without knowing what is in a JSON document, we can deserialize
   it to `serde_json::Value` by going through
   `Deserializer::deserialize_any`.

2. The various `deserialize_*` methods. Non-self-describing formats like
   Postcard need to be told what is in the input in order to deserialize it.
   The `deserialize_*` methods are hints to the deserializer for how to
   interpret the next piece of input. Non-self-describing formats are not
   able to deserialize something like `serde_json::Value` which relies on
   `Deserializer::deserialize_any`.

When implementing `Deserialize`, you should avoid relying on
`Deserializer::deserialize_any` unless you need to be told by the
Deserializer what type is in the input. Know that relying on
`Deserializer::deserialize_any` means your data type will be able to
deserialize from self-describing formats only, ruling out Postcard and many
others.

[Serde data model]: https://serde.rs/data-model.html

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed from the input when deserializing. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website contains example code for
a basic JSON `Deserializer`.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Error`: The error type that can be returned if some error occurs during
- `deserialize_any`: Require the `Deserializer` to figure out how to drive the visitor based
- `deserialize_bool`: Hint that the `Deserialize` type is expecting a `bool` value.
- `deserialize_i8`: Hint that the `Deserialize` type is expecting an `i8` value.
- `deserialize_i16`: Hint that the `Deserialize` type is expecting an `i16` value.
- `deserialize_i32`: Hint that the `Deserialize` type is expecting an `i32` value.
- `deserialize_i64`: Hint that the `Deserialize` type is expecting an `i64` value.
- `deserialize_i128`: Hint that the `Deserialize` type is expecting an `i128` value.
- `deserialize_u8`: Hint that the `Deserialize` type is expecting a `u8` value.
- `deserialize_u16`: Hint that the `Deserialize` type is expecting a `u16` value.
- `deserialize_u32`: Hint that the `Deserialize` type is expecting a `u32` value.
- `deserialize_u64`: Hint that the `Deserialize` type is expecting a `u64` value.
- `deserialize_u128`: Hint that the `Deserialize` type is expecting an `u128` value.
- `deserialize_f32`: Hint that the `Deserialize` type is expecting a `f32` value.
- `deserialize_f64`: Hint that the `Deserialize` type is expecting a `f64` value.
- `deserialize_char`: Hint that the `Deserialize` type is expecting a `char` value.
- `deserialize_str`: Hint that the `Deserialize` type is expecting a string value and does
- `deserialize_string`: Hint that the `Deserialize` type is expecting a string value and would
- `deserialize_bytes`: Hint that the `Deserialize` type is expecting a byte array and does not
- `deserialize_byte_buf`: Hint that the `Deserialize` type is expecting a byte array and would
- `deserialize_option`: Hint that the `Deserialize` type is expecting an optional value.
- `deserialize_unit`: Hint that the `Deserialize` type is expecting a unit value.
- `deserialize_unit_struct`: Hint that the `Deserialize` type is expecting a unit struct with a
- `deserialize_newtype_struct`: Hint that the `Deserialize` type is expecting a newtype struct with a
- `deserialize_seq`: Hint that the `Deserialize` type is expecting a sequence of values.
- `deserialize_tuple`: Hint that the `Deserialize` type is expecting a sequence of values and
- `deserialize_tuple_struct`: Hint that the `Deserialize` type is expecting a tuple struct with a
- `deserialize_map`: Hint that the `Deserialize` type is expecting a map of key-value pairs.
- `deserialize_struct`: Hint that the `Deserialize` type is expecting a struct with a particular
- `deserialize_enum`: Hint that the `Deserialize` type is expecting an enum value with a
- `deserialize_identifier`: Hint that the `Deserialize` type is expecting the name of a struct
- `deserialize_ignored_any`: Hint that the `Deserialize` type needs to deserialize a value whose type
- `is_human_readable`: Determine whether `Deserialize` implementations should expect to



## serde_core::de::EnumAccess

*Trait*

Provides a `Visitor` access to the data of an enum in the input.

`EnumAccess` is created by the `Deserializer` and passed to the
`Visitor` in order to identify which variant of an enum to deserialize.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by the deserialized enum variant. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `EnumAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Error`: The error type that can be returned if some error occurs during
- `Variant`: The `Visitor` that will be used to deserialize the content of the enum
- `variant_seed`: `variant` is called to identify which variant to deserialize.
- `variant`: `variant` is called to identify which variant to deserialize.



## serde_core::de::Error

*Trait*

The `Error` trait allows `Deserialize` implementations to create descriptive
error messages belonging to the `Deserializer` against which they are
currently running.

Every `Deserializer` declares an `Error` type that encompasses both
general-purpose deserialization errors as well as errors specific to the
particular deserialization format. For example the `Error` type of
`serde_json` can represent errors like an invalid JSON escape sequence or an
unterminated string literal, in addition to the error cases that are part of
this trait.

Most deserializers should only need to provide the `Error::custom` method
and inherit the default behavior for the other methods.

# Example implementation

The [example data format] presented on the website shows an error
type appropriate for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `custom`: Raised when there is general error when deserializing a type.
- `invalid_type`: Raised when a `Deserialize` receives a type different from what it was
- `invalid_value`: Raised when a `Deserialize` receives a value of the right type but that
- `invalid_length`: Raised when deserializing a sequence or map and the input data contains
- `unknown_variant`: Raised when a `Deserialize` enum type received a variant with an
- `unknown_field`: Raised when a `Deserialize` struct type received a field with an
- `missing_field`: Raised when a `Deserialize` struct type expected to receive a required
- `duplicate_field`: Raised when a `Deserialize` struct type received more than one of the



## serde_core::de::Expected

*Trait*

`Expected` represents an explanation of what data a `Visitor` was expecting
to receive.

This is used as an argument to the `invalid_type`, `invalid_value`, and
`invalid_length` methods of the `Error` trait to build error messages. The
message should be a noun or noun phrase that completes the sentence "This
Visitor expects to receive ...", for example the message could be "an
integer between 0 and 64". The message should not be capitalized and should
not end with a period.

Within the context of a `Visitor` implementation, the `Visitor` itself
(`&self`) is an implementation of this trait.

```edition2021
# use serde::de::{self, Unexpected, Visitor};
# use std::fmt;
#
# struct Example;
#
# impl<'de> Visitor<'de> for Example {
#     type Value = ();
#
#     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
#         write!(formatter, "definitely not a boolean")
#     }
#
fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
where
    E: de::Error,
{
    Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
}
# }
```

Outside of a `Visitor`, `&"..."` can be used.

```edition2021
# use serde::de::{self, Unexpected};
#
# fn example<E>() -> Result<(), E>
# where
#     E: de::Error,
# {
#     let v = true;
return Err(de::Error::invalid_type(
    Unexpected::Bool(v),
    &"a negative integer",
));
# }
```

**Methods:**

- `fmt`: Format an explanation of what data was being expected. Same signature as



## serde_core::de::IntoDeserializer

*Trait*

Converts an existing value into a `Deserializer` from which other values can
be deserialized.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed from the resulting `Deserializer`. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example

```edition2021
use serde::de::{value, Deserialize, IntoDeserializer};
use serde_derive::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
enum Setting {
    On,
    Off,
}

impl FromStr for Setting {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
```

**Methods:**

- `Deserializer`: The type of the deserializer being converted into.
- `into_deserializer`: Convert this value into a deserializer.



## serde_core::de::MapAccess

*Trait*

Provides a `Visitor` access to each entry of a map in the input.

This is a trait that a `Deserializer` passes to a `Visitor` implementation.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by deserialized map entries. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `MapAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Error`: The error type that can be returned if some error occurs during
- `next_key_seed`: This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
- `next_value_seed`: This returns a `Ok(value)` for the next value in the map.
- `next_entry_seed`: This returns `Ok(Some((key, value)))` for the next (key-value) pair in
- `next_key`: This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
- `next_value`: This returns a `Ok(value)` for the next value in the map.
- `next_entry`: This returns `Ok(Some((key, value)))` for the next (key-value) pair in
- `size_hint`: Returns the number of entries remaining in the map, if known.



## serde_core::de::SeqAccess

*Trait*

Provides a `Visitor` access to each element of a sequence in the input.

This is a trait that a `Deserializer` passes to a `Visitor` implementation,
which deserializes each item in a sequence.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by deserialized sequence elements. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SeqAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Error`: The error type that can be returned if some error occurs during
- `next_element_seed`: This returns `Ok(Some(value))` for the next value in the sequence, or
- `next_element`: This returns `Ok(Some(value))` for the next value in the sequence, or
- `size_hint`: Returns the number of elements remaining in the sequence, if known.



## serde_core::de::Unexpected

*Enum*

`Unexpected` represents an unexpected invocation of any one of the `Visitor`
trait methods.

This is used as an argument to the `invalid_type`, `invalid_value`, and
`invalid_length` methods of the `Error` trait to build error messages.

```edition2021
# use std::fmt;
#
# use serde::de::{self, Unexpected, Visitor};
#
# struct Example;
#
# impl<'de> Visitor<'de> for Example {
#     type Value = ();
#
#     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
#         write!(formatter, "definitely not a boolean")
#     }
#
fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
where
    E: de::Error,
{
    Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
}
# }
```

**Generic Parameters:**
- 'a

**Variants:**
- `Bool(bool)` - The input contained a boolean value that was not expected.
- `Unsigned(u64)` - The input contained an unsigned integer `u8`, `u16`, `u32` or `u64` that
- `Signed(i64)` - The input contained a signed integer `i8`, `i16`, `i32` or `i64` that
- `Float(f64)` - The input contained a floating point `f32` or `f64` that was not
- `Char(char)` - The input contained a `char` that was not expected.
- `Str(&'a str)` - The input contained a `&str` or `String` that was not expected.
- `Bytes(&'a [u8])` - The input contained a `&[u8]` or `Vec<u8>` that was not expected.
- `Unit` - The input contained a unit `()` that was not expected.
- `Option` - The input contained an `Option<T>` that was not expected.
- `NewtypeStruct` - The input contained a newtype struct that was not expected.
- `Seq` - The input contained a sequence that was not expected.
- `Map` - The input contained a map that was not expected.
- `Enum` - The input contained an enum that was not expected.
- `UnitVariant` - The input contained a unit variant that was not expected.
- `NewtypeVariant` - The input contained a newtype variant that was not expected.
- `TupleVariant` - The input contained a tuple variant that was not expected.
- `StructVariant` - The input contained a struct variant that was not expected.
- `Other(&'a str)` - A message stating what uncategorized thing the input contained that was

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Unexpected<'a>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Unexpected<'a>) -> bool`
- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`



## serde_core::de::VariantAccess

*Trait*

`VariantAccess` is a visitor that is created by the `Deserializer` and
passed to the `Deserialize` to deserialize the content of a particular enum
variant.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by the deserialized enum variant. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `VariantAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

**Methods:**

- `Error`: The error type that can be returned if some error occurs during
- `unit_variant`: Called when deserializing a variant with no values.
- `newtype_variant_seed`: Called when deserializing a variant with a single value.
- `newtype_variant`: Called when deserializing a variant with a single value.
- `tuple_variant`: Called when deserializing a tuple-like variant.
- `struct_variant`: Called when deserializing a struct-like variant.



## serde_core::de::Visitor

*Trait*

This trait represents a visitor that walks through a deserializer.

# Lifetime

The `'de` lifetime of this trait is the requirement for lifetime of data
that may be borrowed by `Self::Value`. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example

```edition2021
# use serde::de::{self, Unexpected, Visitor};
# use std::fmt;
#
/// A visitor that deserializes a long string - a string containing at least
/// some minimum number of bytes.
struct LongString {
    min: usize,
}

impl<'de> Visitor<'de> for LongString {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing at least {} bytes", self.min)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s.len() >= self.min {
            Ok(s.to_owned())
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}
```

**Methods:**

- `Value`: The value produced by this visitor.
- `expecting`: Format a message stating what data this Visitor expects to receive.
- `visit_bool`: The input contains a boolean.
- `visit_i8`: The input contains an `i8`.
- `visit_i16`: The input contains an `i16`.
- `visit_i32`: The input contains an `i32`.
- `visit_i64`: The input contains an `i64`.
- `visit_i128`: The input contains a `i128`.
- `visit_u8`: The input contains a `u8`.
- `visit_u16`: The input contains a `u16`.
- `visit_u32`: The input contains a `u32`.
- `visit_u64`: The input contains a `u64`.
- `visit_u128`: The input contains a `u128`.
- `visit_f32`: The input contains an `f32`.
- `visit_f64`: The input contains an `f64`.
- `visit_char`: The input contains a `char`.
- `visit_str`: The input contains a string. The lifetime of the string is ephemeral and
- `visit_borrowed_str`: The input contains a string that lives at least as long as the
- `visit_string`: The input contains a string and ownership of the string is being given
- `visit_bytes`: The input contains a byte array. The lifetime of the byte array is
- `visit_borrowed_bytes`: The input contains a byte array that lives at least as long as the
- `visit_byte_buf`: The input contains a byte array and ownership of the byte array is being
- `visit_none`: The input contains an optional that is absent.
- `visit_some`: The input contains an optional that is present.
- `visit_unit`: The input contains a unit `()`.
- `visit_newtype_struct`: The input contains a newtype struct.
- `visit_seq`: The input contains a sequence of elements.
- `visit_map`: The input contains a key-value map.
- `visit_enum`: The input contains an enum.



## Module: value

Building blocks for deserializing basic values using the `IntoDeserializer`
trait.

```edition2021
use serde::de::{value, Deserialize, IntoDeserializer};
use serde_derive::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
enum Setting {
    On,
    Off,
}

impl FromStr for Setting {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
```



