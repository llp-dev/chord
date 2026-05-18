*[syn](../index.md) / [path](index.md)*

---

# Module `path`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Path`](#path)
  - [`PathSegment`](#pathsegment)
  - [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments)
  - [`AssocType`](#assoctype)
  - [`AssocConst`](#assocconst)
  - [`Constraint`](#constraint)
  - [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments)
  - [`QSelf`](#qself)
- [Enums](#enums)
  - [`PathArguments`](#patharguments)
  - [`GenericArgument`](#genericargument)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Path`](#path) | struct | A path at which a named item is exported (e.g. `alloc::collections::HashMap`). |
| [`PathSegment`](#pathsegment) | struct | A segment of a path together with any path arguments on that segment. |
| [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments) | struct | Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K, V>`. |
| [`AssocType`](#assoctype) | struct | A binding (equality constraint) on an associated type: the `Item = u8` in `Iterator<Item = u8>`. |
| [`AssocConst`](#assocconst) | struct | An equality constraint on an associated constant: the `PANIC = false` in `Trait<PANIC = false>`. |
| [`Constraint`](#constraint) | struct | An associated type bound: `Iterator<Item: Display>`. |
| [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments) | struct | Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) -> C`. |
| [`QSelf`](#qself) | struct | The explicit Self type in a qualified path: the `T` in `<T as Display>::fmt`. |
| [`PathArguments`](#patharguments) | enum | Angle bracketed or parenthesized arguments of a path segment. |
| [`GenericArgument`](#genericargument) | enum | An individual generic argument, like `'a`, `T`, or `Item = T`. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Path`

```rust
struct Path {
    pub leading_colon: Option<token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, token::PathSep>,
}
```

A path at which a named item is exported (e.g. `alloc::collections::HashMap`).

#### Implementations

- <span id="cratepathpath-parse-mod-style"></span>`fn parse_mod_style(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a `Path` containing no path arguments on any of its segments.

  

  # Example

  

  ```rust

  use syn::{Path, Result, Token};

  use syn::parse::{Parse, ParseStream};

  

  // A simplified single `use` statement like:

  //

  //     use alloc::collections::HashMap;

  //

  // Note that generic parameters are not allowed in a `use` statement

  // so the following must not be accepted.

  //

  //     use a::<b>::c;

  struct SingleUse {

      use_token: Token![use],

      path: Path,

  }

  

  impl Parse for SingleUse {

      fn parse(input: ParseStream) -> Result<Self> {

          Ok(SingleUse {

              use_token: input.parse()?,

              path: input.call(Path::parse_mod_style)?,

          })

      }

  }

  ```

- <span id="cratepathpath-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratepathpath-parse-rest"></span>`fn parse_rest(input: ParseStream<'_>, path: &mut Self, expr_style: bool) -> Result<()>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratepathpath-is-mod-style"></span>`fn is_mod_style(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Path`

- <span id="cratepath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Path`

- <span id="cratepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Path`

##### `impl Hash for crate::Path`

- <span id="cratepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::Path`

- <span id="cratepathpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Path`

- <span id="cratepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialEq for syn::Path`

##### `impl Sealed for Path`

##### `impl Spanned for Path`

- <span id="path-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::Path`

- <span id="cratepathpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

A segment of a path together with any path arguments on that segment.

#### Implementations

- <span id="cratepathpathsegment-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Clone for crate::PathSegment`

- <span id="cratepathsegment-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PathSegment`

- <span id="cratepathsegment-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PathSegment`

##### `impl Hash for crate::PathSegment`

- <span id="cratepathsegment-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::PathSegment`

- <span id="cratepathpathsegment-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PathSegment`

- <span id="cratepathsegment-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PathSegment`

##### `impl Spanned for PathSegment`

- <span id="pathsegment-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::PathSegment`

- <span id="cratepathpathsegment-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AngleBracketedGenericArguments`

```rust
struct AngleBracketedGenericArguments {
    pub colon2_token: Option<token::PathSep>,
    pub lt_token: token::Lt,
    pub args: crate::punctuated::Punctuated<GenericArgument, token::Comma>,
    pub gt_token: token::Gt,
}
```

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

#### Implementations

- <span id="cratepathanglebracketedgenericarguments-parse-turbofish"></span>`fn parse_turbofish(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse `::<…>` with mandatory leading `::`.

  

  The ordinary [`Parse`](../parse/index.md) impl for `AngleBracketedGenericArguments`

  parses optional leading `::`.

- <span id="cratepathanglebracketedgenericarguments-do-parse"></span>`fn do_parse(colon2_token: Option<token::PathSep>, input: ParseStream<'_>) -> Result<Self>` — [`PathSep`](../token/index.md#pathsep), [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Clone for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AngleBracketedGenericArguments`

##### `impl Hash for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AngleBracketedGenericArguments`

##### `impl Spanned for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AssocType`

```rust
struct AssocType {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
}
```

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

#### Trait Implementations

##### `impl Clone for crate::AssocType`

- <span id="crateassoctype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AssocType`

- <span id="crateassoctype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocType`

##### `impl Hash for crate::AssocType`

- <span id="crateassoctype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AssocType`

- <span id="crateassoctype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AssocType`

##### `impl Spanned for AssocType`

- <span id="assoctype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AssocType`

- <span id="cratepathassoctype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AssocConst`

```rust
struct AssocConst {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

#### Trait Implementations

##### `impl Clone for crate::AssocConst`

- <span id="crateassocconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AssocConst`

- <span id="crateassocconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocConst`

##### `impl Hash for crate::AssocConst`

- <span id="crateassocconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AssocConst`

- <span id="crateassocconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AssocConst`

##### `impl Spanned for AssocConst`

- <span id="assocconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AssocConst`

- <span id="cratepathassocconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Constraint`

```rust
struct Constraint {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Clone for crate::Constraint`

- <span id="crateconstraint-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Constraint`

- <span id="crateconstraint-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Constraint`

##### `impl Hash for crate::Constraint`

- <span id="crateconstraint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Constraint`

- <span id="crateconstraint-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Constraint`

##### `impl Spanned for Constraint`

- <span id="constraint-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::Constraint`

- <span id="cratepathconstraint-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ParenthesizedGenericArguments`

```rust
struct ParenthesizedGenericArguments {
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<crate::ty::Type, token::Comma>,
    pub output: crate::ty::ReturnType,
}
```

Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
C`.

#### Fields

- **`inputs`**: `crate::punctuated::Punctuated<crate::ty::Type, token::Comma>`

  `(A, B)`

- **`output`**: `crate::ty::ReturnType`

  `C`

#### Implementations

- <span id="crateparenthesizedgenericarguments-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ParenthesizedGenericArguments`

##### `impl Hash for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ParenthesizedGenericArguments`

##### `impl Spanned for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `QSelf`

```rust
struct QSelf {
    pub lt_token: token::Lt,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
    pub position: usize,
    pub as_token: Option<token::As>,
    pub gt_token: token::Gt,
}
```

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

#### Trait Implementations

##### `impl Clone for crate::QSelf`

- <span id="crateqself-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::QSelf`

- <span id="crateqself-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::QSelf`

##### `impl Hash for crate::QSelf`

- <span id="crateqself-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::QSelf`

- <span id="crateqself-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::QSelf`

##### `impl Spanned for crate::path::QSelf`

- <span id="cratepathqself-spanned-span"></span>`fn span(&self) -> Span`

## Enums

### `PathArguments`

```rust
enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
```

Angle bracketed or parenthesized arguments of a path segment.

## Angle bracketed

The `<'a, T>` in `core::slice::iter<'a, T>`.

## Parenthesized

The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Variants

- **`AngleBracketed`**

  The `<'a, T>` in `core::slice::iter<'a, T>`.

- **`Parenthesized`**

  The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Implementations

- <span id="patharguments-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="patharguments-is-none"></span>`fn is_none(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::PathArguments`

- <span id="cratepatharguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PathArguments`

- <span id="cratepatharguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathArguments`

- <span id="patharguments-default"></span>`fn default() -> Self`

##### `impl Eq for crate::PathArguments`

##### `impl Hash for crate::PathArguments`

- <span id="cratepatharguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PathArguments`

- <span id="cratepatharguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PathArguments`

##### `impl Spanned for PathArguments`

- <span id="patharguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::PathArguments`

- <span id="cratepathpatharguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `GenericArgument`

```rust
enum GenericArgument {
    Lifetime(crate::lifetime::Lifetime),
    Type(crate::ty::Type),
    Const(crate::expr::Expr),
    AssocType(AssocType),
    AssocConst(AssocConst),
    Constraint(Constraint),
}
```

An individual generic argument, like `'a`, `T`, or `Item = T`.

#### Variants

- **`Lifetime`**

  A lifetime argument.

- **`Type`**

  A type argument.

- **`Const`**

  A const expression. Must be inside of a block.
  
  NOTE: Identity expressions are represented as Type arguments, as
  they are indistinguishable syntactically.

- **`AssocType`**

  A binding (equality constraint) on an associated type: the `Item =
  u8` in `Iterator<Item = u8>`.

- **`AssocConst`**

  An equality constraint on an associated constant: the `PANIC =
  false` in `Trait<PANIC = false>`.

- **`Constraint`**

  An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Clone for crate::GenericArgument`

- <span id="crategenericargument-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::GenericArgument`

- <span id="crategenericargument-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericArgument`

##### `impl Hash for crate::GenericArgument`

- <span id="crategenericargument-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::GenericArgument`

- <span id="cratepathgenericargument-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::GenericArgument`

- <span id="crategenericargument-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for GenericArgument`

##### `impl Spanned for GenericArgument`

- <span id="genericargument-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::GenericArgument`

- <span id="cratepathgenericargument-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

