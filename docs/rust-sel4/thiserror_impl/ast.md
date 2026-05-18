**thiserror_impl > ast**

# Module: ast

## Contents

**Structs**

- [`Enum`](#enum)
- [`Field`](#field)
- [`Struct`](#struct)
- [`Variant`](#variant)

**Enums**

- [`ContainerKind`](#containerkind)
- [`Input`](#input)

---

## thiserror_impl::ast::ContainerKind

*Enum*

**Variants:**
- `Struct`
- `TupleStruct`
- `UnitStruct`
- `StructVariant`
- `TupleVariant`
- `UnitVariant`

**Methods:**

- `fn from_struct(node: &DataStruct) -> Self`
- `fn from_variant(node: &syn::Variant) -> Self`

**Traits:** Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ContainerKind`



## thiserror_impl::ast::Enum

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `attrs: crate::attr::Attrs<'a>`
- `ident: syn::Ident`
- `generics: &'a syn::Generics`
- `variants: Vec<Variant<'a>>`

**Methods:**

- `fn validate(self: &Self) -> Result<()>`
- `fn has_source(self: &Self) -> bool`
- `fn has_backtrace(self: &Self) -> bool`
- `fn has_display(self: &Self) -> bool`
- `fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Result<Self>`



## thiserror_impl::ast::Field

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `original: &'a syn::Field`
- `attrs: crate::attr::Attrs<'a>`
- `member: crate::unraw::MemberUnraw`
- `ty: &'a syn::Type`
- `contains_generic: bool`

**Methods:**

- `fn validate(self: &Self) -> Result<()>`
- `fn is_backtrace(self: &Self) -> bool`
- `fn source_span(self: &Self) -> Span`
- `fn multiple_from_syn(fields: &'a Fields, scope: &ParamsInScope<'a>) -> Result<Vec<Self>>`
- `fn from_syn(i: usize, node: &'a syn::Field, scope: &ParamsInScope<'a>) -> Result<Self>`



## thiserror_impl::ast::Input

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Struct(Struct<'a>)`
- `Enum(Enum<'a>)`

**Methods:**

- `fn validate(self: &Self) -> Result<()>`
- `fn from_syn(node: &'a DeriveInput) -> Result<Self>`



## thiserror_impl::ast::Struct

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `attrs: crate::attr::Attrs<'a>`
- `ident: syn::Ident`
- `generics: &'a syn::Generics`
- `fields: Vec<Field<'a>>`

**Methods:**

- `fn validate(self: &Self) -> Result<()>`
- `fn from_field(self: &Self) -> Option<&Field>`
- `fn source_field(self: &Self) -> Option<&Field>`
- `fn backtrace_field(self: &Self) -> Option<&Field>`
- `fn distinct_backtrace_field(self: &Self) -> Option<&Field>`
- `fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Self>`



## thiserror_impl::ast::Variant

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `original: &'a syn::Variant`
- `attrs: crate::attr::Attrs<'a>`
- `ident: syn::Ident`
- `fields: Vec<Field<'a>>`

**Methods:**

- `fn from_syn(node: &'a syn::Variant, scope: &ParamsInScope<'a>) -> Result<Self>`
- `fn validate(self: &Self) -> Result<()>`
- `fn from_field(self: &Self) -> Option<&Field>`
- `fn source_field(self: &Self) -> Option<&Field>`
- `fn backtrace_field(self: &Self) -> Option<&Field>`
- `fn distinct_backtrace_field(self: &Self) -> Option<&Field>`



