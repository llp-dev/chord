**syn > data**

# Module: data

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`Field`](#field) - A field of a struct or enum variant.
- [`FieldsNamed`](#fieldsnamed) - Named fields of a struct or struct variant such as `Point { x: f64,
- [`FieldsUnnamed`](#fieldsunnamed) - Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.
- [`Members`](#members)
- [`Variant`](#variant) - An enum variant.

**Enums**

- [`Fields`](#fields) - Data stored within an enum variant or struct.

---

## syn::data::Field

*Struct*

A field of a struct or enum variant.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `mutability: crate::restriction::FieldMutability`
- `ident: Option<crate::ident::Ident>` - Name of the field, if any.
- `colon_token: Option<$crate::token::Colon>`
- `ty: crate::ty::Type`

**Methods:**

- `fn parse_named(input: ParseStream) -> Result<Self>` - Parses a named (braced struct) field.
- `fn parse_unnamed(input: ParseStream) -> Result<Self>` - Parses an unnamed (tuple struct) field.

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::data::Fields

*Enum*

Data stored within an enum variant or struct.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Named(FieldsNamed)` - Named fields of a struct or struct variant such as `Point { x: f64,
- `Unnamed(FieldsUnnamed)` - Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.
- `Unit` - Unit struct or unit variant such as `None`.

**Methods:**

- `fn iter(self: &Self) -> punctuated::Iter<Field>` - Get an iterator over the borrowed [`Field`] items in this object. This
- `fn iter_mut(self: & mut Self) -> punctuated::IterMut<Field>` - Get an iterator over the mutably borrowed [`Field`] items in this
- `fn len(self: &Self) -> usize` - Returns the number of fields.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if there are zero fields.
- `fn members(self: &Self) -> Members` - Get an iterator over the fields of a struct or variant as [`Member`]s.

**Trait Implementations:**

- **From**
  - `fn from(e: FieldsUnnamed) -> Fields`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: FieldsNamed) -> Fields`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



## syn::data::FieldsNamed

*Struct*

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

**Fields:**
- `brace_token: token::Brace`
- `named: crate::punctuated::Punctuated<Field, $crate::token::Comma>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::data::FieldsUnnamed

*Struct*

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

**Fields:**
- `paren_token: token::Paren`
- `unnamed: crate::punctuated::Punctuated<Field, $crate::token::Comma>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::data::Members

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `fields: punctuated::Iter<'a, Field>`
- `index: u32`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::data::Variant

*Struct*

An enum variant.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `ident: crate::ident::Ident` - Name of the variant.
- `fields: Fields` - Content stored in the variant.
- `discriminant: Option<($crate::token::Eq, crate::expr::Expr)>` - Explicit discriminant: `Variant = 1`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## Module: parsing



## Module: printing



