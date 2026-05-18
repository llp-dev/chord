**thiserror_impl > unraw**

# Module: unraw

## Contents

**Structs**

- [`IdentUnraw`](#identunraw)

**Enums**

- [`MemberUnraw`](#memberunraw)

---

## thiserror_impl::unraw::IdentUnraw

*Struct*

**Tuple Struct**: `(proc_macro2::Ident)`

**Methods:**

- `fn new(ident: Ident) -> Self`
- `fn to_local(self: &Self) -> Ident`
- `fn set_span(self: & mut Self, span: Span)`

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> IdentUnraw`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## thiserror_impl::unraw::MemberUnraw

*Enum*

**Variants:**
- `Named(IdentUnraw)`
- `Unnamed(syn::Index)`

**Methods:**

- `fn span(self: &Self) -> Span`

**Traits:** Eq

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<H>(self: &Self, hasher: & mut H)`
- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MemberUnraw`



