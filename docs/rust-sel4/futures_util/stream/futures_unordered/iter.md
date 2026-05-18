**futures_util > stream > futures_unordered > iter**

# Module: stream::futures_unordered::iter

## Contents

**Structs**

- [`IntoIter`](#intoiter) - Owned iterator over all futures in the unordered set.
- [`Iter`](#iter) - Immutable iterator over all the futures in the unordered set.
- [`IterMut`](#itermut) - Mutable iterator over all futures in the unordered set.
- [`IterPinMut`](#iterpinmut) - Mutable iterator over all futures in the unordered set.
- [`IterPinRef`](#iterpinref) - Immutable iterator over all futures in the unordered set.

---

## futures_util::stream::futures_unordered::iter::IntoIter

*Struct*

Owned iterator over all futures in the unordered set.

**Generic Parameters:**
- Fut

**Traits:** Send, ExactSizeIterator, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## futures_util::stream::futures_unordered::iter::Iter

*Struct*

Immutable iterator over all the futures in the unordered set.

**Generic Parameters:**
- 'a
- Fut

**Tuple Struct**: `()`

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::futures_unordered::iter::IterMut

*Struct*

Mutable iterator over all futures in the unordered set.

**Generic Parameters:**
- 'a
- Fut

**Tuple Struct**: `()`

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::futures_unordered::iter::IterPinMut

*Struct*

Mutable iterator over all futures in the unordered set.

**Generic Parameters:**
- 'a
- Fut

**Traits:** ExactSizeIterator, Sync, Send

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::futures_unordered::iter::IterPinRef

*Struct*

Immutable iterator over all futures in the unordered set.

**Generic Parameters:**
- 'a
- Fut

**Traits:** Sync, Send, ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



