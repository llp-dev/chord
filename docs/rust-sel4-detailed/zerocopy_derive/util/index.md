*[zerocopy_derive](../index.md) / [util](index.md)*

---

# Module `util`

## Contents

- [Structs](#structs)
  - [`Ctx`](#ctx)
  - [`ImplBlockBuilder`](#implblockbuilder)
- [Enums](#enums)
  - [`PaddingCheck`](#paddingcheck)
  - [`Trait`](#trait)
  - [`TraitBound`](#traitbound)
  - [`FieldBounds`](#fieldbounds)
  - [`SelfBounds`](#selfbounds)
- [Traits](#traits)
  - [`DataExt`](#dataext)
  - [`BoolExt`](#boolext)
- [Functions](#functions)
  - [`map_fields`](#map-fields)
  - [`to_ident_str`](#to-ident-str)
  - [`normalize_bounds`](#normalize-bounds)
  - [`const_block`](#const-block)
  - [`generate_tag_enum`](#generate-tag-enum)
  - [`enum_size_from_repr`](#enum-size-from-repr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ctx`](#ctx) | struct |  |
| [`ImplBlockBuilder`](#implblockbuilder) | struct |  |
| [`PaddingCheck`](#paddingcheck) | enum | This enum describes what kind of padding check needs to be generated for the associated impl. |
| [`Trait`](#trait) | enum |  |
| [`TraitBound`](#traitbound) | enum |  |
| [`FieldBounds`](#fieldbounds) | enum |  |
| [`SelfBounds`](#selfbounds) | enum |  |
| [`DataExt`](#dataext) | trait |  |
| [`BoolExt`](#boolext) | trait |  |
| [`map_fields`](#map-fields) | fn |  |
| [`to_ident_str`](#to-ident-str) | fn |  |
| [`normalize_bounds`](#normalize-bounds) | fn | Normalizes a slice of bounds by replacing [`TraitBound::Slf`] with `slf`. |
| [`const_block`](#const-block) | fn |  |
| [`generate_tag_enum`](#generate-tag-enum) | fn |  |
| [`enum_size_from_repr`](#enum-size-from-repr) | fn |  |

## Structs

### `Ctx`

```rust
struct Ctx {
    ast: syn::DeriveInput,
    zerocopy_crate: syn::Path,
    skip_on_error: bool,
    on_error_span: Option<proc_macro2::Span>,
}
```

#### Implementations

- <span id="ctx-try-from-derive-input"></span>`fn try_from_derive_input(ast: DeriveInput) -> Result<Self, Error>`

  Attempt to extract a crate path from the provided attributes. Defaults to

  `::zerocopy` if not found.

- <span id="ctx-with-input"></span>`fn with_input(&self, input: &DeriveInput) -> Self`

- <span id="ctx-core-path"></span>`fn core_path(&self) -> TokenStream`

- <span id="ctx-cfg-compile-error"></span>`fn cfg_compile_error(&self) -> TokenStream`

- <span id="ctx-error-or-skip"></span>`fn error_or_skip<E>(&self, error: E) -> Result<TokenStream, E>`

### `ImplBlockBuilder<'a>`

```rust
struct ImplBlockBuilder<'a> {
    ctx: &'a Ctx,
    data: &'a dyn DataExt,
    trt: Trait,
    field_type_trait_bounds: FieldBounds<'a>,
    self_type_trait_bounds: SelfBounds<'a>,
    padding_check: Option<PaddingCheck>,
    param_extras: Vec<syn::GenericParam>,
    inner_extras: Option<proc_macro2::TokenStream>,
    outer_extras: Option<proc_macro2::TokenStream>,
}
```

#### Implementations

- <span id="implblockbuilder-new"></span>`fn new(ctx: &'a Ctx, data: &'a dyn DataExt, trt: Trait, field_type_trait_bounds: FieldBounds<'a>) -> Self` — [`Ctx`](#ctx), [`DataExt`](#dataext), [`Trait`](#trait), [`FieldBounds`](#fieldbounds)

- <span id="implblockbuilder-self-type-trait-bounds"></span>`fn self_type_trait_bounds(self, self_type_trait_bounds: SelfBounds<'a>) -> Self` — [`SelfBounds`](#selfbounds)

- <span id="implblockbuilder-padding-check"></span>`fn padding_check<P: Into<Option<PaddingCheck>>>(self, padding_check: P) -> Self`

- <span id="implblockbuilder-param-extras"></span>`fn param_extras(self, param_extras: Vec<GenericParam>) -> Self`

- <span id="implblockbuilder-inner-extras"></span>`fn inner_extras(self, inner_extras: TokenStream) -> Self`

- <span id="implblockbuilder-outer-extras"></span>`fn outer_extras<T: Into<Option<TokenStream>>>(self, outer_extras: T) -> Self`

- <span id="implblockbuilder-build"></span>`fn build(self) -> TokenStream`

## Enums

### `PaddingCheck`

```rust
enum PaddingCheck {
    Struct,
    ReprCStruct,
    Union,
    Enum {
        tag_type_definition: proc_macro2::TokenStream,
    },
}
```

This enum describes what kind of padding check needs to be generated for the
associated impl.

#### Variants

- **`Struct`**

  Check that the sum of the fields' sizes exactly equals the struct's
  size.

- **`ReprCStruct`**

  Check that a `repr(C)` struct has no padding.

- **`Union`**

  Check that the size of each field exactly equals the union's size.

- **`Enum`**

  Check that every variant of the enum contains no padding.
  
  Because doing so requires a tag enum, this padding check requires an
  additional `TokenStream` which defines the tag enum as `___ZerocopyTag`.

#### Implementations

- <span id="paddingcheck-validator-trait-and-macro-idents"></span>`fn validator_trait_and_macro_idents(&self) -> (Ident, Ident)`

  Returns the idents of the trait to use and the macro to call in order to

  validate that a type passes the relevant padding check.

- <span id="paddingcheck-validator-macro-context"></span>`fn validator_macro_context(&self) -> Option<&TokenStream>`

  Sometimes performing the padding check requires some additional

  "context" code. For enums, this is the definition of the tag enum.

### `Trait`

```rust
enum Trait {
    KnownLayout,
    HasTag,
    HasField {
        variant_id: Box<syn::Expr>,
        field: Box<syn::Type>,
        field_id: Box<syn::Expr>,
    },
    ProjectField {
        variant_id: Box<syn::Expr>,
        field: Box<syn::Type>,
        field_id: Box<syn::Expr>,
        invariants: Box<syn::Type>,
    },
    Immutable,
    TryFromBytes,
    FromZeros,
    FromBytes,
    IntoBytes,
    Unaligned,
    Sized,
    ByteHash,
    ByteEq,
    SplitAt,
}
```

#### Implementations

- <span id="trait-crate-path"></span>`fn crate_path(&self, ctx: &Ctx) -> Path` — [`Ctx`](#ctx)

#### Trait Implementations

##### `impl Clone for Trait`

- <span id="trait-clone"></span>`fn clone(&self) -> Trait` — [`Trait`](#trait)

##### `impl Spanned for Trait`

- <span id="trait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Trait`

- <span id="trait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitBound`

```rust
enum TraitBound {
    Slf,
    Other(Trait),
}
```

### `FieldBounds<'a>`

```rust
enum FieldBounds<'a> {
    None,
    All(&'a [TraitBound]),
    Trailing(&'a [TraitBound]),
    Explicit(Vec<syn::WherePredicate>),
}
```

#### Implementations

- <span id="fieldbounds-const-all-self"></span>`const ALL_SELF: FieldBounds<'a>`

- <span id="fieldbounds-const-trailing-self"></span>`const TRAILING_SELF: FieldBounds<'a>`

### `SelfBounds<'a>`

```rust
enum SelfBounds<'a> {
    None,
    All(&'a [Trait]),
}
```

#### Implementations

- <span id="selfbounds-const-sized"></span>`const SIZED: Self`

## Traits

### `DataExt`

```rust
trait DataExt { ... }
```

#### Required Methods

- `fn fields(&self) -> Vec<(&Visibility, TokenStream, &Type)>`

  Extracts the names and types of all fields. For enums, extracts the

- `fn variants(&self) -> Vec<(Option<&Variant>, Vec<(&Visibility, TokenStream, &Type)>)>`

- `fn tag(&self) -> Option<Ident>`

#### Implementors

- `syn::DataEnum`
- `syn::DataStruct`
- `syn::DataUnion`
- `syn::Data`

### `BoolExt`

```rust
trait BoolExt { ... }
```

#### Required Methods

- `fn then_some<T>(self, t: T) -> Option<T>`

#### Implementors

- `bool`

## Functions

### `map_fields`

```rust
fn map_fields<'a>(fields: impl 'a + IntoIterator<Item = &'a syn::Field>) -> Vec<(&'a syn::Visibility, proc_macro2::TokenStream, &'a syn::Type)>
```

### `to_ident_str`

```rust
fn to_ident_str(t: &impl ToString) -> String
```

### `normalize_bounds`

```rust
fn normalize_bounds<'a>(slf: &'a Trait, bounds: &'a [TraitBound]) -> impl 'a + Iterator<Item = Trait>
```

Normalizes a slice of bounds by replacing [`TraitBound::Slf`](../index.md) with `slf`.

### `const_block`

```rust
fn const_block(items: impl IntoIterator<Item = Option<proc_macro2::TokenStream>>) -> proc_macro2::TokenStream
```

### `generate_tag_enum`

```rust
fn generate_tag_enum(ctx: &Ctx, repr: &Repr<PrimitiveRepr, core::convert::Infallible>, data: &syn::DataEnum) -> proc_macro2::TokenStream
```

### `enum_size_from_repr`

```rust
fn enum_size_from_repr(repr: &Repr<PrimitiveRepr, core::convert::Infallible>) -> Result<usize, syn::Error>
```

