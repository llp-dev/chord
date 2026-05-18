*[bytemuck_derive](../index.md) / [traits](index.md)*

---

# Module `traits`

## Contents

- [Structs](#structs)
  - [`Pod`](#pod)
  - [`AnyBitPattern`](#anybitpattern)
  - [`Zeroable`](#zeroable)
  - [`NoUninit`](#nouninit)
  - [`CheckedBitPattern`](#checkedbitpattern)
  - [`TransparentWrapper`](#transparentwrapper)
  - [`WrappedType`](#wrappedtype)
  - [`Contiguous`](#contiguous)
  - [`Representation`](#representation)
  - [`VariantDiscriminantIterator`](#variantdiscriminantiterator)
- [Enums](#enums)
  - [`IntegerRepr`](#integerrepr)
  - [`Repr`](#repr)
- [Traits](#traits)
  - [`Derivable`](#derivable)
- [Functions](#functions)
  - [`get_zero_variant`](#get-zero-variant)
  - [`get_struct_fields`](#get-struct-fields)
  - [`get_fields`](#get-fields)
  - [`get_enum_variants`](#get-enum-variants)
  - [`get_field_types`](#get-field-types)
  - [`generate_checked_bit_pattern_struct`](#generate-checked-bit-pattern-struct)
  - [`generate_checked_bit_pattern_enum`](#generate-checked-bit-pattern-enum)
  - [`generate_checked_bit_pattern_enum_without_fields`](#generate-checked-bit-pattern-enum-without-fields)
  - [`generate_checked_bit_pattern_enum_with_fields`](#generate-checked-bit-pattern-enum-with-fields)
  - [`generate_assert_no_padding`](#generate-assert-no-padding)
  - [`generate_fields_are_trait`](#generate-fields-are-trait)
  - [`get_enum_discriminant`](#get-enum-discriminant)
  - [`generate_enum_discriminant`](#generate-enum-discriminant)
  - [`get_wrapped_type_from_stream`](#get-wrapped-type-from-stream)
  - [`get_type_from_simple_attr`](#get-type-from-simple-attr)
  - [`get_repr`](#get-repr)
  - [`enum_has_fields`](#enum-has-fields)
  - [`parse_int_expr`](#parse-int-expr)
  - [`bytemuck_crate_name`](#bytemuck-crate-name)
- [Constants](#constants)
  - [`GENERATED_TYPE_DOCUMENTATION`](#generated-type-documentation)
- [Macros](#macros)
  - [`bail!`](#bail)
  - [`mk_repr!`](#mk-repr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pod`](#pod) | struct |  |
| [`AnyBitPattern`](#anybitpattern) | struct |  |
| [`Zeroable`](#zeroable) | struct |  |
| [`NoUninit`](#nouninit) | struct |  |
| [`CheckedBitPattern`](#checkedbitpattern) | struct |  |
| [`TransparentWrapper`](#transparentwrapper) | struct |  |
| [`WrappedType`](#wrappedtype) | struct |  |
| [`Contiguous`](#contiguous) | struct |  |
| [`Representation`](#representation) | struct |  |
| [`VariantDiscriminantIterator`](#variantdiscriminantiterator) | struct |  |
| [`IntegerRepr`](#integerrepr) | enum |  |
| [`Repr`](#repr) | enum |  |
| [`Derivable`](#derivable) | trait |  |
| [`get_zero_variant`](#get-zero-variant) | fn | Helper function to get the variant with discriminant zero (implicit or explicit). |
| [`get_struct_fields`](#get-struct-fields) | fn |  |
| [`get_fields`](#get-fields) | fn | Extract the `Fields` off a `DeriveInput`, or, in the `enum` case, off those of the `enum_variant`, when provided (e.g., for `Zeroable`). |
| [`get_enum_variants`](#get-enum-variants) | fn |  |
| [`get_field_types`](#get-field-types) | fn |  |
| [`generate_checked_bit_pattern_struct`](#generate-checked-bit-pattern-struct) | fn |  |
| [`generate_checked_bit_pattern_enum`](#generate-checked-bit-pattern-enum) | fn |  |
| [`generate_checked_bit_pattern_enum_without_fields`](#generate-checked-bit-pattern-enum-without-fields) | fn |  |
| [`generate_checked_bit_pattern_enum_with_fields`](#generate-checked-bit-pattern-enum-with-fields) | fn |  |
| [`generate_assert_no_padding`](#generate-assert-no-padding) | fn | Check that a struct or enum has no padding by asserting that the size of the type is equal to the sum of the size of it's fields and discriminant (for enums, this must be asserted for each variant). |
| [`generate_fields_are_trait`](#generate-fields-are-trait) | fn | Check that all fields implement a given trait |
| [`get_enum_discriminant`](#get-enum-discriminant) | fn | Get the type of an enum's discriminant. |
| [`generate_enum_discriminant`](#generate-enum-discriminant) | fn |  |
| [`get_wrapped_type_from_stream`](#get-wrapped-type-from-stream) | fn |  |
| [`get_type_from_simple_attr`](#get-type-from-simple-attr) | fn | get a simple `#[foo(bar)]` attribute, returning `bar` |
| [`get_repr`](#get-repr) | fn |  |
| [`enum_has_fields`](#enum-has-fields) | fn |  |
| [`parse_int_expr`](#parse-int-expr) | fn |  |
| [`bytemuck_crate_name`](#bytemuck-crate-name) | fn |  |
| [`GENERATED_TYPE_DOCUMENTATION`](#generated-type-documentation) | const |  |
| [`bail!`](#bail) | macro |  |
| [`mk_repr!`](#mk-repr) | macro |  |

## Structs

### `Pod`

```rust
struct Pod;
```

#### Trait Implementations

##### `impl Derivable for Pod`

- <span id="pod-derivable-ident"></span>`fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

- <span id="pod-derivable-asserts"></span>`fn asserts(input: &DeriveInput, crate_name: &TokenStream) -> Result<TokenStream>`

- <span id="pod-derivable-check-attributes"></span>`fn check_attributes(_ty: &Data, attributes: &[Attribute]) -> Result<()>`

### `AnyBitPattern`

```rust
struct AnyBitPattern;
```

#### Trait Implementations

##### `impl Derivable for AnyBitPattern`

- <span id="anybitpattern-derivable-ident"></span>`fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

- <span id="anybitpattern-derivable-implies-trait"></span>`fn implies_trait(crate_name: &TokenStream) -> Option<TokenStream>`

- <span id="anybitpattern-derivable-asserts"></span>`fn asserts(input: &DeriveInput, crate_name: &TokenStream) -> Result<TokenStream>`

### `Zeroable`

```rust
struct Zeroable;
```

#### Trait Implementations

##### `impl Derivable for Zeroable`

- <span id="zeroable-derivable-ident"></span>`fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

- <span id="zeroable-derivable-check-attributes"></span>`fn check_attributes(ty: &Data, attributes: &[Attribute]) -> Result<()>`

- <span id="zeroable-derivable-asserts"></span>`fn asserts(input: &DeriveInput, crate_name: &TokenStream) -> Result<TokenStream>`

- <span id="zeroable-derivable-explicit-bounds-attribute-name"></span>`fn explicit_bounds_attribute_name() -> Option<&'static str>`

- <span id="zeroable-derivable-perfect-derive-fields"></span>`fn perfect_derive_fields(input: &DeriveInput) -> Option<Fields>`

### `NoUninit`

```rust
struct NoUninit;
```

#### Trait Implementations

##### `impl Derivable for NoUninit`

- <span id="nouninit-derivable-ident"></span>`fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

- <span id="nouninit-derivable-check-attributes"></span>`fn check_attributes(ty: &Data, attributes: &[Attribute]) -> Result<()>`

- <span id="nouninit-derivable-asserts"></span>`fn asserts(input: &DeriveInput, crate_name: &TokenStream) -> Result<TokenStream>`

- <span id="nouninit-derivable-trait-impl"></span>`fn trait_impl(_input: &DeriveInput, _crate_name: &TokenStream) -> Result<(TokenStream, TokenStream)>`

### `CheckedBitPattern`

```rust
struct CheckedBitPattern;
```

#### Trait Implementations

##### `impl Derivable for CheckedBitPattern`

- <span id="checkedbitpattern-derivable-ident"></span>`fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

- <span id="checkedbitpattern-derivable-check-attributes"></span>`fn check_attributes(ty: &Data, attributes: &[Attribute]) -> Result<()>`

- <span id="checkedbitpattern-derivable-asserts"></span>`fn asserts(input: &DeriveInput, crate_name: &TokenStream) -> Result<TokenStream>`

- <span id="checkedbitpattern-derivable-trait-impl"></span>`fn trait_impl(input: &DeriveInput, crate_name: &TokenStream) -> Result<(TokenStream, TokenStream)>`

### `TransparentWrapper`

```rust
struct TransparentWrapper;
```

#### Implementations

- <span id="transparentwrapper-get-wrapped-type"></span>`fn get_wrapped_type(attributes: &[Attribute], fields: &Fields) -> Option<WrappedType>` — [`WrappedType`](#wrappedtype)

#### Trait Implementations

##### `impl Derivable for TransparentWrapper`

- <span id="transparentwrapper-derivable-ident"></span>`fn ident(input: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

- <span id="transparentwrapper-derivable-asserts"></span>`fn asserts(input: &DeriveInput, crate_name: &TokenStream) -> Result<TokenStream>`

- <span id="transparentwrapper-derivable-check-attributes"></span>`fn check_attributes(_ty: &Data, attributes: &[Attribute]) -> Result<()>`

- <span id="transparentwrapper-derivable-requires-where-clause"></span>`fn requires_where_clause() -> bool`

### `WrappedType`

```rust
struct WrappedType {
    wrapped_type: syn::Type,
    explicit: bool,
}
```

#### Fields

- **`explicit`**: `bool`

  Was the type given with a #[transparent(Type)] attribute.

### `Contiguous`

```rust
struct Contiguous;
```

#### Trait Implementations

##### `impl Derivable for Contiguous`

- <span id="contiguous-derivable-ident"></span>`fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

- <span id="contiguous-derivable-trait-impl"></span>`fn trait_impl(input: &DeriveInput, _crate_name: &TokenStream) -> Result<(TokenStream, TokenStream)>`

### `Representation`

```rust
struct Representation {
    packed: Option<u32>,
    align: Option<u32>,
    repr: Repr,
}
```

#### Trait Implementations

##### `impl Clone for Representation`

- <span id="representation-clone"></span>`fn clone(&self) -> Representation` — [`Representation`](#representation)

##### `impl Copy for Representation`

##### `impl Debug for Representation`

- <span id="representation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Representation`

- <span id="representation-default"></span>`fn default() -> Self`

##### `impl Eq for Representation`

##### `impl Parse for Representation`

- <span id="representation-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Representation>` — [`Representation`](#representation)

##### `impl PartialEq for Representation`

- <span id="representation-partialeq-eq"></span>`fn eq(&self, other: &Representation) -> bool` — [`Representation`](#representation)

##### `impl Spanned for Representation`

- <span id="representation-spanned-span"></span>`fn span(&self) -> Span`

##### `impl StructuralPartialEq for Representation`

##### `impl ToTokens for Representation`

- <span id="representation-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `VariantDiscriminantIterator<'a, I: Iterator<Item = &'a Variant> + 'a>`

```rust
struct VariantDiscriminantIterator<'a, I: Iterator<Item = &'a Variant> + 'a> {
    inner: I,
    last_value: i128,
}
```

#### Implementations

- <span id="variantdiscriminantiterator-new"></span>`fn new(inner: I) -> Self`

#### Trait Implementations

##### `impl<I> IntoIterator for VariantDiscriminantIterator<'a, I>`

- <span id="variantdiscriminantiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="variantdiscriminantiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="variantdiscriminantiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = &'a Variant> + 'a> Iterator for VariantDiscriminantIterator<'a, I>`

- <span id="variantdiscriminantiterator-iterator-type-item"></span>`type Item = Result<(i128, &'a Variant), Error>`

- <span id="variantdiscriminantiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `IntegerRepr`

```rust
enum IntegerRepr {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    I128,
    U128,
    Usize,
    Isize,
}
```

#### Trait Implementations

##### `impl Clone for IntegerRepr`

- <span id="integerrepr-clone"></span>`fn clone(&self) -> IntegerRepr` — [`IntegerRepr`](#integerrepr)

##### `impl Copy for IntegerRepr`

##### `impl Debug for IntegerRepr`

- <span id="integerrepr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IntegerRepr`

##### `impl PartialEq for IntegerRepr`

- <span id="integerrepr-partialeq-eq"></span>`fn eq(&self, other: &IntegerRepr) -> bool` — [`IntegerRepr`](#integerrepr)

##### `impl Spanned for IntegerRepr`

- <span id="integerrepr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl StructuralPartialEq for IntegerRepr`

##### `impl ToTokens for IntegerRepr`

- <span id="integerrepr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Repr`

```rust
enum Repr {
    Rust,
    C,
    Transparent,
    Integer(IntegerRepr),
    CWithDiscriminant(IntegerRepr),
}
```

#### Implementations

- <span id="repr-as-integer"></span>`fn as_integer(&self) -> Option<IntegerRepr>` — [`IntegerRepr`](#integerrepr)

#### Trait Implementations

##### `impl Clone for Repr`

- <span id="repr-clone"></span>`fn clone(&self) -> Repr` — [`Repr`](#repr)

##### `impl Copy for Repr`

##### `impl Debug for Repr`

- <span id="repr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Repr`

##### `impl PartialEq for Repr`

- <span id="repr-partialeq-eq"></span>`fn eq(&self, other: &Repr) -> bool` — [`Repr`](#repr)

##### `impl StructuralPartialEq for Repr`

## Traits

### `Derivable`

```rust
trait Derivable { ... }
```

#### Required Methods

- `fn ident(input: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>`

#### Provided Methods

- `fn implies_trait(_crate_name: &TokenStream) -> Option<TokenStream>`

- `fn asserts(_input: &DeriveInput, _crate_name: &TokenStream) -> Result<TokenStream>`

- `fn check_attributes(_ty: &Data, _attributes: &[Attribute]) -> Result<()>`

- `fn trait_impl(_input: &DeriveInput, _crate_name: &TokenStream) -> Result<(TokenStream, TokenStream)>`

- `fn requires_where_clause() -> bool`

- `fn explicit_bounds_attribute_name() -> Option<&'static str>`

- `fn perfect_derive_fields(_input: &DeriveInput) -> Option<Fields>`

  If this trait has a custom meaning for "perfect derive", this function

#### Implementors

- [`AnyBitPattern`](#anybitpattern)
- [`CheckedBitPattern`](#checkedbitpattern)
- [`Contiguous`](#contiguous)
- [`NoUninit`](#nouninit)
- [`Pod`](#pod)
- [`TransparentWrapper`](#transparentwrapper)
- [`Zeroable`](#zeroable)

## Functions

### `get_zero_variant`

```rust
fn get_zero_variant(enum_: &DataEnum) -> syn::Result<Option<&Variant>>
```

Helper function to get the variant with discriminant zero (implicit or
explicit).

### `get_struct_fields`

```rust
fn get_struct_fields(input: &DeriveInput) -> syn::Result<&Fields>
```

### `get_fields`

```rust
fn get_fields(input: &DeriveInput, enum_variant: Option<&Variant>) -> syn::Result<Fields>
```

Extract the `Fields` off a `DeriveInput`, or, in the `enum` case, off
those of the `enum_variant`, when provided (e.g., for `Zeroable`).

We purposely allow not providing an `enum_variant` for cases where
the caller wants to reject supporting `enum`s (e.g., `NoPadding`).

### `get_enum_variants`

```rust
fn get_enum_variants<'a>(input: &'a DeriveInput) -> syn::Result<impl Iterator<Item = &'a Variant> + Clone + 'a>
```

### `get_field_types`

```rust
fn get_field_types<'a>(fields: &'a Fields) -> impl Iterator<Item = &'a Type> + 'a
```

### `generate_checked_bit_pattern_struct`

```rust
fn generate_checked_bit_pattern_struct(input_ident: &proc_macro2::Ident, fields: &Fields, attrs: &[Attribute], crate_name: &proc_macro2::TokenStream) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)>
```

### `generate_checked_bit_pattern_enum`

```rust
fn generate_checked_bit_pattern_enum(input: &DeriveInput, variants: &syn::punctuated::Punctuated<Variant, token::Comma>, crate_name: &proc_macro2::TokenStream) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)>
```

### `generate_checked_bit_pattern_enum_without_fields`

```rust
fn generate_checked_bit_pattern_enum_without_fields(input: &DeriveInput, variants: &syn::punctuated::Punctuated<Variant, token::Comma>, crate_name: &proc_macro2::TokenStream) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)>
```

### `generate_checked_bit_pattern_enum_with_fields`

```rust
fn generate_checked_bit_pattern_enum_with_fields(input: &DeriveInput, variants: &syn::punctuated::Punctuated<Variant, token::Comma>, crate_name: &proc_macro2::TokenStream) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)>
```

### `generate_assert_no_padding`

```rust
fn generate_assert_no_padding(input: &DeriveInput, enum_variant: Option<&Variant>, for_trait: &str) -> syn::Result<proc_macro2::TokenStream>
```

Check that a struct or enum has no padding by asserting that the size of
the type is equal to the sum of the size of it's fields and discriminant
(for enums, this must be asserted for each variant).

### `generate_fields_are_trait`

```rust
fn generate_fields_are_trait(input: &DeriveInput, enum_variant: Option<&Variant>, trait_: syn::Path) -> syn::Result<proc_macro2::TokenStream>
```

Check that all fields implement a given trait

### `get_enum_discriminant`

```rust
fn get_enum_discriminant(input: &DeriveInput, crate_name: &proc_macro2::TokenStream) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)>
```

Get the type of an enum's discriminant.

For `repr(int)` and `repr(C, int)` enums, this will return the known bare
integer type specified.

For `repr(C)` enums, this will extract the underlying size chosen by rustc.
It will return a token stream which is a type expression that evaluates to
a primitive integer type of this size, using our `EnumTagIntegerBytes`
trait.

For fieldless `repr(C)` enums, we can feed the size of the enum directly
into the trait.

For `repr(C)` enums with fields, we generate a new fieldless `repr(C)` enum
with the same variants, then use that in the calculation. This is the
specified behavior, see https://doc.rust-lang.org/stable/reference/type-layout.html#reprc-enums-with-fields

Returns a tuple of (type ident, auxiliary definitions)

### `generate_enum_discriminant`

```rust
fn generate_enum_discriminant(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```

### `get_wrapped_type_from_stream`

```rust
fn get_wrapped_type_from_stream(tokens: proc_macro2::TokenStream) -> Option<syn::Type>
```

### `get_type_from_simple_attr`

```rust
fn get_type_from_simple_attr(attributes: &[Attribute], attr_name: &str) -> Option<syn::Type>
```

get a simple `#[foo(bar)]` attribute, returning `bar`

### `get_repr`

```rust
fn get_repr(attributes: &[Attribute]) -> syn::Result<Representation>
```

### `enum_has_fields`

```rust
fn enum_has_fields<'a>(variants: impl Iterator<Item = &'a Variant>) -> bool
```

### `parse_int_expr`

```rust
fn parse_int_expr(expr: &Expr) -> syn::Result<i128>
```

### `bytemuck_crate_name`

```rust
fn bytemuck_crate_name(input: &DeriveInput) -> proc_macro2::TokenStream
```

## Constants

### `GENERATED_TYPE_DOCUMENTATION`
```rust
const GENERATED_TYPE_DOCUMENTATION: &str;
```

## Macros

### `bail!`

### `mk_repr!`

