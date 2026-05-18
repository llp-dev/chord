# Crate `munge_macro`

The proc macro at the core of munge.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | struct |  |
| [`Destructure`](#destructure) | struct |  |
| [`make_rest_check`](#make-rest-check) | fn |  |
| [`parse_pat`](#parse-pat) | fn |  |
| [`strip_mut`](#strip-mut) | fn |  |
| [`destructure`](#destructure) | fn |  |

## Structs

### `Input`

```rust
struct Input {
    crate_path: syn::Path,
    _arrow: syn::token::FatArrow,
    destructures: syn::punctuated::Punctuated<Destructure, syn::token::Semi>,
}
```

#### Trait Implementations

##### `impl Parse for Input`

- <span id="input-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<Self>`

### `Destructure`

```rust
struct Destructure {
    _let_token: syn::token::Let,
    pat: syn::Pat,
    _eq_token: syn::token::Eq,
    expr: syn::Expr,
}
```

#### Trait Implementations

##### `impl Parse for Destructure`

- <span id="destructure-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<Self>`

## Functions

### `make_rest_check`

```rust
fn make_rest_check(crate_path: &syn::Path, rest: &syn::PatRest) -> proc_macro2::TokenStream
```

### `parse_pat`

```rust
fn parse_pat(crate_path: &syn::Path, pat: &syn::Pat) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream), syn::Error>
```

### `strip_mut`

```rust
fn strip_mut(pat: &syn::Pat) -> Result<syn::Pat, syn::Error>
```

### `destructure`

```rust
fn destructure(input: Input) -> Result<proc_macro2::TokenStream, syn::Error>
```

