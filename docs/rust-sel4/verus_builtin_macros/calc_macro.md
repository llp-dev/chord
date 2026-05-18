**verus_builtin_macros > calc_macro**

# Module: calc_macro

## Contents

**Structs**

- [`CalcInput`](#calcinput)
- [`Relation`](#relation)

**Enums**

- [`RelationOp`](#relationop)

**Functions**

- [`calc_macro`](#calc_macro)
- [`rewrite_expr_to_token_stream`](#rewrite_expr_to_token_stream)

---

## verus_builtin_macros::calc_macro::CalcInput

*Struct*

**Fields:**
- `reln: Relation`
- `steps: Vec<(verus_syn::Expr, Option<Relation>, verus_syn::Block)>`
- `last: verus_syn::Expr`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::Result<Self>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::calc_macro::Relation

*Struct*

**Fields:**
- `op: RelationOp`
- `span: proc_macro2::Span`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## verus_builtin_macros::calc_macro::RelationOp

*Enum*

**Variants:**
- `Eq`
- `Lt`
- `Leq`
- `Gt`
- `Geq`
- `Implies`
- `Iff`

**Methods:**

- `fn compatible(self: &Self, middle: &Self) -> bool` - Confirm that a `middle` relation is consistent with the main relation (self)
- `fn to_expr(self: &Self, left: &Expr, right: &Expr) -> TokenStream` - Translate to a Rust boolean expression

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::calc_macro::calc_macro

*Function*

```rust
fn calc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::calc_macro::rewrite_expr_to_token_stream

*Function*

```rust
fn rewrite_expr_to_token_stream(expr: &verus_syn::Expr) -> proc_macro2::TokenStream
```



