**syn > mac**

# Module: mac

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`Macro`](#macro) - A macro invocation: `println!("{}", mac)`.

**Enums**

- [`MacroDelimiter`](#macrodelimiter) - A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

**Functions**

- [`parse_delimiter`](#parse_delimiter)

---

## syn::mac::Macro

*Struct*

A macro invocation: `println!("{}", mac)`.

**Fields:**
- `path: crate::path::Path`
- `bang_token: $crate::token::Not`
- `delimiter: MacroDelimiter`
- `tokens: proc_macro2::TokenStream`

**Methods:**

- `fn parse_body<T>(self: &Self) -> Result<T>` - Parse the tokens within the macro invocation's delimiters into a syntax
- `fn parse_body_with<F>(self: &Self, parser: F) -> Result<<F as >::Output>` - Parse the tokens within the macro invocation's delimiters using the

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::mac::MacroDelimiter

*Enum*

A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

**Variants:**
- `Paren(crate::token::Paren)`
- `Brace(crate::token::Brace)`
- `Bracket(crate::token::Bracket)`

**Methods:**

- `fn surround(self: &Self, tokens: & mut TokenStream, inner: TokenStream)`
- `fn span(self: &Self) -> &DelimSpan`
- `fn is_brace(self: &Self) -> bool`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::mac::parse_delimiter

*Function*

```rust
fn parse_delimiter(input: crate::parse::ParseStream) -> crate::error::Result<(MacroDelimiter, proc_macro2::TokenStream)>
```



## Module: parsing



## Module: printing



