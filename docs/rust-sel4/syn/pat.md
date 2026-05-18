**syn > pat**

# Module: pat

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`FieldPat`](#fieldpat) - A single field in a struct pattern.
- [`PatIdent`](#patident) - A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
- [`PatOr`](#pator) - A pattern that matches any one of a set of cases.
- [`PatParen`](#patparen) - A parenthesized pattern: `(A | B)`.
- [`PatReference`](#patreference) - A reference pattern: `&mut var`.
- [`PatRest`](#patrest) - The dots in a tuple or slice pattern: `[0, 1, ..]`.
- [`PatSlice`](#patslice) - A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.
- [`PatStruct`](#patstruct) - A struct or struct variant pattern: `Variant { x, y, .. }`.
- [`PatTuple`](#pattuple) - A tuple pattern: `(a, b)`.
- [`PatTupleStruct`](#pattuplestruct) - A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
- [`PatType`](#pattype) - A type ascription pattern: `foo: f64`.
- [`PatWild`](#patwild) - A pattern that matches any value: `_`.

**Enums**

- [`Pat`](#pat) - A pattern in a local binding, function signature, match expression, or

---

## syn::pat::FieldPat

*Struct*

A single field in a struct pattern.

Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `member: crate::expr::Member`
- `colon_token: Option<$crate::token::Colon>`
- `pat: alloc::boxed::Box<Pat>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::pat::Pat

*Enum*

A pattern in a local binding, function signature, match expression, or
various other places.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Const(PatConst)` - A const block: `const { ... }`.
- `Ident(PatIdent)` - A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
- `Lit(PatLit)` - A literal pattern: `0`.
- `Macro(PatMacro)` - A macro in pattern position.
- `Or(PatOr)` - A pattern that matches any one of a set of cases.
- `Paren(PatParen)` - A parenthesized pattern: `(A | B)`.
- `Path(PatPath)` - A path pattern like `Color::Red`, optionally qualified with a
- `Range(PatRange)` - A range pattern: `1..=2`.
- `Reference(PatReference)` - A reference pattern: `&mut var`.
- `Rest(PatRest)` - The dots in a tuple or slice pattern: `[0, 1, ..]`.
- `Slice(PatSlice)` - A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.
- `Struct(PatStruct)` - A struct or struct variant pattern: `Variant { x, y, .. }`.
- `Tuple(PatTuple)` - A tuple pattern: `(a, b)`.
- `TupleStruct(PatTupleStruct)` - A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
- `Type(PatType)` - A type ascription pattern: `foo: f64`.
- `Verbatim(proc_macro2::TokenStream)` - Tokens in pattern position not interpreted by Syn.
- `Wild(PatWild)` - A pattern that matches any value: `_`.

**Methods:**

- `fn parse_single(input: ParseStream) -> Result<Self>` - Parse a pattern that does _not_ involve `|` at the top level.
- `fn parse_multi(input: ParseStream) -> Result<Self>` - Parse a pattern, possibly involving `|`, but not a leading `|`.
- `fn parse_multi_with_leading_vert(input: ParseStream) -> Result<Self>` - Parse a pattern, possibly involving `|`, possibly including a

**Trait Implementations:**

- **From**
  - `fn from(e: PatReference) -> Pat`
- **From**
  - `fn from(e: PatStruct) -> Pat`
- **From**
  - `fn from(e: PatType) -> Pat`
- **From**
  - `fn from(e: PatMacro) -> Pat`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: PatParen) -> Pat`
- **From**
  - `fn from(e: PatSlice) -> Pat`
- **From**
  - `fn from(e: PatTupleStruct) -> Pat`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: PatPath) -> Pat`
- **From**
  - `fn from(e: PatLit) -> Pat`
- **From**
  - `fn from(e: PatOr) -> Pat`
- **From**
  - `fn from(e: PatConst) -> Pat`
- **From**
  - `fn from(e: PatRest) -> Pat`
- **From**
  - `fn from(e: PatTuple) -> Pat`
- **From**
  - `fn from(e: PatWild) -> Pat`
- **From**
  - `fn from(e: PatRange) -> Pat`
- **From**
  - `fn from(e: PatIdent) -> Pat`



## syn::pat::PatIdent

*Struct*

A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

It may also be a unit struct or struct variant (e.g. `None`), or a
constant; these cannot be distinguished syntactically.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `by_ref: Option<$crate::token::Ref>`
- `mutability: Option<$crate::token::Mut>`
- `ident: crate::ident::Ident`
- `subpat: Option<($crate::token::At, alloc::boxed::Box<Pat>)>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatOr

*Struct*

A pattern that matches any one of a set of cases.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `leading_vert: Option<$crate::token::Or>`
- `cases: crate::punctuated::Punctuated<Pat, $crate::token::Or>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatParen

*Struct*

A parenthesized pattern: `(A | B)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `paren_token: token::Paren`
- `pat: alloc::boxed::Box<Pat>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatReference

*Struct*

A reference pattern: `&mut var`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `and_token: $crate::token::And`
- `mutability: Option<$crate::token::Mut>`
- `pat: alloc::boxed::Box<Pat>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatRest

*Struct*

The dots in a tuple or slice pattern: `[0, 1, ..]`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `dot2_token: $crate::token::DotDot`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::pat::PatSlice

*Struct*

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `bracket_token: token::Bracket`
- `elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatStruct

*Struct*

A struct or struct variant pattern: `Variant { x, y, .. }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `qself: Option<crate::path::QSelf>`
- `path: crate::path::Path`
- `brace_token: token::Brace`
- `fields: crate::punctuated::Punctuated<FieldPat, $crate::token::Comma>`
- `rest: Option<PatRest>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::pat::PatTuple

*Struct*

A tuple pattern: `(a, b)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `paren_token: token::Paren`
- `elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatTupleStruct

*Struct*

A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `qself: Option<crate::path::QSelf>`
- `path: crate::path::Path`
- `paren_token: token::Paren`
- `elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatType

*Struct*

A type ascription pattern: `foo: f64`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `pat: alloc::boxed::Box<Pat>`
- `colon_token: $crate::token::Colon`
- `ty: alloc::boxed::Box<crate::ty::Type>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::pat::PatWild

*Struct*

A pattern that matches any value: `_`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `underscore_token: $crate::token::Underscore`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: parsing



## Module: printing



