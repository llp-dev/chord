*[rkyv_derive](../index.md) / [attributes](index.md)*

---

# Module `attributes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Attributes`](#attributes) | struct |  |
| [`FieldAttributes`](#fieldattributes) | struct |  |
| [`VariantAttributes`](#variantattributes) | struct |  |
| [`Niche`](#niche) | enum |  |
| [`try_set_attribute`](#try-set-attribute) | fn |  |

## Structs

### `Attributes`

```rust
struct Attributes {
    pub as_type: Option<syn::Type>,
    pub archived: Option<syn::Ident>,
    pub resolver: Option<syn::Ident>,
    pub remote: Option<syn::Path>,
    pub metas: Vec<syn::Meta>,
    pub compares: Option<syn::punctuated::Punctuated<syn::Path, token::Comma>>,
    pub archive_bounds: Option<syn::punctuated::Punctuated<syn::WherePredicate, token::Comma>>,
    pub serialize_bounds: Option<syn::punctuated::Punctuated<syn::WherePredicate, token::Comma>>,
    pub deserialize_bounds: Option<syn::punctuated::Punctuated<syn::WherePredicate, token::Comma>>,
    pub bytecheck: Option<proc_macro2::TokenStream>,
    pub crate_path: Option<syn::Path>,
}
```

#### Implementations

- <span id="attributes-parse-meta"></span>`fn parse_meta(&mut self, meta: ParseNestedMeta<'_>) -> Result<(), Error>`

- <span id="attributes-parse"></span>`fn parse(input: &DeriveInput) -> Result<Self, Error>`

- <span id="attributes-crate-path"></span>`fn crate_path(&self) -> Path`

#### Trait Implementations

##### `impl Default for Attributes`

- <span id="attributes-default"></span>`fn default() -> Attributes` — [`Attributes`](#attributes)

### `FieldAttributes`

```rust
struct FieldAttributes {
    pub attrs: syn::punctuated::Punctuated<syn::Meta, token::Comma>,
    pub omit_bounds: Option<syn::Path>,
    pub with: Option<syn::Type>,
    pub getter: Option<syn::Path>,
    pub niches: Vec<Niche>,
}
```

#### Implementations

- <span id="fieldattributes-parse-meta"></span>`fn parse_meta(&mut self, meta: ParseNestedMeta<'_>) -> Result<(), Error>`

- <span id="fieldattributes-parse"></span>`fn parse(attributes: &Attributes, input: &Field) -> Result<Self, Error>` — [`Attributes`](#attributes)

- <span id="fieldattributes-archive-bound"></span>`fn archive_bound(&self, rkyv_path: &Path, field: &Field) -> Option<WherePredicate>`

- <span id="fieldattributes-serialize-bound"></span>`fn serialize_bound(&self, rkyv_path: &Path, field: &Field) -> Option<WherePredicate>`

- <span id="fieldattributes-deserialize-bound"></span>`fn deserialize_bound(&self, rkyv_path: &Path, field: &Field) -> Option<WherePredicate>`

- <span id="fieldattributes-archive-item"></span>`fn archive_item(&self, rkyv_path: &Path, field: &Field, name: &str, with_name: &str) -> TokenStream`

- <span id="fieldattributes-archived"></span>`fn archived(&self, rkyv_path: &Path, field: &Field) -> TokenStream`

- <span id="fieldattributes-resolver"></span>`fn resolver(&self, rkyv_path: &Path, field: &Field) -> TokenStream`

- <span id="fieldattributes-resolve"></span>`fn resolve(&self, rkyv_path: &Path, field: &Field) -> TokenStream`

- <span id="fieldattributes-serialize"></span>`fn serialize(&self, rkyv_path: &Path, field: &Field) -> TokenStream`

- <span id="fieldattributes-deserialize"></span>`fn deserialize(&self, rkyv_path: &Path, field: &Field) -> TokenStream`

- <span id="fieldattributes-access-field"></span>`fn access_field(&self, this: &Ident, member: &impl ToTokens) -> TokenStream`

- <span id="fieldattributes-metas"></span>`fn metas(&self) -> TokenStream`

#### Trait Implementations

##### `impl Default for FieldAttributes`

- <span id="fieldattributes-default"></span>`fn default() -> FieldAttributes` — [`FieldAttributes`](#fieldattributes)

### `VariantAttributes`

```rust
struct VariantAttributes {
    pub other: Option<syn::Path>,
}
```

#### Implementations

- <span id="variantattributes-parse-meta"></span>`fn parse_meta(&mut self, meta: ParseNestedMeta<'_>) -> Result<(), Error>`

- <span id="variantattributes-parse"></span>`fn parse(attributes: &Attributes, input: &Variant) -> Result<Self, Error>` — [`Attributes`](#attributes)

#### Trait Implementations

##### `impl Default for VariantAttributes`

- <span id="variantattributes-default"></span>`fn default() -> VariantAttributes` — [`VariantAttributes`](#variantattributes)

## Enums

### `Niche`

```rust
enum Niche {
    Type(Box<syn::Type>),
    Default,
}
```

#### Implementations

- <span id="niche-to-tokens"></span>`fn to_tokens(&self, rkyv_path: &Path) -> TokenStream`

#### Trait Implementations

##### `impl PartialEq for Niche`

- <span id="niche-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Functions

### `try_set_attribute`

```rust
fn try_set_attribute<T: ToTokens>(attribute: &mut Option<T>, value: T, name: &'static str) -> Result<(), syn::Error>
```

