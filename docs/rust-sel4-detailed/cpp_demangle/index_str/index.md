*[cpp_demangle](../index.md) / [index_str](index.md)*

---

# Module `index_str`

Provides the `IndexStr` type to keep track of a substring's index into its
original string is.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IndexStr`](#indexstr) | struct | The `IndexStr` type allows us to take substrings from an original input and keep track of what index the substring is at in the original input. |

## Structs

### `IndexStr<'a>`

```rust
struct IndexStr<'a> {
    idx: usize,
    string: &'a [u8],
}
```

The `IndexStr` type allows us to take substrings from an original input and
keep track of what index the substring is at in the original input.

#### Implementations

- <span id="indexstr-new"></span>`fn new(string: &'a [u8]) -> IndexStr<'a>` — [`IndexStr`](#indexstr)

  Construct a new `IndexStr` (with `index == 0`) from the given input.

- <span id="indexstr-len"></span>`fn len(&self) -> usize`

  Return the length of the string.

- <span id="indexstr-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the string is empty, false otherwise.

- <span id="indexstr-index"></span>`fn index(&self) -> usize`

  Get the index into the original input that this `IndexStr` is at.

- <span id="indexstr-peek"></span>`fn peek(&self) -> Option<u8>`

  Peek at the next byte in this `IndexStr`.

- <span id="indexstr-peek-second"></span>`fn peek_second(&self) -> Option<u8>`

  Peek at the second next byte in this `IndexStr`.

- <span id="indexstr-split-at"></span>`fn split_at(&self, idx: usize) -> (IndexStr<'a>, IndexStr<'a>)` — [`IndexStr`](#indexstr)

  Split the string in two at the given index, resulting in the tuple where

  the first item has range `[0, idx)`, and the second has range `[idx,

  len)`.

  

  Panics if the index is out of bounds.

- <span id="indexstr-try-split-at"></span>`fn try_split_at(&self, idx: usize) -> Option<(IndexStr<'a>, IndexStr<'a>)>` — [`IndexStr`](#indexstr)

  The same as `split_at`, but returns an `Option` rather than panicking

  when the index is out of bounds.

- <span id="indexstr-next"></span>`fn next(&self) -> Option<(u8, IndexStr<'a>)>` — [`IndexStr`](#indexstr)

  Pop the next byte off the front of this string, returning it and the new

  tail string, or `None` if this string is empty.

- <span id="indexstr-next-or"></span>`fn next_or<E>(&self, error: E) -> Result<(u8, IndexStr<'a>), E>` — [`IndexStr`](#indexstr)

  Pop the next byte off the front of this string, returning it and the new

  tail string, or the given error if this string is empty.

#### Trait Implementations

##### `impl AsRef for IndexStr<'a>`

- <span id="indexstr-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for IndexStr<'a>`

- <span id="indexstr-clone"></span>`fn clone(&self) -> IndexStr<'a>` — [`IndexStr`](#indexstr)

##### `impl Copy for IndexStr<'a>`

##### `impl Debug for IndexStr<'a>`

- <span id="indexstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IndexStr<'a>`

##### `impl PartialEq for IndexStr<'a>`

- <span id="indexstr-partialeq-eq"></span>`fn eq(&self, other: &IndexStr<'a>) -> bool` — [`IndexStr`](#indexstr)

##### `impl StructuralPartialEq for IndexStr<'a>`

