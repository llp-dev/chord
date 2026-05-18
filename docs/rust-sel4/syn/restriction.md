**syn > restriction**

# Module: restriction

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`VisRestricted`](#visrestricted) - A visibility level restricted to some path: `pub(self)` or

**Enums**

- [`FieldMutability`](#fieldmutability) - Unused, but reserved for RFC 3323 restrictions.
- [`Visibility`](#visibility) - The visibility level of an item: inherited or `pub` or

---

## syn::restriction::FieldMutability

*Enum*

Unused, but reserved for RFC 3323 restrictions.

**Variants:**
- `None`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::restriction::VisRestricted

*Struct*

A visibility level restricted to some path: `pub(self)` or
`pub(super)` or `pub(crate)` or `pub(in some::module)`.

**Fields:**
- `pub_token: $crate::token::Pub`
- `paren_token: token::Paren`
- `in_token: Option<$crate::token::In>`
- `path: alloc::boxed::Box<crate::path::Path>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::restriction::Visibility

*Enum*

The visibility level of an item: inherited or `pub` or
`pub(restricted)`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Public($crate::token::Pub)` - A public visibility level: `pub`.
- `Restricted(VisRestricted)` - A visibility level restricted to some path: `pub(self)` or
- `Inherited` - An inherited visibility, which usually means private.

**Methods:**

- `fn is_inherited(self: &Self) -> bool`
- `fn parse_pub(input: ParseStream) -> Result<Self>`
- `fn is_some(self: &Self) -> bool`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## Module: parsing



## Module: printing



