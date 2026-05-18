*[serde_core](../../index.md) / [de](../index.md) / [value](index.md)*

---

# Module `value`

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

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Error`](#error)
  - [`UnitDeserializer`](#unitdeserializer)
  - [`BoolDeserializer`](#booldeserializer)
  - [`I8Deserializer`](#i8deserializer)
  - [`I16Deserializer`](#i16deserializer)
  - [`I32Deserializer`](#i32deserializer)
  - [`I64Deserializer`](#i64deserializer)
  - [`I128Deserializer`](#i128deserializer)
  - [`IsizeDeserializer`](#isizedeserializer)
  - [`U8Deserializer`](#u8deserializer)
  - [`U16Deserializer`](#u16deserializer)
  - [`U64Deserializer`](#u64deserializer)
  - [`U128Deserializer`](#u128deserializer)
  - [`UsizeDeserializer`](#usizedeserializer)
  - [`F32Deserializer`](#f32deserializer)
  - [`F64Deserializer`](#f64deserializer)
  - [`CharDeserializer`](#chardeserializer)
  - [`U32Deserializer`](#u32deserializer)
  - [`StrDeserializer`](#strdeserializer)
  - [`BorrowedStrDeserializer`](#borrowedstrdeserializer)
  - [`StringDeserializer`](#stringdeserializer)
  - [`CowStrDeserializer`](#cowstrdeserializer)
  - [`BytesDeserializer`](#bytesdeserializer)
  - [`BorrowedBytesDeserializer`](#borrowedbytesdeserializer)
  - [`SeqDeserializer`](#seqdeserializer)
  - [`ExpectedInSeq`](#expectedinseq)
  - [`SeqAccessDeserializer`](#seqaccessdeserializer)
  - [`MapDeserializer`](#mapdeserializer)
  - [`PairDeserializer`](#pairdeserializer)
  - [`PairVisitor`](#pairvisitor)
  - [`ExpectedInMap`](#expectedinmap)
  - [`MapAccessDeserializer`](#mapaccessdeserializer)
  - [`EnumAccessDeserializer`](#enumaccessdeserializer)
- [Type Aliases](#type-aliases)
  - [`ErrorImpl`](#errorimpl)
- [Macros](#macros)
  - [`impl_copy_clone!`](#impl-copy-clone)
  - [`primitive_deserializer!`](#primitive-deserializer)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Error`](#error) | struct | A minimal representation of all possible errors that can occur using the `IntoDeserializer` trait. |
| [`UnitDeserializer`](#unitdeserializer) | struct | A deserializer holding a `()`. |
| [`BoolDeserializer`](#booldeserializer) | struct | A deserializer holding a `bool`. |
| [`I8Deserializer`](#i8deserializer) | struct | A deserializer holding an `i8`. |
| [`I16Deserializer`](#i16deserializer) | struct | A deserializer holding an `i16`. |
| [`I32Deserializer`](#i32deserializer) | struct | A deserializer holding an `i32`. |
| [`I64Deserializer`](#i64deserializer) | struct | A deserializer holding an `i64`. |
| [`I128Deserializer`](#i128deserializer) | struct | A deserializer holding an `i128`. |
| [`IsizeDeserializer`](#isizedeserializer) | struct | A deserializer holding an `isize`. |
| [`U8Deserializer`](#u8deserializer) | struct | A deserializer holding a `u8`. |
| [`U16Deserializer`](#u16deserializer) | struct | A deserializer holding a `u16`. |
| [`U64Deserializer`](#u64deserializer) | struct | A deserializer holding a `u64`. |
| [`U128Deserializer`](#u128deserializer) | struct | A deserializer holding a `u128`. |
| [`UsizeDeserializer`](#usizedeserializer) | struct | A deserializer holding a `usize`. |
| [`F32Deserializer`](#f32deserializer) | struct | A deserializer holding an `f32`. |
| [`F64Deserializer`](#f64deserializer) | struct | A deserializer holding an `f64`. |
| [`CharDeserializer`](#chardeserializer) | struct | A deserializer holding a `char`. |
| [`U32Deserializer`](#u32deserializer) | struct | A deserializer holding a `u32`. |
| [`StrDeserializer`](#strdeserializer) | struct | A deserializer holding a `&str`. |
| [`BorrowedStrDeserializer`](#borrowedstrdeserializer) | struct | A deserializer holding a `&str` with a lifetime tied to another deserializer. |
| [`StringDeserializer`](#stringdeserializer) | struct | A deserializer holding a `String`. |
| [`CowStrDeserializer`](#cowstrdeserializer) | struct | A deserializer holding a `Cow<str>`. |
| [`BytesDeserializer`](#bytesdeserializer) | struct | A deserializer holding a `&[u8]`. |
| [`BorrowedBytesDeserializer`](#borrowedbytesdeserializer) | struct | A deserializer holding a `&[u8]` with a lifetime tied to another deserializer. |
| [`SeqDeserializer`](#seqdeserializer) | struct | A deserializer that iterates over a sequence. |
| [`ExpectedInSeq`](#expectedinseq) | struct |  |
| [`SeqAccessDeserializer`](#seqaccessdeserializer) | struct | A deserializer holding a `SeqAccess`. |
| [`MapDeserializer`](#mapdeserializer) | struct | A deserializer that iterates over a map. |
| [`PairDeserializer`](#pairdeserializer) | struct |  |
| [`PairVisitor`](#pairvisitor) | struct |  |
| [`ExpectedInMap`](#expectedinmap) | struct |  |
| [`MapAccessDeserializer`](#mapaccessdeserializer) | struct | A deserializer holding a `MapAccess`. |
| [`EnumAccessDeserializer`](#enumaccessdeserializer) | struct | A deserializer holding an `EnumAccess`. |
| [`ErrorImpl`](#errorimpl) | type |  |
| [`impl_copy_clone!`](#impl-copy-clone) | macro |  |
| [`primitive_deserializer!`](#primitive-deserializer) | macro |  |

## Modules

- [`private`](private/index.md)

## Structs

### `Error`

```rust
struct Error {
    err: Box<str>,
}
```

A minimal representation of all possible errors that can occur using the
`IntoDeserializer` trait.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-error-custom"></span>`fn custom<T>(msg: T) -> Self`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../lib/index.md#string)

### `UnitDeserializer<E>`

```rust
struct UnitDeserializer<E> {
    marker: PhantomData<E>,
}
```

A deserializer holding a `()`.

#### Implementations

- <span id="unitdeserializer-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<E> Clone for UnitDeserializer<E>`

- <span id="unitdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for UnitDeserializer<E>`

##### `impl<E> Debug for UnitDeserializer<E>`

- <span id="unitdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for UnitDeserializer<E>`

- <span id="unitdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="unitdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="unitdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="unitdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for UnitDeserializer<E>`

- <span id="unitdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = UnitDeserializer<E>`

- <span id="unitdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BoolDeserializer<E>`

```rust
struct BoolDeserializer<E> {
    value: bool,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `bool`.

#### Implementations

- <span id="booldeserializer-new"></span>`fn new(value: bool) -> Self`

#### Trait Implementations

##### `impl<E> Clone for BoolDeserializer<E>`

- <span id="booldeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for BoolDeserializer<E>`

##### `impl<E> Debug for BoolDeserializer<E>`

- <span id="booldeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for BoolDeserializer<E>`

- <span id="booldeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="booldeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="booldeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for BoolDeserializer<E>`

- <span id="booldeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = BoolDeserializer<E>`

- <span id="booldeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I8Deserializer<E>`

```rust
struct I8Deserializer<E> {
    value: i8,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i8`.

#### Implementations

- <span id="i8deserializer-new"></span>`fn new(value: i8) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I8Deserializer<E>`

- <span id="i8deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I8Deserializer<E>`

##### `impl<E> Debug for I8Deserializer<E>`

- <span id="i8deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for I8Deserializer<E>`

- <span id="i8deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="i8deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i8deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for I8Deserializer<E>`

- <span id="i8deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = I8Deserializer<E>`

- <span id="i8deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I16Deserializer<E>`

```rust
struct I16Deserializer<E> {
    value: i16,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i16`.

#### Implementations

- <span id="i16deserializer-new"></span>`fn new(value: i16) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I16Deserializer<E>`

- <span id="i16deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I16Deserializer<E>`

##### `impl<E> Debug for I16Deserializer<E>`

- <span id="i16deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for I16Deserializer<E>`

- <span id="i16deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="i16deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i16deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for I16Deserializer<E>`

- <span id="i16deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = I16Deserializer<E>`

- <span id="i16deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I32Deserializer<E>`

```rust
struct I32Deserializer<E> {
    value: i32,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i32`.

#### Implementations

- <span id="i32deserializer-new"></span>`fn new(value: i32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I32Deserializer<E>`

- <span id="i32deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I32Deserializer<E>`

##### `impl<E> Debug for I32Deserializer<E>`

- <span id="i32deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for I32Deserializer<E>`

- <span id="i32deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="i32deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i32deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for I32Deserializer<E>`

- <span id="i32deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = I32Deserializer<E>`

- <span id="i32deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I64Deserializer<E>`

```rust
struct I64Deserializer<E> {
    value: i64,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i64`.

#### Implementations

- <span id="i64deserializer-new"></span>`fn new(value: i64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I64Deserializer<E>`

- <span id="i64deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I64Deserializer<E>`

##### `impl<E> Debug for I64Deserializer<E>`

- <span id="i64deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for I64Deserializer<E>`

- <span id="i64deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="i64deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i64deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for I64Deserializer<E>`

- <span id="i64deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = I64Deserializer<E>`

- <span id="i64deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I128Deserializer<E>`

```rust
struct I128Deserializer<E> {
    value: i128,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i128`.

#### Implementations

- <span id="i128deserializer-new"></span>`fn new(value: i128) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I128Deserializer<E>`

- <span id="i128deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I128Deserializer<E>`

##### `impl<E> Debug for I128Deserializer<E>`

- <span id="i128deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for I128Deserializer<E>`

- <span id="i128deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="i128deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="i128deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for I128Deserializer<E>`

- <span id="i128deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = I128Deserializer<E>`

- <span id="i128deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `IsizeDeserializer<E>`

```rust
struct IsizeDeserializer<E> {
    value: isize,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `isize`.

#### Implementations

- <span id="isizedeserializer-new"></span>`fn new(value: isize) -> Self`

#### Trait Implementations

##### `impl<E> Clone for IsizeDeserializer<E>`

- <span id="isizedeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for IsizeDeserializer<E>`

##### `impl<E> Debug for IsizeDeserializer<E>`

- <span id="isizedeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for IsizeDeserializer<E>`

- <span id="isizedeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="isizedeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="isizedeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for IsizeDeserializer<E>`

- <span id="isizedeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = IsizeDeserializer<E>`

- <span id="isizedeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U8Deserializer<E>`

```rust
struct U8Deserializer<E> {
    value: u8,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u8`.

#### Implementations

- <span id="u8deserializer-new"></span>`fn new(value: u8) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U8Deserializer<E>`

- <span id="u8deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U8Deserializer<E>`

##### `impl<E> Debug for U8Deserializer<E>`

- <span id="u8deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for U8Deserializer<E>`

- <span id="u8deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="u8deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u8deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for U8Deserializer<E>`

- <span id="u8deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = U8Deserializer<E>`

- <span id="u8deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U16Deserializer<E>`

```rust
struct U16Deserializer<E> {
    value: u16,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u16`.

#### Implementations

- <span id="u16deserializer-new"></span>`fn new(value: u16) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U16Deserializer<E>`

- <span id="u16deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U16Deserializer<E>`

##### `impl<E> Debug for U16Deserializer<E>`

- <span id="u16deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for U16Deserializer<E>`

- <span id="u16deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="u16deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u16deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for U16Deserializer<E>`

- <span id="u16deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = U16Deserializer<E>`

- <span id="u16deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U64Deserializer<E>`

```rust
struct U64Deserializer<E> {
    value: u64,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u64`.

#### Implementations

- <span id="u64deserializer-new"></span>`fn new(value: u64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U64Deserializer<E>`

- <span id="u64deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U64Deserializer<E>`

##### `impl<E> Debug for U64Deserializer<E>`

- <span id="u64deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for U64Deserializer<E>`

- <span id="u64deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="u64deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u64deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for U64Deserializer<E>`

- <span id="u64deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = U64Deserializer<E>`

- <span id="u64deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U128Deserializer<E>`

```rust
struct U128Deserializer<E> {
    value: u128,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u128`.

#### Implementations

- <span id="u128deserializer-new"></span>`fn new(value: u128) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U128Deserializer<E>`

- <span id="u128deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U128Deserializer<E>`

##### `impl<E> Debug for U128Deserializer<E>`

- <span id="u128deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for U128Deserializer<E>`

- <span id="u128deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="u128deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u128deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for U128Deserializer<E>`

- <span id="u128deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = U128Deserializer<E>`

- <span id="u128deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `UsizeDeserializer<E>`

```rust
struct UsizeDeserializer<E> {
    value: usize,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `usize`.

#### Implementations

- <span id="usizedeserializer-new"></span>`fn new(value: usize) -> Self`

#### Trait Implementations

##### `impl<E> Clone for UsizeDeserializer<E>`

- <span id="usizedeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for UsizeDeserializer<E>`

##### `impl<E> Debug for UsizeDeserializer<E>`

- <span id="usizedeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for UsizeDeserializer<E>`

- <span id="usizedeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="usizedeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="usizedeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for UsizeDeserializer<E>`

- <span id="usizedeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = UsizeDeserializer<E>`

- <span id="usizedeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `F32Deserializer<E>`

```rust
struct F32Deserializer<E> {
    value: f32,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `f32`.

#### Implementations

- <span id="f32deserializer-new"></span>`fn new(value: f32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for F32Deserializer<E>`

- <span id="f32deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for F32Deserializer<E>`

##### `impl<E> Debug for F32Deserializer<E>`

- <span id="f32deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for F32Deserializer<E>`

- <span id="f32deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="f32deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f32deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for F32Deserializer<E>`

- <span id="f32deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = F32Deserializer<E>`

- <span id="f32deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `F64Deserializer<E>`

```rust
struct F64Deserializer<E> {
    value: f64,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `f64`.

#### Implementations

- <span id="f64deserializer-new"></span>`fn new(value: f64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for F64Deserializer<E>`

- <span id="f64deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for F64Deserializer<E>`

##### `impl<E> Debug for F64Deserializer<E>`

- <span id="f64deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for F64Deserializer<E>`

- <span id="f64deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="f64deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="f64deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for F64Deserializer<E>`

- <span id="f64deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = F64Deserializer<E>`

- <span id="f64deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `CharDeserializer<E>`

```rust
struct CharDeserializer<E> {
    value: char,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `char`.

#### Implementations

- <span id="chardeserializer-new"></span>`fn new(value: char) -> Self`

#### Trait Implementations

##### `impl<E> Clone for CharDeserializer<E>`

- <span id="chardeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for CharDeserializer<E>`

##### `impl<E> Debug for CharDeserializer<E>`

- <span id="chardeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for CharDeserializer<E>`

- <span id="chardeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="chardeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="chardeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> IntoDeserializer for CharDeserializer<E>`

- <span id="chardeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = CharDeserializer<E>`

- <span id="chardeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U32Deserializer<E>`

```rust
struct U32Deserializer<E> {
    value: u32,
    marker: PhantomData<E>,
}
```

A deserializer holding a `u32`.

#### Implementations

- <span id="u32deserializer-new"></span>`fn new(value: u32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U32Deserializer<E>`

- <span id="u32deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U32Deserializer<E>`

##### `impl<E> Debug for U32Deserializer<E>`

- <span id="u32deserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for U32Deserializer<E>`

- <span id="u32deserializer-deserializer-type-error"></span>`type Error = E`

- <span id="u32deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="u32deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="u32deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

##### `impl<E> EnumAccess for U32Deserializer<E>`

- <span id="u32deserializer-enumaccess-type-error"></span>`type Error = E`

- <span id="u32deserializer-enumaccess-type-variant"></span>`type Variant = UnitOnly<E>`

- <span id="u32deserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

##### `impl<E> IntoDeserializer for U32Deserializer<E>`

- <span id="u32deserializer-intodeserializer-type-deserializer"></span>`type Deserializer = U32Deserializer<E>`

- <span id="u32deserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `StrDeserializer<'a, E>`

```rust
struct StrDeserializer<'a, E> {
    value: &'a str,
    marker: PhantomData<E>,
}
```

A deserializer holding a `&str`.

#### Implementations

- <span id="strdeserializer-new"></span>`fn new(value: &'a str) -> Self`

#### Trait Implementations

##### `impl<E> Clone for StrDeserializer<'de, E>`

- <span id="strdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for StrDeserializer<'de, E>`

##### `impl<E> Debug for StrDeserializer<'a, E>`

- <span id="strdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for StrDeserializer<'a, E>`

- <span id="strdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="strdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="strdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="strdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="strdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<E> EnumAccess for StrDeserializer<'a, E>`

- <span id="strdeserializer-enumaccess-type-error"></span>`type Error = E`

- <span id="strdeserializer-enumaccess-type-variant"></span>`type Variant = UnitOnly<E>`

- <span id="strdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

##### `impl<E> IntoDeserializer for StrDeserializer<'a, E>`

- <span id="strdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = StrDeserializer<'a, E>`

- <span id="strdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BorrowedStrDeserializer<'de, E>`

```rust
struct BorrowedStrDeserializer<'de, E> {
    value: &'de str,
    marker: PhantomData<E>,
}
```

A deserializer holding a `&str` with a lifetime tied to another
deserializer.

#### Implementations

- <span id="borrowedstrdeserializer-new"></span>`fn new(value: &'de str) -> BorrowedStrDeserializer<'de, E>` — [`BorrowedStrDeserializer`](#borrowedstrdeserializer)

  Create a new borrowed deserializer from the given string.

#### Trait Implementations

##### `impl<E> Clone for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for BorrowedStrDeserializer<'de, E>`

##### `impl<E> Debug for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="borrowedstrdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="borrowedstrdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="borrowedstrdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedstrdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<E> EnumAccess for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-enumaccess-type-error"></span>`type Error = E`

- <span id="borrowedstrdeserializer-enumaccess-type-variant"></span>`type Variant = UnitOnly<E>`

- <span id="borrowedstrdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

##### `impl<E> IntoDeserializer for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `StringDeserializer<E>`

```rust
struct StringDeserializer<E> {
    value: String,
    marker: PhantomData<E>,
}
```

A deserializer holding a `String`.

#### Implementations

- <span id="stringdeserializer-new"></span>`fn new(value: String) -> Self` — [`String`](../../lib/index.md#string)

#### Trait Implementations

##### `impl<E> Clone for StringDeserializer<E>`

- <span id="stringdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Debug for StringDeserializer<E>`

- <span id="stringdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for StringDeserializer<E>`

- <span id="stringdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="stringdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="stringdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="stringdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="stringdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<E> EnumAccess for StringDeserializer<E>`

- <span id="stringdeserializer-enumaccess-type-error"></span>`type Error = E`

- <span id="stringdeserializer-enumaccess-type-variant"></span>`type Variant = UnitOnly<E>`

- <span id="stringdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

##### `impl<E> IntoDeserializer for StringDeserializer<E>`

- <span id="stringdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = StringDeserializer<E>`

- <span id="stringdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `CowStrDeserializer<'a, E>`

```rust
struct CowStrDeserializer<'a, E> {
    value: Cow<'a, str>,
    marker: PhantomData<E>,
}
```

A deserializer holding a `Cow<str>`.

#### Implementations

- <span id="cowstrdeserializer-new"></span>`fn new(value: Cow<'a, str>) -> Self` — [`Cow`](../../lib/index.md#cow)

#### Trait Implementations

##### `impl<E> Clone for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Debug for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="cowstrdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="cowstrdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="cowstrdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="cowstrdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<E> EnumAccess for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-enumaccess-type-error"></span>`type Error = E`

- <span id="cowstrdeserializer-enumaccess-type-variant"></span>`type Variant = UnitOnly<E>`

- <span id="cowstrdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

##### `impl<E> IntoDeserializer for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BytesDeserializer<'a, E>`

```rust
struct BytesDeserializer<'a, E> {
    value: &'a [u8],
    marker: PhantomData<E>,
}
```

A deserializer holding a `&[u8]`. Always calls `Visitor::visit_bytes`.

#### Implementations

- <span id="bytesdeserializer-new"></span>`fn new(value: &'a [u8]) -> Self`

  Create a new deserializer from the given bytes.

#### Trait Implementations

##### `impl<E> Clone for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for BytesDeserializer<'a, E>`

##### `impl<E> Debug for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="bytesdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="bytesdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="bytesdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<E> IntoDeserializer for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BorrowedBytesDeserializer<'de, E>`

```rust
struct BorrowedBytesDeserializer<'de, E> {
    value: &'de [u8],
    marker: PhantomData<E>,
}
```

A deserializer holding a `&[u8]` with a lifetime tied to another
deserializer. Always calls `Visitor::visit_borrowed_bytes`.

#### Implementations

- <span id="borrowedbytesdeserializer-new"></span>`fn new(value: &'de [u8]) -> Self`

  Create a new borrowed deserializer from the given borrowed bytes.

#### Trait Implementations

##### `impl<E> Clone for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for BorrowedBytesDeserializer<'de, E>`

##### `impl<E> Debug for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Deserializer for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="borrowedbytesdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="borrowedbytesdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<E> IntoDeserializer for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `SeqDeserializer<I, E>`

```rust
struct SeqDeserializer<I, E> {
    iter: iter::Fuse<I>,
    count: usize,
    marker: PhantomData<E>,
}
```

A deserializer that iterates over a sequence.

#### Implementations

- <span id="seqdeserializer-new"></span>`fn new(iter: I) -> Self`

  Construct a new `SeqDeserializer<I, E>`.

#### Trait Implementations

##### `impl<I: clone::Clone, E: clone::Clone> Clone for SeqDeserializer<I, E>`

- <span id="seqdeserializer-clone"></span>`fn clone(&self) -> SeqDeserializer<I, E>` — [`SeqDeserializer`](#seqdeserializer)

##### `impl<I, E> Debug for SeqDeserializer<I, E>`

- <span id="seqdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, E> Deserializer for SeqDeserializer<I, E>`

- <span id="seqdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="seqdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="seqdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<I, E> IntoDeserializer for SeqDeserializer<I, E>`

- <span id="seqdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = SeqDeserializer<I, E>`

- <span id="seqdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

##### `impl<I, E> SeqAccess for SeqDeserializer<I, E>`

- <span id="seqdeserializer-seqaccess-type-error"></span>`type Error = E`

- <span id="seqdeserializer-seqaccess-next-element-seed"></span>`fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<<V as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

- <span id="seqdeserializer-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `ExpectedInSeq`

```rust
struct ExpectedInSeq(usize);
```

#### Trait Implementations

##### `impl Expected for ExpectedInSeq`

- <span id="expectedinseq-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SeqAccessDeserializer<A>`

```rust
struct SeqAccessDeserializer<A> {
    seq: A,
}
```

A deserializer holding a `SeqAccess`.

#### Implementations

- <span id="seqaccessdeserializer-new"></span>`fn new(seq: A) -> Self`

  Construct a new `SeqAccessDeserializer<A>`.

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-clone"></span>`fn clone(&self) -> SeqAccessDeserializer<A>` — [`SeqAccessDeserializer`](#seqaccessdeserializer)

##### `impl<A: fmt::Debug> Debug for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> Deserializer for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-deserializer-type-error"></span>`type Error = <A as SeqAccess>::Error`

- <span id="seqaccessdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="seqaccessdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="seqaccessdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<A> IntoDeserializer for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `MapDeserializer<'de, I, E>`

```rust
struct MapDeserializer<'de, I, E>
where
    I: Iterator,
    <I as >::Item: private::Pair {
    iter: iter::Fuse<I>,
    value: Option<<<I as >::Item as Pair>::Second>,
    count: usize,
    lifetime: PhantomData<&'de ()>,
    error: PhantomData<E>,
}
```

A deserializer that iterates over a map.

#### Implementations

- <span id="mapdeserializer-new"></span>`fn new(iter: I) -> Self`

  Construct a new `MapDeserializer<I, E>`.

#### Trait Implementations

##### `impl<I, E> Clone for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<I, E> Debug for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, E> Deserializer for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="mapdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="mapdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="mapdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="mapdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<I, E> IntoDeserializer for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

##### `impl<I, E> MapAccess for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-mapaccess-type-error"></span>`type Error = E`

- <span id="mapdeserializer-mapaccess-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

- <span id="mapdeserializer-mapaccess-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

- <span id="mapdeserializer-mapaccess-next-entry-seed"></span>`fn next_entry_seed<TK, TV>(&mut self, kseed: TK, vseed: TV) -> Result<Option<(<TK as >::Value, <TV as >::Value)>, <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

- <span id="mapdeserializer-mapaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

##### `impl<I, E> SeqAccess for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-seqaccess-type-error"></span>`type Error = E`

- <span id="mapdeserializer-seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

- <span id="mapdeserializer-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `PairDeserializer<A, B, E>`

```rust
struct PairDeserializer<A, B, E>(A, B, PhantomData<E>);
```

#### Trait Implementations

##### `impl<A, B, E> Deserializer for PairDeserializer<A, B, E>`

- <span id="pairdeserializer-deserializer-type-error"></span>`type Error = E`

- <span id="pairdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="pairdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="pairdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="pairdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

### `PairVisitor<A, B, E>`

```rust
struct PairVisitor<A, B, E>(Option<A>, Option<B>, PhantomData<E>);
```

#### Trait Implementations

##### `impl<A, B, E> SeqAccess for PairVisitor<A, B, E>`

- <span id="pairvisitor-seqaccess-type-error"></span>`type Error = E`

- <span id="pairvisitor-seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

- <span id="pairvisitor-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `ExpectedInMap`

```rust
struct ExpectedInMap(usize);
```

#### Trait Implementations

##### `impl Expected for ExpectedInMap`

- <span id="expectedinmap-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MapAccessDeserializer<A>`

```rust
struct MapAccessDeserializer<A> {
    map: A,
}
```

A deserializer holding a `MapAccess`.

#### Implementations

- <span id="mapaccessdeserializer-new"></span>`fn new(map: A) -> Self`

  Construct a new `MapAccessDeserializer<A>`.

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-clone"></span>`fn clone(&self) -> MapAccessDeserializer<A>` — [`MapAccessDeserializer`](#mapaccessdeserializer)

##### `impl<A: fmt::Debug> Debug for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> Deserializer for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-deserializer-type-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapaccessdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="mapaccessdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="mapaccessdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="mapaccessdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<A> EnumAccess for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-enumaccess-type-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapaccessdeserializer-enumaccess-type-variant"></span>`type Variant = MapAsEnum<A>`

- <span id="mapaccessdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md#deserializeseed)

##### `impl<A> IntoDeserializer for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `EnumAccessDeserializer<A>`

```rust
struct EnumAccessDeserializer<A> {
    access: A,
}
```

A deserializer holding an `EnumAccess`.

#### Implementations

- <span id="enumaccessdeserializer-new"></span>`fn new(access: A) -> Self`

  Construct a new `EnumAccessDeserializer<A>`.

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-clone"></span>`fn clone(&self) -> EnumAccessDeserializer<A>` — [`EnumAccessDeserializer`](#enumaccessdeserializer)

##### `impl<A: fmt::Debug> Debug for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> Deserializer for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-deserializer-type-error"></span>`type Error = <A as EnumAccess>::Error`

- <span id="enumaccessdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="enumaccessdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

- <span id="enumaccessdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md#visitor), [`Deserializer`](../index.md#deserializer)

##### `impl<A> IntoDeserializer for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-intodeserializer-type-deserializer"></span>`type Deserializer = EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

## Type Aliases

### `ErrorImpl`

```rust
type ErrorImpl = Box<str>;
```

## Macros

### `impl_copy_clone!`

### `primitive_deserializer!`

