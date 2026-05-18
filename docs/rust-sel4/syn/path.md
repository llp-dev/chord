**syn > path**

# Module: path

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments) - Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
- [`AssocConst`](#assocconst) - An equality constraint on an associated constant: the `PANIC = false` in
- [`AssocType`](#assoctype) - A binding (equality constraint) on an associated type: the `Item = u8`
- [`Constraint`](#constraint) - An associated type bound: `Iterator<Item: Display>`.
- [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments) - Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
- [`Path`](#path) - A path at which a named item is exported (e.g. `alloc::collections::HashMap`).
- [`PathSegment`](#pathsegment) - A segment of a path together with any path arguments on that segment.
- [`QSelf`](#qself) - The explicit Self type in a qualified path: the `T` in `<T as

**Enums**

- [`GenericArgument`](#genericargument) - An individual generic argument, like `'a`, `T`, or `Item = T`.
- [`PathArguments`](#patharguments) - Angle bracketed or parenthesized arguments of a path segment.

---

## syn::path::AngleBracketedGenericArguments

*Struct*

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

**Fields:**
- `colon2_token: Option<$crate::token::PathSep>`
- `lt_token: $crate::token::Lt`
- `args: crate::punctuated::Punctuated<GenericArgument, $crate::token::Comma>`
- `gt_token: $crate::token::Gt`

**Methods:**

- `fn parse_turbofish(input: ParseStream) -> Result<Self>` - Parse `::<…>` with mandatory leading `::`.
- `fn do_parse(colon2_token: Option<$crate::token::PathSep>, input: ParseStream) -> Result<Self>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::path::AssocConst

*Struct*

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

**Fields:**
- `ident: crate::ident::Ident`
- `generics: Option<AngleBracketedGenericArguments>`
- `eq_token: $crate::token::Eq`
- `value: crate::expr::Expr`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::path::AssocType

*Struct*

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

**Fields:**
- `ident: crate::ident::Ident`
- `generics: Option<AngleBracketedGenericArguments>`
- `eq_token: $crate::token::Eq`
- `ty: crate::ty::Type`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::path::Constraint

*Struct*

An associated type bound: `Iterator<Item: Display>`.

**Fields:**
- `ident: crate::ident::Ident`
- `generics: Option<AngleBracketedGenericArguments>`
- `colon_token: $crate::token::Colon`
- `bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::path::GenericArgument

*Enum*

An individual generic argument, like `'a`, `T`, or `Item = T`.

**Variants:**
- `Lifetime(crate::lifetime::Lifetime)` - A lifetime argument.
- `Type(crate::ty::Type)` - A type argument.
- `Const(crate::expr::Expr)` - A const expression. Must be inside of a block.
- `AssocType(AssocType)` - A binding (equality constraint) on an associated type: the `Item =
- `AssocConst(AssocConst)` - An equality constraint on an associated constant: the `PANIC =
- `Constraint(Constraint)` - An associated type bound: `Iterator<Item: Display>`.

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::path::ParenthesizedGenericArguments

*Struct*

Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
C`.

**Fields:**
- `paren_token: token::Paren`
- `inputs: crate::punctuated::Punctuated<crate::ty::Type, $crate::token::Comma>` - `(A, B)`
- `output: crate::ty::ReturnType` - `C`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::path::Path

*Struct*

A path at which a named item is exported (e.g. `alloc::collections::HashMap`).

**Fields:**
- `leading_colon: Option<$crate::token::PathSep>`
- `segments: crate::punctuated::Punctuated<PathSegment, $crate::token::PathSep>`

**Methods:**

- `fn parse_mod_style(input: ParseStream) -> Result<Self>` - Parse a `Path` containing no path arguments on any of its segments.
- `fn parse_helper(input: ParseStream, expr_style: bool) -> Result<Self>`
- `fn parse_rest(input: ParseStream, path: & mut Self, expr_style: bool) -> Result<()>`
- `fn is_mod_style(self: &Self) -> bool`
- `fn is_ident<I>(self: &Self, ident: &I) -> bool` - Determines whether this is a path of length 1 equal to the given
- `fn get_ident(self: &Self) -> Option<&Ident>` - If this path consists of a single ident, returns the ident.
- `fn require_ident(self: &Self) -> Result<&Ident>` - An error if this path is not a single ident, as defined in `get_ident`.

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **From**
  - `fn from(segment: T) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::path::PathArguments

*Enum*

Angle bracketed or parenthesized arguments of a path segment.

## Angle bracketed

The `<'a, T>` in `core::slice::iter<'a, T>`.

## Parenthesized

The `(A, B) -> C` in `Fn(A, B) -> C`.

**Variants:**
- `None`
- `AngleBracketed(AngleBracketedGenericArguments)` - The `<'a, T>` in `core::slice::iter<'a, T>`.
- `Parenthesized(ParenthesizedGenericArguments)` - The `(A, B) -> C` in `Fn(A, B) -> C`.

**Methods:**

- `fn is_empty(self: &Self) -> bool`
- `fn is_none(self: &Self) -> bool`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`



## syn::path::PathSegment

*Struct*

A segment of a path together with any path arguments on that segment.

**Fields:**
- `ident: crate::ident::Ident`
- `arguments: PathArguments`

**Methods:**

- `fn parse_helper(input: ParseStream, expr_style: bool) -> Result<Self>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(ident: T) -> Self`



## syn::path::QSelf

*Struct*

The explicit Self type in a qualified path: the `T` in `<T as
Display>::fmt`.

The actual path, including the trait and the associated item, is stored
separately. The `position` field represents the index of the associated
item qualified with this Self type.

```text
<Vec<T> as a::b::Trait>::AssociatedItem
 ^~~~~~    ~~~~~~~~~~~~~~^
 ty        position = 3

<Vec<T>>::AssociatedItem
 ^~~~~~   ^
 ty       position = 0
```

**Fields:**
- `lt_token: $crate::token::Lt`
- `ty: alloc::boxed::Box<crate::ty::Type>`
- `position: usize`
- `as_token: Option<$crate::token::As>`
- `gt_token: $crate::token::Gt`

**Traits:** Sealed

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Spanned**
  - `fn span(self: &Self) -> Span`



## Module: parsing



## Module: printing



