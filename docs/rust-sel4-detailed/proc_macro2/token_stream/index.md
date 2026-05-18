*[proc_macro2](../index.md) / [token_stream](index.md)*

---

# Module `token_stream`

Public implementation details for the `TokenStream` type, such as iterators.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenStream`](#tokenstream) | struct |  |
| [`IntoIter`](#intoiter) | struct | An iterator over `TokenStream`'s `TokenTree`s. |

## Structs

### `TokenStream`

```rust
struct TokenStream {
    inner: imp::TokenStream,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

#### Implementations

- <span id="tokenstream-new"></span>`fn _new(inner: imp::TokenStream) -> Self` — [`TokenStream`](../imp/index.md#tokenstream)

- <span id="tokenstream-new-fallback"></span>`fn _new_fallback(inner: fallback::TokenStream) -> Self`

- <span id="tokenstream-new"></span>`fn new() -> Self`

  Returns an empty `TokenStream` containing no token trees.

- <span id="tokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

  Checks if this `TokenStream` is empty.

#### Trait Implementations

##### `impl Clone for TokenStream`

- <span id="tokenstream-clone"></span>`fn clone(&self) -> TokenStream` — [`TokenStream`](../index.md#tokenstream)

##### `impl Debug for TokenStream`

- <span id="tokenstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TokenStream`

- <span id="tokenstream-default"></span>`fn default() -> Self`

##### `impl Display for TokenStream`

- <span id="tokenstream-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, tokens: I)`

##### `impl FromIterator for TokenStream`

- <span id="tokenstream-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = TokenTree>>(tokens: I) -> Self`

##### `impl FromStr for TokenStream`

- <span id="tokenstream-fromstr-type-err"></span>`type Err = LexError`

- <span id="tokenstream-fromstr-from-str"></span>`fn from_str(src: &str) -> Result<TokenStream, LexError>` — [`TokenStream`](../index.md#tokenstream), [`LexError`](../index.md#lexerror)

##### `impl IntoIterator for TokenStream`

- <span id="tokenstream-intoiterator-type-item"></span>`type Item = TokenTree`

- <span id="tokenstream-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="tokenstream-intoiterator-into-iter"></span>`fn into_iter(self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl IntoTokenStream for proc_macro2::TokenStream`

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl ToString for TokenStream`

- <span id="tokenstream-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-totokens-raw-string"></span>`fn raw_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md#cursor), [`Reject`](../parse/index.md#reject)

- <span id="proc-macro2tokenstream-totokens-byte-string"></span>`fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md#cursor), [`Reject`](../parse/index.md#reject)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-tokenstreamext-borrow"></span>`fn borrow(&self) -> &T`

### `IntoIter`

```rust
struct IntoIter {
    inner: imp::TokenTreeIter,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

An iterator over `TokenStream`'s `TokenTree`s.

The iteration is "shallow", e.g. the iterator doesn't recurse into
delimited groups, and returns whole groups as token trees.

#### Trait Implementations

##### `impl Clone for IntoIter`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl Debug for IntoIter`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-iterator-type-item"></span>`type Item = TokenTree`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<TokenTree>` — [`TokenTree`](../index.md#tokentree)

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

