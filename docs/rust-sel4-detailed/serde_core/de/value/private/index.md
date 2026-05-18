*[serde_core](../../../index.md) / [de](../../index.md) / [value](../index.md) / [private](index.md)*

---

# Module `private`

## Contents

- [Structs](#structs)
  - [`UnitOnly`](#unitonly)
  - [`MapAsEnum`](#mapasenum)
  - [`SeedTupleVariant`](#seedtuplevariant)
  - [`SeedStructVariant`](#seedstructvariant)
- [Traits](#traits)
  - [`Pair`](#pair)
- [Functions](#functions)
  - [`unit_only`](#unit-only)
  - [`map_as_enum`](#map-as-enum)
- [Type Aliases](#type-aliases)
  - [`First`](#first)
  - [`Second`](#second)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UnitOnly`](#unitonly) | struct |  |
| [`MapAsEnum`](#mapasenum) | struct |  |
| [`SeedTupleVariant`](#seedtuplevariant) | struct |  |
| [`SeedStructVariant`](#seedstructvariant) | struct |  |
| [`Pair`](#pair) | trait | Avoid having to restate the generic types on `MapDeserializer`. |
| [`unit_only`](#unit-only) | fn |  |
| [`map_as_enum`](#map-as-enum) | fn |  |
| [`First`](#first) | type |  |
| [`Second`](#second) | type |  |

## Structs

### `UnitOnly<E>`

```rust
struct UnitOnly<E> {
    marker: PhantomData<E>,
}
```

#### Trait Implementations

##### `impl<E> VariantAccess for UnitOnly<E>`

- <span id="unitonly-variantaccess-type-error"></span>`type Error = E`

- <span id="unitonly-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md#variantaccess)

- <span id="unitonly-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

- <span id="unitonly-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="unitonly-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

### `MapAsEnum<A>`

```rust
struct MapAsEnum<A> {
    map: A,
}
```

#### Trait Implementations

##### `impl<A> VariantAccess for MapAsEnum<A>`

- <span id="mapasenum-variantaccess-type-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapasenum-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md#variantaccess)

- <span id="mapasenum-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

- <span id="mapasenum-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="mapasenum-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

### `SeedTupleVariant<V>`

```rust
struct SeedTupleVariant<V> {
    len: usize,
    visitor: V,
}
```

#### Trait Implementations

##### `impl<V> DeserializeSeed for SeedTupleVariant<V>`

- <span id="seedtuplevariant-deserializeseed-type-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedtuplevariant-deserializeseed-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

### `SeedStructVariant<V>`

```rust
struct SeedStructVariant<V> {
    visitor: V,
}
```

#### Trait Implementations

##### `impl<V> DeserializeSeed for SeedStructVariant<V>`

- <span id="seedstructvariant-deserializeseed-type-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedstructvariant-deserializeseed-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

## Traits

### `Pair`

```rust
trait Pair { ... }
```

Avoid having to restate the generic types on `MapDeserializer`. The
`Iterator::Item` contains enough information to figure out K and V.

#### Associated Types

- `type First`

- `type Second`

#### Required Methods

- `fn split(self) -> (<Self as >::First, <Self as >::Second)`

#### Implementors

- `(A, B)`

## Functions

### `unit_only`

```rust
fn unit_only<T, E>(t: T) -> (T, UnitOnly<E>)
```

### `map_as_enum`

```rust
fn map_as_enum<A>(map: A) -> MapAsEnum<A>
```

## Type Aliases

### `First<T>`

```rust
type First<T> = <T as Pair>::First;
```

### `Second<T>`

```rust
type Second<T> = <T as Pair>::Second;
```

