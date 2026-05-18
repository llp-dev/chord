*[ptr_meta_derive](../index.md) / [attributes](index.md)*

---

# Module `attributes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Attributes`](#attributes) | struct |  |
| [`try_set_attribute`](#try-set-attribute) | fn |  |

## Structs

### `Attributes`

```rust
struct Attributes {
    crate_path: Option<syn::Path>,
}
```

#### Implementations

- <span id="attributes-parse-meta"></span>`fn parse_meta(&mut self, meta: ParseNestedMeta<'_>) -> Result<(), Error>`

- <span id="attributes-parse"></span>`fn parse(attrs: &[Attribute]) -> Result<Self, Error>`

- <span id="attributes-crate-path"></span>`fn crate_path(&self) -> Path`

#### Trait Implementations

##### `impl Default for Attributes`

- <span id="attributes-default"></span>`fn default() -> Attributes` — [`Attributes`](#attributes)

## Functions

### `try_set_attribute`

```rust
fn try_set_attribute<T: ToTokens>(attribute: &mut Option<T>, value: T, name: &'static str) -> Result<(), syn::Error>
```

