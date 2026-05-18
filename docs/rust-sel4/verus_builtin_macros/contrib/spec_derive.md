**verus_builtin_macros > contrib > spec_derive**

# Module: contrib::spec_derive

## Contents

**Functions**

- [`contains_self_reference`](#contains_self_reference)
- [`contains_self_reference_in_data`](#contains_self_reference_in_data)
- [`fields_contain_self_reference`](#fields_contain_self_reference)
- [`generate_enum_impls`](#generate_enum_impls)
- [`generate_field_access`](#generate_field_access)
- [`generate_match_arm`](#generate_match_arm)
- [`generate_spec_field_type`](#generate_spec_field_type)
- [`generate_spec_variant_fields`](#generate_spec_variant_fields)
- [`generate_struct_impls`](#generate_struct_impls)
- [`generate_trait_impls`](#generate_trait_impls)
- [`is_field_excluded`](#is_field_excluded)
- [`is_str_reference`](#is_str_reference)
- [`make_spec_type`](#make_spec_type) - Generates a spec type and implements DeepView/View traits for a struct or enum.
- [`make_spec_type_impl`](#make_spec_type_impl)
- [`parse_excluded_fields`](#parse_excluded_fields)
- [`self_view`](#self_view) - Implements DeepView and View traits where the view is the type itself.
- [`self_view_impl`](#self_view_impl)
- [`strip_references_from_type`](#strip_references_from_type)
- [`validate_excluded_fields`](#validate_excluded_fields)
- [`validate_input`](#validate_input)

---

## verus_builtin_macros::contrib::spec_derive::contains_self_reference

*Function*

```rust
fn contains_self_reference(ty: &syn::Type, self_name: &syn::Ident) -> bool
```



## verus_builtin_macros::contrib::spec_derive::contains_self_reference_in_data

*Function*

```rust
fn contains_self_reference_in_data(data: &syn::Data, self_name: &syn::Ident) -> bool
```



## verus_builtin_macros::contrib::spec_derive::fields_contain_self_reference

*Function*

```rust
fn fields_contain_self_reference(fields: &syn::Fields, self_name: &syn::Ident) -> bool
```



## verus_builtin_macros::contrib::spec_derive::generate_enum_impls

*Function*

```rust
fn generate_enum_impls(name: &syn::Ident, spec_name: &syn::Ident, data_enum: &syn::DataEnum, excluded_fields: &std::collections::HashSet<String>, impl_generics: syn::ImplGenerics, ty_generics: syn::TypeGenerics, where_clause: Option<&syn::WhereClause>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```



## verus_builtin_macros::contrib::spec_derive::generate_field_access

*Function*

```rust
fn generate_field_access(field_name: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream
```



## verus_builtin_macros::contrib::spec_derive::generate_match_arm

*Function*

```rust
fn generate_match_arm(spec_name: &syn::Ident, vname: &syn::Ident, fields: &syn::Fields) -> proc_macro2::TokenStream
```



## verus_builtin_macros::contrib::spec_derive::generate_spec_field_type

*Function*

```rust
fn generate_spec_field_type(ty: &syn::Type) -> proc_macro2::TokenStream
```



## verus_builtin_macros::contrib::spec_derive::generate_spec_variant_fields

*Function*

```rust
fn generate_spec_variant_fields(vname: &syn::Ident, fields: &syn::Fields, excluded_fields: &std::collections::HashSet<String>) -> proc_macro2::TokenStream
```



## verus_builtin_macros::contrib::spec_derive::generate_struct_impls

*Function*

```rust
fn generate_struct_impls(name: &syn::Ident, spec_name: &syn::Ident, data_struct: &syn::DataStruct, excluded_fields: &std::collections::HashSet<String>, impl_generics: syn::ImplGenerics, ty_generics: syn::TypeGenerics, where_clause: Option<&syn::WhereClause>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```



## verus_builtin_macros::contrib::spec_derive::generate_trait_impls

*Function*

```rust
fn generate_trait_impls(name: &syn::Ident, spec_name: &syn::Ident, impl_generics: syn::ImplGenerics, ty_generics: syn::TypeGenerics, where_clause: Option<&syn::WhereClause>, deep_view_body: proc_macro2::TokenStream) -> proc_macro2::TokenStream
```



## verus_builtin_macros::contrib::spec_derive::is_field_excluded

*Function*

```rust
fn is_field_excluded(field_name: &Option<syn::Ident>, excluded_fields: &std::collections::HashSet<String>) -> bool
```



## verus_builtin_macros::contrib::spec_derive::is_str_reference

*Function*

```rust
fn is_str_reference(type_ref: &syn::TypeReference) -> bool
```



## verus_builtin_macros::contrib::spec_derive::make_spec_type

*Function*

Generates a spec type and implements DeepView/View traits for a struct or enum.

# Supported Features
- Structs
- Enums with unit, tuple, and struct variants
- Generic types and lifetimes
- Nested collections (Vec, HashSet) and references
- Field exclusion with `exclude(field1, field2, ...)` for struct fields only

# Limitations
- Union types are not supported
- Cannot exclude enum variants (only struct fields can be excluded)
- Cannot exclude tuple variant fields (only named fields)
- All field types must implement DeepView trait
- No support for generic parameters

# Example
```
#[make_spec_type(exclude(private_field))]
pub struct MyStruct<'a> {
    pub id: u32,
    pub data: Vec<String>,
    pub private_field: String, // Excluded from spec
}
```

```rust
fn make_spec_type(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::contrib::spec_derive::make_spec_type_impl

*Function*

```rust
fn make_spec_type_impl(input: syn::DeriveInput, excluded_fields: std::collections::HashSet<String>) -> Result<proc_macro::TokenStream, syn::Error>
```



## verus_builtin_macros::contrib::spec_derive::parse_excluded_fields

*Function*

```rust
fn parse_excluded_fields(meta: syn::Meta) -> Result<std::collections::HashSet<String>, syn::Error>
```



## verus_builtin_macros::contrib::spec_derive::self_view

*Function*

Implements DeepView and View traits where the view is the type itself.

Use this for types that are already "spec-like" and don't need transformation,
such as primitive types or types that should be viewed as themselves.

# Supported Features
- Structs with any field types
- Enums with any variant types
- Generic types and lifetimes

# Limitations
- No customization options available
- All fields are included in the view (no exclusions)
- Type must be copyable for the implementation to work correctly

# Example
```
#[self_view]
pub enum Status {
    Active,
    Inactive,
    Pending { reason: String },
}
```

```rust
fn self_view(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::contrib::spec_derive::self_view_impl

*Function*

```rust
fn self_view_impl(item: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, syn::Error>
```



## verus_builtin_macros::contrib::spec_derive::strip_references_from_type

*Function*

```rust
fn strip_references_from_type(ty: &syn::Type) -> syn::Type
```



## verus_builtin_macros::contrib::spec_derive::validate_excluded_fields

*Function*

```rust
fn validate_excluded_fields(data: &syn::Data, excluded_fields: &std::collections::HashSet<String>) -> Result<(), syn::Error>
```



## verus_builtin_macros::contrib::spec_derive::validate_input

*Function*

```rust
fn validate_input(input: &syn::DeriveInput) -> Result<(), syn::Error>
```



