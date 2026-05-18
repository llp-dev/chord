*[bytecheck_derive](../index.md) / [attributes](index.md)*

---

# Module `attributes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Attributes`](#attributes) | struct |  |
| [`FieldAttributes`](#fieldattributes) | struct |  |
| [`try_set_attribute`](#try-set-attribute) | fn |  |

## Structs

### `Attributes`

```rust
struct Attributes {
    pub bounds: Option<syn::punctuated::Punctuated<syn::WherePredicate, token::Comma>>,
    crate_path: Option<syn::Path>,
    pub verify: Option<syn::Path>,
}
```

#### Implementations

- <span id="attributes-parse-check-bytes-attributes"></span>`fn parse_check_bytes_attributes(&mut self, meta: ParseNestedMeta<'_>) -> Result<(), Error>`

- <span id="attributes-parse"></span>`fn parse(input: &DeriveInput) -> Result<Self, Error>`

- <span id="attributes-crate-path"></span>`fn crate_path(&self) -> Path`

#### Trait Implementations

##### `impl Default for Attributes`

- <span id="attributes-default"></span>`fn default() -> Attributes` — [`Attributes`](#attributes)

### `FieldAttributes`

```rust
struct FieldAttributes {
    pub omit_bounds: Option<syn::Path>,
}
```

#### Implementations

- <span id="fieldattributes-parse-meta"></span>`fn parse_meta(&mut self, meta: ParseNestedMeta<'_>) -> Result<(), Error>`

- <span id="fieldattributes-parse"></span>`fn parse(input: &Field) -> Result<Self, Error>`

#### Trait Implementations

##### `impl Default for FieldAttributes`

- <span id="fieldattributes-default"></span>`fn default() -> FieldAttributes` — [`FieldAttributes`](#fieldattributes)

## Functions

### `try_set_attribute`

```rust
fn try_set_attribute<T: ToTokens>(attribute: &mut Option<T>, value: T, name: &'static str) -> Result<(), syn::Error>
```

