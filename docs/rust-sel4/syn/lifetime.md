**syn > lifetime**

# Module: lifetime

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`Lifetime`](#lifetime) - A Rust lifetime: `'a`.

---

## syn::lifetime::Lifetime

*Struct*

A Rust lifetime: `'a`.

Lifetime names must conform to the following rules:

- Must start with an apostrophe.
- Must not consist of just an apostrophe: `'`.
- Character after the apostrophe must be `_` or a Unicode code point with
  the XID_Start property.
- All following characters must be Unicode code points with the XID_Continue
  property.

**Fields:**
- `apostrophe: proc_macro2::Span`
- `ident: proc_macro2::Ident`

**Methods:**

- `fn new(symbol: &str, span: Span) -> Self` - # Panics
- `fn span(self: &Self) -> Span`
- `fn set_span(self: & mut Self, span: Span)`

**Traits:** Token, Eq, Sealed

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Lifetime) -> Ordering`
- **Hash**
  - `fn hash<H>(self: &Self, h: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Lifetime) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Lifetime) -> Option<Ordering>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## Module: parsing



## Module: printing



