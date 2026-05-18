*[num_enum_derive](../../index.md) / [enum_attributes](../index.md) / [kw](index.md)*

---

# Module `kw`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`constructor`](#constructor) | struct |  |
| [`error_type`](#error-type) | struct |  |
| [`name`](#name) | struct |  |

## Structs

### `constructor`

```rust
struct constructor {
    pub span: __private::Span,
}
```

#### Trait Implementations

##### `impl Clone for constructor`

- <span id="constructor-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for constructor`

##### `impl Debug for constructor`

- <span id="constructor-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for constructor`

- <span id="constructor-default"></span>`fn default() -> Self`

##### `impl Eq for constructor`

##### `impl Hash for constructor`

- <span id="constructor-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for constructor`

- <span id="constructor-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<constructor>` — [`constructor`](#constructor)

##### `impl PartialEq for constructor`

- <span id="constructor-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for constructor`

- <span id="constructor-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for constructor`

- <span id="constructor-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for constructor`

- <span id="constructor-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="constructor-token-display"></span>`fn display() -> &'static str`

### `error_type`

```rust
struct error_type {
    pub span: __private::Span,
}
```

#### Trait Implementations

##### `impl Clone for error_type`

- <span id="error-type-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for error_type`

##### `impl Debug for error_type`

- <span id="error-type-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for error_type`

- <span id="error-type-default"></span>`fn default() -> Self`

##### `impl Eq for error_type`

##### `impl Hash for error_type`

- <span id="error-type-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for error_type`

- <span id="error-type-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<error_type>` — [`error_type`](#error-type)

##### `impl PartialEq for error_type`

- <span id="error-type-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for error_type`

- <span id="error-type-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for error_type`

- <span id="error-type-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for error_type`

- <span id="error-type-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="error-type-token-display"></span>`fn display() -> &'static str`

### `name`

```rust
struct name {
    pub span: __private::Span,
}
```

#### Trait Implementations

##### `impl Clone for name`

- <span id="name-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for name`

##### `impl Debug for name`

- <span id="name-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for name`

- <span id="name-default"></span>`fn default() -> Self`

##### `impl Eq for name`

##### `impl Hash for name`

- <span id="name-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for name`

- <span id="name-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<name>` — [`name`](#name)

##### `impl PartialEq for name`

- <span id="name-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for name`

- <span id="name-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for name`

- <span id="name-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for name`

- <span id="name-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="name-token-display"></span>`fn display() -> &'static str`

