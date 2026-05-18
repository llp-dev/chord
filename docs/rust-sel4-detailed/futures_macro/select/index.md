*[futures_macro](../index.md) / [select](index.md)*

---

# Module `select`

The futures-rs `select!` macro implementation.

## Contents

- [Modules](#modules)
  - [`kw`](#kw)
- [Structs](#structs)
  - [`Select`](#select)
- [Enums](#enums)
  - [`CaseKind`](#casekind)
- [Functions](#functions)
  - [`declare_result_enum`](#declare-result-enum)
  - [`select`](#select)
  - [`select_biased`](#select-biased)
  - [`select_inner`](#select-inner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`kw`](#kw) | mod |  |
| [`Select`](#select) | struct |  |
| [`CaseKind`](#casekind) | enum |  |
| [`declare_result_enum`](#declare-result-enum) | fn |  |
| [`select`](#select) | fn | The `select!` macro. |
| [`select_biased`](#select-biased) | fn | The `select_biased!` macro. |
| [`select_inner`](#select-inner) | fn |  |

## Modules

- [`kw`](kw/index.md)

## Structs

### `Select`

```rust
struct Select {
    complete: Option<syn::Expr>,
    default: Option<syn::Expr>,
    normal_fut_exprs: Vec<syn::Expr>,
    normal_fut_handlers: Vec<(syn::Pat, syn::Expr)>,
}
```

#### Trait Implementations

##### `impl Parse for Select`

- <span id="select-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

## Enums

### `CaseKind`

```rust
enum CaseKind {
    Complete,
    Default,
    Normal(syn::Pat, syn::Expr),
}
```

## Functions

### `declare_result_enum`

```rust
fn declare_result_enum(result_ident: syn::Ident, variants: usize, complete: bool, span: proc_macro2::Span) -> (Vec<syn::Ident>, syn::ItemEnum)
```

### `select`

```rust
fn select(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

The `select!` macro.

### `select_biased`

```rust
fn select_biased(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

The `select_biased!` macro.

### `select_inner`

```rust
fn select_inner(input: proc_macro::TokenStream, random: bool) -> proc_macro::TokenStream
```

