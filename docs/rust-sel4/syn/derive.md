**syn > derive**

# Module: derive

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`DataEnum`](#dataenum) - An enum input to a `proc_macro_derive` macro.
- [`DataStruct`](#datastruct) - A struct input to a `proc_macro_derive` macro.
- [`DataUnion`](#dataunion) - An untagged union input to a `proc_macro_derive` macro.
- [`DeriveInput`](#deriveinput) - Data structure sent to a `proc_macro_derive` macro.

**Enums**

- [`Data`](#data) - The storage of a struct, enum or union data structure.

---

## syn::derive::Data

*Enum*

The storage of a struct, enum or union data structure.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Struct(DataStruct)`
- `Enum(DataEnum)`
- `Union(DataUnion)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::derive::DataEnum

*Struct*

An enum input to a `proc_macro_derive` macro.

**Fields:**
- `enum_token: $crate::token::Enum`
- `brace_token: token::Brace`
- `variants: crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::derive::DataStruct

*Struct*

A struct input to a `proc_macro_derive` macro.

**Fields:**
- `struct_token: $crate::token::Struct`
- `fields: crate::data::Fields`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::derive::DataUnion

*Struct*

An untagged union input to a `proc_macro_derive` macro.

**Fields:**
- `union_token: $crate::token::Union`
- `fields: crate::data::FieldsNamed`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::derive::DeriveInput

*Struct*

Data structure sent to a `proc_macro_derive` macro.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `data: Data`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **From**
  - `fn from(input: ItemStruct) -> DeriveInput`
- **From**
  - `fn from(input: ItemEnum) -> DeriveInput`
- **From**
  - `fn from(input: ItemUnion) -> DeriveInput`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## Module: parsing



## Module: printing



