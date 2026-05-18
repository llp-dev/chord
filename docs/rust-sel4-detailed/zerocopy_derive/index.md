# Crate `zerocopy_derive`

Derive macros for [`zerocopy`](../zerocopy/index.md)'s traits.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive`](#derive) | mod |  |
| [`repr`](#repr) | mod |  |
| [`util`](#util) | mod |  |
| [`IntoTokenStream`](#intotokenstream) | trait |  |
| [`ident!`](#ident) | macro |  |
| [`derive!`](#derive) | macro | Defines a derive function named `$outer` which parses its input `TokenStream` as a `DeriveInput` and then invokes the `$inner` function. |

## Modules

- [`derive`](derive/index.md)
- [`repr`](repr/index.md)
- [`util`](util/index.md)

## Traits

### `IntoTokenStream`

```rust
trait IntoTokenStream { ... }
```

#### Required Methods

- `fn into_ts(self) -> proc_macro2::TokenStream`

#### Implementors

- `Result<proc_macro2::TokenStream, syn::Error>`
- `proc_macro2::TokenStream`

## Macros

### `ident!`

### `derive!`

Defines a derive function named `$outer` which parses its input
`TokenStream` as a `DeriveInput` and then invokes the `$inner` function.

Note that the separate `$outer` parameter is required - proc macro functions
are currently required to live at the crate root, and so the caller must
specify the name in order to avoid name collisions.

