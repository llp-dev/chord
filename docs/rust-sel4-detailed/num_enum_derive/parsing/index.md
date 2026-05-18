*[num_enum_derive](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Structs](#structs)
  - [`EnumInfo`](#enuminfo)
  - [`VariantInfo`](#variantinfo)
  - [`ErrorType`](#errortype)
- [Enums](#enums)
  - [`DiscriminantValue`](#discriminantvalue)
- [Functions](#functions)
  - [`literal`](#literal)
  - [`parse_discriminant`](#parse-discriminant)
  - [`parse_alternative_values`](#parse-alternative-values)
  - [`get_crate_path`](#get-crate-path)
  - [`crate_path_as_string`](#crate-path-as-string)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EnumInfo`](#enuminfo) | struct |  |
| [`VariantInfo`](#variantinfo) | struct |  |
| [`ErrorType`](#errortype) | struct |  |
| [`DiscriminantValue`](#discriminantvalue) | enum |  |
| [`literal`](#literal) | fn |  |
| [`parse_discriminant`](#parse-discriminant) | fn |  |
| [`parse_alternative_values`](#parse-alternative-values) | fn |  |
| [`get_crate_path`](#get-crate-path) | fn |  |
| [`crate_path_as_string`](#crate-path-as-string) | fn |  |

## Structs

### `EnumInfo`

```rust
struct EnumInfo {
    name: syn::Ident,
    repr: syn::Ident,
    crate_path: Option<syn::Path>,
    variants: Vec<VariantInfo>,
    error_type_info: ErrorType,
}
```

#### Implementations

- <span id="enuminfo-is-naturally-exhaustive"></span>`fn is_naturally_exhaustive(&self) -> Result<bool>`

  Returns whether the number of variants (ignoring defaults, catch-alls, etc) is the same as

  the capacity of the repr.

- <span id="enuminfo-default"></span>`fn default(&self) -> Option<&Ident>`

- <span id="enuminfo-catch-all"></span>`fn catch_all(&self) -> Option<&Ident>`

- <span id="enuminfo-variant-idents"></span>`fn variant_idents(&self) -> Vec<Ident>`

- <span id="enuminfo-expression-idents"></span>`fn expression_idents(&self) -> Vec<Vec<Ident>>`

- <span id="enuminfo-variant-expressions"></span>`fn variant_expressions(&self) -> Vec<Vec<Expr>>`

- <span id="enuminfo-parse-attrs"></span>`fn parse_attrs<Attrs: Iterator<Item = Attribute>>(attrs: Attrs) -> Result<(Ident, crate::enum_attributes::Attributes)>` — [`Attributes`](../enum_attributes/index.md#attributes)

#### Trait Implementations

##### `impl Parse for EnumInfo`

- <span id="enuminfo-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `VariantInfo`

```rust
struct VariantInfo {
    ident: syn::Ident,
    is_default: bool,
    is_catch_all: bool,
    canonical_value: syn::Expr,
    alternative_values: Vec<syn::Expr>,
}
```

#### Implementations

- <span id="variantinfo-all-values"></span>`fn all_values(&self) -> impl Iterator<Item = &Expr>`

### `ErrorType`

```rust
struct ErrorType {
    name: syn::Path,
    constructor: syn::Path,
}
```

## Enums

### `DiscriminantValue`

```rust
enum DiscriminantValue {
    Literal(i128),
    Expr(syn::Expr),
}
```

## Functions

### `literal`

```rust
fn literal(i: i128) -> syn::Expr
```

### `parse_discriminant`

```rust
fn parse_discriminant(val_exp: &syn::Expr) -> syn::Result<DiscriminantValue>
```

### `parse_alternative_values`

```rust
fn parse_alternative_values(val_expr: &syn::Expr) -> syn::Result<Vec<DiscriminantValue>>
```

### `get_crate_path`

```rust
fn get_crate_path(path: Option<syn::Path>) -> syn::Path
```

### `crate_path_as_string`

```rust
fn crate_path_as_string(path: &syn::Path) -> syn::Result<String>
```

