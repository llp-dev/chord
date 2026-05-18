*[ring](../../index.md) / [polyfill](../index.md) / [leading_zeros_skipped](index.md)*

---

# Module `leading_zeros_skipped`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LeadingZerosStripped`](#leadingzerosstripped) | struct | An iterator that skips all leading zeros. |

## Structs

### `LeadingZerosStripped<I>`

```rust
struct LeadingZerosStripped<I>
where
    I: Iterator {
    inner: core::iter::Peekable<I>,
}
```

An iterator that skips all leading zeros.

When the wrapped iterator is all zeros, then the last item is retained.

#### Implementations

- <span id="leadingzerosstripped-new"></span>`fn new(inner: I) -> Self`

#### Trait Implementations

##### `impl<I> Clone for LeadingZerosStripped<I>`

- <span id="leadingzerosstripped-clone"></span>`fn clone(&self) -> Self`

##### `impl<I> ExactSizeIterator for LeadingZerosStripped<I>`

##### `impl<I> IntoIterator for LeadingZerosStripped<I>`

- <span id="leadingzerosstripped-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="leadingzerosstripped-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="leadingzerosstripped-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for LeadingZerosStripped<I>`

- <span id="leadingzerosstripped-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="leadingzerosstripped-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="leadingzerosstripped-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

