*[syn](../index.md) / [lifetime](index.md)*

---

# Module `lifetime`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Lifetime`](#lifetime) | struct | A Rust lifetime: `'a`. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Lifetime`

```rust
struct Lifetime {
    pub apostrophe: proc_macro2::Span,
    pub ident: proc_macro2::Ident,
}
```

A Rust lifetime: `'a`.

Lifetime names must conform to the following rules:

- Must start with an apostrophe.
- Must not consist of just an apostrophe: `'`.
- Character after the apostrophe must be `_` or a Unicode code point with
  the XID_Start property.
- All following characters must be Unicode code points with the XID_Continue
  property.

#### Implementations

- <span id="lifetime-new"></span>`fn new(symbol: &str, span: Span) -> Self`

  # Panics

  

  Panics if the lifetime does not conform to the bulleted rules above.

  

  # Invocation

  

  ```rust

  use proc_macro2::Span;

  use syn::Lifetime;

  

  fn f() -> Lifetime {

  Lifetime::new("'a", Span::call_site())

  }

  ```

- <span id="lifetime-span"></span>`fn span(&self) -> Span`

- <span id="lifetime-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Clone for Lifetime`

- <span id="lifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Lifetime`

- <span id="cratelifetime-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Lifetime`

- <span id="lifetime-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Lifetime`

##### `impl Hash for Lifetime`

- <span id="lifetime-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl Ord for Lifetime`

- <span id="lifetime-ord-cmp"></span>`fn cmp(&self, other: &Lifetime) -> Ordering` — [`Lifetime`](#lifetime)

##### `impl Parse for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Lifetime`

- <span id="lifetime-partialeq-eq"></span>`fn eq(&self, other: &Lifetime) -> bool` — [`Lifetime`](#lifetime)

##### `impl PartialOrd for Lifetime`

- <span id="lifetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Lifetime) -> Option<Ordering>` — [`Lifetime`](#lifetime)

##### `impl Sealed for Lifetime`

##### `impl Spanned for Lifetime`

- <span id="lifetime-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToString for Lifetime`

- <span id="lifetime-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lifetime::Lifetime`

