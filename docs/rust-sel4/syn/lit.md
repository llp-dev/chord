**syn > lit**

# Module: lit

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)
- [`value`](#value)

**Macros**

- [`lit_extra_traits`](#lit_extra_traits)

**Structs**

- [`LitBool`](#litbool) - A boolean literal: `true` or `false`.
- [`LitByte`](#litbyte) - A byte literal: `b'f'`.
- [`LitByteStr`](#litbytestr) - A byte string literal: `b"foo"`.
- [`LitCStr`](#litcstr) - A nul-terminated C-string literal: `c"foo"`.
- [`LitChar`](#litchar) - A character literal: `'a'`.
- [`LitFloat`](#litfloat) - A floating point literal: `1f64` or `1.0e10f64`.
- [`LitFloatRepr`](#litfloatrepr)
- [`LitInt`](#litint) - An integer literal: `1` or `1u16`.
- [`LitIntRepr`](#litintrepr)
- [`LitRepr`](#litrepr)
- [`LitStr`](#litstr) - A UTF-8 string literal: `"foo"`.

**Enums**

- [`Lit`](#lit) - A Rust literal such as a string or integer or boolean.

---

## syn::lit::Lit

*Enum*

A Rust literal such as a string or integer or boolean.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Str(LitStr)` - A UTF-8 string literal: `"foo"`.
- `ByteStr(LitByteStr)` - A byte string literal: `b"foo"`.
- `CStr(LitCStr)` - A nul-terminated C-string literal: `c"foo"`.
- `Byte(LitByte)` - A byte literal: `b'f'`.
- `Char(LitChar)` - A character literal: `'a'`.
- `Int(LitInt)` - An integer literal: `1` or `1u16`.
- `Float(LitFloat)` - A floating point literal: `1f64` or `1.0e10f64`.
- `Bool(LitBool)` - A boolean literal: `true` or `false`.
- `Verbatim(proc_macro2::Literal)` - A raw token literal not interpreted by Syn.

**Methods:**

- `fn new(token: Literal) -> Self` - Interpret a Syn literal from a proc-macro2 literal.
- `fn from_str(token: Literal, repr: &str) -> Self`
- `fn suffix(self: &Self) -> &str`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`

**Traits:** Token, Sealed

**Trait Implementations:**

- **From**
  - `fn from(e: LitChar) -> Lit`
- **From**
  - `fn from(e: LitBool) -> Lit`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: LitStr) -> Lit`
- **From**
  - `fn from(e: LitByte) -> Lit`
- **From**
  - `fn from(e: LitFloat) -> Lit`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: LitCStr) -> Lit`
- **From**
  - `fn from(e: LitInt) -> Lit`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: LitByteStr) -> Lit`



## syn::lit::LitBool

*Struct*

A boolean literal: `true` or `false`.

**Fields:**
- `value: bool`
- `span: proc_macro2::Span`

**Methods:**

- `fn new(value: bool, span: Span) -> Self`
- `fn value(self: &Self) -> bool`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn token(self: &Self) -> Ident`

**Traits:** Sealed, Token

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::lit::LitByte

*Struct*

A byte literal: `b'f'`.

**Fields:**
- `repr: alloc::boxed::Box<LitRepr>`

**Methods:**

- `fn new(value: u8, span: Span) -> Self`
- `fn value(self: &Self) -> u8`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn suffix(self: &Self) -> &str`
- `fn token(self: &Self) -> Literal`

**Traits:** Sealed, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::lit::LitByteStr

*Struct*

A byte string literal: `b"foo"`.

**Fields:**
- `repr: alloc::boxed::Box<LitRepr>`

**Methods:**

- `fn new(value: &[u8], span: Span) -> Self`
- `fn value(self: &Self) -> Vec<u8>`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn suffix(self: &Self) -> &str`
- `fn token(self: &Self) -> Literal`

**Traits:** Token, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::lit::LitCStr

*Struct*

A nul-terminated C-string literal: `c"foo"`.

**Fields:**
- `repr: alloc::boxed::Box<LitRepr>`

**Methods:**

- `fn new(value: &CStr, span: Span) -> Self`
- `fn value(self: &Self) -> CString`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn suffix(self: &Self) -> &str`
- `fn token(self: &Self) -> Literal`

**Traits:** Token, Sealed

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::lit::LitChar

*Struct*

A character literal: `'a'`.

**Fields:**
- `repr: alloc::boxed::Box<LitRepr>`

**Methods:**

- `fn new(value: char, span: Span) -> Self`
- `fn value(self: &Self) -> char`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn suffix(self: &Self) -> &str`
- `fn token(self: &Self) -> Literal`

**Traits:** Token, Sealed

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::lit::LitFloat

*Struct*

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

**Fields:**
- `repr: alloc::boxed::Box<LitFloatRepr>`

**Methods:**

- `fn new(repr: &str, span: Span) -> Self`
- `fn base10_digits(self: &Self) -> &str`
- `fn base10_parse<N>(self: &Self) -> Result<N>`
- `fn suffix(self: &Self) -> &str`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn token(self: &Self) -> Literal`

**Traits:** Sealed, Token

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **From**
  - `fn from(token: Literal) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::lit::LitFloatRepr

*Struct*

**Fields:**
- `token: proc_macro2::Literal`
- `digits: alloc::boxed::Box<str>`
- `suffix: alloc::boxed::Box<str>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::lit::LitInt

*Struct*

An integer literal: `1` or `1u16`.

**Fields:**
- `repr: alloc::boxed::Box<LitIntRepr>`

**Methods:**

- `fn new(repr: &str, span: Span) -> Self`
- `fn base10_digits(self: &Self) -> &str`
- `fn base10_parse<N>(self: &Self) -> Result<N>` - Parses the literal into a selected number type.
- `fn suffix(self: &Self) -> &str`
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn token(self: &Self) -> Literal`

**Traits:** Token, Sealed

**Trait Implementations:**

- **From**
  - `fn from(token: Literal) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::lit::LitIntRepr

*Struct*

**Fields:**
- `token: proc_macro2::Literal`
- `digits: alloc::boxed::Box<str>`
- `suffix: alloc::boxed::Box<str>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::lit::LitRepr

*Struct*

**Fields:**
- `token: proc_macro2::Literal`
- `suffix: alloc::boxed::Box<str>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::lit::LitStr

*Struct*

A UTF-8 string literal: `"foo"`.

**Fields:**
- `repr: alloc::boxed::Box<LitRepr>`

**Methods:**

- `fn new(value: &str, span: Span) -> Self`
- `fn value(self: &Self) -> String`
- `fn parse<T>(self: &Self) -> Result<T>` - Parse a syntax tree node from the content of this string literal.
- `fn parse_with<F>(self: &Self, parser: F) -> Result<<F as >::Output>` - Invoke parser on the content of this string literal.
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`
- `fn suffix(self: &Self) -> &str`
- `fn token(self: &Self) -> Literal`

**Traits:** Sealed, Token

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::lit::lit_extra_traits

*Declarative Macro*

```rust
macro_rules! lit_extra_traits {
    ($ty:ident) => { ... };
}
```



## Module: parsing



## Module: printing



## Module: value



