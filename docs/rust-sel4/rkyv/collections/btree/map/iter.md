**rkyv > collections > btree > map > iter**

# Module: collections::btree::map::iter

## Contents

**Structs**

- [`Iter`](#iter) - An iterator over the entires of an `ArchivedBTreeMap`.
- [`IterSeal`](#iterseal) - An iterator over the entires of an `ArchivedBTreeMap`.
- [`Keys`](#keys) - An iterator over the keys of an `ArchivedBTreeMap`.
- [`Range`](#range) - An iterator over a sub-range of entries of an `ArchivedBTreeMap`.
- [`RangeSeal`](#rangeseal) - A mutable iterator over a sub-range of entries of an `ArchivedBTreeMap`.
- [`Values`](#values) - An iterator over the values of an `ArchivedBTreeMap`.
- [`ValuesSeal`](#valuesseal) - A mutable iterator over the values of an `ArchivedBTreeMap`.

---

## rkyv::collections::btree::map::iter::Iter

*Struct*

An iterator over the entires of an `ArchivedBTreeMap`.

This struct is created by the [`iter`](ArchivedBTreeMap::iter) method on
[`ArchivedBTreeMap`]. See its documentation for more.

**Generic Parameters:**
- 'a
- K
- V
- const E

**Traits:** FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::btree::map::iter::IterSeal

*Struct*

An iterator over the entires of an `ArchivedBTreeMap`.

This struct is created by the [`iter_seal`](ArchivedBTreeMap::iter_seal)
method on [`ArchivedBTreeMap`]. See its documentation for more.

**Generic Parameters:**
- 'a
- K
- V
- const E

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::btree::map::iter::Keys

*Struct*

An iterator over the keys of an `ArchivedBTreeMap`.

This struct is created by the [`keys`](ArchivedBTreeMap::keys) method on
[`ArchivedBTreeMap`]. See its documentation for more.

**Generic Parameters:**
- 'a
- K
- V
- const E

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::btree::map::iter::Range

*Struct*

An iterator over a sub-range of entries of an `ArchivedBTreeMap`.

**Generic Parameters:**
- 'a
- K
- V
- const E

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::btree::map::iter::RangeSeal

*Struct*

A mutable iterator over a sub-range of entries of an `ArchivedBTreeMap`.

**Generic Parameters:**
- 'a
- K
- V
- const E

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::btree::map::iter::Values

*Struct*

An iterator over the values of an `ArchivedBTreeMap`.

This struct is created by the [`values`](ArchivedBTreeMap::keys) method on
[`ArchivedBTreeMap`]. See its documentation for more.

**Generic Parameters:**
- 'a
- K
- V
- const E

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::btree::map::iter::ValuesSeal

*Struct*

A mutable iterator over the values of an `ArchivedBTreeMap`.

This struct is created by the [`values_pin`](ArchivedBTreeMap::keys) method
on [`ArchivedBTreeMap`]. See its documentation for more.

**Generic Parameters:**
- 'a
- K
- V
- const E

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



