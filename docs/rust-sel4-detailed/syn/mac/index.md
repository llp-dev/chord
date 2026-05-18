*[syn](../index.md) / [mac](index.md)*

---

# Module `mac`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Macro`](#macro) | struct | A macro invocation: `println!("{}", mac)`. |
| [`MacroDelimiter`](#macrodelimiter) | enum | A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`. |
| [`parse_delimiter`](#parse-delimiter) | fn |  |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Macro`

```rust
struct Macro {
    pub path: crate::path::Path,
    pub bang_token: token::Not,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

A macro invocation: `println!("{}", mac)`.

#### Implementations

- <span id="macro-parse-body"></span>`fn parse_body<T: Parse>(&self) -> Result<T>` — [`Result`](../error/index.md#result)

  Parse the tokens within the macro invocation's delimiters into a syntax

  tree.

  

  This is equivalent to `syn::parse2::<T>(mac.tokens)` except that it

  produces a more useful span when `tokens` is empty.

  

  # Example

  

  ```rust

  use syn::{parse_quote, Expr, ExprLit, Ident, Lit, LitStr, Macro, Token};

  use syn::ext::IdentExt;

  use syn::parse::{Error, Parse, ParseStream, Result};

  use syn::punctuated::Punctuated;

  

  // The arguments expected by libcore's format_args macro, and as a

  // result most other formatting and printing macros like println.

  //

  //     println!("{} is {number:.prec$}", "x", prec=5, number=0.01)

  struct FormatArgs {

      format_string: Expr,

      positional_args: Vec<Expr>,

      named_args: Vec<(Ident, Expr)>,

  }

  

  impl Parse for FormatArgs {

      fn parse(input: ParseStream) -> Result<Self> {

          let format_string: Expr;

          let mut positional_args = Vec::new();

          let mut named_args = Vec::new();

  

          format_string = input.parse()?;

          while !input.is_empty() {

              input.parse::<Token![,]>()?;

              if input.is_empty() {

                  break;

              }

              if input.peek(Ident::peek_any) && input.peek2(Token![=]) {

                  while !input.is_empty() {

                      let name: Ident = input.call(Ident::parse_any)?;

                      input.parse::<Token![=]>()?;

                      let value: Expr = input.parse()?;

                      named_args.push((name, value));

                      if input.is_empty() {

                          break;

                      }

                      input.parse::<Token![,]>()?;

                  }

                  break;

              }

              positional_args.push(input.parse()?);

          }

  

          Ok(FormatArgs {

              format_string,

              positional_args,

              named_args,

          })

      }

  }

  

  // Extract the first argument, the format string literal, from an

  // invocation of a formatting or printing macro.

  fn get_format_string(m: &Macro) -> Result<LitStr> {

      let args: FormatArgs = m.parse_body()?;

      match args.format_string {

          Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) => Ok(lit),

          other => {

              // First argument was not a string literal expression.

              // Maybe something like: println!(concat!(...), ...)

              Err(Error::new_spanned(other, "format string must be a string literal"))

          }

      }

  }

  

  fn main() {

      let invocation = parse_quote! {

          println!("{:?}", Instant::now())

      };

      let lit = get_format_string(&invocation).unwrap();

      assert_eq!(lit.value(), "{:?}");

  }

  ```

- <span id="macro-parse-body-with"></span>`fn parse_body_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md#result), [`Parser`](../parse/index.md#parser)

  Parse the tokens within the macro invocation's delimiters using the

  given parser.

#### Trait Implementations

##### `impl Clone for crate::Macro`

- <span id="cratemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Macro`

- <span id="cratemacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Macro`

##### `impl Hash for crate::Macro`

- <span id="cratemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::mac::Macro`

- <span id="cratemacmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Macro`

- <span id="cratemacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Macro`

##### `impl Spanned for Macro`

- <span id="macro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::mac::Macro`

- <span id="cratemacmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `MacroDelimiter`

```rust
enum MacroDelimiter {
    Paren(crate::token::Paren),
    Brace(crate::token::Brace),
    Bracket(crate::token::Bracket),
}
```

A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

#### Implementations

- <span id="cratemacmacrodelimiter-surround"></span>`fn surround(&self, tokens: &mut TokenStream, inner: TokenStream)`

#### Trait Implementations

##### `impl Clone for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MacroDelimiter`

##### `impl Hash for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Functions

### `parse_delimiter`

```rust
fn parse_delimiter(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(MacroDelimiter, proc_macro2::TokenStream)>
```

