*[rkyv_derive](../../index.md) / [archive](../index.md) / [printing](index.md)*

---

# Module `printing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Printing`](#printing) | struct |  |

## Structs

### `Printing`

```rust
struct Printing {
    pub rkyv_path: syn::Path,
    pub vis: syn::Visibility,
    pub name: syn::Ident,
    pub archived_name: syn::Ident,
    pub archived_type: syn::Type,
    pub resolver_name: syn::Ident,
    pub archived_metas: Vec<syn::Meta>,
}
```

#### Implementations

- <span id="printing-new"></span>`fn new(input: &DeriveInput, attributes: &Attributes) -> Result<Self, Error>` — [`Attributes`](../../attributes/index.md#attributes)

