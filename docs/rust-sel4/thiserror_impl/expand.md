**thiserror_impl > expand**

# Module: expand

## Contents

**Functions**

- [`call_site_ident`](#call_site_ident)
- [`derive`](#derive)
- [`fields_pat`](#fields_pat)
- [`from_initializer`](#from_initializer)
- [`impl_enum`](#impl_enum)
- [`impl_struct`](#impl_struct)
- [`try_expand`](#try_expand)
- [`type_is_option`](#type_is_option)
- [`type_parameter_of_option`](#type_parameter_of_option)
- [`unoptional_type`](#unoptional_type)
- [`use_as_display`](#use_as_display)

---

## thiserror_impl::expand::call_site_ident

*Function*

```rust
fn call_site_ident(ident: &proc_macro2::Ident) -> proc_macro2::Ident
```



## thiserror_impl::expand::derive

*Function*

```rust
fn derive(input: &syn::DeriveInput) -> proc_macro2::TokenStream
```



## thiserror_impl::expand::fields_pat

*Function*

```rust
fn fields_pat(fields: &[crate::ast::Field]) -> proc_macro2::TokenStream
```



## thiserror_impl::expand::from_initializer

*Function*

```rust
fn from_initializer(from_field: &crate::ast::Field, backtrace_field: Option<&crate::ast::Field>, source_var: &proc_macro2::Ident) -> proc_macro2::TokenStream
```



## thiserror_impl::expand::impl_enum

*Function*

```rust
fn impl_enum(input: crate::ast::Enum) -> proc_macro2::TokenStream
```



## thiserror_impl::expand::impl_struct

*Function*

```rust
fn impl_struct(input: crate::ast::Struct) -> proc_macro2::TokenStream
```



## thiserror_impl::expand::try_expand

*Function*

```rust
fn try_expand(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```



## thiserror_impl::expand::type_is_option

*Function*

```rust
fn type_is_option(ty: &syn::Type) -> bool
```



## thiserror_impl::expand::type_parameter_of_option

*Function*

```rust
fn type_parameter_of_option(ty: &syn::Type) -> Option<&syn::Type>
```



## thiserror_impl::expand::unoptional_type

*Function*

```rust
fn unoptional_type(ty: &syn::Type) -> proc_macro2::TokenStream
```



## thiserror_impl::expand::use_as_display

*Function*

```rust
fn use_as_display(needs_as_display: bool) -> Option<proc_macro2::TokenStream>
```



