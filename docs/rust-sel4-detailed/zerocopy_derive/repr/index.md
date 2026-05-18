*[zerocopy_derive](../index.md) / [repr](index.md)*

---

# Module `repr`

## Contents

- [Modules](#modules)
  - [`util`](#util)
- [Structs](#structs)
  - [`UnsupportedReprError`](#unsupportedreprerror)
  - [`UnrecognizedReprError`](#unrecognizedreprerror)
- [Enums](#enums)
  - [`Repr`](#repr)
  - [`CompoundRepr`](#compoundrepr)
  - [`PrimitiveRepr`](#primitiverepr)
  - [`AlignRepr`](#alignrepr)
  - [`RawRepr`](#rawrepr)
  - [`FromRawReprError`](#fromrawreprerror)
  - [`FromRawReprsError`](#fromrawreprserror)
  - [`FromAttrsError`](#fromattrserror)
- [Functions](#functions)
  - [`try_from_raw_reprs`](#try-from-raw-reprs)
- [Type Aliases](#type-aliases)
  - [`StructUnionRepr`](#structunionrepr)
  - [`EnumRepr`](#enumrepr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`util`](#util) | mod |  |
| [`UnsupportedReprError`](#unsupportedreprerror) | struct | The representation hint is not supported for the decorated type. |
| [`UnrecognizedReprError`](#unrecognizedreprerror) | struct | The representation hint could not be parsed or was unrecognized. |
| [`Repr`](#repr) | enum | The computed representation of a type. |
| [`CompoundRepr`](#compoundrepr) | enum | A compound representation: `repr(C)`, `repr(Rust)`, or `repr(Int)`. |
| [`PrimitiveRepr`](#primitiverepr) | enum | `repr(Int)` |
| [`AlignRepr`](#alignrepr) | enum | `repr(packed(...))` or `repr(align(...))` |
| [`RawRepr`](#rawrepr) | enum | The result of parsing a single `#[repr(...)]` attribute or a single directive inside a compound `#[repr(..., ...)]` attribute. |
| [`FromRawReprError`](#fromrawreprerror) | enum | The error from converting from a `RawRepr`. |
| [`FromRawReprsError`](#fromrawreprserror) | enum | The error from extracting a high-level repr type from a list of `RawRepr`s. |
| [`FromAttrsError`](#fromattrserror) | enum | The error returned from [`Repr::from_attrs`]. |
| [`try_from_raw_reprs`](#try-from-raw-reprs) | fn | Tries to extract a high-level repr from a list of `RawRepr`s. |
| [`StructUnionRepr`](#structunionrepr) | type | The representations which can legally appear on a struct or union type. |
| [`EnumRepr`](#enumrepr) | type | The representations which can legally appear on an enum type. |

## Modules

- [`util`](util/index.md)

## Structs

### `UnsupportedReprError`

```rust
struct UnsupportedReprError;
```

The representation hint is not supported for the decorated type.

### `UnrecognizedReprError`

```rust
struct UnrecognizedReprError;
```

The representation hint could not be parsed or was unrecognized.

## Enums

### `Repr<Prim, Packed>`

```rust
enum Repr<Prim, Packed> {
    Transparent(proc_macro2::Span),
    Compound(Spanned<CompoundRepr<Prim>>, Option<Spanned<AlignRepr<Packed>>>),
}
```

The computed representation of a type.

This is the result of processing all `#[repr(...)]` attributes on a type, if
any. A `Repr` is only capable of representing legal combinations of
`#[repr(...)]` attributes.

#### Variants

- **`Transparent`**

  `#[repr(transparent)]`

- **`Compound`**

  A compound representation: `repr(C)`, `repr(Rust)`, or `repr(Int)`
  optionally combined with `repr(packed(...))` or `repr(align(...))`

#### Implementations

- <span id="repr-repr-type-name"></span>`fn repr_type_name(&self) -> &str`

  Gets the name of this "repr type" - the non-align `repr(X)` that is used

  in prose to refer to this type.

  

  For example, we would refer to `#[repr(C, align(4))] struct Foo { ... }`

  as a "`repr(C)` struct".

- <span id="repr-is-transparent"></span>`fn is_transparent(&self) -> bool`

- <span id="repr-is-c"></span>`fn is_c(&self) -> bool`

- <span id="repr-is-primitive"></span>`fn is_primitive(&self) -> bool`

- <span id="repr-get-packed"></span>`fn get_packed(&self) -> Option<&Packed>`

- <span id="repr-get-align"></span>`fn get_align(&self) -> Option<Spanned<NonZeroU32>>` — [`Spanned`](util/index.md#spanned)

- <span id="repr-is-align-gt-1"></span>`fn is_align_gt_1(&self) -> bool`

- <span id="repr-unaligned-validate-no-align-gt-1"></span>`fn unaligned_validate_no_align_gt_1(&self) -> Result<(), Error>`

  When deriving `Unaligned`, validate that the decorated type has no

  `#[repr(align(N))]` attribute where `N > 1`. If no such attribute exists

  (including if `N == 1`), this returns `Ok(())`, and otherwise it returns

  a descriptive error.

#### Trait Implementations

##### `impl Spanned for Repr<Prim, Packed>`

- <span id="repr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl<Prim, Packed> ToTokens for Repr<Prim, Packed>`

- <span id="repr-totokens-to-tokens"></span>`fn to_tokens(&self, ts: &mut TokenStream)`

### `CompoundRepr<Prim>`

```rust
enum CompoundRepr<Prim> {
    C,
    Rust,
    Primitive(Prim),
}
```

A compound representation: `repr(C)`, `repr(Rust)`, or `repr(Int)`.

### `PrimitiveRepr`

```rust
enum PrimitiveRepr {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}
```

`repr(Int)`

#### Trait Implementations

##### `impl Clone for PrimitiveRepr`

- <span id="primitiverepr-clone"></span>`fn clone(&self) -> PrimitiveRepr` — [`PrimitiveRepr`](#primitiverepr)

##### `impl Copy for PrimitiveRepr`

##### `impl Inhabited for PrimitiveRepr`

### `AlignRepr<Packed>`

```rust
enum AlignRepr<Packed> {
    Packed(Packed),
    Align(core::num::NonZeroU32),
}
```

`repr(packed(...))` or `repr(align(...))`

### `RawRepr`

```rust
enum RawRepr {
    Transparent,
    C,
    Rust,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    Align(core::num::NonZeroU32),
    PackedN(core::num::NonZeroU32),
    Packed,
}
```

The result of parsing a single `#[repr(...)]` attribute or a single
directive inside a compound `#[repr(..., ...)]` attribute.

#### Implementations

- <span id="rawrepr-from-attrs"></span>`fn from_attrs(attrs: &[Attribute]) -> Result<Vec<Spanned<RawRepr>>, Spanned<UnrecognizedReprError>>` — [`Spanned`](util/index.md#spanned), [`RawRepr`](#rawrepr), [`UnrecognizedReprError`](#unrecognizedreprerror)

- <span id="rawrepr-from-meta"></span>`fn from_meta(meta: &Meta) -> Result<RawRepr, UnrecognizedReprError>` — [`RawRepr`](#rawrepr), [`UnrecognizedReprError`](#unrecognizedreprerror)

#### Trait Implementations

##### `impl Clone for RawRepr`

- <span id="rawrepr-clone"></span>`fn clone(&self) -> RawRepr` — [`RawRepr`](#rawrepr)

##### `impl Copy for RawRepr`

##### `impl Eq for RawRepr`

##### `impl PartialEq for RawRepr`

- <span id="rawrepr-partialeq-eq"></span>`fn eq(&self, other: &RawRepr) -> bool` — [`RawRepr`](#rawrepr)

##### `impl StructuralPartialEq for RawRepr`

### `FromRawReprError<E>`

```rust
enum FromRawReprError<E> {
    None,
    Err(E),
}
```

The error from converting from a `RawRepr`.

#### Variants

- **`None`**

  The `RawRepr` doesn't affect the high-level repr we're parsing (e.g.
  it's `align(...)` and we're parsing a `CompoundRepr`).

- **`Err`**

  The `RawRepr` is invalid for the high-level repr we're parsing (e.g.
  it's `packed` repr and we're parsing an `AlignRepr` for an enum type).

### `FromRawReprsError<E>`

```rust
enum FromRawReprsError<E> {
    Single(E),
    Conflict,
}
```

The error from extracting a high-level repr type from a list of `RawRepr`s.

#### Variants

- **`Single`**

  One of the `RawRepr`s is invalid for the high-level repr we're parsing
  (e.g. there's a `packed` repr and we're parsing an `AlignRepr` for an
  enum type).

- **`Conflict`**

  Two `RawRepr`s appear which both affect the high-level repr we're
  parsing (e.g., the list is `#[repr(align(2), packed)]`). Note that we
  conservatively treat redundant reprs as conflicting (e.g.
  `#[repr(packed, packed)]`).

### `FromAttrsError`

```rust
enum FromAttrsError {
    FromRawReprs(FromRawReprsError<UnsupportedReprError>),
    Unrecognized,
}
```

The error returned from `Repr::from_attrs`.

## Functions

### `try_from_raw_reprs`

```rust
fn try_from_raw_reprs<'a, E, R: TryFrom<RawRepr, Error = FromRawReprError<E>>>(r: impl IntoIterator<Item = &'a Spanned<RawRepr>>) -> Result<Option<Spanned<R>>, Spanned<FromRawReprsError<E>>>
```

Tries to extract a high-level repr from a list of `RawRepr`s.

## Type Aliases

### `StructUnionRepr`

```rust
type StructUnionRepr = Repr<core::convert::Infallible, core::num::NonZeroU32>;
```

The representations which can legally appear on a struct or union type.

### `EnumRepr`

```rust
type EnumRepr = Repr<PrimitiveRepr, core::convert::Infallible>;
```

The representations which can legally appear on an enum type.

