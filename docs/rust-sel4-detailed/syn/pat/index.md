*[syn](../index.md) / [pat](index.md)*

---

# Module `pat`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`PatConst`](#patconst)
  - [`PatLit`](#patlit)
  - [`PatMacro`](#patmacro)
  - [`PatPath`](#patpath)
  - [`PatRange`](#patrange)
  - [`PatIdent`](#patident)
  - [`PatOr`](#pator)
  - [`PatParen`](#patparen)
  - [`PatReference`](#patreference)
  - [`PatRest`](#patrest)
  - [`PatSlice`](#patslice)
  - [`PatStruct`](#patstruct)
  - [`PatTuple`](#pattuple)
  - [`PatTupleStruct`](#pattuplestruct)
  - [`PatType`](#pattype)
  - [`PatWild`](#patwild)
  - [`FieldPat`](#fieldpat)
- [Enums](#enums)
  - [`Pat`](#pat)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`PatConst`](#patconst) | struct |  |
| [`PatLit`](#patlit) | struct |  |
| [`PatMacro`](#patmacro) | struct |  |
| [`PatPath`](#patpath) | struct |  |
| [`PatRange`](#patrange) | struct |  |
| [`PatIdent`](#patident) | struct | A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`. |
| [`PatOr`](#pator) | struct | A pattern that matches any one of a set of cases. |
| [`PatParen`](#patparen) | struct | A parenthesized pattern: `(A \| B)`. |
| [`PatReference`](#patreference) | struct | A reference pattern: `&mut var`. |
| [`PatRest`](#patrest) | struct | The dots in a tuple or slice pattern: `[0, 1, ..]`. |
| [`PatSlice`](#patslice) | struct | A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`. |
| [`PatStruct`](#patstruct) | struct | A struct or struct variant pattern: `Variant { x, y, .. |
| [`PatTuple`](#pattuple) | struct | A tuple pattern: `(a, b)`. |
| [`PatTupleStruct`](#pattuplestruct) | struct | A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`. |
| [`PatType`](#pattype) | struct | A type ascription pattern: `foo: f64`. |
| [`PatWild`](#patwild) | struct | A pattern that matches any value: `_`. |
| [`FieldPat`](#fieldpat) | struct | A single field in a struct pattern. |
| [`Pat`](#pat) | enum | A pattern in a local binding, function signature, match expression, or various other places. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `PatConst`

```rust
struct PatConst {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

#### Implementations

- <span id="crateexprconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprConst`

- <span id="crateexprconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprConst`

- <span id="crateexprconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl Hash for crate::ExprConst`

- <span id="crateexprconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprConst`

- <span id="crateexprexprconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprConst`

- <span id="crateexprconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprConst`

##### `impl Spanned for ExprConst`

- <span id="exprconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprConst`

- <span id="crateexprexprconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatLit`

```rust
struct PatLit {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- <span id="crateexprlit-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLit`

- <span id="crateexprlit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLit`

- <span id="crateexprlit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl Hash for crate::ExprLit`

- <span id="crateexprlit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLit`

- <span id="crateexprexprlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprLit`

- <span id="crateexprlit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLit`

##### `impl Spanned for ExprLit`

- <span id="exprlit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLit`

- <span id="crateexprexprlit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatMacro`

```rust
struct PatMacro {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- <span id="crateexprmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMacro`

- <span id="crateexprmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMacro`

- <span id="crateexprmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl Hash for crate::ExprMacro`

- <span id="crateexprmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprMacro`

- <span id="crateexprmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMacro`

##### `impl Spanned for ExprMacro`

- <span id="exprmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatPath`

```rust
struct PatPath {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `core::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- <span id="crateexprpath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprPath`

- <span id="crateexprpath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprPath`

- <span id="crateexprpath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl Hash for crate::ExprPath`

- <span id="crateexprpath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprPath`

- <span id="crateexprexprpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprPath`

- <span id="crateexprpath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprPath`

##### `impl Spanned for ExprPath`

- <span id="exprpath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprPath`

- <span id="crateexprexprpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatRange`

```rust
struct PatRange {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub start: Option<alloc::boxed::Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<alloc::boxed::Box<Expr>>,
}
```

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- <span id="crateexprrange-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRange`

- <span id="crateexprrange-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRange`

- <span id="crateexprrange-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl Hash for crate::ExprRange`

- <span id="crateexprrange-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRange`

- <span id="crateexprexprrange-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprRange`

- <span id="crateexprrange-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRange`

##### `impl Spanned for ExprRange`

- <span id="exprrange-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRange`

- <span id="crateexprexprrange-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatIdent`

```rust
struct PatIdent {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub by_ref: Option<token::Ref>,
    pub mutability: Option<token::Mut>,
    pub ident: crate::ident::Ident,
    pub subpat: Option<(token::At, alloc::boxed::Box<Pat>)>,
}
```

A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

It may also be a unit struct or struct variant (e.g. `None`), or a
constant; these cannot be distinguished syntactically.

#### Implementations

- <span id="cratepatident-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatIdent`

- <span id="cratepatident-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatIdent`

- <span id="cratepatident-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatIdent`

##### `impl Hash for crate::PatIdent`

- <span id="cratepatident-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatIdent`

- <span id="cratepatident-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatIdent`

##### `impl Spanned for PatIdent`

- <span id="patident-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatIdent`

- <span id="cratepatpatident-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatOr`

```rust
struct PatOr {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub leading_vert: Option<token::Or>,
    pub cases: crate::punctuated::Punctuated<Pat, token::Or>,
}
```

A pattern that matches any one of a set of cases.

#### Implementations

- <span id="cratepator-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatOr`

- <span id="cratepator-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatOr`

- <span id="cratepator-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatOr`

##### `impl Hash for crate::PatOr`

- <span id="cratepator-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatOr`

- <span id="cratepator-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatOr`

##### `impl Spanned for PatOr`

- <span id="pator-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatOr`

- <span id="cratepatpator-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatParen`

```rust
struct PatParen {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub pat: alloc::boxed::Box<Pat>,
}
```

A parenthesized pattern: `(A | B)`.

#### Implementations

- <span id="cratepatparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatParen`

- <span id="cratepatparen-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatParen`

- <span id="cratepatparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatParen`

##### `impl Hash for crate::PatParen`

- <span id="cratepatparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatParen`

- <span id="cratepatparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatParen`

##### `impl Spanned for PatParen`

- <span id="patparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatParen`

- <span id="cratepatpatparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatReference`

```rust
struct PatReference {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub pat: alloc::boxed::Box<Pat>,
}
```

A reference pattern: `&mut var`.

#### Implementations

- <span id="cratepatreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatReference`

- <span id="cratepatreference-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatReference`

- <span id="cratepatreference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatReference`

##### `impl Hash for crate::PatReference`

- <span id="cratepatreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatReference`

- <span id="cratepatreference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatReference`

##### `impl Spanned for PatReference`

- <span id="patreference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatReference`

- <span id="cratepatpatreference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatRest`

```rust
struct PatRest {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub dot2_token: token::DotDot,
}
```

The dots in a tuple or slice pattern: `[0, 1, ..]`.

#### Implementations

- <span id="cratepatrest-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatRest`

- <span id="cratepatrest-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatRest`

- <span id="cratepatrest-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatRest`

##### `impl Hash for crate::PatRest`

- <span id="cratepatrest-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatRest`

- <span id="cratepatrest-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatRest`

##### `impl Spanned for PatRest`

- <span id="patrest-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatRest`

- <span id="cratepatpatrest-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatSlice`

```rust
struct PatSlice {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

#### Implementations

- <span id="cratepatslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatSlice`

- <span id="cratepatslice-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatSlice`

- <span id="cratepatslice-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatSlice`

##### `impl Hash for crate::PatSlice`

- <span id="cratepatslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatSlice`

- <span id="cratepatslice-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatSlice`

##### `impl Spanned for PatSlice`

- <span id="patslice-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatSlice`

- <span id="cratepatpatslice-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatStruct`

```rust
struct PatStruct {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldPat, token::Comma>,
    pub rest: Option<PatRest>,
}
```

A struct or struct variant pattern: `Variant { x, y, .. }`.

#### Implementations

- <span id="cratepatstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatStruct`

- <span id="cratepatstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatStruct`

- <span id="cratepatstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatStruct`

##### `impl Hash for crate::PatStruct`

- <span id="cratepatstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatStruct`

- <span id="cratepatstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatStruct`

##### `impl Spanned for PatStruct`

- <span id="patstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatStruct`

- <span id="cratepatpatstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatTuple`

```rust
struct PatTuple {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

A tuple pattern: `(a, b)`.

#### Implementations

- <span id="cratepattuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTuple`

- <span id="cratepattuple-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatTuple`

- <span id="cratepattuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTuple`

##### `impl Hash for crate::PatTuple`

- <span id="cratepattuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatTuple`

- <span id="cratepattuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatTuple`

##### `impl Spanned for PatTuple`

- <span id="pattuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatTuple`

- <span id="cratepatpattuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatTupleStruct`

```rust
struct PatTupleStruct {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

#### Implementations

- <span id="cratepattuplestruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTupleStruct`

- <span id="cratepattuplestruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatTupleStruct`

- <span id="cratepattuplestruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTupleStruct`

##### `impl Hash for crate::PatTupleStruct`

- <span id="cratepattuplestruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatTupleStruct`

- <span id="cratepattuplestruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatTupleStruct`

##### `impl Spanned for PatTupleStruct`

- <span id="pattuplestruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatTupleStruct`

- <span id="cratepatpattuplestruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatType`

```rust
struct PatType {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub pat: alloc::boxed::Box<Pat>,
    pub colon_token: token::Colon,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
}
```

A type ascription pattern: `foo: f64`.

#### Implementations

- <span id="cratepattype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatType`

- <span id="cratepattype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatType`

- <span id="cratepattype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatType`

##### `impl Hash for crate::PatType`

- <span id="cratepattype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::pat::PatType`

- <span id="cratepatpattype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PatType`

- <span id="cratepattype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatType`

##### `impl Spanned for PatType`

- <span id="pattype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatType`

- <span id="cratepatpattype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatWild`

```rust
struct PatWild {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

A pattern that matches any value: `_`.

#### Implementations

- <span id="cratepatwild-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatWild`

- <span id="cratepatwild-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatWild`

- <span id="cratepatwild-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatWild`

##### `impl Hash for crate::PatWild`

- <span id="cratepatwild-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatWild`

- <span id="cratepatwild-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatWild`

##### `impl Spanned for PatWild`

- <span id="patwild-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatWild`

- <span id="cratepatpatwild-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldPat`

```rust
struct FieldPat {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub member: crate::expr::Member,
    pub colon_token: Option<token::Colon>,
    pub pat: alloc::boxed::Box<Pat>,
}
```

A single field in a struct pattern.

Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.

#### Trait Implementations

##### `impl Clone for crate::FieldPat`

- <span id="cratefieldpat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldPat`

- <span id="cratefieldpat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldPat`

##### `impl Hash for crate::FieldPat`

- <span id="cratefieldpat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::FieldPat`

- <span id="cratefieldpat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldPat`

##### `impl Spanned for FieldPat`

- <span id="fieldpat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::FieldPat`

- <span id="cratepatfieldpat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `Pat`

```rust
enum Pat {
    Const(PatConst),
    Ident(PatIdent),
    Lit(PatLit),
    Macro(PatMacro),
    Or(PatOr),
    Paren(PatParen),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Rest(PatRest),
    Slice(PatSlice),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Type(PatType),
    Verbatim(proc_macro2::TokenStream),
    Wild(PatWild),
}
```

A pattern in a local binding, function signature, match expression, or
various other places.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  A const block: `const { ... }`.

- **`Ident`**

  A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

- **`Lit`**

  A literal pattern: `0`.

- **`Macro`**

  A macro in pattern position.

- **`Or`**

  A pattern that matches any one of a set of cases.

- **`Paren`**

  A parenthesized pattern: `(A | B)`.

- **`Path`**

  A path pattern like `Color::Red`, optionally qualified with a
  self-type.
  
  Unqualified path patterns can legally refer to variants, structs,
  constants or associated constants. Qualified path patterns like
  `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
  associated constants.

- **`Range`**

  A range pattern: `1..=2`.

- **`Reference`**

  A reference pattern: `&mut var`.

- **`Rest`**

  The dots in a tuple or slice pattern: `[0, 1, ..]`.

- **`Slice`**

  A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

- **`Struct`**

  A struct or struct variant pattern: `Variant { x, y, .. }`.

- **`Tuple`**

  A tuple pattern: `(a, b)`.

- **`TupleStruct`**

  A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

- **`Type`**

  A type ascription pattern: `foo: f64`.

- **`Verbatim`**

  Tokens in pattern position not interpreted by Syn.

- **`Wild`**

  A pattern that matches any value: `_`.

#### Implementations

- <span id="cratepatpat-parse-single"></span>`fn parse_single(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a pattern that does _not_ involve `|` at the top level.

  

  This parser matches the behavior of the `$:pat_param` macro_rules

  matcher, and on editions prior to Rust 2021, the behavior of

  `$:pat`.

  

  In Rust syntax, some examples of where this syntax would occur are

  in the argument pattern of functions and closures. Patterns using

  `|` are not allowed to occur in these positions.

  

  ```compile_fail

  fn f(Some(_) | None: Option<T>) {

      let _ = |Some(_) | None: Option<T>| {};

      //       ^^^^^^^^^^^^^^^^^^^^^^^^^??? :(

  }

  ```

  

  ```console

  error: top-level or-patterns are not allowed in function parameters

   --> src/main.rs:1:6

    |

  1 | fn f(Some(_) | None: Option<T>) {

    |      ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Some(_) | None)`

  ```

- <span id="cratepatpat-parse-multi"></span>`fn parse_multi(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a pattern, possibly involving `|`, but not a leading `|`.

- <span id="cratepatpat-parse-multi-with-leading-vert"></span>`fn parse_multi_with_leading_vert(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a pattern, possibly involving `|`, possibly including a

  leading `|`.

  

  This parser matches the behavior of the Rust 2021 edition's `$:pat`

  macro_rules matcher.

  

  In Rust syntax, an example of where this syntax would occur is in

  the pattern of a `match` arm, where the language permits an optional

  leading `|`, although it is not idiomatic to write one there in

  handwritten code.

  

  ```rust

  let wat = None;

  match wat {

      | None | Some(false) => {}

      | Some(true) => {}

  }

  ```

  

  The compiler accepts it only to facilitate some situations in

  macro-generated code where a macro author might need to write:

  

  ```rust

  macro_rules! doc {

      ($value:expr, ($($conditions1:pat),*), ($($conditions2:pat),*), $then:expr) => {

  match $value {

      $(| $conditions1)* $(| $conditions2)* => $then

  }

      };

  }

  

  doc!(true, (true), (false), {});

  doc!(true, (), (true, false), {});

  doc!(true, (true, false), (), {});

  ```

  

  Expressing the same thing correctly in the case that either one (but

  not both) of `$conditions1` and `$conditions2` might be empty,

  without leading `|`, is complex.

  

  Use `Pat::parse_multi` instead if you are not intending to support

  macro-generated macro input.

#### Trait Implementations

##### `impl Clone for crate::Pat`

- <span id="cratepat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Pat`

- <span id="cratepat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Pat`

##### `impl Hash for crate::Pat`

- <span id="cratepat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Pat`

- <span id="cratepat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Pat`

##### `impl Spanned for Pat`

- <span id="pat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Pat`

- <span id="pat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

