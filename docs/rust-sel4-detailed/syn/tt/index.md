*[syn](../index.md) / [tt](index.md)*

---

# Module `tt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenTreeHelper`](#tokentreehelper) | struct |  |
| [`TokenStreamHelper`](#tokenstreamhelper) | struct |  |

## Structs

### `TokenTreeHelper<'a>`

```rust
struct TokenTreeHelper<'a>(&'a proc_macro2::TokenTree);
```

#### Trait Implementations

##### `impl Hash for TokenTreeHelper<'a>`

- <span id="tokentreehelper-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl PartialEq for TokenTreeHelper<'a>`

- <span id="tokentreehelper-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `TokenStreamHelper<'a>`

```rust
struct TokenStreamHelper<'a>(&'a proc_macro2::TokenStream);
```

#### Trait Implementations

##### `impl Hash for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

