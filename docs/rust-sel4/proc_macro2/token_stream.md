**proc_macro2 > token_stream**

# Module: token_stream

## Contents

**Structs**

- [`IntoIter`](#intoiter) - An iterator over `TokenStream`'s `TokenTree`s.

---

## proc_macro2::token_stream::IntoIter

*Struct*

An iterator over `TokenStream`'s `TokenTree`s.

The iteration is "shallow", e.g. the iterator doesn't recurse into
delimited groups, and returns whole groups as token trees.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<TokenTree>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> IntoIter`



