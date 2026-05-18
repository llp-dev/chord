*[rustversion](../index.md) / [iter](index.md)*

---

# Module `iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IterImpl`](#iterimpl) | struct |  |
| [`new`](#new) | fn |  |
| [`Iter`](#iter) | type |  |

## Structs

### `IterImpl`

```rust
struct IterImpl {
    stack: Vec<token_stream::IntoIter>,
    peeked: Option<proc_macro::TokenTree>,
}
```

#### Implementations

- <span id="iterimpl-peek"></span>`fn peek(&mut self) -> Option<&TokenTree>`

#### Trait Implementations

##### `impl IntoIterator for IterImpl`

- <span id="iterimpl-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterimpl-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterimpl-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IterImpl`

- <span id="iterimpl-iterator-type-item"></span>`type Item = TokenTree`

- <span id="iterimpl-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Functions

### `new`

```rust
fn new(tokens: proc_macro::TokenStream) -> IterImpl
```

## Type Aliases

### `Iter<'a>`

```rust
type Iter<'a> = &'a mut IterImpl;
```

