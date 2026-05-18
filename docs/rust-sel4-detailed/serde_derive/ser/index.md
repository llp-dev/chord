*[serde_derive](../index.md) / [ser](index.md)*

---

# Module `ser`

## Contents

- [Structs](#structs)
  - [`Parameters`](#parameters)
- [Enums](#enums)
  - [`TupleVariant`](#tuplevariant)
  - [`StructVariant`](#structvariant)
  - [`StructTrait`](#structtrait)
  - [`TupleTrait`](#tupletrait)
- [Functions](#functions)
  - [`expand_derive_serialize`](#expand-derive-serialize)
  - [`precondition`](#precondition)
  - [`build_generics`](#build-generics)
  - [`needs_serialize_bound`](#needs-serialize-bound)
  - [`serialize_body`](#serialize-body)
  - [`serialize_transparent`](#serialize-transparent)
  - [`serialize_into`](#serialize-into)
  - [`serialize_unit_struct`](#serialize-unit-struct)
  - [`serialize_newtype_struct`](#serialize-newtype-struct)
  - [`serialize_tuple_struct`](#serialize-tuple-struct)
  - [`serialize_struct`](#serialize-struct)
  - [`serialize_struct_tag_field`](#serialize-struct-tag-field)
  - [`serialize_struct_as_struct`](#serialize-struct-as-struct)
  - [`serialize_struct_as_map`](#serialize-struct-as-map)
  - [`serialize_enum`](#serialize-enum)
  - [`serialize_variant`](#serialize-variant)
  - [`serialize_externally_tagged_variant`](#serialize-externally-tagged-variant)
  - [`serialize_internally_tagged_variant`](#serialize-internally-tagged-variant)
  - [`serialize_adjacently_tagged_variant`](#serialize-adjacently-tagged-variant)
  - [`serialize_untagged_variant`](#serialize-untagged-variant)
  - [`serialize_tuple_variant`](#serialize-tuple-variant)
  - [`serialize_struct_variant`](#serialize-struct-variant)
  - [`serialize_struct_variant_with_flatten`](#serialize-struct-variant-with-flatten)
  - [`serialize_tuple_struct_visitor`](#serialize-tuple-struct-visitor)
  - [`serialize_struct_visitor`](#serialize-struct-visitor)
  - [`wrap_serialize_field_with`](#wrap-serialize-field-with)
  - [`wrap_serialize_variant_with`](#wrap-serialize-variant-with)
  - [`wrap_serialize_with`](#wrap-serialize-with)
  - [`mut_if`](#mut-if)
  - [`get_member`](#get-member)
  - [`effective_style`](#effective-style)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parameters`](#parameters) | struct |  |
| [`TupleVariant`](#tuplevariant) | enum |  |
| [`StructVariant`](#structvariant) | enum |  |
| [`StructTrait`](#structtrait) | enum |  |
| [`TupleTrait`](#tupletrait) | enum |  |
| [`expand_derive_serialize`](#expand-derive-serialize) | fn |  |
| [`precondition`](#precondition) | fn |  |
| [`build_generics`](#build-generics) | fn |  |
| [`needs_serialize_bound`](#needs-serialize-bound) | fn |  |
| [`serialize_body`](#serialize-body) | fn |  |
| [`serialize_transparent`](#serialize-transparent) | fn |  |
| [`serialize_into`](#serialize-into) | fn |  |
| [`serialize_unit_struct`](#serialize-unit-struct) | fn |  |
| [`serialize_newtype_struct`](#serialize-newtype-struct) | fn |  |
| [`serialize_tuple_struct`](#serialize-tuple-struct) | fn |  |
| [`serialize_struct`](#serialize-struct) | fn |  |
| [`serialize_struct_tag_field`](#serialize-struct-tag-field) | fn |  |
| [`serialize_struct_as_struct`](#serialize-struct-as-struct) | fn |  |
| [`serialize_struct_as_map`](#serialize-struct-as-map) | fn |  |
| [`serialize_enum`](#serialize-enum) | fn |  |
| [`serialize_variant`](#serialize-variant) | fn |  |
| [`serialize_externally_tagged_variant`](#serialize-externally-tagged-variant) | fn |  |
| [`serialize_internally_tagged_variant`](#serialize-internally-tagged-variant) | fn |  |
| [`serialize_adjacently_tagged_variant`](#serialize-adjacently-tagged-variant) | fn |  |
| [`serialize_untagged_variant`](#serialize-untagged-variant) | fn |  |
| [`serialize_tuple_variant`](#serialize-tuple-variant) | fn |  |
| [`serialize_struct_variant`](#serialize-struct-variant) | fn |  |
| [`serialize_struct_variant_with_flatten`](#serialize-struct-variant-with-flatten) | fn |  |
| [`serialize_tuple_struct_visitor`](#serialize-tuple-struct-visitor) | fn |  |
| [`serialize_struct_visitor`](#serialize-struct-visitor) | fn |  |
| [`wrap_serialize_field_with`](#wrap-serialize-field-with) | fn |  |
| [`wrap_serialize_variant_with`](#wrap-serialize-variant-with) | fn |  |
| [`wrap_serialize_with`](#wrap-serialize-with) | fn |  |
| [`mut_if`](#mut-if) | fn |  |
| [`get_member`](#get-member) | fn |  |
| [`effective_style`](#effective-style) | fn |  |

## Structs

### `Parameters`

```rust
struct Parameters {
    self_var: syn::Ident,
    this_type: syn::Path,
    this_value: syn::Path,
    generics: syn::Generics,
    is_remote: bool,
    is_packed: bool,
}
```

#### Fields

- **`self_var`**: `syn::Ident`

  Variable holding the value being serialized. Either `self` for local
  types or `__self` for remote types.

- **`this_type`**: `syn::Path`

  Path to the type the impl is for. Either a single `Ident` for local
  types (does not include generic parameters) or `some::remote::Path` for
  remote types.

- **`this_value`**: `syn::Path`

  Same as `this_type` but using `::<T>` for generic parameters for use in
  expression position.

- **`generics`**: `syn::Generics`

  Generics including any explicit and inferred bounds for the impl.

- **`is_remote`**: `bool`

  Type has a `serde(remote = "...")` attribute.

- **`is_packed`**: `bool`

  Type has a repr(packed) attribute.

#### Implementations

- <span id="parameters-new"></span>`fn new(cont: &Container<'_>) -> Self` — [`Container`](../internals/ast/index.md#container)

- <span id="parameters-type-name"></span>`fn type_name(&self) -> String`

  Type name to use in error messages and `&'static str` arguments to

  various Serializer methods.

## Enums

### `TupleVariant<'a>`

```rust
enum TupleVariant<'a> {
    ExternallyTagged {
        type_name: &'a crate::internals::name::Name,
        variant_index: u32,
        variant_name: &'a crate::internals::name::Name,
    },
    Untagged,
}
```

### `StructVariant<'a>`

```rust
enum StructVariant<'a> {
    ExternallyTagged {
        variant_index: u32,
        variant_name: &'a crate::internals::name::Name,
    },
    InternallyTagged {
        tag: &'a str,
        variant_name: &'a crate::internals::name::Name,
    },
    Untagged,
}
```

### `StructTrait`

```rust
enum StructTrait {
    SerializeMap,
    SerializeStruct,
    SerializeStructVariant,
}
```

#### Implementations

- <span id="structtrait-serialize-field"></span>`fn serialize_field(&self, span: Span) -> TokenStream`

- <span id="structtrait-skip-field"></span>`fn skip_field(&self, span: Span) -> Option<TokenStream>`

### `TupleTrait`

```rust
enum TupleTrait {
    SerializeTuple,
    SerializeTupleStruct,
    SerializeTupleVariant,
}
```

#### Implementations

- <span id="tupletrait-serialize-element"></span>`fn serialize_element(&self, span: Span) -> TokenStream`

## Functions

### `expand_derive_serialize`

```rust
fn expand_derive_serialize(input: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```

### `precondition`

```rust
fn precondition(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `build_generics`

```rust
fn build_generics(cont: &crate::internals::ast::Container<'_>) -> syn::Generics
```

### `needs_serialize_bound`

```rust
fn needs_serialize_bound(field: &attr::Field, variant: Option<&attr::Variant>) -> bool
```

### `serialize_body`

```rust
fn serialize_body(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

### `serialize_transparent`

```rust
fn serialize_transparent(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

### `serialize_into`

```rust
fn serialize_into(params: &Parameters, type_into: &syn::Type) -> crate::fragment::Fragment
```

### `serialize_unit_struct`

```rust
fn serialize_unit_struct(cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_newtype_struct`

```rust
fn serialize_newtype_struct(params: &Parameters, field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_tuple_struct`

```rust
fn serialize_tuple_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_struct`

```rust
fn serialize_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_struct_tag_field`

```rust
fn serialize_struct_tag_field(cattrs: &attr::Container, struct_trait: &StructTrait) -> proc_macro2::TokenStream
```

### `serialize_struct_as_struct`

```rust
fn serialize_struct_as_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_struct_as_map`

```rust
fn serialize_struct_as_map(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_enum`

```rust
fn serialize_enum(params: &Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_variant`

```rust
fn serialize_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, variant_index: u32, cattrs: &attr::Container) -> proc_macro2::TokenStream
```

### `serialize_externally_tagged_variant`

```rust
fn serialize_externally_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, variant_index: u32, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_internally_tagged_variant`

```rust
fn serialize_internally_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container, tag: &str) -> crate::fragment::Fragment
```

### `serialize_adjacently_tagged_variant`

```rust
fn serialize_adjacently_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container, variant_index: u32, tag: &str, content: &str) -> crate::fragment::Fragment
```

### `serialize_untagged_variant`

```rust
fn serialize_untagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_tuple_variant`

```rust
fn serialize_tuple_variant(context: TupleVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>]) -> crate::fragment::Fragment
```

### `serialize_struct_variant`

```rust
fn serialize_struct_variant(context: StructVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], name: &crate::internals::name::Name) -> crate::fragment::Fragment
```

### `serialize_struct_variant_with_flatten`

```rust
fn serialize_struct_variant_with_flatten(context: StructVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], name: &crate::internals::name::Name) -> crate::fragment::Fragment
```

### `serialize_tuple_struct_visitor`

```rust
fn serialize_tuple_struct_visitor(fields: &[crate::internals::ast::Field<'_>], params: &Parameters, is_enum: bool, tuple_trait: &TupleTrait) -> Vec<proc_macro2::TokenStream>
```

### `serialize_struct_visitor`

```rust
fn serialize_struct_visitor(fields: &[crate::internals::ast::Field<'_>], params: &Parameters, is_enum: bool, struct_trait: &StructTrait) -> Vec<proc_macro2::TokenStream>
```

### `wrap_serialize_field_with`

```rust
fn wrap_serialize_field_with(params: &Parameters, field_ty: &syn::Type, serialize_with: &syn::ExprPath, field_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream
```

### `wrap_serialize_variant_with`

```rust
fn wrap_serialize_variant_with(params: &Parameters, serialize_with: &syn::ExprPath, variant: &crate::internals::ast::Variant<'_>) -> proc_macro2::TokenStream
```

### `wrap_serialize_with`

```rust
fn wrap_serialize_with(params: &Parameters, serialize_with: &syn::ExprPath, field_tys: &[&syn::Type], field_exprs: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream
```

### `mut_if`

```rust
fn mut_if(is_mut: bool) -> Option<proc_macro2::TokenStream>
```

### `get_member`

```rust
fn get_member(params: &Parameters, field: &crate::internals::ast::Field<'_>, member: &syn::Member) -> proc_macro2::TokenStream
```

### `effective_style`

```rust
fn effective_style(variant: &crate::internals::ast::Variant<'_>) -> crate::internals::ast::Style
```

