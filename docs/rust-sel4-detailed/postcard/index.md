# Crate `postcard`

[![Documentation](https://docs.rs/postcard/badge.svg)](https://docs.rs/postcard)

Postcard is a `#![no_std]` focused serializer and deserializer for Serde.

Postcard aims to be convenient for developers in constrained environments, while
allowing for flexibility to customize behavior as needed.

## Design Goals

1. Design primarily for `#![no_std]` usage, in embedded or other constrained contexts
2. Support a maximal set of `serde` features, so `postcard` can be used as a drop in replacement
3. Avoid special differences in code between communication code written for a microcontroller or a desktop/server PC
4. Be resource efficient - memory usage, code size, developer time, and CPU time; in that order
5. Allow library users to customize the serialization and deserialization  behavior to fit their bespoke needs

## Format Stability

As of v1.0.0, `postcard` has a documented and stable wire format. More information about this
wire format can be found in the `spec/` folder of the Postcard repository, or viewed online
at <https://postcard.jamesmunns.com>.

Work towards the Postcard Specification and portions of the Postcard 1.0 Release
were sponsored by Mozilla Corporation.

## Variable Length Data

All signed and unsigned integers larger than eight bits are encoded using a [Varint].
This includes the length of array slices, as well as the discriminant of `enums`.

For more information, see the [Varint] chapter of the wire specification.

## Example - Serialization/Deserialization

Postcard can serialize and deserialize messages similar to other `serde` formats.

Using the default `heapless` feature to serialize to a `heapless::Vec<u8>`:

```rust
use core::ops::Deref;
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_vec};
use heapless::Vec;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}
let message = "hElLo";
let bytes = [0x01, 0x10, 0x02, 0x20];
let output: Vec<u8, 11> = to_vec(&RefStruct {
    bytes: &bytes,
    str_s: message,
}).unwrap();

assert_eq!(
    &[0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',],
    output.deref()
);

let out: RefStruct = from_bytes(output.deref()).unwrap();
assert_eq!(
    out,
    RefStruct {
        bytes: &bytes,
        str_s: message,
    }
);
```

Or the optional `alloc` feature to serialize to an `alloc::vec::Vec<u8>`:

```rust
use core::ops::Deref;
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_allocvec};
extern crate alloc;
use alloc::vec::Vec;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}
let message = "hElLo";
let bytes = [0x01, 0x10, 0x02, 0x20];
let output: Vec<u8> = to_allocvec(&RefStruct {
    bytes: &bytes,
    str_s: message,
}).unwrap();

assert_eq!(
    &[0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',],
    output.deref()
);

let out: RefStruct = from_bytes(output.deref()).unwrap();
assert_eq!(
    out,
    RefStruct {
        bytes: &bytes,
        str_s: message,
    }
);
```

## Flavors

`postcard` supports a system called `Flavors`, which are used to modify the way
postcard serializes or processes serialized data. These flavors act as "plugins" or "middlewares"
during the serialization or deserialization process, and can be combined to obtain complex protocol formats.

See the documentation of the `ser_flavors` or `de_flavors` modules for more information on usage.

## Setup - `Cargo.toml`

Don't forget to add [the `no-std` subset](https://serde.rs/no-std.html) of `serde` along with `postcard` to the `[dependencies]` section of your `Cargo.toml`!

```toml
[dependencies]
postcard = "1.0.0"

By default, `serde` has the `std` feature enabled, which makes it unsuitable for embedded targets
disabling default-features fixes this
serde = { version = "1.0.*", default-features = false }
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Contents

- [Modules](#modules)
  - [`accumulator`](#accumulator)
  - [`de`](#de)
  - [`eio`](#eio)
  - [`error`](#error)
  - [`fixint`](#fixint)
  - [`ser`](#ser)
  - [`varint`](#varint)
  - [`max_size`](#max-size)
  - [`experimental`](#experimental)
  - [`de_flavors`](#de-flavors)
  - [`ser_flavors`](#ser-flavors)
- [Structs](#structs)
  - [`Deserializer`](#deserializer)
  - [`Serializer`](#serializer)
- [Enums](#enums)
  - [`Error`](#error)
- [Functions](#functions)
  - [`from_bytes`](#from-bytes)
  - [`from_bytes_cobs`](#from-bytes-cobs)
  - [`take_from_bytes`](#take-from-bytes)
  - [`take_from_bytes_cobs`](#take-from-bytes-cobs)
  - [`serialize_with_flavor`](#serialize-with-flavor)
  - [`to_extend`](#to-extend)
  - [`to_slice`](#to-slice)
  - [`to_slice_cobs`](#to-slice-cobs)
  - [`to_allocvec`](#to-allocvec)
  - [`to_allocvec_cobs`](#to-allocvec-cobs)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`accumulator`](#accumulator) | mod | An accumulator used to collect chunked COBS data and deserialize it. |
| [`de`](#de) | mod |  |
| [`eio`](#eio) | mod |  |
| [`error`](#error) | mod |  |
| [`fixint`](#fixint) | mod | # Fixed Size Integers |
| [`ser`](#ser) | mod |  |
| [`varint`](#varint) | mod |  |
| [`max_size`](#max-size) | mod |  |
| [`experimental`](#experimental) | mod | # Experimental Postcard Features |
| [`de_flavors`](#de-flavors) | mod |  |
| [`ser_flavors`](#ser-flavors) | mod |  |
| [`Deserializer`](#deserializer) | struct |  |
| [`Serializer`](#serializer) | struct |  |
| [`Error`](#error) | enum |  |
| [`from_bytes`](#from-bytes) | fn |  |
| [`from_bytes_cobs`](#from-bytes-cobs) | fn |  |
| [`take_from_bytes`](#take-from-bytes) | fn |  |
| [`take_from_bytes_cobs`](#take-from-bytes-cobs) | fn |  |
| [`serialize_with_flavor`](#serialize-with-flavor) | fn |  |
| [`to_extend`](#to-extend) | fn |  |
| [`to_slice`](#to-slice) | fn |  |
| [`to_slice_cobs`](#to-slice-cobs) | fn |  |
| [`to_allocvec`](#to-allocvec) | fn |  |
| [`to_allocvec_cobs`](#to-allocvec-cobs) | fn |  |
| [`Result`](#result) | type |  |

## Modules

- [`accumulator`](accumulator/index.md) — An accumulator used to collect chunked COBS data and deserialize it.
- [`de`](de/index.md)
- [`eio`](eio/index.md)
- [`error`](error/index.md)
- [`fixint`](fixint/index.md) — # Fixed Size Integers
- [`ser`](ser/index.md)
- [`varint`](varint/index.md)
- [`max_size`](max_size/index.md)
- [`experimental`](experimental/index.md) — # Experimental Postcard Features
- [`de_flavors`](de_flavors/index.md) — # Deserialization Flavors
- [`ser_flavors`](ser_flavors/index.md) — # Serialization Flavors

## Structs

### `Deserializer<'de, F: Flavor<'de>>`

```rust
struct Deserializer<'de, F: Flavor<'de>> {
    flavor: F,
    _plt: core::marker::PhantomData<&'de ()>,
}
```

A `serde` compatible deserializer, generic over “Flavors” of deserializing plugins.

Please note that postcard messages are not self-describing and therefore incompatible with
[internally tagged enums](https://serde.rs/enum-representations.html#internally-tagged).

#### Implementations

- <span id="deserializer-from-flavor"></span>`fn from_flavor(flavor: F) -> Self`

  Obtain a Deserializer from a slice of bytes

- <span id="deserializer-finalize"></span>`fn finalize(self) -> Result<<F as >::Remainder>` — [`Result`](error/index.md#result), [`Flavor`](de/flavors/index.md#flavor)

  Return the remaining (unused) bytes in the Deserializer along with any

  additional data provided by the [`Flavor`](de/flavors/index.md)

#### Trait Implementations

##### `impl<F: Flavor<'de>> Deserializer for &mut Deserializer<'de, F>`

- <span id="mut-deserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-deserializer-is-human-readable"></span>`fn is_human_readable(&self) -> bool`

- <span id="mut-deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, _visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, _visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

##### `impl<F: Flavor<'de>> EnumAccess for &mut Deserializer<'de, F>`

- <span id="mut-deserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-enumaccess-type-variant"></span>`type Variant = &mut Deserializer<'de, F>`

- <span id="mut-deserializer-enumaccess-variant-seed"></span>`fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](error/index.md#result)

##### `impl<F: Flavor<'de>> VariantAccess for &mut Deserializer<'de, F>`

- <span id="mut-deserializer-variantaccess-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-variantaccess-tuple-variant"></span>`fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-variantaccess-struct-variant"></span>`fn struct_variant<V: Visitor<'de>>(self, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

### `Serializer<F>`

```rust
struct Serializer<F>
where
    F: Flavor {
    pub output: F,
}
```

A `serde` compatible serializer, generic over "Flavors" of serializing plugins.

It should rarely be necessary to directly use this type unless you are implementing your
own `SerFlavor`.

See the docs for `SerFlavor` for more information about "flavors" of serialization


#### Fields

- **`output`**: `F`

  This is the Flavor(s) that will be used to modify or store any bytes generated
  by serialization

#### Implementations

- <span id="serializer-try-push-varint-usize"></span>`fn try_push_varint_usize(&mut self, data: usize) -> Result<()>` — [`Result`](error/index.md#result)

  Attempt to push a variably encoded [usize] into the output data stream

- <span id="serializer-try-push-varint-u128"></span>`fn try_push_varint_u128(&mut self, data: u128) -> Result<()>` — [`Result`](error/index.md#result)

  Attempt to push a variably encoded [`u128`](../ring/aead/gcm/index.md) into the output data stream

- <span id="serializer-try-push-varint-u64"></span>`fn try_push_varint_u64(&mut self, data: u64) -> Result<()>` — [`Result`](error/index.md#result)

  Attempt to push a variably encoded [u64] into the output data stream

- <span id="serializer-try-push-varint-u32"></span>`fn try_push_varint_u32(&mut self, data: u32) -> Result<()>` — [`Result`](error/index.md#result)

  Attempt to push a variably encoded [u32] into the output data stream

- <span id="serializer-try-push-varint-u16"></span>`fn try_push_varint_u16(&mut self, data: u16) -> Result<()>` — [`Result`](error/index.md#result)

  Attempt to push a variably encoded [`u16`](../gimli/leb128/read/index.md) into the output data stream

#### Trait Implementations

##### `impl<F> SerializeMap for &mut Serializer<F>`

- <span id="mut-serializer-serializemap-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializemap-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializemap-serialize-key"></span>`fn serialize_key<T>(&mut self, key: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializemap-serialize-value"></span>`fn serialize_value<T>(&mut self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializemap-end"></span>`fn end(self) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<F> SerializeSeq for &mut Serializer<F>`

- <span id="mut-serializer-serializeseq-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializeseq-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializeseq-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializeseq-end"></span>`fn end(self) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<F> SerializeStruct for &mut Serializer<F>`

- <span id="mut-serializer-serializestruct-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializestruct-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializestruct-serialize-field"></span>`fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializestruct-end"></span>`fn end(self) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<F> SerializeStructVariant for &mut Serializer<F>`

- <span id="mut-serializer-serializestructvariant-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializestructvariant-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializestructvariant-serialize-field"></span>`fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializestructvariant-end"></span>`fn end(self) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<F> SerializeTuple for &mut Serializer<F>`

- <span id="mut-serializer-serializetuple-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializetuple-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializetuple-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializetuple-end"></span>`fn end(self) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<F> SerializeTupleStruct for &mut Serializer<F>`

- <span id="mut-serializer-serializetuplestruct-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializetuplestruct-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializetuplestruct-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializetuplestruct-end"></span>`fn end(self) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<F> SerializeTupleVariant for &mut Serializer<F>`

- <span id="mut-serializer-serializetuplevariant-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializetuplevariant-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializetuplevariant-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializetuplevariant-end"></span>`fn end(self) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<F> Serializer for &mut Serializer<F>`

- <span id="mut-serializer-serializer-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializer-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializer-type-serializeseq"></span>`type SerializeSeq = &mut Serializer<F>`

- <span id="mut-serializer-serializer-type-serializetuple"></span>`type SerializeTuple = &mut Serializer<F>`

- <span id="mut-serializer-serializer-type-serializetuplestruct"></span>`type SerializeTupleStruct = &mut Serializer<F>`

- <span id="mut-serializer-serializer-type-serializetuplevariant"></span>`type SerializeTupleVariant = &mut Serializer<F>`

- <span id="mut-serializer-serializer-type-serializemap"></span>`type SerializeMap = &mut Serializer<F>`

- <span id="mut-serializer-serializer-type-serializestruct"></span>`type SerializeStruct = &mut Serializer<F>`

- <span id="mut-serializer-serializer-type-serializestructvariant"></span>`type SerializeStructVariant = &mut Serializer<F>`

- <span id="mut-serializer-serializer-is-human-readable"></span>`fn is_human_readable(&self) -> bool`

- <span id="mut-serializer-serializer-serialize-bool"></span>`fn serialize_bool(self, v: bool) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i8"></span>`fn serialize_i8(self, v: i8) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i16"></span>`fn serialize_i16(self, v: i16) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i32"></span>`fn serialize_i32(self, v: i32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i64"></span>`fn serialize_i64(self, v: i64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i128"></span>`fn serialize_i128(self, v: i128) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u8"></span>`fn serialize_u8(self, v: u8) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u16"></span>`fn serialize_u16(self, v: u16) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u32"></span>`fn serialize_u32(self, v: u32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u64"></span>`fn serialize_u64(self, v: u64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u128"></span>`fn serialize_u128(self, v: u128) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-f32"></span>`fn serialize_f32(self, v: f32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-f64"></span>`fn serialize_f64(self, v: f64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-char"></span>`fn serialize_char(self, v: char) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-str"></span>`fn serialize_str(self, v: &str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-bytes"></span>`fn serialize_bytes(self, v: &[u8]) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-none"></span>`fn serialize_none(self) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, variant_index: u32, _variant: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-tuple"></span>`fn serialize_tuple(self, _len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-struct"></span>`fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](error/index.md#result)

- <span id="mut-serializer-serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<<Self as >::Ok>` — [`Result`](error/index.md#result)

## Enums

### `Error`

```rust
enum Error {
    WontImplement,
    NotYetImplemented,
    SerializeBufferFull,
    SerializeSeqLengthUnknown,
    DeserializeUnexpectedEnd,
    DeserializeBadVarint,
    DeserializeBadBool,
    DeserializeBadChar,
    DeserializeBadUtf8,
    DeserializeBadOption,
    DeserializeBadEnum,
    DeserializeBadEncoding,
    DeserializeBadCrc,
    SerdeSerCustom,
    SerdeDeCustom,
    CollectStrError,
}
```

This is the error type used by Postcard

#### Variants

- **`WontImplement`**

  This is a feature that postcard will never implement

- **`NotYetImplemented`**

  This is a feature that postcard intends to support, but does not yet

- **`SerializeBufferFull`**

  The serialize buffer is full

- **`SerializeSeqLengthUnknown`**

  The length of a sequence must be known

- **`DeserializeUnexpectedEnd`**

  Hit the end of buffer, expected more data

- **`DeserializeBadVarint`**

  Found a varint that didn't terminate. Is the usize too big for this platform?

- **`DeserializeBadBool`**

  Found a bool that wasn't 0 or 1

- **`DeserializeBadChar`**

  Found an invalid unicode char

- **`DeserializeBadUtf8`**

  Tried to parse invalid utf-8

- **`DeserializeBadOption`**

  Found an Option discriminant that wasn't 0 or 1

- **`DeserializeBadEnum`**

  Found an enum discriminant that was > `u32::MAX`

- **`DeserializeBadEncoding`**

  The original data was not well encoded

- **`DeserializeBadCrc`**

  Bad CRC while deserializing

- **`SerdeSerCustom`**

  Serde Serialization Error

- **`SerdeDeCustom`**

  Serde Deserialization Error

- **`CollectStrError`**

  Error while processing `collect_str` during serialization

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](error/index.md#error)

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Error`

- <span id="error-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Error`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

- <span id="error-error-custom"></span>`fn custom<T>(_msg: T) -> Self`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](error/index.md#error)

##### `impl Serialize for Error`

- <span id="error-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `from_bytes`

```rust
fn from_bytes<'a, T>(s: &'a [u8]) -> crate::error::Result<T>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a byte slice. The unused portion (if any)
of the byte slice is not returned.

### `from_bytes_cobs`

```rust
fn from_bytes_cobs<'a, T>(s: &'a mut [u8]) -> crate::error::Result<T>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a cobs-encoded byte slice.

The unused portion (if any) of the byte slice is not returned.
The used portion of the input slice is modified during deserialization (even if an error is returned).
Therefore, if this is not desired, pass a clone of the original slice.

### `take_from_bytes`

```rust
fn take_from_bytes<'a, T>(s: &'a [u8]) -> crate::error::Result<(T, &'a [u8])>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a byte slice. The unused portion (if any)
of the byte slice is returned for further usage

### `take_from_bytes_cobs`

```rust
fn take_from_bytes_cobs<'a, T>(s: &'a mut [u8]) -> crate::error::Result<(T, &'a mut [u8])>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a cobs-encoded byte slice.

The unused portion (if any) of the byte slice is returned for further usage.
The used portion of the input slice is modified during deserialization (even if an error is returned).
Therefore, if this is not desired, pass a clone of the original slice.

### `serialize_with_flavor`

```rust
fn serialize_with_flavor<T, S, O>(value: &T, storage: S) -> crate::error::Result<O>
where
    T: Serialize + ?Sized,
    S: Flavor<Output = O>
```

`serialize_with_flavor()` has three generic parameters, `T, F, O`.

* `T`: This is the type that is being serialized
* `S`: This is the Storage that is used during serialization
* `O`: This is the resulting storage type that is returned containing the serialized data

For more information about how Flavors work, please see the
[`flavors` module documentation](./flavors/index.html).

```rust
use postcard::{
    serialize_with_flavor,
    ser_flavors::{Cobs, Slice},
};

let mut buf = [0u8; 32];

let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
let buffer = &mut [0u8; 32];
let res = serialize_with_flavor::<[u8], Cobs<Slice>, &mut [u8]>(
    data,
    Cobs::try_new(Slice::new(buffer)).unwrap(),
).unwrap();

assert_eq!(res, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
```

### `to_extend`

```rust
fn to_extend<T, W>(value: &T, writer: W) -> crate::error::Result<W>
where
    T: Serialize + ?Sized,
    W: core::iter::Extend<u8>
```

Serialize a `T` to a `core::iter::Extend`,
## Example

```rust
use postcard::to_extend;
let mut vec = Vec::new();

let ser = to_extend(&true, vec).unwrap();
let vec = to_extend("Hi!", ser).unwrap();
assert_eq!(&vec[0..5], &[0x01, 0x03, b'H', b'i', b'!']);
```

### `to_slice`

```rust
fn to_slice<'a, 'b, T>(value: &'b T, buf: &'a mut [u8]) -> crate::error::Result<&'a mut [u8]>
where
    T: Serialize + ?Sized
```

Serialize a `T` to the given slice, with the resulting slice containing
data in a serialized format.

When successful, this function returns the slice containing the
serialized message

## Example

```rust
use postcard::to_slice;
let mut buf = [0u8; 32];

let used = to_slice(&true, &mut buf).unwrap();
assert_eq!(used, &[0x01]);

let used = to_slice("Hi!", &mut buf).unwrap();
assert_eq!(used, &[0x03, b'H', b'i', b'!']);

// NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
let used = to_slice(data, &mut buf).unwrap();
assert_eq!(used, &[0x04, 0x01, 0x00, 0x20, 0x30]);

let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
let used = to_slice(data, &mut buf).unwrap();
assert_eq!(used, &[0x01, 0x00, 0x20, 0x30]);
```

### `to_slice_cobs`

```rust
fn to_slice_cobs<'a, 'b, T>(value: &'b T, buf: &'a mut [u8]) -> crate::error::Result<&'a mut [u8]>
where
    T: Serialize + ?Sized
```

Serialize a `T` to the given slice, with the resulting slice containing
data in a serialized then COBS encoded format. The terminating sentinel
`0x00` byte is included in the output buffer.

When successful, this function returns the slice containing the
serialized and encoded message.

## Example

```rust
use postcard::to_slice_cobs;
let mut buf = [0u8; 32];

let used = to_slice_cobs(&false, &mut buf).unwrap();
assert_eq!(used, &[0x01, 0x01, 0x00]);

let used = to_slice_cobs("1", &mut buf).unwrap();
assert_eq!(used, &[0x03, 0x01, b'1', 0x00]);

let used = to_slice_cobs("Hi!", &mut buf).unwrap();
assert_eq!(used, &[0x05, 0x03, b'H', b'i', b'!', 0x00]);

let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
let used = to_slice_cobs(data, &mut buf).unwrap();
assert_eq!(used, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
```

### `to_allocvec`

```rust
fn to_allocvec<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: Serialize + ?Sized
```

Serialize a `T` to an `alloc::vec::Vec<u8>`.

## Example

```rust
use postcard::to_allocvec;

let ser: Vec<u8> = to_allocvec(&true).unwrap();
assert_eq!(ser.as_slice(), &[0x01]);

let ser: Vec<u8> = to_allocvec("Hi!").unwrap();
assert_eq!(ser.as_slice(), &[0x03, b'H', b'i', b'!']);
```

### `to_allocvec_cobs`

```rust
fn to_allocvec_cobs<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: Serialize + ?Sized
```

Serialize and COBS encode a `T` to an `alloc::vec::Vec<u8>`.

The terminating sentinel `0x00` byte is included in the output.

## Example

```rust
use postcard::to_allocvec_cobs;

let ser: Vec<u8> = to_allocvec_cobs(&true).unwrap();
assert_eq!(ser.as_slice(), &[0x02, 0x01, 0x00]);

let ser: Vec<u8> = to_allocvec_cobs("Hi!").unwrap();
assert_eq!(ser.as_slice(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);
```

## Type Aliases

### `Result<T>`

```rust
type Result<T> = ::core::result::Result<T, Error>;
```

This is the Result type used by Postcard.

