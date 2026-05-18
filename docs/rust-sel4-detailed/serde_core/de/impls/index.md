*[serde_core](../../index.md) / [de](../index.md) / [impls](index.md)*

---

# Module `impls`

## Contents

- [Modules](#modules)
  - [`range`](#range)
  - [`range_from`](#range-from)
  - [`range_to`](#range-to)
- [Structs](#structs)
  - [`UnitVisitor`](#unitvisitor)
  - [`BoolVisitor`](#boolvisitor)
  - [`CharVisitor`](#charvisitor)
  - [`StringVisitor`](#stringvisitor)
  - [`StringInPlaceVisitor`](#stringinplacevisitor)
  - [`StrVisitor`](#strvisitor)
  - [`BytesVisitor`](#bytesvisitor)
  - [`CStringVisitor`](#cstringvisitor)
  - [`OptionVisitor`](#optionvisitor)
  - [`PhantomDataVisitor`](#phantomdatavisitor)
  - [`ArrayVisitor`](#arrayvisitor)
  - [`ArrayInPlaceVisitor`](#arrayinplacevisitor)
  - [`PathVisitor`](#pathvisitor)
  - [`PathBufVisitor`](#pathbufvisitor)
  - [`OsStringVisitor`](#osstringvisitor)
  - [`FromStrVisitor`](#fromstrvisitor)
- [Enums](#enums)
  - [`OsStringKind`](#osstringkind)
- [Functions](#functions)
  - [`nop_reserve`](#nop-reserve)
- [Macros](#macros)
  - [`impl_deserialize_num!`](#impl-deserialize-num)
  - [`num_self!`](#num-self)
  - [`num_as_self!`](#num-as-self)
  - [`num_as_copysign_self!`](#num-as-copysign-self)
  - [`int_to_int!`](#int-to-int)
  - [`int_to_uint!`](#int-to-uint)
  - [`uint_to_self!`](#uint-to-self)
  - [`num_128!`](#num-128)
  - [`forwarded_impl!`](#forwarded-impl)
  - [`seq_impl!`](#seq-impl)
  - [`array_impls!`](#array-impls)
  - [`tuple_impls!`](#tuple-impls)
  - [`tuple_impl_body!`](#tuple-impl-body)
  - [`map_impl!`](#map-impl)
  - [`parse_ip_impl!`](#parse-ip-impl)
  - [`variant_identifier!`](#variant-identifier)
  - [`deserialize_enum!`](#deserialize-enum)
  - [`parse_socket_impl!`](#parse-socket-impl)
  - [`box_forwarded_impl!`](#box-forwarded-impl)
  - [`atomic_impl!`](#atomic-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`range`](#range) | mod |  |
| [`range_from`](#range-from) | mod |  |
| [`range_to`](#range-to) | mod |  |
| [`UnitVisitor`](#unitvisitor) | struct |  |
| [`BoolVisitor`](#boolvisitor) | struct |  |
| [`CharVisitor`](#charvisitor) | struct |  |
| [`StringVisitor`](#stringvisitor) | struct |  |
| [`StringInPlaceVisitor`](#stringinplacevisitor) | struct |  |
| [`StrVisitor`](#strvisitor) | struct |  |
| [`BytesVisitor`](#bytesvisitor) | struct |  |
| [`CStringVisitor`](#cstringvisitor) | struct |  |
| [`OptionVisitor`](#optionvisitor) | struct |  |
| [`PhantomDataVisitor`](#phantomdatavisitor) | struct |  |
| [`ArrayVisitor`](#arrayvisitor) | struct |  |
| [`ArrayInPlaceVisitor`](#arrayinplacevisitor) | struct |  |
| [`PathVisitor`](#pathvisitor) | struct |  |
| [`PathBufVisitor`](#pathbufvisitor) | struct |  |
| [`OsStringVisitor`](#osstringvisitor) | struct |  |
| [`FromStrVisitor`](#fromstrvisitor) | struct |  |
| [`OsStringKind`](#osstringkind) | enum |  |
| [`nop_reserve`](#nop-reserve) | fn |  |
| [`impl_deserialize_num!`](#impl-deserialize-num) | macro |  |
| [`num_self!`](#num-self) | macro |  |
| [`num_as_self!`](#num-as-self) | macro |  |
| [`num_as_copysign_self!`](#num-as-copysign-self) | macro |  |
| [`int_to_int!`](#int-to-int) | macro |  |
| [`int_to_uint!`](#int-to-uint) | macro |  |
| [`uint_to_self!`](#uint-to-self) | macro |  |
| [`num_128!`](#num-128) | macro |  |
| [`forwarded_impl!`](#forwarded-impl) | macro |  |
| [`seq_impl!`](#seq-impl) | macro |  |
| [`array_impls!`](#array-impls) | macro |  |
| [`tuple_impls!`](#tuple-impls) | macro |  |
| [`tuple_impl_body!`](#tuple-impl-body) | macro |  |
| [`map_impl!`](#map-impl) | macro |  |
| [`parse_ip_impl!`](#parse-ip-impl) | macro |  |
| [`variant_identifier!`](#variant-identifier) | macro |  |
| [`deserialize_enum!`](#deserialize-enum) | macro |  |
| [`parse_socket_impl!`](#parse-socket-impl) | macro |  |
| [`box_forwarded_impl!`](#box-forwarded-impl) | macro |  |
| [`atomic_impl!`](#atomic-impl) | macro |  |

## Modules

- [`range`](range/index.md)
- [`range_from`](range_from/index.md)
- [`range_to`](range_to/index.md)

## Structs

### `UnitVisitor`

```rust
struct UnitVisitor;
```

#### Trait Implementations

##### `impl Expected for UnitVisitor`

- <span id="unitvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for UnitVisitor`

- <span id="unitvisitor-visitor-type-value"></span>`type Value = ()`

- <span id="unitvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="unitvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `BoolVisitor`

```rust
struct BoolVisitor;
```

#### Trait Implementations

##### `impl Expected for BoolVisitor`

- <span id="boolvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for BoolVisitor`

- <span id="boolvisitor-visitor-type-value"></span>`type Value = bool`

- <span id="boolvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="boolvisitor-visitor-visit-bool"></span>`fn visit_bool<E>(self, v: bool) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `CharVisitor`

```rust
struct CharVisitor;
```

#### Trait Implementations

##### `impl Expected for CharVisitor`

- <span id="charvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for CharVisitor`

- <span id="charvisitor-visitor-type-value"></span>`type Value = char`

- <span id="charvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="charvisitor-visitor-visit-char"></span>`fn visit_char<E>(self, v: char) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="charvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `StringVisitor`

```rust
struct StringVisitor;
```

#### Trait Implementations

##### `impl Expected for StringVisitor`

- <span id="stringvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for StringVisitor`

- <span id="stringvisitor-visitor-type-value"></span>`type Value = String`

- <span id="stringvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `StringInPlaceVisitor<'a>`

```rust
struct StringInPlaceVisitor<'a>(&'a mut String);
```

#### Trait Implementations

##### `impl Expected for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-visitor-type-value"></span>`type Value = ()`

- <span id="stringinplacevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringinplacevisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `StrVisitor`

```rust
struct StrVisitor;
```

#### Trait Implementations

##### `impl Expected for StrVisitor`

- <span id="strvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for StrVisitor`

- <span id="strvisitor-visitor-type-value"></span>`type Value = &'a str`

- <span id="strvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="strvisitor-visitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="strvisitor-visitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `BytesVisitor`

```rust
struct BytesVisitor;
```

#### Trait Implementations

##### `impl Expected for BytesVisitor`

- <span id="bytesvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for BytesVisitor`

- <span id="bytesvisitor-visitor-type-value"></span>`type Value = &'a [u8]`

- <span id="bytesvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="bytesvisitor-visitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="bytesvisitor-visitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `CStringVisitor`

```rust
struct CStringVisitor;
```

#### Trait Implementations

##### `impl Expected for CStringVisitor`

- <span id="cstringvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for CStringVisitor`

- <span id="cstringvisitor-visitor-type-value"></span>`type Value = CString`

- <span id="cstringvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="cstringvisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

### `OptionVisitor<T>`

```rust
struct OptionVisitor<T> {
    marker: PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<T> Expected for OptionVisitor<T>`

- <span id="optionvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for OptionVisitor<T>`

- <span id="optionvisitor-visitor-type-value"></span>`type Value = Option<T>`

- <span id="optionvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="optionvisitor-visitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="optionvisitor-visitor-visit-some"></span>`fn visit_some<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`Visitor`](../index.md#visitor)

### `PhantomDataVisitor<T: ?Sized>`

```rust
struct PhantomDataVisitor<T: ?Sized> {
    marker: PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<T> Expected for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-visitor-type-value"></span>`type Value = PhantomData<T>`

- <span id="phantomdatavisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="phantomdatavisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `ArrayVisitor<A>`

```rust
struct ArrayVisitor<A> {
    marker: PhantomData<A>,
}
```

#### Implementations

- <span id="arrayvisitor-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Expected for ArrayVisitor<A>`

- <span id="arrayvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for ArrayVisitor<[T; 0]>`

- <span id="arrayvisitor-visitor-type-value"></span>`type Value = [T; 0]`

- <span id="arrayvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayvisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, _: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `ArrayInPlaceVisitor<'a, A: 'a>`

```rust
struct ArrayInPlaceVisitor<'a, A: 'a>(&'a mut A);
```

#### Trait Implementations

##### `impl Expected for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for ArrayInPlaceVisitor<'a, [T; 1]>`

- <span id="arrayinplacevisitor-visitor-type-value"></span>`type Value = ()`

- <span id="arrayinplacevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayinplacevisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `PathVisitor`

```rust
struct PathVisitor;
```

#### Trait Implementations

##### `impl Expected for PathVisitor`

- <span id="pathvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for PathVisitor`

- <span id="pathvisitor-visitor-type-value"></span>`type Value = &'a Path`

- <span id="pathvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathvisitor-visitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathvisitor-visitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `PathBufVisitor`

```rust
struct PathBufVisitor;
```

#### Trait Implementations

##### `impl Expected for PathBufVisitor`

- <span id="pathbufvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for PathBufVisitor`

- <span id="pathbufvisitor-visitor-type-value"></span>`type Value = PathBuf`

- <span id="pathbufvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathbufvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `OsStringVisitor`

```rust
struct OsStringVisitor;
```

#### Trait Implementations

##### `impl Expected for OsStringVisitor`

- <span id="osstringvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for OsStringVisitor`

- <span id="osstringvisitor-visitor-type-value"></span>`type Value = OsString`

- <span id="osstringvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="osstringvisitor-visitor-visit-enum"></span>`fn visit_enum<A>(self, data: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `FromStrVisitor<T>`

```rust
struct FromStrVisitor<T> {
    expecting: &'static str,
    ty: PhantomData<T>,
}
```

#### Implementations

- <span id="fromstrvisitor-new"></span>`fn new(expecting: &'static str) -> Self`

#### Trait Implementations

##### `impl<T> Expected for FromStrVisitor<T>`

- <span id="fromstrvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for FromStrVisitor<T>`

- <span id="fromstrvisitor-visitor-type-value"></span>`type Value = T`

- <span id="fromstrvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="fromstrvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

## Enums

### `OsStringKind`

```rust
enum OsStringKind {
    Unix,
    Windows,
}
```

#### Trait Implementations

##### `impl Deserialize for OsStringKind`

- <span id="osstringkind-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../index.md#deserializer)

##### `impl DeserializeOwned for OsStringKind`

## Functions

### `nop_reserve`

```rust
fn nop_reserve<T>(_seq: T, _n: usize)
```

## Macros

### `impl_deserialize_num!`

### `num_self!`

### `num_as_self!`

### `num_as_copysign_self!`

### `int_to_int!`

### `int_to_uint!`

### `uint_to_self!`

### `num_128!`

### `forwarded_impl!`

### `seq_impl!`

### `array_impls!`

### `tuple_impls!`

### `tuple_impl_body!`

### `map_impl!`

### `parse_ip_impl!`

### `variant_identifier!`

### `deserialize_enum!`

### `parse_socket_impl!`

### `box_forwarded_impl!`

### `atomic_impl!`

