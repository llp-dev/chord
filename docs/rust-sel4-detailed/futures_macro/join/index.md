*[futures_macro](../index.md) / [join](index.md)*

---

# Module `join`

The futures-rs `join!` macro implementation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Join`](#join) | struct |  |
| [`bind_futures`](#bind-futures) | fn |  |
| [`join`](#join) | fn | The `join!` macro. |
| [`try_join`](#try-join) | fn | The `try_join!` macro. |

## Structs

### `Join`

```rust
struct Join {
    fut_exprs: Vec<syn::Expr>,
}
```

#### Trait Implementations

##### `impl Default for Join`

- <span id="join-default"></span>`fn default() -> Join` — [`Join`](#join)

##### `impl Parse for Join`

- <span id="join-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

## Functions

### `bind_futures`

```rust
fn bind_futures(fut_exprs: Vec<syn::Expr>, span: proc_macro2::Span) -> (Vec<proc_macro2::TokenStream>, Vec<syn::Ident>)
```

### `join`

```rust
fn join(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

The `join!` macro.

### `try_join`

```rust
fn try_join(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

The `try_join!` macro.

