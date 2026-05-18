*[num_enum_derive](../../index.md) / [variant_attributes](../index.md) / [kw](index.md)*

---

# Module `kw`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`default`](#default) | struct |  |
| [`catch_all`](#catch-all) | struct |  |
| [`alternatives`](#alternatives) | struct |  |

## Structs

### `default`

```rust
struct default {
    pub span: __private::Span,
}
```

#### Trait Implementations

##### `impl Clone for default`

- <span id="default-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for default`

##### `impl Debug for default`

- <span id="default-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for default`

- <span id="default-default"></span>`fn default() -> Self`

##### `impl Eq for default`

##### `impl Hash for default`

- <span id="default-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for default`

- <span id="default-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<default>` — [`default`](#default)

##### `impl PartialEq for default`

- <span id="default-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for default`

- <span id="default-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for default`

- <span id="default-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for default`

- <span id="default-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="default-token-display"></span>`fn display() -> &'static str`

### `catch_all`

```rust
struct catch_all {
    pub span: __private::Span,
}
```

#### Trait Implementations

##### `impl Clone for catch_all`

- <span id="catch-all-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for catch_all`

##### `impl Debug for catch_all`

- <span id="catch-all-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for catch_all`

- <span id="catch-all-default"></span>`fn default() -> Self`

##### `impl Eq for catch_all`

##### `impl Hash for catch_all`

- <span id="catch-all-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for catch_all`

- <span id="catch-all-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<catch_all>` — [`catch_all`](#catch-all)

##### `impl PartialEq for catch_all`

- <span id="catch-all-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for catch_all`

- <span id="catch-all-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for catch_all`

- <span id="catch-all-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for catch_all`

- <span id="catch-all-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="catch-all-token-display"></span>`fn display() -> &'static str`

### `alternatives`

```rust
struct alternatives {
    pub span: __private::Span,
}
```

#### Trait Implementations

##### `impl Clone for alternatives`

- <span id="alternatives-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for alternatives`

##### `impl Debug for alternatives`

- <span id="alternatives-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for alternatives`

- <span id="alternatives-default"></span>`fn default() -> Self`

##### `impl Eq for alternatives`

##### `impl Hash for alternatives`

- <span id="alternatives-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for alternatives`

- <span id="alternatives-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<alternatives>` — [`alternatives`](#alternatives)

##### `impl PartialEq for alternatives`

- <span id="alternatives-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for alternatives`

- <span id="alternatives-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for alternatives`

- <span id="alternatives-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for alternatives`

- <span id="alternatives-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="alternatives-token-display"></span>`fn display() -> &'static str`

