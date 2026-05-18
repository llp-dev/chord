*[futures_macro](../../index.md) / [select](../index.md) / [kw](index.md)*

---

# Module `kw`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`complete`](#complete) | struct |  |

## Structs

### `complete`

```rust
struct complete {
    pub span: __private::Span,
}
```

#### Trait Implementations

##### `impl Clone for complete`

- <span id="complete-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for complete`

##### `impl Debug for complete`

- <span id="complete-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for complete`

- <span id="complete-default"></span>`fn default() -> Self`

##### `impl Eq for complete`

##### `impl Hash for complete`

- <span id="complete-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for complete`

- <span id="complete-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<complete>` — [`complete`](#complete)

##### `impl PartialEq for complete`

- <span id="complete-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for complete`

- <span id="complete-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for complete`

- <span id="complete-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for complete`

- <span id="complete-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="complete-token-display"></span>`fn display() -> &'static str`

