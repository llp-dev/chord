**memchr > memmem**

# Module: memmem

## Contents

**Structs**

- [`FindIter`](#finditer) - An iterator over non-overlapping substring matches.
- [`FindRevIter`](#findreviter) - An iterator over non-overlapping substring matches in reverse.
- [`Finder`](#finder) - A single substring searcher fixed to a particular needle.
- [`FinderBuilder`](#finderbuilder) - A builder for constructing non-default forward or reverse memmem finders.
- [`FinderRev`](#finderrev) - A single substring reverse searcher fixed to a particular needle.

**Functions**

- [`find`](#find) - Returns the index of the first occurrence of the given needle.
- [`find_iter`](#find_iter) - Returns an iterator over all non-overlapping occurrences of a substring in
- [`rfind`](#rfind) - Returns the index of the last occurrence of the given needle.
- [`rfind_iter`](#rfind_iter) - Returns a reverse iterator over all non-overlapping occurrences of a

---

## memchr::memmem::FindIter

*Struct*

An iterator over non-overlapping substring matches.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

**Generic Parameters:**
- 'h
- 'n

**Methods:**

- `fn into_owned(self: Self) -> FindIter<'h, 'static>` - Convert this iterator into its owned variant, such that it no longer

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<usize>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> FindIter<'h, 'n>`



## memchr::memmem::FindRevIter

*Struct*

An iterator over non-overlapping substring matches in reverse.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

**Generic Parameters:**
- 'h
- 'n

**Methods:**

- `fn into_owned(self: Self) -> FindRevIter<'h, 'static>` - Convert this iterator into its owned variant, such that it no longer

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<usize>`
- **Clone**
  - `fn clone(self: &Self) -> FindRevIter<'h, 'n>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::memmem::Finder

*Struct*

A single substring searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general, using
[`find`] is good enough, but `Finder` is useful when you can meaningfully
observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `Finder` that is not connected to
the lifetime of its needle.

**Generic Parameters:**
- 'n

**Methods:**

- `fn new<B>(needle: &'n B) -> Finder<'n>` - Create a new finder for the given needle.
- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>` - Returns the index of the first occurrence of this needle in the given
- `fn find_iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> FindIter<'h, 'a>` - Returns an iterator over all occurrences of a substring in a haystack.
- `fn into_owned(self: Self) -> Finder<'static>` - Convert this finder into its owned variant, such that it no longer
- `fn as_ref(self: &Self) -> Finder` - Convert this finder into its borrowed variant.
- `fn needle(self: &Self) -> &[u8]` - Returns the needle that this finder searches for.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Finder<'n>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::memmem::FinderBuilder

*Struct*

A builder for constructing non-default forward or reverse memmem finders.

A builder is primarily useful for configuring a substring searcher.
Currently, the only configuration exposed is the ability to disable
heuristic prefilters used to speed up certain searches.

**Methods:**

- `fn new() -> FinderBuilder` - Create a new finder builder with default settings.
- `fn build_forward<'n, B>(self: &Self, needle: &'n B) -> Finder<'n>` - Build a forward finder using the given needle from the current
- `fn build_forward_owned<B>(self: &Self, needle: B) -> Finder<'static>` - Build an owned forward finder using the given needle from the current
- `fn build_forward_with_ranker<'n, R, B>(self: &Self, ranker: R, needle: &'n B) -> Finder<'n>` - Build a forward finder using the given needle and a custom heuristic
- `fn build_forward_with_ranker_owned<R, B>(self: &Self, ranker: R, needle: B) -> Finder<'static>` - Build an owned forward finder using the given needle and a custom
- `fn build_reverse<'n, B>(self: &Self, needle: &'n B) -> FinderRev<'n>` - Build a reverse finder using the given needle from the current
- `fn build_reverse_owned<B>(self: &Self, needle: B) -> FinderRev<'static>` - Build an owned reverse finder using the given needle from the current
- `fn prefilter(self: & mut Self, prefilter: Prefilter) -> & mut FinderBuilder` - Configure the prefilter setting for the finder.

**Trait Implementations:**

- **Default**
  - `fn default() -> FinderBuilder`
- **Clone**
  - `fn clone(self: &Self) -> FinderBuilder`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::memmem::FinderRev

*Struct*

A single substring reverse searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general,
using [`rfind`] is good enough, but `FinderRev` is useful when you can
meaningfully observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `FinderRev` that is not connected to
the lifetime of its needle.

**Generic Parameters:**
- 'n

**Methods:**

- `fn new<B>(needle: &'n B) -> FinderRev<'n>` - Create a new reverse finder for the given needle.
- `fn rfind<B>(self: &Self, haystack: B) -> Option<usize>` - Returns the index of the last occurrence of this needle in the given
- `fn rfind_iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> FindRevIter<'h, 'a>` - Returns a reverse iterator over all occurrences of a substring in a
- `fn into_owned(self: Self) -> FinderRev<'static>` - Convert this finder into its owned variant, such that it no longer
- `fn as_ref(self: &Self) -> FinderRev` - Convert this finder into its borrowed variant.
- `fn needle(self: &Self) -> &[u8]` - Returns the needle that this finder searches for.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FinderRev<'n>`



## memchr::memmem::find

*Function*

Returns the index of the first occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`Finder`] once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::find(haystack, b"foo"));
assert_eq!(Some(4), memmem::find(haystack, b"bar"));
assert_eq!(None, memmem::find(haystack, b"quux"));
```

```rust
fn find(haystack: &[u8], needle: &[u8]) -> Option<usize>
```



## memchr::memmem::find_iter

*Function*

Returns an iterator over all non-overlapping occurrences of a substring in
a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::find_iter(haystack, b"foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

```rust
fn find_iter<'h, 'n, N>(haystack: &'h [u8], needle: &'n N) -> FindIter<'h, 'n>
```



## memchr::memmem::rfind

*Function*

Returns the index of the last occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`FinderRev`] once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::rfind(haystack, b"foo"));
assert_eq!(Some(4), memmem::rfind(haystack, b"bar"));
assert_eq!(Some(8), memmem::rfind(haystack, b"ba"));
assert_eq!(None, memmem::rfind(haystack, b"quux"));
```

```rust
fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize>
```



## memchr::memmem::rfind_iter

*Function*

Returns a reverse iterator over all non-overlapping occurrences of a
substring in a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::rfind_iter(haystack, b"foo");
assert_eq!(Some(16), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(0), it.next());
assert_eq!(None, it.next());
```

```rust
fn rfind_iter<'h, 'n, N>(haystack: &'h [u8], needle: &'n N) -> FindRevIter<'h, 'n>
```



