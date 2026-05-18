**syn > path > printing**

# Module: path::printing

## Contents

**Enums**

- [`PathStyle`](#pathstyle)

**Functions**

- [`conditionally_print_turbofish`](#conditionally_print_turbofish)
- [`print_angle_bracketed_generic_arguments`](#print_angle_bracketed_generic_arguments)
- [`print_parenthesized_generic_arguments`](#print_parenthesized_generic_arguments)
- [`print_path`](#print_path)
- [`print_path_arguments`](#print_path_arguments)
- [`print_path_segment`](#print_path_segment)
- [`print_qpath`](#print_qpath)

---

## syn::path::printing::PathStyle

*Enum*

**Variants:**
- `Expr`
- `Mod`
- `AsWritten`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::path::printing::conditionally_print_turbofish

*Function*

```rust
fn conditionally_print_turbofish(tokens: & mut proc_macro2::TokenStream, colon2_token: &Option<$crate::token::PathSep>, style: PathStyle)
```



## syn::path::printing::print_angle_bracketed_generic_arguments

*Function*

```rust
fn print_angle_bracketed_generic_arguments(tokens: & mut proc_macro2::TokenStream, arguments: &crate::path::AngleBracketedGenericArguments, style: PathStyle)
```



## syn::path::printing::print_parenthesized_generic_arguments

*Function*

```rust
fn print_parenthesized_generic_arguments(tokens: & mut proc_macro2::TokenStream, arguments: &crate::path::ParenthesizedGenericArguments, style: PathStyle)
```



## syn::path::printing::print_path

*Function*

```rust
fn print_path(tokens: & mut proc_macro2::TokenStream, path: &crate::path::Path, style: PathStyle)
```



## syn::path::printing::print_path_arguments

*Function*

```rust
fn print_path_arguments(tokens: & mut proc_macro2::TokenStream, arguments: &crate::path::PathArguments, style: PathStyle)
```



## syn::path::printing::print_path_segment

*Function*

```rust
fn print_path_segment(tokens: & mut proc_macro2::TokenStream, segment: &crate::path::PathSegment, style: PathStyle)
```



## syn::path::printing::print_qpath

*Function*

```rust
fn print_qpath(tokens: & mut proc_macro2::TokenStream, qself: &Option<crate::path::QSelf>, path: &crate::path::Path, style: PathStyle)
```



