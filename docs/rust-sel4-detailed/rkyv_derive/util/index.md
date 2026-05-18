*[rkyv_derive](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FieldsIter`](#fieldsiter) | enum |  |
| [`strip_raw`](#strip-raw) | fn |  |
| [`variant_fields`](#variant-fields) | fn |  |
| [`iter_fields`](#iter-fields) | fn |  |
| [`strip_generics_from_path`](#strip-generics-from-path) | fn |  |
| [`VariantFieldsFn`](#variantfieldsfn) | type |  |

## Enums

### `FieldsIter<'a>`

```rust
enum FieldsIter<'a> {
    Struct(syn::punctuated::Iter<'a, syn::Field>),
    Enum(core::iter::FlatMap<syn::punctuated::Iter<'a, syn::Variant>, syn::punctuated::Iter<'a, syn::Field>, fn(&syn::Variant) -> syn::punctuated::Iter<'_, syn::Field>>),
}
```

#### Trait Implementations

##### `impl IntoIterator for FieldsIter<'a>`

- <span id="fieldsiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="fieldsiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="fieldsiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FieldsIter<'a>`

- <span id="fieldsiter-iterator-type-item"></span>`type Item = &'a Field`

- <span id="fieldsiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Functions

### `strip_raw`

```rust
fn strip_raw(ident: &proc_macro2::Ident) -> String
```

### `variant_fields`

```rust
fn variant_fields(variant: &syn::Variant) -> syn::punctuated::Iter<'_, syn::Field>
```

### `iter_fields`

```rust
fn iter_fields(data: &syn::Data) -> FieldsIter<'_>
```

### `strip_generics_from_path`

```rust
fn strip_generics_from_path(path: syn::Path) -> syn::Path
```

## Type Aliases

### `VariantFieldsFn`

```rust
type VariantFieldsFn = fn(&syn::Variant) -> syn::punctuated::Iter<'_, syn::Field>;
```

