**syn > punctuated**

# Module: punctuated

## Contents

**Modules**

- [`printing`](#printing)

**Structs**

- [`IntoIter`](#intoiter) - An iterator over owned values of type `T`.
- [`IntoPairs`](#intopairs) - An iterator over owned pairs of type `Pair<T, P>`.
- [`Iter`](#iter) - An iterator over borrowed values of type `&T`.
- [`IterMut`](#itermut) - An iterator over mutably borrowed values of type `&mut T`.
- [`Pairs`](#pairs) - An iterator over borrowed pairs of type `Pair<&T, &P>`.
- [`PairsMut`](#pairsmut) - An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`.
- [`PrivateIter`](#privateiter)
- [`PrivateIterMut`](#privateitermut)
- [`Punctuated`](#punctuated) - **A punctuated sequence of syntax tree nodes of type `T` separated by

**Enums**

- [`Pair`](#pair) - A single syntax tree node of type `T` followed by its trailing punctuation

**Functions**

- [`do_extend`](#do_extend)
- [`empty_punctuated_iter`](#empty_punctuated_iter)
- [`empty_punctuated_iter_mut`](#empty_punctuated_iter_mut)

**Traits**

- [`IterMutTrait`](#itermuttrait)
- [`IterTrait`](#itertrait)

---

## syn::punctuated::IntoIter

*Struct*

An iterator over owned values of type `T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- T

**Fields:**
- `inner: vec::IntoIter<T>`

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::punctuated::IntoPairs

*Struct*

An iterator over owned pairs of type `Pair<T, P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- T
- P

**Fields:**
- `inner: vec::IntoIter<(T, P)>`
- `last: option::IntoIter<T>`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



## syn::punctuated::Iter

*Struct*

An iterator over borrowed values of type `&T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- 'a
- T

**Fields:**
- `inner: alloc::boxed::Box<crate::drops::NoDrop<dyn IterTrait>>`

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::punctuated::IterMut

*Struct*

An iterator over mutably borrowed values of type `&mut T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- 'a
- T

**Fields:**
- `inner: alloc::boxed::Box<crate::drops::NoDrop<dyn IterMutTrait>>`

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## syn::punctuated::IterMutTrait

*Trait*



## syn::punctuated::IterTrait

*Trait*

**Methods:**

- `clone_box`



## syn::punctuated::Pair

*Enum*

A single syntax tree node of type `T` followed by its trailing punctuation
of type `P` if any.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- T
- P

**Variants:**
- `Punctuated(T, P)`
- `End(T)`

**Methods:**

- `fn cloned(self: Self) -> Pair<T, P>`
- `fn into_value(self: Self) -> T` - Extracts the syntax tree node from this punctuated pair, discarding the
- `fn value(self: &Self) -> &T` - Borrows the syntax tree node from this punctuated pair.
- `fn value_mut(self: & mut Self) -> & mut T` - Mutably borrows the syntax tree node from this punctuated pair.
- `fn punct(self: &Self) -> Option<&P>` - Borrows the punctuation from this punctuated pair, unless this pair is
- `fn punct_mut(self: & mut Self) -> Option<& mut P>` - Mutably borrows the punctuation from this punctuated pair, unless the
- `fn new(t: T, p: Option<P>) -> Self` - Creates a punctuated pair out of a syntax tree node and an optional
- `fn into_tuple(self: Self) -> (T, Option<P>)` - Produces this punctuated pair as a tuple of syntax tree node and

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::punctuated::Pairs

*Struct*

An iterator over borrowed pairs of type `Pair<&T, &P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- 'a
- T
- P

**Fields:**
- `inner: slice::Iter<'a, (T, P)>`
- `last: option::IntoIter<&'a T>`

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## syn::punctuated::PairsMut

*Struct*

An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- 'a
- T
- P

**Fields:**
- `inner: slice::IterMut<'a, (T, P)>`
- `last: option::IntoIter<&'a  mut T>`

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## syn::punctuated::PrivateIter

*Struct*

**Generic Parameters:**
- 'a
- T
- P

**Fields:**
- `inner: slice::Iter<'a, (T, P)>`
- `last: option::IntoIter<&'a T>`

**Traits:** TrivialDrop

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



## syn::punctuated::PrivateIterMut

*Struct*

**Generic Parameters:**
- 'a
- T
- P

**Fields:**
- `inner: slice::IterMut<'a, (T, P)>`
- `last: option::IntoIter<&'a  mut T>`

**Traits:** TrivialDrop

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## syn::punctuated::Punctuated

*Struct*

**A punctuated sequence of syntax tree nodes of type `T` separated by
punctuation of type `P`.**

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

**Generic Parameters:**
- T
- P

**Fields:**
- `inner: alloc::vec::Vec<(T, P)>`
- `last: Option<alloc::boxed::Box<T>>`

**Methods:**

- `fn new() -> Self` - Creates an empty punctuated sequence.
- `fn is_empty(self: &Self) -> bool` - Determines whether this punctuated sequence is empty, meaning it
- `fn len(self: &Self) -> usize` - Returns the number of syntax tree nodes in this punctuated sequence.
- `fn first(self: &Self) -> Option<&T>` - Borrows the first element in this sequence.
- `fn first_mut(self: & mut Self) -> Option<& mut T>` - Mutably borrows the first element in this sequence.
- `fn last(self: &Self) -> Option<&T>` - Borrows the last element in this sequence.
- `fn last_mut(self: & mut Self) -> Option<& mut T>` - Mutably borrows the last element in this sequence.
- `fn get(self: &Self, index: usize) -> Option<&T>` - Borrows the element at the given index.
- `fn get_mut(self: & mut Self, index: usize) -> Option<& mut T>` - Mutably borrows the element at the given index.
- `fn iter(self: &Self) -> Iter<T>` - Returns an iterator over borrowed syntax tree nodes of type `&T`.
- `fn iter_mut(self: & mut Self) -> IterMut<T>` - Returns an iterator over mutably borrowed syntax tree nodes of type
- `fn pairs(self: &Self) -> Pairs<T, P>` - Returns an iterator over the contents of this sequence as borrowed
- `fn pairs_mut(self: & mut Self) -> PairsMut<T, P>` - Returns an iterator over the contents of this sequence as mutably
- `fn into_pairs(self: Self) -> IntoPairs<T, P>` - Returns an iterator over the contents of this sequence as owned
- `fn push_value(self: & mut Self, value: T)` - Appends a syntax tree node onto the end of this punctuated sequence. The
- `fn push_punct(self: & mut Self, punctuation: P)` - Appends a trailing punctuation onto the end of this punctuated sequence.
- `fn pop(self: & mut Self) -> Option<Pair<T, P>>` - Removes the last punctuated pair from this sequence, or `None` if the
- `fn pop_punct(self: & mut Self) -> Option<P>` - Removes the trailing punctuation from this punctuated sequence, or
- `fn trailing_punct(self: &Self) -> bool` - Determines whether this punctuated sequence ends with a trailing
- `fn empty_or_trailing(self: &Self) -> bool` - Returns true if either this `Punctuated` is empty, or it has a trailing
- `fn push(self: & mut Self, value: T)` - Appends a syntax tree node onto the end of this punctuated sequence.
- `fn insert(self: & mut Self, index: usize, value: T)` - Inserts an element at position `index`.
- `fn clear(self: & mut Self)` - Clears the sequence of all values and punctuation, making it empty.
- `fn parse_terminated(input: ParseStream) -> Result<Self>` - Parses zero or more occurrences of `T` separated by punctuation of type
- `fn parse_terminated_with<'a>(input: ParseStream<'a>, parser: fn(...)) -> Result<Self>` - Parses zero or more occurrences of `T` using the given parse function,
- `fn parse_separated_nonempty(input: ParseStream) -> Result<Self>` - Parses one or more occurrences of `T` separated by punctuation of type
- `fn parse_separated_nonempty_with<'a>(input: ParseStream<'a>, parser: fn(...)) -> Result<Self>` - Parses one or more occurrences of `T` using the given parse function,

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Index**
  - `fn index(self: &Self, index: usize) -> &<Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
  - `fn clone_from(self: & mut Self, other: &Self)`
- **Extend**
  - `fn extend<I>(self: & mut Self, i: I)`
- **FromIterator**
  - `fn from_iter<I>(i: I) -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: usize) -> & mut <Self as >::Output`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **FromIterator**
  - `fn from_iter<I>(i: I) -> Self`
- **Extend**
  - `fn extend<I>(self: & mut Self, i: I)`



## syn::punctuated::do_extend

*Function*

```rust
fn do_extend<T, P, I>(punctuated: & mut Punctuated<T, P>, i: I)
```



## syn::punctuated::empty_punctuated_iter

*Function*

```rust
fn empty_punctuated_iter<'a, T>() -> Iter<'a, T>
```



## syn::punctuated::empty_punctuated_iter_mut

*Function*

```rust
fn empty_punctuated_iter_mut<'a, T>() -> IterMut<'a, T>
```



## Module: printing



