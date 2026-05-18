**thiserror_impl > attr**

# Module: attr

## Contents

**Structs**

- [`Attrs`](#attrs)
- [`Display`](#display)
- [`Fmt`](#fmt)
- [`From`](#from)
- [`Source`](#source)
- [`Transparent`](#transparent)

**Enums**

- [`Trait`](#trait)

**Functions**

- [`get`](#get)
- [`parse_error_attribute`](#parse_error_attribute)
- [`parse_token_expr`](#parse_token_expr)

---

## thiserror_impl::attr::Attrs

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `display: Option<Display<'a>>`
- `source: Option<Source<'a>>`
- `backtrace: Option<&'a syn::Attribute>`
- `from: Option<From<'a>>`
- `transparent: Option<Transparent<'a>>`
- `fmt: Option<Fmt<'a>>`



## thiserror_impl::attr::Display

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `original: &'a syn::Attribute`
- `fmt: syn::LitStr`
- `args: proc_macro2::TokenStream`
- `requires_fmt_machinery: bool`
- `has_bonus_display: bool`
- `infinite_recursive: bool`
- `implied_bounds: std::collections::BTreeSet<(usize, Trait)>`
- `bindings: Vec<(syn::Ident, proc_macro2::TokenStream)>`

**Methods:**

- `fn expand_shorthand(self: & mut Self, fields: &[Field], container: ContainerKind) -> Result<()>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Display<'a>`



## thiserror_impl::attr::Fmt

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `original: &'a syn::Attribute`
- `path: syn::ExprPath`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Fmt<'a>`



## thiserror_impl::attr::From

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `original: &'a syn::Attribute`
- `span: proc_macro2::Span`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> From<'a>`



## thiserror_impl::attr::Source

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `original: &'a syn::Attribute`
- `span: proc_macro2::Span`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Source<'a>`



## thiserror_impl::attr::Trait

*Enum*

**Variants:**
- `Debug`
- `Display`
- `Octal`
- `LowerHex`
- `UpperHex`
- `Pointer`
- `Binary`
- `LowerExp`
- `UpperExp`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Trait`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Trait) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Trait) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Trait) -> bool`



## thiserror_impl::attr::Transparent

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `original: &'a syn::Attribute`
- `span: proc_macro2::Span`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Transparent<'a>`



## thiserror_impl::attr::get

*Function*

```rust
fn get(input: &[syn::Attribute]) -> syn::Result<Attrs>
```



## thiserror_impl::attr::parse_error_attribute

*Function*

```rust
fn parse_error_attribute<'a>(attrs: & mut Attrs<'a>, attr: &'a syn::Attribute) -> syn::Result<()>
```



## thiserror_impl::attr::parse_token_expr

*Function*

```rust
fn parse_token_expr(input: syn::parse::ParseStream, begin_expr: bool) -> syn::Result<proc_macro2::TokenStream>
```



