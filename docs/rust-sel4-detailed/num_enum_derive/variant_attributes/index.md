*[num_enum_derive](../index.md) / [variant_attributes](index.md)*

---

# Module `variant_attributes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`kw`](#kw) | mod |  |
| [`NumEnumVariantAttributes`](#numenumvariantattributes) | struct |  |
| [`VariantDefaultAttribute`](#variantdefaultattribute) | struct |  |
| [`VariantCatchAllAttribute`](#variantcatchallattribute) | struct |  |
| [`VariantAlternativesAttribute`](#variantalternativesattribute) | struct |  |
| [`NumEnumVariantAttributeItem`](#numenumvariantattributeitem) | enum |  |

## Modules

- [`kw`](kw/index.md)

## Structs

### `NumEnumVariantAttributes`

```rust
struct NumEnumVariantAttributes {
    items: syn::punctuated::Punctuated<NumEnumVariantAttributeItem, token::Comma>,
}
```

#### Trait Implementations

##### `impl Parse for NumEnumVariantAttributes`

- <span id="numenumvariantattributes-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `VariantDefaultAttribute`

```rust
struct VariantDefaultAttribute {
    keyword: kw::default,
}
```

#### Trait Implementations

##### `impl Parse for VariantDefaultAttribute`

- <span id="variantdefaultattribute-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `VariantCatchAllAttribute`

```rust
struct VariantCatchAllAttribute {
    keyword: kw::catch_all,
}
```

#### Trait Implementations

##### `impl Parse for VariantCatchAllAttribute`

- <span id="variantcatchallattribute-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `VariantAlternativesAttribute`

```rust
struct VariantAlternativesAttribute {
    _keyword: kw::alternatives,
    _eq_token: token::Eq,
    _bracket_token: syn::token::Bracket,
    expressions: syn::punctuated::Punctuated<syn::Expr, token::Comma>,
}
```

#### Trait Implementations

##### `impl Parse for VariantAlternativesAttribute`

- <span id="variantalternativesattribute-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

## Enums

### `NumEnumVariantAttributeItem`

```rust
enum NumEnumVariantAttributeItem {
    Default(VariantDefaultAttribute),
    CatchAll(VariantCatchAllAttribute),
    Alternatives(VariantAlternativesAttribute),
}
```

#### Trait Implementations

##### `impl Parse for NumEnumVariantAttributeItem`

- <span id="numenumvariantattributeitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

