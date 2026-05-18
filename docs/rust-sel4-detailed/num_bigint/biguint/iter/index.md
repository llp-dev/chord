*[num_bigint](../../index.md) / [biguint](../index.md) / [iter](index.md)*

---

# Module `iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`U32Digits`](#u32digits) | struct | An iterator of `u32` digits representation of a `BigUint` or `BigInt`, ordered least significant digit first. |
| [`U64Digits`](#u64digits) | struct | An iterator of `u64` digits representation of a `BigUint` or `BigInt`, ordered least significant digit first. |

## Structs

### `U32Digits<'a>`

```rust
struct U32Digits<'a> {
    data: &'a [u64],
    next_is_lo: bool,
    last_hi_is_zero: bool,
}
```

An iterator of `u32` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

#### Implementations

- <span id="u32digits-new"></span>`fn new(data: &'a [u64]) -> Self`

#### Trait Implementations

##### `impl DoubleEndedIterator for U32Digits<'_>`

- <span id="u32digits-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for U32Digits<'_>`

- <span id="u32digits-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for U32Digits<'_>`

##### `impl IntoIterator for U32Digits<'a>`

- <span id="u32digits-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="u32digits-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="u32digits-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for U32Digits<'_>`

- <span id="u32digits-iterator-type-item"></span>`type Item = u32`

- <span id="u32digits-iterator-next"></span>`fn next(&mut self) -> Option<u32>`

- <span id="u32digits-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="u32digits-iterator-last"></span>`fn last(self) -> Option<u32>`

- <span id="u32digits-iterator-count"></span>`fn count(self) -> usize`

### `U64Digits<'a>`

```rust
struct U64Digits<'a> {
    it: core::slice::Iter<'a, u64>,
}
```

An iterator of `u64` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

#### Implementations

- <span id="u64digits-new"></span>`fn new(data: &'a [u64]) -> Self`

#### Trait Implementations

##### `impl DoubleEndedIterator for U64Digits<'_>`

- <span id="u64digits-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for U64Digits<'_>`

- <span id="u64digits-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for U64Digits<'_>`

##### `impl IntoIterator for U64Digits<'a>`

- <span id="u64digits-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="u64digits-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="u64digits-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for U64Digits<'_>`

- <span id="u64digits-iterator-type-item"></span>`type Item = u64`

- <span id="u64digits-iterator-next"></span>`fn next(&mut self) -> Option<u64>`

- <span id="u64digits-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="u64digits-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<u64>`

- <span id="u64digits-iterator-last"></span>`fn last(self) -> Option<u64>`

- <span id="u64digits-iterator-count"></span>`fn count(self) -> usize`

