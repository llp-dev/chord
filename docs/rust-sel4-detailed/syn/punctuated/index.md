*[syn](../index.md) / [punctuated](index.md)*

---

# Module `punctuated`

A punctuated sequence of syntax tree nodes separated by punctuation.

Lots of things in Rust are punctuated sequences.

- The fields of a struct are `Punctuated<Field, Token![,]>`.
- The segments of a path are `Punctuated<PathSegment, Token![::]>`.
- The bounds on a generic parameter are `Punctuated<TypeParamBound,
  Token![+]>`.
- The arguments to a function call are `Punctuated<Expr, Token![,]>`.

This module provides a common representation for these punctuated sequences
in the form of the `Punctuated<T, P>` type. We store a vector of pairs of
syntax tree node + punctuation, where every node in the sequence is followed
by punctuation except for possibly the final one.

```text
a_function_call(arg1, arg2, arg3);
                ~~~~^ ~~~~^ ~~~~
```

## Contents

- [Modules](#modules)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Punctuated`](#punctuated)
  - [`Pairs`](#pairs)
  - [`PairsMut`](#pairsmut)
  - [`IntoPairs`](#intopairs)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
  - [`PrivateIter`](#privateiter)
  - [`IterMut`](#itermut)
  - [`PrivateIterMut`](#privateitermut)
- [Enums](#enums)
  - [`Pair`](#pair)
- [Traits](#traits)
  - [`IterTrait`](#itertrait)
  - [`IterMutTrait`](#itermuttrait)
- [Functions](#functions)
  - [`do_extend`](#do-extend)
  - [`empty_punctuated_iter`](#empty-punctuated-iter)
  - [`empty_punctuated_iter_mut`](#empty-punctuated-iter-mut)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`printing`](#printing) | mod |  |
| [`Punctuated`](#punctuated) | struct | **A punctuated sequence of syntax tree nodes of type `T` separated by punctuation of type `P`.** |
| [`Pairs`](#pairs) | struct | An iterator over borrowed pairs of type `Pair<&T, &P>`. |
| [`PairsMut`](#pairsmut) | struct | An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`. |
| [`IntoPairs`](#intopairs) | struct | An iterator over owned pairs of type `Pair<T, P>`. |
| [`IntoIter`](#intoiter) | struct | An iterator over owned values of type `T`. |
| [`Iter`](#iter) | struct | An iterator over borrowed values of type `&T`. |
| [`PrivateIter`](#privateiter) | struct |  |
| [`IterMut`](#itermut) | struct | An iterator over mutably borrowed values of type `&mut T`. |
| [`PrivateIterMut`](#privateitermut) | struct |  |
| [`Pair`](#pair) | enum | A single syntax tree node of type `T` followed by its trailing punctuation of type `P` if any. |
| [`IterTrait`](#itertrait) | trait |  |
| [`IterMutTrait`](#itermuttrait) | trait |  |
| [`do_extend`](#do-extend) | fn |  |
| [`empty_punctuated_iter`](#empty-punctuated-iter) | fn |  |
| [`empty_punctuated_iter_mut`](#empty-punctuated-iter-mut) | fn |  |

## Modules

- [`printing`](printing/index.md)

## Structs

### `Punctuated<T, P>`

```rust
struct Punctuated<T, P> {
    inner: alloc::vec::Vec<(T, P)>,
    last: Option<alloc::boxed::Box<T>>,
}
```

**A punctuated sequence of syntax tree nodes of type `T` separated by
punctuation of type `P`.**

Refer to the [module documentation] for details about punctuated sequences.


#### Implementations

- <span id="punctuated-new"></span>`const fn new() -> Self`

  Creates an empty punctuated sequence.

- <span id="punctuated-is-empty"></span>`fn is_empty(&self) -> bool`

  Determines whether this punctuated sequence is empty, meaning it

  contains no syntax tree nodes or punctuation.

- <span id="punctuated-len"></span>`fn len(&self) -> usize`

  Returns the number of syntax tree nodes in this punctuated sequence.

  

  This is the number of nodes of type `T`, not counting the punctuation of

  type `P`.

- <span id="punctuated-first"></span>`fn first(&self) -> Option<&T>`

  Borrows the first element in this sequence.

- <span id="punctuated-first-mut"></span>`fn first_mut(&mut self) -> Option<&mut T>`

  Mutably borrows the first element in this sequence.

- <span id="punctuated-last"></span>`fn last(&self) -> Option<&T>`

  Borrows the last element in this sequence.

- <span id="punctuated-last-mut"></span>`fn last_mut(&mut self) -> Option<&mut T>`

  Mutably borrows the last element in this sequence.

- <span id="punctuated-get"></span>`fn get(&self, index: usize) -> Option<&T>`

  Borrows the element at the given index.

- <span id="punctuated-get-mut"></span>`fn get_mut(&mut self, index: usize) -> Option<&mut T>`

  Mutably borrows the element at the given index.

- <span id="punctuated-iter"></span>`fn iter(&self) -> Iter<'_, T>` — [`Iter`](#iter)

  Returns an iterator over borrowed syntax tree nodes of type `&T`.

- <span id="punctuated-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, T>` — [`IterMut`](#itermut)

  Returns an iterator over mutably borrowed syntax tree nodes of type

  `&mut T`.

- <span id="punctuated-pairs"></span>`fn pairs(&self) -> Pairs<'_, T, P>` — [`Pairs`](#pairs)

  Returns an iterator over the contents of this sequence as borrowed

  punctuated pairs.

- <span id="punctuated-pairs-mut"></span>`fn pairs_mut(&mut self) -> PairsMut<'_, T, P>` — [`PairsMut`](#pairsmut)

  Returns an iterator over the contents of this sequence as mutably

  borrowed punctuated pairs.

- <span id="punctuated-into-pairs"></span>`fn into_pairs(self) -> IntoPairs<T, P>` — [`IntoPairs`](#intopairs)

  Returns an iterator over the contents of this sequence as owned

  punctuated pairs.

- <span id="punctuated-push-value"></span>`fn push_value(&mut self, value: T)`

  Appends a syntax tree node onto the end of this punctuated sequence. The

  sequence must already have a trailing punctuation, or be empty.

  

  Use `push` instead if the punctuated sequence may or may not already

  have trailing punctuation.

  

  # Panics

  

  Panics if the sequence is nonempty and does not already have a trailing

  punctuation.

- <span id="punctuated-push-punct"></span>`fn push_punct(&mut self, punctuation: P)`

  Appends a trailing punctuation onto the end of this punctuated sequence.

  The sequence must be non-empty and must not already have trailing

  punctuation.

  

  # Panics

  

  Panics if the sequence is empty or already has a trailing punctuation.

- <span id="punctuated-pop"></span>`fn pop(&mut self) -> Option<Pair<T, P>>` — [`Pair`](#pair)

  Removes the last punctuated pair from this sequence, or `None` if the

  sequence is empty.

- <span id="punctuated-pop-punct"></span>`fn pop_punct(&mut self) -> Option<P>`

  Removes the trailing punctuation from this punctuated sequence, or

  `None` if there isn't any.

- <span id="punctuated-trailing-punct"></span>`fn trailing_punct(&self) -> bool`

  Determines whether this punctuated sequence ends with a trailing

  punctuation.

- <span id="punctuated-empty-or-trailing"></span>`fn empty_or_trailing(&self) -> bool`

  Returns true if either this `Punctuated` is empty, or it has a trailing

  punctuation.

  

  Equivalent to `punctuated.is_empty() || punctuated.trailing_punct()`.

- <span id="punctuated-push"></span>`fn push(&mut self, value: T)`

  Appends a syntax tree node onto the end of this punctuated sequence.

  

  If there is not a trailing punctuation in this sequence when this method

  is called, the default value of punctuation type `P` is inserted before

  the given value of type `T`.

- <span id="punctuated-insert"></span>`fn insert(&mut self, index: usize, value: T)`

  Inserts an element at position `index`.

  

  # Panics

  

  Panics if `index` is greater than the number of elements previously in

  this punctuated sequence.

- <span id="punctuated-clear"></span>`fn clear(&mut self)`

  Clears the sequence of all values and punctuation, making it empty.

- <span id="punctuated-parse-terminated"></span>`fn parse_terminated(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses zero or more occurrences of `T` separated by punctuation of type

  `P`, with optional trailing punctuation.

  

  Parsing continues until the end of this parse stream. The entire content

  of this parse stream must consist of `T` and `P`.

- <span id="punctuated-parse-terminated-with"></span>`fn parse_terminated_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses zero or more occurrences of `T` using the given parse function,

  separated by punctuation of type `P`, with optional trailing

  punctuation.

  

  Like `parse_terminated`, the entire content of this stream is expected

  to be parsed.

- <span id="punctuated-parse-separated-nonempty"></span>`fn parse_separated_nonempty(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses one or more occurrences of `T` separated by punctuation of type

  `P`, not accepting trailing punctuation.

  

  Parsing continues as long as punctuation `P` is present at the head of

  the stream. This method returns upon parsing a `T` and observing that it

  is not followed by a `P`, even if there are remaining tokens in the

  stream.

- <span id="punctuated-parse-separated-nonempty-with"></span>`fn parse_separated_nonempty_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses one or more occurrences of `T` using the given parse function,

  separated by punctuation of type `P`, not accepting trailing

  punctuation.

  

  Like `parse_separated_nonempty`, may complete early without parsing

  the entire content of this stream.

#### Trait Implementations

##### `impl<T, P> Clone for Punctuated<T, P>`

- <span id="punctuated-clone"></span>`fn clone(&self) -> Self`

- <span id="punctuated-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<T: Debug, P: Debug> Debug for Punctuated<T, P>`

- <span id="punctuated-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, P> Default for Punctuated<T, P>`

- <span id="punctuated-default"></span>`fn default() -> Self`

##### `impl<T, P> Eq for Punctuated<T, P>`

##### `impl<T, P> Extend for Punctuated<T, P>`

- <span id="punctuated-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, i: I)`

##### `impl<T, P> FromIterator for Punctuated<T, P>`

- <span id="punctuated-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(i: I) -> Self`

##### `impl<T, P> Hash for Punctuated<T, P>`

- <span id="punctuated-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T, P> Index for Punctuated<T, P>`

- <span id="punctuated-index-type-output"></span>`type Output = T`

- <span id="punctuated-index"></span>`fn index(&self, index: usize) -> &<Self as >::Output`

##### `impl<T, P> IndexMut for Punctuated<T, P>`

- <span id="punctuated-indexmut-index-mut"></span>`fn index_mut(&mut self, index: usize) -> &mut <Self as >::Output`

##### `impl<T, P> IntoIterator for Punctuated<T, P>`

- <span id="punctuated-intoiterator-type-item"></span>`type Item = T`

- <span id="punctuated-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<T>`

- <span id="punctuated-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, P> PartialEq for Punctuated<T, P>`

- <span id="punctuated-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Punctuated<T, P>`

##### `impl<T> Spanned for Punctuated<T, P>`

- <span id="punctuated-spanned-span"></span>`fn span(&self) -> Span`

##### `impl<T, P> ToTokens for crate::punctuated::Punctuated<T, P>`

- <span id="cratepunctuatedpunctuated-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Pairs<'a, T: 'a, P: 'a>`

```rust
struct Pairs<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: option::IntoIter<&'a T>,
}
```

An iterator over borrowed pairs of type `Pair<&T, &P>`.

Refer to the [module documentation] for details about punctuated sequences.


#### Trait Implementations

##### `impl<T, P> Clone for Pairs<'a, T, P>`

- <span id="pairs-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, P> DoubleEndedIterator for Pairs<'a, T, P>`

- <span id="pairs-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> ExactSizeIterator for Pairs<'a, T, P>`

- <span id="pairs-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for Pairs<'a, T, P>`

- <span id="pairs-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pairs-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pairs-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, P> Iterator for Pairs<'a, T, P>`

- <span id="pairs-iterator-type-item"></span>`type Item = Pair<&'a T, &'a P>`

- <span id="pairs-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="pairs-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `PairsMut<'a, T: 'a, P: 'a>`

```rust
struct PairsMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: option::IntoIter<&'a mut T>,
}
```

An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`.

Refer to the [module documentation] for details about punctuated sequences.


#### Trait Implementations

##### `impl<T, P> DoubleEndedIterator for PairsMut<'a, T, P>`

- <span id="pairsmut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> ExactSizeIterator for PairsMut<'a, T, P>`

- <span id="pairsmut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for PairsMut<'a, T, P>`

- <span id="pairsmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pairsmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pairsmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, P> Iterator for PairsMut<'a, T, P>`

- <span id="pairsmut-iterator-type-item"></span>`type Item = Pair<&'a mut T, &'a mut P>`

- <span id="pairsmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="pairsmut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IntoPairs<T, P>`

```rust
struct IntoPairs<T, P> {
    inner: vec::IntoIter<(T, P)>,
    last: option::IntoIter<T>,
}
```

An iterator over owned pairs of type `Pair<T, P>`.

Refer to the [module documentation] for details about punctuated sequences.


#### Trait Implementations

##### `impl<T, P> Clone for IntoPairs<T, P>`

- <span id="intopairs-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, P> DoubleEndedIterator for IntoPairs<T, P>`

- <span id="intopairs-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> ExactSizeIterator for IntoPairs<T, P>`

- <span id="intopairs-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for IntoPairs<T, P>`

- <span id="intopairs-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intopairs-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intopairs-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, P> Iterator for IntoPairs<T, P>`

- <span id="intopairs-iterator-type-item"></span>`type Item = Pair<T, P>`

- <span id="intopairs-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intopairs-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

An iterator over owned values of type `T`.

Refer to the [module documentation] for details about punctuated sequences.


#### Trait Implementations

##### `impl<T> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> DoubleEndedIterator for IntoIter<T>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IntoIter<T>`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for IntoIter<T>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IntoIter<T>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Iter<'a, T: 'a>`

```rust
struct Iter<'a, T: 'a> {
    inner: alloc::boxed::Box<crate::drops::NoDrop<dyn IterTrait<'a, T>>>,
}
```

An iterator over borrowed values of type `&T`.

Refer to the [module documentation] for details about punctuated sequences.


#### Trait Implementations

##### `impl<T> Clone for Iter<'a, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> DoubleEndedIterator for Iter<'a, T>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Iter<'a, T>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for Iter<'a, T>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Iter<'a, T>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a T`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `PrivateIter<'a, T: 'a, P: 'a>`

```rust
struct PrivateIter<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: option::IntoIter<&'a T>,
}
```

#### Trait Implementations

##### `impl<T, P> Clone for PrivateIter<'a, T, P>`

- <span id="privateiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, P> DoubleEndedIterator for PrivateIter<'a, T, P>`

- <span id="privateiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> ExactSizeIterator for PrivateIter<'a, T, P>`

- <span id="privateiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for PrivateIter<'a, T, P>`

- <span id="privateiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="privateiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="privateiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, P> Iterator for PrivateIter<'a, T, P>`

- <span id="privateiter-iterator-type-item"></span>`type Item = &'a T`

- <span id="privateiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> TrivialDrop for PrivateIter<'a, T, P>`

### `IterMut<'a, T: 'a>`

```rust
struct IterMut<'a, T: 'a> {
    inner: alloc::boxed::Box<crate::drops::NoDrop<dyn IterMutTrait<'a, T, Item = &'a mut T>>>,
}
```

An iterator over mutably borrowed values of type `&mut T`.

Refer to the [module documentation] for details about punctuated sequences.


#### Trait Implementations

##### `impl<T> DoubleEndedIterator for IterMut<'a, T>`

- <span id="itermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IterMut<'a, T>`

- <span id="itermut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for IterMut<'a, T>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterMut<'a, T>`

- <span id="itermut-iterator-type-item"></span>`type Item = &'a mut T`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `PrivateIterMut<'a, T: 'a, P: 'a>`

```rust
struct PrivateIterMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: option::IntoIter<&'a mut T>,
}
```

#### Trait Implementations

##### `impl<T, P> DoubleEndedIterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> ExactSizeIterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="privateitermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="privateitermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, P> Iterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-iterator-type-item"></span>`type Item = &'a mut T`

- <span id="privateitermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> TrivialDrop for PrivateIterMut<'a, T, P>`

## Enums

### `Pair<T, P>`

```rust
enum Pair<T, P> {
    Punctuated(T, P),
    End(T),
}
```

A single syntax tree node of type `T` followed by its trailing punctuation
of type `P` if any.

Refer to the [module documentation] for details about punctuated sequences.


#### Implementations

- <span id="pair-into-value"></span>`fn into_value(self) -> T`

  Extracts the syntax tree node from this punctuated pair, discarding the

  following punctuation.

- <span id="pair-value"></span>`fn value(&self) -> &T`

  Borrows the syntax tree node from this punctuated pair.

- <span id="pair-value-mut"></span>`fn value_mut(&mut self) -> &mut T`

  Mutably borrows the syntax tree node from this punctuated pair.

- <span id="pair-punct"></span>`fn punct(&self) -> Option<&P>`

  Borrows the punctuation from this punctuated pair, unless this pair is

  the final one and there is no trailing punctuation.

- <span id="pair-punct-mut"></span>`fn punct_mut(&mut self) -> Option<&mut P>`

  Mutably borrows the punctuation from this punctuated pair, unless the

  pair is the final one and there is no trailing punctuation.

  

  # Example

  

  ```rust

  use proc_macro2::Span;

  use syn::punctuated::Punctuated;

  use syn::{parse_quote, Token, TypeParamBound};

  

  let mut punctuated = Punctuated::<TypeParamBound, Token![+]>::new();

  let span = Span::call_site();

  

  punctuated.insert(0, parse_quote!('lifetime));

  if let Some(punct) = punctuated.pairs_mut().next().unwrap().punct_mut() {

      punct.span = span;

  }

  ```

- <span id="pair-new"></span>`fn new(t: T, p: Option<P>) -> Self`

  Creates a punctuated pair out of a syntax tree node and an optional

  following punctuation.

- <span id="pair-into-tuple"></span>`fn into_tuple(self) -> (T, Option<P>)`

  Produces this punctuated pair as a tuple of syntax tree node and

  optional following punctuation.

#### Trait Implementations

##### `impl<T, P> Clone for Pair<T, P>`

- <span id="pair-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, P> Copy for Pair<T, P>`

##### `impl<T, P> Extend for Punctuated<T, P>`

- <span id="punctuated-extend"></span>`fn extend<I: IntoIterator<Item = Pair<T, P>>>(&mut self, i: I)`

##### `impl<T, P> FromIterator for Punctuated<T, P>`

- <span id="punctuated-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = Pair<T, P>>>(i: I) -> Self`

##### `impl<T> Sealed for Pair<T, P>`

##### `impl<T> Spanned for Pair<T, P>`

- <span id="pair-spanned-span"></span>`fn span(&self) -> Span`

##### `impl<T, P> ToTokens for crate::punctuated::Pair<T, P>`

- <span id="cratepunctuatedpair-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Traits

### `IterTrait<'a, T: 'a>`

```rust
trait IterTrait<'a, T: 'a>: Iterator<Item = &'a T> + DoubleEndedIterator + ExactSizeIterator { ... }
```

#### Required Methods

- `fn clone_box(&self) -> Box<NoDrop<dyn IterTrait<'a, T>>>`

#### Implementors

- `I`

### `IterMutTrait<'a, T: 'a>`

```rust
trait IterMutTrait<'a, T: 'a>: DoubleEndedIterator<Item = &'a mut T> + ExactSizeIterator<Item = &'a mut T> { ... }
```

#### Implementors

- `I`

## Functions

### `do_extend`

```rust
fn do_extend<T, P, I>(punctuated: &mut Punctuated<T, P>, i: I)
where
    I: Iterator<Item = Pair<T, P>>
```

### `empty_punctuated_iter`

```rust
fn empty_punctuated_iter<'a, T>() -> Iter<'a, T>
```

### `empty_punctuated_iter_mut`

```rust
fn empty_punctuated_iter_mut<'a, T>() -> IterMut<'a, T>
```

