*[postcard](../../index.md) / [ser](../index.md) / [serializer](index.md)*

---

# Module `serializer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Serializer`](#serializer) | struct | A `serde` compatible serializer, generic over "Flavors" of serializing plugins. |
| [`zig_zag_i16`](#zig-zag-i16) | fn |  |
| [`zig_zag_i32`](#zig-zag-i32) | fn |  |
| [`zig_zag_i64`](#zig-zag-i64) | fn |  |
| [`zig_zag_i128`](#zig-zag-i128) | fn |  |

## Structs

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

- <span id="serializer-try-push-varint-usize"></span>`fn try_push_varint_usize(&mut self, data: usize) -> Result<()>` — [`Result`](../../error/index.md#result)

  Attempt to push a variably encoded [usize] into the output data stream

- <span id="serializer-try-push-varint-u128"></span>`fn try_push_varint_u128(&mut self, data: u128) -> Result<()>` — [`Result`](../../error/index.md#result)

  Attempt to push a variably encoded [`u128`](../../../ring/aead/gcm/index.md) into the output data stream

- <span id="serializer-try-push-varint-u64"></span>`fn try_push_varint_u64(&mut self, data: u64) -> Result<()>` — [`Result`](../../error/index.md#result)

  Attempt to push a variably encoded [u64] into the output data stream

- <span id="serializer-try-push-varint-u32"></span>`fn try_push_varint_u32(&mut self, data: u32) -> Result<()>` — [`Result`](../../error/index.md#result)

  Attempt to push a variably encoded [u32] into the output data stream

- <span id="serializer-try-push-varint-u16"></span>`fn try_push_varint_u16(&mut self, data: u16) -> Result<()>` — [`Result`](../../error/index.md#result)

  Attempt to push a variably encoded [`u16`](../../../gimli/leb128/read/index.md) into the output data stream

#### Trait Implementations

##### `impl<F> SerializeMap for &mut Serializer<F>`

- <span id="mut-serializer-serializemap-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializemap-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializemap-serialize-key"></span>`fn serialize_key<T>(&mut self, key: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializemap-serialize-value"></span>`fn serialize_value<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializemap-end"></span>`fn end(self) -> Result<()>` — [`Result`](../../error/index.md#result)

##### `impl<F> SerializeSeq for &mut Serializer<F>`

- <span id="mut-serializer-serializeseq-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializeseq-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializeseq-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializeseq-end"></span>`fn end(self) -> Result<()>` — [`Result`](../../error/index.md#result)

##### `impl<F> SerializeStruct for &mut Serializer<F>`

- <span id="mut-serializer-serializestruct-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializestruct-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializestruct-serialize-field"></span>`fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializestruct-end"></span>`fn end(self) -> Result<()>` — [`Result`](../../error/index.md#result)

##### `impl<F> SerializeStructVariant for &mut Serializer<F>`

- <span id="mut-serializer-serializestructvariant-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializestructvariant-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializestructvariant-serialize-field"></span>`fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializestructvariant-end"></span>`fn end(self) -> Result<()>` — [`Result`](../../error/index.md#result)

##### `impl<F> SerializeTuple for &mut Serializer<F>`

- <span id="mut-serializer-serializetuple-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializetuple-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializetuple-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializetuple-end"></span>`fn end(self) -> Result<()>` — [`Result`](../../error/index.md#result)

##### `impl<F> SerializeTupleStruct for &mut Serializer<F>`

- <span id="mut-serializer-serializetuplestruct-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializetuplestruct-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializetuplestruct-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializetuplestruct-end"></span>`fn end(self) -> Result<()>` — [`Result`](../../error/index.md#result)

##### `impl<F> SerializeTupleVariant for &mut Serializer<F>`

- <span id="mut-serializer-serializetuplevariant-type-ok"></span>`type Ok = ()`

- <span id="mut-serializer-serializetuplevariant-type-error"></span>`type Error = Error`

- <span id="mut-serializer-serializetuplevariant-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializetuplevariant-end"></span>`fn end(self) -> Result<()>` — [`Result`](../../error/index.md#result)

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

- <span id="mut-serializer-serializer-serialize-bool"></span>`fn serialize_bool(self, v: bool) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i8"></span>`fn serialize_i8(self, v: i8) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i16"></span>`fn serialize_i16(self, v: i16) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i32"></span>`fn serialize_i32(self, v: i32) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i64"></span>`fn serialize_i64(self, v: i64) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-i128"></span>`fn serialize_i128(self, v: i128) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u8"></span>`fn serialize_u8(self, v: u8) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u16"></span>`fn serialize_u16(self, v: u16) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u32"></span>`fn serialize_u32(self, v: u32) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u64"></span>`fn serialize_u64(self, v: u64) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-u128"></span>`fn serialize_u128(self, v: u128) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-f32"></span>`fn serialize_f32(self, v: f32) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-f64"></span>`fn serialize_f64(self, v: f64) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-char"></span>`fn serialize_char(self, v: char) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-str"></span>`fn serialize_str(self, v: &str) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-bytes"></span>`fn serialize_bytes(self, v: &[u8]) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-none"></span>`fn serialize_none(self) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, variant_index: u32, _variant: &'static str, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-tuple"></span>`fn serialize_tuple(self, _len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-struct"></span>`fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../../error/index.md#result)

- <span id="mut-serializer-serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<<Self as >::Ok>` — [`Result`](../../error/index.md#result)

## Functions

### `zig_zag_i16`

```rust
fn zig_zag_i16(n: i16) -> u16
```

### `zig_zag_i32`

```rust
fn zig_zag_i32(n: i32) -> u32
```

### `zig_zag_i64`

```rust
fn zig_zag_i64(n: i64) -> u64
```

### `zig_zag_i128`

```rust
fn zig_zag_i128(n: i128) -> u128
```

