*[syn](../index.md) / [token](index.md)*

---

# Module `token`

Tokens representing Rust punctuation, keywords, and delimiters.

The type names in this module can be difficult to keep straight, so we
prefer to use the `Token!` macro instead. This is a type-macro that
expands to the token type of the given token.

# Example

The [`ItemStatic`](../item/index.md) syntax tree node is defined like this.

```rust
use syn::{Attribute, Expr, Ident, Token, Type, Visibility};

pub struct ItemStatic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_token: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: Box<Type>,
    pub eq_token: Token![=],
    pub expr: Box<Expr>,
    pub semi_token: Token![;],
}
```

# Parsing

Keywords and punctuation can be parsed through the `ParseStream::parse`
method. Delimiter tokens are parsed using the `parenthesized!`,
`bracketed!` and `braced!` macros.




```rust
use syn::{Attribute, Result};
use syn::parse::{Parse, ParseStream};

enum ItemStatic {}

// Parse the ItemStatic struct shown above.
impl Parse for ItemStatic {
    fn parse(input: ParseStream) -> Result<Self> {
        use syn::ItemStatic;
        fn parse(input: ParseStream) -> Result<ItemStatic> {
        Ok(ItemStatic {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            expr: input.parse()?,
            semi_token: input.parse()?,
        })
        }
        unimplemented!()
    }
}
```

# Other operations

Every keyword and punctuation token supports the following operations.

- [Peeking] — `input.peek(Token![...])`

- [Parsing] — `input.parse::<Token![...]>()?`

- [Printing] — `quote!( ... #the_token ... )`

- Construction from a `Span` — `let the_token = Token![...](sp)`

- Field access to its span — `let sp = the_token.span`





## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Underscore`](#underscore)
  - [`Group`](#group)
  - [`Abstract`](#abstract)
  - [`As`](#as)
  - [`Async`](#async)
  - [`Auto`](#auto)
  - [`Await`](#await)
  - [`Become`](#become)
  - [`Box`](#box)
  - [`Break`](#break)
  - [`Const`](#const)
  - [`Continue`](#continue)
  - [`Crate`](#crate)
  - [`Default`](#default)
  - [`Do`](#do)
  - [`Dyn`](#dyn)
  - [`Else`](#else)
  - [`Enum`](#enum)
  - [`Extern`](#extern)
  - [`Final`](#final)
  - [`Fn`](#fn)
  - [`For`](#for)
  - [`If`](#if)
  - [`Impl`](#impl)
  - [`In`](#in)
  - [`Let`](#let)
  - [`Loop`](#loop)
  - [`Macro`](#macro)
  - [`Match`](#match)
  - [`Mod`](#mod)
  - [`Move`](#move)
  - [`Mut`](#mut)
  - [`Override`](#override)
  - [`Priv`](#priv)
  - [`Pub`](#pub)
  - [`Raw`](#raw)
  - [`Ref`](#ref)
  - [`Return`](#return)
  - [`SelfType`](#selftype)
  - [`SelfValue`](#selfvalue)
  - [`Static`](#static)
  - [`Struct`](#struct)
  - [`Super`](#super)
  - [`Trait`](#trait)
  - [`Try`](#try)
  - [`Type`](#type)
  - [`Typeof`](#typeof)
  - [`Union`](#union)
  - [`Unsafe`](#unsafe)
  - [`Unsized`](#unsized)
  - [`Use`](#use)
  - [`Virtual`](#virtual)
  - [`Where`](#where)
  - [`While`](#while)
  - [`Yield`](#yield)
  - [`And`](#and)
  - [`AndAnd`](#andand)
  - [`AndEq`](#andeq)
  - [`At`](#at)
  - [`Caret`](#caret)
  - [`CaretEq`](#careteq)
  - [`Colon`](#colon)
  - [`Comma`](#comma)
  - [`Dollar`](#dollar)
  - [`Dot`](#dot)
  - [`DotDot`](#dotdot)
  - [`DotDotDot`](#dotdotdot)
  - [`DotDotEq`](#dotdoteq)
  - [`Eq`](#eq)
  - [`EqEq`](#eqeq)
  - [`FatArrow`](#fatarrow)
  - [`Ge`](#ge)
  - [`Gt`](#gt)
  - [`LArrow`](#larrow)
  - [`Le`](#le)
  - [`Lt`](#lt)
  - [`Minus`](#minus)
  - [`MinusEq`](#minuseq)
  - [`Ne`](#ne)
  - [`Not`](#not)
  - [`Or`](#or)
  - [`OrEq`](#oreq)
  - [`OrOr`](#oror)
  - [`PathSep`](#pathsep)
  - [`Percent`](#percent)
  - [`PercentEq`](#percenteq)
  - [`Plus`](#plus)
  - [`PlusEq`](#pluseq)
  - [`Pound`](#pound)
  - [`Question`](#question)
  - [`RArrow`](#rarrow)
  - [`Semi`](#semi)
  - [`Shl`](#shl)
  - [`ShlEq`](#shleq)
  - [`Shr`](#shr)
  - [`ShrEq`](#shreq)
  - [`Slash`](#slash)
  - [`SlashEq`](#slasheq)
  - [`Star`](#star)
  - [`StarEq`](#stareq)
  - [`Tilde`](#tilde)
  - [`Brace`](#brace)
  - [`Bracket`](#bracket)
  - [`Paren`](#paren)
- [Traits](#traits)
  - [`Token`](#token)
- [Macros](#macros)
  - [`impl_low_level_token!`](#impl-low-level-token)
  - [`define_keywords!`](#define-keywords)
  - [`impl_deref_if_len_is_1!`](#impl-deref-if-len-is-1)
  - [`define_punctuation_structs!`](#define-punctuation-structs)
  - [`define_punctuation!`](#define-punctuation)
  - [`define_delimiters!`](#define-delimiters)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Underscore`](#underscore) | struct | `_` |
| [`Group`](#group) | struct | None-delimited group |
| [`Abstract`](#abstract) | struct | `abstract` |
| [`As`](#as) | struct | `as` |
| [`Async`](#async) | struct | `async` |
| [`Auto`](#auto) | struct | `auto` |
| [`Await`](#await) | struct | `await` |
| [`Become`](#become) | struct | `become` |
| [`Box`](#box) | struct | `box` |
| [`Break`](#break) | struct | `break` |
| [`Const`](#const) | struct | `const` |
| [`Continue`](#continue) | struct | `continue` |
| [`Crate`](#crate) | struct | `crate` |
| [`Default`](#default) | struct | `default` |
| [`Do`](#do) | struct | `do` |
| [`Dyn`](#dyn) | struct | `dyn` |
| [`Else`](#else) | struct | `else` |
| [`Enum`](#enum) | struct | `enum` |
| [`Extern`](#extern) | struct | `extern` |
| [`Final`](#final) | struct | `final` |
| [`Fn`](#fn) | struct | `fn` |
| [`For`](#for) | struct | `for` |
| [`If`](#if) | struct | `if` |
| [`Impl`](#impl) | struct | `impl` |
| [`In`](#in) | struct | `in` |
| [`Let`](#let) | struct | `let` |
| [`Loop`](#loop) | struct | `loop` |
| [`Macro`](#macro) | struct | `macro` |
| [`Match`](#match) | struct | `match` |
| [`Mod`](#mod) | struct | `mod` |
| [`Move`](#move) | struct | `move` |
| [`Mut`](#mut) | struct | `mut` |
| [`Override`](#override) | struct | `override` |
| [`Priv`](#priv) | struct | `priv` |
| [`Pub`](#pub) | struct | `pub` |
| [`Raw`](#raw) | struct | `raw` |
| [`Ref`](#ref) | struct | `ref` |
| [`Return`](#return) | struct | `return` |
| [`SelfType`](#selftype) | struct | `Self` |
| [`SelfValue`](#selfvalue) | struct | `self` |
| [`Static`](#static) | struct | `static` |
| [`Struct`](#struct) | struct | `struct` |
| [`Super`](#super) | struct | `super` |
| [`Trait`](#trait) | struct | `trait` |
| [`Try`](#try) | struct | `try` |
| [`Type`](#type) | struct | `type` |
| [`Typeof`](#typeof) | struct | `typeof` |
| [`Union`](#union) | struct | `union` |
| [`Unsafe`](#unsafe) | struct | `unsafe` |
| [`Unsized`](#unsized) | struct | `unsized` |
| [`Use`](#use) | struct | `use` |
| [`Virtual`](#virtual) | struct | `virtual` |
| [`Where`](#where) | struct | `where` |
| [`While`](#while) | struct | `while` |
| [`Yield`](#yield) | struct | `yield` |
| [`And`](#and) | struct | `&` |
| [`AndAnd`](#andand) | struct | `&&` |
| [`AndEq`](#andeq) | struct | `&=` |
| [`At`](#at) | struct | `@` |
| [`Caret`](#caret) | struct | `^` |
| [`CaretEq`](#careteq) | struct | `^=` |
| [`Colon`](#colon) | struct | `:` |
| [`Comma`](#comma) | struct | `,` |
| [`Dollar`](#dollar) | struct | `$` |
| [`Dot`](#dot) | struct | `.` |
| [`DotDot`](#dotdot) | struct | `..` |
| [`DotDotDot`](#dotdotdot) | struct | `...` |
| [`DotDotEq`](#dotdoteq) | struct | `..=` |
| [`Eq`](#eq) | struct | `=` |
| [`EqEq`](#eqeq) | struct | `==` |
| [`FatArrow`](#fatarrow) | struct | `=>` |
| [`Ge`](#ge) | struct | `>=` |
| [`Gt`](#gt) | struct | `>` |
| [`LArrow`](#larrow) | struct | `<-` |
| [`Le`](#le) | struct | `<=` |
| [`Lt`](#lt) | struct | `<` |
| [`Minus`](#minus) | struct | `-` |
| [`MinusEq`](#minuseq) | struct | `-=` |
| [`Ne`](#ne) | struct | `!=` |
| [`Not`](#not) | struct | `!` |
| [`Or`](#or) | struct | `\|` |
| [`OrEq`](#oreq) | struct | `\|=` |
| [`OrOr`](#oror) | struct | `\|\|` |
| [`PathSep`](#pathsep) | struct | `::` |
| [`Percent`](#percent) | struct | `%` |
| [`PercentEq`](#percenteq) | struct | `%=` |
| [`Plus`](#plus) | struct | `+` |
| [`PlusEq`](#pluseq) | struct | `+=` |
| [`Pound`](#pound) | struct | `#` |
| [`Question`](#question) | struct | `?` |
| [`RArrow`](#rarrow) | struct | `->` |
| [`Semi`](#semi) | struct | `;` |
| [`Shl`](#shl) | struct | `<<` |
| [`ShlEq`](#shleq) | struct | `<<=` |
| [`Shr`](#shr) | struct | `>>` |
| [`ShrEq`](#shreq) | struct | `>>=` |
| [`Slash`](#slash) | struct | `/` |
| [`SlashEq`](#slasheq) | struct | `/=` |
| [`Star`](#star) | struct | `*` |
| [`StarEq`](#stareq) | struct | `*=` |
| [`Tilde`](#tilde) | struct | `~` |
| [`Brace`](#brace) | struct | `{`&hellip;`}` |
| [`Bracket`](#bracket) | struct | `[`&hellip;`]` |
| [`Paren`](#paren) | struct | `(`&hellip;`)` |
| [`Token`](#token) | trait | Marker trait for types that represent single tokens. |
| [`impl_low_level_token!`](#impl-low-level-token) | macro |  |
| [`define_keywords!`](#define-keywords) | macro |  |
| [`impl_deref_if_len_is_1!`](#impl-deref-if-len-is-1) | macro |  |
| [`define_punctuation_structs!`](#define-punctuation-structs) | macro |  |
| [`define_punctuation!`](#define-punctuation) | macro |  |
| [`define_delimiters!`](#define-delimiters) | macro |  |

## Modules

- [`private`](private/index.md)

## Structs

### `Underscore`

```rust
struct Underscore {
    pub spans: [Span; 1],
}
```

`_`

Usage:
 wildcard patterns, inferred types, unnamed items in constants, extern crates, use declarations, and destructuring assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Underscore`

- <span id="underscore-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Underscore`

##### `impl Debug for Underscore`

- <span id="underscore-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Underscore`

- <span id="underscore-default"></span>`fn default() -> Self`

##### `impl Deref for Underscore`

- <span id="underscore-deref-type-target"></span>`type Target = WithSpan`

- <span id="underscore-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Underscore`

- <span id="underscore-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Underscore`

##### `impl Hash for Underscore`

- <span id="underscore-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Underscore`

- <span id="underscore-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Underscore`

- <span id="underscore-partialeq-eq"></span>`fn eq(&self, _other: &Underscore) -> bool` — [`Underscore`](#underscore)

##### `impl Receiver for Underscore`

- <span id="underscore-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Underscore`

##### `impl Spanned for Underscore`

- <span id="underscore-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Underscore`

- <span id="underscore-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Underscore`

### `Group`

```rust
struct Group {
    pub span: proc_macro2::Span,
}
```

None-delimited group

#### Implementations

- <span id="group-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Group`

##### `impl Debug for Group`

- <span id="group-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Group`

- <span id="group-default"></span>`fn default() -> Self`

##### `impl Eq for Group`

##### `impl Hash for Group`

- <span id="group-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl PartialEq for Group`

- <span id="group-partialeq-eq"></span>`fn eq(&self, _other: &Group) -> bool` — [`Group`](#group)

##### `impl Sealed for Group`

##### `impl Token for Group`

### `Abstract`

```rust
struct Abstract {
    pub span: Span,
}
```

`abstract`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Abstract`

- <span id="abstract-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Abstract`

##### `impl Debug for Abstract`

- <span id="abstract-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abstract`

- <span id="abstract-default"></span>`fn default() -> Self`

##### `impl Eq for Abstract`

##### `impl Hash for Abstract`

- <span id="abstract-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Abstract`

- <span id="abstract-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Abstract`

- <span id="abstract-partialeq-eq"></span>`fn eq(&self, _other: &Abstract) -> bool` — [`Abstract`](#abstract)

##### `impl Sealed for Abstract`

##### `impl Spanned for Abstract`

- <span id="abstract-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Abstract`

- <span id="abstract-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Abstract`

### `As`

```rust
struct As {
    pub span: Span,
}
```

`as`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for As`

- <span id="as-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for As`

##### `impl Debug for As`

- <span id="as-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for As`

- <span id="as-default"></span>`fn default() -> Self`

##### `impl Eq for As`

##### `impl Hash for As`

- <span id="as-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for As`

- <span id="as-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for As`

- <span id="as-partialeq-eq"></span>`fn eq(&self, _other: &As) -> bool` — [`As`](#as)

##### `impl Sealed for As`

##### `impl Spanned for As`

- <span id="as-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for As`

- <span id="as-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for As`

### `Async`

```rust
struct Async {
    pub span: Span,
}
```

`async`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Async`

- <span id="async-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Async`

##### `impl Debug for Async`

- <span id="async-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Async`

- <span id="async-default"></span>`fn default() -> Self`

##### `impl Eq for Async`

##### `impl Hash for Async`

- <span id="async-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Async`

- <span id="async-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Async`

- <span id="async-partialeq-eq"></span>`fn eq(&self, _other: &Async) -> bool` — [`Async`](#async)

##### `impl Sealed for Async`

##### `impl Spanned for Async`

- <span id="async-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Async`

- <span id="async-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Async`

### `Auto`

```rust
struct Auto {
    pub span: Span,
}
```

`auto`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Auto`

- <span id="auto-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Auto`

##### `impl Debug for Auto`

- <span id="auto-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Auto`

- <span id="auto-default"></span>`fn default() -> Self`

##### `impl Eq for Auto`

##### `impl Hash for Auto`

- <span id="auto-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Auto`

- <span id="auto-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Auto`

- <span id="auto-partialeq-eq"></span>`fn eq(&self, _other: &Auto) -> bool` — [`Auto`](#auto)

##### `impl Sealed for Auto`

##### `impl Spanned for Auto`

- <span id="auto-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Auto`

- <span id="auto-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Auto`

### `Await`

```rust
struct Await {
    pub span: Span,
}
```

`await`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Await`

- <span id="await-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Await`

##### `impl Debug for Await`

- <span id="await-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Await`

- <span id="await-default"></span>`fn default() -> Self`

##### `impl Eq for Await`

##### `impl Hash for Await`

- <span id="await-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Await`

- <span id="await-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Await`

- <span id="await-partialeq-eq"></span>`fn eq(&self, _other: &Await) -> bool` — [`Await`](#await)

##### `impl Sealed for Await`

##### `impl Spanned for Await`

- <span id="await-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Await`

- <span id="await-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Await`

### `Become`

```rust
struct Become {
    pub span: Span,
}
```

`become`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Become`

- <span id="become-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Become`

##### `impl Debug for Become`

- <span id="become-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Become`

- <span id="become-default"></span>`fn default() -> Self`

##### `impl Eq for Become`

##### `impl Hash for Become`

- <span id="become-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Become`

- <span id="become-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Become`

- <span id="become-partialeq-eq"></span>`fn eq(&self, _other: &Become) -> bool` — [`Become`](#become)

##### `impl Sealed for Become`

##### `impl Spanned for Become`

- <span id="become-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Become`

- <span id="become-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Become`

### `Box`

```rust
struct Box {
    pub span: Span,
}
```

`box`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Box`

- <span id="box-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Box`

##### `impl Debug for Box`

- <span id="box-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Box`

- <span id="box-default"></span>`fn default() -> Self`

##### `impl Eq for Box`

##### `impl Hash for Box`

- <span id="box-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Box`

- <span id="box-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Box`

- <span id="box-partialeq-eq"></span>`fn eq(&self, _other: &Box) -> bool` — [`Box`](#box)

##### `impl Sealed for Box`

##### `impl Spanned for Box`

- <span id="box-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Box`

- <span id="box-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Box`

### `Break`

```rust
struct Break {
    pub span: Span,
}
```

`break`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Break`

- <span id="break-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Break`

##### `impl Debug for Break`

- <span id="break-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Break`

- <span id="break-default"></span>`fn default() -> Self`

##### `impl Eq for Break`

##### `impl Hash for Break`

- <span id="break-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Break`

- <span id="break-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Break`

- <span id="break-partialeq-eq"></span>`fn eq(&self, _other: &Break) -> bool` — [`Break`](#break)

##### `impl Sealed for Break`

##### `impl Spanned for Break`

- <span id="break-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Break`

- <span id="break-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Break`

### `Const`

```rust
struct Const {
    pub span: Span,
}
```

`const`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Const`

- <span id="const-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Const`

##### `impl Debug for Const`

- <span id="const-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Const`

- <span id="const-default"></span>`fn default() -> Self`

##### `impl Eq for Const`

##### `impl Hash for Const`

- <span id="const-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Const`

- <span id="const-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Const`

- <span id="const-partialeq-eq"></span>`fn eq(&self, _other: &Const) -> bool` — [`Const`](#const)

##### `impl Sealed for Const`

##### `impl Spanned for Const`

- <span id="const-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Const`

- <span id="const-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Const`

### `Continue`

```rust
struct Continue {
    pub span: Span,
}
```

`continue`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Continue`

- <span id="continue-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Continue`

##### `impl Debug for Continue`

- <span id="continue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Continue`

- <span id="continue-default"></span>`fn default() -> Self`

##### `impl Eq for Continue`

##### `impl Hash for Continue`

- <span id="continue-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Continue`

- <span id="continue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Continue`

- <span id="continue-partialeq-eq"></span>`fn eq(&self, _other: &Continue) -> bool` — [`Continue`](#continue)

##### `impl Sealed for Continue`

##### `impl Spanned for Continue`

- <span id="continue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Continue`

- <span id="continue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Continue`

### `Crate`

```rust
struct Crate {
    pub span: Span,
}
```

`crate`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Crate`

- <span id="crate-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Crate`

##### `impl Debug for Crate`

- <span id="crate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Crate`

- <span id="crate-default"></span>`fn default() -> Self`

##### `impl Eq for Crate`

##### `impl Hash for Crate`

- <span id="crate-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Crate`

- <span id="crate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Crate`

- <span id="crate-partialeq-eq"></span>`fn eq(&self, _other: &Crate) -> bool` — [`Crate`](#crate)

##### `impl Sealed for Crate`

##### `impl Spanned for Crate`

- <span id="crate-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Crate`

- <span id="crate-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Crate`

### `Default`

```rust
struct Default {
    pub span: Span,
}
```

`default`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Default`

- <span id="default-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Default`

##### `impl Debug for Default`

- <span id="default-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Default`

- <span id="default-default"></span>`fn default() -> Self`

##### `impl Eq for Default`

##### `impl Hash for Default`

- <span id="default-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Default`

- <span id="default-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Default`

- <span id="default-partialeq-eq"></span>`fn eq(&self, _other: &Default) -> bool` — [`Default`](#default)

##### `impl Sealed for Default`

##### `impl Spanned for Default`

- <span id="default-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Default`

- <span id="default-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Default`

### `Do`

```rust
struct Do {
    pub span: Span,
}
```

`do`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Do`

- <span id="do-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Do`

##### `impl Debug for Do`

- <span id="do-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Do`

- <span id="do-default"></span>`fn default() -> Self`

##### `impl Eq for Do`

##### `impl Hash for Do`

- <span id="do-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Do`

- <span id="do-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Do`

- <span id="do-partialeq-eq"></span>`fn eq(&self, _other: &Do) -> bool` — [`Do`](#do)

##### `impl Sealed for Do`

##### `impl Spanned for Do`

- <span id="do-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Do`

- <span id="do-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Do`

### `Dyn`

```rust
struct Dyn {
    pub span: Span,
}
```

`dyn`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Dyn`

- <span id="dyn-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Dyn`

##### `impl Debug for Dyn`

- <span id="dyn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dyn`

- <span id="dyn-default"></span>`fn default() -> Self`

##### `impl Eq for Dyn`

##### `impl Hash for Dyn`

- <span id="dyn-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Dyn`

- <span id="dyn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Dyn`

- <span id="dyn-partialeq-eq"></span>`fn eq(&self, _other: &Dyn) -> bool` — [`Dyn`](#dyn)

##### `impl Sealed for Dyn`

##### `impl Spanned for Dyn`

- <span id="dyn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Dyn`

- <span id="dyn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Dyn`

### `Else`

```rust
struct Else {
    pub span: Span,
}
```

`else`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Else`

- <span id="else-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Else`

##### `impl Debug for Else`

- <span id="else-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Else`

- <span id="else-default"></span>`fn default() -> Self`

##### `impl Eq for Else`

##### `impl Hash for Else`

- <span id="else-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Else`

- <span id="else-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Else`

- <span id="else-partialeq-eq"></span>`fn eq(&self, _other: &Else) -> bool` — [`Else`](#else)

##### `impl Sealed for Else`

##### `impl Spanned for Else`

- <span id="else-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Else`

- <span id="else-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Else`

### `Enum`

```rust
struct Enum {
    pub span: Span,
}
```

`enum`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Enum`

- <span id="enum-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Enum`

##### `impl Debug for Enum`

- <span id="enum-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Enum`

- <span id="enum-default"></span>`fn default() -> Self`

##### `impl Eq for Enum`

##### `impl Hash for Enum`

- <span id="enum-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Enum`

- <span id="enum-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Enum`

- <span id="enum-partialeq-eq"></span>`fn eq(&self, _other: &Enum) -> bool` — [`Enum`](#enum)

##### `impl Sealed for Enum`

##### `impl Spanned for Enum`

- <span id="enum-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Enum`

- <span id="enum-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Enum`

### `Extern`

```rust
struct Extern {
    pub span: Span,
}
```

`extern`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Extern`

- <span id="extern-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Extern`

##### `impl Debug for Extern`

- <span id="extern-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Extern`

- <span id="extern-default"></span>`fn default() -> Self`

##### `impl Eq for Extern`

##### `impl Hash for Extern`

- <span id="extern-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Extern`

- <span id="extern-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Extern`

- <span id="extern-partialeq-eq"></span>`fn eq(&self, _other: &Extern) -> bool` — [`Extern`](#extern)

##### `impl Sealed for Extern`

##### `impl Spanned for Extern`

- <span id="extern-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Extern`

- <span id="extern-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Extern`

### `Final`

```rust
struct Final {
    pub span: Span,
}
```

`final`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Final`

- <span id="final-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Final`

##### `impl Debug for Final`

- <span id="final-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Final`

- <span id="final-default"></span>`fn default() -> Self`

##### `impl Eq for Final`

##### `impl Hash for Final`

- <span id="final-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Final`

- <span id="final-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Final`

- <span id="final-partialeq-eq"></span>`fn eq(&self, _other: &Final) -> bool` — [`Final`](#final)

##### `impl Sealed for Final`

##### `impl Spanned for Final`

- <span id="final-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Final`

- <span id="final-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Final`

### `Fn`

```rust
struct Fn {
    pub span: Span,
}
```

`fn`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Fn`

- <span id="fn-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Fn`

##### `impl Debug for Fn`

- <span id="fn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Fn`

- <span id="fn-default"></span>`fn default() -> Self`

##### `impl Eq for Fn`

##### `impl Hash for Fn`

- <span id="fn-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Fn`

- <span id="fn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Fn`

- <span id="fn-partialeq-eq"></span>`fn eq(&self, _other: &Fn) -> bool` — [`Fn`](#fn)

##### `impl Sealed for Fn`

##### `impl Spanned for Fn`

- <span id="fn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Fn`

- <span id="fn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Fn`

### `For`

```rust
struct For {
    pub span: Span,
}
```

`for`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for For`

- <span id="for-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for For`

##### `impl Debug for For`

- <span id="for-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for For`

- <span id="for-default"></span>`fn default() -> Self`

##### `impl Eq for For`

##### `impl Hash for For`

- <span id="for-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for For`

- <span id="for-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for For`

- <span id="for-partialeq-eq"></span>`fn eq(&self, _other: &For) -> bool` — [`For`](#for)

##### `impl Sealed for For`

##### `impl Spanned for For`

- <span id="for-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for For`

- <span id="for-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for For`

### `If`

```rust
struct If {
    pub span: Span,
}
```

`if`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for If`

- <span id="if-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for If`

##### `impl Debug for If`

- <span id="if-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for If`

- <span id="if-default"></span>`fn default() -> Self`

##### `impl Eq for If`

##### `impl Hash for If`

- <span id="if-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for If`

- <span id="if-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for If`

- <span id="if-partialeq-eq"></span>`fn eq(&self, _other: &If) -> bool` — [`If`](#if)

##### `impl Sealed for If`

##### `impl Spanned for If`

- <span id="if-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for If`

- <span id="if-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for If`

### `Impl`

```rust
struct Impl {
    pub span: Span,
}
```

`impl`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Impl`

- <span id="impl-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Impl`

##### `impl Debug for Impl`

- <span id="impl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Impl`

- <span id="impl-default"></span>`fn default() -> Self`

##### `impl Eq for Impl`

##### `impl Hash for Impl`

- <span id="impl-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Impl`

- <span id="impl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Impl`

- <span id="impl-partialeq-eq"></span>`fn eq(&self, _other: &Impl) -> bool` — [`Impl`](#impl)

##### `impl Sealed for Impl`

##### `impl Spanned for Impl`

- <span id="impl-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Impl`

- <span id="impl-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Impl`

### `In`

```rust
struct In {
    pub span: Span,
}
```

`in`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for In`

- <span id="in-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for In`

##### `impl Debug for In`

- <span id="in-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for In`

- <span id="in-default"></span>`fn default() -> Self`

##### `impl Eq for In`

##### `impl Hash for In`

- <span id="in-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for In`

- <span id="in-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for In`

- <span id="in-partialeq-eq"></span>`fn eq(&self, _other: &In) -> bool` — [`In`](#in)

##### `impl Sealed for In`

##### `impl Spanned for In`

- <span id="in-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for In`

- <span id="in-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for In`

### `Let`

```rust
struct Let {
    pub span: Span,
}
```

`let`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Let`

- <span id="let-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Let`

##### `impl Debug for Let`

- <span id="let-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Let`

- <span id="let-default"></span>`fn default() -> Self`

##### `impl Eq for Let`

##### `impl Hash for Let`

- <span id="let-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Let`

- <span id="let-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Let`

- <span id="let-partialeq-eq"></span>`fn eq(&self, _other: &Let) -> bool` — [`Let`](#let)

##### `impl Sealed for Let`

##### `impl Spanned for Let`

- <span id="let-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Let`

- <span id="let-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Let`

### `Loop`

```rust
struct Loop {
    pub span: Span,
}
```

`loop`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Loop`

- <span id="loop-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Loop`

##### `impl Debug for Loop`

- <span id="loop-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Loop`

- <span id="loop-default"></span>`fn default() -> Self`

##### `impl Eq for Loop`

##### `impl Hash for Loop`

- <span id="loop-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Loop`

- <span id="loop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Loop`

- <span id="loop-partialeq-eq"></span>`fn eq(&self, _other: &Loop) -> bool` — [`Loop`](#loop)

##### `impl Sealed for Loop`

##### `impl Spanned for Loop`

- <span id="loop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Loop`

- <span id="loop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Loop`

### `Macro`

```rust
struct Macro {
    pub span: Span,
}
```

`macro`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Macro`

- <span id="macro-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Macro`

##### `impl Debug for Macro`

- <span id="macro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Macro`

- <span id="macro-default"></span>`fn default() -> Self`

##### `impl Eq for Macro`

##### `impl Hash for Macro`

- <span id="macro-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Macro`

- <span id="macro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Macro`

- <span id="macro-partialeq-eq"></span>`fn eq(&self, _other: &Macro) -> bool` — [`Macro`](#macro)

##### `impl Sealed for Macro`

##### `impl Spanned for Macro`

- <span id="macro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Macro`

- <span id="macro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Macro`

### `Match`

```rust
struct Match {
    pub span: Span,
}
```

`match`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Match`

- <span id="match-default"></span>`fn default() -> Self`

##### `impl Eq for Match`

##### `impl Hash for Match`

- <span id="match-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Match`

- <span id="match-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Match`

- <span id="match-partialeq-eq"></span>`fn eq(&self, _other: &Match) -> bool` — [`Match`](#match)

##### `impl Sealed for Match`

##### `impl Spanned for Match`

- <span id="match-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Match`

- <span id="match-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Match`

### `Mod`

```rust
struct Mod {
    pub span: Span,
}
```

`mod`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Mod`

- <span id="mod-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Mod`

##### `impl Debug for Mod`

- <span id="mod-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Mod`

- <span id="mod-default"></span>`fn default() -> Self`

##### `impl Eq for Mod`

##### `impl Hash for Mod`

- <span id="mod-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Mod`

- <span id="mod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Mod`

- <span id="mod-partialeq-eq"></span>`fn eq(&self, _other: &Mod) -> bool` — [`Mod`](#mod)

##### `impl Sealed for Mod`

##### `impl Spanned for Mod`

- <span id="mod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Mod`

- <span id="mod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Mod`

### `Move`

```rust
struct Move {
    pub span: Span,
}
```

`move`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Move`

- <span id="move-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Move`

##### `impl Debug for Move`

- <span id="move-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Move`

- <span id="move-default"></span>`fn default() -> Self`

##### `impl Eq for Move`

##### `impl Hash for Move`

- <span id="move-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Move`

- <span id="move-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Move`

- <span id="move-partialeq-eq"></span>`fn eq(&self, _other: &Move) -> bool` — [`Move`](#move)

##### `impl Sealed for Move`

##### `impl Spanned for Move`

- <span id="move-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Move`

- <span id="move-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Move`

### `Mut`

```rust
struct Mut {
    pub span: Span,
}
```

`mut`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Mut`

- <span id="mut-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Mut`

##### `impl Debug for Mut`

- <span id="mut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Mut`

- <span id="mut-default"></span>`fn default() -> Self`

##### `impl Eq for Mut`

##### `impl Hash for Mut`

- <span id="mut-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Mut`

- <span id="mut-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Mut`

- <span id="mut-partialeq-eq"></span>`fn eq(&self, _other: &Mut) -> bool` — [`Mut`](#mut)

##### `impl Sealed for Mut`

##### `impl Spanned for Mut`

- <span id="mut-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Mut`

- <span id="mut-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Mut`

### `Override`

```rust
struct Override {
    pub span: Span,
}
```

`override`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Override`

- <span id="override-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Override`

##### `impl Debug for Override`

- <span id="override-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Override`

- <span id="override-default"></span>`fn default() -> Self`

##### `impl Eq for Override`

##### `impl Hash for Override`

- <span id="override-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Override`

- <span id="override-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Override`

- <span id="override-partialeq-eq"></span>`fn eq(&self, _other: &Override) -> bool` — [`Override`](#override)

##### `impl Sealed for Override`

##### `impl Spanned for Override`

- <span id="override-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Override`

- <span id="override-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Override`

### `Priv`

```rust
struct Priv {
    pub span: Span,
}
```

`priv`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Priv`

- <span id="priv-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Priv`

##### `impl Debug for Priv`

- <span id="priv-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Priv`

- <span id="priv-default"></span>`fn default() -> Self`

##### `impl Eq for Priv`

##### `impl Hash for Priv`

- <span id="priv-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Priv`

- <span id="priv-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Priv`

- <span id="priv-partialeq-eq"></span>`fn eq(&self, _other: &Priv) -> bool` — [`Priv`](#priv)

##### `impl Sealed for Priv`

##### `impl Spanned for Priv`

- <span id="priv-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Priv`

- <span id="priv-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Priv`

### `Pub`

```rust
struct Pub {
    pub span: Span,
}
```

`pub`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Pub`

- <span id="pub-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Pub`

##### `impl Debug for Pub`

- <span id="pub-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pub`

- <span id="pub-default"></span>`fn default() -> Self`

##### `impl Eq for Pub`

##### `impl Hash for Pub`

- <span id="pub-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Pub`

- <span id="pub-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Pub`

- <span id="pub-partialeq-eq"></span>`fn eq(&self, _other: &Pub) -> bool` — [`Pub`](#pub)

##### `impl Sealed for Pub`

##### `impl Spanned for Pub`

- <span id="pub-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Pub`

- <span id="pub-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Pub`

### `Raw`

```rust
struct Raw {
    pub span: Span,
}
```

`raw`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Raw`

- <span id="raw-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Raw`

##### `impl Debug for Raw`

- <span id="raw-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Raw`

- <span id="raw-default"></span>`fn default() -> Self`

##### `impl Eq for Raw`

##### `impl Hash for Raw`

- <span id="raw-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Raw`

- <span id="raw-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Raw`

- <span id="raw-partialeq-eq"></span>`fn eq(&self, _other: &Raw) -> bool` — [`Raw`](#raw)

##### `impl Sealed for Raw`

##### `impl Spanned for Raw`

- <span id="raw-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Raw`

- <span id="raw-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Raw`

### `Ref`

```rust
struct Ref {
    pub span: Span,
}
```

`ref`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Ref`

- <span id="ref-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Ref`

##### `impl Debug for Ref`

- <span id="ref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ref`

- <span id="ref-default"></span>`fn default() -> Self`

##### `impl Eq for Ref`

##### `impl Hash for Ref`

- <span id="ref-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Ref`

- <span id="ref-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Ref`

- <span id="ref-partialeq-eq"></span>`fn eq(&self, _other: &Ref) -> bool` — [`Ref`](#ref)

##### `impl Sealed for Ref`

##### `impl Spanned for Ref`

- <span id="ref-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Ref`

- <span id="ref-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Ref`

### `Return`

```rust
struct Return {
    pub span: Span,
}
```

`return`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Return`

- <span id="return-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Return`

##### `impl Debug for Return`

- <span id="return-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Return`

- <span id="return-default"></span>`fn default() -> Self`

##### `impl Eq for Return`

##### `impl Hash for Return`

- <span id="return-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Return`

- <span id="return-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Return`

- <span id="return-partialeq-eq"></span>`fn eq(&self, _other: &Return) -> bool` — [`Return`](#return)

##### `impl Sealed for Return`

##### `impl Spanned for Return`

- <span id="return-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Return`

- <span id="return-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Return`

### `SelfType`

```rust
struct SelfType {
    pub span: Span,
}
```

`Self`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for SelfType`

- <span id="selftype-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for SelfType`

##### `impl Debug for SelfType`

- <span id="selftype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SelfType`

- <span id="selftype-default"></span>`fn default() -> Self`

##### `impl Eq for SelfType`

##### `impl Hash for SelfType`

- <span id="selftype-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for SelfType`

- <span id="selftype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for SelfType`

- <span id="selftype-partialeq-eq"></span>`fn eq(&self, _other: &SelfType) -> bool` — [`SelfType`](#selftype)

##### `impl Sealed for SelfType`

##### `impl Spanned for SelfType`

- <span id="selftype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for SelfType`

- <span id="selftype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for SelfType`

### `SelfValue`

```rust
struct SelfValue {
    pub span: Span,
}
```

`self`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for SelfValue`

- <span id="selfvalue-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for SelfValue`

##### `impl Debug for SelfValue`

- <span id="selfvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SelfValue`

- <span id="selfvalue-default"></span>`fn default() -> Self`

##### `impl Eq for SelfValue`

##### `impl Hash for SelfValue`

- <span id="selfvalue-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for SelfValue`

- <span id="selfvalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for SelfValue`

- <span id="selfvalue-partialeq-eq"></span>`fn eq(&self, _other: &SelfValue) -> bool` — [`SelfValue`](#selfvalue)

##### `impl Sealed for SelfValue`

##### `impl Spanned for SelfValue`

- <span id="selfvalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for SelfValue`

- <span id="selfvalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for SelfValue`

### `Static`

```rust
struct Static {
    pub span: Span,
}
```

`static`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Static`

- <span id="static-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Static`

##### `impl Debug for Static`

- <span id="static-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Static`

- <span id="static-default"></span>`fn default() -> Self`

##### `impl Eq for Static`

##### `impl Hash for Static`

- <span id="static-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Static`

- <span id="static-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Static`

- <span id="static-partialeq-eq"></span>`fn eq(&self, _other: &Static) -> bool` — [`Static`](#static)

##### `impl Sealed for Static`

##### `impl Spanned for Static`

- <span id="static-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Static`

- <span id="static-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Static`

### `Struct`

```rust
struct Struct {
    pub span: Span,
}
```

`struct`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Struct`

- <span id="struct-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Struct`

##### `impl Debug for Struct`

- <span id="struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Struct`

- <span id="struct-default"></span>`fn default() -> Self`

##### `impl Eq for Struct`

##### `impl Hash for Struct`

- <span id="struct-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Struct`

- <span id="struct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Struct`

- <span id="struct-partialeq-eq"></span>`fn eq(&self, _other: &Struct) -> bool` — [`Struct`](#struct)

##### `impl Sealed for Struct`

##### `impl Spanned for Struct`

- <span id="struct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Struct`

- <span id="struct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Struct`

### `Super`

```rust
struct Super {
    pub span: Span,
}
```

`super`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Super`

- <span id="super-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Super`

##### `impl Debug for Super`

- <span id="super-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Super`

- <span id="super-default"></span>`fn default() -> Self`

##### `impl Eq for Super`

##### `impl Hash for Super`

- <span id="super-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Super`

- <span id="super-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Super`

- <span id="super-partialeq-eq"></span>`fn eq(&self, _other: &Super) -> bool` — [`Super`](#super)

##### `impl Sealed for Super`

##### `impl Spanned for Super`

- <span id="super-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Super`

- <span id="super-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Super`

### `Trait`

```rust
struct Trait {
    pub span: Span,
}
```

`trait`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Trait`

- <span id="trait-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Trait`

##### `impl Debug for Trait`

- <span id="trait-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Trait`

- <span id="trait-default"></span>`fn default() -> Self`

##### `impl Eq for Trait`

##### `impl Hash for Trait`

- <span id="trait-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Trait`

- <span id="trait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Trait`

- <span id="trait-partialeq-eq"></span>`fn eq(&self, _other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl Sealed for Trait`

##### `impl Spanned for Trait`

- <span id="trait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Trait`

- <span id="trait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Trait`

### `Try`

```rust
struct Try {
    pub span: Span,
}
```

`try`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Try`

- <span id="try-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Try`

##### `impl Debug for Try`

- <span id="try-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Try`

- <span id="try-default"></span>`fn default() -> Self`

##### `impl Eq for Try`

##### `impl Hash for Try`

- <span id="try-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Try`

- <span id="try-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Try`

- <span id="try-partialeq-eq"></span>`fn eq(&self, _other: &Try) -> bool` — [`Try`](#try)

##### `impl Sealed for Try`

##### `impl Spanned for Try`

- <span id="try-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Try`

- <span id="try-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Try`

### `Type`

```rust
struct Type {
    pub span: Span,
}
```

`type`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Type`

- <span id="type-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Type`

##### `impl Debug for Type`

- <span id="type-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Type`

- <span id="type-default"></span>`fn default() -> Self`

##### `impl Eq for Type`

##### `impl Hash for Type`

- <span id="type-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Type`

- <span id="type-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Type`

- <span id="type-partialeq-eq"></span>`fn eq(&self, _other: &Type) -> bool` — [`Type`](#type)

##### `impl Sealed for Type`

##### `impl Spanned for Type`

- <span id="type-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Type`

- <span id="type-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Type`

### `Typeof`

```rust
struct Typeof {
    pub span: Span,
}
```

`typeof`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Typeof`

- <span id="typeof-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Typeof`

##### `impl Debug for Typeof`

- <span id="typeof-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Typeof`

- <span id="typeof-default"></span>`fn default() -> Self`

##### `impl Eq for Typeof`

##### `impl Hash for Typeof`

- <span id="typeof-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Typeof`

- <span id="typeof-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Typeof`

- <span id="typeof-partialeq-eq"></span>`fn eq(&self, _other: &Typeof) -> bool` — [`Typeof`](#typeof)

##### `impl Sealed for Typeof`

##### `impl Spanned for Typeof`

- <span id="typeof-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Typeof`

- <span id="typeof-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Typeof`

### `Union`

```rust
struct Union {
    pub span: Span,
}
```

`union`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Union`

- <span id="union-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Union`

##### `impl Debug for Union`

- <span id="union-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Union`

- <span id="union-default"></span>`fn default() -> Self`

##### `impl Eq for Union`

##### `impl Hash for Union`

- <span id="union-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Union`

- <span id="union-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Union`

- <span id="union-partialeq-eq"></span>`fn eq(&self, _other: &Union) -> bool` — [`Union`](#union)

##### `impl Sealed for Union`

##### `impl Spanned for Union`

- <span id="union-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Union`

- <span id="union-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Union`

### `Unsafe`

```rust
struct Unsafe {
    pub span: Span,
}
```

`unsafe`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Unsafe`

- <span id="unsafe-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Unsafe`

##### `impl Debug for Unsafe`

- <span id="unsafe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Unsafe`

- <span id="unsafe-default"></span>`fn default() -> Self`

##### `impl Eq for Unsafe`

##### `impl Hash for Unsafe`

- <span id="unsafe-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Unsafe`

- <span id="unsafe-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Unsafe`

- <span id="unsafe-partialeq-eq"></span>`fn eq(&self, _other: &Unsafe) -> bool` — [`Unsafe`](#unsafe)

##### `impl Sealed for Unsafe`

##### `impl Spanned for Unsafe`

- <span id="unsafe-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Unsafe`

- <span id="unsafe-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Unsafe`

### `Unsized`

```rust
struct Unsized {
    pub span: Span,
}
```

`unsized`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Unsized`

- <span id="unsized-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Unsized`

##### `impl Debug for Unsized`

- <span id="unsized-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Unsized`

- <span id="unsized-default"></span>`fn default() -> Self`

##### `impl Eq for Unsized`

##### `impl Hash for Unsized`

- <span id="unsized-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Unsized`

- <span id="unsized-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Unsized`

- <span id="unsized-partialeq-eq"></span>`fn eq(&self, _other: &Unsized) -> bool` — [`Unsized`](#unsized)

##### `impl Sealed for Unsized`

##### `impl Spanned for Unsized`

- <span id="unsized-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Unsized`

- <span id="unsized-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Unsized`

### `Use`

```rust
struct Use {
    pub span: Span,
}
```

`use`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Use`

- <span id="use-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Use`

##### `impl Debug for Use`

- <span id="use-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Use`

- <span id="use-default"></span>`fn default() -> Self`

##### `impl Eq for Use`

##### `impl Hash for Use`

- <span id="use-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Use`

- <span id="use-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Use`

- <span id="use-partialeq-eq"></span>`fn eq(&self, _other: &Use) -> bool` — [`Use`](#use)

##### `impl Sealed for Use`

##### `impl Spanned for Use`

- <span id="use-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Use`

- <span id="use-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Use`

### `Virtual`

```rust
struct Virtual {
    pub span: Span,
}
```

`virtual`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Virtual`

- <span id="virtual-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Virtual`

##### `impl Debug for Virtual`

- <span id="virtual-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Virtual`

- <span id="virtual-default"></span>`fn default() -> Self`

##### `impl Eq for Virtual`

##### `impl Hash for Virtual`

- <span id="virtual-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Virtual`

- <span id="virtual-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Virtual`

- <span id="virtual-partialeq-eq"></span>`fn eq(&self, _other: &Virtual) -> bool` — [`Virtual`](#virtual)

##### `impl Sealed for Virtual`

##### `impl Spanned for Virtual`

- <span id="virtual-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Virtual`

- <span id="virtual-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Virtual`

### `Where`

```rust
struct Where {
    pub span: Span,
}
```

`where`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Where`

- <span id="where-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Where`

##### `impl Debug for Where`

- <span id="where-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Where`

- <span id="where-default"></span>`fn default() -> Self`

##### `impl Eq for Where`

##### `impl Hash for Where`

- <span id="where-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Where`

- <span id="where-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Where`

- <span id="where-partialeq-eq"></span>`fn eq(&self, _other: &Where) -> bool` — [`Where`](#where)

##### `impl Sealed for Where`

##### `impl Spanned for Where`

- <span id="where-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Where`

- <span id="where-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Where`

### `While`

```rust
struct While {
    pub span: Span,
}
```

`while`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for While`

- <span id="while-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for While`

##### `impl Debug for While`

- <span id="while-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for While`

- <span id="while-default"></span>`fn default() -> Self`

##### `impl Eq for While`

##### `impl Hash for While`

- <span id="while-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for While`

- <span id="while-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for While`

- <span id="while-partialeq-eq"></span>`fn eq(&self, _other: &While) -> bool` — [`While`](#while)

##### `impl Sealed for While`

##### `impl Spanned for While`

- <span id="while-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for While`

- <span id="while-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for While`

### `Yield`

```rust
struct Yield {
    pub span: Span,
}
```

`yield`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Yield`

- <span id="yield-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- <span id="yield-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Yield`

- <span id="yield-default"></span>`fn default() -> Self`

##### `impl Eq for Yield`

##### `impl Hash for Yield`

- <span id="yield-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Yield`

- <span id="yield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Yield`

- <span id="yield-partialeq-eq"></span>`fn eq(&self, _other: &Yield) -> bool` — [`Yield`](#yield)

##### `impl Sealed for Yield`

##### `impl Spanned for Yield`

- <span id="yield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Yield`

- <span id="yield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Yield`

### `And`

```rust
struct And {
    pub spans: [Span; 1],
}
```

`&`

Usage:
 bitwise and logical AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for And`

- <span id="and-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for And`

##### `impl Debug for And`

- <span id="and-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for And`

- <span id="and-default"></span>`fn default() -> Self`

##### `impl Deref for And`

- <span id="and-deref-type-target"></span>`type Target = WithSpan`

- <span id="and-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for And`

- <span id="and-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for And`

##### `impl Hash for And`

- <span id="and-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for And`

- <span id="and-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for And`

- <span id="and-partialeq-eq"></span>`fn eq(&self, _other: &And) -> bool` — [`And`](#and)

##### `impl Receiver for And`

- <span id="and-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for And`

##### `impl Spanned for And`

- <span id="and-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for And`

- <span id="and-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for And`

### `AndAnd`

```rust
struct AndAnd {
    pub spans: [Span; 2],
}
```

`&&`

Usage:
 lazy AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for AndAnd`

- <span id="andand-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for AndAnd`

##### `impl Debug for AndAnd`

- <span id="andand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AndAnd`

- <span id="andand-default"></span>`fn default() -> Self`

##### `impl Eq for AndAnd`

##### `impl Hash for AndAnd`

- <span id="andand-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for AndAnd`

- <span id="andand-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for AndAnd`

- <span id="andand-partialeq-eq"></span>`fn eq(&self, _other: &AndAnd) -> bool` — [`AndAnd`](#andand)

##### `impl Sealed for AndAnd`

##### `impl Spanned for AndAnd`

- <span id="andand-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for AndAnd`

- <span id="andand-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for AndAnd`

### `AndEq`

```rust
struct AndEq {
    pub spans: [Span; 2],
}
```

`&=`

Usage:
 bitwise AND assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for AndEq`

- <span id="andeq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for AndEq`

##### `impl Debug for AndEq`

- <span id="andeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AndEq`

- <span id="andeq-default"></span>`fn default() -> Self`

##### `impl Eq for AndEq`

##### `impl Hash for AndEq`

- <span id="andeq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for AndEq`

- <span id="andeq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for AndEq`

- <span id="andeq-partialeq-eq"></span>`fn eq(&self, _other: &AndEq) -> bool` — [`AndEq`](#andeq)

##### `impl Sealed for AndEq`

##### `impl Spanned for AndEq`

- <span id="andeq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for AndEq`

- <span id="andeq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for AndEq`

### `At`

```rust
struct At {
    pub spans: [Span; 1],
}
```

`@`

Usage:
 subpattern binding.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for At`

- <span id="at-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for At`

##### `impl Debug for At`

- <span id="at-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for At`

- <span id="at-default"></span>`fn default() -> Self`

##### `impl Deref for At`

- <span id="at-deref-type-target"></span>`type Target = WithSpan`

- <span id="at-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for At`

- <span id="at-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for At`

##### `impl Hash for At`

- <span id="at-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for At`

- <span id="at-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for At`

- <span id="at-partialeq-eq"></span>`fn eq(&self, _other: &At) -> bool` — [`At`](#at)

##### `impl Receiver for At`

- <span id="at-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for At`

##### `impl Spanned for At`

- <span id="at-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for At`

- <span id="at-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for At`

### `Caret`

```rust
struct Caret {
    pub spans: [Span; 1],
}
```

`^`

Usage:
 bitwise and logical XOR.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Caret`

- <span id="caret-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Caret`

##### `impl Debug for Caret`

- <span id="caret-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Caret`

- <span id="caret-default"></span>`fn default() -> Self`

##### `impl Deref for Caret`

- <span id="caret-deref-type-target"></span>`type Target = WithSpan`

- <span id="caret-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Caret`

- <span id="caret-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Caret`

##### `impl Hash for Caret`

- <span id="caret-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Caret`

- <span id="caret-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Caret`

- <span id="caret-partialeq-eq"></span>`fn eq(&self, _other: &Caret) -> bool` — [`Caret`](#caret)

##### `impl Receiver for Caret`

- <span id="caret-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Caret`

##### `impl Spanned for Caret`

- <span id="caret-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Caret`

- <span id="caret-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Caret`

### `CaretEq`

```rust
struct CaretEq {
    pub spans: [Span; 2],
}
```

`^=`

Usage:
 bitwise XOR assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for CaretEq`

- <span id="careteq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for CaretEq`

##### `impl Debug for CaretEq`

- <span id="careteq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CaretEq`

- <span id="careteq-default"></span>`fn default() -> Self`

##### `impl Eq for CaretEq`

##### `impl Hash for CaretEq`

- <span id="careteq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for CaretEq`

- <span id="careteq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for CaretEq`

- <span id="careteq-partialeq-eq"></span>`fn eq(&self, _other: &CaretEq) -> bool` — [`CaretEq`](#careteq)

##### `impl Sealed for CaretEq`

##### `impl Spanned for CaretEq`

- <span id="careteq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for CaretEq`

- <span id="careteq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for CaretEq`

### `Colon`

```rust
struct Colon {
    pub spans: [Span; 1],
}
```

`:`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Colon`

- <span id="colon-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Colon`

##### `impl Debug for Colon`

- <span id="colon-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Colon`

- <span id="colon-default"></span>`fn default() -> Self`

##### `impl Deref for Colon`

- <span id="colon-deref-type-target"></span>`type Target = WithSpan`

- <span id="colon-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Colon`

- <span id="colon-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Colon`

##### `impl Hash for Colon`

- <span id="colon-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Colon`

- <span id="colon-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Colon`

- <span id="colon-partialeq-eq"></span>`fn eq(&self, _other: &Colon) -> bool` — [`Colon`](#colon)

##### `impl Receiver for Colon`

- <span id="colon-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Colon`

##### `impl Spanned for Colon`

- <span id="colon-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Colon`

- <span id="colon-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Colon`

### `Comma`

```rust
struct Comma {
    pub spans: [Span; 1],
}
```

`,`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Comma`

- <span id="comma-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Comma`

##### `impl Debug for Comma`

- <span id="comma-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Comma`

- <span id="comma-default"></span>`fn default() -> Self`

##### `impl Deref for Comma`

- <span id="comma-deref-type-target"></span>`type Target = WithSpan`

- <span id="comma-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Comma`

- <span id="comma-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Comma`

##### `impl Hash for Comma`

- <span id="comma-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Comma`

- <span id="comma-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Comma`

- <span id="comma-partialeq-eq"></span>`fn eq(&self, _other: &Comma) -> bool` — [`Comma`](#comma)

##### `impl Receiver for Comma`

- <span id="comma-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Comma`

##### `impl Spanned for Comma`

- <span id="comma-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Comma`

- <span id="comma-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Comma`

### `Dollar`

```rust
struct Dollar {
    pub spans: [Span; 1],
}
```

`$`

Usage:
 macros.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Dollar`

- <span id="dollar-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Dollar`

##### `impl Debug for Dollar`

- <span id="dollar-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dollar`

- <span id="dollar-default"></span>`fn default() -> Self`

##### `impl Deref for Dollar`

- <span id="dollar-deref-type-target"></span>`type Target = WithSpan`

- <span id="dollar-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Dollar`

- <span id="dollar-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Dollar`

##### `impl Hash for Dollar`

- <span id="dollar-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Dollar`

- <span id="dollar-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Dollar`

- <span id="dollar-partialeq-eq"></span>`fn eq(&self, _other: &Dollar) -> bool` — [`Dollar`](#dollar)

##### `impl Receiver for Dollar`

- <span id="dollar-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Dollar`

##### `impl Spanned for Dollar`

- <span id="dollar-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Dollar`

- <span id="dollar-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Dollar`

### `Dot`

```rust
struct Dot {
    pub spans: [Span; 1],
}
```

`.`

Usage:
 field access, tuple index.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Dot`

- <span id="dot-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Dot`

##### `impl Debug for Dot`

- <span id="dot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dot`

- <span id="dot-default"></span>`fn default() -> Self`

##### `impl Deref for Dot`

- <span id="dot-deref-type-target"></span>`type Target = WithSpan`

- <span id="dot-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Dot`

- <span id="dot-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Dot`

##### `impl Hash for Dot`

- <span id="dot-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Dot`

- <span id="dot-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Dot`

- <span id="dot-partialeq-eq"></span>`fn eq(&self, _other: &Dot) -> bool` — [`Dot`](#dot)

##### `impl Receiver for Dot`

- <span id="dot-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Dot`

##### `impl Spanned for Dot`

- <span id="dot-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Dot`

- <span id="dot-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Dot`

### `DotDot`

```rust
struct DotDot {
    pub spans: [Span; 2],
}
```

`..`

Usage:
 range, struct expressions, patterns, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for DotDot`

- <span id="dotdot-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for DotDot`

##### `impl Debug for DotDot`

- <span id="dotdot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDot`

- <span id="dotdot-default"></span>`fn default() -> Self`

##### `impl Eq for DotDot`

##### `impl Hash for DotDot`

- <span id="dotdot-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for DotDot`

- <span id="dotdot-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for DotDot`

- <span id="dotdot-partialeq-eq"></span>`fn eq(&self, _other: &DotDot) -> bool` — [`DotDot`](#dotdot)

##### `impl Sealed for DotDot`

##### `impl Spanned for DotDot`

- <span id="dotdot-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for DotDot`

- <span id="dotdot-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for DotDot`

### `DotDotDot`

```rust
struct DotDotDot {
    pub spans: [Span; 3],
}
```

`...`

Usage:
 variadic functions, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for DotDotDot`

- <span id="dotdotdot-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for DotDotDot`

##### `impl Debug for DotDotDot`

- <span id="dotdotdot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDotDot`

- <span id="dotdotdot-default"></span>`fn default() -> Self`

##### `impl Eq for DotDotDot`

##### `impl Hash for DotDotDot`

- <span id="dotdotdot-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for DotDotDot`

- <span id="dotdotdot-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for DotDotDot`

- <span id="dotdotdot-partialeq-eq"></span>`fn eq(&self, _other: &DotDotDot) -> bool` — [`DotDotDot`](#dotdotdot)

##### `impl Sealed for DotDotDot`

##### `impl Spanned for DotDotDot`

- <span id="dotdotdot-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for DotDotDot`

- <span id="dotdotdot-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for DotDotDot`

### `DotDotEq`

```rust
struct DotDotEq {
    pub spans: [Span; 3],
}
```

`..=`

Usage:
 inclusive range, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for DotDotEq`

- <span id="dotdoteq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for DotDotEq`

##### `impl Debug for DotDotEq`

- <span id="dotdoteq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDotEq`

- <span id="dotdoteq-default"></span>`fn default() -> Self`

##### `impl Eq for DotDotEq`

##### `impl Hash for DotDotEq`

- <span id="dotdoteq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for DotDotEq`

- <span id="dotdoteq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for DotDotEq`

- <span id="dotdoteq-partialeq-eq"></span>`fn eq(&self, _other: &DotDotEq) -> bool` — [`DotDotEq`](#dotdoteq)

##### `impl Sealed for DotDotEq`

##### `impl Spanned for DotDotEq`

- <span id="dotdoteq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for DotDotEq`

- <span id="dotdoteq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for DotDotEq`

### `Eq`

```rust
struct Eq {
    pub spans: [Span; 1],
}
```

`=`

Usage:
 assignment, attributes, various type definitions.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Eq`

- <span id="eq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Eq`

##### `impl Debug for Eq`

- <span id="eq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Eq`

- <span id="eq-default"></span>`fn default() -> Self`

##### `impl Deref for Eq`

- <span id="eq-deref-type-target"></span>`type Target = WithSpan`

- <span id="eq-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Eq`

- <span id="eq-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Eq`

##### `impl Hash for Eq`

- <span id="eq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Eq`

- <span id="eq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Eq`

- <span id="eq-partialeq-eq"></span>`fn eq(&self, _other: &Eq) -> bool` — [`Eq`](#eq)

##### `impl Receiver for Eq`

- <span id="eq-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Eq`

##### `impl Spanned for Eq`

- <span id="eq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Eq`

- <span id="eq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Eq`

### `EqEq`

```rust
struct EqEq {
    pub spans: [Span; 2],
}
```

`==`

Usage:
 equal.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for EqEq`

- <span id="eqeq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for EqEq`

##### `impl Debug for EqEq`

- <span id="eqeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EqEq`

- <span id="eqeq-default"></span>`fn default() -> Self`

##### `impl Eq for EqEq`

##### `impl Hash for EqEq`

- <span id="eqeq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for EqEq`

- <span id="eqeq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for EqEq`

- <span id="eqeq-partialeq-eq"></span>`fn eq(&self, _other: &EqEq) -> bool` — [`EqEq`](#eqeq)

##### `impl Sealed for EqEq`

##### `impl Spanned for EqEq`

- <span id="eqeq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for EqEq`

- <span id="eqeq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for EqEq`

### `FatArrow`

```rust
struct FatArrow {
    pub spans: [Span; 2],
}
```

`=>`

Usage:
 match arms, macros.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for FatArrow`

- <span id="fatarrow-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for FatArrow`

##### `impl Debug for FatArrow`

- <span id="fatarrow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FatArrow`

- <span id="fatarrow-default"></span>`fn default() -> Self`

##### `impl Eq for FatArrow`

##### `impl Hash for FatArrow`

- <span id="fatarrow-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for FatArrow`

- <span id="fatarrow-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for FatArrow`

- <span id="fatarrow-partialeq-eq"></span>`fn eq(&self, _other: &FatArrow) -> bool` — [`FatArrow`](#fatarrow)

##### `impl Sealed for FatArrow`

##### `impl Spanned for FatArrow`

- <span id="fatarrow-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for FatArrow`

- <span id="fatarrow-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for FatArrow`

### `Ge`

```rust
struct Ge {
    pub spans: [Span; 2],
}
```

`>=`

Usage:
 greater than or equal to, generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Ge`

- <span id="ge-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Ge`

##### `impl Debug for Ge`

- <span id="ge-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ge`

- <span id="ge-default"></span>`fn default() -> Self`

##### `impl Eq for Ge`

##### `impl Hash for Ge`

- <span id="ge-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Ge`

- <span id="ge-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Ge`

- <span id="ge-partialeq-eq"></span>`fn eq(&self, _other: &Ge) -> bool` — [`Ge`](#ge)

##### `impl Sealed for Ge`

##### `impl Spanned for Ge`

- <span id="ge-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Ge`

- <span id="ge-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Ge`

### `Gt`

```rust
struct Gt {
    pub spans: [Span; 1],
}
```

`>`

Usage:
 greater than, generics, paths.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Gt`

- <span id="gt-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Gt`

##### `impl Debug for Gt`

- <span id="gt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Gt`

- <span id="gt-default"></span>`fn default() -> Self`

##### `impl Deref for Gt`

- <span id="gt-deref-type-target"></span>`type Target = WithSpan`

- <span id="gt-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Gt`

- <span id="gt-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Gt`

##### `impl Hash for Gt`

- <span id="gt-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Gt`

- <span id="gt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Gt`

- <span id="gt-partialeq-eq"></span>`fn eq(&self, _other: &Gt) -> bool` — [`Gt`](#gt)

##### `impl Receiver for Gt`

- <span id="gt-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Gt`

##### `impl Spanned for Gt`

- <span id="gt-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Gt`

- <span id="gt-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Gt`

### `LArrow`

```rust
struct LArrow {
    pub spans: [Span; 2],
}
```

`<-`

Usage:
 unused.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for LArrow`

- <span id="larrow-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for LArrow`

##### `impl Debug for LArrow`

- <span id="larrow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LArrow`

- <span id="larrow-default"></span>`fn default() -> Self`

##### `impl Eq for LArrow`

##### `impl Hash for LArrow`

- <span id="larrow-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for LArrow`

- <span id="larrow-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LArrow`

- <span id="larrow-partialeq-eq"></span>`fn eq(&self, _other: &LArrow) -> bool` — [`LArrow`](#larrow)

##### `impl Sealed for LArrow`

##### `impl Spanned for LArrow`

- <span id="larrow-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for LArrow`

- <span id="larrow-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for LArrow`

### `Le`

```rust
struct Le {
    pub spans: [Span; 2],
}
```

`<=`

Usage:
 less than or equal to.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Le`

- <span id="le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Le`

##### `impl Debug for Le`

- <span id="le-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Le`

- <span id="le-default"></span>`fn default() -> Self`

##### `impl Eq for Le`

##### `impl Hash for Le`

- <span id="le-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Le`

- <span id="le-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Le`

- <span id="le-partialeq-eq"></span>`fn eq(&self, _other: &Le) -> bool` — [`Le`](#le)

##### `impl Sealed for Le`

##### `impl Spanned for Le`

- <span id="le-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Le`

- <span id="le-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Le`

### `Lt`

```rust
struct Lt {
    pub spans: [Span; 1],
}
```

`<`

Usage:
 less than, generics, paths.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Lt`

- <span id="lt-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Lt`

##### `impl Debug for Lt`

- <span id="lt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Lt`

- <span id="lt-default"></span>`fn default() -> Self`

##### `impl Deref for Lt`

- <span id="lt-deref-type-target"></span>`type Target = WithSpan`

- <span id="lt-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Lt`

- <span id="lt-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Lt`

##### `impl Hash for Lt`

- <span id="lt-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Lt`

- <span id="lt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Lt`

- <span id="lt-partialeq-eq"></span>`fn eq(&self, _other: &Lt) -> bool` — [`Lt`](#lt)

##### `impl Receiver for Lt`

- <span id="lt-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Lt`

##### `impl Spanned for Lt`

- <span id="lt-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Lt`

- <span id="lt-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Lt`

### `Minus`

```rust
struct Minus {
    pub spans: [Span; 1],
}
```

`-`

Usage:
 subtraction, negation.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Minus`

- <span id="minus-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Minus`

##### `impl Debug for Minus`

- <span id="minus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Minus`

- <span id="minus-default"></span>`fn default() -> Self`

##### `impl Deref for Minus`

- <span id="minus-deref-type-target"></span>`type Target = WithSpan`

- <span id="minus-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Minus`

- <span id="minus-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Minus`

##### `impl Hash for Minus`

- <span id="minus-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Minus`

- <span id="minus-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Minus`

- <span id="minus-partialeq-eq"></span>`fn eq(&self, _other: &Minus) -> bool` — [`Minus`](#minus)

##### `impl Receiver for Minus`

- <span id="minus-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Minus`

##### `impl Spanned for Minus`

- <span id="minus-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Minus`

- <span id="minus-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Minus`

### `MinusEq`

```rust
struct MinusEq {
    pub spans: [Span; 2],
}
```

`-=`

Usage:
 subtraction assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for MinusEq`

- <span id="minuseq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for MinusEq`

##### `impl Debug for MinusEq`

- <span id="minuseq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MinusEq`

- <span id="minuseq-default"></span>`fn default() -> Self`

##### `impl Eq for MinusEq`

##### `impl Hash for MinusEq`

- <span id="minuseq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for MinusEq`

- <span id="minuseq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for MinusEq`

- <span id="minuseq-partialeq-eq"></span>`fn eq(&self, _other: &MinusEq) -> bool` — [`MinusEq`](#minuseq)

##### `impl Sealed for MinusEq`

##### `impl Spanned for MinusEq`

- <span id="minuseq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for MinusEq`

- <span id="minuseq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for MinusEq`

### `Ne`

```rust
struct Ne {
    pub spans: [Span; 2],
}
```

`!=`

Usage:
 not equal.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Ne`

- <span id="ne-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Ne`

##### `impl Debug for Ne`

- <span id="ne-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ne`

- <span id="ne-default"></span>`fn default() -> Self`

##### `impl Eq for Ne`

##### `impl Hash for Ne`

- <span id="ne-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Ne`

- <span id="ne-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Ne`

- <span id="ne-partialeq-eq"></span>`fn eq(&self, _other: &Ne) -> bool` — [`Ne`](#ne)

##### `impl Sealed for Ne`

##### `impl Spanned for Ne`

- <span id="ne-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Ne`

- <span id="ne-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Ne`

### `Not`

```rust
struct Not {
    pub spans: [Span; 1],
}
```

`!`

Usage:
 bitwise and logical NOT, macro calls, inner attributes, never type, negative impls.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Not`

- <span id="not-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Not`

##### `impl Debug for Not`

- <span id="not-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Not`

- <span id="not-default"></span>`fn default() -> Self`

##### `impl Deref for Not`

- <span id="not-deref-type-target"></span>`type Target = WithSpan`

- <span id="not-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Not`

- <span id="not-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Not`

##### `impl Hash for Not`

- <span id="not-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Not`

- <span id="not-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Not`

- <span id="not-partialeq-eq"></span>`fn eq(&self, _other: &Not) -> bool` — [`Not`](#not)

##### `impl Receiver for Not`

- <span id="not-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Not`

##### `impl Spanned for Not`

- <span id="not-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Not`

- <span id="not-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Not`

### `Or`

```rust
struct Or {
    pub spans: [Span; 1],
}
```

`|`

Usage:
 bitwise and logical OR, closures, patterns in match, if let, and while let.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Or`

- <span id="or-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Or`

##### `impl Debug for Or`

- <span id="or-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Or`

- <span id="or-default"></span>`fn default() -> Self`

##### `impl Deref for Or`

- <span id="or-deref-type-target"></span>`type Target = WithSpan`

- <span id="or-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Or`

- <span id="or-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Or`

##### `impl Hash for Or`

- <span id="or-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Or`

- <span id="or-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Or`

- <span id="or-partialeq-eq"></span>`fn eq(&self, _other: &Or) -> bool` — [`Or`](#or)

##### `impl Receiver for Or`

- <span id="or-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Or`

##### `impl Spanned for Or`

- <span id="or-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Or`

- <span id="or-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Or`

### `OrEq`

```rust
struct OrEq {
    pub spans: [Span; 2],
}
```

`|=`

Usage:
 bitwise OR assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for OrEq`

- <span id="oreq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for OrEq`

##### `impl Debug for OrEq`

- <span id="oreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OrEq`

- <span id="oreq-default"></span>`fn default() -> Self`

##### `impl Eq for OrEq`

##### `impl Hash for OrEq`

- <span id="oreq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for OrEq`

- <span id="oreq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for OrEq`

- <span id="oreq-partialeq-eq"></span>`fn eq(&self, _other: &OrEq) -> bool` — [`OrEq`](#oreq)

##### `impl Sealed for OrEq`

##### `impl Spanned for OrEq`

- <span id="oreq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for OrEq`

- <span id="oreq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for OrEq`

### `OrOr`

```rust
struct OrOr {
    pub spans: [Span; 2],
}
```

`||`

Usage:
 lazy OR, closures.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for OrOr`

- <span id="oror-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for OrOr`

##### `impl Debug for OrOr`

- <span id="oror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OrOr`

- <span id="oror-default"></span>`fn default() -> Self`

##### `impl Eq for OrOr`

##### `impl Hash for OrOr`

- <span id="oror-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for OrOr`

- <span id="oror-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for OrOr`

- <span id="oror-partialeq-eq"></span>`fn eq(&self, _other: &OrOr) -> bool` — [`OrOr`](#oror)

##### `impl Sealed for OrOr`

##### `impl Spanned for OrOr`

- <span id="oror-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for OrOr`

- <span id="oror-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for OrOr`

### `PathSep`

```rust
struct PathSep {
    pub spans: [Span; 2],
}
```

`::`

Usage:
 path separator.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for PathSep`

- <span id="pathsep-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for PathSep`

##### `impl Debug for PathSep`

- <span id="pathsep-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathSep`

- <span id="pathsep-default"></span>`fn default() -> Self`

##### `impl Eq for PathSep`

##### `impl Hash for PathSep`

- <span id="pathsep-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for PathSep`

- <span id="pathsep-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for PathSep`

- <span id="pathsep-partialeq-eq"></span>`fn eq(&self, _other: &PathSep) -> bool` — [`PathSep`](#pathsep)

##### `impl Sealed for PathSep`

##### `impl Spanned for PathSep`

- <span id="pathsep-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for PathSep`

- <span id="pathsep-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for PathSep`

### `Percent`

```rust
struct Percent {
    pub spans: [Span; 1],
}
```

`%`

Usage:
 remainder.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Percent`

- <span id="percent-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Percent`

##### `impl Debug for Percent`

- <span id="percent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Percent`

- <span id="percent-default"></span>`fn default() -> Self`

##### `impl Deref for Percent`

- <span id="percent-deref-type-target"></span>`type Target = WithSpan`

- <span id="percent-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Percent`

- <span id="percent-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Percent`

##### `impl Hash for Percent`

- <span id="percent-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Percent`

- <span id="percent-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Percent`

- <span id="percent-partialeq-eq"></span>`fn eq(&self, _other: &Percent) -> bool` — [`Percent`](#percent)

##### `impl Receiver for Percent`

- <span id="percent-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Percent`

##### `impl Spanned for Percent`

- <span id="percent-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Percent`

- <span id="percent-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Percent`

### `PercentEq`

```rust
struct PercentEq {
    pub spans: [Span; 2],
}
```

`%=`

Usage:
 remainder assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for PercentEq`

- <span id="percenteq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for PercentEq`

##### `impl Debug for PercentEq`

- <span id="percenteq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PercentEq`

- <span id="percenteq-default"></span>`fn default() -> Self`

##### `impl Eq for PercentEq`

##### `impl Hash for PercentEq`

- <span id="percenteq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for PercentEq`

- <span id="percenteq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for PercentEq`

- <span id="percenteq-partialeq-eq"></span>`fn eq(&self, _other: &PercentEq) -> bool` — [`PercentEq`](#percenteq)

##### `impl Sealed for PercentEq`

##### `impl Spanned for PercentEq`

- <span id="percenteq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for PercentEq`

- <span id="percenteq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for PercentEq`

### `Plus`

```rust
struct Plus {
    pub spans: [Span; 1],
}
```

`+`

Usage:
 addition, trait bounds, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Plus`

- <span id="plus-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Plus`

##### `impl Debug for Plus`

- <span id="plus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Plus`

- <span id="plus-default"></span>`fn default() -> Self`

##### `impl Deref for Plus`

- <span id="plus-deref-type-target"></span>`type Target = WithSpan`

- <span id="plus-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Plus`

- <span id="plus-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Plus`

##### `impl Hash for Plus`

- <span id="plus-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Plus`

- <span id="plus-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Plus`

- <span id="plus-partialeq-eq"></span>`fn eq(&self, _other: &Plus) -> bool` — [`Plus`](#plus)

##### `impl Receiver for Plus`

- <span id="plus-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Plus`

##### `impl Spanned for Plus`

- <span id="plus-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Plus`

- <span id="plus-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Plus`

### `PlusEq`

```rust
struct PlusEq {
    pub spans: [Span; 2],
}
```

`+=`

Usage:
 addition assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for PlusEq`

- <span id="pluseq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for PlusEq`

##### `impl Debug for PlusEq`

- <span id="pluseq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PlusEq`

- <span id="pluseq-default"></span>`fn default() -> Self`

##### `impl Eq for PlusEq`

##### `impl Hash for PlusEq`

- <span id="pluseq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for PlusEq`

- <span id="pluseq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for PlusEq`

- <span id="pluseq-partialeq-eq"></span>`fn eq(&self, _other: &PlusEq) -> bool` — [`PlusEq`](#pluseq)

##### `impl Sealed for PlusEq`

##### `impl Spanned for PlusEq`

- <span id="pluseq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for PlusEq`

- <span id="pluseq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for PlusEq`

### `Pound`

```rust
struct Pound {
    pub spans: [Span; 1],
}
```

`#`

Usage:
 attributes.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Pound`

- <span id="pound-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Pound`

##### `impl Debug for Pound`

- <span id="pound-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pound`

- <span id="pound-default"></span>`fn default() -> Self`

##### `impl Deref for Pound`

- <span id="pound-deref-type-target"></span>`type Target = WithSpan`

- <span id="pound-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Pound`

- <span id="pound-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Pound`

##### `impl Hash for Pound`

- <span id="pound-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Pound`

- <span id="pound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Pound`

- <span id="pound-partialeq-eq"></span>`fn eq(&self, _other: &Pound) -> bool` — [`Pound`](#pound)

##### `impl Receiver for Pound`

- <span id="pound-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Pound`

##### `impl Spanned for Pound`

- <span id="pound-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Pound`

- <span id="pound-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Pound`

### `Question`

```rust
struct Question {
    pub spans: [Span; 1],
}
```

`?`

Usage:
 question mark operator, questionably sized, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Question`

- <span id="question-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Question`

##### `impl Debug for Question`

- <span id="question-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Question`

- <span id="question-default"></span>`fn default() -> Self`

##### `impl Deref for Question`

- <span id="question-deref-type-target"></span>`type Target = WithSpan`

- <span id="question-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Question`

- <span id="question-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Question`

##### `impl Hash for Question`

- <span id="question-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Question`

- <span id="question-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Question`

- <span id="question-partialeq-eq"></span>`fn eq(&self, _other: &Question) -> bool` — [`Question`](#question)

##### `impl Receiver for Question`

- <span id="question-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Question`

##### `impl Spanned for Question`

- <span id="question-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Question`

- <span id="question-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Question`

### `RArrow`

```rust
struct RArrow {
    pub spans: [Span; 2],
}
```

`->`

Usage:
 function return type, closure return type, function pointer type.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for RArrow`

- <span id="rarrow-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for RArrow`

##### `impl Debug for RArrow`

- <span id="rarrow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RArrow`

- <span id="rarrow-default"></span>`fn default() -> Self`

##### `impl Eq for RArrow`

##### `impl Hash for RArrow`

- <span id="rarrow-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for RArrow`

- <span id="rarrow-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for RArrow`

- <span id="rarrow-partialeq-eq"></span>`fn eq(&self, _other: &RArrow) -> bool` — [`RArrow`](#rarrow)

##### `impl Sealed for RArrow`

##### `impl Spanned for RArrow`

- <span id="rarrow-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for RArrow`

- <span id="rarrow-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for RArrow`

### `Semi`

```rust
struct Semi {
    pub spans: [Span; 1],
}
```

`;`

Usage:
 terminator for various items and statements, array types.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Semi`

- <span id="semi-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Semi`

##### `impl Debug for Semi`

- <span id="semi-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Semi`

- <span id="semi-default"></span>`fn default() -> Self`

##### `impl Deref for Semi`

- <span id="semi-deref-type-target"></span>`type Target = WithSpan`

- <span id="semi-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Semi`

- <span id="semi-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Semi`

##### `impl Hash for Semi`

- <span id="semi-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Semi`

- <span id="semi-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Semi`

- <span id="semi-partialeq-eq"></span>`fn eq(&self, _other: &Semi) -> bool` — [`Semi`](#semi)

##### `impl Receiver for Semi`

- <span id="semi-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Semi`

##### `impl Spanned for Semi`

- <span id="semi-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Semi`

- <span id="semi-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Semi`

### `Shl`

```rust
struct Shl {
    pub spans: [Span; 2],
}
```

`<<`

Usage:
 shift left, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Shl`

- <span id="shl-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Shl`

##### `impl Debug for Shl`

- <span id="shl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Shl`

- <span id="shl-default"></span>`fn default() -> Self`

##### `impl Eq for Shl`

##### `impl Hash for Shl`

- <span id="shl-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Shl`

- <span id="shl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Shl`

- <span id="shl-partialeq-eq"></span>`fn eq(&self, _other: &Shl) -> bool` — [`Shl`](#shl)

##### `impl Sealed for Shl`

##### `impl Spanned for Shl`

- <span id="shl-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Shl`

- <span id="shl-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Shl`

### `ShlEq`

```rust
struct ShlEq {
    pub spans: [Span; 3],
}
```

`<<=`

Usage:
 shift left assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for ShlEq`

- <span id="shleq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for ShlEq`

##### `impl Debug for ShlEq`

- <span id="shleq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ShlEq`

- <span id="shleq-default"></span>`fn default() -> Self`

##### `impl Eq for ShlEq`

##### `impl Hash for ShlEq`

- <span id="shleq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for ShlEq`

- <span id="shleq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for ShlEq`

- <span id="shleq-partialeq-eq"></span>`fn eq(&self, _other: &ShlEq) -> bool` — [`ShlEq`](#shleq)

##### `impl Sealed for ShlEq`

##### `impl Spanned for ShlEq`

- <span id="shleq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for ShlEq`

- <span id="shleq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for ShlEq`

### `Shr`

```rust
struct Shr {
    pub spans: [Span; 2],
}
```

`>>`

Usage:
 shift right, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Shr`

- <span id="shr-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Shr`

##### `impl Debug for Shr`

- <span id="shr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Shr`

- <span id="shr-default"></span>`fn default() -> Self`

##### `impl Eq for Shr`

##### `impl Hash for Shr`

- <span id="shr-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Shr`

- <span id="shr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Shr`

- <span id="shr-partialeq-eq"></span>`fn eq(&self, _other: &Shr) -> bool` — [`Shr`](#shr)

##### `impl Sealed for Shr`

##### `impl Spanned for Shr`

- <span id="shr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Shr`

- <span id="shr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Shr`

### `ShrEq`

```rust
struct ShrEq {
    pub spans: [Span; 3],
}
```

`>>=`

Usage:
 shift right assignment, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for ShrEq`

- <span id="shreq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for ShrEq`

##### `impl Debug for ShrEq`

- <span id="shreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ShrEq`

- <span id="shreq-default"></span>`fn default() -> Self`

##### `impl Eq for ShrEq`

##### `impl Hash for ShrEq`

- <span id="shreq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for ShrEq`

- <span id="shreq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for ShrEq`

- <span id="shreq-partialeq-eq"></span>`fn eq(&self, _other: &ShrEq) -> bool` — [`ShrEq`](#shreq)

##### `impl Sealed for ShrEq`

##### `impl Spanned for ShrEq`

- <span id="shreq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for ShrEq`

- <span id="shreq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for ShrEq`

### `Slash`

```rust
struct Slash {
    pub spans: [Span; 1],
}
```

`/`

Usage:
 division.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Slash`

- <span id="slash-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Slash`

##### `impl Debug for Slash`

- <span id="slash-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Slash`

- <span id="slash-default"></span>`fn default() -> Self`

##### `impl Deref for Slash`

- <span id="slash-deref-type-target"></span>`type Target = WithSpan`

- <span id="slash-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Slash`

- <span id="slash-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Slash`

##### `impl Hash for Slash`

- <span id="slash-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Slash`

- <span id="slash-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Slash`

- <span id="slash-partialeq-eq"></span>`fn eq(&self, _other: &Slash) -> bool` — [`Slash`](#slash)

##### `impl Receiver for Slash`

- <span id="slash-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Slash`

##### `impl Spanned for Slash`

- <span id="slash-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Slash`

- <span id="slash-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Slash`

### `SlashEq`

```rust
struct SlashEq {
    pub spans: [Span; 2],
}
```

`/=`

Usage:
 division assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for SlashEq`

- <span id="slasheq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for SlashEq`

##### `impl Debug for SlashEq`

- <span id="slasheq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SlashEq`

- <span id="slasheq-default"></span>`fn default() -> Self`

##### `impl Eq for SlashEq`

##### `impl Hash for SlashEq`

- <span id="slasheq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for SlashEq`

- <span id="slasheq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for SlashEq`

- <span id="slasheq-partialeq-eq"></span>`fn eq(&self, _other: &SlashEq) -> bool` — [`SlashEq`](#slasheq)

##### `impl Sealed for SlashEq`

##### `impl Spanned for SlashEq`

- <span id="slasheq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for SlashEq`

- <span id="slasheq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for SlashEq`

### `Star`

```rust
struct Star {
    pub spans: [Span; 1],
}
```

`*`

Usage:
 multiplication, dereference, raw pointers, macro Kleene matcher, use wildcards.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Star`

- <span id="star-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Star`

##### `impl Debug for Star`

- <span id="star-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Star`

- <span id="star-default"></span>`fn default() -> Self`

##### `impl Deref for Star`

- <span id="star-deref-type-target"></span>`type Target = WithSpan`

- <span id="star-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Star`

- <span id="star-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Star`

##### `impl Hash for Star`

- <span id="star-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Star`

- <span id="star-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Star`

- <span id="star-partialeq-eq"></span>`fn eq(&self, _other: &Star) -> bool` — [`Star`](#star)

##### `impl Receiver for Star`

- <span id="star-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Star`

##### `impl Spanned for Star`

- <span id="star-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Star`

- <span id="star-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Star`

### `StarEq`

```rust
struct StarEq {
    pub spans: [Span; 2],
}
```

`*=`

Usage:
 multiplication assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for StarEq`

- <span id="stareq-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for StarEq`

##### `impl Debug for StarEq`

- <span id="stareq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StarEq`

- <span id="stareq-default"></span>`fn default() -> Self`

##### `impl Eq for StarEq`

##### `impl Hash for StarEq`

- <span id="stareq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for StarEq`

- <span id="stareq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for StarEq`

- <span id="stareq-partialeq-eq"></span>`fn eq(&self, _other: &StarEq) -> bool` — [`StarEq`](#stareq)

##### `impl Sealed for StarEq`

##### `impl Spanned for StarEq`

- <span id="stareq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for StarEq`

- <span id="stareq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for StarEq`

### `Tilde`

```rust
struct Tilde {
    pub spans: [Span; 1],
}
```

`~`

Usage:
 unused since before Rust 1.0.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Tilde`

- <span id="tilde-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Tilde`

##### `impl Debug for Tilde`

- <span id="tilde-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Tilde`

- <span id="tilde-default"></span>`fn default() -> Self`

##### `impl Deref for Tilde`

- <span id="tilde-deref-type-target"></span>`type Target = WithSpan`

- <span id="tilde-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Tilde`

- <span id="tilde-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Tilde`

##### `impl Hash for Tilde`

- <span id="tilde-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl Parse for Tilde`

- <span id="tilde-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Tilde`

- <span id="tilde-partialeq-eq"></span>`fn eq(&self, _other: &Tilde) -> bool` — [`Tilde`](#tilde)

##### `impl Receiver for Tilde`

- <span id="tilde-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Tilde`

##### `impl Spanned for Tilde`

- <span id="tilde-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Tilde`

- <span id="tilde-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Tilde`

### `Brace`

```rust
struct Brace {
    pub span: DelimSpan,
}
```

`{`&hellip;`}`

#### Implementations

- <span id="brace-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Brace`

- <span id="brace-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Brace`

##### `impl Debug for Brace`

- <span id="brace-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Brace`

- <span id="brace-default"></span>`fn default() -> Self`

##### `impl Eq for Brace`

##### `impl Hash for Brace`

- <span id="brace-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl PartialEq for Brace`

- <span id="brace-partialeq-eq"></span>`fn eq(&self, _other: &Brace) -> bool` — [`Brace`](#brace)

##### `impl Sealed for Brace`

##### `impl Token for Brace`

### `Bracket`

```rust
struct Bracket {
    pub span: DelimSpan,
}
```

``&hellip;``

#### Implementations

- <span id="bracket-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Bracket`

- <span id="bracket-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Bracket`

##### `impl Debug for Bracket`

- <span id="bracket-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bracket`

- <span id="bracket-default"></span>`fn default() -> Self`

##### `impl Eq for Bracket`

##### `impl Hash for Bracket`

- <span id="bracket-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl PartialEq for Bracket`

- <span id="bracket-partialeq-eq"></span>`fn eq(&self, _other: &Bracket) -> bool` — [`Bracket`](#bracket)

##### `impl Sealed for Bracket`

##### `impl Token for Bracket`

### `Paren`

```rust
struct Paren {
    pub span: DelimSpan,
}
```

`(`&hellip;`)`

#### Implementations

- <span id="paren-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Paren`

- <span id="paren-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Paren`

##### `impl Debug for Paren`

- <span id="paren-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Paren`

- <span id="paren-default"></span>`fn default() -> Self`

##### `impl Eq for Paren`

##### `impl Hash for Paren`

- <span id="paren-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl PartialEq for Paren`

- <span id="paren-partialeq-eq"></span>`fn eq(&self, _other: &Paren) -> bool` — [`Paren`](#paren)

##### `impl Sealed for Paren`

##### `impl Token for Paren`

## Traits

### `Token`

```rust
trait Token: private::Sealed { ... }
```

Marker trait for types that represent single tokens.

This trait is sealed and cannot be implemented for types outside of Syn.

#### Implementors

- [`Abstract`](#abstract)
- [`AndAnd`](#andand)
- [`AndEq`](#andeq)
- [`And`](#and)
- [`As`](#as)
- [`Async`](#async)
- [`At`](#at)
- [`Auto`](#auto)
- [`Await`](#await)
- [`Become`](#become)
- [`Box`](#box)
- [`Brace`](#brace)
- [`Bracket`](#bracket)
- [`Break`](#break)
- [`CaretEq`](#careteq)
- [`Caret`](#caret)
- [`Colon`](#colon)
- [`Comma`](#comma)
- [`Const`](#const)
- [`Continue`](#continue)
- [`Crate`](#crate)
- [`Default`](#default)
- [`Do`](#do)
- [`Dollar`](#dollar)
- [`DotDotDot`](#dotdotdot)
- [`DotDotEq`](#dotdoteq)
- [`DotDot`](#dotdot)
- [`Dot`](#dot)
- [`Dyn`](#dyn)
- [`Else`](#else)
- [`Enum`](#enum)
- [`EqEq`](#eqeq)
- [`Eq`](#eq)
- [`Extern`](#extern)
- [`FatArrow`](#fatarrow)
- [`Final`](#final)
- [`Fn`](#fn)
- [`For`](#for)
- [`Ge`](#ge)
- [`Group`](#group)
- [`Gt`](#gt)
- [`Ident`](../ident/index.md#ident)
- [`If`](#if)
- [`Impl`](#impl)
- [`In`](#in)
- [`LArrow`](#larrow)
- [`Le`](#le)
- [`Let`](#let)
- [`Lifetime`](../lifetime/index.md#lifetime)
- [`LitBool`](../lit/index.md#litbool)
- [`LitByteStr`](../lit/index.md#litbytestr)
- [`LitByte`](../lit/index.md#litbyte)
- [`LitCStr`](../lit/index.md#litcstr)
- [`LitChar`](../lit/index.md#litchar)
- [`LitFloat`](../lit/index.md#litfloat)
- [`LitInt`](../lit/index.md#litint)
- [`LitStr`](../lit/index.md#litstr)
- [`Lit`](../lit/index.md#lit)
- [`Loop`](#loop)
- [`Lt`](#lt)
- [`Macro`](#macro)
- [`Match`](#match)
- [`MinusEq`](#minuseq)
- [`Minus`](#minus)
- [`Mod`](#mod)
- [`Move`](#move)
- [`Mut`](#mut)
- [`Ne`](#ne)
- [`Not`](#not)
- [`OrEq`](#oreq)
- [`OrOr`](#oror)
- [`Or`](#or)
- [`Override`](#override)
- [`Paren`](#paren)
- [`PathSep`](#pathsep)
- [`PercentEq`](#percenteq)
- [`Percent`](#percent)
- [`PlusEq`](#pluseq)
- [`Plus`](#plus)
- [`Pound`](#pound)
- [`Priv`](#priv)
- [`Pub`](#pub)
- [`Question`](#question)
- [`RArrow`](#rarrow)
- [`Raw`](#raw)
- [`Ref`](#ref)
- [`Return`](#return)
- [`SelfType`](#selftype)
- [`SelfValue`](#selfvalue)
- [`Semi`](#semi)
- [`ShlEq`](#shleq)
- [`Shl`](#shl)
- [`ShrEq`](#shreq)
- [`Shr`](#shr)
- [`SlashEq`](#slasheq)
- [`Slash`](#slash)
- [`StarEq`](#stareq)
- [`Star`](#star)
- [`Static`](#static)
- [`Struct`](#struct)
- [`Super`](#super)
- [`Tilde`](#tilde)
- [`Trait`](#trait)
- [`Try`](#try)
- [`Type`](#type)
- [`Typeof`](#typeof)
- [`Underscore`](#underscore)
- [`Union`](#union)
- [`Unsafe`](#unsafe)
- [`Unsized`](#unsized)
- [`Use`](#use)
- [`Virtual`](#virtual)
- [`Where`](#where)
- [`While`](#while)
- [`Yield`](#yield)
- `T`
- `proc_macro2::Group`
- `proc_macro2::Literal`
- `proc_macro2::Punct`
- `proc_macro2::TokenTree`

## Macros

### `impl_low_level_token!`

### `define_keywords!`

### `impl_deref_if_len_is_1!`

### `define_punctuation_structs!`

### `define_punctuation!`

### `define_delimiters!`

