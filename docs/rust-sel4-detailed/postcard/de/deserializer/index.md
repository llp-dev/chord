*[postcard](../../index.md) / [de](../index.md) / [deserializer](index.md)*

---

# Module `deserializer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Deserializer`](#deserializer) | struct | A `serde` compatible deserializer, generic over “Flavors” of deserializing plugins. |
| [`SeqAccess`](#seqaccess) | struct |  |
| [`MapAccess`](#mapaccess) | struct |  |
| [`de_zig_zag_i16`](#de-zig-zag-i16) | fn |  |
| [`de_zig_zag_i32`](#de-zig-zag-i32) | fn |  |
| [`de_zig_zag_i64`](#de-zig-zag-i64) | fn |  |
| [`de_zig_zag_i128`](#de-zig-zag-i128) | fn |  |

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

- <span id="deserializer-finalize"></span>`fn finalize(self) -> Result<<F as >::Remainder>` — [`Result`](../../error/index.md#result), [`Flavor`](../flavors/index.md#flavor)

  Return the remaining (unused) bytes in the Deserializer along with any

  additional data provided by the [`Flavor`](../flavors/index.md)

#### Trait Implementations

##### `impl<F: Flavor<'de>> Deserializer for &mut Deserializer<'de, F>`

- <span id="mut-deserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-deserializer-is-human-readable"></span>`fn is_human_readable(&self) -> bool`

- <span id="mut-deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, _visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, _visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

##### `impl<F: Flavor<'de>> EnumAccess for &mut Deserializer<'de, F>`

- <span id="mut-deserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-enumaccess-type-variant"></span>`type Variant = &mut Deserializer<'de, F>`

- <span id="mut-deserializer-enumaccess-variant-seed"></span>`fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](../../error/index.md#result)

##### `impl<F: Flavor<'de>> VariantAccess for &mut Deserializer<'de, F>`

- <span id="mut-deserializer-variantaccess-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-variantaccess-tuple-variant"></span>`fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mut-deserializer-variantaccess-struct-variant"></span>`fn struct_variant<V: Visitor<'de>>(self, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

### `SeqAccess<'a, 'b, F: Flavor<'b>>`

```rust
struct SeqAccess<'a, 'b, F: Flavor<'b>> {
    deserializer: &'a mut Deserializer<'b, F>,
    len: usize,
}
```

#### Trait Implementations

##### `impl<F: Flavor<'b>> SeqAccess for SeqAccess<'a, 'b, F>`

- <span id="seqaccess-seqaccess-type-error"></span>`type Error = Error`

- <span id="seqaccess-seqaccess-next-element-seed"></span>`fn next_element_seed<V: DeserializeSeed<'b>>(&mut self, seed: V) -> Result<Option<<V as >::Value>>` — [`Result`](../../error/index.md#result)

- <span id="seqaccess-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `MapAccess<'a, 'b, F: Flavor<'b>>`

```rust
struct MapAccess<'a, 'b, F: Flavor<'b>> {
    deserializer: &'a mut Deserializer<'b, F>,
    len: usize,
}
```

#### Trait Implementations

##### `impl<F: Flavor<'b>> MapAccess for MapAccess<'a, 'b, F>`

- <span id="mapaccess-mapaccess-type-error"></span>`type Error = Error`

- <span id="mapaccess-mapaccess-next-key-seed"></span>`fn next_key_seed<K: DeserializeSeed<'b>>(&mut self, seed: K) -> Result<Option<<K as >::Value>>` — [`Result`](../../error/index.md#result)

- <span id="mapaccess-mapaccess-next-value-seed"></span>`fn next_value_seed<V: DeserializeSeed<'b>>(&mut self, seed: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md#result)

- <span id="mapaccess-mapaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

## Functions

### `de_zig_zag_i16`

```rust
fn de_zig_zag_i16(n: u16) -> i16
```

### `de_zig_zag_i32`

```rust
fn de_zig_zag_i32(n: u32) -> i32
```

### `de_zig_zag_i64`

```rust
fn de_zig_zag_i64(n: u64) -> i64
```

### `de_zig_zag_i128`

```rust
fn de_zig_zag_i128(n: u128) -> i128
```

