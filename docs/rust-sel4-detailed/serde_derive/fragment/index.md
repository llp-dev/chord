*[serde_derive](../index.md) / [fragment](index.md)*

---

# Module `fragment`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Expr`](#expr) | struct | Interpolate a fragment in place of an expression. |
| [`Stmts`](#stmts) | struct | Interpolate a fragment as the statements of a block. |
| [`Match`](#match) | struct | Interpolate a fragment as the value part of a `match` expression. |
| [`Fragment`](#fragment) | enum |  |
| [`quote_expr!`](#quote-expr) | macro |  |
| [`quote_block!`](#quote-block) | macro |  |

## Structs

### `Expr`

```rust
struct Expr(Fragment);
```

Interpolate a fragment in place of an expression. This involves surrounding
Block fragments in curly braces.

#### Trait Implementations

##### `impl Spanned for Expr`

- <span id="expr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Expr`

- <span id="expr-totokens-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

### `Stmts`

```rust
struct Stmts(Fragment);
```

Interpolate a fragment as the statements of a block.

#### Trait Implementations

##### `impl Spanned for Stmts`

- <span id="stmts-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Stmts`

- <span id="stmts-totokens-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

### `Match`

```rust
struct Match(Fragment);
```

Interpolate a fragment as the value part of a `match` expression. This
involves putting a comma after expressions and curly braces around blocks.

#### Trait Implementations

##### `impl Spanned for Match`

- <span id="match-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Match`

- <span id="match-totokens-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

## Enums

### `Fragment`

```rust
enum Fragment {
    Expr(proc_macro2::TokenStream),
    Block(proc_macro2::TokenStream),
}
```

#### Variants

- **`Expr`**

  Tokens that can be used as an expression.

- **`Block`**

  Tokens that can be used inside a block. The surrounding curly braces are
  not part of these tokens.

#### Trait Implementations

##### `impl AsRef for Fragment`

- <span id="fragment-asref-as-ref"></span>`fn as_ref(&self) -> &TokenStream`

## Macros

### `quote_expr!`

### `quote_block!`

