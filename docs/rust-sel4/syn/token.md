**syn > token**

# Module: token

## Contents

**Modules**

- [`private`](#private)

**Macros**

- [`define_delimiters`](#define_delimiters)
- [`define_keywords`](#define_keywords)
- [`define_punctuation`](#define_punctuation)
- [`define_punctuation_structs`](#define_punctuation_structs)
- [`impl_deref_if_len_is_1`](#impl_deref_if_len_is_1)
- [`impl_low_level_token`](#impl_low_level_token)

**Structs**

- [`Abstract`](#abstract) - `abstract`
- [`And`](#and) - `&`
- [`AndAnd`](#andand) - `&&`
- [`AndEq`](#andeq) - `&=`
- [`As`](#as) - `as`
- [`Async`](#async) - `async`
- [`At`](#at) - `@`
- [`Auto`](#auto) - `auto`
- [`Await`](#await) - `await`
- [`Become`](#become) - `become`
- [`Box`](#box) - `box`
- [`Brace`](#brace) - `{`&hellip;`}`
- [`Bracket`](#bracket) - `[`&hellip;`]`
- [`Break`](#break) - `break`
- [`Caret`](#caret) - `^`
- [`CaretEq`](#careteq) - `^=`
- [`Colon`](#colon) - `:`
- [`Comma`](#comma) - `,`
- [`Const`](#const) - `const`
- [`Continue`](#continue) - `continue`
- [`Crate`](#crate) - `crate`
- [`Default`](#default) - `default`
- [`Do`](#do) - `do`
- [`Dollar`](#dollar) - `$`
- [`Dot`](#dot) - `.`
- [`DotDot`](#dotdot) - `..`
- [`DotDotDot`](#dotdotdot) - `...`
- [`DotDotEq`](#dotdoteq) - `..=`
- [`Dyn`](#dyn) - `dyn`
- [`Else`](#else) - `else`
- [`Enum`](#enum) - `enum`
- [`Eq`](#eq) - `=`
- [`EqEq`](#eqeq) - `==`
- [`Extern`](#extern) - `extern`
- [`FatArrow`](#fatarrow) - `=>`
- [`Final`](#final) - `final`
- [`Fn`](#fn) - `fn`
- [`For`](#for) - `for`
- [`Ge`](#ge) - `>=`
- [`Group`](#group) - None-delimited group
- [`Gt`](#gt) - `>`
- [`If`](#if) - `if`
- [`Impl`](#impl) - `impl`
- [`In`](#in) - `in`
- [`LArrow`](#larrow) - `<-`
- [`Le`](#le) - `<=`
- [`Let`](#let) - `let`
- [`Loop`](#loop) - `loop`
- [`Lt`](#lt) - `<`
- [`Macro`](#macro) - `macro`
- [`Match`](#match) - `match`
- [`Minus`](#minus) - `-`
- [`MinusEq`](#minuseq) - `-=`
- [`Mod`](#mod) - `mod`
- [`Move`](#move) - `move`
- [`Mut`](#mut) - `mut`
- [`Ne`](#ne) - `!=`
- [`Not`](#not) - `!`
- [`Or`](#or) - `|`
- [`OrEq`](#oreq) - `|=`
- [`OrOr`](#oror) - `||`
- [`Override`](#override) - `override`
- [`Paren`](#paren) - `(`&hellip;`)`
- [`PathSep`](#pathsep) - `::`
- [`Percent`](#percent) - `%`
- [`PercentEq`](#percenteq) - `%=`
- [`Plus`](#plus) - `+`
- [`PlusEq`](#pluseq) - `+=`
- [`Pound`](#pound) - `#`
- [`Priv`](#priv) - `priv`
- [`Pub`](#pub) - `pub`
- [`Question`](#question) - `?`
- [`RArrow`](#rarrow) - `->`
- [`Raw`](#raw) - `raw`
- [`Ref`](#ref) - `ref`
- [`Return`](#return) - `return`
- [`SelfType`](#selftype) - `Self`
- [`SelfValue`](#selfvalue) - `self`
- [`Semi`](#semi) - `;`
- [`Shl`](#shl) - `<<`
- [`ShlEq`](#shleq) - `<<=`
- [`Shr`](#shr) - `>>`
- [`ShrEq`](#shreq) - `>>=`
- [`Slash`](#slash) - `/`
- [`SlashEq`](#slasheq) - `/=`
- [`Star`](#star) - `*`
- [`StarEq`](#stareq) - `*=`
- [`Static`](#static) - `static`
- [`Struct`](#struct) - `struct`
- [`Super`](#super) - `super`
- [`Tilde`](#tilde) - `~`
- [`Trait`](#trait) - `trait`
- [`Try`](#try) - `try`
- [`Type`](#type) - `type`
- [`Typeof`](#typeof) - `typeof`
- [`Underscore`](#underscore) - `_`
- [`Union`](#union) - `union`
- [`Unsafe`](#unsafe) - `unsafe`
- [`Unsized`](#unsized) - `unsized`
- [`Use`](#use) - `use`
- [`Virtual`](#virtual) - `virtual`
- [`Where`](#where) - `where`
- [`While`](#while) - `while`
- [`Yield`](#yield) - `yield`

**Traits**

- [`Token`](#token) - Marker trait for types that represent single tokens.

---

## syn::token::Abstract

*Struct*

`abstract`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::And

*Struct*

`&`

Usage:
 bitwise and logical AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::AndAnd

*Struct*

`&&`

Usage:
 lazy AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::AndEq

*Struct*

`&=`

Usage:
 bitwise AND assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::As

*Struct*

`as`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Async

*Struct*

`async`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::At

*Struct*

`@`

Usage:
 subpattern binding.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Auto

*Struct*

`auto`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Await

*Struct*

`await`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Become

*Struct*

`become`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Box

*Struct*

`box`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Brace

*Struct*

`{`&hellip;`}`

**Fields:**
- `span: DelimSpan`

**Methods:**

- `fn surround<F>(self: &Self, tokens: & mut TokenStream, f: F)`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Bracket

*Struct*

`[`&hellip;`]`

**Fields:**
- `span: DelimSpan`

**Methods:**

- `fn surround<F>(self: &Self, tokens: & mut TokenStream, f: F)`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Break

*Struct*

`break`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Caret

*Struct*

`^`

Usage:
 bitwise and logical XOR.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::CaretEq

*Struct*

`^=`

Usage:
 bitwise XOR assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Colon

*Struct*

`:`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Comma

*Struct*

`,`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## syn::token::Const

*Struct*

`const`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Continue

*Struct*

`continue`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Crate

*Struct*

`crate`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Default

*Struct*

`default`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Do

*Struct*

`do`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Dollar

*Struct*

`$`

Usage:
 macros.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Dot

*Struct*

`.`

Usage:
 field access, tuple index.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::DotDot

*Struct*

`..`

Usage:
 range, struct expressions, patterns, range patterns.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::DotDotDot

*Struct*

`...`

Usage:
 variadic functions, range patterns.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 3]`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::DotDotEq

*Struct*

`..=`

Usage:
 inclusive range, range patterns.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 3]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Dyn

*Struct*

`dyn`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Else

*Struct*

`else`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Enum

*Struct*

`enum`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Eq

*Struct*

`=`

Usage:
 assignment, attributes, various type definitions.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::EqEq

*Struct*

`==`

Usage:
 equal.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Extern

*Struct*

`extern`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::FatArrow

*Struct*

`=>`

Usage:
 match arms, macros.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Final

*Struct*

`final`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Fn

*Struct*

`fn`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::For

*Struct*

`for`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Ge

*Struct*

`>=`

Usage:
 greater than or equal to, generics.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Group

*Struct*

None-delimited group

**Fields:**
- `span: proc_macro2::Span`

**Methods:**

- `fn surround<F>(self: &Self, tokens: & mut TokenStream, f: F)`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Gt

*Struct*

`>`

Usage:
 greater than, generics, paths.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::If

*Struct*

`if`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Impl

*Struct*

`impl`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::In

*Struct*

`in`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::LArrow

*Struct*

`<-`

Usage:
 unused.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Le

*Struct*

`<=`

Usage:
 less than or equal to.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Let

*Struct*

`let`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Loop

*Struct*

`loop`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Lt

*Struct*

`<`

Usage:
 less than, generics, paths.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## syn::token::Macro

*Struct*

`macro`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Match

*Struct*

`match`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Minus

*Struct*

`-`

Usage:
 subtraction, negation.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::MinusEq

*Struct*

`-=`

Usage:
 subtraction assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Mod

*Struct*

`mod`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Move

*Struct*

`move`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Mut

*Struct*

`mut`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Ne

*Struct*

`!=`

Usage:
 not equal.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Not

*Struct*

`!`

Usage:
 bitwise and logical NOT, macro calls, inner attributes, never type, negative impls.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Or

*Struct*

`|`

Usage:
 bitwise and logical OR, closures, patterns in match, if let, and while let.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## syn::token::OrEq

*Struct*

`|=`

Usage:
 bitwise OR assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::OrOr

*Struct*

`||`

Usage:
 lazy OR, closures.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Override

*Struct*

`override`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Paren

*Struct*

`(`&hellip;`)`

**Fields:**
- `span: DelimSpan`

**Methods:**

- `fn surround<F>(self: &Self, tokens: & mut TokenStream, f: F)`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::PathSep

*Struct*

`::`

Usage:
 path separator.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Percent

*Struct*

`%`

Usage:
 remainder.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::PercentEq

*Struct*

`%=`

Usage:
 remainder assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Plus

*Struct*

`+`

Usage:
 addition, trait bounds, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



## syn::token::PlusEq

*Struct*

`+=`

Usage:
 addition assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Pound

*Struct*

`#`

Usage:
 attributes.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## syn::token::Priv

*Struct*

`priv`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Pub

*Struct*

`pub`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Question

*Struct*

`?`

Usage:
 question mark operator, questionably sized, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::RArrow

*Struct*

`->`

Usage:
 function return type, closure return type, function pointer type.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Raw

*Struct*

`raw`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Ref

*Struct*

`ref`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Return

*Struct*

`return`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::SelfType

*Struct*

`Self`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::SelfValue

*Struct*

`self`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Semi

*Struct*

`;`

Usage:
 terminator for various items and statements, array types.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## syn::token::Shl

*Struct*

`<<`

Usage:
 shift left, nested generics.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::ShlEq

*Struct*

`<<=`

Usage:
 shift left assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 3]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Shr

*Struct*

`>>`

Usage:
 shift right, nested generics.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::ShrEq

*Struct*

`>>=`

Usage:
 shift right assignment, nested generics.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 3]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Slash

*Struct*

`/`

Usage:
 division.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## syn::token::SlashEq

*Struct*

`/=`

Usage:
 division assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Star

*Struct*

`*`

Usage:
 multiplication, dereference, raw pointers, macro Kleene matcher, use wildcards.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## syn::token::StarEq

*Struct*

`*=`

Usage:
 multiplication assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 2]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Static

*Struct*

`static`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Struct

*Struct*

`struct`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Super

*Struct*

`super`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Tilde

*Struct*

`~`

Usage:
 unused since before Rust 1.0.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Token

*Trait*

Marker trait for types that represent single tokens.

This trait is sealed and cannot be implemented for types outside of Syn.



## syn::token::Trait

*Struct*

`trait`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Try

*Struct*

`try`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`



## syn::token::Type

*Struct*

`type`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Typeof

*Struct*

`typeof`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Underscore

*Struct*

`_`

Usage:
 wildcard patterns, inferred types, unnamed items in constants, extern crates, use declarations, and destructuring assignment.

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `spans: [Span; 1]`

**Traits:** Sealed, Token, Copy

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Union

*Struct*

`union`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Unsafe

*Struct*

`unsafe`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Unsized

*Struct*

`unsized`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Copy, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::Use

*Struct*

`use`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Virtual

*Struct*

`virtual`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Sealed, Copy, Token

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::token::Where

*Struct*

`where`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Token, Sealed

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::While

*Struct*

`while`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Token, Sealed, Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::token::Yield

*Struct*

`yield`

Don't try to remember the name of this type &mdash; use the
[`Token!`] macro instead.

[`Token!`]: crate::token

**Fields:**
- `span: Span`

**Traits:** Copy, Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::token::define_delimiters

*Declarative Macro*

```rust
macro_rules! define_delimiters {
    ($($delim:ident pub struct $name:ident #[$doc:meta])*) => { ... };
}
```



## syn::token::define_keywords

*Declarative Macro*

```rust
macro_rules! define_keywords {
    ($($token:literal pub struct $name:ident)*) => { ... };
}
```



## syn::token::define_punctuation

*Declarative Macro*

```rust
macro_rules! define_punctuation {
    ($($token:literal pub struct $name:ident/$len:tt #[doc = $usage:literal])*) => { ... };
}
```



## syn::token::define_punctuation_structs

*Declarative Macro*

```rust
macro_rules! define_punctuation_structs {
    ($($token:literal pub struct $name:ident/$len:tt #[doc = $usage:literal])*) => { ... };
}
```



## syn::token::impl_deref_if_len_is_1

*Declarative Macro*

```rust
macro_rules! impl_deref_if_len_is_1 {
    ($name:ident/1) => { ... };
    ($name:ident/$len:literal) => { ... };
}
```



## syn::token::impl_low_level_token

*Declarative Macro*

```rust
macro_rules! impl_low_level_token {
    ($display:literal $($path:ident)::+ $get:ident) => { ... };
}
```



## Module: private



